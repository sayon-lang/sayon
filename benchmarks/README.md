# Benchmarks

Eight small programs, each written three times: in Sayon, in C++, and in Rust. Every version does the same work in the same way and prints the same thing, so the only difference is the language and its compiler.

Each program's folder has a README that says what it does, what's the same in all three versions, what differs and why, and a table of the latest numbers: processor time, the fastest and slowest run, peak memory, and the size of each executable. [results.md](results.md) puts them all side by side.

## The programs

| Folder | What it does | What it mostly exercises |
|---|---|---|
| [fannkuch](fannkuch/README.md) | Flips stacks of pancakes for every starting order of 1 to n, and counts the most flips any order needs | Small integer arrays, tight loops, swaps |
| [nbody](nbody/README.md) | Moves the Sun and the four gas giants step by step under each other's gravity | Floating point math on an array of structures |
| [spectral_norm](spectral_norm/README.md) | Finds the largest singular value of an infinite matrix with the power method | Floating point math, many small function calls |
| [mandelbrot](mandelbrot/README.md) | Counts the points of a grid that stay in the Mandelbrot set | Floating point loops |
| [binary_trees](binary_trees/README.md) | Builds many complete binary trees, walks them, and drops them, while one tree lives throughout | Lots of small allocations, freed right away |
| [word_count](word_count/README.md) | Makes up words from a handful of syllables and counts how often each comes up | Short strings, a dictionary keyed by them |
| [sort](sort/README.md) | Sorts millions of pseudo-random integers with the standard library | Comparing and moving integers in memory |
| [text](text/README.md) | Formats a million lines, joins them, splits them up again, and searches them | String formatting, joining, splitting, searching |

The random numbers come from the same simple generator in all three versions, written out in each, so every version sees the same data.

## Keeping it fair

The point is to compare languages, not to make Sayon look good, so a few rules:

- Same algorithm everywhere. If one version did less work, the comparison would mean nothing. For instance, Sayon's sort is stable, so the C++ and Rust versions use `std::stable_sort` and `sort`, not their faster unstable sorts. And since Sayon's `split` gives an array of lines and `length` counts characters, the C++ and Rust text programs collect their lines and count characters too.
- Each language is built the way it's normally built for release: C++ with `clang++ -O2`, Rust with `rustc -C opt-level=3`, Sayon with `sayon build --release`. Nobody gets `-march=native` or fast math. C++ and Sayon were also tried at `-O3`, and neither got faster across the board, so both stay at `-O2`, the same LLVM pipeline for both.
- Sayon keeps its safety checks. Integer overflow and array bounds are checked in every Sayon program. C++ checks neither, and Rust's release builds skip the overflow checks, so where Sayon is slower, that's often why.
- Each language brings its own standard library and memory allocator, as any program would. Sayon's allocator, and its strings that keep short text inside themselves, are what every Sayon program gets, not something added for these benchmarks. C++ and Rust use their defaults, which on Windows means the system's heap, and none of the programs swaps in another allocator or uses an arena. Where that decides a result, as in binary_trees, the program's README says so.
- The output has to match. The runner compares what all three print and stops if they differ.
- Each version is written the way that language is usually written, not tuned by hand. Where a language's own rules give it an edge, such as Rust knowing that two references can't overlap in nbody, the program's README says so instead of hiding it.

## Running them

You need `clang++` and `rustc` on your `PATH`, and a `sayon` to test. Then, from this folder:

```sh
sayon run run.sayon                  # all of them, five runs each
sayon run run.sayon -- nbody sort    # only these two
sayon run run.sayon -- --runs 9      # more runs, steadier numbers
```

Set `SAYON` to measure a `sayon` other than the one on your `PATH`, such as one you just built: `SAYON=../target/release/sayon.exe`.

The runner builds each program into `build/`, runs every version once to check the output (which also gets them into the disk cache, and past any virus scan of a new file), then takes turns running C++, Rust, and Sayon until each has run as many times as you asked. It writes each program's table into its README, between the two marker lines there, and the summary into `results.md`.

## What the numbers mean

**Processor time** is how long the program actually ran on the processor, its own time and the system's on its behalf, added up over all its threads. It comes from the operating system when the program ends. Unlike time on the clock, it hardly moves when the computer is busy with something else, which matters on a laptop with a browser open.

**Peak memory** is the most memory the program held at once: the peak working set on Windows, and the maximum resident size on Linux.

These are small programs, so treat the numbers as a rough guide to where Sayon is quick and where it isn't. They don't predict how a big program will do.
