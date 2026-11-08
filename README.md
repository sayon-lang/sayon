<p align="center">
  <img src="assets/logo.svg" alt="Sayon logo" width="200">
</p>

<h1 align="center">Sayon</h1>

<p align="center">Programming made easy. Safety made automatic.</p>

Sayon is a programming language that reads like a script and runs like a compiled program. You write short, indented code, mostly without type annotations, and the compiler works out the types, who owns each value, and when it's freed. There's no garbage collector, and nothing is copied behind your back.

The name is Cebuano for "easy", which is the whole idea: ordinary programs shouldn't need much ceremony, and when you do care about what your program costs, you can see it.

```sayon
function total(values: Array<integer>) -> integer
    result = 0
    for value in values
        result += value
    return result

numbers = [1, 2, 3]
print("Sum: {total(numbers)}")
print(length(numbers))  # still usable: total only borrowed it
```

`total` borrows the array, so `numbers` is still there afterwards. A function that changes its argument says `update`, and one that keeps it says `own`. When you get it wrong, the compiler points at both places and suggests a fix:

```text
error[E_MOVED_VALUE]: `names` was transferred to `team`
  --> moved.sayon:3:7
   |
 2 | team = names
   |        ----- ownership transferred here
 3 | print(names)
   |       ^^^^^ `names` no longer owns the array
   |
   = help: use `team` here, or write `team = clone(names)` when you need two independent copies
```

Every error has a code like that one, and `sayon explain E_MOVED_VALUE` says more about it.

## What it's like

- Blocks are indentation, keywords are plain words, and strings fill in `{expressions}`.
- Types are inferred, and you can write them out where you like. A union such as `integer or nothing` narrows by itself once you've checked it.
- Ownership is checked when the program compiles, borrowing is worked out for you, and values are freed the moment their owner goes away.
- Integer overflow and array bounds are checked, always.
- One command covers the whole workflow: `sayon run`, `build`, `check`, `test`, `fmt`, `lint`, `init`, `install`, and `publish`.

## Where it stands

Sayon is young. It runs real programs, but the language can still change, and nothing has been released yet. Today there is:

- An interpreter and a native compiler that give the same results. Every example runs both ways, and the tests check that it frees everything it allocated.
- Native executables for Windows and Linux, on x64 and ARM64, WebAssembly for WASI and for web pages, and Android apps. macOS comes later. `examples/sketch` is one program that runs as a window, a web page, and an app.
- Structures, enums, generics, closures, `match` with exhaustiveness checks, and errors as values you can `try`.
- Tasks and channels, threads with mutexes, sockets, TLS, an HTTP client, other programs with pipes, and calls into C.
- A standard library written in Sayon: files, JSON, collections, time with time zones, text, math, hashing, and graphics with canvases, fonts, PNG and JPEG images, and windows.
- Tests, a formatter, a linter, a language server with a VS Code extension, packages with a lock file, and a package registry, which so far runs only locally.
- A sandbox, `sayon run --sandbox`, for running code from people you don't know. The website's playground runs on it.

Features planned for later are refused with a message that says when they're coming, instead of half working.

## Speed

Programs compile to native code through LLVM, with the overflow and bounds checks left in. [benchmarks](benchmarks) holds eight small programs written in Sayon, C++, and Rust, doing the same work in each, and a runner that builds and measures all of them. Each program has its own README with a full table. Here's processor time and peak memory on an Intel Core i3-7100U laptop with Windows 10, median of seven runs:

| Benchmark | C++ | Rust | Sayon | Memory: C++, Rust, Sayon |
|---|---:|---:|---:|---:|
| [binary_trees](benchmarks/binary_trees/README.md) | 0.80 s | 0.81 s | 0.27 s | 6.7, 7.0, 6.2 MB |
| [text](benchmarks/text/README.md) | 0.56 s | 0.66 s | 0.33 s | 172.9, 143.4, 67.8 MB |
| [fannkuch](benchmarks/fannkuch/README.md) | 0.28 s | 0.30 s | 0.30 s | 3.1, 3.4, 3.2 MB |
| [mandelbrot](benchmarks/mandelbrot/README.md) | 0.62 s | 0.44 s | 0.50 s | 3.1, 3.3, 3.2 MB |
| [spectral_norm](benchmarks/spectral_norm/README.md) | 0.22 s | 0.17 s | 0.27 s | 3.1, 3.4, 3.3 MB |
| [nbody](benchmarks/nbody/README.md) | 0.23 s | 0.11 s | 0.25 s | 3.1, 3.4, 3.2 MB |
| [word_count](benchmarks/word_count/README.md) | 0.12 s | 0.42 s | 0.31 s | 3.1, 3.5, 3.3 MB |
| [sort](benchmarks/sort/README.md) | 0.41 s | 0.20 s | 0.58 s | 40.2, 38.2, 38.1 MB |

Sayon is the fastest of the three on binary_trees and text, by a wide margin, and holds the least memory on both. On binary_trees that's mostly its allocator, since the C++ and Rust versions get their memory from the system's. On text it's short strings: Sayon keeps up to 23 bytes inside the string itself, so none of the million lines needs memory of its own. It also beats Rust on word_count, and all three come out about even on fannkuch.

It's slower on the rest. On spectral_norm, part of that is the overflow checks, which C++ and Rust leave out of release builds. Rust's sort is a carefully tuned driftsort, and in nbody Rust knows that two references can't point at the same body, which neither C++ nor Sayon can tell. C++ is faster still on word_count. Each program's README says what differs.

Sayon holds less memory than Rust on all eight, and less than C++ on binary_trees, text, and sort. On the small ones it's 0.1 to 0.2 MB more than C++, which is its runtime at startup. Its executables are about twice the size of theirs, around 300 KB.

The laptop was busy while these ran, as it usually is, so a number can move by a fifth or so from one run to the next. The three versions take turns, though, so they share the same load, and the per-program READMEs show the fastest and slowest runs. [results.md](benchmarks/results.md) has everything side by side, and the [benchmarks README](benchmarks/README.md) explains how they're kept fair and how to run them yourself.

## Safety

Sayon and Rust stop the same memory bugs before a program runs: using a value after it's freed, freeing it twice, a reference that outlives what it points to, and two threads changing the same data at once. Neither has null references. Both let you step outside the rules with `unsafe`, to call C or handle raw memory, and in both, shared handles that point at each other in a circle keep each other alive, which is what weak handles are for.

Where they differ:

| | Rust | Sayon |
|---|---|---|
| Integer overflow | A panic in debug builds. Release builds wrap around without a word, unless you turn on `overflow-checks` | An `OverflowError`, in every build |
| Converting between number types | `as` cuts off or clamps the value without a word. `try_from` checks | Always checked: `integer8(300)` is a `ConversionError` |
| An index past the end, a missing key, division by zero | A panic, which ends the thread unless `catch_unwind` stops it | An error like any other, which `try` can catch |
| What a program can reach | Anything the process can | The same in a normal build. But the compiler works out which capabilities a program needs, such as files or sockets, and refuses code its target can't support, and `sayon run --sandbox` runs a stranger's file with only the console, clocks, random numbers, and code that touches nothing outside |
| Code that runs when you build | Build scripts and procedural macros | None. Sayon has no build scripts or macros, so building a package never runs its code |

So Sayon checks more by default. The thing it can't match yet is time: Rust's compiler has been used and tested by a great many people for years, and Sayon's is young, so a bug in it could still let through something the rules forbid.

## Installing

There's no release yet, so for now you build the installer yourself (see below). Once there is one, you'll download the installer for your system from the [releases page](https://github.com/sayon-lang/sayon/releases):

- On Windows, `sayon-<version>-windows-x64-setup.exe` installs Sayon for your account, without administrator rights. Uninstall it from Settings > Apps.
- On Linux, `sayon-<version>-linux-x64-setup` installs into `~/.local/share/sayon` and links `~/.local/bin/sayon`.

[Installing](https://github.com/sayon-lang/www.sayon-lang.org/blob/main/docs/get-started/installing.md) covers the other systems and every option.

## Documentation

The guides live with the website, in [www.sayon-lang.org](https://github.com/sayon-lang/www.sayon-lang.org/tree/main/docs). Good places to start:

- [Your first program](https://github.com/sayon-lang/www.sayon-lang.org/blob/main/docs/get-started/first-program.md) walks through a script, a project, tests, and formatting.
- [All the guides](https://github.com/sayon-lang/www.sayon-lang.org/blob/main/docs/README.md), sorted into the language, the standard library, the tools, and references.

The references that change along with the compiler stay here, in `docs`:

- [Features](docs/features.md): what's done, what's planned, and when.
- [Error codes](docs/errors.md): what every error means. `sayon explain` prints the same text.
- [Standard library API](docs/api.md): everything the standard library exports, generated from `stdlib/` by `scripts/api_inventory.sayon`.
- [Implementation notes](docs/implementation.md): the grammar, ownership rules, cleanup order, data layouts, and how errors cross function calls.
- [Licenses](docs/licenses.md): what in Sayon, and in the programs it builds, comes from others, and under which licenses.

## Building from source

The compiler is written in Rust. `rust-toolchain.toml` pins the version, and `rustup` fetches it for you.

```sh
cargo build
cargo run -p sayon-cli -- run tests/programs/functions.sayon
cargo run -p sayon-cli -- build tests/programs/functions.sayon -o functions.exe
cargo test
```

Native builds need [LLVM](https://releases.llvm.org)'s clang when you compile. Sayon looks for it on `PATH`, in LLVM's usual install folder, or wherever `SAYON_CLANG` points. The programs it builds don't need it.

`scripts/package.ps1` builds the installer for Windows, `dist/sayon-<version>-windows-x64-setup.exe`, along with a zip of the same files and `dist/SHA256SUMS.txt`. On Linux, `scripts/package.sh` builds `dist/sayon-<version>-linux-x64-setup` and a tarball.

With `sayon` on your `PATH`, a new project goes like this:

```sh
sayon init hello
cd hello
sayon run       # Hello, world!
sayon test
sayon build     # build/debug/hello
```

[examples](examples) has a few programs to read: a word counter, a directory inventory, an HTTP file server that runs a task per connection, a word counter that spreads files over threads, and a list kept in C memory that C sorts. `tests/programs` holds programs with their expected output, and `tests/errors` holds broken programs with the errors they should produce.

## Repository layout

| Path | What's there |
|---|---|
| `crates/target` | The systems Sayon builds for, and what each one can do |
| `crates/syntax` | Source files, diagnostics, the lexer, and the parser |
| `crates/check` | Name resolution, type inference, and ownership checking |
| `crates/ir` | The intermediate form, with moves, drops, and error paths spelled out |
| `crates/interp` | The interpreter |
| `crates/driver` | Loads a program's modules and runs the compiler's phases |
| `crates/codegen` | Native code, by way of LLVM IR |
| `crates/runtime` | The runtime library compiled programs link with |
| `crates/lsp` | The language server behind `sayon lsp` |
| `crates/cli` | The `sayon` command |
| `crates/installer` | The installer, `sayon-setup` |
| `stdlib` | The standard library, written in Sayon |
| `benchmarks` | Programs in Sayon, C++, and Rust, and the runner that compares them |
| `examples` | Programs to read, and `sketch`, which runs everywhere Sayon does |
| `tests` | Programs with expected output, and expected errors |
| `docs` | Features, error codes, implementation notes, and licenses |
| `scripts` | Packaging for each system |
| `assets` | The logo and icons |

A few other repositories sit beside this one:

| Repository | What it holds |
| --- | --- |
| [www.sayon-lang.org](https://github.com/sayon-lang/www.sayon-lang.org) | The website, written in Sayon: the guides, the playground, the package registry, and accounts |
| [packages-index](https://github.com/sayon-lang/packages-index) | The registry's index, which `sayon install` reads |
| [sayon-vscode](https://github.com/sayon-lang/sayon-vscode) | The VS Code extension |

If you check out the website and the extension next to this repository, as `../www.sayon-lang.org` and `../sayon-vscode`, `cargo test` tests them with the `sayon` built here, and says it skipped them otherwise. `SAYON_REPOSITORIES` can name another folder that holds them.

## License

Sayon uses the [Apache License 2.0 with LLVM's exception](LICENSE) (`Apache-2.0 WITH LLVM-exception`). The exception means the parts of Sayon that end up inside your program, such as the runtime, can ship with it without carrying Sayon's license or notices. [docs/licenses.md](docs/licenses.md) lists what comes from others and under which licenses.
