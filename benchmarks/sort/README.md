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
| 1000000 | 78 ms | 47 ms | 47 ms | 0.11 s | too short to tell |
| 3000000 | 0.20 s | 0.12 s | 0.14 s | 0.39 s | 3.12× |
| 10000000 | 0.91 s | 0.45 s | 0.45 s | 1.25 s | 2.76× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 63 ms to 0.11 s | 0 ms to 94 ms | 0 ms to 78 ms | 47 ms to 0.16 s |
| 3000000 | 0.11 s to 0.38 s | 63 ms to 0.20 s | 63 ms to 0.17 s | 0.20 s to 0.45 s |
| 10000000 | 0.78 s to 1.03 s | 0.33 s to 0.61 s | 0.31 s to 0.69 s | 1.17 s to 1.41 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 15.6 MB | 19.2 MB | 19.2 MB | 15.2 MB | 0.97× |
| 3000000 | 40.2 MB | 38.2 MB | 38.2 MB | 38.1 MB | 1.00× |
| 10000000 | 125.8 MB | 131.9 MB | 131.9 MB | 131.8 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.21 s | 0.10 s | 0.10 s | 0.30 s | 2.92× |
| 3000000 | 0.64 s | 0.32 s | 0.32 s | 0.90 s | 2.86× |
| 10000000 | 2.19 s | 1.10 s | 1.11 s | 3.25 s | 2.95× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.69 s | 1.85 s | 1.86 s | 0.96 s |
| Executable | 174 KB | 142 KB | 143 KB | 306 KB |

At the largest input, Sayon took 2.76× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
