# vm

Runs a little program on a stack machine: an interpreter that reads one instruction at a time, from an array of numbers, and does what it says. The program adds up i × i, modulo the prime 1,000,000,007, for every i below a million, three million, and ten million, which takes seventeen instructions for each i, or 170 million at the largest input. Interpreters for scripting languages spend their time in a loop like this one.

## What's the same in all three

The same program, written as the same 38 numbers, and the same machine: a loop that looks at the code of the next instruction and picks what to do, a stack of 16 places in an array, and three variables in another. The choice is a `switch` in C++ and a `match` in Rust and Sayon.

## What differs

Naming the codes. C++ names them in an `enum` and Rust as constants, and both use the names in the program and in their `switch` or `match`. Sayon's program and its cases are written as the plain numbers, with a comment saying which number does what.

Sayon checks each array index and each addition and multiplication for overflow. Rust checks the indexes it can't prove are in range, and its arithmetic only in its checked build. C++ checks neither.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 63 ms | 78 ms | 63 ms | 63 ms | 1.00× |
| 3000000 | 0.16 s | 0.20 s | 0.22 s | 0.22 s | 1.40× |
| 10000000 | 0.56 s | 0.62 s | 0.73 s | 0.75 s | 1.33× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 47 ms to 78 ms | 31 ms to 0.11 s | 63 ms to 0.11 s | 63 ms to 0.12 s |
| 3000000 | 0.12 s to 0.17 s | 0.17 s to 0.38 s | 0.17 s to 0.38 s | 0.17 s to 0.25 s |
| 10000000 | 0.25 s to 0.64 s | 0.20 s to 1.39 s | 0.61 s to 1.34 s | 0.64 s to 1.05 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 3000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| 10000000 | 3.1 MB | 3.3 MB | 3.4 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 70 ms | 72 ms | 75 ms | 65 ms | 0.93× |
| 3000000 | 0.16 s | 0.21 s | 0.21 s | 0.23 s | 1.40× |
| 10000000 | 1.50 s | 1.83 s | 1.80 s | 1.96 s | 1.31× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.18 s | 0.64 s | 0.63 s | 0.47 s |
| Executable | 148 KB | 138 KB | 139 KB | 307 KB |

At the largest input, Sayon took 1.33× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.18× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
