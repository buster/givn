//! Test-only OpenCode runtime admission and supervised process-group leases.

use std::io;
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct Admission {
    pub schema: u32,
    pub lock_sha256: String,
    pub platform: String,
    pub executables: Vec<PathBuf>,
    pub versions: RuntimeVersions,
    pub help_sha256: serde_json::Value,
    pub shell: RuntimeShell,
    pub runtime_tmp: PathBuf,
    pub isolation: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct RuntimeVersions {
    pub v1_floor: String,
    pub v1_latest: String,
    pub v2_latest: String,
    pub node: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct RuntimeShell {
    pub executable: PathBuf,
    pub reason: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SupervisorLease {
    child: Child,
    lease: Option<ChildStdin>,
}

impl SupervisorLease {
    pub fn close(mut self) -> io::Result<std::process::ExitStatus> {
        let status = self.child.wait()?;
        drop(self.lease.take());
        if status.success() {
            Ok(status)
        } else {
            Err(io::Error::other(format!("supervisor exited with {status}")))
        }
    }
}

impl Drop for SupervisorLease {
    fn drop(&mut self) {
        drop(self.lease.take());
        match self.child.wait() {
            Ok(status) if !status.success() => {
                eprintln!("OpenCode supervisor teardown reported {status}");
            }
            Ok(_) => {}
            Err(error) => eprintln!("failed to wait for OpenCode supervisor teardown: {error}"),
        }
    }
}

pub fn admission() -> anyhow::Result<Admission> {
    let json = if let Ok(json) = std::env::var("GIVN_OPENCODE_ADMISSION_JSON") {
        json
    } else {
        let output = Command::new("sh")
            .arg("tests/fixtures/opencode/resolve-runtime.sh")
            .arg("--json")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "runtime validation failed before scenario: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)?
    };
    let parsed: Admission = serde_json::from_str(&json)?;
    anyhow::ensure!(parsed.schema == 1, "unsupported runtime admission schema");
    anyhow::ensure!(
        parsed.lock_sha256 == lock_sha256()?,
        "runtime admission lock digest is stale"
    );
    anyhow::ensure!(
        parsed.versions.v1_floor == "1.18.29"
            && parsed.versions.v1_latest == "1.18.34"
            && parsed.versions.v2_latest == "2.0.22"
            && parsed.versions.node == "v24.21.0",
        "runtime versions do not match the admitted lock"
    );
    anyhow::ensure!(
        parsed.shell.executable.is_file(),
        "selected shell does not exist"
    );
    anyhow::ensure!(
        !parsed.shell.reason.is_empty(),
        "selected shell diagnostic is absent"
    );
    anyhow::ensure!(
        parsed.runtime_tmp.is_dir(),
        "runtime-selected tmp directory is absent"
    );
    for (index, path) in parsed.executables.iter().enumerate() {
        let exists = if index == 5 {
            path.is_dir()
        } else {
            path.is_file()
        };
        anyhow::ensure!(exists, "admitted runtime is missing: {}", path.display());
    }
    Ok(parsed)
}

pub fn ensure_admitted() -> anyhow::Result<Admission> {
    admission()
}

pub fn launch_supervisor_with_env(
    root: PathBuf,
    argv: &[String],
    extra_env: &[(&str, &str)],
) -> anyhow::Result<SupervisorLease> {
    let admission = admission()?;
    anyhow::ensure!(
        std::env::var_os("GIVN_OPENCODE_SUPERVISOR").is_some(),
        "setup failure: GIVN_OPENCODE_SUPERVISOR is not prepared"
    );
    let supervisor = PathBuf::from(std::env::var_os("GIVN_OPENCODE_SUPERVISOR").unwrap());
    anyhow::ensure!(
        supervisor.is_file(),
        "prepared supervisor is missing: {}",
        supervisor.display()
    );
    anyhow::ensure!(
        root.is_absolute()
            && root
                .file_name()
                .is_some_and(|value| value.to_string_lossy().starts_with("givn-opencode-")),
        "refusing fixture root outside owned OpenCode temp directory"
    );
    let root_parent = root.parent().expect("fixture root parent");
    anyhow::ensure!(root_parent.is_dir(), "fixture temp parent does not exist");
    let mut command = Command::new(supervisor);
    command
        .arg("--timeout-seconds")
        .arg("60")
        .arg("--root")
        .arg(&root)
        .arg("--")
        .args(argv)
        .env_clear()
        .env("PATH", allowed_path(&admission))
        .env("HOME", root.join("home"))
        .env("TMPDIR", admission.runtime_tmp.clone())
        .env("XDG_CONFIG_HOME", root.join("config"))
        .env("XDG_DATA_HOME", root.join("data"))
        .env("XDG_CACHE_HOME", root.join("cache"))
        .env("XDG_STATE_HOME", root.join("state"))
        .env("GIT_CONFIG_GLOBAL", root.join("gitconfig"))
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("GIVN_OPENCODE_V1_FLOOR", runtime_at(&admission, 0))
        .env("GIVN_OPENCODE_V1_LATEST", runtime_at(&admission, 1))
        .env("GIVN_OPENCODE_V2_LATEST", runtime_at(&admission, 2))
        .env("GIVN_OPENCODE_NODE", runtime_at(&admission, 3))
        .env("GIVN_OPENCODE_FISH", runtime_at(&admission, 4))
        .env("GIVN_OPENCODE_GIT", runtime_at(&admission, 6))
        .env("GIVN_COMPILED_GIVN", env!("CARGO_BIN_EXE_givn"))
        .env(
            "GIVN_OPENCODE_PROJECT",
            std::env::var_os("GIVN_OPENCODE_PROJECT")
                .unwrap_or(root.join("project").into_os_string()),
        )
        .env(
            "GIVN_FIXTURE_RESULT_PATH",
            std::env::var_os("GIVN_FIXTURE_RESULT_PATH").unwrap_or(
                admission
                    .runtime_tmp
                    .join(format!("givn-opencode-result-{}.json", std::process::id()))
                    .into_os_string(),
            ),
        )
        .env(
            "GIVN_OPENCODE_MODULES",
            admission.executables[5].as_os_str(),
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command.envs(extra_env.iter().copied());
    let mut child = command.spawn()?;
    let lease = child.stdin.take();
    anyhow::ensure!(
        lease.is_some(),
        "supervisor parent lease pipe was not created"
    );
    Ok(SupervisorLease { child, lease })
}

#[allow(dead_code)]
pub fn env_clear_fixture() -> anyhow::Result<Command> {
    let runtime = admission()?;
    let mut command = Command::new("env");
    command
        .env_clear()
        .env("PATH", allowed_path(&runtime))
        .env("HOME", runtime.runtime_tmp.join("home"))
        .env("TMPDIR", runtime.runtime_tmp.clone())
        .env("XDG_CONFIG_HOME", runtime.runtime_tmp.join("config"))
        .env("XDG_DATA_HOME", runtime.runtime_tmp.join("data"))
        .env("XDG_CACHE_HOME", runtime.runtime_tmp.join("cache"))
        .env("XDG_STATE_HOME", runtime.runtime_tmp.join("state"))
        .env("GIT_CONFIG_GLOBAL", runtime.runtime_tmp.join("gitconfig"))
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("LANG", "C.UTF-8")
        .env("LC_ALL", "C.UTF-8")
        .env("GIVN_FIXTURE_API_KEY", "fixture-not-a-secret")
        .env("GIVN_OPENCODE_V1_FLOOR", runtime_at(&runtime, 0))
        .env("GIVN_OPENCODE_V1_LATEST", runtime_at(&runtime, 1))
        .env("GIVN_OPENCODE_V2_LATEST", runtime_at(&runtime, 2))
        .env("GIVN_OPENCODE_NODE", runtime_at(&runtime, 3))
        .env("GIVN_OPENCODE_GIT", runtime_at(&runtime, 6))
        .env("GIVN_OPENCODE_MODULES", &runtime.executables[5])
        .env(
            "GIVN_OPENCODE_ADMISSION_JSON",
            std::env::var_os("GIVN_OPENCODE_ADMISSION_JSON").unwrap_or_default(),
        )
        .env(
            "GIVN_OPENCODE_SUPERVISOR",
            std::env::var_os("GIVN_OPENCODE_SUPERVISOR").unwrap_or_default(),
        );
    Ok(command)
}

fn runtime_at(admission: &Admission, index: usize) -> String {
    admission
        .executables
        .get(index)
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn allowed_path(admission: &Admission) -> String {
    let mut dirs = admission
        .executables
        .iter()
        .filter_map(|path| path.parent())
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    dirs.extend([
        "/usr/bin".to_string(),
        "/bin".to_string(),
        "/usr/sbin".to_string(),
        "/sbin".to_string(),
    ]);
    dirs.join(":")
}

fn lock_sha256() -> anyhow::Result<String> {
    use sha2::{Digest as _, Sha256};
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/opencode/runtime-lock.json");
    let bytes = std::fs::read(path)?;
    let digest = Sha256::digest(bytes);
    Ok(digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>())
}
