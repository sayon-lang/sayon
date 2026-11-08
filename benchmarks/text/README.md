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
| 300000 | 0.16 s | 0.20 s | 0.20 s | 0.11 s | 0.70× |
| 1000000 | 0.55 s | 0.64 s | 0.62 s | 0.38 s | 0.69× |
| 3000000 | 1.64 s | 1.89 s | 2.05 s | 1.11 s | 0.68× |

The fastest and slowest runs:

| Input | C++ | Rust | Rust, checked | Sayon |
|---:|---:|---:|---:|---:|
| 300000 | 0.16 s to 0.17 s | 0.19 s to 0.23 s | 0.19 s to 0.23 s | 94 ms to 0.12 s |
| 1000000 | 0.53 s to 0.70 s | 0.62 s to 0.77 s | 0.61 s to 0.67 s | 0.36 s to 0.38 s |
| 3000000 | 1.61 s to 2.30 s | 1.86 s to 2.80 s | 1.88 s to 2.45 s | 1.09 s to 1.50 s |

Peak memory:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 300000 | 53.5 MB | 49.2 MB | 49.2 MB | 23.3 MB | 0.47× |
| 1000000 | 172.9 MB | 143.4 MB | 143.4 MB | 67.8 MB | 0.47× |
| 3000000 | 539.8 MB | 425.9 MB | 425.9 MB | 196.8 MB | 0.46× |

Time on the clock, from starting the program to its end:

| Input | C++ | Rust | Rust, checked | Sayon | Sayon against the better |
|---:|---:|---:|---:|---:|---:|
| 300000 | 0.16 s | 0.20 s | 0.20 s | 0.12 s | 0.74× |
| 1000000 | 0.54 s | 0.64 s | 0.63 s | 0.38 s | 0.70× |
| 3000000 | 1.64 s | 1.89 s | 2.04 s | 1.12 s | 0.68× |

Building:

| | C++ | Rust | Rust, checked | Sayon |
|---|---:|---:|---:|---:|
| Time to build | 1.71 s | 0.86 s | 0.81 s | 0.43 s |
| Executable | 166 KB | 140 KB | 140 KB | 308 KB |

At the largest input, Sayon was the fastest, in 0.68× the processor time of the faster of C++ and Rust, and held the least memory, 0.46× that of the leaner one. With its overflow checks on, Rust took 1.08× its own processor time without them.

Measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows, 7 runs of each build at each input. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`, and again with `-C overflow-checks=on`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
