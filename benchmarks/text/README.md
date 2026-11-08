# text

Formats a million lines like `item 1234: 5678`, joins them into one big string, splits that back into an array of lines, and goes through them counting characters and the lines that have a 7 in them.

## What's the same in all three

The same lines, joined and split the same way, and the same counts printed. All three collect the split lines into an array before looking through them, since that's what Sayon's `split` gives. And all three count characters rather than bytes, since that's what Sayon's `length` counts. The text is all ASCII, so the two come to the same number, but each version still does the counting.

## What differs

The C++ version builds each line with `+` on temporary strings. Rust uses `format!`, and Sayon uses string interpolation, which builds the line in one buffer sized up front.

How the lines are kept. They are at most 18 characters long, and Sayon's strings hold up to 23 bytes inside themselves, so none of its lines needs memory of its own, before the join or after the split. Microsoft's `std::string` holds up to 15 characters, which most of these lines pass, and Rust's `String` always allocates. That's why Sayon holds less than half the memory of the other two here.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.56 s | 0.66 s | 0.33 s |
| Fastest and slowest run | 0.42 to 0.61 s | 0.58 to 0.78 s | 0.25 to 0.41 s |
| Peak memory, median | 172.9 MB | 143.4 MB | 67.8 MB |
| Executable | 166 KB | 140 KB | 308 KB |

Sayon was the fastest, in 0.58× the time of the faster of the other two, and held the least memory, 0.47× that of the leaner one.

Input 1000000, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
