<p align="center">
  <img src="assets/logo-animated.svg" alt="Sayon logo" width="200">
</p>

<h1 align="center">Sayon</h1>

Sayon is a programming language. You write short, indented code, mostly without type annotations, and the compiler works out the types, who owns each value, and when it's freed. There's no garbage collector, and nothing is copied behind your back. A program runs in the interpreter while you work on it, or compiles to a native executable through LLVM.

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

## Why use Sayon

- **It's easy to read.** Blocks are indentation, and where other languages use symbols, Sayon mostly uses words: `and`, `or`, and `not`, and `update` and `own` to say what a function does with its arguments. Strings fill in `{expressions}` where you write them. Types are inferred, and you can write them out where it helps.
- **Memory is safe without a garbage collector.** Ownership is checked when the program compiles, borrowing is worked out for you, and there are no lifetimes to write. Every value is freed once, the moment its owner goes away.
- **The checks stay on.** Integer overflow, array bounds, and conversions between number types are checked in every build, release builds too. A failed check is an error your program can catch with `try`, like any other.
- **Errors help you fix them.** Every error has a stable code, points at the problem, and suggests a fix.
- **One tool does the whole job.** `sayon` runs, builds, checks, tests, formats, and lints programs, starts projects, installs and publishes packages, and talks to your editor.
- **One program runs in many places.** The same code runs on Windows and Linux, on x64 and ARM64, as WebAssembly for WASI and for web pages, and as an Android app.
- **Code from strangers can run safely.** `sayon run --sandbox` runs a file with only the console, clocks, and random numbers, and stops it past set limits of time, memory, and output. The website's playground runs on it.

### How its safety compares with Rust's

Sayon and Rust stop the same memory bugs before a program runs: using a value after it's freed, freeing it twice, a reference that outlives what it points to, and two threads changing the same data at once. Neither has null references. Both let you step outside the rules with `unsafe`, to call C or handle raw memory, and in both, shared handles that point at each other in a circle keep each other alive, which is what weak handles are for.

Where they differ:

| | Rust | Sayon |
|---|---|---|
| Integer overflow | A panic in debug builds. Release builds wrap around without a word, unless you turn on `overflow-checks` | An `OverflowError`, in every build |
| Converting between number types | `as` cuts off or clamps the value without a word. `try_from` checks | Always checked: `integer8(300)` is a `ConversionError` |
| An index past the end, a missing key, division by zero | A panic, which ends the thread unless `catch_unwind` stops it | An error like any other, which `try` can catch |
| What a program can reach | Anything the process can | The same in a normal build. But the compiler works out which capabilities a program needs, such as files or sockets, and refuses code its target can't support, and `sayon run --sandbox` runs a stranger's file with only the console, clocks, random numbers, and code that touches nothing outside |
| Code that runs when you build | Build scripts and procedural macros | None. Sayon has no build scripts or macros, so building a package never runs its code |

So Sayon checks more by default. What it can't match yet is time: Rust's compiler has been used and tested by a great many people for years, and Sayon's is young, so a bug in it could still let through something the rules forbid.

## Features

**The language**

- Indented blocks, plain-word keywords, and strings that fill in `{expressions}`.
- Types worked out by the compiler, which you can write out anywhere. A union such as `integer or nothing` narrows by itself once you've checked it.
- Structures, enums, generics, closures, and methods, and `match`, which checks that every case is covered.
- Ownership: a parameter borrows by default, `update` lends a value to be changed, and `own` hands it over. Views borrow part of an array without copying it.
- Shared values, cells, and weak handles, for data with several owners.
- Errors as values: `try` and `catch` handle them as blocks, and `try` before an expression turns an error into an `Outcome` to check.
- `finally` functions, which run when a value is destroyed, for files, locks, and the like.
- Calls into C, with `unsafe` around what the compiler can't check.

**Running programs**

- An interpreter and a native compiler that give the same results. Every test program runs both ways, and the compiled ones must free everything they allocate.
- Native executables for Windows and Linux, on x64 and ARM64, WebAssembly for WASI and for web pages, and Android apps. `examples/sketch` is one program that runs as a window, a web page, and an app.
- Tasks and channels, threads with mutexes, sockets, TLS, an HTTP client, and other programs run through pipes.
- Text of up to 23 bytes kept inside the string itself on 64-bit systems, and an allocator of Sayon's own, so short strings and small objects cost little.

**The standard library**, written in Sayon, in 13 modules: files and paths, input and output, the operating system, JSON, collections (stacks, queues, lists, ordered maps), text and Unicode, math with seeded random numbers, time with time zones, tasks and threads, networking, hashing, raw memory for C, and graphics with canvases, fonts, PNG and JPEG images, and windows. [The API](docs/api.md) lists every function.

**The tools**

- `sayon run`, `build`, `check`, `test`, `fmt`, `lint`, `init`, `install`, `publish`, `explain`, and `target`.
- A language server with a [VS Code extension](https://github.com/sayon-lang/sayon-vscode): errors as you type, hover, completion, signature help, going to definitions, references, and renaming.
- Packages with versions, a lock file, and a registry.
- A sandbox for running code from people you don't know.

## Limitations

- **It's young.** The language can still change, nothing has been released, and every feature is still marked experimental. The compiler hasn't had years of use behind it.
- **Not every platform yet.** There's no macOS or iOS. Compiled programs on desktop ARM64 can't run tasks yet, and you can't build on one desktop system for another.
- **Slower than C++ and Rust on most programs.** In the latest benchmarks, Sayon took the least time of the three on 2 of the 13 programs long enough to time, and within a fifth of the faster of the others on 4 more. On the other 7 it took up to 2.8 times as long. Rust's sort is nearly three times as fast as Sayon's. Function calls cost more, since every Sayon call also hands back whether an error was raised, so the fib benchmark takes nearly three times as long. Loops over arrays keep their bounds and overflow checks, which stops them using the vector instructions Rust's use, and Rust's borrowing tells its compiler that two references can't overlap, which Sayon's doesn't yet. [RESULTS.md](benchmarks/RESULTS.md) has the numbers.
- **No way yet to make an array of a given size in one step.** A program pushes each element instead, which costs time, and memory while the array grows: the sieve benchmark holds a third more memory than C++ and Rust.
- **Bigger executables.** A small program compiles to about 300 KB, about twice what C++ and Rust make.
- **The interpreter is slow.** `sayon run` interprets the program, at a couple of microseconds a loop step on an ordinary laptop. Use `sayon run --native` or `sayon build` when speed matters.
- **No public package registry yet.** The registry runs on your own computer until the website goes live.
- **Not there yet:** `async` and `await`, macros, a debugger, a REPL, text input and a GPU backend for graphics, TLS servers, and signed Android releases. The [feature ledger](docs/features.md) has each with the milestone it's planned for, and a language feature among them gives an error saying when it's coming, instead of half working.

## Status

Sayon is at version 0.0.1, and nothing has been released yet. The version stays there until the first stable release.

The work is planned in nine milestones: a first usable release, an expressive core, systems and concurrency, portable applications, an ecosystem, a production release, Apple's platforms, advanced tooling, and last, a compiler written in Sayon itself. It's in the fifth now: packages and the registry, the standard library, the language server, and running on desktop ARM64. The [feature ledger](docs/features.md) lists what's done, what's planned, and for which milestone.

## AI assistance

I build Sayon with help from Claude Opus 5.5, an AI model made by Anthropic. It has helped me research, design, write, debug, and test it: the compiler, the runtime, the standard library, the tools, the website, and these docs.

The idea didn't come from the AI. It came from deep research into programming languages, and from being tired of looking at weird symbols and complex, tangled syntax. I wanted a language I'd enjoy reading, so I decided to make one.

## License

Sayon uses the [Apache License 2.0 with LLVM's exception](LICENSE) (`Apache-2.0 WITH LLVM-exception`). The exception means the parts of Sayon that end up inside your program, such as the runtime, can ship with it without carrying Sayon's license or notices. [docs/licenses.md](docs/licenses.md) lists what comes from others and under which licenses.

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

## Benchmarks

[benchmarks](benchmarks) holds fourteen programs written in Sayon, C++, and Rust that do the same work, from printing one line to sorting ten million numbers, and a runner, written in Sayon, that builds them all, checks that they print the same thing, and measures them at two or three input sizes each. Every Rust program is also built with its overflow checks on, to show what checking costs. Each program's README says what differs between its versions and why, [RESULTS.md](benchmarks/RESULTS.md) has the latest numbers side by side, and `measurements.json` every run's.

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
| `crates/runtime` | The runtime library compiled programs link with, and Sayon's allocator |
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
