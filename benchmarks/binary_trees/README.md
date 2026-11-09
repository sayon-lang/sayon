# binary_trees

Builds a great many complete binary trees of different depths, walks each one to count its nodes, and drops it, while one long-lived tree stays the whole time. The deepest trees are 13, 15, then 17 levels deep, and each level doubles the work. It's a test of allocating and freeing lots of small objects quickly.

## What's the same in all three

The same trees, of the same depths, built and walked in the same order, and the same counts printed. Every tree is freed as soon as it has been walked, by its owner going away. There's no garbage collector in any of the three, and none of them uses a memory pool or an arena written for the benchmark.

## What differs

How a node is laid out, which follows each language's usual way of writing a tree. In C++ each node is its own allocation holding two `unique_ptr`s, and in Rust each is a `Box` holding an `Option` of two more. In Sayon a structure can't contain itself directly, so a node holds an array of its children. A leaf's array is empty and costs no allocation at all, and the two children of an inner node share one allocation. So Sayon makes about half as many allocations for the same tree.

Where the memory comes from. Each program uses its language's own allocator, as any program would. C++'s `new` and Rust's `Box` ask the system's allocator, which on Windows is the process heap. Sayon's programs get small blocks from Sayon's allocator, which keeps a list of free blocks for each size and hands them out without calling the system, and puts no header in front of a block. That's most of Sayon's lead here. The other two could close much of the gap by switching allocators, such as Rust with mimalloc through `#[global_allocator]`, or by putting each tree in an arena. This benchmark sticks to what each language does by default.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 13 | 94 ms | 78 ms | 0.11 s | 31 ms | 0.40× |
| 15 | 0.47 s | 0.41 s | 0.45 s | 0.17 s | 0.42× |
| 17 | 2.02 s | 2.23 s | 2.38 s | 0.75 s | 0.37× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 13 | 47 ms to 0.14 s | 63 ms to 0.16 s | 63 ms to 0.19 s | 0 ms to 94 ms |
| 15 | 0.33 s to 0.69 s | 0.30 s to 0.78 s | 0.27 s to 0.61 s | 0.12 s to 0.25 s |
| 17 | 1.72 s to 2.33 s | 1.98 s to 2.42 s | 2.19 s to 2.44 s | 0.70 s to 0.94 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 13 | 3.7 MB | 4.0 MB | 4.0 MB | 4.0 MB | 1.08× |
| 15 | 6.7 MB | 7.0 MB | 7.0 MB | 6.2 MB | 0.93× |
| 17 | 18.8 MB | 19.1 MB | 19.1 MB | 15.3 MB | 0.81× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 13 | 0.25 s | 0.26 s | 0.25 s | 0.11 s | 0.44× |
| 15 | 1.15 s | 1.16 s | 1.14 s | 0.46 s | 0.40× |
| 17 | 5.08 s | 5.21 s | 5.33 s | 2.09 s | 0.41× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.35 s | 1.35 s | 1.28 s | 0.72 s |
| Executable | 148 KB | 137 KB | 139 KB | 304 KB |

At the largest input, Sayon was the fastest, in 0.37× the processor time of the faster of C++ and Rust, and held the least memory, 0.81× that of the leaner one. With its overflow checks on, Rust took 1.06× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
