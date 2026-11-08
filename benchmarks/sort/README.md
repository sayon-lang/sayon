# sort

Fills an array with three million pseudo-random integers and sorts it with the standard library, then prints a few values and a checksum to show the order is right.

## What's the same in all three

The same numbers, and a stable sort from each standard library: `std::stable_sort` in C++, `sort` in Rust, and `collections.sort` in Sayon, which is stable. Comparing against the unstable sorts of C++ and Rust would make the others do less.

## What differs

How the sorts are built. Rust's is driftsort, a carefully tuned merge sort that finds runs already in order and merges without branches. C++'s is Microsoft's `stable_sort`. Sayon's is a plain merge sort written in Sayon: insertion sort for parts of 16 elements or fewer, and merges through a spare array half the input's size. All three take time in proportion to n log n.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.41 s | 0.20 s | 0.58 s |
| Fastest and slowest run | 0.34 to 0.47 s | 0.12 to 0.23 s | 0.38 to 0.64 s |
| Peak memory, median | 40.2 MB | 38.2 MB | 38.1 MB |
| Executable | 174 KB | 142 KB | 306 KB |

Sayon took 2.85× the time of the faster of the other two, and held the least memory, 1.00× that of the leaner one.

Input 3000000, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
