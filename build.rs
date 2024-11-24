extern crate winreg;
#[macro_use]
extern crate failure;

use std::env::var;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;
use failure::Error;

fn get_windows_kits_dir() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;
    Ok(dir.into())
}

fn get_km_dir(win_kits_dir: &PathBuf) -> Result<PathBuf, Error> {
    let readdir = Path::new(win_kits_dir).join("lib").read_dir()?;

    let max_libdir = readdir
        .filter_map(|dir| dir.ok())
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map(|c| c.starts_with("10.") && dir.join("km").is_dir())
                .unwrap_or(false)
        }).max()
        .ok_or_else(|| format_err!("Failed to find valid km directory in `{:?}`", win_kits_dir))?;

    Ok(max_libdir.join("km"))
}

fn main() {
    let win_kits_dir = get_windows_kits_dir().unwrap();
    let win_km_dir = get_km_dir(&win_kits_dir).unwrap();

    let target = var("TARGET").unwrap();

    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("i686") {
        "x86"
    } else {
        panic!("Unsupported target architecture");
    };

    let win_lib_dir = win_km_dir.join(arch);
    println!(
        "cargo:rustc-link-search=native={}",
        win_lib_dir.to_str().unwrap()
    );
}
