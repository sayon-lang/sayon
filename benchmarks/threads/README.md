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
| 1000000 | 0.16 s | 94 ms | 0.39 s | 0.31 s | 3.33× |
| 4000000 | 0.67 s | 0.69 s | 2.56 s | 1.98 s | 2.95× |
| 10000000 | 2.03 s | 2.16 s | 6.81 s | 5.39 s | 2.65× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0 ms to 0.27 s | 16 ms to 0.22 s | 0.23 s to 0.53 s | 0.12 s to 0.59 s |
| 4000000 | 0.41 s to 0.73 s | 0.47 s to 0.80 s | 2.45 s to 2.77 s | 1.77 s to 2.12 s |
| 10000000 | 1.95 s to 2.36 s | 1.86 s to 2.34 s | 6.42 s to 7.11 s | 5.09 s to 5.64 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |
| 4000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |
| 10000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.12 s | 0.12 s | 0.34 s | 0.30 s | 2.54× |
| 4000000 | 0.48 s | 0.48 s | 1.41 s | 1.19 s | 2.47× |
| 10000000 | 1.17 s | 1.26 s | 3.66 s | 2.96 s | 2.53× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.95 s | 2.38 s | 2.23 s | 1.05 s |
| Executable | 174 KB | 161 KB | 162 KB | 398 KB |

At the largest input, Sayon took 2.65× the processor time of the faster of C++ and Rust, and held 1.18× the memory of the leaner one. With its overflow checks on, Rust took 3.16× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
