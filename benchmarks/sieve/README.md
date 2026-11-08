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
| 10000000 | 0.12 s | 0.12 s | 0.12 s | 0.16 s | 1.25× |
| 40000000 | 0.58 s | 0.58 s | 0.58 s | 0.67 s | 1.16× |
| 100000000 | 1.42 s | 1.44 s | 1.50 s | 1.70 s | 1.20× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 10000000 | 63 ms to 0.12 s | 0.11 s to 0.14 s | 94 ms to 0.14 s | 94 ms to 0.17 s |
| 40000000 | 0.48 s to 0.62 s | 0.55 s to 0.75 s | 0.53 s to 0.64 s | 0.62 s to 0.84 s |
| 100000000 | 1.38 s to 1.47 s | 1.41 s to 1.64 s | 1.42 s to 1.61 s | 1.62 s to 1.80 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10000000 | 12.2 MB | 12.4 MB | 12.4 MB | 19.7 MB | 1.62× |
| 40000000 | 40.8 MB | 41.0 MB | 41.0 MB | 67.8 MB | 1.66× |
| 100000000 | 98.0 MB | 98.3 MB | 98.3 MB | 131.8 MB | 1.34× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 10000000 | 0.14 s | 0.15 s | 0.15 s | 0.16 s | 1.16× |
| 40000000 | 0.71 s | 0.75 s | 0.66 s | 0.75 s | 1.06× |
| 100000000 | 1.44 s | 1.43 s | 1.53 s | 1.72 s | 1.20× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.12 s | 0.87 s | 1.15 s | 1.20 s |
| Executable | 148 KB | 136 KB | 136 KB | 303 KB |

At the largest input, Sayon took 1.20× the processor time of the faster of C++ and Rust, and held 1.34× the memory of the leaner one. With its overflow checks on, Rust took 1.04× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
