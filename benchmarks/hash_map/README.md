# hash_map

Puts integer keys into the standard library's hash map, each with a value, then looks every key up and adds up the values, takes every other key out, and counts how many of the keys are still there. With 200,000, 800,000, and 2 million keys, the largest run makes seven million operations on a table far bigger than the processor's caches.

## What's the same in all three

The same keys, from the same pseudo-random sequence, which has no repeats this short, and the same steps in the same order, each with its language's own hash map: `std::unordered_map` in C++, `HashMap` in Rust, and `Dict` in Sayon.

## What differs

How the maps are built. C++'s keeps each entry in a node of its own, linked from a bucket, so it allocates for every key. Rust's keeps its entries in one array, checks a group of slots at once, and hashes with SipHash, which resists attacks that pick keys to collide and costs more per key. Sayon's keeps the keys and values in insertion order, in arrays of their own, with a table of positions to find them by. Taking a key out leaves an empty place, and the empty places are closed up together, once there are more of them than entries.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 200000 | 0.11 s | 47 ms | 47 ms | 63 ms | too short to tell |
| 800000 | 0.53 s | 0.22 s | 0.23 s | 0.36 s | 1.64× |
| 2000000 | 1.44 s | 0.81 s | 0.81 s | 1.03 s | 1.27× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 200000 | 94 ms to 0.12 s | 31 ms to 78 ms | 47 ms to 63 ms | 47 ms to 63 ms |
| 800000 | 0.52 s to 0.55 s | 0.22 s to 0.23 s | 0.22 s to 0.28 s | 0.34 s to 0.38 s |
| 2000000 | 1.42 s to 1.47 s | 0.80 s to 0.83 s | 0.80 s to 0.83 s | 1.02 s to 1.05 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 200000 | 17.4 MB | 11.9 MB | 11.9 MB | 12.9 MB | 1.08× |
| 800000 | 61.7 MB | 35.6 MB | 35.6 MB | 37.2 MB | 1.05× |
| 2000000 | 142.1 MB | 121.2 MB | 121.2 MB | 81.8 MB | 0.67× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 200000 | 0.11 s | 52 ms | 53 ms | 65 ms | 1.24× |
| 800000 | 0.54 s | 0.23 s | 0.23 s | 0.36 s | 1.57× |
| 2000000 | 1.43 s | 0.81 s | 0.81 s | 1.03 s | 1.28× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.43 s | 1.17 s | 1.13 s | 0.39 s |
| Executable | 153 KB | 141 KB | 142 KB | 310 KB |

At the largest input, Sayon took 1.27× the processor time of the faster of C++ and Rust, and held the least memory, 0.67× that of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
