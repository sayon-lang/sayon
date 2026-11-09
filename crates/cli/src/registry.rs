/*!
The registry commands, `sayon login`, `logout`, `publish`, `search`, and
`info`, and the registry client `sayon install` fetches packages with.
The protocol is described in the repository of Sayon's website, which
holds the registry, www.sayon-lang.org: a registry is named by its
index's address, and the index's `config.json` says where its API and
archives are. The driver reads what they hold
(`sayon_driver::registry`), and `http` carries the requests.
*/

use crate::compile::{Options, current_project, print_package_error};
use crate::credentials::{self, Saved};
use crate::http::{self, Response};
use crate::{status, usage_error};
use sayon_driver::json::Json;
use sayon_driver::manifest::Source;
use sayon_driver::packages::PackageError;
use sayon_driver::registry::{self, Config, Index, Published};
use sayon_driver::semver::Version;
use std::collections::HashMap;
use std::io::{BufRead, IsTerminal, Write};
use std::time::{Duration, Instant};

/** The registry a command uses: `--registry`, or the default. */
pub fn address(chosen: Option<&String>) -> Result<String, u8> {
    let address = match chosen {
        Some(address) => registry::normalize(address),
        None => registry::default_address(),
    };
    if !registry::valid_address(&address) {
        return Err(usage_error(&format!(
            "\"{address}\" is not a registry's address; it starts with `https://`, or `http://localhost` for one on this computer"
        )));
    }
    Ok(address)
}

fn error(message: &str, help: Option<&str>, options: &Options) {
    let error = PackageError {
        message: message.to_string(),
        help: help.map(str::to_string),
    };
    print_package_error(&error, options);
}

/** Percent-encodes a part of an address. */
fn encode(text: &str) -> String {
    let mut out = String::new();
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/** The registry's reason for refusing a request. */
fn refusal(response: &Response) -> String {
    std::str::from_utf8(&response.body)
        .ok()
        .and_then(|text| Json::parse(text).ok())
        .and_then(|answer| answer.get("error").as_str().map(registry::printable))
        .unwrap_or_else(|| format!("the registry answered with status {}", response.status))
}

/**
Refuses an address that is no registry's, wherever it came from,
`SAYON_REGISTRY` as well as a manifest or an index: Sayon reaches
registries over HTTPS, or over HTTP on this computer.
*/
fn reachable(registry: &str) -> Result<(), String> {
    if registry::valid_address(registry) {
        Ok(())
    } else {
        Err(format!(
            "\"{registry}\" is not a registry's address: Sayon reaches registries over HTTPS, or over HTTP on this computer"
        ))
    }
}

fn reach(registry: &str, error: String) -> String {
    let local = if registry == registry::DEFAULT {
        "; it is the registry run on this computer, which `sayon run -- --port 4100` in www.sayon-lang.org, Sayon's website, starts, unless SAYON_REGISTRY names another"
    } else {
        ""
    };
    format!("cannot reach the registry {registry}: {error}{local}")
}

/** Reads a registry's `config.json`, from the top of its index. */
fn fetch_config(registry: &str) -> Result<Config, String> {
    reachable(registry)?;
    let url = format!("{registry}/config.json");
    let response = http::request("GET", &url, &[("Accept", "application/json")], &[])
        .map_err(|e| reach(registry, e))?;
    if response.status != 200 {
        return Err(format!(
            "{registry} is not a registry's index: its config.json answered with status {}",
            response.status
        ));
    }
    std::str::from_utf8(&response.body)
        .ok()
        .and_then(|text| Json::parse(text).ok())
        .ok_or_else(|| format!("{registry}'s config.json is not JSON"))
        .and_then(|answer| registry::config(&answer))
}

/**
A package's file of a registry's index, or nothing when it has no such
package.
*/
fn fetch_entries(registry: &str, package: &str) -> Result<Option<Vec<Published>>, String> {
    reachable(registry)?;
    let url = format!("{registry}/{}", registry::index_path(package));
    let response = http::request("GET", &url, &[], &[]).map_err(|e| reach(registry, e))?;
    if response.status == 404 {
        return Ok(None);
    }
    if response.status != 200 {
        return Err(format!(
            "{registry} did not give the versions of `{package}`: {}",
            refusal(&response)
        ));
    }
    let file = std::str::from_utf8(&response.body)
        .map_err(|_| format!("{registry}'s index of `{package}` is not text"))?;
    registry::entries(file)
        .map(Some)
        .map_err(|message| format!("{registry}'s index of `{package}` cannot be read: {message}"))
}

/**
Asks a registry's API at `api`, reading a JSON answer. `kind` is the
type of a body sent.
*/
fn call(
    api: &str,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: &[u8],
    kind: &str,
) -> Result<(u16, Json), String> {
    let bearer = token.map(|token| format!("Bearer {token}"));
    let mut headers = vec![("Accept", "application/json")];
    if let Some(bearer) = &bearer {
        headers.push(("Authorization", bearer));
    }
    if !body.is_empty() {
        headers.push(("Content-Type", kind));
    }
    let url = format!("{api}/api/v{}{path}", registry::PROTOCOL);
    let response = http::request(method, &url, &headers, body).map_err(|e| reach(api, e))?;
    if !(200..300).contains(&response.status) {
        return Err(refusal(&response));
    }
    let answer = std::str::from_utf8(&response.body)
        .ok()
        .and_then(|text| Json::parse(text).ok())
        .ok_or_else(|| format!("{api} gave an answer that is not the registry protocol's"))?;
    Ok((response.status, answer))
}

/** Asks the registry's API, reading a JSON answer. */
fn ask(
    registry: &str,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: &[u8],
) -> Result<(u16, Json), String> {
    let api = fetch_config(registry)?.api;
    call(&api, method, path, token, body, "application/octet-stream")
}

/**
Reads a line from the terminal without showing it, or from whatever
standard input is.
*/
fn read_secret(prompt: &str) -> std::io::Result<String> {
    let terminal = std::io::stdin().is_terminal();
    if terminal {
        eprint!("{prompt}");
        let _ = std::io::stderr().flush();
    }
    let hidden = terminal && console::hide_input();
    let mut line = String::new();
    let read = std::io::stdin().lock().read_line(&mut line);
    if hidden {
        console::show_input();
        eprintln!();
    }
    read.map(|_| line.trim().to_string())
}

/**
`sayon login`: signs in on the registry's page, in the browser, where
the account's owner approves a new token for this computer; or with
`with_token`, takes a token from the input, as a build server does.
Either way it keeps the token, with the account's name.
*/
pub fn login(registry: &str, with_token: bool, open_browser: bool, options: &Options) -> u8 {
    let signed = if with_token {
        token_login(registry, options)
    } else {
        browser_login(registry, open_browser, options)
    };
    let (account, token) = match signed {
        Ok(signed) => signed,
        Err(code) => return code,
    };
    if let Err(message) = credentials::save(
        registry,
        &Saved {
            account: account.clone(),
            token,
        },
    ) {
        error(&message, None, options);
        return status::PROGRAM_FAILED;
    }
    if !options.quiet {
        eprintln!(
            "logged in to {registry} as {account}; the token is kept in {}",
            credentials::place()
        );
    }
    status::SUCCESS
}

/** A token from the input, and the account `GET /me` says it belongs to. */
fn token_login(registry: &str, options: &Options) -> Result<(String, String), u8> {
    let token = match read_secret(&format!("token for {registry}: ")) {
        Ok(token) if !token.is_empty() => token,
        Ok(_) => {
            error(
                "no token was given",
                Some("paste a token from your account's page at the registry, then press Enter"),
                options,
            );
            return Err(status::USAGE);
        }
        Err(e) => {
            error(&format!("cannot read the token: {e}"), None, options);
            return Err(status::INPUT);
        }
    };
    if token.chars().any(char::is_whitespace) {
        error("a token is one word, with no spaces", None, options);
        return Err(status::USAGE);
    }
    let account = match ask(registry, "GET", "/me", Some(&token), &[]) {
        Ok((_, answer)) => answer.get("name").as_str().unwrap_or_default().to_string(),
        Err(message) => {
            error(&message, None, options);
            return Err(status::PROGRAM_FAILED);
        }
    };
    /* The name is kept beside the token and shown, so it is a name. */
    if !sayon_driver::manifest::valid_name(&account) {
        error(
            &format!("{registry} did not say which account the token belongs to"),
            None,
            options,
        );
        return Err(status::PROGRAM_FAILED);
    }
    Ok((account, token))
}

/** Whether text is a sign-in's code: two fours of capital letters. */
fn valid_code(code: &str) -> bool {
    code.len() == 9
        && code.char_indices().all(|(position, letter)| {
            if position == 4 {
                letter == '-'
            } else {
                letter.is_ascii_uppercase()
            }
        })
}

/**
Signs in on the registry's page for a new sign-in, `<api>/login/<code>`,
opened in the browser unless `open_browser` is false, and waits until it
is approved there; then the sign-in's secret, which only this command
has, takes the account's new token. The page shows the code this command
prints, so that its user can tell the sign-in is theirs.
*/
fn browser_login(
    registry: &str,
    open_browser: bool,
    options: &Options,
) -> Result<(String, String), u8> {
    const JSON: &str = "application/json";
    let failed = |message: String| {
        error(&message, None, options);
        status::PROGRAM_FAILED
    };
    let api = fetch_config(registry).map_err(failed)?.api;
    let asked = Json::object(vec![("machine", Json::string(machine_name()))]).to_string();
    let (_, started) = call(&api, "POST", "/logins", None, asked.as_bytes(), JSON)
        .map_err(|message| failed(format!("cannot start a sign-in: {message}")))?;
    let secret = started
        .get("secret")
        .as_str()
        .unwrap_or_default()
        .to_string();
    let code = started.get("code").as_str().unwrap_or_default().to_string();
    let secret_fits = secret.len() == 64
        && secret
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !secret_fits || !valid_code(&code) {
        return Err(failed(format!(
            "{registry} started a sign-in that is not the registry protocol's"
        )));
    }
    let interval = started.get("interval").as_u32().unwrap_or(2).clamp(1, 10);
    let lasts = started
        .get("expires")
        .as_u32()
        .unwrap_or(600)
        .clamp(10, 3600);
    let page = format!("{api}/login/{code}");
    eprintln!("To sign in, open this page, check that it shows the code {code}, and approve it:");
    eprintln!();
    eprintln!("    {page}");
    eprintln!();
    if open_browser && browser::open(&page) {
        eprintln!("It is opening in your browser; waiting for your approval there.");
    } else {
        eprintln!("Waiting for your approval there.");
    }
    let asked = Json::object(vec![("secret", Json::string(secret))]).to_string();
    let deadline = Instant::now() + Duration::from_secs(u64::from(lasts));
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_secs(u64::from(interval)));
        let (_, answer) = call(&api, "POST", "/logins/poll", None, asked.as_bytes(), JSON)
            .map_err(|message| failed(format!("the sign-in did not finish: {message}")))?;
        match answer.get("status").as_str() {
            Some("waiting") => {}
            Some("approved") => {
                let account = answer.get("account").as_str().unwrap_or_default();
                let token = answer.get("token").as_str().unwrap_or_default();
                /* Both are kept, and the name is shown. */
                let token_fits = !token.is_empty()
                    && !token.chars().any(|c| c.is_whitespace() || c.is_control());
                if !sayon_driver::manifest::valid_name(account) || !token_fits {
                    return Err(failed(format!(
                        "{registry} approved the sign-in, but sent no account and token that Sayon can keep"
                    )));
                }
                return Ok((account.to_string(), token.to_string()));
            }
            Some("denied") => {
                error(
                    "the sign-in was denied on the registry's page",
                    Some("run `sayon login` again to sign in"),
                    options,
                );
                return Err(status::PROGRAM_FAILED);
            }
            _ => {
                return Err(failed(format!(
                    "{registry} told how the sign-in stands as the registry protocol does not"
                )));
            }
        }
    }
    error(
        "the sign-in expired before it was approved",
        Some("run `sayon login` again, and approve it within ten minutes"),
        options,
    );
    Err(status::PROGRAM_FAILED)
}

/**
This computer's name, which the page that approves a sign-in shows, and
the new token is named after.
*/
fn machine_name() -> String {
    #[cfg(windows)]
    let name = std::env::var("COMPUTERNAME").ok();
    #[cfg(unix)]
    let name = {
        unsafe extern "C" {
            fn gethostname(name: *mut std::ffi::c_char, length: usize) -> std::ffi::c_int;
        }
        let mut buffer = [0u8; 256];
        /* SAFETY: the buffer is as long as the length given. */
        let found = unsafe { gethostname(buffer.as_mut_ptr().cast(), buffer.len()) } == 0;
        let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
        found.then(|| String::from_utf8_lossy(&buffer[..end]).into_owned())
    };
    #[cfg(not(any(windows, unix)))]
    let name: Option<String> = None;
    name.filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "a computer".to_string())
}

/**
`sayon logout`: revokes the registry's token, so that it no longer
works anywhere, and forgets it. Without the registry, the token is
forgotten all the same, and its account's page can revoke it later.
*/
pub fn logout(registry: &str, options: &Options) -> u8 {
    let saved = match credentials::load(registry) {
        Ok(Some(saved)) => saved,
        Ok(None) => {
            if !options.quiet {
                eprintln!("not logged in to {registry}");
            }
            return status::SUCCESS;
        }
        Err(message) => {
            error(&message, None, options);
            return status::PROGRAM_FAILED;
        }
    };
    let revoked = ask(
        registry,
        "DELETE",
        "/tokens/current",
        Some(&saved.token),
        &[],
    );
    if let Err(message) = credentials::forget(registry) {
        error(&message, None, options);
        return status::PROGRAM_FAILED;
    }
    match revoked {
        Ok(_) => {
            if !options.quiet {
                eprintln!("logged out of {registry}; its token is revoked");
            }
        }
        Err(message) => eprintln!(
            "logged out of {registry} on this computer, but the token was not revoked: {message}; revoke it on your account's page"
        ),
    }
    status::SUCCESS
}

fn size(bytes: usize) -> String {
    match bytes {
        0..1024 => format!("{bytes} bytes"),
        1024..1_048_576 => format!("{:.1} KiB", bytes as f64 / 1024.0),
        _ => format!("{:.1} MiB", bytes as f64 / 1_048_576.0),
    }
}

/**
`sayon publish`: sends the project's package to the registry, or with
`dry_run` shows what it would send.
*/
pub fn publish(registry: &str, dry_run: bool, options: &Options) -> u8 {
    let project = match current_project(options) {
        Ok(Some(project)) => project,
        Ok(None) => {
            return usage_error(
                "`sayon publish` publishes a project; run it in a directory with sayon.toml",
            );
        }
        Err(code) => return code,
    };
    let manifest = &project.manifest;
    for dependency in &manifest.dependencies {
        if !matches!(dependency.source, Source::Registry { .. }) {
            error(
                &format!(
                    "the dependency `{}` is not from a registry, so the package's users could not get it",
                    dependency.alias
                ),
                Some(
                    "publish it, and depend on it by version, as `name = \"1.0\"`; development dependencies may be local",
                ),
                options,
            );
            return status::COMPILE_FAILED;
        }
    }
    let (archive, files) = match sayon_driver::archive::pack(&project.root) {
        Ok(packed) => packed,
        Err(message) => {
            error(&message, None, options);
            return status::COMPILE_FAILED;
        }
    };
    let digest = sayon_driver::archive::digest(&archive);
    let what = format!("{} {}", manifest.name, manifest.version);
    if dry_run {
        println!("{what}: {} files, {}", files.len(), size(archive.len()));
        for file in &files {
            println!("    {file}");
        }
        println!("digest {digest}");
        if !options.quiet {
            eprintln!("nothing was published, as `--dry-run` asked");
        }
        return status::SUCCESS;
    }
    let token = match std::env::var("SAYON_TOKEN") {
        Ok(token) if !token.trim().is_empty() => token.trim().to_string(),
        _ => match credentials::load(registry) {
            Ok(Some(saved)) => saved.token,
            Ok(None) => {
                error(
                    &format!("publishing needs an account at {registry}"),
                    Some("run `sayon login` first"),
                    options,
                );
                return status::USAGE;
            }
            Err(message) => {
                error(&message, None, options);
                return status::PROGRAM_FAILED;
            }
        },
    };
    let path = format!(
        "/packages/{}/{}",
        encode(&manifest.name),
        encode(&manifest.version)
    );
    match ask(registry, "PUT", &path, Some(&token), &archive) {
        Ok(_) => {
            if !options.quiet {
                eprintln!(
                    "published {what} to {registry} ({} files, {})",
                    files.len(),
                    size(archive.len())
                );
            }
            status::SUCCESS
        }
        Err(message) => {
            error(
                &format!("{what} was not published: {message}"),
                None,
                options,
            );
            status::PROGRAM_FAILED
        }
    }
}

/** `sayon search`: the packages whose names or descriptions hold `query`. */
pub fn search(registry: &str, query: &str, limit: u32, options: &Options) -> u8 {
    let path = format!("/search?q={}&limit={limit}", encode(query));
    let found = match ask(registry, "GET", &path, None, &[])
        .and_then(|(_, answer)| registry::search(&answer))
    {
        Ok(found) => found,
        Err(message) => {
            error(&message, None, options);
            return status::PROGRAM_FAILED;
        }
    };
    if found.is_empty() {
        if !options.quiet {
            eprintln!("no packages in {registry} match \"{query}\"");
        }
        return status::SUCCESS;
    }
    let width = found
        .iter()
        .map(|f| f.name.len() + f.latest.len() + 3)
        .max()
        .unwrap_or(0);
    for package in &found {
        let left = format!("{} = \"{}\"", package.name, package.latest);
        if package.description.is_empty() {
            println!("{left}");
        } else {
            println!("{left:width$}  # {}", package.description);
        }
    }
    status::SUCCESS
}

/** `sayon info`: what the registry's index holds of a package. */
pub fn info(registry: &str, name: &str, options: &Options) -> u8 {
    let help = Some("`sayon search` finds packages by part of their names");
    let versions = match fetch_entries(registry, name) {
        Ok(Some(versions)) if !versions.is_empty() => versions,
        Ok(_) => {
            error(
                &format!("there is no package `{name}` in {registry}"),
                help,
                options,
            );
            return status::PROGRAM_FAILED;
        }
        Err(message) => {
            error(&message, help, options);
            return status::PROGRAM_FAILED;
        }
    };
    let latest = registry::latest(&versions).expect("a version");
    println!("{name} {}", latest.version);
    if !latest.description.is_empty() {
        println!("{}", latest.description);
    }
    println!();
    println!("registry: {registry}");
    if latest.dependencies.is_empty() {
        println!("dependencies: none");
    } else {
        println!("dependencies:");
        for dependency in &latest.dependencies {
            if let Source::Registry {
                package,
                requirement,
                registry: other,
            } = &dependency.source
            {
                let from = other
                    .as_deref()
                    .map(|r| format!(" from {r}"))
                    .unwrap_or_default();
                let renamed = if *package == dependency.alias {
                    String::new()
                } else {
                    format!(" ({package})")
                };
                println!("    {}{renamed} \"{requirement}\"{from}", dependency.alias);
            }
        }
    }
    println!("versions:");
    for published in versions.iter().rev() {
        let day = published
            .published
            .get(..10)
            .unwrap_or(&published.published);
        println!(
            "    {:<12} {day}  {}",
            published.version.to_string(),
            published.digest
        );
    }
    println!();
    println!("in sayon.toml: {name} = \"{}\"", latest.version);
    status::SUCCESS
}

/**
The registries `sayon install` fetches packages from, with each one's
`config.json` once it has been read.
*/
#[derive(Default)]
pub struct Client {
    configs: HashMap<String, Config>,
}

impl Client {
    fn config(&mut self, registry: &str) -> Result<Config, PackageError> {
        if let Some(config) = self.configs.get(registry) {
            return Ok(config.clone());
        }
        let config = fetch_config(registry).map_err(PackageError::new)?;
        self.configs.insert(registry.to_string(), config.clone());
        Ok(config)
    }
}

impl Index for Client {
    fn versions(&mut self, registry: &str, package: &str) -> Result<Vec<Published>, PackageError> {
        match fetch_entries(registry, package).map_err(PackageError::new)? {
            Some(versions) if !versions.is_empty() => Ok(versions),
            _ => Err(
                PackageError::new(format!("there is no package `{package}` in {registry}"))
                    .help("check its name in sayon.toml; `sayon search` finds packages"),
            ),
        }
    }

    fn archive(
        &mut self,
        registry: &str,
        package: &str,
        version: &Version,
    ) -> Result<Vec<u8>, PackageError> {
        let url = self.config(registry)?.archive(&encode(package), version);
        let response = http::request("GET", &url, &[], &[])
            .map_err(|e| PackageError::new(reach(registry, e)))?;
        if response.status != 200 {
            return Err(PackageError::new(format!(
                "{registry} did not give {package} {version}: {}",
                refusal(&response)
            )));
        }
        Ok(response.body)
    }
}

/**
Opening a page in the browser the system is set to use: through the shell
on Windows, `open` on macOS, and `xdg-open` on Linux. Only addresses of
`http:` and `https:` are opened, so that nothing else is ever run.
*/
mod browser {
    fn openable(page: &str) -> bool {
        (page.starts_with("https://") || page.starts_with("http://"))
            && !page
                .chars()
                .any(|c| c.is_whitespace() || c.is_control() || c == '"')
    }

    /** Whether the page was handed to a browser. */
    #[cfg(windows)]
    pub fn open(page: &str) -> bool {
        use std::ffi::c_void;
        const SW_SHOWNORMAL: i32 = 1;
        #[link(name = "shell32")]
        unsafe extern "system" {
            fn ShellExecuteW(
                window: *mut c_void,
                operation: *const u16,
                file: *const u16,
                parameters: *const u16,
                directory: *const u16,
                show: i32,
            ) -> isize;
        }
        if !openable(page) {
            return false;
        }
        let wide = |text: &str| text.encode_utf16().chain([0]).collect::<Vec<u16>>();
        let (operation, file) = (wide("open"), wide(page));
        /* SAFETY: both texts end in NUL, and the rest may be null. */
        let done = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                operation.as_ptr(),
                file.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };
        /* Above 32 is success; the rest are errors. */
        done > 32
    }

    #[cfg(not(windows))]
    pub fn open(page: &str) -> bool {
        use std::process::{Command, Stdio};
        if !openable(page) {
            return false;
        }
        let opener = if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };
        Command::new(opener)
            .arg(page)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .is_ok()
    }
}

/** Turning the terminal's echo off while a token is typed. */
#[cfg(windows)]
mod console {
    use std::ffi::c_void;

    const STD_INPUT_HANDLE: u32 = -10i32 as u32;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetStdHandle(which: u32) -> *mut c_void;
        fn GetConsoleMode(console: *mut c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(console: *mut c_void, mode: u32) -> i32;
    }

    fn set_echo(on: bool) -> bool {
        unsafe {
            let input = GetStdHandle(STD_INPUT_HANDLE);
            let mut mode = 0;
            if GetConsoleMode(input, &mut mode) == 0 {
                return false;
            }
            let mode = if on {
                mode | ENABLE_ECHO_INPUT
            } else {
                mode & !ENABLE_ECHO_INPUT
            };
            SetConsoleMode(input, mode) != 0
        }
    }

    pub fn hide_input() -> bool {
        set_echo(false)
    }

    pub fn show_input() {
        set_echo(true);
    }
}

#[cfg(not(windows))]
mod console {
    use std::process::{Command, Stdio};

    fn stty(setting: &str) -> bool {
        Command::new("stty")
            .arg(setting)
            .stdin(Stdio::inherit())
            .status()
            .is_ok_and(|status| status.success())
    }

    pub fn hide_input() -> bool {
        stty("-echo")
    }

    pub fn show_input() {
        stty("echo");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_two_fours_of_capitals() {
        assert!(valid_code("WDJB-MJHT"));
        for wrong in [
            "wdjb-mjht",
            "WDJBMJHT",
            "WDJB-MJH",
            "WDJ-BMJHT",
            "WDJB_MJHT",
            "WDJB-MJÉ",
        ] {
            assert!(!valid_code(wrong), "{wrong}");
        }
    }

    #[test]
    fn parts_of_addresses_are_encoded() {
        assert_eq!(encode("json parser"), "json%20parser");
        assert_eq!(encode("a&b=c/é"), "a%26b%3Dc%2F%C3%A9");
        assert_eq!(encode("1.0.0-beta.1"), "1.0.0-beta.1");
    }
}
