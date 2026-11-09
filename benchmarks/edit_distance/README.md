# edit_distance

Works out the edit distance, the fewest one-letter insertions, deletions, and changes that turn one word into another, for twenty pairs of pseudo-random words over a four-letter alphabet. The words are 500, 1,000, and 2,000 letters long, so the largest input fills in 80 million cells of the classic table.

## What's the same in all three

The same dynamic programming over two rows of 64-bit integers, the row above and the row being filled in, which swap places after each row. The letters are bytes, and the words come from the same generator as the other programs, a byte for each letter.

## What differs

Swapping the rows: C++ uses `std::swap` and Rust `std::mem::swap`, and Sayon moves the two arrays through a third variable. None of them copies a row.

Sayon checks every index and every addition. Rust checks indexes it can't prove are in range, and its additions only in its checked build. C++ checks neither.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 500 | 47 ms | 47 ms | 31 ms | 31 ms | too short to tell |
| 1000 | 0.16 s | 0.12 s | 78 ms | 94 ms | 0.75× |
| 2000 | 0.56 s | 0.44 s | 0.28 s | 0.31 s | 0.71× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 500 | 47 ms to 47 ms | 31 ms to 78 ms | 16 ms to 31 ms | 31 ms to 63 ms |
| 1000 | 0.12 s to 0.17 s | 0.11 s to 0.14 s | 63 ms to 0.11 s | 63 ms to 0.12 s |
| 2000 | 0.55 s to 0.61 s | 0.44 s to 0.50 s | 0.25 s to 0.33 s | 0.30 s to 0.42 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 500 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.07× |
| 1000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.4 MB | 1.08× |
| 2000 | 3.1 MB | 3.6 MB | 3.7 MB | 3.5 MB | 1.13× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 500 | 58 ms | 43 ms | 32 ms | 34 ms | 0.81× |
| 1000 | 0.15 s | 0.13 s | 93 ms | 0.11 s | 0.88× |
| 2000 | 0.57 s | 0.45 s | 0.29 s | 0.32 s | 0.73× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.27 s | 0.71 s | 0.74 s | 0.48 s |
| Executable | 150 KB | 138 KB | 138 KB | 305 KB |

At the largest input, Sayon was the fastest, in 0.71× the processor time of the faster of C++ and Rust, and held 1.13× the memory of the leaner one. With its overflow checks on, Rust took 0.64× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
