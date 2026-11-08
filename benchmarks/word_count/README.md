# word_count

Makes up two million words from ten syllables, counts how often each word comes up in a dictionary, then sorts the words by count and prints the five most common. The words are short, two to six letters, and there are only 1,110 different ones, so nearly every word is one the dictionary already has.

## What's the same in all three

The same words, from the same simple random number generator. Each word is built a syllable at a time into a string, counted in the standard library's hash map (`std::unordered_map`, `HashMap`, Sayon's dictionary), and the results are sorted the same way.

## What differs

C++'s `std::string` keeps short strings inside itself, up to 15 characters with Microsoft's library, and Sayon's strings keep up to 23 bytes, so in both none of these words touches the heap. Rust allocates every word. C++'s `++counts[word]` and Rust's `entry` look a word up once. The Sayon version asks `haskey` first and then updates the count, but the dictionary remembers where it found the last key, so the second lookup compares the word with that one entry instead of hashing it again.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.12 s | 0.42 s | 0.31 s |
| Fastest and slowest run | 0.09 to 0.16 s | 0.38 to 0.47 s | 0.23 to 0.36 s |
| Peak memory, median | 3.1 MB | 3.5 MB | 3.3 MB |
| Executable | 161 KB | 151 KB | 318 KB |

Sayon took 2.50× the time of the faster of the other two, and held 1.06× the memory of the leaner one.

Input 2000000, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
