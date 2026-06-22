//! Shared test fixtures for givn characterization tests.
//!
//! Black-box seam: invoke the compiled `givn` binary as a subprocess with a
//! per-test `tempfile::TempDir` as the current directory. The `TempDir` is
//! held by the returned `TestEnv` so it lives as long as the test needs it;
//! it is removed automatically on drop, including on panic.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

/// Handle carrying the per-test temp directory. Drop removes the directory.
#[derive(Debug)]
pub struct TestEnv {
    dir: TempDir,
}

impl TestEnv {
    /// Absolute path to the temp directory (the project root for `givn`).
    pub fn root(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }

    /// Join a relative path under the temp root.
    pub fn path(&self, rel: &str) -> PathBuf {
        self.root().join(rel)
    }
}

/// A bare temp directory with no `givn/` subtree — for the uninitialised
/// project scenario. Do NOT run `givn init` here.
pub fn make_bare_root() -> TestEnv {
    let dir = TempDir::new().expect("create bare temp dir");
    TestEnv { dir }
}

/// The default fixture: a temp directory with `givn init` already run, so
/// `givn/specs`, `givn/changes`, `givn/archive`, config, and skills are
/// scaffolded exactly as a real initialised project (no duplicated logic).
pub fn make_init_root() -> TestEnv {
    let env = make_bare_root();
    let out = run_givn(&env, &["init"]);
    assert!(
        out.status.success(),
        "givn init failed in temp dir: stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    env
}

/// Invoke the `givn` binary with `env`'s temp dir as current directory and
/// the given args, returning captured stdout/stderr/exit code.
pub fn run_givn(env: &TestEnv, args: &[&str]) -> Output {
    let bin = env!("CARGO_BIN_EXE_givn");
    Command::new(bin)
        .args(args)
        .current_dir(env.root())
        .output()
        .expect("spawn givn subprocess")
}

/// Write a project override at `givn/artifacts/<artifact>/<file>` inside the
/// temp root. `givn init` does not scaffold `givn/artifacts/` (SMELL-3), so
/// this helper `create_dir_all`s the subtree itself.
pub fn write_override(env: &TestEnv, artifact: &str, file: &str, content: &str) {
    let dir = env.path(&format!("givn/artifacts/{artifact}"));
    fs::create_dir_all(&dir).expect("create artifacts override dir");
    fs::write(dir.join(file), content).expect("write override file");
}
