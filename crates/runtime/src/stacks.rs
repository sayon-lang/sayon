/*!
Data stacks: where a Sayon program keeps everything whose size it learns only
while it runs, such as the elements of an array, the text of a long string,
and the entries of a dictionary.

A data stack is a stretch of address space that the system reserves at once
and commits as it fills. Its top moves up as values go on it. A scope notes
the top when it starts, as a [`Mark`], and puts the top back when it ends,
which frees everything the scope put on the stack in one store, however much
that was. Nothing in this module takes memory from a heap.

A thread or a task has a set of stacks, [`Stacks`], up to [`MOST`] of them,
cut from one reservation. The compiler decides which value goes on which
stack, so that a value that can still grow is alone at the top of its stack.

IMPORTANT: the rules below keep every value on a stack valid. The compiler
relies on them when it places values, and this module checks them: the
first two always, since breaking them would write over other values, and
the third by its arithmetic. Each names the test that guards it.

1. A value grows in place only when it ends at its stack's top
   ([`Stack::grow`]); growing any other value would write over the values
   above it. Guarded by `growing_a_value_below_the_top_is_refused`.
2. Marks are restored in the reverse order they were taken, so restoring one
   frees exactly what went on the stack after it ([`Stack::restore`]).
   Guarded by `restoring_frees_what_came_after_the_mark` and
   `a_mark_above_the_top_is_refused`.
3. Sizes and addresses are computed with checked arithmetic, so a size that
   doesn't fit gives [`Full`] instead of wrapping around, on 32-bit
   WebAssembly too. Guarded by `sizes_that_do_not_fit_are_refused`.
*/

use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/** The most stacks a thread or a task has. */
pub const MOST: usize = 8;

/** How much each of a thread's stacks may hold unless the program says otherwise. */
#[cfg(target_pointer_width = "64")]
pub const THREAD_RESERVE: usize = 4 << 30;
/** How much each of a task's stacks may hold unless the program says otherwise. */
#[cfg(target_pointer_width = "64")]
pub const TASK_RESERVE: usize = 64 << 20;
/**
How much each of a thread's stacks may hold unless the program says
otherwise. WebAssembly can't reserve addresses without giving memory, so its
stacks are smaller.
*/
#[cfg(target_pointer_width = "32")]
pub const THREAD_RESERVE: usize = 16 << 20;
/** How much each of a task's stacks may hold unless the program says otherwise. */
#[cfg(target_pointer_width = "32")]
pub const TASK_RESERVE: usize = 4 << 20;

/**
Memory is committed, and given back, in steps of this many bytes: a multiple
of every page size Sayon's targets use (4, 16, and 64 KiB).
*/
const STEP: usize = 64 << 10;
/** Giving memory back starts when a restore leaves more than this committed above the top. */
const TRIM_ABOVE: usize = 64 << 20;
/** How much stays committed above the top when memory is given back. */
const TRIM_KEEP: usize = 4 << 20;

/** The byte that checking mode writes over what a scope frees. */
pub const FREED: u8 = 0xDD;

/**
One data stack: the part of a reservation from `base` to `end`, of which
`base` to `committed` can be used without asking the system, and `base` to
`top` holds values. Generated code reads and writes these words directly, so
their order is fixed.
*/
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Stack {
    top: usize,
    committed: usize,
    end: usize,
    base: usize,
}

/** Where a stack's top was when a scope started. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mark(usize);

impl Mark {
    /** The mark as a word, which generated code keeps in a slot of its own. */
    pub fn word(self) -> usize {
        self.0
    }

    /** A mark that generated code kept as a word; [`Stack::restore`] checks it. */
    pub fn from_word(word: usize) -> Mark {
        Mark(word)
    }
}

/** Why a stack could not take more. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Full {
    /** It would pass the end of the stack's reservation, of `reserved` bytes. */
    Reservation { reserved: usize },
    /** The system refused to reserve or commit more memory. */
    Refused,
    /** The program would hold more than the `limit` bytes set with [`limit`]. */
    Limit { limit: usize },
}

impl fmt::Display for Full {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Full::Reservation { reserved } => {
                out.write_str("a data stack is full at ")?;
                size(out, reserved)?;
                out.write_str(
                    "; --stack-reserve sets how much each of a thread's stacks may hold, and --task-stack-reserve each of a task's",
                )
            }
            Full::Refused => out.write_str("the system has no more memory to give"),
            Full::Limit { limit } => {
                out.write_str("the program would hold more than its limit of ")?;
                size(out, limit)?;
                out.write_str(" on its data stacks")
            }
        }
    }
}

/** Writes a number of bytes in the largest unit that keeps it whole. */
fn size(out: &mut fmt::Formatter, bytes: usize) -> fmt::Result {
    for (unit, shift) in [("GiB", 30), ("MiB", 20), ("KiB", 10)] {
        if bytes >= 1 << shift && bytes.is_multiple_of(1 << shift) {
            return write!(out, "{} {unit}", bytes >> shift);
        }
    }
    write!(out, "{bytes} bytes")
}

/** Whether checking mode is on: debug builds of a program turn it on. */
static CHECKING: AtomicBool = AtomicBool::new(false);
/** The bytes committed on every stack of the program. */
static COMMITTED: AtomicUsize = AtomicUsize::new(0);
/** The most bytes the program's stacks may commit, which the sandbox lowers. */
static LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);

/**
Turns checking mode on or off. In checking mode, what a scope frees is
written over with [`FREED`], so that a bug which reads freed memory finds a
pattern the tests can see instead of a value that still looks right.
*/
pub fn set_checking(on: bool) {
    CHECKING.store(on, Ordering::Relaxed);
}

/** Lets the program's stacks commit at most `bytes` in all; past it, [`Full::Limit`]. */
pub fn limit(bytes: usize) {
    LIMIT.store(bytes, Ordering::Relaxed);
}

/** The bytes committed on every stack of the program. */
pub fn committed() -> usize {
    COMMITTED.load(Ordering::Relaxed)
}

/** Counts `bytes` more committed, unless that would pass the limit. */
fn admit(bytes: usize) -> Result<(), Full> {
    let before = COMMITTED.fetch_add(bytes, Ordering::Relaxed);
    let limit = LIMIT.load(Ordering::Relaxed);
    match before.checked_add(bytes) {
        Some(after) if after <= limit => Ok(()),
        _ => {
            COMMITTED.fetch_sub(bytes, Ordering::Relaxed);
            Err(Full::Limit { limit })
        }
    }
}

/** Counts `bytes` given back. */
fn give_back(bytes: usize) {
    COMMITTED.fetch_sub(bytes, Ordering::Relaxed);
}

/** `value` rounded up to a multiple of `step`, a power of two, or nothing when that doesn't fit. */
fn round_up(value: usize, step: usize) -> Option<usize> {
    Some(value.checked_add(step - 1)? & !(step - 1))
}

/**
Stops the program over a broken rule of this module, which only a bug in
Sayon itself can break. Tests see a panic instead, so that they can check
that the rule is enforced.
*/
#[cold]
pub(crate) fn broken(what: &str) -> ! {
    #[cfg(test)]
    panic!("{what}");
    #[cfg(not(test))]
    {
        report!("fatal error: {what}, a bug in Sayon itself");
        std::process::abort()
    }
}

impl Stack {
    /** A stack of nothing, as the unused places of a set hold. */
    const NONE: Stack = Stack {
        top: 0,
        committed: 0,
        end: 0,
        base: 0,
    };

    /** An empty stack over the `size` bytes from `base`, none committed yet. */
    fn over(base: usize, size: usize) -> Stack {
        Stack {
            top: base,
            committed: base,
            end: base + size,
            base,
        }
    }

    /** Where the top is now, for a scope to restore when it ends. */
    #[inline]
    pub fn mark(&self) -> Mark {
        Mark(self.top)
    }

    /** Where the stack's values end. */
    #[inline]
    pub fn top(&self) -> *mut u8 {
        self.top as *mut u8
    }

    /**
    Puts `size` bytes aligned to `align`, a power of two no bigger than
    64 KiB, on top of the stack, and gives where they start. The bytes hold
    whatever they held before.
    */
    #[inline]
    pub fn take(&mut self, size: usize, align: usize) -> Result<*mut u8, Full> {
        debug_assert!(align.is_power_of_two() && align <= STEP);
        /* IMPORTANT: rule 3 of this module, checked arithmetic throughout. */
        let start = round_up(self.top, align).ok_or_else(|| self.too_big())?;
        let new_top = start.checked_add(size).ok_or_else(|| self.too_big())?;
        if new_top > self.committed {
            self.commit_to(new_top)?;
        }
        self.top = new_top;
        Ok(start as *mut u8)
    }

    /**
    Makes the value of `size` bytes at `start` `more` bytes bigger, where it
    is. The new bytes hold whatever they held before.
    */
    #[inline]
    pub fn grow(&mut self, start: *mut u8, size: usize, more: usize) -> Result<(), Full> {
        /*
        IMPORTANT: rule 1 of this module. Only the value at the top may grow:
        any other has values above it, which growing would write over.
        */
        if (start as usize).checked_add(size) != Some(self.top) {
            broken("a value grew on a data stack while another sat above it");
        }
        let new_top = self.top.checked_add(more).ok_or_else(|| self.too_big())?;
        if new_top > self.committed {
            self.commit_to(new_top)?;
        }
        self.top = new_top;
        Ok(())
    }

    /**
    Puts the top back where it was at `mark`, which frees every value put on
    the stack since. In checking mode the freed bytes are written over with
    [`FREED`].
    */
    #[inline]
    pub fn restore(&mut self, mark: Mark) {
        /*
        IMPORTANT: rule 2 of this module. A mark below the base belongs to
        another stack, and one above the top was taken in a scope that has
        ended already: restoring either would put values back that are gone.
        */
        if mark.0 < self.base || mark.0 > self.top {
            broken("a scope ended out of order on a data stack");
        }
        if CHECKING.load(Ordering::Relaxed) {
            /* SAFETY: from the mark to the top is committed memory of this stack. */
            unsafe { std::ptr::write_bytes(mark.0 as *mut u8, FREED, self.top - mark.0) };
        }
        self.top = mark.0;
        if self.committed - self.top > TRIM_ABOVE {
            self.trim();
        }
    }

    /** The answer to a request that can't fit in any reservation. */
    #[cold]
    fn too_big(&self) -> Full {
        Full::Reservation {
            reserved: self.end - self.base,
        }
    }

    /**
    Commits enough memory for the top to reach `needed`: at least one step,
    and at least an eighth of what is committed already, so that a stack
    that grows large asks the system less often.
    */
    #[cold]
    fn commit_to(&mut self, needed: usize) -> Result<(), Full> {
        if needed > self.end {
            return Err(self.too_big());
        }
        let wanted = (needed - self.committed)
            .max(STEP)
            .max((self.committed - self.base) / 8);
        let step = round_up(wanted, STEP).ok_or(Full::Refused)?;
        let new_committed = self.committed.saturating_add(step).min(self.end);
        let bytes = new_committed - self.committed;
        admit(bytes)?;
        if !os::commit(self.committed, bytes) {
            give_back(bytes);
            return Err(Full::Refused);
        }
        self.committed = new_committed;
        Ok(())
    }

    /** Gives the system back what is committed beyond a little above the top. */
    #[cold]
    fn trim(&mut self) {
        let kept = (self.top - self.base).checked_add(TRIM_KEEP);
        let Some(from) = kept.and_then(|kept| round_up(kept, STEP)?.checked_add(self.base)) else {
            return;
        };
        if from < self.committed {
            os::decommit(from, self.committed - from);
            give_back(self.committed - from);
            self.committed = from;
        }
    }
}

/**
The data stacks of a thread or a task, cut from one reservation. Dropping
the set gives all of its memory back to the system.

IMPORTANT: no value on these stacks may be used once the set is dropped. The
runtime keeps a thread's set for as long as the thread runs Sayon code, and
a task's for as long as the task lives.
*/
#[repr(C)]
#[derive(Debug)]
pub struct Stacks {
    stacks: [Stack; MOST],
    count: usize,
    base: usize,
    size: usize,
}

impl Stacks {
    /**
    Reserves `count` stacks, from 1 to [`MOST`], that may hold `each` bytes
    apiece. Nothing is committed until a stack is used.
    */
    pub fn reserve(count: usize, each: usize) -> Result<Stacks, Full> {
        if count == 0 || count > MOST {
            broken("a thread or task asked for a number of data stacks Sayon doesn't give");
        }
        let each = round_up(each.max(STEP), STEP).ok_or(Full::Refused)?;
        let size = each.checked_mul(count).ok_or(Full::Refused)?;
        let base = os::reserve(size);
        if base == 0 {
            return Err(Full::Refused);
        }
        let mut stacks = [Stack::NONE; MOST];
        for (index, stack) in stacks.iter_mut().take(count).enumerate() {
            *stack = Stack::over(base + index * each, each);
        }
        Ok(Stacks {
            stacks,
            count,
            base,
            size,
        })
    }

    /** Stack number `index` of the set. */
    #[inline]
    pub fn stack(&mut self, index: usize) -> &mut Stack {
        if index >= self.count {
            broken("a value was placed on a data stack its thread doesn't have");
        }
        &mut self.stacks[index]
    }

    /** The number of the stack whose reservation holds `address`, if one of the set does. */
    #[inline]
    pub fn index_of(&self, address: usize) -> Option<usize> {
        let offset = address.checked_sub(self.base)?;
        if offset >= self.size {
            return None;
        }
        Some(offset / (self.size / self.count))
    }
}

/**
The tests of the runtime count what every stack of the program commits, and
change the limit and checking mode, which all stacks share, so those that
use stacks run one at a time.
*/
#[cfg(test)]
pub(crate) fn alone() -> std::sync::MutexGuard<'static, ()> {
    static ONE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    ONE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Drop for Stacks {
    fn drop(&mut self) {
        for stack in &self.stacks[..self.count] {
            give_back(stack.committed - stack.base);
        }
        os::release(self.base, self.size);
    }
}

#[cfg(windows)]
mod os {
    use std::ffi::c_void;

    const MEM_COMMIT: u32 = 0x1000;
    const MEM_RESERVE: u32 = 0x2000;
    const MEM_DECOMMIT: u32 = 0x4000;
    const MEM_RELEASE: u32 = 0x8000;
    const PAGE_NOACCESS: u32 = 0x01;
    const PAGE_READWRITE: u32 = 0x04;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn VirtualAlloc(
            address: *mut c_void,
            size: usize,
            kind: u32,
            protection: u32,
        ) -> *mut c_void;
        fn VirtualFree(address: *mut c_void, size: usize, kind: u32) -> i32;
    }

    /** Reserves `size` bytes of addresses, none of them usable yet, or gives 0. */
    pub fn reserve(size: usize) -> usize {
        /* SAFETY: a new reservation, which touches no memory that exists. */
        unsafe { VirtualAlloc(std::ptr::null_mut(), size, MEM_RESERVE, PAGE_NOACCESS) as usize }
    }

    /** Makes the `size` bytes from `start`, inside a reservation, usable. */
    pub fn commit(start: usize, size: usize) -> bool {
        /* SAFETY: addresses inside a reservation of this module's. */
        unsafe { !VirtualAlloc(start as *mut c_void, size, MEM_COMMIT, PAGE_READWRITE).is_null() }
    }

    /** Gives back the memory of `size` bytes from `start`, keeping their addresses reserved. */
    pub fn decommit(start: usize, size: usize) {
        /* SAFETY: committed memory of a reservation of this module's, above every value. */
        unsafe { VirtualFree(start as *mut c_void, size, MEM_DECOMMIT) };
    }

    /** Gives back a whole reservation and its memory. */
    pub fn release(start: usize, _size: usize) {
        /* SAFETY: a reservation of this module's that no value uses any more. */
        unsafe { VirtualFree(start as *mut c_void, 0, MEM_RELEASE) };
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod os {
    use std::ffi::c_void;

    const PROT_NONE: i32 = 0;
    const PROT_READ: i32 = 1;
    const PROT_WRITE: i32 = 2;
    const MAP_PRIVATE: i32 = 0x02;
    const MAP_FIXED: i32 = 0x10;
    const MAP_ANONYMOUS: i32 = 0x20;
    const MAP_NORESERVE: i32 = 0x4000;
    const NOTHING: i32 = MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE;

    unsafe extern "C" {
        fn mmap(
            address: *mut c_void,
            length: usize,
            protection: i32,
            flags: i32,
            fd: i32,
            offset: i64,
        ) -> *mut c_void;
        fn mprotect(address: *mut c_void, length: usize, protection: i32) -> i32;
        fn munmap(address: *mut c_void, length: usize) -> i32;
    }

    /** Reserves `size` bytes of addresses, none of them usable yet, or gives 0. */
    pub fn reserve(size: usize) -> usize {
        /* SAFETY: a new mapping, which touches no memory that exists. */
        let address = unsafe { mmap(std::ptr::null_mut(), size, PROT_NONE, NOTHING, -1, 0) };
        if address as isize == -1 {
            0
        } else {
            address as usize
        }
    }

    /** Makes the `size` bytes from `start`, inside a reservation, usable. */
    pub fn commit(start: usize, size: usize) -> bool {
        /* SAFETY: addresses inside a reservation of this module's. */
        unsafe { mprotect(start as *mut c_void, size, PROT_READ | PROT_WRITE) == 0 }
    }

    /**
    Gives back the memory of `size` bytes from `start`, keeping their
    addresses reserved: a fresh mapping without access takes their place.
    */
    pub fn decommit(start: usize, size: usize) {
        /* SAFETY: committed memory of a reservation of this module's, above every value. */
        unsafe {
            mmap(
                start as *mut c_void,
                size,
                PROT_NONE,
                NOTHING | MAP_FIXED,
                -1,
                0,
            )
        };
    }

    /** Gives back a whole reservation and its memory. */
    pub fn release(start: usize, size: usize) {
        /* SAFETY: a reservation of this module's that no value uses any more. */
        unsafe { munmap(start as *mut c_void, size) };
    }
}

/**
WebAssembly has one memory, which grows at its end and never shrinks, and
whatever it holds can be used. So reserving takes the stacks' bytes from the
end of that memory at once, and committing and giving back do nothing.
*/
#[cfg(target_arch = "wasm32")]
mod os {
    const PAGE: usize = 64 << 10;

    /** Grows the memory by `size` bytes and gives where they start, or gives 0. */
    pub fn reserve(size: usize) -> usize {
        let previous = core::arch::wasm32::memory_grow(0, size.div_ceil(PAGE));
        if previous == usize::MAX {
            0
        } else {
            previous * PAGE
        }
    }

    pub fn commit(_start: usize, _size: usize) -> bool {
        true
    }

    pub fn decommit(_start: usize, _size: usize) {}

    /** Memory can't be given back on WebAssembly, so a reservation stays the program's. */
    pub fn release(_start: usize, _size: usize) {}
}

#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "android",
    target_arch = "wasm32"
)))]
mod os {
    fn unavailable() -> ! {
        report!(
            "fatal error: data stacks are written only for Windows, Linux, Android, and WebAssembly so far"
        );
        std::process::abort()
    }

    pub fn reserve(_size: usize) -> usize {
        unavailable()
    }

    pub fn commit(_start: usize, _size: usize) -> bool {
        unavailable()
    }

    pub fn decommit(_start: usize, _size: usize) {
        unavailable()
    }

    pub fn release(_start: usize, _size: usize) {
        unavailable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /** Stacks of 1 MiB, small enough for a test to fill. */
    const SMALL: usize = 1 << 20;

    #[test]
    fn each_address_belongs_to_the_stack_that_holds_it() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(3, SMALL).unwrap();
        let first = stacks.stack(0).take(8, 8).unwrap() as usize;
        let third = stacks.stack(2).take(8, 8).unwrap() as usize;
        assert_eq!(stacks.index_of(first), Some(0));
        assert_eq!(stacks.index_of(first + SMALL), Some(1));
        assert_eq!(stacks.index_of(third), Some(2));
        assert_eq!(stacks.index_of(first - 1), None);
        assert_eq!(stacks.index_of(first + 3 * SMALL), None);
    }

    #[test]
    fn values_are_aligned_apart_and_usable() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(2, SMALL).unwrap();
        let stack = stacks.stack(0);
        let mut previous_end = 0;
        for (size, align) in [
            (1, 1),
            (3, 2),
            (8, 8),
            (24, 8),
            (100, 16),
            (5000, 64),
            (70_000, 4096),
            (0, 8),
        ] {
            let at = stack.take(size, align).unwrap() as usize;
            assert_eq!(at % align, 0, "{size} bytes aligned to {align}");
            assert!(at >= previous_end, "{size} bytes overlap the value before");
            /* SAFETY: the bytes just taken. */
            unsafe { std::ptr::write_bytes(at as *mut u8, 0xAB, size) };
            previous_end = at + size;
        }
        let other = stacks.stack(1).take(16, 8).unwrap() as usize;
        assert!(
            other >= stacks.stacks[0].end,
            "the second stack starts after the first"
        );
    }

    #[test]
    fn restoring_frees_what_came_after_the_mark() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let first = stack.take(64, 8).unwrap();
        let outer = stack.mark();
        let freed = stack.take(1000, 8).unwrap();
        let inner = stack.mark();
        stack.take(3000, 8).unwrap();
        stack.restore(inner);
        stack.restore(outer);
        assert_eq!(
            stack.take(1000, 8).unwrap(),
            freed,
            "the freed place is used again"
        );
        assert!(first < freed);
    }

    #[test]
    #[should_panic(expected = "a scope ended out of order on a data stack")]
    fn a_mark_above_the_top_is_refused() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let outer = stack.mark();
        stack.take(64, 8).unwrap();
        let inner = stack.mark();
        stack.restore(outer);
        stack.restore(inner);
    }

    #[test]
    #[should_panic(expected = "a scope ended out of order on a data stack")]
    fn a_mark_of_another_stack_is_refused() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(2, SMALL).unwrap();
        let mark = stacks.stack(0).mark();
        stacks.stack(1).take(64, 8).unwrap();
        stacks.stack(1).restore(mark);
    }

    #[test]
    fn the_top_value_grows_where_it_is() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let at = stack.take(16, 8).unwrap();
        /* SAFETY: the 16 bytes just taken, then the 200,000 the value grew by. */
        unsafe {
            std::ptr::write_bytes(at, 7, 16);
            stack.grow(at, 16, 200_000).unwrap();
            assert_eq!(*at.add(15), 7, "growing kept what the value held");
            std::ptr::write_bytes(at.add(16), 9, 200_000);
        }
        assert_eq!(stack.top, at as usize + 200_016);
    }

    #[test]
    #[should_panic(expected = "a value grew on a data stack while another sat above it")]
    fn growing_a_value_below_the_top_is_refused() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let below = stack.take(16, 8).unwrap();
        stack.take(16, 8).unwrap();
        let _ = stack.grow(below, 16, 16);
    }

    #[test]
    fn a_full_stack_says_so_and_takes_values_again_after_a_restore() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let mark = stack.mark();
        let mut taken = 0;
        let full = loop {
            match stack.take(4096, 8) {
                Ok(_) => taken += 4096,
                Err(full) => break full,
            }
        };
        assert_eq!(full, Full::Reservation { reserved: SMALL });
        assert_eq!(taken, SMALL, "the whole reservation was usable");
        stack.restore(mark);
        assert!(stack.take(4096, 8).is_ok());
    }

    #[test]
    fn sizes_that_do_not_fit_are_refused() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let full = Full::Reservation { reserved: SMALL };
        assert_eq!(stack.take(usize::MAX, 1), Err(full));
        assert_eq!(stack.take(usize::MAX - 2, 16), Err(full));
        let at = stack.take(8, 8).unwrap();
        assert_eq!(stack.grow(at, 8, usize::MAX), Err(full));
        assert_eq!(stack.grow(at, 8, SMALL), Err(full));
        assert_eq!(
            stack.take(8, 8).unwrap() as usize,
            at as usize + 8,
            "a refused request leaves the stack as it was"
        );
        assert_eq!(
            Stacks::reserve(MOST, usize::MAX).unwrap_err(),
            Full::Refused
        );
    }

    #[test]
    fn memory_is_committed_as_the_stack_fills_and_given_back_after() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, 256 << 20).unwrap();
        let stack = stacks.stack(0);
        assert_eq!(
            stack.committed, stack.base,
            "nothing is committed before it's used"
        );
        let mark = stack.mark();
        stack.take(100, 8).unwrap();
        assert_eq!(stack.committed - stack.base, STEP);
        stack.take(100 << 20, 64).unwrap();
        assert!(stack.committed - stack.base >= 100 << 20);
        stack.restore(mark);
        assert!(
            stack.committed - stack.top <= TRIM_KEEP + STEP,
            "most of what the big value had went back to the system"
        );
        /* What went back can be committed again. */
        let again = stack.take(150 << 20, 64).unwrap();
        /* SAFETY: the last byte of the value just taken. */
        unsafe { *again.add((150 << 20) - 1) = 1 };
    }

    #[test]
    fn every_byte_committed_is_counted_and_given_back() {
        let _alone = alone();
        let before = committed();
        {
            let mut stacks = Stacks::reserve(3, SMALL).unwrap();
            for index in 0..3 {
                stacks.stack(index).take(1000 * (index + 1), 8).unwrap();
            }
            assert_eq!(committed(), before + 3 * STEP);
        }
        assert_eq!(
            committed(),
            before,
            "dropping the stacks gave everything back"
        );
    }

    #[test]
    fn the_limit_stops_a_stack_before_the_system_does() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, 64 << 20).unwrap();
        let stack = stacks.stack(0);
        let allowed = committed() + (1 << 20);
        limit(allowed);
        let full = loop {
            if let Err(full) = stack.take(STEP, 8) {
                break full;
            }
        };
        limit(usize::MAX);
        assert_eq!(full, Full::Limit { limit: allowed });
        assert!(committed() <= allowed);
    }

    #[test]
    fn checking_writes_over_what_a_scope_frees() {
        let _alone = alone();
        let mut stacks = Stacks::reserve(1, SMALL).unwrap();
        let stack = stacks.stack(0);
        let mark = stack.mark();
        let at = stack.take(32, 8).unwrap();
        /* SAFETY: the 32 bytes just taken, which stay committed after the restore. */
        unsafe { std::ptr::write_bytes(at, 1, 32) };
        set_checking(true);
        stack.restore(mark);
        set_checking(false);
        /* SAFETY: as above. */
        let freed = unsafe { std::slice::from_raw_parts(at, 32) };
        assert!(freed.iter().all(|byte| *byte == FREED));
    }

    #[test]
    fn a_full_stack_names_its_size_and_the_options_that_set_it() {
        let message = Full::Reservation { reserved: 4 << 30 }.to_string();
        assert!(
            message.starts_with("a data stack is full at 4 GiB;"),
            "{message}"
        );
        assert!(message.contains("--stack-reserve") && message.contains("--task-stack-reserve"));
        assert!(
            Full::Reservation { reserved: 64 << 20 }
                .to_string()
                .contains("full at 64 MiB;")
        );
        assert!(
            Full::Reservation { reserved: 1000 }
                .to_string()
                .contains("full at 1000 bytes;")
        );
        assert_eq!(
            Full::Limit { limit: 3 << 10 }.to_string(),
            "the program would hold more than its limit of 3 KiB on its data stacks"
        );
    }
}
