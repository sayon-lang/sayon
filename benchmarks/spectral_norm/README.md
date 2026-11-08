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
| 1000 | 94 ms | 94 ms | 0.11 s | 0.14 s | 1.50× |
| 2000 | 0.38 s | 0.34 s | 0.41 s | 0.50 s | 1.45× |
| 4000 | 1.44 s | 1.30 s | 1.64 s | 1.95 s | 1.51× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000 | 94 ms to 0.11 s | 63 ms to 94 ms | 94 ms to 0.12 s | 0.12 s to 0.16 s |
| 2000 | 0.36 s to 0.39 s | 0.33 s to 0.42 s | 0.38 s to 0.44 s | 0.47 s to 0.52 s |
| 4000 | 1.41 s to 1.48 s | 1.27 s to 1.50 s | 1.61 s to 1.73 s | 1.94 s to 2.02 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 2000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| 4000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 0.10 s | 92 ms | 0.11 s | 0.13 s | 1.45× |
| 2000 | 0.39 s | 0.34 s | 0.42 s | 0.51 s | 1.49× |
| 4000 | 1.46 s | 1.32 s | 1.67 s | 1.99 s | 1.51× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.65 s | 0.94 s | 0.80 s | 0.52 s |
| Executable | 149 KB | 160 KB | 159 KB | 308 KB |

At the largest input, Sayon took 1.51× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.27× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
