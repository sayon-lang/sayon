# spectral_norm

Works out the largest singular value of a particular infinite matrix, using the power method on its top-left n by n corner. Each matrix entry is computed on the fly from its row and column with a small formula, so the inner loop calls a tiny function millions of times.

## What's the same in all three

Ten rounds of multiplying a vector by the matrix and by its transpose, with the entries computed by the same formula, and the result printed to nine decimal places.

## What differs

The entry formula does integer arithmetic, `(i + j) * (i + j + 1) / 2 + i + 1`. Sayon checks each of those operations for overflow, and the checks keep LLVM from turning the inner loop into vector instructions. C++ and Rust release builds don't check, so their compilers can.

## Results

<!-- results: run.sayon writes this part -->

| | C++ | Rust | Sayon |
|---|---:|---:|---:|
| Processor time, median | 0.22 s | 0.17 s | 0.27 s |
| Fastest and slowest run | 0.12 to 0.25 s | 0.14 to 0.19 s | 0.22 to 0.34 s |
| Peak memory, median | 3.1 MB | 3.4 MB | 3.3 MB |
| Executable | 149 KB | 160 KB | 308 KB |

Sayon took 1.55× the time of the faster of the other two, and held 1.06× the memory of the leaner one.

Input 1500, 7 runs of each, measured on 2026-11-08 on Intel(R) Core(TM) i3-7100U CPU @ 2.40GHz, windows. C++: clang version 23.1.4 (https://github.com/llvm/llvm-project 7a941dfb2877a0b8cf30323fefa85ad019e0b0f5) with `-O2`. Rust: rustc 1.98.1 (48a229cea 2026-09-01) with `-C opt-level=3`. Sayon: sayon 0.0.1 with `--release`.

<!-- end of results -->
