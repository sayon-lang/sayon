# fannkuch

Take a stack of pancakes numbered 1 to n. Look at the top one: if it says k, flip the top k pancakes over. Keep going until pancake 1 is on top, and count the flips. This program does that for every order the stack can start in, all n! of them, and reports the most flips any order needed, along with a checksum of all the counts.

It's the classic fannkuch-redux benchmark, on one thread. Almost all the time goes into small integer arrays: copying one, reversing the front of it, and stepping to the next order.

## What's the same in all three

The versions visit the orders in the same sequence, flip with the same swap loop, and print the same checksum and maximum. Each keeps the stack in an array or vector of plain integers.

## What differs

Sayon checks every array index and every integer operation. C++ checks neither, and Rust checks indexes but not overflow in a release build. The arrays here are short and the checks predictable, so they cost little, and the three come out about even.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.28 s | 0.30 s | 0.30 s |
| Fastest and slowest run | 0.23 to 0.33 s | 0.20 to 0.34 s | 0.20 to 0.39 s |
| Peak memory, median | 3.1 MB | 3.4 MB | 3.2 MB |
| Executable | 150 KB | 137 KB | 305 KB |

Sayon took 1.06× the time of the faster of the other two, and held 1.05× the memory of the leaner one.

Input 10, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
