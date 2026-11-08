# sort

Fills an array with pseudo-random integers, a million, three million, then ten million of them, and sorts it with the standard library, then prints a few values and a checksum to show the order is right.

## What's the same in all three

The same numbers, and a stable sort from each standard library: `std::stable_sort` in C++, `sort` in Rust, and `collections.sort` in Sayon, which is stable. Comparing against the unstable sorts of C++ and Rust would make the others do less.

## What differs

How the sorts are built. Rust's is driftsort, a carefully tuned merge sort that finds runs already in order and merges without branches. C++'s is Microsoft's `stable_sort`. Sayon's is a plain merge sort written in Sayon: insertion sort for parts of 16 elements or fewer, and merges through a spare array half the input's size. All three take time in proportion to n log n.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.14 s | 63 ms | 63 ms | 0.19 s | 3.00× |
| 3000000 | 0.41 s | 0.19 s | 0.19 s | 0.61 s | 3.25× |
| 10000000 | 1.44 s | 0.69 s | 0.66 s | 2.20 s | 3.20× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0.12 s to 0.14 s | 47 ms to 63 ms | 47 ms to 94 ms | 0.17 s to 0.20 s |
| 3000000 | 0.39 s to 0.48 s | 0.17 s to 0.20 s | 0.17 s to 0.22 s | 0.59 s to 0.62 s |
| 10000000 | 1.44 s to 1.45 s | 0.67 s to 0.70 s | 0.66 s to 0.69 s | 2.19 s to 2.22 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 15.6 MB | 19.2 MB | 19.2 MB | 15.2 MB | 0.97× |
| 3000000 | 40.2 MB | 38.2 MB | 38.2 MB | 38.1 MB | 1.00× |
| 10000000 | 125.8 MB | 131.9 MB | 131.9 MB | 131.8 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.13 s | 63 ms | 63 ms | 0.19 s | 3.04× |
| 3000000 | 0.41 s | 0.19 s | 0.19 s | 0.61 s | 3.20× |
| 10000000 | 1.45 s | 0.68 s | 0.68 s | 2.21 s | 3.25× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.62 s | 1.17 s | 0.92 s | 0.52 s |
| Executable | 174 KB | 142 KB | 143 KB | 306 KB |

At the largest input, Sayon took 3.20× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 0.95× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
