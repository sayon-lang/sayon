# Benchmark results

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows.

- C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`
- Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`
- Sayon: sayon 0.0.1 with `--release`

Each number is the median of 7 runs. Processor time adds up the time the program's threads ran, and the system's time on its behalf, so it barely changes with what else the computer is doing. Peak memory is the most the program held in memory at once. Each program's README has its own table, with the fastest and slowest runs and the size of each executable.

## Processor time

| Benchmark | Input | C++ | Rust | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|
| [fannkuch](fannkuch/README.md) | 10 | 0.28 s | 0.30 s | 0.30 s | 1.06× |
| [nbody](nbody/README.md) | 2000000 | 0.23 s | 0.11 s | 0.25 s | 2.29× |
| [spectral_norm](spectral_norm/README.md) | 1500 | 0.22 s | 0.17 s | 0.27 s | 1.55× |
| [mandelbrot](mandelbrot/README.md) | 2000 | 0.62 s | 0.44 s | 0.50 s | 1.14× |
| [binary_trees](binary_trees/README.md) | 15 | 0.80 s | 0.81 s | 0.27 s | 0.33× |
| [word_count](word_count/README.md) | 2000000 | 0.12 s | 0.42 s | 0.31 s | 2.50× |
| [sort](sort/README.md) | 3000000 | 0.41 s | 0.20 s | 0.58 s | 2.85× |
| [text](text/README.md) | 1000000 | 0.56 s | 0.66 s | 0.33 s | 0.58× |

## Peak memory

| Benchmark | Input | C++ | Rust | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|
| [fannkuch](fannkuch/README.md) | 10 | 3.1 MB | 3.4 MB | 3.2 MB | 1.05× |
| [nbody](nbody/README.md) | 2000000 | 3.1 MB | 3.4 MB | 3.2 MB | 1.05× |
| [spectral_norm](spectral_norm/README.md) | 1500 | 3.1 MB | 3.4 MB | 3.3 MB | 1.06× |
| [mandelbrot](mandelbrot/README.md) | 2000 | 3.1 MB | 3.3 MB | 3.2 MB | 1.05× |
| [binary_trees](binary_trees/README.md) | 15 | 6.7 MB | 7.0 MB | 6.2 MB | 0.93× |
| [word_count](word_count/README.md) | 2000000 | 3.1 MB | 3.5 MB | 3.3 MB | 1.06× |
| [sort](sort/README.md) | 3000000 | 40.2 MB | 38.2 MB | 38.1 MB | 1.00× |
| [text](text/README.md) | 1000000 | 172.9 MB | 143.4 MB | 67.8 MB | 0.47× |
