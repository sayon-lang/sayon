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
| 1000 | 47 ms | 47 ms | 31 ms | 63 ms | too short to tell |
| 2000 | 0.22 s | 0.19 s | 0.16 s | 0.17 s | 0.92× |
| 4000 | 0.84 s | 0.64 s | 0.64 s | 0.64 s | 1.00× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000 | 31 ms to 63 ms | 16 ms to 63 ms | 16 ms to 63 ms | 31 ms to 94 ms |
| 2000 | 0.17 s to 0.30 s | 0.11 s to 0.23 s | 0.11 s to 0.22 s | 0.12 s to 0.23 s |
| 4000 | 0.75 s to 0.92 s | 0.50 s to 0.70 s | 0.56 s to 0.78 s | 0.50 s to 0.69 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 2000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 4000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 0.30 s | 0.23 s | 0.22 s | 0.23 s | 1.02× |
| 2000 | 1.09 s | 0.86 s | 0.86 s | 0.88 s | 1.03× |
| 4000 | 4.35 s | 3.34 s | 3.33 s | 3.44 s | 1.03× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 0.73 s | 1.02 s | 0.93 s | 0.93 s |
| Executable | 146 KB | 136 KB | 136 KB | 302 KB |

At the largest input, Sayon took 1.00× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
