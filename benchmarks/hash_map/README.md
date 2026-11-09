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
| 200000 | 47 ms | 47 ms | 47 ms | 47 ms | too short to tell |
| 800000 | 0.33 s | 0.19 s | 0.16 s | 0.22 s | 1.17× |
| 2000000 | 1.00 s | 0.61 s | 0.61 s | 0.69 s | 1.13× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 200000 | 16 ms to 0.12 s | 16 ms to 63 ms | 0 ms to 78 ms | 16 ms to 78 ms |
| 800000 | 0.20 s to 0.61 s | 94 ms to 0.27 s | 0.11 s to 0.19 s | 0.11 s to 0.33 s |
| 2000000 | 0.83 s to 1.11 s | 0.34 s to 0.78 s | 0.38 s to 0.62 s | 0.55 s to 0.78 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 200000 | 17.5 MB | 11.9 MB | 11.9 MB | 12.1 MB | 1.02× |
| 800000 | 61.7 MB | 35.6 MB | 35.6 MB | 36.4 MB | 1.02× |
| 2000000 | 142.1 MB | 121.3 MB | 121.2 MB | 81.0 MB | 0.67× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 200000 | 0.20 s | 88 ms | 87 ms | 0.12 s | 1.33× |
| 800000 | 0.90 s | 0.40 s | 0.41 s | 0.63 s | 1.56× |
| 2000000 | 2.43 s | 1.37 s | 1.38 s | 1.73 s | 1.26× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 2.78 s | 2.20 s | 2.01 s | 0.90 s |
| Executable | 153 KB | 141 KB | 142 KB | 310 KB |

At the largest input, Sayon took 1.13× the processor time of the faster of C++ and Rust, and held the least memory, 0.67× that of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
