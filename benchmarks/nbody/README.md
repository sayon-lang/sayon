# nbody

The Sun and the four gas giants pull on each other, and the program moves them forward in small time steps: a million of them, then four million, then twelve million. It prints the system's total energy before and after, to nine decimal places, which also checks that every version did exactly the same arithmetic.

## What's the same in all three

Each body is a structure of seven floats kept in an array, and every step compares each pair of bodies once, the same way, in the same order. No version uses SIMD by hand.

## What differs

Rust walks the pairs with `split_at_mut`, which hands its compiler two references it knows can't point at the same body. So it keeps the first body's position and speed in registers while it goes through the others. C++ and Sayon index the same array twice, and their compilers can't tell that the two bodies are different, so they read those values from memory again after every change. That is a real advantage of Rust's borrowing rules, and most of its lead here.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 47 ms | 16 ms | 16 ms | 31 ms | too short to tell |
| 4000000 | 0.16 s | 78 ms | 47 ms | 0.16 s | 2.00× |
| 12000000 | 0.42 s | 0.22 s | 0.23 s | 0.48 s | 2.21× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0 ms to 63 ms | 0 ms to 47 ms | 0 ms to 31 ms | 0 ms to 63 ms |
| 4000000 | 0.12 s to 0.19 s | 47 ms to 0.14 s | 31 ms to 0.12 s | 47 ms to 0.22 s |
| 12000000 | 0.36 s to 0.52 s | 0.14 s to 0.31 s | 0.17 s to 0.25 s | 0.39 s to 0.52 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 4000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 12000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.22 s | 0.11 s | 0.11 s | 0.21 s | 1.91× |
| 4000000 | 0.80 s | 0.39 s | 0.39 s | 0.81 s | 2.05× |
| 12000000 | 2.40 s | 1.16 s | 1.13 s | 2.38 s | 2.05× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 3.26 s | 1.77 s | 1.25 s | 2.28 s |
| Executable | 149 KB | 162 KB | 162 KB | 309 KB |

At the largest input, Sayon took 2.21× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.07× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
