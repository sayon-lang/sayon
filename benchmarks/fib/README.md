# fib

Works out a Fibonacci number the slow way, with a function that calls itself for the two numbers before it, and prints it. Fibonacci 34, 37, and 40 take about 18 million, 78 million, and 330 million calls. It measures what a function call costs.

## What's the same in all three

The same function, written the plain way in each language, on 64-bit integers.

## What differs

How a call hands back its result. C++ and Rust return the number in a register. A Sayon function returns whether it raised an error, and writes its result through a pointer, so that an error can travel up through every call, and each call does a little more for that. Sayon also checks every addition and subtraction for overflow; the checked Rust build shows what such checks cost Rust.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 34 | 0 ms | 0 ms | 16 ms | 16 ms | too short to tell |
| 37 | 47 ms | 31 ms | 47 ms | 94 ms | too short to tell |
| 40 | 0.12 s | 0.12 s | 0.23 s | 0.38 s | 3.00× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 34 | 0 ms to 16 ms | 0 ms to 16 ms | 0 ms to 47 ms | 0 ms to 63 ms |
| 37 | 16 ms to 63 ms | 16 ms to 63 ms | 31 ms to 63 ms | 78 ms to 0.16 s |
| 40 | 78 ms to 0.19 s | 94 ms to 0.16 s | 0.17 s to 0.25 s | 0.33 s to 0.42 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 34 | 3.1 MB | 3.4 MB | 3.3 MB | 3.2 MB | 1.05× |
| 37 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 40 | 3.1 MB | 3.3 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 34 | 59 ms | 61 ms | 84 ms | 0.13 s | 2.20× |
| 37 | 0.20 s | 0.18 s | 0.29 s | 0.49 s | 2.65× |
| 40 | 0.79 s | 0.73 s | 1.13 s | 1.98 s | 2.69× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 0.69 s | 1.14 s | 0.95 s | 0.82 s |
| Executable | 145 KB | 135 KB | 135 KB | 301 KB |

At the largest input, Sayon took 3.00× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.88× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
