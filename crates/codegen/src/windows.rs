//! Building for Windows of another processor, such as Windows ARM64's
//! programs on Windows x64. Clang compiles for the target's triple, and lld
//! links with Visual Studio's C runtime and the Windows SDK's libraries for
//! the target's processor, which clang finds where it finds this computer's.
//! The SDK holds every processor's libraries; Visual Studio installs its C
//! runtime for each processor with a component of its own.

use std::path::PathBuf;
use std::process::Command;

use sayon_target::Target;

/// What Visual Studio calls the libraries of a processor.
pub struct Libraries {
    /// The processor, as people write it.
    pub processor: &'static str,
    /// The folder of the libraries in Visual Studio's C++ tools.
    pub folder: &'static str,
    /// The Visual Studio component that installs them, and its name in the
    /// Visual Studio Installer.
    pub component: &'static str,
    pub component_name: &'static str,
}

pub fn libraries(target: &Target) -> Libraries {
    if target.processor == "arm64" {
        Libraries {
            processor: "ARM64",
            folder: "arm64",
            component: "Microsoft.VisualStudio.Component.VC.Tools.ARM64",
            component_name: "MSVC ARM64/ARM64EC build tools",
        }
    } else {
        Libraries {
            processor: "x64",
            folder: "x64",
            component: "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            component_name: "MSVC x64/x86 build tools",
        }
    }
}

/// Visual Studio's C runtime library for `target` (`msvcrt.lib`), where
/// clang finds Visual Studio's tools: in `VCToolsInstallDir`, or the default
/// tools of the newest installation with C++ tools; or how to install it.
pub fn c_runtime(target: &Target) -> Result<PathBuf, String> {
    let names = libraries(target);
    let tools = match std::env::var_os("VCToolsInstallDir") {
        Some(dir) => Some(PathBuf::from(dir)),
        None => default_tools(),
    };
    match tools.map(|dir| dir.join("lib").join(names.folder).join("msvcrt.lib")) {
        Some(library) if library.is_file() => Ok(library),
        _ => Err(format!(
            "Visual Studio's C runtime for {target}, which its programs link with, was not \
             found; the Visual Studio Installer adds it with the component \"{}\" ({})",
            names.component_name, names.component
        )),
    }
}

/// The default C++ tools of the newest Visual Studio installation that has
/// them, as `vswhere` and the version file Visual Studio keeps name them.
fn default_tools() -> Option<PathBuf> {
    let vswhere = PathBuf::from(std::env::var_os("ProgramFiles(x86)")?)
        .join("Microsoft Visual Studio")
        .join("Installer")
        .join("vswhere.exe");
    let output = Command::new(vswhere)
        .args(["-latest", "-products", "*", "-requires"])
        .arg(libraries_of_this_computer())
        .args(["-property", "installationPath"])
        .output()
        .ok()?;
    let vc = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()).join("VC");
    let version = std::fs::read_to_string(
        vc.join("Auxiliary")
            .join("Build")
            .join("Microsoft.VCToolsVersion.default.txt"),
    )
    .ok()?;
    Some(vc.join("Tools").join("MSVC").join(version.trim()))
}

/// The component of the C++ tools for this computer's own processor, which
/// every installation that builds programs here has.
fn libraries_of_this_computer() -> &'static str {
    libraries(sayon_target::host()).component
}
