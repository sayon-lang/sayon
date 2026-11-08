# hello

Prints one line and ends. It measures what every program pays before it does any work of its own: starting, ending, and the least memory and the smallest executable a program can have.

## What's the same in all three

One line, `Hello, world!`, printed the usual way in each language: `printf` in C++, `println!` in Rust, and `print` in Sayon.

## What differs

Nothing in the programs. What differs is what each brings along. On Windows, the C++ program has the C library linked into it, the Rust program carries its standard library and loads the C runtime's DLLs that Visual C++ and Windows install, and the Sayon program carries its runtime and loads only Windows' own system libraries. A program this short runs on the processor for less than the system's clock tick, about 16 ms on Windows, so its processor time shows as 0 or 16 ms. Time on the clock and memory are the numbers to look at.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1 | 0 ms | 16 ms | 0 ms | 0 ms | too short to tell |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1 | 0 ms to 16 ms | 0 ms to 31 ms | 0 ms to 16 ms | 0 ms to 16 ms |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.04× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1 | 11 ms | 12 ms | 11 ms | 12 ms | 1.07× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 0.41 s | 0.44 s | 0.43 s | 0.44 s |
| Executable | 115 KB | 128 KB | 128 KB | 257 KB |

Processor time is too short to tell here, so by time on the clock, Sayon took 1.07× the time of the faster of C++ and Rust, and held 1.04× the memory of the leaner one. With its overflow checks on, Rust took 0.98× its own time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
