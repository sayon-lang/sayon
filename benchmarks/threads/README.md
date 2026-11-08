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
| 1000000 | 0.25 s | 0.23 s | 0.80 s | 0.59 s | 2.53× |
| 4000000 | 1.06 s | 1.08 s | 3.17 s | 2.55 s | 2.40× |
| 10000000 | 2.55 s | 2.56 s | 8.11 s | 6.03 s | 2.37× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0.19 s to 0.30 s | 0.22 s to 0.28 s | 0.75 s to 0.83 s | 0.55 s to 0.62 s |
| 4000000 | 1.02 s to 1.19 s | 0.98 s to 1.14 s | 3.09 s to 3.38 s | 2.42 s to 2.67 s |
| 10000000 | 2.41 s to 2.69 s | 2.41 s to 2.70 s | 7.53 s to 8.33 s | 5.95 s to 6.19 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.19× |
| 4000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |
| 10000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.19× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 76 ms | 76 ms | 0.21 s | 0.17 s | 2.26× |
| 4000000 | 0.32 s | 0.31 s | 0.94 s | 0.77 s | 2.46× |
| 10000000 | 0.79 s | 0.80 s | 2.61 s | 1.88 s | 2.38× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.31 s | 6.21 s | 1.86 s | 1.51 s |
| Executable | 174 KB | 161 KB | 162 KB | 398 KB |

At the largest input, Sayon took 2.37× the processor time of the faster of C++ and Rust, and held 1.19× the memory of the leaner one. With its overflow checks on, Rust took 3.16× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
