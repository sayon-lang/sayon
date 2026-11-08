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

Programs compile to native code through LLVM, with the overflow and bounds checks left in. [benchmarks](benchmarks) holds eight small programs written in Sayon, C++, and Rust, doing the same work in each, and a runner that builds and measures all of them. Processor time on an Intel Core i3-7100U laptop with Windows 10, median of five runs:

| Benchmark | C++ | Rust | Sayon |
|---|---:|---:|---:|
| binary_trees | 1.53 s | 1.58 s | 1.11 s |
| mandelbrot | 0.89 s | 0.69 s | 0.73 s |
| fannkuch | 0.48 s | 0.52 s | 0.66 s |
| nbody | 0.42 s | 0.22 s | 0.45 s |
| spectral_norm | 0.36 s | 0.34 s | 0.56 s |
| text | 0.83 s | 0.98 s | 1.20 s |
| sort | 0.56 s | 0.28 s | 0.86 s |
| word_count | 0.20 s | 0.58 s | 0.66 s |

Sayon is the fastest of the three at building and dropping trees, ahead of C++ on mandelbrot, and level with C++ on nbody, where Rust is twice as fast as both. It's slower on the rest, partly because of the checks C++ and Rust leave out of release builds, and on word_count because C++ keeps short strings inside the string itself. Memory use is close to theirs: about half a megabyte more for the small programs, a few megabytes more on sort, and less than C++ on text. The [benchmarks README](benchmarks/README.md) explains each program and how to run them yourself, and [results.md](benchmarks/results.md) has the memory numbers too.

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
