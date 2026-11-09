# dijkstra

Finds the shortest distance from one place to every other on a pseudo-random map of 100,000, 300,000, and a million places, each with four one-way roads to random places, with Dijkstra's algorithm. It prints how many places it reached, their distances added up, and the farthest. The work is building the map, an array of roads for every place, and running a binary heap that holds the places to visit next.

## What's the same in all three

The same map, the same algorithm, and the same heap: a binary heap kept in an array, written out the same way in all three, rather than `std::priority_queue` in C++ and `BinaryHeap` in Rust, which Sayon doesn't have. A place can go on the heap more than once, and is skipped when it comes off again after its distance is known. Each road is a small structure of two 64-bit integers.

## What differs

Making the road lists: C++ makes a million empty `std::vector`s at once, and Rust and Sayon push an empty array for each place. Each place's list then grows as its four roads are added, in all three.

Sayon checks the index of every array access and every addition. Rust checks indexes it can't prove are in range, and additions only in its checked build. C++ checks neither.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 100000 | 0.11 s | 94 ms | 94 ms | 78 ms | 0.83× |
| 300000 | 0.38 s | 0.28 s | 0.28 s | 0.27 s | 0.94× |
| 1000000 | 1.42 s | 1.03 s | 1.03 s | 0.95 s | 0.92× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 100000 | 0.11 s to 0.14 s | 78 ms to 0.11 s | 78 ms to 0.11 s | 78 ms to 94 ms |
| 300000 | 0.36 s to 0.38 s | 0.28 s to 0.30 s | 0.27 s to 0.31 s | 0.27 s to 0.28 s |
| 1000000 | 1.41 s to 1.45 s | 1.02 s to 1.05 s | 1.02 s to 1.05 s | 0.94 s to 0.97 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 100000 | 16.2 MB | 16.7 MB | 16.7 MB | 14.6 MB | 0.90× |
| 300000 | 41.3 MB | 44.2 MB | 44.2 MB | 39.3 MB | 0.95× |
| 1000000 | 132.9 MB | 127.1 MB | 127.0 MB | 111.5 MB | 0.88× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 100000 | 0.11 s | 88 ms | 93 ms | 81 ms | 0.91× |
| 300000 | 0.38 s | 0.29 s | 0.29 s | 0.26 s | 0.92× |
| 1000000 | 1.42 s | 1.03 s | 1.03 s | 0.95 s | 0.92× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.30 s | 0.78 s | 0.92 s | 0.82 s |
| Executable | 151 KB | 138 KB | 138 KB | 305 KB |

At the largest input, Sayon was the fastest, in 0.92× the processor time of the faster of C++ and Rust, and held the least memory, 0.88× that of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
