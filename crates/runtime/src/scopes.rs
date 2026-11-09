/*!
The data stacks of the running thread, as compiled code uses them while
Sayon moves its values onto them.

For now arrays go there, and only those a function makes and keeps to
itself, which the compiler picks (`sayon_ir::stacks`). A scope holding such
arrays marks its stack when it starts and restores the mark on every way
out, which frees their storage at once. Every other array keeps its storage
on the heap, and an array on a stack moves to the heap when it can't grow
where it is.

IMPORTANT: an array grows where it is only while it ends at its stack's top
and no scope has opened on that stack since its storage was made. A scope
opened since then would restore its mark below the array's new end and cut
it short. So each stack counts its open scopes, and an array on a stack
keeps the count from when it was made in the word before its elements.
Guarded by `an_array_moves_to_the_heap_when_a_scope_opened_above_it`.
*/

use crate::stacks::{MOST, Mark, Stacks, THREAD_RESERVE, broken};
use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;

/** The word before an array's elements on a stack, which holds the count of open scopes. */
const HEADER: usize = 8;

struct Thread {
    /** Whether the thread has tried to reserve its stacks. */
    tried: bool,
    /** The thread's stacks, or `None` before it tried, or when the system refused. */
    stacks: Option<Stacks>,
    /** How many scopes are open on each stack. */
    open: [usize; MOST],
}

thread_local! {
    /*
    A thread keeps its stacks as long as it lives. They're never dropped, so
    that the thread needs no destructor registered, which would take memory
    from a heap; a thread that ends leaves its reservation to the system.
    */
    static THREAD: UnsafeCell<ManuallyDrop<Thread>> = const {
        UnsafeCell::new(ManuallyDrop::new(Thread {
            tried: false,
            stacks: None,
            open: [0; MOST],
        }))
    };
}

/** Runs `f` on the running thread's state; `f` calls nothing that runs it again. */
fn with<T>(f: impl FnOnce(&mut Thread) -> T) -> T {
    /* SAFETY: the state is the thread's own, and `f` doesn't reach it again, so this is its only reference. */
    THREAD.with(|thread| f(unsafe { &mut *thread.get() }))
}

impl Thread {
    /** Reserves the thread's stacks the first time they're needed. */
    fn ready(&mut self) {
        if !self.tried {
            self.tried = true;
            self.stacks = Stacks::reserve(MOST, THREAD_RESERVE).ok();
        }
    }
}

/**
Marks data stack `stack` as a scope that keeps arrays there begins, and gives
the mark to restore when it ends, or 0 when the thread can't have stacks.
*/
#[unsafe(no_mangle)]
pub extern "C" fn sayon_stack_mark(stack: i32) -> i64 {
    mark(stack as usize)
}

/** Ends the scope whose `mark` began it on data stack `stack`, freeing what it put there. */
#[unsafe(no_mangle)]
pub extern "C" fn sayon_stack_restore(stack: i32, mark: i64) {
    restore(stack as usize, mark)
}

/** What `sayon_stack_mark` does, where a broken rule can unwind for the tests. */
fn mark(index: usize) -> i64 {
    with(|thread| {
        thread.ready();
        let Some(stacks) = thread.stacks.as_mut() else {
            return 0;
        };
        let mark = stacks.stack(index).mark();
        thread.open[index] += 1;
        mark.word() as i64
    })
}

/** What `sayon_stack_restore` does, where a broken rule can unwind for the tests. */
fn restore(index: usize, mark: i64) {
    with(|thread| {
        let Some(stacks) = thread.stacks.as_mut() else {
            return;
        };
        let stack = stacks.stack(index);
        let Some(open) = thread.open[index].checked_sub(1) else {
            broken("a scope ended on a data stack it never began on");
        };
        thread.open[index] = open;
        stack.restore(Mark::from_word(mark as usize));
    })
}

/**
Room on data stack `stack` for `capacity` elements of `size` bytes, after the
header, or nothing when the thread has no stacks or the stack is full, and
the array goes on the heap instead.
*/
pub(crate) fn place(capacity: usize, size: usize, stack: usize) -> Option<*mut u8> {
    with(|thread| {
        thread.ready();
        let stacks = thread.stacks.as_mut()?;
        let bytes = capacity.checked_mul(size)?.checked_add(HEADER)?;
        let start = stacks.stack(stack).take(bytes, HEADER).ok()?;
        /* SAFETY: the start of what was just taken, a word that is the header. */
        unsafe { (start as *mut usize).write(thread.open[stack]) };
        /* SAFETY: within what was just taken. */
        Some(unsafe { start.add(HEADER) })
    })
}

/** What growing an array's storage comes to. */
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Room {
    /** It grew where it is. */
    Grown,
    /** It is on a stack but can't grow there, so it moves to the heap. */
    Moving,
    /** It isn't on the thread's stacks. */
    Heap,
}

/**
Grows the storage at `elements`, which has room for `capacity` elements of
`size` bytes, to room for `wanted`, where it is when the rule of this module
allows it.
*/
pub(crate) fn grow(elements: *mut u8, capacity: usize, wanted: usize, size: usize) -> Room {
    with(|thread| {
        let Some(stacks) = thread.stacks.as_mut() else {
            return Room::Heap;
        };
        let address = elements as usize;
        let Some(index) = stacks.index_of(address) else {
            return Room::Heap;
        };
        let start = address - HEADER;
        /* SAFETY: storage on a stack starts with its header. */
        let made_with = unsafe { (start as *const usize).read() };
        let stack = stacks.stack(index);
        /* It was taken from the stack, so it fits. */
        let used = capacity * size;
        /* IMPORTANT: the rule of this module. */
        if made_with != thread.open[index] || address + used != stack.top() as usize {
            return Room::Moving;
        }
        let Some(more) = (wanted - capacity).checked_mul(size) else {
            return Room::Moving;
        };
        match stack.grow(start as *mut u8, HEADER + used, more) {
            Ok(()) => Room::Grown,
            Err(_) => Room::Moving,
        }
    })
}

/**
Whether `elements` lies on the thread's stacks. Storage there is freed with
its scope, never one array at a time.
*/
pub(crate) fn holds(elements: *const u8) -> bool {
    with(|thread| {
        thread
            .stacks
            .as_ref()
            .is_some_and(|stacks| stacks.index_of(elements as usize).is_some())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stacks::alone;

    #[test]
    fn restoring_a_mark_frees_what_its_scope_put_on_the_stack() {
        let _alone = alone();
        let begun = mark(1);
        let first = place(4, 8, 1).unwrap();
        restore(1, begun);
        let begun = mark(1);
        assert_eq!(
            place(4, 8, 1).unwrap(),
            first,
            "the freed place is used again"
        );
        restore(1, begun);
    }

    #[test]
    fn an_array_on_top_grows_where_it_is() {
        let _alone = alone();
        let begun = mark(1);
        let elements = place(4, 8, 1).unwrap();
        assert_eq!(grow(elements, 4, 1000, 8), Room::Grown);
        /* SAFETY: the array has room for 1000 elements of 8 bytes now. */
        unsafe { std::ptr::write_bytes(elements, 3, 8000) };
        restore(1, begun);
    }

    #[test]
    fn an_array_moves_to_the_heap_when_a_scope_opened_above_it() {
        let _alone = alone();
        let outer = mark(1);
        let elements = place(4, 8, 1).unwrap();
        let inner = mark(1);
        assert_eq!(
            grow(elements, 4, 8, 8),
            Room::Moving,
            "the inner scope would cut it short"
        );
        restore(1, inner);
        assert_eq!(
            grow(elements, 4, 8, 8),
            Room::Grown,
            "once the inner scope ended, it may grow"
        );
        restore(1, outer);
    }

    #[test]
    fn an_array_with_another_above_it_moves_to_the_heap() {
        let _alone = alone();
        let begun = mark(2);
        let below = place(4, 8, 2).unwrap();
        let above = place(4, 8, 2).unwrap();
        assert_eq!(grow(below, 4, 8, 8), Room::Moving);
        assert_eq!(grow(above, 4, 8, 8), Room::Grown);
        restore(2, begun);
    }

    #[test]
    fn storage_off_the_stacks_is_left_to_the_heap() {
        let _alone = alone();
        let begun = mark(1);
        let on_stack = place(4, 8, 1).unwrap();
        let storage = [0u64; 4];
        let elsewhere = storage.as_ptr() as *mut u8;
        assert!(holds(on_stack));
        assert!(!holds(elsewhere));
        assert_eq!(grow(elsewhere, 4, 8, 8), Room::Heap);
        restore(1, begun);
    }

    #[test]
    #[should_panic(expected = "a scope ended on a data stack it never began on")]
    fn a_restore_without_a_mark_is_refused() {
        let _alone = alone();
        let begun = mark(3);
        restore(3, begun);
        restore(3, begun);
    }
}
