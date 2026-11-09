# Benchmark results

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows.

- C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`
- Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. *Rust, checked* is the same build with `-C overflow-checks=on`, which checks integer arithmetic for overflow, as Sayon always does.
- Sayon: sayon 0.0.1 with `--release`

Each program ran at two or three inputs, the largest taking seconds. At each input, every build first ran once to check that they all print the same thing, then they took turns, 7 runs each, and each number here is the median of those runs. *Sayon against the better* divides Sayon's number by the better of C++'s and Rust's, so below 1 means Sayon did better. Processor time under 50 ms is a few of the system's clock ticks, too coarse to compare, so it says *too short to tell* instead. Each program's README has its full tables: every input, the fastest and slowest runs, time on the clock, and how long each build took. [measurements.json](measurements.json) has every run's numbers.

At the largest inputs, Sayon took the least processor time of the three on 4 of the 19 programs that run long enough to time, and less than Rust on 5. It held the least memory on 7 of all 20, and less than Rust on 18.

## Processor time at the largest input

How long the program ran on the processor, its threads added up, and the system's time on its behalf. It hardly changes with what else the computer is doing.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 0 ms | 0 ms | 0 ms | 0 ms | too short to tell |
| [fib](fib/README.md) | 40 | 0.12 s | 0.12 s | 0.23 s | 0.38 s | 3.00× |
| [fannkuch](fannkuch/README.md) | 11 | 1.30 s | 1.38 s | 1.27 s | 1.44 s | 1.11× |
| [nbody](nbody/README.md) | 12000000 | 0.42 s | 0.22 s | 0.23 s | 0.48 s | 2.21× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 0.42 s | 0.34 s | 0.48 s | 0.59 s | 1.73× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 0.84 s | 0.64 s | 0.64 s | 0.64 s | 1.00× |
| [matrix](matrix/README.md) | 1000 | 0.47 s | 0.25 s | 0.25 s | 0.56 s | 2.25× |
| [sieve](sieve/README.md) | 100000000 | 0.62 s | 0.70 s | 0.62 s | 0.73 s | 1.18× |
| [binary_trees](binary_trees/README.md) | 17 | 2.09 s | 2.03 s | 1.69 s | 0.66 s | 0.32× |
| [hash_map](hash_map/README.md) | 2000000 | 0.58 s | 0.33 s | 0.36 s | 0.42 s | 1.29× |
| [word_count](word_count/README.md) | 10000000 | 0.22 s | 0.78 s | 0.53 s | 0.47 s | 2.14× |
| [sort](sort/README.md) | 10000000 | 1.44 s | 0.72 s | 0.69 s | 2.19 s | 3.04× |
| [text](text/README.md) | 3000000 | 1.62 s | 1.84 s | 1.84 s | 1.09 s | 0.67× |
| [threads](threads/README.md) | 10000000 | 2.86 s | 2.88 s | 8.88 s | 6.75 s | 2.36× |
| [nqueens](nqueens/README.md) | 13 | 0.45 s | 0.41 s | 0.45 s | 0.55 s | 1.35× |
| [life](life/README.md) | 500 | 0.81 s | 1.19 s | 1.09 s | 1.27 s | 1.56× |
| [edit_distance](edit_distance/README.md) | 2000 | 0.56 s | 0.44 s | 0.28 s | 0.31 s | 0.71× |
| [dijkstra](dijkstra/README.md) | 1000000 | 1.42 s | 1.03 s | 1.03 s | 0.95 s | 0.92× |
| [json](json/README.md) | 1000000 | 0.73 s | 0.38 s | 0.39 s | 0.59 s | 1.58× |
| [vm](vm/README.md) | 10000000 | 0.56 s | 0.62 s | 0.73 s | 0.75 s | 1.33× |

## Peak memory at the largest input

The most memory the program held at once.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the leaner |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.04× |
| [fib](fib/README.md) | 40 | 3.1 MB | 3.3 MB | 3.4 MB | 3.2 MB | 1.05× |
| [fannkuch](fannkuch/README.md) | 11 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.05× |
| [nbody](nbody/README.md) | 12000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.3 MB | 1.05× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| [matrix](matrix/README.md) | 1000 | 27.1 MB | 27.9 MB | 27.9 MB | 26.3 MB | 0.97× |
| [sieve](sieve/README.md) | 100000000 | 98.0 MB | 98.3 MB | 98.3 MB | 131.7 MB | 1.34× |
| [binary_trees](binary_trees/README.md) | 17 | 18.8 MB | 19.1 MB | 19.1 MB | 15.3 MB | 0.81× |
| [hash_map](hash_map/README.md) | 2000000 | 142.1 MB | 121.3 MB | 121.3 MB | 81.8 MB | 0.67× |
| [word_count](word_count/README.md) | 10000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.05× |
| [sort](sort/README.md) | 10000000 | 125.5 MB | 131.9 MB | 131.9 MB | 131.8 MB | 1.05× |
| [text](text/README.md) | 3000000 | 539.7 MB | 425.9 MB | 426.0 MB | 196.9 MB | 0.46× |
| [threads](threads/README.md) | 10000000 | 3.1 MB | 3.4 MB | 3.4 MB | 3.7 MB | 1.18× |
| [nqueens](nqueens/README.md) | 13 | 3.1 MB | 3.4 MB | 3.4 MB | 3.2 MB | 1.04× |
| [life](life/README.md) | 500 | 3.4 MB | 3.7 MB | 3.7 MB | 3.3 MB | 0.97× |
| [edit_distance](edit_distance/README.md) | 2000 | 3.1 MB | 3.6 MB | 3.7 MB | 3.5 MB | 1.13× |
| [dijkstra](dijkstra/README.md) | 1000000 | 132.9 MB | 127.1 MB | 127.0 MB | 111.5 MB | 0.88× |
| [json](json/README.md) | 1000000 | 147.9 MB | 132.0 MB | 131.9 MB | 128.7 MB | 0.98× |
| [vm](vm/README.md) | 10000000 | 3.1 MB | 3.3 MB | 3.4 MB | 3.2 MB | 1.05× |

## Time on the clock at the largest input

From starting the program to its end, as a person waiting for it would count. Threads that run at once shorten it, and a busy computer lengthens it.

| Benchmark | Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|---:|
| [hello](hello/README.md) | 1 | 17 ms | 20 ms | 18 ms | 18 ms | 1.05× |
| [fib](fib/README.md) | 40 | 0.79 s | 0.73 s | 1.13 s | 1.98 s | 2.69× |
| [fannkuch](fannkuch/README.md) | 11 | 6.99 s | 7.32 s | 7.02 s | 8.14 s | 1.16× |
| [nbody](nbody/README.md) | 12000000 | 2.40 s | 1.16 s | 1.13 s | 2.38 s | 2.05× |
| [spectral_norm](spectral_norm/README.md) | 4000 | 2.37 s | 2.13 s | 2.67 s | 3.51 s | 1.65× |
| [mandelbrot](mandelbrot/README.md) | 4000 | 4.35 s | 3.34 s | 3.33 s | 3.44 s | 1.03× |
| [matrix](matrix/README.md) | 1000 | 2.07 s | 1.21 s | 1.21 s | 2.75 s | 2.26× |
| [sieve](sieve/README.md) | 100000000 | 2.59 s | 2.58 s | 2.70 s | 3.39 s | 1.32× |
| [binary_trees](binary_trees/README.md) | 17 | 5.57 s | 6.10 s | 4.83 s | 2.18 s | 0.39× |
| [hash_map](hash_map/README.md) | 2000000 | 2.73 s | 1.54 s | 1.52 s | 1.89 s | 1.23× |
| [word_count](word_count/README.md) | 10000000 | 1.00 s | 3.53 s | 2.63 s | 2.44 s | 2.43× |
| [sort](sort/README.md) | 10000000 | 1.57 s | 0.75 s | 0.71 s | 2.33 s | 3.10× |
| [text](text/README.md) | 3000000 | 1.63 s | 1.85 s | 1.86 s | 1.11 s | 0.68× |
| [threads](threads/README.md) | 10000000 | 0.76 s | 0.76 s | 2.35 s | 1.77 s | 2.34× |
| [nqueens](nqueens/README.md) | 13 | 0.46 s | 0.43 s | 0.46 s | 0.55 s | 1.29× |
| [life](life/README.md) | 500 | 0.82 s | 1.18 s | 1.10 s | 1.27 s | 1.54× |
| [edit_distance](edit_distance/README.md) | 2000 | 0.57 s | 0.45 s | 0.29 s | 0.32 s | 0.73× |
| [dijkstra](dijkstra/README.md) | 1000000 | 1.42 s | 1.03 s | 1.03 s | 0.95 s | 0.92× |
| [json](json/README.md) | 1000000 | 0.73 s | 0.37 s | 0.40 s | 0.60 s | 1.59× |
| [vm](vm/README.md) | 10000000 | 1.50 s | 1.83 s | 1.80 s | 1.96 s | 1.31× |

## As the input grows

Sayon's processor time against the better of C++ and Rust, at each input.

| Benchmark | Inputs | Sayon against the better, at each |
|---|---|---|
| hello | 1 | too short to tell |
| fib | 34, 37, 40 | too short to tell, too short to tell, 3.00× |
| fannkuch | 10, 11 | 1.00×, 1.11× |
| nbody | 1000000, 4000000, 12000000 | too short to tell, 2.00×, 2.21× |
| spectral_norm | 1000, 2000, 4000 | too short to tell, 1.29×, 1.73× |
| mandelbrot | 1000, 2000, 4000 | too short to tell, 0.92×, 1.00× |
| matrix | 400, 700, 1000 | too short to tell, 2.80×, 2.25× |
| sieve | 10000000, 40000000, 100000000 | too short to tell, 1.00×, 1.18× |
| binary_trees | 13, 15, 17 | 0.50×, 0.41×, 0.32× |
| hash_map | 200000, 800000, 2000000 | too short to tell, 1.11×, 1.29× |
| word_count | 1000000, 4000000, 10000000 | too short to tell, 2.33×, 2.14× |
| sort | 1000000, 3000000, 10000000 | 2.50×, 2.79×, 3.04× |
| text | 300000, 1000000, 3000000 | 0.70×, 0.73×, 0.67× |
| threads | 1000000, 4000000, 10000000 | 2.47×, 2.41×, 2.36× |
| nqueens | 12, 13 | 1.40×, 1.35× |
| life | 50, 200, 500 | 1.57×, 1.52×, 1.56× |
| edit_distance | 500, 1000, 2000 | too short to tell, 0.75×, 0.71× |
| dijkstra | 100000, 300000, 1000000 | 0.83×, 0.94×, 0.92× |
| json | 100000, 300000, 1000000 | too short to tell, 1.57×, 1.58× |
| vm | 1000000, 3000000, 10000000 | 1.00×, 1.40×, 1.33× |

## What checking for overflow costs

Sayon checks every integer operation for overflow in every build. Rust does in debug builds, and in release builds only when asked, as the checked build does. This compares the two Rust builds at the largest input, and Sayon with the checked one.

| Benchmark | Rust | Rust, checked | What the checks cost Rust | Sayon | Sayon against checked Rust |
|---|---:|---:|---:|---:|---:|
| hello | 0 ms | 0 ms | too short to tell | 0 ms | too short to tell |
| fib | 0.12 s | 0.23 s | 1.88× | 0.38 s | 1.60× |
| fannkuch | 1.38 s | 1.27 s | 0.92× | 1.44 s | 1.14× |
| nbody | 0.22 s | 0.23 s | 1.07× | 0.48 s | 2.07× |
| spectral_norm | 0.34 s | 0.48 s | 1.41× | 0.59 s | 1.23× |
| mandelbrot | 0.64 s | 0.64 s | 1.00× | 0.64 s | 1.00× |
| matrix | 0.25 s | 0.25 s | 1.00× | 0.56 s | 2.25× |
| sieve | 0.70 s | 0.62 s | 0.89× | 0.73 s | 1.18× |
| binary_trees | 2.03 s | 1.69 s | 0.83× | 0.66 s | 0.39× |
| hash_map | 0.33 s | 0.36 s | 1.10× | 0.42 s | 1.17× |
| word_count | 0.78 s | 0.53 s | 0.68× | 0.47 s | 0.88× |
| sort | 0.72 s | 0.69 s | 0.96× | 2.19 s | 3.18× |
| text | 1.84 s | 1.84 s | 1.00× | 1.09 s | 0.59× |
| threads | 2.88 s | 8.88 s | 3.09× | 6.75 s | 0.76× |
| nqueens | 0.41 s | 0.45 s | 1.12× | 0.55 s | 1.21× |
| life | 1.19 s | 1.09 s | 0.92× | 1.27 s | 1.16× |
| edit_distance | 0.44 s | 0.28 s | 0.64× | 0.31 s | 1.11× |
| dijkstra | 1.03 s | 1.03 s | 1.00× | 0.95 s | 0.92× |
| json | 0.38 s | 0.39 s | 1.04× | 0.59 s | 1.52× |
| vm | 0.62 s | 0.73 s | 1.18× | 0.75 s | 1.02× |

## Building

How long each compiler took to build the program, and how big the executable is.

| Benchmark | Time to build: C++, Rust, Rust checked, Sayon | Executable: C++, Rust, Rust checked, Sayon |
|---|---:|---:|
| hello | 2.90 s, 2.01 s, 2.02 s, 2.80 s | 115 KB, 128 KB, 128 KB, 257 KB |
| fib | 0.69 s, 1.14 s, 0.95 s, 0.82 s | 145 KB, 135 KB, 135 KB, 301 KB |
| fannkuch | 3.73 s, 1.62 s, 1.30 s, 1.02 s | 150 KB, 137 KB, 138 KB, 305 KB |
| nbody | 3.26 s, 1.77 s, 1.25 s, 2.28 s | 149 KB, 162 KB, 162 KB, 309 KB |
| spectral_norm | 2.62 s, 1.50 s, 1.15 s, 0.95 s | 149 KB, 160 KB, 159 KB, 308 KB |
| mandelbrot | 0.73 s, 1.02 s, 0.93 s, 0.93 s | 146 KB, 136 KB, 136 KB, 302 KB |
| matrix | 2.31 s, 1.13 s, 1.01 s, 0.86 s | 149 KB, 159 KB, 159 KB, 308 KB |
| sieve | 3.53 s, 0.98 s, 0.96 s, 0.76 s | 148 KB, 136 KB, 136 KB, 303 KB |
| binary_trees | 2.90 s, 1.15 s, 1.03 s, 0.84 s | 148 KB, 137 KB, 139 KB, 304 KB |
| hash_map | 3.17 s, 1.93 s, 1.84 s, 0.83 s | 153 KB, 141 KB, 142 KB, 310 KB |
| word_count | 4.08 s, 1.95 s, 2.08 s, 1.04 s | 161 KB, 151 KB, 151 KB, 319 KB |
| sort | 3.99 s, 1.61 s, 1.51 s, 1.07 s | 174 KB, 142 KB, 143 KB, 306 KB |
| text | 3.21 s, 1.61 s, 1.41 s, 1.10 s | 166 KB, 140 KB, 140 KB, 308 KB |
| threads | 1.95 s, 1.12 s, 1.10 s, 0.67 s | 174 KB, 161 KB, 162 KB, 398 KB |
| nqueens | 1.18 s, 0.56 s, 0.63 s, 0.42 s | 148 KB, 136 KB, 137 KB, 303 KB |
| life | 1.22 s, 0.67 s, 0.64 s, 0.49 s | 150 KB, 137 KB, 137 KB, 305 KB |
| edit_distance | 1.27 s, 0.71 s, 0.74 s, 0.48 s | 150 KB, 138 KB, 138 KB, 305 KB |
| dijkstra | 1.30 s, 0.78 s, 0.92 s, 0.82 s | 151 KB, 138 KB, 138 KB, 305 KB |
| json | 1.32 s, 0.69 s, 0.70 s, 0.52 s | 155 KB, 138 KB, 139 KB, 308 KB |
| vm | 1.18 s, 0.64 s, 0.63 s, 0.47 s | 148 KB, 138 KB, 139 KB, 307 KB |
