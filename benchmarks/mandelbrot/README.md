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
| 1000 | 0.19 s | 0.14 s | 0.14 s | 0.16 s | 1.11× |
| 2000 | 0.69 s | 0.53 s | 0.53 s | 0.55 s | 1.03× |
| 4000 | 2.77 s | 2.09 s | 2.09 s | 2.16 s | 1.03× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000 | 0.16 s to 0.20 s | 0.12 s to 0.16 s | 0.12 s to 0.14 s | 0.14 s to 0.17 s |
| 2000 | 0.67 s to 0.70 s | 0.53 s to 0.55 s | 0.50 s to 0.55 s | 0.48 s to 0.56 s |
| 4000 | 2.70 s to 2.81 s | 2.06 s to 2.17 s | 2.05 s to 2.12 s | 2.09 s to 2.19 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 2000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |
| 4000 | 3.1 MB | 3.3 MB | 3.3 MB | 3.2 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000 | 0.18 s | 0.14 s | 0.14 s | 0.15 s | 1.05× |
| 2000 | 0.71 s | 0.55 s | 0.55 s | 0.56 s | 1.03× |
| 4000 | 2.93 s | 2.14 s | 2.14 s | 2.20 s | 1.03× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 0.45 s | 0.86 s | 0.56 s | 0.52 s |
| Executable | 146 KB | 136 KB | 136 KB | 302 KB |

At the largest input, Sayon took 1.03× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
