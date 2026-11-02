//! The registry end to end: `sayon login`, on the registry's page and with
//! a token, `publish`, `search`, `info`, `logout`, and `install` of
//! versions, against the registry itself, part of Sayon's website, a Sayon
//! program, run by the `sayon` built here from its repository checked out
//! beside this one (`../www.sayon-lang.org`, or in `SAYON_REPOSITORIES`);
//! without it, those tests are skipped. They also send the registry
//! publications that a package cannot be, keep tokens out of what is
//! written and shown, and start the registry again on its data. A registry
//! that lies, a server of the test's own, checks what `sayon install`
//! refuses of one.

use sayon_driver::archive::{DOMAIN, LARGEST, digest};
use sayon_driver::json::Json;
use sayon_driver::registry::index_path;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct Registry {
    child: Child,
    /// The address of its index, which names the registry.
    url: String,
    tokens: Json,
}

impl Registry {
    /// Where its API listens: `127.0.0.1:<port>`.
    fn host(&self) -> String {
        self.url
            .trim_start_matches("http://")
            .trim_end_matches("/index")
            .to_string()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Starts a registry on its data folder in `base`, with these accounts made
/// or given new tokens.
fn registry(base: &Path, accounts: &[&str]) -> Option<Registry> {
    let repositories = std::env::var_os("SAYON_REPOSITORIES")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.."));
    let repository = repositories.join("www.sayon-lang.org");
    if !repository.join("src/main.sayon").is_file() {
        eprintln!(
            "SKIPPED: the registry's tests (no www.sayon-lang.org in {})",
            repositories.display()
        );
        return None;
    }
    let log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(base.join("registry.log"))
        .unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_sayon"));
    command
        .args(["run", "--", "--port", "0", "--data"])
        .arg(base.join("registry"));
    for account in accounts {
        command.args(["--account", account]);
    }
    let mut child = command
        .current_dir(&repository)
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(log)
        .spawn()
        .expect("sayon runs the registry");
    let mut line = String::new();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    stdout.read_line(&mut line).unwrap();
    let started = Json::parse(line.trim()).unwrap_or_else(|_| {
        let log = std::fs::read_to_string(base.join("registry.log")).unwrap_or_default();
        panic!("the registry did not start: {line}{log}")
    });
    // What it prints after its first line, of requests that failed, goes to
    // the log as well.
    let mut rest = std::fs::OpenOptions::new()
        .append(true)
        .open(base.join("registry.log"))
        .unwrap();
    std::thread::spawn(move || std::io::copy(&mut stdout, &mut rest));
    Some(Registry {
        url: started.get("index").as_str().unwrap().to_string(),
        tokens: started.get("accounts").clone(),
        child,
    })
}

struct Run {
    status: i32,
    stdout: String,
    stderr: String,
}

/// Runs `sayon` with a registry, a cache, and a file of credentials, and
/// keeps everything it showed.
struct Sayon {
    url: String,
    cache: PathBuf,
    credentials: PathBuf,
    shown: RefCell<String>,
}

impl Sayon {
    fn new(url: &str, cache: PathBuf, credentials: PathBuf) -> Sayon {
        Sayon {
            url: url.to_string(),
            cache,
            credentials,
            shown: RefCell::default(),
        }
    }

    fn run(&self, dir: &Path, args: &[&str], input: &str) -> Run {
        let mut child = Command::new(env!("CARGO_BIN_EXE_sayon"))
            .args(args)
            .current_dir(dir)
            .env("SAYON_REGISTRY", &self.url)
            .env("SAYON_CACHE", &self.cache)
            .env("SAYON_CREDENTIALS", &self.credentials)
            .env_remove("SAYON_TOKEN")
            .env("NO_COLOR", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("sayon runs");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        let run = Run {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
            stderr: String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"),
        };
        let mut shown = self.shown.borrow_mut();
        shown.push_str(&run.stdout);
        shown.push_str(&run.stderr);
        run
    }

    fn ok(&self, dir: &Path, args: &[&str]) -> Run {
        let run = self.run(dir, args, "");
        assert_eq!(run.status, 0, "sayon {}: {}", args.join(" "), run.stderr);
        run
    }

    fn fails(&self, dir: &Path, args: &[&str], message: &str) -> Run {
        let run = self.run(dir, args, "");
        assert_ne!(run.status, 0, "sayon {} succeeded", args.join(" "));
        assert!(
            run.stderr.contains(message),
            "sayon {}: {}",
            args.join(" "),
            run.stderr
        );
        run
    }

    /// Starts `sayon` without waiting for it, with what it shows on its
    /// standard error coming line by line.
    fn start(&self, dir: &Path, args: &[&str]) -> (Child, Receiver<String>) {
        let mut child = Command::new(env!("CARGO_BIN_EXE_sayon"))
            .args(args)
            .current_dir(dir)
            .env("SAYON_REGISTRY", &self.url)
            .env("SAYON_CACHE", &self.cache)
            .env("SAYON_CREDENTIALS", &self.credentials)
            .env_remove("SAYON_TOKEN")
            .env("NO_COLOR", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("sayon runs");
        let errors = child.stderr.take().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(errors).lines().map_while(Result::ok) {
                if sender.send(line).is_err() {
                    break;
                }
            }
        });
        (child, receiver)
    }
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// Every file and folder below `dir`.
fn under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut folders = vec![dir.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path.clone());
            }
            out.push(path);
        }
    }
    out
}

/// What `sayon install` keeps of what it fetched: every file in the cache,
/// and every folder below those of its registries.
fn kept(cache: &Path) -> Vec<PathBuf> {
    let depth = |path: &Path| path.strip_prefix(cache).unwrap().components().count();
    under(cache)
        .into_iter()
        .filter(|path| path.is_file() || depth(path) > 2)
        .collect()
}

/// Asserts that no file below `dir` holds the token, but the credentials
/// `sayon login` keeps it in.
fn kept_secret(dir: &Path, token: &str) {
    for file in under(dir) {
        let credentials = file
            .file_name()
            .is_some_and(|name| name.to_string_lossy().ends_with("credentials"));
        if file.is_file() && !credentials {
            let bytes = std::fs::read(&file).unwrap();
            assert!(
                !bytes
                    .windows(token.len())
                    .any(|part| part == token.as_bytes()),
                "{} holds a token",
                file.display()
            );
        }
    }
}

/// A file of an archive: its path, and its contents.
type File<'a> = (&'a str, &'a [u8]);

/// An archive of files, given as paths and contents, in the order given:
/// what a registry that lies could serve, or a publisher send.
fn archive_of(files: &[File]) -> Vec<u8> {
    let mut out = DOMAIN.to_vec();
    for (path, content) in files {
        out.extend((path.len() as u64).to_le_bytes());
        out.extend(path.as_bytes());
        out.extend((content.len() as u64).to_le_bytes());
        out.extend(*content);
    }
    out
}

/// Sends a request as no `sayon` would, or as a browser does, and reads the
/// status, the head, and the body of the answer. A `Content-Length` given
/// among the headers replaces the body's.
fn exchange(
    host: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: &[u8],
) -> (u16, String, String) {
    let mut stream = TcpStream::connect(host).unwrap();
    let mut head = format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n");
    for (name, value) in headers {
        head.push_str(&format!("{name}: {value}\r\n"));
    }
    if !headers.iter().any(|(name, _)| *name == "Content-Length") {
        head.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes()).unwrap();
    stream.write_all(body).unwrap();
    let mut answer = Vec::new();
    stream.read_to_end(&mut answer).unwrap();
    let answer = String::from_utf8_lossy(&answer);
    let status = answer.get(9..12).and_then(|code| code.parse().ok());
    let (head, body) = answer.split_once("\r\n\r\n").unwrap_or_default();
    (status.unwrap_or(0), head.to_string(), body.to_string())
}

/// The status and the body of the answer to a request [`exchange`] sends.
fn send(
    host: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: &[u8],
) -> (u16, String) {
    let (status, _, body) = exchange(host, method, path, headers, body);
    (status, body)
}

/// What a [`Liar`] answers each path with: a status, and a body.
type Answers = Arc<Mutex<HashMap<String, (u16, Vec<u8>)>>>;

/// A registry of the test's own, which answers each path with what it was
/// given, so that it can lie as no registry should.
struct Liar {
    address: String,
    answers: Answers,
    stop: Arc<AtomicBool>,
}

impl Liar {
    fn start() -> Liar {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let answers: Answers = Arc::default();
        let stop = Arc::new(AtomicBool::new(false));
        let (given, stopped) = (answers.clone(), stop.clone());
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                if stopped.load(Ordering::SeqCst) {
                    break;
                }
                let Ok(mut stream) = stream else { continue };
                let mut head = Vec::new();
                let mut byte = [0];
                while !head.ends_with(b"\r\n\r\n") && stream.read(&mut byte).is_ok_and(|n| n == 1) {
                    head.push(byte[0]);
                }
                let head = String::from_utf8_lossy(&head);
                let target = head.split(' ').nth(1).unwrap_or("/");
                let path = target.split('?').next().unwrap_or(target);
                let (status, body) = given.lock().unwrap().get(path).cloned().unwrap_or((
                    404,
                    br#"{"error":"there is nothing at this address"}"#.to_vec(),
                ));
                let head = format!(
                    "HTTP/1.1 {status} Answer\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                // `sayon` stops reading an answer longer than it reads.
                let _ = stream
                    .write_all(head.as_bytes())
                    .and_then(|()| stream.write_all(&body));
            }
        });
        let liar = Liar {
            address,
            answers,
            stop,
        };
        liar.config(&liar.address);
        liar
    }

    /// The address of its index, which names it.
    fn url(&self) -> String {
        format!("{}/index", self.address)
    }

    fn answer(&self, path: &str, status: u16, body: &[u8]) {
        self.answers
            .lock()
            .unwrap()
            .insert(path.to_string(), (status, body.to_vec()));
    }

    /// Its `config.json`, which says its API and archives are at `api`.
    fn config(&self, api: &str) {
        let config = format!(
            r#"{{"protocol":1,"api":"{api}","download":"{api}/archive/{{name}}/{{version}}"}}"#
        );
        self.answer("/index/config.json", 200, config.as_bytes());
    }

    /// Lists version 1.0.0 of a package with a digest and dependencies,
    /// written as JSON, and serves `archive` as its archive.
    fn list(&self, name: &str, archive: &[u8], digest: &str, dependencies: &str) {
        let line = format!(
            r#"{{"name":"{name}","version":"1.0.0","digest":"{digest}","description":"Says \u001b[31mhello\u001b[0m\r\nto you","dependencies":[{dependencies}],"published":"2026-11-02T00:00:00Z"}}"#
        );
        self.answer(
            &format!("/index/{}", index_path(name)),
            200,
            format!("{line}\n").as_bytes(),
        );
        self.answer(&format!("/archive/{name}/1.0.0"), 200, archive);
    }
}

impl Drop for Liar {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.address.trim_start_matches("http://"));
    }
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sr-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Writes the package `strings` at a version, its `shout` ending with `end`.
fn strings(dir: &Path, version: &str, end: &str) {
    write(
        &dir.join("sayon.toml"),
        &format!(
            "[package]\nname = \"strings\"\nversion = \"{version}\"\ndescription = \"Helpers for text\"\n"
        ),
    );
    write(
        &dir.join("src/shout.sayon"),
        &format!("function shout(text: string) -> string\n    return text + \"{end}\"\n"),
    );
}

#[test]
fn packages_are_published_found_and_installed() {
    let base = scratch("flow");
    let Some(registry) = registry(&base, &["alice", "bob"]) else {
        return;
    };
    let alice = Sayon::new(
        &registry.url,
        base.join("cache"),
        base.join("alice-credentials"),
    );
    let bob = Sayon::new(
        &registry.url,
        base.join("cache"),
        base.join("bob-credentials"),
    );
    let token = |name: &str| format!("{}\n", registry.tokens.get(name).as_str().unwrap());

    // Publishing needs an account, and a token the registry knows.
    let package = base.join("strings");
    strings(&package, "0.2.0", "!");
    alice.fails(&package, &["publish"], "run `sayon login`");
    let wrong = alice.run(&base, &["login", "--token"], "sayon_not_a_token\n");
    assert_ne!(wrong.status, 0);
    assert!(
        wrong.stderr.contains("not one this registry knows"),
        "{}",
        wrong.stderr
    );
    let login = alice.run(&base, &["login", "--token"], &token("alice"));
    assert_eq!(login.status, 0, "{}", login.stderr);
    assert!(login.stderr.contains("as alice"), "{}", login.stderr);

    let dry = alice.ok(&package, &["publish", "--dry-run"]);
    assert!(
        dry.stdout.starts_with("strings 0.2.0: 2 files"),
        "{}",
        dry.stdout
    );
    assert!(
        dry.stdout.contains("    src/shout.sayon\n"),
        "{}",
        dry.stdout
    );
    alice.fails(&base, &["info", "strings"], "there is no package `strings`");
    alice.ok(&package, &["publish"]);
    alice.fails(&package, &["publish"], "already published");
    strings(&package, "0.2.1", "!");
    alice.ok(&package, &["publish"]);
    strings(&package, "0.3.0", "?");
    alice.ok(&package, &["publish"]);

    // A name belongs to the account that published it first.
    let login = bob.run(&base, &["login", "--token"], &token("bob"));
    assert_eq!(login.status, 0, "{}", login.stderr);
    strings(&package, "0.4.0", "?");
    bob.fails(&package, &["publish"], "belongs to the account `alice`");

    // A published package depends only on registry packages.
    let words = base.join("words");
    write(
        &words.join("sayon.toml"),
        "[package]\nname = \"words\"\nversion = \"1.0.0\"\n\n[dependencies]\nstrings = { path = \"../strings\" }\n",
    );
    write(
        &words.join("src/words.sayon"),
        "strings = import(\"dependencies/strings/src/shout.sayon\")\n\nfunction loud(text: string) -> string\n    return strings.shout(strings.shout(text))\n",
    );
    alice.fails(&words, &["publish"], "is not from a registry");
    write(
        &words.join("sayon.toml"),
        "[package]\nname = \"words\"\nversion = \"1.0.0\"\ndescription = \"Words, louder\"\n\n[dependencies]\nstrings = \"0.2\"\n",
    );
    alice.ok(&words, &["publish"]);

    let search = alice.ok(&base, &["search", "text"]);
    assert_eq!(search.stdout, "strings = \"0.3.0\"  # Helpers for text\n");
    let none = alice.ok(&base, &["search", "nothing"]);
    assert!(none.stderr.contains("no packages"), "{}", none.stderr);
    let info = alice.ok(&base, &["info", "words"]);
    assert!(
        info.stdout.starts_with("words 1.0.0\nWords, louder\n"),
        "{}",
        info.stdout
    );
    assert!(
        info.stdout.contains("    strings \"0.2\"\n"),
        "{}",
        info.stdout
    );
    let info = alice.ok(&base, &["info", "strings"]);
    let listed: Vec<&str> = info
        .stdout
        .lines()
        .skip_while(|line| *line != "versions:")
        .skip(1)
        .take(3)
        .map(|line| line.split_whitespace().next().unwrap())
        .collect();
    assert_eq!(listed, ["0.3.0", "0.2.1", "0.2.0"]);

    // Installing chooses one version of each package that every range
    // accepts, and keeps it while they still do.
    let app = base.join("app");
    let manifest = "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nwords = \"1\"\nstrings = \"0.2\"\n";
    write(&app.join("sayon.toml"), manifest);
    write(
        &app.join("src/main.sayon"),
        "words = import(\"dependencies/words/src/words.sayon\")\nstrings = import(\"dependencies/strings/src/shout.sayon\")\n\nprint(words.loud(\"hey\"), strings.shout(\"you\"))\n",
    );
    alice.fails(&app, &["install", "--offline"], "installing is offline");
    let installed = alice.ok(&app, &["install"]);
    assert!(
        installed.stderr.contains("2 packages; wrote sayon.lock"),
        "{}",
        installed.stderr
    );
    let lock = std::fs::read_to_string(app.join("sayon.lock")).unwrap();
    let url = &registry.url;
    assert!(
        lock.contains(&format!(
            "dependencies = [\"strings = registry:{url}#strings@0.2.1\", \"words = registry:{url}#words@1.0.0\"]"
        )),
        "{lock}"
    );
    assert!(lock.contains("digest = \"sha256:"), "{lock}");
    assert_eq!(alice.ok(&app, &["run"]).stdout, "hey!! you!\n");

    strings(&package, "0.2.2", "!");
    alice.ok(&package, &["publish"]);
    let again = alice.ok(&app, &["install"]);
    assert!(again.stderr.contains("up to date"), "{}", again.stderr);
    std::fs::remove_file(app.join("sayon.lock")).unwrap();
    alice.ok(&app, &["install"]);
    let lock = std::fs::read_to_string(app.join("sayon.lock")).unwrap();
    assert!(lock.contains("#strings@0.2.2\""), "{lock}");
    alice.ok(&app, &["install", "--offline", "--locked"]);

    // A cached copy that changed is refused, and installing fetches it again.
    let cached = std::fs::read_dir(base.join("cache/registry"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
        .join("strings-0.2.2/src/shout.sayon");
    let original = std::fs::read_to_string(&cached).unwrap();
    std::fs::write(&cached, format!("{original}# changed\n")).unwrap();
    alice.fails(&app, &["run"], "does not match sayon.lock");
    alice.ok(&app, &["install"]);
    assert_eq!(alice.ok(&app, &["run"]).stdout, "hey!! you!\n");

    // Ranges no one version matches are an error that names them.
    write(
        &app.join("sayon.toml"),
        &manifest.replace("strings = \"0.2\"", "strings = \"0.3\""),
    );
    alice.fails(&app, &["run"], "does not accept");
    alice.fails(
        &app,
        &["install"],
        "matches every range asked for: \"0.3\" (this project), \"0.2\" (words 1.0.0)",
    );
    write(
        &app.join("sayon.toml"),
        &manifest.replace("words = \"1\"", "words = \"2\""),
    );
    alice.fails(&app, &["install"], "no version of `words`");
    write(
        &app.join("sayon.toml"),
        &manifest.replace("words = \"1\"", "wordz = \"1\""),
    );
    alice.fails(&app, &["install"], "there is no package `wordz`");

    // `sayon logout` revokes the token on the registry as well.
    let out = alice.ok(&base, &["logout"]);
    assert!(
        out.stderr.contains("its token is revoked"),
        "{}",
        out.stderr
    );
    alice.fails(&package, &["publish"], "run `sayon login`");
    let revoked = format!("Bearer {}", registry.tokens.get("alice").as_str().unwrap());
    let me = send(
        &registry.host(),
        "GET",
        "/api/v1/me",
        &[("Authorization", &revoked)],
        b"",
    );
    assert_eq!(me.0, 401, "{}", me.1);

    // A token is shown once, when it is made, and kept in the credentials:
    // in nothing `sayon` showed, and in no file of the projects, the cache,
    // or the registry, which keeps the SHA-256 of those not revoked.
    let tokens = std::fs::read_to_string(base.join("registry/tokens.json")).unwrap();
    for name in ["alice", "bob"] {
        let token = registry.tokens.get(name).as_str().unwrap();
        let hash = digest(token.as_bytes());
        let listed = tokens.contains(&hash["sha256:".len()..]);
        assert_eq!(listed, name == "bob", "{tokens}");
        for sayon in [&alice, &bob] {
            assert!(!sayon.shown.borrow().contains(token));
        }
        kept_secret(&base, token);
    }
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn a_registry_that_lies_is_refused() {
    let base = scratch("liar");
    let liar = Liar::start();
    let sayon = Sayon::new(&liar.url(), base.join("cache"), base.join("credentials"));
    let app = base.join("app");
    let toml = |name: &str| format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\n");
    let hello: &[u8] = b"function hello() -> string\n    return \"hello\"\n";
    let package = |name: &str| {
        archive_of(&[
            ("sayon.toml", toml(name).as_bytes()),
            ("src/hello.sayon", hello),
        ])
    };
    // Whatever is refused, installing writes no sayon.lock, keeps nothing,
    // and shows no control characters a registry sent.
    let refused = |sayon: &Sayon, name: &str, message: &str| {
        write(
            &app.join("sayon.toml"),
            &format!(
                "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\n{name} = \"1\"\n"
            ),
        );
        let run = sayon.fails(&app, &["install"], message);
        assert!(!run.stderr.contains('\u{1b}'), "{}", run.stderr);
        assert!(!app.join("sayon.lock").exists());
        let kept = kept(&base.join("cache"));
        assert!(kept.is_empty(), "{kept:?}");
    };

    // Registries are reached over HTTPS, or over HTTP on this computer.
    let elsewhere = Sayon::new(
        "http://registry.example.com/index",
        base.join("cache"),
        base.join("credentials"),
    );
    refused(
        &elsewhere,
        "hello",
        "\"http://registry.example.com/index\" is not a registry's address",
    );
    liar.list("hello", &package("hello"), &digest(&package("hello")), "");
    liar.config("http://registry.example.com");
    refused(
        &sayon,
        "hello",
        "config.json names addresses Sayon does not use",
    );
    liar.config(&liar.address);

    // An archive is the one the index lists, whole.
    let listed = package("changed");
    let mut given = listed.clone();
    *given.last_mut().unwrap() ^= 1;
    liar.list("changed", &given, &digest(&listed), "");
    let changed = format!("has the digest {}, not {}", digest(&given), digest(&listed));
    refused(&sayon, "changed", &changed);
    let mut cut = package("cut");
    cut.pop();
    liar.list("cut", &cut, &digest(&cut), "");
    refused(
        &sayon,
        "cut",
        "the archive is damaged or not a Sayon package",
    );
    let content = vec![b'#'; LARGEST];
    let large = archive_of(&[
        ("sayon.toml", toml("large").as_bytes()),
        ("src/large.sayon", &content),
    ]);
    liar.list("large", &large, &digest(&large), "");
    refused(
        &sayon,
        "large",
        "the archive is larger than a package can be",
    );
    liar.list(
        "endless",
        &vec![0; 33 * 1024 * 1024],
        &digest(b"endless"),
        "",
    );
    refused(&sayon, "endless", "is longer than Sayon reads");

    // Whatever its digest, an archive holds only what a package can: no
    // path that leaves its folder or is left out of packages, each once,
    // in order, and none differing from another only in case.
    let manifest = toml("unpacked");
    let manifest = manifest.as_bytes();
    let archives: [(&[File], &str); 10] = [
        (
            &[("../outside.sayon", hello), ("sayon.toml", manifest)],
            "`../outside.sayon`, which a package cannot",
        ),
        (
            &[
                ("sayon.toml", manifest),
                ("src/../../../../../outside.sayon", hello),
            ],
            "which a package cannot",
        ),
        (
            &[("/outside.sayon", hello), ("sayon.toml", manifest)],
            "`/outside.sayon`, which a package cannot",
        ),
        (
            &[("C:/outside.sayon", hello), ("sayon.toml", manifest)],
            "`C:/outside.sayon`, which a package cannot",
        ),
        (
            &[
                ("sayon.toml", manifest),
                ("src\\..\\..\\outside.sayon", hello),
            ],
            "which a package cannot",
        ),
        (
            &[
                (".git/hooks/post-checkout", hello),
                ("sayon.toml", manifest),
            ],
            "`.git/hooks/post-checkout`, which a package cannot",
        ),
        (
            &[("sayon.lock", hello), ("sayon.toml", manifest)],
            "`sayon.lock`, which a package cannot",
        ),
        (
            &[("sayon.toml", manifest), ("sayon.toml", manifest)],
            "`sayon.toml` out of order or twice",
        ),
        (
            &[
                ("sayon.toml", manifest),
                ("src/b.sayon", hello),
                ("src/a.sayon", hello),
            ],
            "`src/a.sayon` out of order or twice",
        ),
        (
            &[
                ("SRC/hello.sayon", hello),
                ("sayon.toml", manifest),
                ("src/hello.sayon", hello),
            ],
            "`src/hello.sayon` and another path that differs only in case",
        ),
    ];
    for (files, message) in archives {
        let archive = archive_of(files);
        liar.list("unpacked", &archive, &digest(&archive), "");
        refused(&sayon, "unpacked", message);
    }
    assert!(!base.join("outside.sayon").exists());

    // An index lists dependencies as manifests write them: names that
    // become folders of the cache and lines of sayon.lock, ranges, and
    // registries reached over HTTPS or on this computer.
    let lister = package("lister");
    let dependencies = [
        (
            r#"{"alias":"evil","package":"../../../../evil","requirement":"1","registry":null}"#,
            "named \"../../../../evil\", which is not a package name",
        ),
        (
            r##"{"alias":"x = registry:https://elsewhere.example/index#x@1.0.0\", \"y","package":"strings","requirement":"1","registry":null}"##,
            "which is not a package name",
        ),
        (
            r#"{"alias":"text","package":"strings","requirement":"the newest","registry":null}"#,
            "\"the newest\", which is not a range of versions",
        ),
        (
            r#"{"alias":"text","package":"strings","requirement":"1","registry":"http://registry.example.com/index"}"#,
            "\"http://registry.example.com/index\", which is not a registry's address",
        ),
    ];
    for (dependency, message) in dependencies {
        liar.list("lister", &lister, &digest(&lister), dependency);
        refused(&sayon, "lister", message);
    }
    let gone = package("gone");
    liar.list("gone", &gone, &digest(&gone), "");
    liar.answer(
        "/archive/gone/1.0.0",
        451,
        br#"{"error":"\u001b[2Jgone for good"}"#,
    );
    refused(&sayon, "gone", "did not give gone 1.0.0: [2Jgone for good");

    // An account's name is kept beside its token, so it is a name.
    liar.answer(
        "/api/v1/me",
        200,
        br#"{"name":"mallory\nhttp://127.0.0.1:4100/index mallory planted"}"#,
    );
    let login = sayon.run(&base, &["login", "--token"], "sayon_not_a_token\n");
    assert_ne!(login.status, 0);
    assert!(
        login
            .stderr
            .contains("did not say which account the token belongs to"),
        "{}",
        login.stderr
    );
    assert!(!base.join("credentials").exists());

    // What it lists truly is installed, and what it says is shown without
    // control characters.
    write(
        &app.join("src/main.sayon"),
        "hello = import(\"dependencies/hello/src/hello.sayon\")\n\nprint(hello.hello())\n",
    );
    write(
        &app.join("sayon.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nhello = \"1\"\n",
    );
    sayon.ok(&app, &["install"]);
    assert_eq!(sayon.ok(&app, &["run"]).stdout, "hello\n");
    let info = sayon.ok(&base, &["info", "hello"]);
    assert!(
        info.stdout
            .starts_with("hello 1.0.0\nSays [31mhello[0mto you\n"),
        "{}",
        info.stdout
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn publications_a_package_cannot_be_are_refused() {
    let base = scratch("refused");
    let Some(registry) = registry(&base, &["alice"]) else {
        return;
    };
    let host = registry.host();
    let bearer = format!("Bearer {}", registry.tokens.get("alice").as_str().unwrap());
    let alice = [("Authorization", bearer.as_str())];
    let put = |path: &str, headers: &[(&str, &str)], body: &[u8]| {
        send(
            &host,
            "PUT",
            &format!("/api/v1/packages/{path}"),
            headers,
            body,
        )
    };
    // An answer's status, and its error or, for a success, its body.
    let answers = |(got, body): (u16, String), status: u16, text: &str| {
        let shown = Json::parse(&body)
            .ok()
            .and_then(|answer| answer.get("error").as_str().map(str::to_string))
            .unwrap_or(body);
        assert_eq!(got, status, "{shown}");
        assert!(shown.contains(text), "{shown}");
    };
    let toml = |name: &str, version: &str| {
        format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\n")
    };
    let manifest = toml("strings", "1.0.0");
    let manifest = manifest.as_bytes();
    let source: &[u8] = b"function shout(text: string) -> string\n    return text + \"!\"\n";
    let good = archive_of(&[("sayon.toml", manifest), ("src/shout.sayon", source)]);

    // Publishing needs an account's token, and a name and a version.
    answers(
        put("strings/1.0.0", &[], &good),
        401,
        "run `sayon login` first",
    );
    let stranger = [("Authorization", "Bearer sayon_not_a_token")];
    answers(
        put("strings/1.0.0", &stranger, &good),
        401,
        "not one this registry knows",
    );
    answers(
        put("Strings/1.0.0", &alice, &good),
        400,
        "`Strings` is not a package name",
    );
    answers(
        put("..%2F..%2Fstrings/1.0.0", &alice, &good),
        400,
        "is not a package name",
    );
    answers(
        put("strings/latest", &alice, &good),
        400,
        "`latest` is not a version",
    );

    // An archive holds only what a package can, and its sayon.toml names
    // the version, with dependencies `sayon` can install.
    let other = toml("strings", "2.0.0");
    let words = toml("words", "1.0.0");
    let archives: [(&[File], &str); 8] = [
        (
            &[("../outside.sayon", source), ("sayon.toml", manifest)],
            "the archive holds `../outside.sayon`, which a package cannot",
        ),
        (
            &[(".hidden", source), ("sayon.toml", manifest)],
            "the archive holds `.hidden`, which a package cannot",
        ),
        (
            &[("sayon.toml", manifest), ("sayon.toml", manifest)],
            "the archive holds `sayon.toml` out of order or twice",
        ),
        (
            &[
                ("sayon.toml", manifest),
                ("src/b.sayon", source),
                ("src/a.sayon", source),
            ],
            "the archive holds `src/a.sayon` out of order or twice",
        ),
        (
            &[
                ("SRC/shout.sayon", source),
                ("sayon.toml", manifest),
                ("src/shout.sayon", source),
            ],
            "the archive holds `src/shout.sayon` and another path that differs only in case",
        ),
        (
            &[("src/shout.sayon", source)],
            "the archive has no sayon.toml",
        ),
        (
            &[("sayon.toml", other.as_bytes())],
            "the archive's sayon.toml is strings 2.0.0, not strings 1.0.0",
        ),
        (
            &[("sayon.toml", words.as_bytes())],
            "the archive's sayon.toml is words 1.0.0, not strings 1.0.0",
        ),
    ];
    for (files, message) in archives {
        answers(
            put("strings/1.0.0", &alice, &archive_of(files)),
            400,
            message,
        );
    }
    let mut cut = good.clone();
    cut.pop();
    answers(
        put("strings/1.0.0", &alice, &cut),
        400,
        "the archive is damaged or not a Sayon package",
    );
    let dependencies = [
        (
            "evil = { version = \"1\", package = \"../../evil\" }",
            "a dependency named \"../../evil\", which is not a package name",
        ),
        (
            "text = \"the newest\"",
            "asks for \"the newest\", which is not a range of versions",
        ),
        (
            "text = { version = \"1\", package = \"strings\", registry = \"http://registry.example.com/index\" }",
            "comes from \"http://registry.example.com/index\", which is not a registry's address",
        ),
    ];
    for (dependency, message) in dependencies {
        let manifest = format!("{words}\n[dependencies]\n{dependency}\n");
        let archive = archive_of(&[("sayon.toml", manifest.as_bytes())]);
        answers(put("words/1.0.0", &alice, &archive), 400, message);
    }

    // A package holds 10 MiB at most, whether the request says it is
    // larger or is.
    let announced = [alice[0], ("Content-Length", "10486785")];
    answers(
        put("strings/1.0.0", &announced, b""),
        413,
        "a package holds 10 MiB at most",
    );
    let content = vec![b'#'; LARGEST];
    let large = archive_of(&[("sayon.toml", manifest), ("src/large.sayon", &content)]);
    answers(
        put("strings/1.0.0", &alice, &large),
        413,
        "a package holds 10 MiB at most",
    );

    // None of it is listed or kept, and the version can still be published.
    assert_eq!(send(&host, "GET", "/index/st/ri/strings", &[], b"").0, 404);
    let data = base.join("registry");
    let kept: Vec<PathBuf> = under(&data)
        .into_iter()
        .filter(|path| {
            path.is_file() && !path.ends_with("accounts.json") && !path.ends_with("tokens.json")
        })
        .collect();
    assert!(kept.is_empty(), "{kept:?}");
    answers(put("strings/1.0.0", &alice, &good), 201, &digest(&good));
    let (status, listed) = send(&host, "GET", "/index/st/ri/strings", &[], b"");
    assert_eq!(status, 200);
    assert!(listed.contains(&digest(&good)), "{listed}");
    answers(
        put("strings/1.0.0", &alice, &good),
        409,
        "already published",
    );
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn the_registry_keeps_its_data_and_finishes_what_a_crash_left() {
    let base = scratch("restart");
    let Some(first) = registry(&base, &["alice"]) else {
        return;
    };
    let token = first.tokens.get("alice").as_str().unwrap().to_string();
    let alice = Sayon::new(&first.url, base.join("cache"), base.join("credentials"));
    let login = alice.run(&base, &["login", "--token"], &format!("{token}\n"));
    assert_eq!(login.status, 0, "{}", login.stderr);
    let package = base.join("strings");
    strings(&package, "0.2.0", "!");
    alice.ok(&package, &["publish"]);
    drop(first);

    // A crash part way through publishing 0.3.0 leaves an archive that is
    // not listed, and temporary files half written beside the archive, the
    // package's file of the index, and the accounts.
    let data = base.join("registry");
    write(
        &data.join("archives/strings/0.3.0.archive"),
        "half an archive",
    );
    write(
        &data.join("archives/strings/0.3.0.archive.tmp"),
        "half an archive",
    );
    let file = data.join("index").join(index_path("strings"));
    let listed = std::fs::read_to_string(&file).unwrap();
    let half = &listed[..listed.len() / 2];
    write(Path::new(&format!("{}.tmp", file.display())), half);
    write(&data.join("accounts.json.tmp"), "{\"alice\": ");
    write(&data.join("tokens.json.tmp"), "{\"");

    // Started again on its data, without making accounts, the registry
    // knows the token and lists what it listed; publishing 0.3.0 again
    // finishes it, with its own archive.
    let Some(second) = registry(&base, &[]) else {
        return;
    };
    let alice = Sayon::new(&second.url, base.join("cache"), base.join("credentials"));
    let login = alice.run(&base, &["login", "--token"], &format!("{token}\n"));
    assert_eq!(login.status, 0, "{}", login.stderr);
    let info = alice.ok(&base, &["info", "strings"]);
    assert!(
        info.stdout.starts_with("strings 0.2.0\n"),
        "{}",
        info.stdout
    );
    strings(&package, "0.3.0", "?");
    alice.ok(&package, &["publish"]);
    let app = base.join("app");
    write(
        &app.join("sayon.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nstrings = \"0.3\"\n",
    );
    write(
        &app.join("src/main.sayon"),
        "strings = import(\"dependencies/strings/src/shout.sayon\")\n\nprint(strings.shout(\"hey\"))\n",
    );
    alice.ok(&app, &["install"]);
    assert_eq!(alice.ok(&app, &["run"]).stdout, "hey?\n");
    let info = alice.ok(&base, &["info", "strings"]);
    let versions: Vec<&str> = info
        .stdout
        .lines()
        .skip_while(|line| *line != "versions:")
        .skip(1)
        .take(2)
        .map(|line| line.split_whitespace().next().unwrap())
        .collect();
    assert_eq!(versions, ["0.3.0", "0.2.0"]);
    assert!(!alice.shown.borrow().contains(&token));
    kept_secret(&base, &token);
    drop(second);
    let _ = std::fs::remove_dir_all(&base);
}

/// The path of the link holding `part`, such as /verify/, in the mail to
/// `to` that the registry kept in its data folder, having no mail server.
fn mailed_link(base: &Path, to: &str, part: &str) -> String {
    for file in under(&base.join("registry/mail")) {
        let text = std::fs::read_to_string(&file).unwrap_or_default();
        if !text.contains(&format!("To: <{to}>")) {
            continue;
        }
        let found = text
            .lines()
            .find(|line| line.starts_with("http") && line.contains(part));
        if let Some(line) = found {
            let rest = line.split_once("://").map_or(line, |(_, rest)| rest);
            let start = rest.find('/').expect("a link has a path");
            return rest[start..].trim().to_string();
        }
    }
    panic!("no mail to {to} holds a link with {part}");
}

/// The value of the cookie an answer's head sets, as a request sends it
/// back, or "" without one.
fn cookie_of(head: &str) -> String {
    head.lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("set-cookie").then(|| {
                value
                    .split(';')
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .to_string()
            })
        })
        .unwrap_or_default()
}

/// The page `sayon login` shows for its sign-in, once it shows it.
fn sign_in_page(lines: &Receiver<String>, seen: &mut String) -> String {
    loop {
        let line = lines
            .recv_timeout(Duration::from_secs(120))
            .unwrap_or_else(|_| panic!("sayon login showed no page: {seen}"));
        seen.push_str(&line);
        seen.push('\n');
        if let Some(start) = line.find("http://")
            && line.contains("/login/")
        {
            return line[start..].trim().to_string();
        }
    }
}

/// Waits for a `sayon` that was started, and gives its exit status and the
/// rest of what it showed.
fn finish(mut child: Child, lines: Receiver<String>, seen: &mut String) -> i32 {
    let status = child.wait().unwrap().code().unwrap_or(-1);
    for line in lines.iter() {
        seen.push_str(&line);
        seen.push('\n');
    }
    status
}

#[test]
fn signing_in_on_the_registrys_page_gives_this_computer_a_token() {
    let base = scratch("browser");
    let Some(registry) = registry(&base, &[]) else {
        return;
    };
    let host = registry.host();
    let origin = format!("http://{host}");
    let carol = Sayon::new(&registry.url, base.join("cache"), base.join("credentials"));

    // An account made on the registry's page: the browser is signed in.
    let form = [
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Origin", origin.as_str()),
    ];
    let fields = b"name=carol&email=carol%40example.com&password=a+long+walk+home+at+dusk&again=a+long+walk+home+at+dusk";
    let (status, head, body) = exchange(&host, "POST", "/sign-up", &form, fields);
    assert_eq!(status, 303, "{head}{body}");
    let cookie = cookie_of(&head);
    assert!(cookie.starts_with("sayon_session="), "{head}");
    let signed = [
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Origin", origin.as_str()),
        ("Cookie", cookie.as_str()),
    ];

    // `sayon login` shows the page of its sign-in, with its code, and waits
    // until the sign-in is approved there.
    let mut seen = String::new();
    let (child, lines) = carol.start(&base, &["login", "--no-browser"]);
    let page = sign_in_page(&lines, &mut seen);
    let path = page.trim_start_matches(&origin).to_string();
    let code = path.trim_start_matches("/login/");
    assert!(seen.contains(&format!("the code {code}")), "{seen}");
    let (status, approval) = send(&host, "GET", &path, &signed[2..], b"");
    assert_eq!(status, 200, "{approval}");
    assert!(approval.contains("Approve this sign-in?"), "{approval}");
    // A form from another site approves nothing.
    let elsewhere = [signed[0], ("Origin", "https://example.com"), signed[2]];
    assert_eq!(
        send(&host, "POST", &path, &elsewhere, b"decision=approve").0,
        403
    );
    assert_eq!(
        send(&host, "POST", &path, &signed, b"decision=approve").0,
        303
    );
    assert_eq!(finish(child, lines, &mut seen), 0, "{seen}");
    assert!(seen.contains("as carol"), "{seen}");

    // The token it keeps is the account's, and is shown nowhere.
    let kept = std::fs::read_to_string(base.join("credentials")).unwrap();
    let token = kept
        .split_whitespace()
        .find(|word| word.starts_with("sayon_"))
        .expect("the credentials hold a token")
        .to_string();
    let bearer = format!("Bearer {token}");
    let (status, me) = send(
        &host,
        "GET",
        "/api/v1/me",
        &[("Authorization", &bearer)],
        b"",
    );
    assert_eq!((status, me.as_str()), (200, "{\"name\":\"carol\"}"));
    assert!(!seen.contains(&token) && !carol.shown.borrow().contains(&token));
    // It publishes once the account's email is verified, by the link the
    // registry mailed, which it keeps in its data folder without a mail
    // server.
    let package = base.join("strings");
    strings(&package, "0.2.0", "!");
    carol.fails(&package, &["publish"], "has not verified its email");
    let link = mailed_link(&base, "carol@example.com", "/verify/");
    assert_eq!(send(&host, "GET", &link, &[], b"").0, 200);
    carol.ok(&package, &["publish"]);
    kept_secret(&base, &token);

    // A sign-in denied on its page gives no token.
    let mut seen = String::new();
    let (child, lines) = carol.start(&base, &["login", "--no-browser"]);
    let page = sign_in_page(&lines, &mut seen);
    let path = page.trim_start_matches(&origin).to_string();
    assert_eq!(send(&host, "POST", &path, &signed, b"decision=deny").0, 303);
    assert_ne!(finish(child, lines, &mut seen), 0, "{seen}");
    assert!(seen.contains("denied"), "{seen}");

    // `sayon logout` revokes the token.
    let out = carol.ok(&base, &["logout"]);
    assert!(
        out.stderr.contains("its token is revoked"),
        "{}",
        out.stderr
    );
    let (status, _) = send(
        &host,
        "GET",
        "/api/v1/me",
        &[("Authorization", &bearer)],
        b"",
    );
    assert_eq!(status, 401);
    let _ = std::fs::remove_dir_all(&base);
}
