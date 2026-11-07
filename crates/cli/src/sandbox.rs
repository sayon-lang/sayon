//! `sayon run --sandbox`: what keeps a program from anyone from taking more
//! of the computer than its share. The checker and the interpreter keep it
//! from what the sandbox does not give it (`sayon_target::SANDBOX`); this
//! keeps its time and its memory. A watchdog thread stops it after
//! [`PROCESSOR_SECONDS`] of the processor or [`WALL_SECONDS`] in all, and
//! the allocator, which counts what is allocated, past [`MEMORY`]; the
//! interpreter stops it past [`OUTPUT`] written. The checking counts, too.
//!
//! The operating system backs these, a little past them, in case a mistake
//! here lets a program through: on Windows a job object, which also starts
//! no other process; on Linux a limit on processor time.

use crate::status;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

pub const PROCESSOR_SECONDS: u64 = 5;
pub const WALL_SECONDS: u64 = 10;
pub const MEMORY: usize = 256 << 20;
pub const OUTPUT: usize = 64 << 10;

/// What is allocated now, and how much may be: everything but in the
/// sandbox.
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
/// Set once the program is being stopped, so that what stopping it
/// allocates is no longer counted.
static STOPPING: AtomicBool = AtomicBool::new(false);

/// The program's allocator: the system's, counting what it gives.
pub struct Counting;

impl Counting {
    /// Stops the program when `more` bytes would take it past the limit.
    fn admit(more: usize) {
        if !STOPPING.load(Ordering::Relaxed)
            && ALLOCATED.load(Ordering::Relaxed).saturating_add(more)
                > LIMIT.load(Ordering::Relaxed)
        {
            stop(&format!("it used more than {} MiB of memory", MEMORY >> 20));
        }
    }
}

// SAFETY: each call passes its arguments on to the system's allocator, as
// they came, and only counts the sizes.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Counting::admit(layout.size());
        // SAFETY: the caller's layout, unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Counting::admit(layout.size());
        // SAFETY: the caller's layout, unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: a block this allocator gave, with its layout.
        unsafe { System.dealloc(pointer, layout) };
        ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if size > layout.size() {
            Counting::admit(size - layout.size());
        }
        // SAFETY: a block this allocator gave, with its layout.
        let moved = unsafe { System.realloc(pointer, layout, size) };
        if !moved.is_null() {
            if size > layout.size() {
                ALLOCATED.fetch_add(size - layout.size(), Ordering::Relaxed);
            } else {
                ALLOCATED.fetch_sub(layout.size() - size, Ordering::Relaxed);
            }
        }
        moved
    }
}

/// Puts this process in the sandbox's limits, from now on: what checks and
/// runs the program in it counts.
pub fn enter() {
    LIMIT.store(
        ALLOCATED.load(Ordering::Relaxed).saturating_add(MEMORY),
        Ordering::Relaxed,
    );
    os::back_up();
    let started = Instant::now();
    let watching = std::thread::Builder::new()
        .name("sandbox".into())
        .spawn(move || {
            loop {
                std::thread::sleep(Duration::from_millis(20));
                if started.elapsed() >= Duration::from_secs(WALL_SECONDS) {
                    stop(&format!("it ran for {WALL_SECONDS} seconds"));
                }
                if os::processor_time() >= Duration::from_secs(PROCESSOR_SECONDS) {
                    stop(&format!(
                        "it used {PROCESSOR_SECONDS} seconds of processor time"
                    ));
                }
            }
        });
    if watching.is_err() {
        stop("the sandbox could not watch it");
    }
}

/// Says that the sandbox stopped the program, and why; the status to exit
/// with.
pub fn stopped(reason: &str) -> u8 {
    eprintln!("the sandbox stopped the program: {reason}");
    status::FATAL
}

/// Stops the program at once, from wherever it is, saying why.
pub fn stop(reason: &str) -> ! {
    if STOPPING.swap(true, Ordering::SeqCst) {
        // Another thread is stopping it already.
        loop {
            std::thread::park();
        }
    }
    let code = stopped(reason);
    std::process::exit(code as i32)
}

#[cfg(windows)]
mod os {
    use super::{MEMORY, PROCESSOR_SECONDS};
    use std::ffi::c_void;
    use std::time::Duration;

    const JOB_OBJECT_EXTENDED_LIMIT_INFORMATION: u32 = 9;
    const JOB_OBJECT_LIMIT_PROCESS_TIME: u32 = 0x0002;
    const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: u32 = 0x0008;
    const JOB_OBJECT_LIMIT_PROCESS_MEMORY: u32 = 0x0100;
    const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: u32 = 0x0400;

    /// `JOBOBJECT_BASIC_LIMIT_INFORMATION`.
    #[repr(C)]
    #[derive(Default)]
    struct BasicLimits {
        process_time: i64,
        job_time: i64,
        flags: u32,
        smallest_working_set: usize,
        largest_working_set: usize,
        active_processes: u32,
        affinity: usize,
        priority_class: u32,
        scheduling_class: u32,
    }

    /// `JOBOBJECT_EXTENDED_LIMIT_INFORMATION`, with its `IO_COUNTERS`.
    #[repr(C)]
    #[derive(Default)]
    struct ExtendedLimits {
        basic: BasicLimits,
        counters: [u64; 6],
        process_memory: usize,
        job_memory: usize,
        peak_process_memory: usize,
        peak_job_memory: usize,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut c_void;
        fn GetProcessTimes(
            process: *mut c_void,
            creation: *mut u64,
            exit: *mut u64,
            kernel: *mut u64,
            user: *mut u64,
        ) -> i32;
        fn CreateJobObjectW(attributes: *const c_void, name: *const u16) -> *mut c_void;
        fn SetInformationJobObject(
            job: *mut c_void,
            class: u32,
            information: *const c_void,
            length: u32,
        ) -> i32;
        fn AssignProcessToJobObject(job: *mut c_void, process: *mut c_void) -> i32;
    }

    /// The processor time this process has used, in its own code and the
    /// system's for it.
    pub fn processor_time() -> Duration {
        let (mut creation, mut exit, mut kernel, mut user) = (0u64, 0u64, 0u64, 0u64);
        // SAFETY: four times to fill, for this process's pseudo-handle.
        let read = unsafe {
            GetProcessTimes(
                GetCurrentProcess(),
                &mut creation,
                &mut exit,
                &mut kernel,
                &mut user,
            )
        };
        if read == 0 {
            return Duration::ZERO;
        }
        // In units of 100 nanoseconds.
        Duration::from_nanos((kernel + user).saturating_mul(100))
    }

    /// A job for this process, which ends it past twice the processor time
    /// and four times the memory, starts no other process, and shows no
    /// window when the process fails. Without one the watchdog still stops
    /// the program.
    pub fn back_up() {
        let mut limits = ExtendedLimits::default();
        limits.basic.flags = JOB_OBJECT_LIMIT_PROCESS_TIME
            | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
            | JOB_OBJECT_LIMIT_PROCESS_MEMORY
            | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION;
        limits.basic.process_time = (2 * PROCESSOR_SECONDS as i64) * 10_000_000;
        limits.basic.active_processes = 1;
        limits.process_memory = 4 * MEMORY;
        // SAFETY: a new, unnamed job, given limits of the size its class
        // takes, then this process's pseudo-handle.
        unsafe {
            let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
            if job.is_null() {
                return;
            }
            let set = SetInformationJobObject(
                job,
                JOB_OBJECT_EXTENDED_LIMIT_INFORMATION,
                &limits as *const ExtendedLimits as *const c_void,
                std::mem::size_of::<ExtendedLimits>() as u32,
            );
            if set != 0 {
                AssignProcessToJobObject(job, GetCurrentProcess());
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod os {
    use super::PROCESSOR_SECONDS;
    use std::time::Duration;

    const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
    const RLIMIT_CPU: i32 = 0;

    #[repr(C)]
    struct Timespec {
        seconds: i64,
        nanoseconds: i64,
    }

    #[repr(C)]
    struct Limit {
        current: u64,
        maximum: u64,
    }

    unsafe extern "C" {
        fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
        fn setrlimit(resource: i32, limit: *const Limit) -> i32;
    }

    /// The processor time this process has used.
    pub fn processor_time() -> Duration {
        let mut time = Timespec {
            seconds: 0,
            nanoseconds: 0,
        };
        // SAFETY: a time to fill.
        if unsafe { clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut time) } != 0 {
            return Duration::ZERO;
        }
        Duration::new(time.seconds as u64, time.nanoseconds as u32)
    }

    /// A limit on processor time, past which the system ends the process:
    /// twice the sandbox's.
    pub fn back_up() {
        let limit = Limit {
            current: 2 * PROCESSOR_SECONDS,
            maximum: 2 * PROCESSOR_SECONDS + 1,
        };
        // SAFETY: a limit to set.
        unsafe { setrlimit(RLIMIT_CPU, &limit) };
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
mod os {
    use std::time::Duration;

    /// Not measured here: the wall's limit stops the program.
    pub fn processor_time() -> Duration {
        Duration::ZERO
    }

    pub fn back_up() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The processor time grows with work, however busy the computer is
    /// with other things: work goes on until it has grown, or a minute.
    #[cfg(any(windows, target_os = "linux"))]
    #[test]
    fn processor_time_counts_work() {
        let before = os::processor_time();
        let started = Instant::now();
        let mut total: u64 = 0;
        while os::processor_time() < before + Duration::from_millis(100) {
            for _ in 0..100_000 {
                total = total.wrapping_add(std::hint::black_box(1));
            }
            assert!(
                started.elapsed() < Duration::from_secs(60),
                "the processor time stayed {:?} ({total})",
                os::processor_time()
            );
        }
    }

    #[test]
    fn what_is_allocated_is_counted() {
        let before = ALLOCATED.load(Ordering::Relaxed);
        let block = std::hint::black_box(vec![0u8; 1 << 20]);
        assert!(ALLOCATED.load(Ordering::Relaxed) >= before + (1 << 20));
        drop(block);
    }
}
