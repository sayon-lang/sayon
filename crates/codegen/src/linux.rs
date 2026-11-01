//! Building for Linux of another processor. Clang compiles for the target's
//! triple and links with that processor's GNU C library, which the system's
//! cross packages install (on Debian and Ubuntu in `/usr/<GNU triple>`, with
//! the cross GCC's startup files and `libgcc`), and where clang finds it by
//! itself.

use std::path::{Path, PathBuf};
use std::process::Command;

use sayon_target::Target;

/// What a Linux target of another processor is called on Debian and Ubuntu.
pub struct Cross {
    /// Its processor, as people write it.
    pub processor: &'static str,
    /// The GNU triple that names the folder of its C library.
    pub gnu_triple: &'static str,
    /// The packages that install the C library.
    pub packages: &'static str,
    /// QEMU's user-mode emulator for its processor, from `qemu-user`.
    pub emulator: &'static str,
}

pub fn cross(target: &Target) -> Cross {
    if target.processor == "arm64" {
        Cross {
            processor: "ARM64",
            gnu_triple: "aarch64-linux-gnu",
            packages: "gcc-aarch64-linux-gnu libc6-dev-arm64-cross",
            emulator: "qemu-aarch64",
        }
    } else {
        Cross {
            processor: "x86-64",
            gnu_triple: "x86_64-linux-gnu",
            packages: "gcc-x86-64-linux-gnu libc6-dev-amd64-cross",
            emulator: "qemu-x86_64",
        }
    }
}

/// The startup file of the C library that clang links programs for
/// `target` with, found as clang finds it; or how to install the library.
pub fn c_library(clang: &Path, target: &Target) -> Result<PathBuf, String> {
    let output = Command::new(clang)
        .arg(format!("--target={}", target.triple))
        .arg("-print-file-name=crt1.o")
        .output()
        .map_err(|error| format!("could not start {}: {error}", clang.display()))?;
    // Clang prints the name alone when it finds no such file.
    let found = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
    if found.is_absolute() && found.is_file() {
        return Ok(found);
    }
    Err(format!(
        "the GNU C library for {target}, which its programs link with, was not found; \
         on Debian and Ubuntu, install it with `sudo apt install {}`",
        cross(target).packages
    ))
}
