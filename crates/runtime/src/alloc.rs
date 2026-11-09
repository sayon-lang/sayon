/*!
Sayon's memory allocator: what the values of a Sayon program, and the
`sayon` tool itself, get their memory from.

Small blocks, up to 1 KiB, come in size classes. Each class keeps a list
of its free blocks, so allocating takes the first block off the list and
freeing puts it back, under a lock held for a few instructions and no
call to the system. A list that runs dry is refilled from a span of
64 KiB, which is cut into blocks of the class by moving a pointer along
it. A block carries no header, since Rust says how big a block is when it
frees it, and the size names the class. So a block of 48 bytes takes 48
bytes, where the system's allocator adds a header and rounds up. Larger
blocks, the spans among them, come from the system.

Memory freed into a class stays with the class for the next block of its
size; only large blocks go back to the system.

The sandbox (`sayon run --sandbox`) counts what a program holds and stops
it past a limit: see [`limit`].
*/

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::UnsafeCell;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

/** The largest block a class holds; larger ones come from the system. */
const LARGEST: usize = 1024;
/** The alignment every block of a class has. */
const ALIGN: usize = 16;
/** The memory a class takes from the system at a time. */
const SPAN: usize = 64 * 1024;

/**
The sizes of the classes: steps of 16 bytes up to 128, then of 32, 64,
and 128, so that no block is more than a quarter bigger than asked for.
*/
const SIZES: [usize; 20] = [
    16, 32, 48, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 448, 512, 640, 768, 896, 1024,
];

/** The class of a block of `size` bytes, from 1 to `LARGEST`. */
#[inline]
fn class(size: usize) -> usize {
    match size {
        0..=128 => (size.max(1) - 1) / 16,
        129..=256 => 8 + (size - 129) / 32,
        257..=512 => 12 + (size - 257) / 64,
        _ => 16 + (size - 513) / 128,
    }
}

/** A free block, which holds the next free block of its class. */
struct Free {
    next: *mut Free,
}

/** A class's free blocks, and the rest of its newest span. */
struct Pool {
    free: *mut Free,
    next: *mut u8,
    end: *mut u8,
}

struct Class {
    taken: AtomicBool,
    pool: UnsafeCell<Pool>,
}

/* SAFETY: a class's pool is only touched while its lock is held. */
unsafe impl Sync for Class {}

impl Class {
    const fn new() -> Class {
        Class {
            taken: AtomicBool::new(false),
            pool: UnsafeCell::new(Pool {
                free: std::ptr::null_mut(),
                next: std::ptr::null_mut(),
                end: std::ptr::null_mut(),
            }),
        }
    }

    /**
    Runs `f` on the pool while holding its lock. The lock is held for a
    few instructions, so a thread that finds it taken spins; but the
    thread holding it may have been put aside by the system, so after a
    while the waiting thread gives up its turn instead.
    */
    #[inline]
    fn with<T>(&self, f: impl FnOnce(&mut Pool) -> T) -> T {
        let mut spins = 0;
        while self
            .taken
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.taken.load(Ordering::Relaxed) {
                if spins < 100 {
                    spins += 1;
                    std::hint::spin_loop();
                } else {
                    std::thread::yield_now();
                }
            }
        }
        /* SAFETY: the lock is held, so this is the only reference. */
        let result = f(unsafe { &mut *self.pool.get() });
        self.taken.store(false, Ordering::Release);
        result
    }
}

static CLASSES: [Class; SIZES.len()] = [const { Class::new() }; SIZES.len()];

/**
Whether what the program holds is counted, which it is only in the
sandbox, so that other programs don't pay for counting.
*/
static COUNTING: AtomicBool = AtomicBool::new(false);
/**
What the program holds beyond what it held when counting began. Freeing
blocks from before then takes it below zero, which only leaves more room.
*/
static HELD: AtomicIsize = AtomicIsize::new(0);
static LIMIT: AtomicIsize = AtomicIsize::new(isize::MAX);
/** What the sandbox does when the limit is passed: stop the program. */
static ON_LIMIT: OnceLock<fn() -> !> = OnceLock::new();

/**
Counts what the program holds from now on, and calls `stop` when it would
hold more than `bytes` beyond what it holds now. Counting stops then, so
that what stopping the program allocates is not counted against it.
*/
pub fn limit(bytes: usize, stop: fn() -> !) {
    let _ = ON_LIMIT.set(stop);
    HELD.store(0, Ordering::Relaxed);
    LIMIT.store(bytes.min(isize::MAX as usize) as isize, Ordering::Relaxed);
    COUNTING.store(true, Ordering::Release);
}

/** Stops counting, as when the program is stopped for another reason. */
pub fn unlimit() {
    COUNTING.store(false, Ordering::Relaxed);
}

/** The bytes the program holds beyond what it held when counting began. */
pub fn held() -> isize {
    HELD.load(Ordering::Relaxed)
}

/** Counts `size` more bytes held, stopping the program past the limit. */
#[inline]
fn admit(size: usize) {
    if COUNTING.load(Ordering::Relaxed) {
        count(size as isize);
    }
}

#[cold]
fn count(size: isize) {
    let held = HELD.fetch_add(size, Ordering::Relaxed) + size;
    if held > LIMIT.load(Ordering::Relaxed)
        && COUNTING.swap(false, Ordering::Relaxed)
        && let Some(stop) = ON_LIMIT.get()
    {
        stop();
    }
}

#[inline]
fn release(size: usize) {
    if COUNTING.load(Ordering::Relaxed) {
        HELD.fetch_sub(size as isize, Ordering::Relaxed);
    }
}

/** Whether a block of `layout` comes from a class. */
#[inline]
fn pooled(layout: Layout) -> bool {
    layout.size() <= LARGEST && layout.align() <= ALIGN
}

/** The allocator. */
pub struct Allocator;

impl Allocator {
    /** A block of class `index`: one off its list, or the next from its span. */
    #[inline]
    unsafe fn take(index: usize) -> *mut u8 {
        let size = SIZES[index];
        CLASSES[index].with(|pool| {
            if !pool.free.is_null() {
                let block = pool.free;
                /* SAFETY: a block on the list holds the next one. */
                pool.free = unsafe { (*block).next };
                return block as *mut u8;
            }
            if pool.next.is_null() || (pool.end as usize) - (pool.next as usize) < size {
                /* SAFETY: a span's layout is valid. */
                let span = unsafe { System.alloc(Layout::from_size_align_unchecked(SPAN, ALIGN)) };
                if span.is_null() {
                    return std::ptr::null_mut();
                }
                pool.next = span;
                /* SAFETY: within the span just made. */
                pool.end = unsafe { span.add(SPAN) };
            }
            let block = pool.next;
            /* SAFETY: there is room for the block in the span. */
            pool.next = unsafe { pool.next.add(size) };
            block
        })
    }

    /** Puts a block of class `index` back on its list. */
    #[inline]
    unsafe fn give(index: usize, block: *mut u8) {
        CLASSES[index].with(|pool| {
            let free = block as *mut Free;
            /* SAFETY: the block is free and big enough to hold a pointer. */
            unsafe { (*free).next = pool.free };
            pool.free = free;
        });
    }
}

/** Counts the change from `old` bytes held to `new`. */
#[inline]
fn resize(old: usize, new: usize) {
    if new > old {
        admit(new - old);
    } else {
        release(old - new);
    }
}

/*
SAFETY: a block from a class is `SIZES[class]` bytes, at least what was
asked for, aligned to `ALIGN`, at least what was asked for, and no other
block overlaps it until it is freed; other blocks come from the system as
asked.
*/
unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        admit(layout.size());
        let block = if pooled(layout) {
            /* SAFETY: the class of the block's size. */
            unsafe { Allocator::take(class(layout.size())) }
        } else {
            /* SAFETY: the caller's layout, unchanged. */
            unsafe { System.alloc(layout) }
        };
        if block.is_null() {
            release(layout.size());
        }
        block
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if pooled(layout) {
            /* SAFETY: as `alloc`; a block from a class may hold old bytes. */
            unsafe {
                let block = self.alloc(layout);
                if !block.is_null() {
                    std::ptr::write_bytes(block, 0, layout.size());
                }
                block
            }
        } else {
            admit(layout.size());
            /* SAFETY: the caller's layout, unchanged. */
            let block = unsafe { System.alloc_zeroed(layout) };
            if block.is_null() {
                release(layout.size());
            }
            block
        }
    }

    #[inline]
    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        release(layout.size());
        if pooled(layout) {
            /* SAFETY: a block of this class, which this allocator gave. */
            unsafe { Allocator::give(class(layout.size()), block) }
        } else {
            /* SAFETY: a block the system gave, with its layout. */
            unsafe { System.dealloc(block, layout) }
        }
    }

    unsafe fn realloc(&self, block: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        /* SAFETY: the caller's block and layout; `size` is not zero. */
        unsafe {
            let new_layout = Layout::from_size_align_unchecked(size, layout.align());
            if pooled(layout) && pooled(new_layout) && class(layout.size()) == class(size) {
                /* Still the same class: the block already has the room. */
                resize(layout.size(), size);
                return block;
            }
            if !pooled(layout) && !pooled(new_layout) {
                resize(layout.size(), size);
                let moved = System.realloc(block, layout, size);
                if moved.is_null() {
                    resize(size, layout.size());
                }
                return moved;
            }
            let moved = self.alloc(new_layout);
            if !moved.is_null() {
                std::ptr::copy_nonoverlapping(block, moved, layout.size().min(size));
                self.dealloc(block, layout);
            }
            moved
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_size_has_a_class_big_enough() {
        let mut previous = 0;
        for size in 1..=LARGEST {
            let index = class(size);
            assert!(
                SIZES[index] >= size,
                "{size} in a class of {}",
                SIZES[index]
            );
            assert!(index >= previous, "classes grow with the size");
            if index > 0 {
                assert!(
                    SIZES[index - 1] < size,
                    "{size} could be in a smaller class"
                );
            }
            previous = index;
        }
    }

    #[test]
    fn blocks_come_back_and_keep_their_alignment() {
        let allocator = Allocator;
        let mut blocks = Vec::new();
        for round in 0..3 {
            for size in [1, 15, 16, 17, 48, 100, 129, 300, 700, 1024, 2000, 70000] {
                let layout = Layout::from_size_align(size, 8).unwrap();
                /* SAFETY: a valid layout; each block is freed once below. */
                let block = unsafe { allocator.alloc(layout) };
                assert!(!block.is_null());
                assert_eq!(block as usize % 8, 0);
                /* SAFETY: the block has `size` bytes. */
                unsafe { std::ptr::write_bytes(block, round as u8, size) };
                blocks.push((block, layout));
            }
            for (block, layout) in blocks.drain(..) {
                /* SAFETY: allocated above with this layout. */
                unsafe { allocator.dealloc(block, layout) };
            }
        }
    }

    #[test]
    fn growing_a_block_keeps_what_it_held() {
        let allocator = Allocator;
        let mut layout = Layout::from_size_align(10, 1).unwrap();
        /* SAFETY: valid layouts, and the block is freed at the end. */
        unsafe {
            let mut block = allocator.alloc(layout);
            for index in 0..10 {
                *block.add(index) = index as u8;
            }
            for size in [12, 40, 200, 1500, 5000, 30] {
                block = allocator.realloc(block, layout, size);
                layout = Layout::from_size_align(size, 1).unwrap();
                for index in 0..10 {
                    assert_eq!(*block.add(index), index as u8);
                }
            }
            allocator.dealloc(block, layout);
        }
    }
}
