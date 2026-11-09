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
| 10 | 0.20 s | 0.14 s | 0.20 s | 0.20 s | 1.44× |
| 11 | 2.45 s | 2.44 s | 2.20 s | 2.58 s | 1.06× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 10 | 0.14 s to 0.23 s | 0.11 s to 0.22 s | 0.14 s to 0.22 s | 94 ms to 0.33 s |
| 11 | 2.20 s to 2.52 s | 2.38 s to 2.64 s | 2.00 s to 2.27 s | 2.45 s to 2.78 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 11 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10 | 0.42 s | 0.44 s | 0.43 s | 0.49 s | 1.14× |
| 11 | 4.85 s | 5.16 s | 5.04 s | 5.69 s | 1.17× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 8.11 s | 1.52 s | 1.52 s | 1.25 s |
| Executable | 150 KB | 137 KB | 138 KB | 305 KB |

At the largest input, Sayon took 1.06× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 0.90× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
