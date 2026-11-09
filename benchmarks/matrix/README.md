# matrix

Multiplies two square matrices of floating point numbers, each kept row by row in one array, and prints the sum of the result and one of its entries. The matrices are 400, 700, and 1,000 numbers on a side, and the largest takes a billion multiplications and additions, in three nested loops.

## What's the same in all three

The same loops in the same order, so that the innermost one walks straight along a row of the result and a row of the second matrix. The same arithmetic in the same order, with no fast math, so all three print the same digits.

## What differs

Whether the innermost loop works on two numbers at a time. Rust passes the arrays to the multiplying function as slices, and knows that the one it writes can't overlap the two it reads, so its compiler turns that loop into vector instructions. C++ passes vectors by reference, so its compiler has to allow for them overlapping, and Sayon checks every index and every index computation for overflow. Neither of those two loops uses vector instructions, so each does one number at a time.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 400 | 31 ms | 0 ms | 16 ms | 47 ms | too short to tell |
| 700 | 0.16 s | 78 ms | 0.11 s | 0.22 s | 2.80× |
| 1000 | 0.47 s | 0.25 s | 0.25 s | 0.56 s | 2.25× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 400 | 0 ms to 63 ms | 0 ms to 31 ms | 0 ms to 31 ms | 31 ms to 63 ms |
| 700 | 0.12 s to 0.22 s | 16 ms to 0.11 s | 78 ms to 0.16 s | 0.14 s to 0.27 s |
| 1000 | 0.33 s to 0.53 s | 0.11 s to 0.31 s | 0.22 s to 0.30 s | 0.42 s to 0.62 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 400 | 8.3 MB | 9.0 MB | 9.0 MB | 7.4 MB | 0.89× |
| 700 | 18.3 MB | 16.2 MB | 16.2 MB | 14.6 MB | 0.90× |
| 1000 | 27.1 MB | 27.9 MB | 27.9 MB | 26.3 MB | 0.97× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 400 | 0.17 s | 80 ms | 83 ms | 0.20 s | 2.51× |
| 700 | 0.81 s | 0.41 s | 0.43 s | 1.12 s | 2.71× |
| 1000 | 2.07 s | 1.21 s | 1.21 s | 2.75 s | 2.26× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.31 s | 1.13 s | 1.01 s | 0.86 s |
| Executable | 149 KB | 159 KB | 159 KB | 308 KB |

At the largest input, Sayon took 2.25× the processor time of the faster of C++ and Rust, and held the least memory, 0.97× that of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
