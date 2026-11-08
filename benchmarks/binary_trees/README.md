# binary_trees

Builds a great many complete binary trees of different depths, walks each one to count its nodes, and drops it, while one long-lived tree stays the whole time. It's a test of allocating and freeing lots of small objects quickly.

## What's the same in all three

The same trees, of the same depths, built and walked in the same order, and the same counts printed. Every tree is freed as soon as it has been walked, by its owner going away. There's no garbage collector in any of the three, and none of them uses a memory pool or an arena written for the benchmark.

## What differs

How a node is laid out, which follows each language's usual way of writing a tree. In C++ each node is its own allocation holding two `unique_ptr`s, and in Rust each is a `Box` holding an `Option` of two more. In Sayon a structure can't contain itself directly, so a node holds an array of its children. A leaf's array is empty and costs no allocation at all, and the two children of an inner node share one allocation. So Sayon makes about half as many allocations for the same tree.

Where the memory comes from. Each program uses its language's own allocator, as any program would. C++'s `new` and Rust's `Box` ask the system's allocator, which on Windows is the process heap. Sayon's programs get small blocks from Sayon's allocator, which keeps a list of free blocks for each size and hands them out without calling the system, and puts no header in front of a block. That's most of Sayon's lead here. The other two could close much of the gap by switching allocators, such as Rust with mimalloc through `#[global_allocator]`, or by putting each tree in an arena. This benchmark sticks to what each language does by default.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.80 s | 0.81 s | 0.27 s |
| Fastest and slowest run | 0.62 to 0.88 s | 0.73 to 1.03 s | 0.17 to 0.31 s |
| Peak memory, median | 6.7 MB | 7.0 MB | 6.2 MB |
| Executable | 148 KB | 137 KB | 304 KB |

Sayon was the fastest, in 0.33× the time of the faster of the other two, and held the least memory, 0.93× that of the leaner one.

Input 15, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
