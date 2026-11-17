//! Resolution of the product paths the package drives.
//!
//! The package links no product crate. The built `givn` binary and the built
//! `ready_fixture` arrive by path from the project runner (`run-tests.sh`,
//! `measure-coverage.sh`); a missing or non-executable value fails by name,
//! never by a bare "no such file" from `spawn`.

use std::path::{Path, PathBuf};

/// The built `givn` binary every scenario drives. Required: the package has no
/// in-process product to fall back to.
pub fn givn() -> PathBuf {
    required("GIVN_BIN")
}

/// The built `ready_fixture` the run-execution scenarios start. Required only
/// by those scenarios, so it is resolved on first use rather than at startup.
pub fn ready_fixture() -> PathBuf {
    required("GIVN_READY_FIXTURE")
}

/// The product checkout this package sits in.
///
/// A few scenarios assert on read-only repository files the product ships
/// (`Cargo.toml`, `CHANGELOG.md`, `assets/**`) and on the shared
/// `givn/specs/**` corpus. The package is the product repository's
/// `e2e/` directory, so the default is its parent; `GIVN_ROOT` overrides it
/// for a checkout copy that lives elsewhere.
pub fn product_root() -> PathBuf {
    if let Some(root) = std::env::var_os("GIVN_ROOT") {
        return PathBuf::from(root);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the e2e package always sits inside the product checkout")
        .to_path_buf()
}

fn required(name: &str) -> PathBuf {
    let raw = std::env::var_os(name).unwrap_or_else(|| {
        panic!(
            "{name} is unset: the @e2e package drives the built product, so run \
             ./run-tests.sh --e2e --package e2e (or export {name})"
        )
    });
    let path = PathBuf::from(raw);
    assert!(
        path.is_file(),
        "{name} names {} which is not a file",
        path.display()
    );
    assert!(
        is_executable(&path),
        "{name} names {} which is not executable",
        path.display()
    );
    path
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}

/// The product's version, read from its `Cargo.toml`.
///
/// The package's own `CARGO_PKG_VERSION` is `0.0.0`; a scenario that asserts
/// the version the product prints or writes must read the product's manifest.
pub fn product_version() -> String {
    let manifest = product_root().join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).unwrap_or_else(|error| {
        panic!(
            "cannot read {} for the product version: {error}",
            manifest.display()
        )
    });
    let mut in_package = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package && let Some(rest) = trimmed.strip_prefix("version = ") {
            return rest.trim_matches('"').to_string();
        }
    }
    panic!("{} carries no [package] version", manifest.display());
}
