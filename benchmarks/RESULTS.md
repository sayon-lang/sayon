# Benchmark results

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows.

- C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`
- Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. *Rust, checked* is the same build with `-C overflow-checks=on`, which checks integer arithmetic for overflow, as Sayon always does.
- Sayon: sayon 0.0.1 with `--release`

Each program ran at two or three inputs, the largest taking seconds. At each input, every build first ran once to check that they all print the same thing, then they took turns, 7 runs each, and each number here is the median of those runs. *Sayon against the better* divides Sayon's number by the better of C++'s and Rust's, so below 1 means Sayon did better. Processor time under 50 ms is a few of the system's clock ticks, too coarse to compare, so it says *too short to tell* instead. Each program's README has its full tables: every input, the fastest and slowest runs, time on the clock, and how long each build took. [measurements.json](measurements.json) has every run's numbers.

At the largest inputs, Sayon took the least processor time of the three on 2 of the 13 programs that run long enough to time, and less than Rust on 3. It held the least memory on 4 of all 14, and less than Rust on 12.

## Processor time at the largest input

How long the program ran on the processor, its threads added up, and the system's time on its behalf. It hardly changes with what else the computer is doing.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 0 ms | 16 ms | 0 ms | 0 ms | too short to tell |
| [fib](fib/README.md) | 40 | 0.47 s | 0.45 s | 0.69 s | 1.42 s | 3.14× |
| [fannkuch](fannkuch/README.md) | 11 | 4.02 s | 4.12 s | 3.94 s | 4.58 s | 1.14× |
| [nbody](nbody/README.md) | 12000000 | 1.47 s | 0.69 s | 0.69 s | 1.41 s | 2.05× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 1.44 s | 1.30 s | 1.64 s | 1.95 s | 1.51× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 2.77 s | 2.09 s | 2.09 s | 2.16 s | 1.03× |
| [matrix](matrix/README.md) | 1000 | 1.47 s | 0.84 s | 0.83 s | 1.95 s | 2.31× |
| [sieve](sieve/README.md) | 100000000 | 1.42 s | 1.44 s | 1.50 s | 1.70 s | 1.20× |
| [binary_trees](binary_trees/README.md) | 17 | 3.09 s | 3.20 s | 3.05 s | 1.42 s | 0.46× |
| [hash_map](hash_map/README.md) | 2000000 | 1.44 s | 0.81 s | 0.81 s | 1.03 s | 1.27× |
| [word_count](word_count/README.md) | 10000000 | 0.61 s | 1.83 s | 1.91 s | 1.34 s | 2.21× |
| [sort](sort/README.md) | 10000000 | 1.44 s | 0.69 s | 0.66 s | 2.20 s | 3.20× |
| [text](text/README.md) | 3000000 | 1.64 s | 1.89 s | 2.05 s | 1.11 s | 0.68× |
| [threads](threads/README.md) | 10000000 | 2.55 s | 2.56 s | 8.11 s | 6.03 s | 2.37× |

## Peak memory at the largest input

The most memory the program held at once.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the leaner |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.04× |
| [fib](fib/README.md) | 40 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| [fannkuch](fannkuch/README.md) | 11 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| [nbody](nbody/README.md) | 12000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| [matrix](matrix/README.md) | 1000 | 27.1 MB | 27.9 MB | 27.9 MB | 26.3 MB | 0.97× |
| [sieve](sieve/README.md) | 100000000 | 98.0 MB | 98.3 MB | 98.3 MB | 131.8 MB | 1.34× |
| [binary_trees](binary_trees/README.md) | 17 | 18.8 MB | 19.1 MB | 19.1 MB | 15.3 MB | 0.81× |
| [hash_map](hash_map/README.md) | 2000000 | 142.1 MB | 121.2 MB | 121.2 MB | 81.8 MB | 0.67× |
| [word_count](word_count/README.md) | 10000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.05× |
| [sort](sort/README.md) | 10000000 | 125.8 MB | 131.9 MB | 131.9 MB | 131.8 MB | 1.05× |
| [text](text/README.md) | 3000000 | 539.8 MB | 425.9 MB | 425.9 MB | 196.8 MB | 0.46× |
| [threads](threads/README.md) | 10000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.19× |

## Time on the clock at the largest input

From starting the program to its end, as a person waiting for it would count. Threads that run at once shorten it, and a busy computer lengthens it.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 11 ms | 12 ms | 11 ms | 12 ms | 1.07× |
| [fib](fib/README.md) | 40 | 0.50 s | 0.49 s | 0.76 s | 1.44 s | 2.95× |
| [fannkuch](fannkuch/README.md) | 11 | 4.25 s | 4.28 s | 4.08 s | 4.72 s | 1.11× |
| [nbody](nbody/README.md) | 12000000 | 1.57 s | 0.70 s | 0.70 s | 1.47 s | 2.09× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 1.46 s | 1.32 s | 1.67 s | 1.99 s | 1.51× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 2.93 s | 2.14 s | 2.14 s | 2.20 s | 1.03× |
| [matrix](matrix/README.md) | 1000 | 1.76 s | 1.08 s | 1.11 s | 2.23 s | 2.06× |
| [sieve](sieve/README.md) | 100000000 | 1.44 s | 1.43 s | 1.53 s | 1.72 s | 1.20× |
| [binary_trees](binary_trees/README.md) | 17 | 3.17 s | 3.27 s | 3.10 s | 1.44 s | 0.45× |
| [hash_map](hash_map/README.md) | 2000000 | 1.43 s | 0.81 s | 0.81 s | 1.03 s | 1.28× |
| [word_count](word_count/README.md) | 10000000 | 0.61 s | 1.83 s | 1.90 s | 1.36 s | 2.20× |
| [sort](sort/README.md) | 10000000 | 1.45 s | 0.68 s | 0.68 s | 2.21 s | 3.25× |
| [text](text/README.md) | 3000000 | 1.64 s | 1.89 s | 2.04 s | 1.12 s | 0.68× |
| [threads](threads/README.md) | 10000000 | 0.79 s | 0.80 s | 2.61 s | 1.88 s | 2.38× |

## As the input grows

Sayon's processor time against the better of C++ and Rust, at each input.

| Benchmark | Inputs | Sayon against the better, at each |
|---|---|---|
| hello | 1 | too short to tell |
| fib | 34, 37, 40 | too short to tell, 2.62×, 3.14× |
| fannkuch | 10, 11 | 1.10×, 1.14× |
| nbody | 1000000, 4000000, 12000000 | 2.00×, 2.07×, 2.05× |
| spectral_norm | 1000, 2000, 4000 | 1.50×, 1.45×, 1.51× |
| mandelbrot | 1000, 2000, 4000 | 1.11×, 1.03×, 1.03× |
| matrix | 400, 700, 1000 | too short to tell, 2.56×, 2.31× |
| sieve | 10000000, 40000000, 100000000 | 1.25×, 1.16×, 1.20× |
| binary_trees | 13, 15, 17 | 0.40×, 0.43×, 0.46× |
| hash_map | 200000, 800000, 2000000 | too short to tell, 1.64×, 1.27× |
| word_count | 1000000, 4000000, 10000000 | 2.25×, 2.12×, 2.21× |
| sort | 1000000, 3000000, 10000000 | 3.00×, 3.25×, 3.20× |
| text | 300000, 1000000, 3000000 | 0.70×, 0.69×, 0.68× |
| threads | 1000000, 4000000, 10000000 | 2.53×, 2.40×, 2.37× |

## What checking for overflow costs

Sayon checks every integer operation for overflow in every build. Rust does in debug builds, and in release builds only when asked, as the checked build does. This compares the two Rust builds at the largest input, and Sayon with the checked one.

| Benchmark | Rust | Rust, checked | What the checks cost Rust | Sayon | Sayon against checked Rust |
|---|---:|---:|---:|---:|---:|
| hello | 16 ms | 0 ms | too short to tell | 0 ms | too short to tell |
| fib | 0.45 s | 0.69 s | 1.52× | 1.42 s | 2.07× |
| fannkuch | 4.12 s | 3.94 s | 0.95× | 4.58 s | 1.16× |
| nbody | 0.69 s | 0.69 s | 1.00× | 1.41 s | 2.05× |
| spectral_norm | 1.30 s | 1.64 s | 1.27× | 1.95 s | 1.19× |
| mandelbrot | 2.09 s | 2.09 s | 1.00× | 2.16 s | 1.03× |
| matrix | 0.84 s | 0.83 s | 0.98× | 1.95 s | 2.36× |
| sieve | 1.44 s | 1.50 s | 1.04× | 1.70 s | 1.14× |
| binary_trees | 3.20 s | 3.05 s | 0.95× | 1.42 s | 0.47× |
| hash_map | 0.81 s | 0.81 s | 1.00× | 1.03 s | 1.27× |
| word_count | 1.83 s | 1.91 s | 1.04× | 1.34 s | 0.70× |
| sort | 0.69 s | 0.66 s | 0.95× | 2.20 s | 3.36× |
| text | 1.89 s | 2.05 s | 1.08× | 1.11 s | 0.54× |
| threads | 2.56 s | 8.11 s | 3.16× | 6.03 s | 0.74× |

## Building

How long each compiler took to build the program, and how big the executable is.

| Benchmark | Time to build: C++, Rust, Rust checked, Sayon | Executable: C++, Rust, Rust checked, Sayon |
|---|---:|---:|
| hello | 0.41 s, 0.44 s, 0.43 s, 0.44 s | 115 KB, 128 KB, 128 KB, 257 KB |
| fib | 0.60 s, 0.72 s, 0.55 s, 0.42 s | 145 KB, 135 KB, 135 KB, 301 KB |
| fannkuch | 2.31 s, 1.01 s, 0.90 s, 0.60 s | 150 KB, 137 KB, 138 KB, 305 KB |
| nbody | 2.41 s, 1.24 s, 1.21 s, 1.18 s | 149 KB, 162 KB, 162 KB, 309 KB |
| spectral_norm | 1.65 s, 0.94 s, 0.80 s, 0.52 s | 149 KB, 160 KB, 159 KB, 308 KB |
| mandelbrot | 0.45 s, 0.86 s, 0.56 s, 0.52 s | 146 KB, 136 KB, 136 KB, 302 KB |
| matrix | 1.43 s, 0.66 s, 0.84 s, 0.49 s | 149 KB, 159 KB, 159 KB, 308 KB |
| sieve | 2.12 s, 0.87 s, 1.15 s, 1.20 s | 148 KB, 136 KB, 136 KB, 303 KB |
| binary_trees | 1.43 s, 0.87 s, 0.63 s, 0.55 s | 148 KB, 137 KB, 139 KB, 304 KB |
| hash_map | 1.43 s, 1.17 s, 1.13 s, 0.39 s | 153 KB, 141 KB, 142 KB, 310 KB |
| word_count | 2.38 s, 1.34 s, 1.08 s, 0.62 s | 161 KB, 151 KB, 151 KB, 319 KB |
| sort | 1.62 s, 1.17 s, 0.92 s, 0.52 s | 174 KB, 142 KB, 143 KB, 306 KB |
| text | 1.71 s, 0.86 s, 0.81 s, 0.43 s | 166 KB, 140 KB, 140 KB, 308 KB |
| threads | 2.31 s, 6.21 s, 1.86 s, 1.51 s | 174 KB, 161 KB, 162 KB, 398 KB |
