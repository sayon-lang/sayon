# mandelbrot

Lays a grid over the plane, 2,000 points on a side, and counts how many points stay inside the Mandelbrot set for 50 steps. A point is out as soon as it lands more than 2 away from the origin, so most of the work is a short floating point loop with an early way out.

## What's the same in all three

The same grid, the same 50 steps, the same test for leaving the set, and a count of the points that never leave. The work per point is in a small function, as written in each language.

## What differs

Not much. There are no arrays to check and no integer arithmetic in the hot loop, so the three come down to how their compilers schedule the same floating point work.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.62 s | 0.44 s | 0.50 s |
| Fastest and slowest run | 0.55 to 0.77 s | 0.39 to 0.48 s | 0.42 to 0.52 s |
| Peak memory, median | 3.1 MB | 3.3 MB | 3.2 MB |
| Executable | 146 KB | 136 KB | 302 KB |

Sayon took 1.14× the time of the faster of the other two, and held 1.05× the memory of the leaner one.

Input 2000, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
