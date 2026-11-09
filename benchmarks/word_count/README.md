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
| 1000000 | 31 ms | 0.16 s | 0.16 s | 94 ms | too short to tell |
| 4000000 | 0.20 s | 0.56 s | 0.44 s | 0.34 s | 1.69× |
| 10000000 | 0.42 s | 1.30 s | 1.28 s | 0.91 s | 2.15× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 1000000 | 0 ms to 0.11 s | 63 ms to 0.22 s | 78 ms to 0.25 s | 47 ms to 0.19 s |
| 4000000 | 0.12 s to 0.25 s | 0.39 s to 0.66 s | 0.39 s to 0.67 s | 0.23 s to 0.48 s |
| 10000000 | 0.27 s to 0.59 s | 1.22 s to 1.45 s | 1.14 s to 1.48 s | 0.73 s to 0.98 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.06× |
| 4000000 | 3.1 MB | 3.5 MB | 3.5 MB | 3.3 MB | 1.06× |
| 10000000 | 3.1 MB | 3.6 MB | 3.5 MB | 3.3 MB | 1.06× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 1000000 | 0.11 s | 0.32 s | 0.32 s | 0.24 s | 2.12× |
| 4000000 | 0.38 s | 1.22 s | 1.24 s | 0.84 s | 2.21× |
| 10000000 | 0.97 s | 3.05 s | 3.04 s | 2.19 s | 2.25× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 5.19 s | 2.46 s | 2.16 s | 1.02 s |
| Executable | 161 KB | 151 KB | 151 KB | 319 KB |

At the largest input, Sayon took 2.15× the processor time of the faster of C++ and Rust, and held 1.06× the memory of the leaner one. With its overflow checks on, Rust took 0.99× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
