# nbody

The Sun and the four gas giants pull on each other, and the program moves them forward in small time steps, two million of them. It prints the system's total energy before and after, to nine decimal places, which also checks that every version did exactly the same arithmetic.

## What's the same in all three

Each body is a structure of seven floats kept in an array, and every step compares each pair of bodies once, the same way, in the same order. No version uses SIMD by hand.

## What differs

Rust walks the pairs with `split_at_mut`, which hands its compiler two references it knows can't point at the same body. So it keeps the first body's position and speed in registers while it goes through the others. C++ and Sayon index the same array twice, and their compilers can't tell that the two bodies are different, so they read those values from memory again after every change. That is a real advantage of Rust's borrowing rules, and most of its lead here.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.23 s | 0.11 s | 0.25 s |
| Fastest and slowest run | 0.14 to 0.28 s | 0.08 to 0.16 s | 0.20 to 0.27 s |
| Peak memory, median | 3.1 MB | 3.4 MB | 3.2 MB |
| Executable | 149 KB | 162 KB | 309 KB |

Sayon took 2.29× the time of the faster of the other two, and held 1.05× the memory of the leaner one.

Input 2000000, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
