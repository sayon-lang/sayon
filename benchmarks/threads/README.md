# threads

Follows the Collatz sequence, halving even numbers and taking odd ones to three times themselves plus one, from every number below a limit until it reaches 1. It adds up the steps, and finds the number that takes the most. Four threads share the work, a quarter of the numbers each, for limits of 1, 4, and 10 million.

## What's the same in all three

Four threads from each language's standard library: `std::thread` in C++, `std::thread::spawn` in Rust, and `threads.spawn_parallel` in Sayon. Each gets a quarter of the numbers, and its results come back when it's joined.

## What differs

The checks on `3 * n + 1`. Sayon checks the multiplication and the addition for overflow, as the checked Rust build does, and C++ and Rust's usual build don't. For threads, time on the clock is the number to look at: processor time adds up all four threads, so it doesn't fall when they run at once. The laptop these ran on has two cores, with two hardware threads each.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.23 s | 0.28 s | 0.72 s | 0.58 s | 2.47× |
| 4000000 | 1.03 s | 1.00 s | 2.72 s | 2.41 s | 2.41× |
| 10000000 | 2.86 s | 2.88 s | 8.88 s | 6.75 s | 2.36× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0.19 s to 0.30 s | 0.14 s to 0.30 s | 0.45 s to 0.80 s | 0.53 s to 0.61 s |
| 4000000 | 0.78 s to 1.08 s | 0.84 s to 1.14 s | 2.30 s to 3.23 s | 2.00 s to 2.62 s |
| 10000000 | 2.78 s to 2.92 s | 2.81 s to 2.89 s | 8.80 s to 8.91 s | 6.67 s to 6.91 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |
| 4000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |
| 10000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 77 ms | 77 ms | 0.23 s | 0.18 s | 2.41× |
| 4000000 | 0.31 s | 0.31 s | 0.99 s | 0.74 s | 2.37× |
| 10000000 | 0.76 s | 0.76 s | 2.35 s | 1.77 s | 2.34× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.95 s | 1.12 s | 1.10 s | 0.67 s |
| Executable | 174 KB | 161 KB | 162 KB | 398 KB |

At the largest input, Sayon took 2.36× the processor time of the faster of C++ and Rust, and held 1.18× the memory of the leaner one. With its overflow checks on, Rust took 3.09× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
