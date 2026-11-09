# sieve

Counts the primes below a limit with the sieve of Eratosthenes, crossing out the multiples of each prime in an array of one byte per number, and prints how many primes there are and the largest. The limits are 10, 40, and 100 million, so the largest array is 100 MB, walked through again and again.

## What's the same in all three

The same algorithm, with a byte for each number: `std::vector<unsigned char>` in C++, rather than `std::vector<bool>`, which packs eight numbers to a byte, `Vec<bool>` in Rust, and `Array<boolean>` in Sayon.

## What differs

Making the array. C++ and Rust make an array of a given size in one step: C++ fills it with zeros, and Rust asks the system for memory that is already zero. Sayon can't make an array of a given size yet, so its program pushes a `false` for each number, and the array grows by doubling, copying what it holds each time. That costs time, and while it grows, the old array and the new one can both be in memory.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10000000 | 94 ms | 0.11 s | 94 ms | 78 ms | 0.83× |
| 40000000 | 0.39 s | 0.45 s | 0.44 s | 0.42 s | 1.08× |
| 100000000 | 1.06 s | 1.05 s | 1.08 s | 1.23 s | 1.18× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 10000000 | 31 ms to 0.14 s | 16 ms to 0.14 s | 63 ms to 0.14 s | 47 ms to 0.16 s |
| 40000000 | 0.20 s to 0.58 s | 0.20 s to 0.58 s | 0.30 s to 0.56 s | 0.33 s to 0.67 s |
| 100000000 | 0.88 s to 1.30 s | 0.80 s to 1.14 s | 0.92 s to 1.17 s | 1.14 s to 1.31 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10000000 | 12.2 MB | 12.4 MB | 12.4 MB | 19.8 MB | 1.62× |
| 40000000 | 40.8 MB | 41.0 MB | 41.0 MB | 67.8 MB | 1.66× |
| 100000000 | 98.0 MB | 98.3 MB | 98.3 MB | 131.8 MB | 1.34× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10000000 | 0.21 s | 0.21 s | 0.21 s | 0.25 s | 1.20× |
| 40000000 | 0.92 s | 0.88 s | 0.93 s | 1.05 s | 1.18× |
| 100000000 | 2.39 s | 2.41 s | 2.36 s | 2.71 s | 1.13× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.02 s | 1.31 s | 1.33 s | 0.65 s |
| Executable | 148 KB | 136 KB | 136 KB | 303 KB |

At the largest input, Sayon took 1.18× the processor time of the faster of C++ and Rust, and held 1.34× the memory of the leaner one. With its overflow checks on, Rust took 1.03× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
