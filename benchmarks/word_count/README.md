# word_count

Makes up words from ten syllables, a million, four million, then ten million of them, counts how often each word comes up in a dictionary, then sorts the words by count and prints the five most common. The words are short, two to six letters, and there are only 1,110 different ones, so nearly every word is one the dictionary already has.

## What's the same in all three

The same words, from the same simple random number generator. Each word is built a syllable at a time into a string, counted in the standard library's hash map (`std::unordered_map`, `HashMap`, Sayon's dictionary), and the results are sorted the same way.

## What differs

C++'s `std::string` keeps short strings inside itself, up to 15 characters with Microsoft's library, and Sayon's strings keep up to 23 bytes, so in both none of these words touches the heap. Rust allocates every word. C++'s `++counts[word]` and Rust's `entry` look a word up once. The Sayon version asks `haskey` first and then updates the count, but the dictionary remembers where it found the last key, so the second lookup compares the word with that one entry instead of hashing it again.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0 ms | 0.11 s | 78 ms | 47 ms | too short to tell |
| 4000000 | 94 ms | 0.28 s | 0.19 s | 0.22 s | 2.33× |
| 10000000 | 0.22 s | 0.78 s | 0.53 s | 0.47 s | 2.14× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0 ms to 16 ms | 31 ms to 0.12 s | 31 ms to 0.11 s | 31 ms to 94 ms |
| 4000000 | 31 ms to 0.11 s | 0.14 s to 0.34 s | 0.14 s to 0.31 s | 0.12 s to 0.27 s |
| 10000000 | 0.19 s to 0.33 s | 0.62 s to 1.09 s | 0.45 s to 0.94 s | 0.41 s to 0.81 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.05× |
| 4000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.05× |
| 10000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.05× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.13 s | 0.35 s | 0.27 s | 0.26 s | 2.02× |
| 4000000 | 0.43 s | 1.33 s | 1.02 s | 0.94 s | 2.20× |
| 10000000 | 1.00 s | 3.53 s | 2.63 s | 2.44 s | 2.43× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 4.08 s | 1.95 s | 2.08 s | 1.04 s |
| Executable | 161 KB | 151 KB | 151 KB | 319 KB |

At the largest input, Sayon took 2.14× the processor time of the faster of C++ and Rust, and held 1.05× the memory of the leaner one. With its overflow checks on, Rust took 0.68× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
