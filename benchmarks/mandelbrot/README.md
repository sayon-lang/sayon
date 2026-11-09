# mandelbrot

Lays a grid over the plane, 1,000, 2,000, then 4,000 points on a side, and counts how many points stay inside the Mandelbrot set for 50 steps. A point is out as soon as it lands more than 2 away from the origin, so most of the work is a short floating point loop with an early way out.

## What's the same in all three

The same grid, the same 50 steps, the same test for leaving the set, and a count of the points that never leave. The work per point is in a small function, as written in each language.

## What differs

Not much. There are no arrays to check and no integer arithmetic in the hot loop, so the three come down to how their compilers schedule the same floating point work.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 94 ms | 31 ms | 78 ms | 94 ms | too short to tell |
| 2000 | 0.31 s | 0.27 s | 0.31 s | 0.34 s | 1.29× |
| 4000 | 1.59 s | 1.20 s | 1.17 s | 1.34 s | 1.12× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000 | 63 ms to 0.12 s | 16 ms to 94 ms | 47 ms to 94 ms | 47 ms to 0.11 s |
| 2000 | 0.28 s to 0.45 s | 0.19 s to 0.38 s | 0.25 s to 0.34 s | 0.25 s to 0.39 s |
| 4000 | 1.52 s to 1.83 s | 1.16 s to 1.33 s | 1.08 s to 1.27 s | 1.19 s to 1.47 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 2000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 4000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 0.22 s | 0.18 s | 0.18 s | 0.18 s | 1.04× |
| 2000 | 0.85 s | 0.65 s | 0.62 s | 0.62 s | 0.95× |
| 4000 | 3.83 s | 2.96 s | 2.92 s | 3.04 s | 1.03× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 0.67 s | 1.19 s | 1.22 s | 0.76 s |
| Executable | 146 KB | 136 KB | 136 KB | 302 KB |

At the largest input, Sayon took 1.12× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 0.97× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
