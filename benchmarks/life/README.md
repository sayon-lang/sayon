# life

Runs Conway's Game of Life on a 512 by 512 board whose edges wrap around, from a pseudo-random start, for 50, 200, and 500 generations, then counts the cells still alive. Each generation reads the eight neighbours of every cell, a little over two million reads, and writes the next board.

## What's the same in all three

A byte for each cell, two boards that swap places after every generation, the same order of additions for the neighbours, and the remainder operator to wrap the edges. The start comes from the same generator as the other programs.

## What differs

Swapping the boards: C++ uses `std::swap` and Rust `std::mem::swap`, and Sayon moves the two arrays through a third variable. None of them copies a board.

The sums. C++ adds the bytes as `int`s. Rust and Sayon add them as bytes, and Sayon checks every sum for overflow, as Rust does only in its checked build. Sayon and Rust also check every neighbour's index against the board, which C++ doesn't.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 50 | 0.11 s | 0.17 s | 0.14 s | 0.17 s | 1.57× |
| 200 | 0.36 s | 0.52 s | 0.48 s | 0.55 s | 1.52× |
| 500 | 0.81 s | 1.19 s | 1.09 s | 1.27 s | 1.56× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 50 | 94 ms to 0.14 s | 0.14 s to 0.22 s | 0.14 s to 0.19 s | 0.16 s to 0.22 s |
| 200 | 0.36 s to 0.38 s | 0.50 s to 0.55 s | 0.45 s to 0.53 s | 0.55 s to 0.62 s |
| 500 | 0.80 s to 0.88 s | 1.16 s to 1.25 s | 1.08 s to 1.11 s | 1.23 s to 1.31 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 50 | 3.4 MB | 3.7 MB | 3.7 MB | 3.3 MB | 0.97× |
| 200 | 3.4 MB | 3.7 MB | 3.7 MB | 3.3 MB | 0.97× |
| 500 | 3.4 MB | 3.7 MB | 3.7 MB | 3.3 MB | 0.97× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 50 | 0.20 s | 0.16 s | 0.15 s | 0.26 s | 1.61× |
| 200 | 0.38 s | 0.52 s | 0.48 s | 0.55 s | 1.47× |
| 500 | 0.82 s | 1.18 s | 1.10 s | 1.27 s | 1.54× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.22 s | 0.67 s | 0.64 s | 0.49 s |
| Executable | 150 KB | 137 KB | 137 KB | 305 KB |

At the largest input, Sayon took 1.56× the processor time of the faster of C++ and Rust, and held the least memory, 0.97× that of the leaner one. With its overflow checks on, Rust took 0.92× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
