# json

Writes a JSON document of 100,000, 300,000, and a million records as one long string, the largest about 90 MB, then reads it back with a small parser that walks the bytes. Each record has a number, a name, a list of two tags, a price, and a boolean. The parser counts the records, adds up the numbers, and counts the letters of the strings and the `true`s.

## What's the same in all three

The same document, built by adding each record to the end of one string, and the same parser, which looks at one byte at a time and calls itself for the values inside objects and lists. None of them uses a JSON library: Sayon has one, but C++ has none in its standard library, so all three use the same hand-written parser instead.

## What differs

Writing a record. C++ joins temporary strings with `+` and appends the result, Rust writes straight into the document with `write!`, and Sayon builds each record with interpolation and appends it. Each is the usual way in its language.

Reading the bytes. C++ indexes its `std::string`, and Rust a byte slice of its `String`. Sayon takes a view of the string's bytes with `strings.byte_view`, which copies nothing, and indexes that, checking each index against the view.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 100000 | 78 ms | 47 ms | 47 ms | 63 ms | too short to tell |
| 300000 | 0.23 s | 0.11 s | 0.12 s | 0.17 s | 1.57× |
| 1000000 | 0.73 s | 0.38 s | 0.39 s | 0.59 s | 1.58× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 100000 | 63 ms to 78 ms | 31 ms to 63 ms | 31 ms to 47 ms | 47 ms to 78 ms |
| 300000 | 0.20 s to 0.23 s | 94 ms to 0.12 s | 0.11 s to 0.12 s | 0.16 s to 0.19 s |
| 1000000 | 0.72 s to 0.75 s | 0.36 s to 0.39 s | 0.38 s to 0.41 s | 0.58 s to 0.59 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 100000 | 17.0 MB | 19.9 MB | 19.9 MB | 20.2 MB | 1.19× |
| 300000 | 47.0 MB | 35.9 MB | 35.9 MB | 35.7 MB | 0.99× |
| 1000000 | 147.9 MB | 132.0 MB | 131.9 MB | 128.7 MB | 0.98× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 100000 | 82 ms | 48 ms | 49 ms | 69 ms | 1.44× |
| 300000 | 0.23 s | 0.11 s | 0.12 s | 0.18 s | 1.60× |
| 1000000 | 0.73 s | 0.37 s | 0.40 s | 0.60 s | 1.59× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.32 s | 0.69 s | 0.70 s | 0.52 s |
| Executable | 155 KB | 138 KB | 139 KB | 308 KB |

At the largest input, Sayon took 1.58× the processor time of the faster of C++ and Rust, and held the least memory, 0.98× that of the leaner one. With its overflow checks on, Rust took 1.04× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
