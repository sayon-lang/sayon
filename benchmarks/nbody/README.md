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
| 1000000 | 0.12 s | 63 ms | 63 ms | 0.12 s | 2.00× |
| 4000000 | 0.50 s | 0.23 s | 0.23 s | 0.48 s | 2.07× |
| 12000000 | 1.47 s | 0.69 s | 0.69 s | 1.41 s | 2.05× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0.11 s to 0.14 s | 63 ms to 63 ms | 47 ms to 78 ms | 94 ms to 0.14 s |
| 4000000 | 0.48 s to 0.52 s | 0.22 s to 0.30 s | 0.22 s to 0.27 s | 0.45 s to 0.50 s |
| 12000000 | 1.38 s to 1.53 s | 0.66 s to 0.77 s | 0.67 s to 0.84 s | 1.34 s to 1.53 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 4000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 12000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.14 s | 70 ms | 69 ms | 0.14 s | 1.95× |
| 4000000 | 0.51 s | 0.25 s | 0.24 s | 0.50 s | 1.99× |
| 12000000 | 1.57 s | 0.70 s | 0.70 s | 1.47 s | 2.09× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.41 s | 1.24 s | 1.21 s | 1.18 s |
| Executable | 149 KB | 162 KB | 162 KB | 309 KB |

At the largest input, Sayon took 2.05× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
