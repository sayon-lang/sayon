# Benchmarks

Twenty programs, each written three times: in Sayon, in C++, and in Rust. Every version does the same work in the same way and prints the same thing, so the only difference is the language and its compiler. Each runs at two or three inputs, from a fraction of a second up to several seconds, so you can see how each language holds up as the work grows.

Each program's folder has a README that says what it does, what's the same in all three versions, what differs and why, and tables of the latest numbers at every input: processor time, the fastest and slowest runs, peak memory, time on the clock, and how long each compiler took. [RESULTS.md](RESULTS.md) puts them all side by side.

## The programs

| Folder | What it does | What it mostly exercises |
|---|---|---|
| [hello](hello/README.md) | Prints one line | Starting and ending a program, the smallest memory and executable |
| [fib](fib/README.md) | Works out Fibonacci numbers by calling itself twice for each | Function calls |
| [fannkuch](fannkuch/README.md) | Flips stacks of pancakes for every starting order of 1 to n, and counts the most flips any order needs | Small integer arrays, tight loops, swaps |
| [nbody](nbody/README.md) | Moves the Sun and the four gas giants step by step under each other's gravity | Floating point math on an array of structures |
| [spectral_norm](spectral_norm/README.md) | Finds the largest singular value of an infinite matrix with the power method | Floating point math, many small function calls |
| [mandelbrot](mandelbrot/README.md) | Counts the points of a grid that stay in the Mandelbrot set | Floating point loops |
| [matrix](matrix/README.md) | Multiplies two square matrices of floating point numbers | Nested loops over arrays, vector instructions |
| [sieve](sieve/README.md) | Counts the primes below a limit with the sieve of Eratosthenes | A big array, walked again and again |
| [binary_trees](binary_trees/README.md) | Builds many complete binary trees, walks them, and drops them, while one tree lives throughout | Lots of small allocations, freed right away |
| [hash_map](hash_map/README.md) | Puts integer keys in a hash map, looks them up, and takes half of them out | Hashing, and a table bigger than the caches |
| [word_count](word_count/README.md) | Makes up words from a handful of syllables and counts how often each comes up | Short strings, a dictionary keyed by them |
| [sort](sort/README.md) | Sorts millions of pseudo-random integers with the standard library | Comparing and moving integers in memory |
| [text](text/README.md) | Formats millions of lines, joins them, splits them up again, and searches them | String formatting, joining, splitting, searching |
| [threads](threads/README.md) | Follows the Collatz sequence from every number below a limit, on four threads | Integer work spread over the processor's cores |
| [nqueens](nqueens/README.md) | Counts the ways to put n queens on an n by n board so that none attacks another | Recursion and backtracking, small arrays of flags |
| [life](life/README.md) | Runs Conway's Game of Life on a board whose edges wrap around | A grid read in nested loops, eight neighbours for every cell |
| [edit_distance](edit_distance/README.md) | Counts the fewest one-letter edits that turn one word into another, for twenty pairs of long words | Dynamic programming over two rows of integers |
| [dijkstra](dijkstra/README.md) | Finds the shortest distances from one place to every other on a random map | An array of roads for every place, and a binary heap |
| [json](json/README.md) | Writes a big JSON document as one string, then parses it back | Building a long string, and a parser walking its bytes |
| [vm](vm/README.md) | Runs a little program on a stack machine, one instruction at a time | An interpreter's loop: a match on each instruction's code |

The random numbers come from the same simple generator in all three versions, written out in each, so every version sees the same data.

## Keeping it fair

The point is to compare languages, not to make Sayon look good, so a few rules:

- Same algorithm everywhere. If one version did less work, the comparison would mean nothing. For instance, Sayon's sort is stable, so the C++ and Rust versions use `std::stable_sort` and `sort`, not their faster unstable sorts. And since Sayon's `split` gives an array of lines and `length` counts characters, the C++ and Rust text programs collect their lines and count characters too.
- Each language is built the way it's normally built for release: C++ with `clang++ -O2`, Rust with `rustc -C opt-level=3`, Sayon with `sayon build --release`. Nobody gets `-march=native` or fast math. C++ and Sayon were also tried at `-O3`, and neither got faster across the board, so both stay at `-O2`, the same LLVM pipeline for both.
- Sayon keeps its safety checks. Integer overflow and array bounds are checked in every Sayon program. C++ checks neither, and Rust's release builds skip the overflow checks, so where Sayon is slower, that's often why. To show how much, every Rust program is also built a second time with `-C overflow-checks=on`, which checks integer arithmetic as Sayon does. That build is listed as *Rust, checked*, and it is only there to compare with: "the better of C++ and Rust" always means the usual builds.
- Each language brings its own standard library and memory allocator, as any program would. Sayon's allocator, and its strings that keep short text inside themselves, are what every Sayon program gets, not something added for these benchmarks. C++ and Rust use their defaults, which on Windows means the system's heap, and none of the programs swaps in another allocator or uses an arena. Where that decides a result, as in binary_trees, the program's README says so.
- The output has to match, at every input. The runner compares what all of them print and stops if they differ.
- Each version is written the way that language is usually written, not tuned by hand. Where a language's own rules give it an edge, such as Rust knowing that two references can't overlap in nbody, or Sayon still lacking a way to make an array of a given size in sieve, the program's README says so instead of hiding it.

## Running them

You need `clang++` and `rustc` on your `PATH`, and a `sayon` to test. Then, from this folder:

```sh
sayon run run.sayon                  # all of them, seven runs each
sayon run run.sayon -- nbody sort    # only these two
sayon run run.sayon -- --runs 3      # fewer runs, rougher numbers
sayon run run.sayon -- --quick       # only the smallest inputs, to try it out
sayon run run.sayon -- --report      # the tables again, from measurements.json
```

Set `SAYON` to measure a `sayon` other than the one on your `PATH`, such as one you just built: `SAYON=../target/release/sayon.exe`. All of them take about half an hour on the laptop the results come from.

The runner builds each program four times into `build/`, timing each compiler. At each input it runs every build once to check the output (which also gets them into the disk cache, and past any virus scan of a new file), then takes turns running them until each has run as many times as you asked. It writes each program's tables into its README, between the two marker lines there, and the whole picture into `RESULTS.md`. After a full run it also keeps every run's numbers in `measurements.json`, from which `--report` writes the tables again without measuring anything. With `--quick` it writes none of these.

## What the numbers mean

**Processor time** is how long the program actually ran on the processor, its own time and the system's on its behalf, added up over all its threads. It comes from the operating system when the program ends. Unlike time on the clock, it hardly moves when the computer is busy with something else, which matters on a laptop with a browser open. Windows counts it in clock ticks of about 16 ms, so very short runs show as 0 or 16 ms.

**Time on the clock** runs from starting the program to its end, as you'd count it waiting. It's the number for threads, since it falls when threads run at once, and for hello, which is all starting and ending. It moves more than processor time when the computer is busy.

**Peak memory** is the most memory the program held at once: the peak working set on Windows, and the maximum resident size on Linux.

**Time to build** is how long the compiler took, start to finish, and **executable** is the size of what it made.

Each number is the median of the runs, and each program's README also shows the fastest and slowest. These are small programs, so treat the numbers as a rough guide to where Sayon is quick and where it isn't. They don't predict how a big program will do.
