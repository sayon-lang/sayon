# text

Formats lines like `item 1234: 5678`, 300,000, a million, then three million of them, joins them into one big string, splits that back into an array of lines, and goes through them counting characters and the lines that have a 7 in them.

## What's the same in all three

The same lines, joined and split the same way, and the same counts printed. All three collect the split lines into an array before looking through them, since that's what Sayon's `split` gives. And all three count characters rather than bytes, since that's what Sayon's `length` counts. The text is all ASCII, so the two come to the same number, but each version still does the counting.

## What differs

The C++ version builds each line with `+` on temporary strings. Rust uses `format!`, and Sayon uses string interpolation, which builds the line in one buffer sized up front.

How the lines are kept. They are at most 18 characters long, and Sayon's strings hold up to 23 bytes inside themselves, so none of its lines needs memory of its own, before the join or after the split. Microsoft's `std::string` holds up to 15 characters, which most of these lines pass, and Rust's `String` always allocates. That's why Sayon holds less than half the memory of the other two here.

## Results

<!-- results: run.sayon writes this part -->

Processor time, the median of 7 runs:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 300000 | 0.16 s | 0.20 s | 0.19 s | 0.11 s | 0.70× |
| 1000000 | 0.52 s | 0.61 s | 0.61 s | 0.38 s | 0.73× |
| 3000000 | 1.62 s | 1.84 s | 1.84 s | 1.09 s | 0.67× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 300000 | 0.14 s to 0.17 s | 0.19 s to 0.20 s | 0.19 s to 0.22 s | 0.11 s to 0.12 s |
| 1000000 | 0.52 s to 0.55 s | 0.61 s to 0.66 s | 0.59 s to 0.64 s | 0.36 s to 0.41 s |
| 3000000 | 1.59 s to 1.66 s | 1.83 s to 2.08 s | 1.83 s to 1.95 s | 1.08 s to 1.12 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 300000 | 53.5 MB | 49.2 MB | 49.2 MB | 23.3 MB | 0.47× |
| 1000000 | 172.9 MB | 143.4 MB | 143.4 MB | 67.8 MB | 0.47× |
| 3000000 | 539.7 MB | 425.9 MB | 426.0 MB | 196.9 MB | 0.46× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 300000 | 0.17 s | 0.20 s | 0.20 s | 0.12 s | 0.73× |
| 1000000 | 0.53 s | 0.61 s | 0.61 s | 0.38 s | 0.72× |
| 3000000 | 1.63 s | 1.85 s | 1.86 s | 1.11 s | 0.68× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 3.21 s | 1.61 s | 1.41 s | 1.10 s |
| Executable | 166 KB | 140 KB | 140 KB | 308 KB |

At the largest input, Sayon was the fastest, in 0.67× the processor time of the faster of C++ and Rust, and held the least memory, 0.46× that of the leaner one. With its overflow checks on, Rust took 1.00× its own processor time without them.

Measured on 2026-11-09 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
