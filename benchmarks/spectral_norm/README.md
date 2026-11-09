# spectral_norm

Works out the largest singular value of a particular infinite matrix, using the power method on its top-left n by n corner, for n of 1,000, 2,000, and 4,000. Each matrix entry is computed on the fly from its row and column with a small formula, so the inner loop calls a tiny function millions of times.

## What's the same in all three

Ten rounds of multiplying a vector by the matrix and by its transpose, with the entries computed by the same formula, and the result printed to nine decimal places.

## What differs

The entry formula does integer arithmetic, `(i + j) * (i + j + 1) / 2 + i + 1`. Sayon checks each of those operations for overflow, and the checks keep LLVM from turning the inner loop into vector instructions. C++ and Rust release builds don't check, so their compilers can.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 47 ms | 63 ms | 63 ms | 78 ms | too short to tell |
| 2000 | 0.25 s | 0.20 s | 0.25 s | 0.33 s | 1.62× |
| 4000 | 0.91 s | 0.86 s | 1.05 s | 1.17 s | 1.36× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000 | 16 ms to 94 ms | 0 ms to 0.11 s | 47 ms to 94 ms | 47 ms to 94 ms |
| 2000 | 0.16 s to 0.30 s | 0.16 s to 0.27 s | 0.22 s to 0.38 s | 0.20 s to 0.39 s |
| 4000 | 0.77 s to 1.02 s | 0.67 s to 0.95 s | 0.86 s to 1.16 s | 1.08 s to 1.36 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 2000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 4000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.06× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 0.14 s | 0.13 s | 0.15 s | 0.18 s | 1.46× |
| 2000 | 0.51 s | 0.45 s | 0.57 s | 0.68 s | 1.50× |
| 4000 | 1.96 s | 1.75 s | 2.23 s | 2.68 s | 1.53× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.30 s | 1.45 s | 1.24 s | 0.77 s |
| Executable | 149 KB | 160 KB | 159 KB | 308 KB |

At the largest input, Sayon took 1.36× the processor time of the faster of C++ and Rust, and held 1.06× the memory of the leaner one. With its overflow checks on, Rust took 1.22× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
