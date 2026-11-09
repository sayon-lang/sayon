# nqueens

Counts the ways to put n queens on an n by n board so that no two attack each other, on boards of 12 and 13 squares a side. It places a queen in each row in turn, tries every column that is still safe, and backs up when a row has none left, so most of the time goes into recursive calls and checks of three small arrays of flags.

## What's the same in all three

The same search, and a byte for each flag: one array for the columns, and one for each direction of diagonal. `std::vector<unsigned char>` in C++, `Vec<bool>` in Rust, and `Array<boolean>` in Sayon. Each call gets the three arrays to change: by reference in C++, as `&mut` slices in Rust, and as `update` parameters in Sayon.

## What differs

Little. Sayon checks every index against its array, as Rust does where it can't prove the index is in range, and Sayon also checks the sums that make the diagonal numbers for overflow. C++ checks neither. Making the arrays costs nothing worth counting here: C++ and Rust make them at their size in one step, and Sayon pushes one flag at a time, a few dozen of them.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 12 | 94 ms | 78 ms | 94 ms | 0.11 s | 1.40× |
| 13 | 0.45 s | 0.41 s | 0.45 s | 0.55 s | 1.35× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 12 | 78 ms to 0.11 s | 78 ms to 94 ms | 78 ms to 94 ms | 94 ms to 0.11 s |
| 13 | 0.44 s to 0.48 s | 0.41 s to 0.44 s | 0.44 s to 0.47 s | 0.53 s to 0.56 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 12 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 13 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.04× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 12 | 91 ms | 82 ms | 89 ms | 0.10 s | 1.27× |
| 13 | 0.46 s | 0.43 s | 0.46 s | 0.55 s | 1.29× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.18 s | 0.56 s | 0.63 s | 0.42 s |
| Executable | 148 KB | 136 KB | 137 KB | 303 KB |

At the largest input, Sayon took 1.35× the processor time of the faster of C++ and Rust, and held 1.04× the memory of the leaner one. With its overflow checks on, Rust took 1.12× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
