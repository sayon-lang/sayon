# fannkuch

Take a stack of pancakes numbered 1 to n. Look at the top one: if it says k, flip the top k pancakes over. Keep going until pancake 1 is on top, and count the flips. This program does that for every order the stack can start in, all n! of them, and reports the most flips any order needed, along with a checksum of all the counts. It runs with stacks of 10 and of 11 pancakes: 3,628,800 orders, then 39,916,800.

It's the classic fannkuch-redux benchmark, on one thread. Almost all the time goes into small integer arrays: copying one, reversing the front of it, and stepping to the next order.

## What's the same in all three

The versions visit the orders in the same sequence, flip with the same swap loop, and print the same checksum and maximum. Each keeps the stack in an array or vector of plain integers.

## What differs

Sayon checks every array index and every integer operation. C++ checks neither, and Rust checks indexes but not overflow in a release build. The arrays here are short and the checks predictable, so they cost little, and the three come out about even.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10 | 94 ms | 0.12 s | 94 ms | 94 ms | 1.00× |
| 11 | 1.30 s | 1.38 s | 1.27 s | 1.44 s | 1.11× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 10 | 31 ms to 0.11 s | 94 ms to 0.16 s | 63 ms to 0.14 s | 47 ms to 0.17 s |
| 11 | 1.14 s to 1.41 s | 1.14 s to 1.66 s | 1.14 s to 1.41 s | 1.23 s to 1.55 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 11 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10 | 0.57 s | 0.58 s | 0.58 s | 0.65 s | 1.14× |
| 11 | 6.99 s | 7.32 s | 7.02 s | 8.14 s | 1.16× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 3.73 s | 1.62 s | 1.30 s | 1.02 s |
| Executable | 150 KB | 137 KB | 138 KB | 305 KB |

At the largest input, Sayon took 1.11× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 0.92× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
