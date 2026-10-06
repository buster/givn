//! Test-only OpenCode runtime admission and supervised process-group leases.

use std::io;
use std::path::{Path, PathBuf};
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

/// The runtime-driver key of a feature-file release label.
pub fn release_key(label: &str) -> anyhow::Result<&'static str> {
    match label {
        "V1 floor" => Ok("v1_floor"),
        "V1 latest" => Ok("v1_latest"),
        "V2 latest" => Ok("v2_latest"),
        other => anyhow::bail!("unsupported OpenCode fixture release: {other}"),
    }
}

/// Run the real OpenCode release in `project` (already prepared by the real
/// Givn CLI) through the supervised runtime driver and return its evidence.
///
/// `scenario` is `main` or `subagent`. The project must contain the generated
/// canonical plugin; the driver writes the fixture configuration after the
/// scenario's own commands, never before them.
pub fn run_opencode_shell(
    project: &Path,
    label: &str,
    scenario: &str,
    extra_env: &[(&str, &str)],
) -> anyhow::Result<serde_json::Value> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let admission = admission()?;
    let release = release_key(label)?;
    let temp = std::fs::canonicalize(&admission.runtime_tmp)?;
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = temp.join(format!("givn-opencode-{}-{nonce}", std::process::id()));
    for directory in ["home", "config", "data", "cache", "state", "tmp"] {
        std::fs::create_dir_all(root.join(directory))?;
    }
    let result_path = temp.join(format!(
        "givn-opencode-result-{}-{nonce}.json",
        std::process::id()
    ));
    let project = std::fs::canonicalize(project)?;
    let project_string = project.to_string_lossy().into_owned();
    let result_string = result_path.to_string_lossy().into_owned();
    let mut extras = vec![
        ("GIVN_OPENCODE_PROJECT", project_string.as_str()),
        ("GIVN_FIXTURE_RESULT_PATH", result_string.as_str()),
    ];
    extras.extend_from_slice(extra_env);
    let driver =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/opencode/runtime_driver.mjs");
    let argv = vec![
        admission.executables[3].to_string_lossy().into_owned(),
        driver.to_string_lossy().into_owned(),
        "run".to_owned(),
        release.to_owned(),
        scenario.to_owned(),
    ];
    let lease = launch_supervisor_with_env(root.clone(), &argv, &extras)?;
    lease.close().map_err(|error| {
        anyhow::anyhow!("real OpenCode {label} run or its teardown failed: {error}")
    })?;
    let bytes = std::fs::read(&result_path)
        .map_err(|error| anyhow::anyhow!("real OpenCode {label} left no evidence: {error}"))?;
    let _ = std::fs::remove_file(&result_path);
    let evidence: serde_json::Value = serde_json::from_slice(&bytes)?;
    anyhow::ensure!(
        evidence["release"] == release,
        "evidence is for another release: {}",
        evidence["release"]
    );
    Ok(evidence)
}

/// Directory of the intermediate evidence shards of one scenario.
fn transcript_shard_dir(title: &str) -> PathBuf {
    let slug = title
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("reports/visual")
        .join(slug)
}

/// Write one example's labeled transcript shard. Each example owns its file, so
/// concurrent examples never append to a shared transcript; the assembling
/// script orders the shards.
pub fn write_transcript_shard(title: &str, shard: &str, text: &str) -> anyhow::Result<()> {
    let directory = transcript_shard_dir(title);
    std::fs::create_dir_all(&directory)?;
    // Random temporary-path prefixes are the only redaction.
    let mut text = text.to_owned();
    let mut prefixes = vec![std::env::temp_dir()];
    if let Ok(physical) = std::fs::canonicalize(std::env::temp_dir()) {
        prefixes.push(physical);
    }
    prefixes.sort_by_key(|prefix| std::cmp::Reverse(prefix.as_os_str().len()));
    for prefix in prefixes {
        text = text.replace(prefix.to_string_lossy().trim_end_matches('/'), "<TMPDIR>");
    }
    std::fs::write(directory.join(format!("{shard}.txt")), text)?;
    Ok(())
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
    let repository = std::fs::canonicalize(env!("CARGO_MANIFEST_DIR"))?;
    let physical_parent = std::fs::canonicalize(root_parent)?;
    anyhow::ensure!(
        !physical_parent.starts_with(&repository),
        "refusing fixture root inside the repository checkout: {}",
        root.display()
    );
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
