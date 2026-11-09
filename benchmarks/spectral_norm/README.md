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
| 1000 | 31 ms | 31 ms | 47 ms | 47 ms | too short to tell |
| 2000 | 0.12 s | 0.11 s | 0.12 s | 0.14 s | 1.29× |
| 4000 | 0.42 s | 0.34 s | 0.48 s | 0.59 s | 1.73× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000 | 0 ms to 63 ms | 0 ms to 47 ms | 0 ms to 0.11 s | 16 ms to 94 ms |
| 2000 | 47 ms to 0.16 s | 78 ms to 0.12 s | 0.11 s to 0.16 s | 0.11 s to 0.17 s |
| 4000 | 0.39 s to 0.56 s | 0.28 s to 0.45 s | 0.39 s to 0.55 s | 0.48 s to 0.69 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 2000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 4000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 0.27 s | 0.17 s | 0.21 s | 0.28 s | 1.68× |
| 2000 | 0.61 s | 0.55 s | 0.69 s | 0.83 s | 1.50× |
| 4000 | 2.37 s | 2.13 s | 2.67 s | 3.51 s | 1.65× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.62 s | 1.50 s | 1.15 s | 0.95 s |
| Executable | 149 KB | 160 KB | 159 KB | 308 KB |

At the largest input, Sayon took 1.73× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.41× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
