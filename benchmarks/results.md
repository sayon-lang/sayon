# Benchmark results

Measured on 2026-11-08, on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows.

- C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5), with `-O2`
- Rust: rustc 1.98.1 (48a229cea 2026-09-01), with `-C opt-level=3`
- Sayon: sayon 0.0.1, with `sayon build --release`

Each number is the median of 5 runs. Processor time adds up the time the program's threads ran, and the system's time on its behalf, so it barely changes with what else the computer is doing. Peak memory is the most the program held in memory at once.

## Processor time

| Benchmark | Input | C++ | Rust | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|
| fannkuch | 10 | 0.48 s | 0.52 s | 0.66 s | 1.35× |
| nbody | 2000000 | 0.42 s | 0.22 s | 0.45 s | 2.07× |
| spectral_norm | 1500 | 0.36 s | 0.34 s | 0.56 s | 1.64× |
| mandelbrot | 2000 | 0.89 s | 0.69 s | 0.73 s | 1.07× |
| binary_trees | 15 | 1.53 s | 1.58 s | 1.11 s | 0.72× |
| word_count | 2000000 | 0.20 s | 0.58 s | 0.66 s | 3.23× |
| sort | 3000000 | 0.56 s | 0.28 s | 0.86 s | 3.06× |
| text | 1000000 | 0.83 s | 0.98 s | 1.20 s | 1.45× |

## Peak memory

| Benchmark | Input | C++ | Rust | Sayon | Sayon against the better |
|---|---:|---:|---:|---:|---:|
| fannkuch | 10 | 3.1 MB | 3.4 MB | 3.6 MB | 1.18× |
| nbody | 2000000 | 3.1 MB | 3.4 MB | 3.7 MB | 1.18× |
| spectral_norm | 1500 | 3.1 MB | 3.4 MB | 3.7 MB | 1.16× |
| mandelbrot | 2000 | 3.1 MB | 3.3 MB | 3.6 MB | 1.18× |
| binary_trees | 15 | 6.7 MB | 7.0 MB | 7.1 MB | 1.06× |
| word_count | 2000000 | 3.1 MB | 3.5 MB | 3.7 MB | 1.18× |
| sort | 3000000 | 40.2 MB | 38.2 MB | 42.0 MB | 1.10× |
| text | 1000000 | 172.9 MB | 143.4 MB | 151.2 MB | 1.05× |
