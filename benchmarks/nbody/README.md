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
| 1000000 | 63 ms | 63 ms | 31 ms | 63 ms | 1.00× |
| 4000000 | 0.28 s | 0.12 s | 0.16 s | 0.28 s | 2.25× |
| 12000000 | 0.81 s | 0.42 s | 0.39 s | 0.81 s | 1.93× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 31 ms to 0.14 s | 16 ms to 78 ms | 16 ms to 63 ms | 31 ms to 0.11 s |
| 4000000 | 0.23 s to 0.42 s | 0.11 s to 0.19 s | 0.11 s to 0.20 s | 0.17 s to 0.34 s |
| 12000000 | 0.69 s to 0.94 s | 0.33 s to 0.47 s | 0.36 s to 0.53 s | 0.64 s to 0.97 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 4000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 12000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.17 s | 85 ms | 85 ms | 0.17 s | 1.98× |
| 4000000 | 0.64 s | 0.31 s | 0.30 s | 0.62 s | 1.98× |
| 12000000 | 1.84 s | 0.89 s | 0.88 s | 1.81 s | 2.02× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.80 s | 1.73 s | 1.38 s | 1.47 s |
| Executable | 149 KB | 162 KB | 162 KB | 309 KB |

At the largest input, Sayon took 1.93× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 0.93× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
