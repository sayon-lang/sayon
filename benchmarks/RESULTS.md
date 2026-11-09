# Benchmark results

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows.

- C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`
- Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. *Rust, checked* is the same build with `-C overflow-checks=on`, which checks integer arithmetic for overflow, as Sayon always does.
- Sayon: sayon 0.0.1 with `--release`

Each program ran at two or three inputs, the largest taking seconds. At each input, every build first ran once to check that they all print the same thing, then they took turns, 7 runs each, and each number here is the median of those runs. *Sayon against the better* divides Sayon's number by the better of C++'s and Rust's, so below 1 means Sayon did better. Processor time under 50 ms is a few of the system's clock ticks, too coarse to compare, so it says *too short to tell* instead. Each program's README has its full tables: every input, the fastest and slowest runs, time on the clock, and how long each build took. [measurements.json](measurements.json) has every run's numbers.

At the largest inputs, Sayon took the least processor time of the three on 2 of the 13 programs that run long enough to time, and less than Rust on 3. It held the least memory on 4 of all 14, and less than Rust on 12.

## Processor time at the largest input

How long the program ran on the processor, its threads added up, and the system's time on its behalf. It hardly changes with what else the computer is doing.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 0 ms | 0 ms | 0 ms | 0 ms | too short to tell |
| [fib](fib/README.md) | 40 | 0.23 s | 0.23 s | 0.36 s | 0.66 s | 2.80× |
| [fannkuch](fannkuch/README.md) | 11 | 2.45 s | 2.44 s | 2.20 s | 2.58 s | 1.06× |
| [nbody](nbody/README.md) | 12000000 | 0.81 s | 0.42 s | 0.39 s | 0.81 s | 1.93× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 0.91 s | 0.86 s | 1.05 s | 1.17 s | 1.36× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 1.59 s | 1.20 s | 1.17 s | 1.34 s | 1.12× |
| [matrix](matrix/README.md) | 1000 | 0.81 s | 0.50 s | 0.52 s | 1.06 s | 2.12× |
| [sieve](sieve/README.md) | 100000000 | 1.06 s | 1.05 s | 1.08 s | 1.23 s | 1.18× |
| [binary_trees](binary_trees/README.md) | 17 | 2.02 s | 2.23 s | 2.38 s | 0.75 s | 0.37× |
| [hash_map](hash_map/README.md) | 2000000 | 1.00 s | 0.61 s | 0.61 s | 0.69 s | 1.13× |
| [word_count](word_count/README.md) | 10000000 | 0.42 s | 1.30 s | 1.28 s | 0.91 s | 2.15× |
| [sort](sort/README.md) | 10000000 | 0.91 s | 0.45 s | 0.45 s | 1.25 s | 2.76× |
| [text](text/README.md) | 3000000 | 1.08 s | 1.20 s | 1.22 s | 0.77 s | 0.71× |
| [threads](threads/README.md) | 10000000 | 2.03 s | 2.16 s | 6.81 s | 5.39 s | 2.65× |

## Peak memory at the largest input

The most memory the program held at once.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the leaner |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| [fib](fib/README.md) | 40 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| [fannkuch](fannkuch/README.md) | 11 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| [nbody](nbody/README.md) | 12000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.06× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| [matrix](matrix/README.md) | 1000 | 27.6 MB | 27.9 MB | 27.9 MB | 26.3 MB | 0.95× |
| [sieve](sieve/README.md) | 100000000 | 98.0 MB | 98.3 MB | 98.3 MB | 131.8 MB | 1.34× |
| [binary_trees](binary_trees/README.md) | 17 | 18.8 MB | 19.1 MB | 19.1 MB | 15.3 MB | 0.81× |
| [hash_map](hash_map/README.md) | 2000000 | 142.1 MB | 121.3 MB | 121.2 MB | 81.0 MB | 0.67× |
| [word_count](word_count/README.md) | 10000000 | 3.1 MB | 3.6 MB | 3.5 MB | 3.3 MB | 1.06× |
| [sort](sort/README.md) | 10000000 | 125.8 MB | 131.9 MB | 131.9 MB | 131.8 MB | 1.05× |
| [text](text/README.md) | 3000000 | 539.8 MB | 425.9 MB | 425.9 MB | 196.8 MB | 0.46× |
| [threads](threads/README.md) | 10000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |

## Time on the clock at the largest input

From starting the program to its end, as a person waiting for it would count. Threads that run at once shorten it, and a busy computer lengthens it.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 14 ms | 15 ms | 15 ms | 15 ms | 1.07× |
| [fib](fib/README.md) | 40 | 0.55 s | 0.55 s | 0.84 s | 1.49 s | 2.71× |
| [fannkuch](fannkuch/README.md) | 11 | 4.85 s | 5.16 s | 5.04 s | 5.69 s | 1.17× |
| [nbody](nbody/README.md) | 12000000 | 1.84 s | 0.89 s | 0.88 s | 1.81 s | 2.02× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 1.96 s | 1.75 s | 2.23 s | 2.68 s | 1.53× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 3.83 s | 2.96 s | 2.92 s | 3.04 s | 1.03× |
| [matrix](matrix/README.md) | 1000 | 1.94 s | 1.12 s | 1.12 s | 2.48 s | 2.22× |
| [sieve](sieve/README.md) | 100000000 | 2.39 s | 2.41 s | 2.36 s | 2.71 s | 1.13× |
| [binary_trees](binary_trees/README.md) | 17 | 5.08 s | 5.21 s | 5.33 s | 2.09 s | 0.41× |
| [hash_map](hash_map/README.md) | 2000000 | 2.43 s | 1.37 s | 1.38 s | 1.73 s | 1.26× |
| [word_count](word_count/README.md) | 10000000 | 0.97 s | 3.05 s | 3.04 s | 2.19 s | 2.25× |
| [sort](sort/README.md) | 10000000 | 2.19 s | 1.10 s | 1.11 s | 3.25 s | 2.95× |
| [text](text/README.md) | 3000000 | 2.60 s | 2.99 s | 3.04 s | 1.73 s | 0.66× |
| [threads](threads/README.md) | 10000000 | 1.17 s | 1.26 s | 3.66 s | 2.96 s | 2.53× |

## As the input grows

Sayon's processor time against the better of C++ and Rust, at each input.

| Benchmark | Inputs | Sayon against the better, at each |
|---|---|---|
| hello | 1 | too short to tell |
| fib | 34, 37, 40 | too short to tell, 3.25×, 2.80× |
| fannkuch | 10, 11 | 1.44×, 1.06× |
| nbody | 1000000, 4000000, 12000000 | 1.00×, 2.25×, 1.93× |
| spectral_norm | 1000, 2000, 4000 | too short to tell, 1.62×, 1.36× |
| mandelbrot | 1000, 2000, 4000 | too short to tell, 1.29×, 1.12× |
| matrix | 400, 700, 1000 | too short to tell, 3.14×, 2.12× |
| sieve | 10000000, 40000000, 100000000 | 0.83×, 1.08×, 1.18× |
| binary_trees | 13, 15, 17 | 0.40×, 0.42×, 0.37× |
| hash_map | 200000, 800000, 2000000 | too short to tell, 1.17×, 1.13× |
| word_count | 1000000, 4000000, 10000000 | too short to tell, 1.69×, 2.15× |
| sort | 1000000, 3000000, 10000000 | too short to tell, 3.12×, 2.76× |
| text | 300000, 1000000, 3000000 | 0.71×, 0.70×, 0.71× |
| threads | 1000000, 4000000, 10000000 | 3.33×, 2.95×, 2.65× |

## What checking for overflow costs

Sayon checks every integer operation for overflow in every build. Rust does in debug builds, and in release builds only when asked, as the checked build does. This compares the two Rust builds at the largest input, and Sayon with the checked one.

| Benchmark | Rust | Rust, checked | What the checks cost Rust | Sayon | Sayon against checked Rust |
|---|---:|---:|---:|---:|---:|
| hello | 0 ms | 0 ms | too short to tell | 0 ms | too short to tell |
| fib | 0.23 s | 0.36 s | 1.53× | 0.66 s | 1.83× |
| fannkuch | 2.44 s | 2.20 s | 0.90× | 2.58 s | 1.17× |
| nbody | 0.42 s | 0.39 s | 0.93× | 0.81 s | 2.08× |
| spectral_norm | 0.86 s | 1.05 s | 1.22× | 1.17 s | 1.12× |
| mandelbrot | 1.20 s | 1.17 s | 0.97× | 1.34 s | 1.15× |
| matrix | 0.50 s | 0.52 s | 1.03× | 1.06 s | 2.06× |
| sieve | 1.05 s | 1.08 s | 1.03× | 1.23 s | 1.14× |
| binary_trees | 2.23 s | 2.38 s | 1.06× | 0.75 s | 0.32× |
| hash_map | 0.61 s | 0.61 s | 1.00× | 0.69 s | 1.13× |
| word_count | 1.30 s | 1.28 s | 0.99× | 0.91 s | 0.71× |
| sort | 0.45 s | 0.45 s | 1.00× | 1.25 s | 2.76× |
| text | 1.20 s | 1.22 s | 1.01× | 0.77 s | 0.63× |
| threads | 2.16 s | 6.81 s | 3.16× | 5.39 s | 0.79× |

## Building

How long each compiler took to build the program, and how big the executable is.

| Benchmark | Time to build: C++, Rust, Rust checked, Sayon | Executable: C++, Rust, Rust checked, Sayon |
|---|---:|---:|
| hello | 5.75 s, 1.64 s, 1.15 s, 6.16 s | 115 KB, 128 KB, 128 KB, 257 KB |
| fib | 1.55 s, 1.43 s, 1.55 s, 1.71 s | 145 KB, 135 KB, 135 KB, 301 KB |
| fannkuch | 8.11 s, 1.52 s, 1.52 s, 1.25 s | 150 KB, 137 KB, 138 KB, 305 KB |
| nbody | 2.80 s, 1.73 s, 1.38 s, 1.47 s | 149 KB, 162 KB, 162 KB, 309 KB |
| spectral_norm | 2.30 s, 1.45 s, 1.24 s, 0.77 s | 149 KB, 160 KB, 159 KB, 308 KB |
| mandelbrot | 0.67 s, 1.19 s, 1.22 s, 0.76 s | 146 KB, 136 KB, 136 KB, 302 KB |
| matrix | 2.23 s, 1.72 s, 1.32 s, 0.78 s | 149 KB, 159 KB, 159 KB, 308 KB |
| sieve | 2.02 s, 1.31 s, 1.33 s, 0.65 s | 148 KB, 136 KB, 136 KB, 303 KB |
| binary_trees | 2.35 s, 1.35 s, 1.28 s, 0.72 s | 148 KB, 137 KB, 139 KB, 304 KB |
| hash_map | 2.78 s, 2.20 s, 2.01 s, 0.90 s | 153 KB, 141 KB, 142 KB, 310 KB |
| word_count | 5.19 s, 2.46 s, 2.16 s, 1.02 s | 161 KB, 151 KB, 151 KB, 319 KB |
| sort | 2.69 s, 1.85 s, 1.86 s, 0.96 s | 174 KB, 142 KB, 143 KB, 306 KB |
| text | 2.55 s, 1.54 s, 1.52 s, 0.72 s | 166 KB, 140 KB, 140 KB, 308 KB |
| threads | 2.95 s, 2.38 s, 2.23 s, 1.05 s | 174 KB, 161 KB, 162 KB, 398 KB |
