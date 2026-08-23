//! Shared test fixtures for givn characterization tests.
//!
//! Black-box seam: invoke the compiled `givn` binary as a subprocess with a
//! per-test `tempfile::TempDir` as the current directory. The `TempDir` is
//! held by the returned `TestEnv` so it lives as long as the test needs it;
//! it is removed automatically on drop, including on panic.

use std::fs;
use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use std::time::Duration;
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

/// A local Hugging Face-compatible source used by model acquisition scenarios.
/// It serves deterministic small artifacts and is never used by production code.
#[derive(Debug)]
pub struct ArtifactFixtureServer {
    endpoint: String,
    stop: Arc<AtomicBool>,
    events: Arc<Mutex<Vec<String>>>,
    thread: Option<JoinHandle<()>>,
}

impl ArtifactFixtureServer {
    pub fn start() -> Self {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind artifact fixture");
        listener
            .set_nonblocking(true)
            .expect("configure artifact fixture listener");
        let endpoint = format!(
            "http://{}",
            listener
                .local_addr()
                .expect("read artifact fixture address")
        );
        let stop = Arc::new(AtomicBool::new(false));
        let events = Arc::new(Mutex::new(Vec::new()));
        let stop_thread = Arc::clone(&stop);
        let events_thread = Arc::clone(&events);
        let thread = thread::spawn(move || {
            while !stop_thread.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _)) => serve_artifact_request(stream, &events_thread),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(_) => break,
                }
            }
        });
        Self {
            endpoint,
            stop,
            events,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn events(&self) -> Vec<String> {
        self.events.lock().expect("lock artifact events").clone()
    }
}

impl Drop for ArtifactFixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            thread.join().expect("join artifact fixture server");
        }
    }
}

fn serve_artifact_request(mut stream: TcpStream, events: &Mutex<Vec<String>>) {
    let mut request = [0u8; 8192];
    let Ok(size) = stream.read(&mut request) else {
        return;
    };
    let request = String::from_utf8_lossy(&request[..size]);
    let Some(path) = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
    else {
        return;
    };
    events
        .lock()
        .expect("lock artifact events")
        .push(path.to_string());
    let payload = artifact_fixture_payload(path);
    let metadata = request.contains("bytes=0-0");
    let body = if metadata {
        &payload[..payload.len().min(1)]
    } else {
        payload.as_slice()
    };
    let end = body.len().saturating_sub(1);
    let response = format!(
        "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nContent-Range: bytes 0-{end}/{}\r\netag: \"fixture-{}\"\r\nx-repo-commit: {}\r\nConnection: close\r\n\r\n",
        body.len(),
        payload.len(),
        stable_fixture_hash(path),
        fixture_revision(path),
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(body);
}

fn fixture_revision(path: &str) -> &str {
    path.split_once("/resolve/")
        .and_then(|(_, rest)| rest.split_once('/'))
        .map(|(revision, _)| revision)
        .unwrap_or("fixture")
}

fn stable_fixture_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn artifact_fixture_payload(path: &str) -> Vec<u8> {
    if path.ends_with("tokenizer.json") {
        return br#"{
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [],
            "normalizer": null,
            "pre_tokenizer": null,
            "post_processor": null,
            "decoder": null,
            "model": {
                "type": "BPE",
                "dropout": null,
                "unk_token": "[UNK]",
                "fuse_unk": false,
                "byte_fallback": false,
                "vocab": {"[UNK]": 0, "[PAD]": 1, "hello": 2},
                "merges": []
            }
        }"#
        .to_vec();
    }
    if path.ends_with("config.json") && path.contains("mDeBERTa") {
        return br#"{"id2label":{"0":"entailment","1":"neutral","2":"contradiction"},"pad_token_id":0}"#.to_vec();
    }
    if path.ends_with("config.json") {
        return br#"{"pad_token_id":0}"#.to_vec();
    }
    if path.ends_with("tokenizer_config.json") {
        return br#"{"model_max_length":512,"pad_token":"[PAD]"}"#.to_vec();
    }
    if path.ends_with("special_tokens_map.json") {
        return b"{}".to_vec();
    }
    if path.ends_with("model.onnx.data") {
        return b"fixture-onnx-external-data".to_vec();
    }
    if path.ends_with("model.onnx") {
        return b"fixture-onnx-model".to_vec();
    }
    b"fixture-artifact".to_vec()
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
    run_givn_with_env(env, args, None)
}

/// Invoke the `givn` binary with one optional environment override.
pub fn run_givn_with_env(
    env: &TestEnv,
    args: &[&str],
    environment: Option<(&str, &str)>,
) -> Output {
    let environments = environment.into_iter().collect::<Vec<_>>();
    run_givn_with_envs(env, args, &environments)
}

/// Invoke the `givn` binary with multiple optional environment overrides.
pub fn run_givn_with_envs(env: &TestEnv, args: &[&str], environments: &[(&str, &str)]) -> Output {
    let bin = env!("CARGO_BIN_EXE_givn");
    let mut command = Command::new(bin);
    command
        .args(args)
        .current_dir(env.root())
        .env_remove("GIVN_SPEC_MAX_RERANK_PAIRS");
    for (key, value) in environments {
        command.env(key, value);
    }
    command.output().expect("spawn givn subprocess")
}

/// Invoke the `givn` binary with `env`'s temp dir as current directory,
/// one stdin line written (`answer`), then stdin closed. Simulates an
/// interactive answer to a prompt without a terminal.
pub fn run_givn_with_stdin(env: &TestEnv, args: &[&str], answer: &str) -> Output {
    run_givn_with_stdin_and_envs(env, args, answer, &[])
}

/// Invoke the `givn` binary with piped stdin and environment overrides.
pub fn run_givn_with_stdin_and_envs(
    env: &TestEnv,
    args: &[&str],
    input: &str,
    environments: &[(&str, &str)],
) -> Output {
    use std::io::Write;
    use std::process::Stdio;

    let bin = env!("CARGO_BIN_EXE_givn");
    let mut child = Command::new(bin)
        .args(args)
        .current_dir(env.root())
        .envs(environments.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn givn subprocess with piped stdin");

    {
        let mut stdin = child.stdin.take().expect("piped stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("write answer to givn stdin");
    }

    child.wait_with_output().expect("wait for givn subprocess")
}

/// Write a project override at `givn/artifacts/<artifact>/<file>` inside the
/// temp root. `givn init` does not scaffold `givn/artifacts/` (SMELL-3), so
/// this helper `create_dir_all`s the subtree itself.
pub fn write_override(env: &TestEnv, artifact: &str, file: &str, content: &str) {
    let dir = env.path(&format!("givn/artifacts/{artifact}"));
    fs::create_dir_all(&dir).expect("create artifacts override dir");
    fs::write(dir.join(file), content).expect("write override file");
}
