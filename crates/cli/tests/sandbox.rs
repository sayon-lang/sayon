/*!
`sayon run --sandbox`: a program from anyone runs, kept from what the
sandbox does not give it, before it runs and while it does, and within
its limits of time, memory, and output.
*/

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

struct Outcome {
    stdout: String,
    stderr: String,
    status: i32,
    took: Duration,
}

/** A folder of its own for a test, empty. */
fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sayon-sandbox-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a folder");
    dir
}

/** Runs `main.sayon` of `dir` in the sandbox. */
fn run_in(dir: &Path) -> Outcome {
    let started = Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_sayon"))
        .args(["run", "--sandbox", "--color", "never", "main.sayon"])
        .current_dir(dir)
        .output()
        .expect("run sayon");
    Outcome {
        stdout: String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
        stderr: String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"),
        status: output.status.code().unwrap_or(-1),
        took: started.elapsed(),
    }
}

/** Runs `source` as `main.sayon` in a folder of its own, in the sandbox. */
fn run(name: &str, source: &str) -> Outcome {
    let dir = folder(name);
    std::fs::write(dir.join("main.sayon"), source).expect("the program");
    let outcome = run_in(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    outcome
}

/** The program was refused before any of it ran, for `needing`. */
fn refused(outcome: &Outcome, needing: &str) {
    assert_eq!(outcome.status, 2, "{}", outcome.stderr);
    assert!(outcome.stdout.is_empty(), "{}", outcome.stdout);
    assert!(
        outcome.stderr.contains("programs in the sandbox cannot")
            && outcome.stderr.contains(&format!("this needs `{needing}`")),
        "{}",
        outcome.stderr
    );
}

/** The sandbox stopped the program, saying one of `reasons`. */
fn stopped(outcome: &Outcome, reasons: &[&str]) {
    assert_eq!(outcome.status, 3, "{}", outcome.stderr);
    let said = outcome
        .stderr
        .lines()
        .find(|line| line.starts_with("the sandbox stopped the program: "))
        .unwrap_or_else(|| panic!("no stop in {}", outcome.stderr));
    assert!(
        reasons.iter().any(|reason| said.ends_with(reason)),
        "{said}"
    );
}

#[test]
fn a_program_runs_and_its_status_comes_back() {
    let hello = run(
        "hello",
        "strings = import(\"standard/strings.sayon\")\nprint(strings.upper(\"hello\"), supports(\"console\"), supports(\"files\"))\n",
    );
    assert_eq!(hello.status, 0, "{}", hello.stderr);
    assert_eq!(hello.stdout, "HELLO true false\n");
    let seven = run(
        "seven",
        "function main() -> integer\n    print(\"done\")\n    return 7\n",
    );
    assert_eq!(
        (seven.status, seven.stdout.as_str()),
        (7, "done\n"),
        "{}",
        seven.stderr
    );
}

#[test]
fn what_the_sandbox_does_not_give_is_refused_before_anything_runs() {
    let programs = [
        (
            "files",
            "print(\"first\")\nprint(readfile(\"secret.txt\"))\n",
        ),
        (
            "files",
            "io = import(\"standard/io.sayon\")\nio.write_file(\"made.txt\", \"x\")\n",
        ),
        (
            "processes",
            "os = import(\"standard/os.sayon\")\nos.run(\"cmd\", [\"/c\", \"echo\"])\n",
        ),
        (
            "environment",
            "os = import(\"standard/os.sayon\")\nprint(os.environment(\"PATH\"))\n",
        ),
        (
            "network",
            "network = import(\"standard/network.sayon\")\nconnection = network.connect(\"127.0.0.1\", 9)\n",
        ),
        (
            "input",
            "io = import(\"standard/io.sayon\")\nprint(io.read_line())\n",
        ),
        (
            "foreign",
            "foreign \"c\"\n    function abs(value: c_int) -> c_int\n\nunsafe\n    print(abs(-3))\n",
        ),
        (
            "threads",
            "threads = import(\"standard/threads.sayon\")\ntask = threads.spawn_parallel(function() -> integer\n    return 1\n)\nprint(threads.join(task))\n",
        ),
    ];
    for (index, (needing, source)) in programs.iter().enumerate() {
        refused(&run(&format!("refused-{index}"), source), needing);
    }
}

#[test]
fn platform_branches_are_checked_for_the_platform_that_runs() {
    let branch = run(
        "branch",
        "if platform != \"web\"\n    print(readfile(\"secret.txt\"))\nelse\n    print(\"a web page\")\n",
    );
    refused(&branch, "files");
    let around = run(
        "around",
        "if supports(\"files\")\n    print(readfile(\"secret.txt\"))\nelse\n    print(\"no files here\")\n",
    );
    assert_eq!(
        (around.status, around.stdout.as_str()),
        (0, "no files here\n"),
        "{}",
        around.stderr
    );
}

#[test]
fn only_the_standard_library_is_imported() {
    let dir = folder("imports");
    std::fs::write(
        dir.join("helper.sayon"),
        "function secret() -> string\n    return \"kept\"\n",
    )
    .unwrap();
    for (index, path) in [
        "helper.sayon",
        "../outside.sayon",
        "dependencies/helper/helper.sayon",
    ]
    .iter()
    .enumerate()
    {
        std::fs::write(
            dir.join("main.sayon"),
            format!("helper = import(\"{path}\")\nprint(helper.secret())\n"),
        )
        .unwrap();
        let outcome = run_in(&dir);
        assert_eq!(outcome.status, 2, "{index}: {}", outcome.stderr);
        assert!(
            outcome.stderr.contains("E_SANDBOX_IMPORT") && !outcome.stderr.contains("kept"),
            "{index}: {}",
            outcome.stderr
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_project_around_the_file_is_not_used() {
    let dir = folder("project");
    std::fs::write(dir.join("sayon.toml"), "this is not a manifest [\n").unwrap();
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("src/lib.sayon"),
        "function secret() -> string\n    return \"kept\"\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("play")).unwrap();
    std::fs::write(dir.join("play/main.sayon"), "print(\"on its own\")\n").unwrap();
    let alone = run_in(&dir.join("play"));
    assert_eq!(
        (alone.status, alone.stdout.as_str()),
        (0, "on its own\n"),
        "{}",
        alone.stderr
    );
    std::fs::write(
        dir.join("play/main.sayon"),
        "lib = import(\"src/lib.sayon\")\nprint(lib.secret())\n",
    )
    .unwrap();
    let reaching = run_in(&dir.join("play"));
    assert!(
        reaching.status == 2 && reaching.stderr.contains("E_SANDBOX_IMPORT"),
        "{}",
        reaching.stderr
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/**
On a busy computer the program may not get its processor time before
its time in all runs out; either stops it.
*/
#[test]
fn an_endless_loop_is_stopped() {
    let outcome = run("loop", "count = 0\nwhile true\n    count += 1\n");
    stopped(
        &outcome,
        &[
            "it used 5 seconds of processor time",
            "it ran for 10 seconds",
        ],
    );
    assert!(outcome.took < Duration::from_secs(20), "{:?}", outcome.took);
}

#[test]
fn a_long_sleep_is_stopped_after_ten_seconds() {
    let outcome = run(
        "sleep",
        "time = import(\"standard/time.sayon\")\nprint(\"sleeping\")\ntime.sleep(time.seconds(60))\nprint(\"awake\")\n",
    );
    stopped(&outcome, &["it ran for 10 seconds"]);
    assert_eq!(outcome.stdout, "sleeping\n");
    assert!(outcome.took < Duration::from_secs(20), "{:?}", outcome.took);
}

#[test]
fn too_much_memory_stops_the_program() {
    let outcome = run(
        "memory",
        "strings = import(\"standard/strings.sayon\")\nitems: Array<string> = []\nwhile true\n    push(items, strings.repeat(\"x\", 4000000))\n",
    );
    stopped(&outcome, &["it used more than 256 MiB of memory"]);
}

#[test]
fn too_much_output_stops_the_program() {
    let outcome = run(
        "output",
        "io = import(\"standard/io.sayon\")\nwhile true\n    print(\"a line of output\")\n    io.write_error(\"and of errors\\n\")\n",
    );
    stopped(&outcome, &["it wrote more than 64 KiB of output"]);
    let written = outcome.stdout.len() + outcome.stderr.len();
    assert!(written > 60_000 && written < 70_000, "{written} bytes");
}
