//! Test-only driver for `tests/fixtures/opencode/characterize.mjs`.
//!
//! It runs the generated OpenCode attribution plugin through its public hooks
//! with controlled payloads in a fresh Node process per case. It is not
//! evidence of real-loader behavior.

use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Context;
use serde_json::{Value, json};

/// Longest a characterization case may run. The plugin's own bounded wait for
/// a configured default is two seconds, so a case beyond this is a hang.
const DEADLINE: Duration = Duration::from_secs(20);

/// Result of one characterization run.
#[derive(Debug, Clone)]
pub struct Characterization {
    /// Shape of the imported module: `id`, `server`, `setup`, `named`.
    pub module: Value,
    /// Hook names the plugin registered, per generation.
    pub hooks: Value,
    /// The environment of every created shell, in creation order.
    pub shells: Vec<Value>,
    /// Every event the harness fired: hook group, hook name, sorted payload keys.
    pub events: Vec<Value>,
    /// Every `console.warn` message the plugin emitted.
    pub warnings: Vec<String>,
}

/// Import `asset` in a fresh process and replay `actions` through its public hooks.
pub fn run(
    asset: &Path,
    generation: &str,
    initial_env: &serde_json::Map<String, Value>,
    actions: &[Value],
) -> anyhow::Result<Characterization> {
    let node = std::env::var_os("GIVN_OPENCODE_NODE").with_context(
        || "OpenCode setup failure: GIVN_OPENCODE_NODE is not exported by the development shell",
    )?;
    let harness =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/opencode/characterize.mjs");
    run_with(
        &node,
        &harness,
        DEADLINE,
        asset,
        generation,
        initial_env,
        actions,
    )
}

/// Like [`run`], with an explicit interpreter, harness script and deadline.
/// The child is killed when the deadline passes.
pub fn run_with(
    node: &OsStr,
    harness: &Path,
    deadline: Duration,
    asset: &Path,
    generation: &str,
    initial_env: &serde_json::Map<String, Value>,
    actions: &[Value],
) -> anyhow::Result<Characterization> {
    let request = json!({
        "asset": asset,
        "generation": generation,
        "initialEnv": initial_env,
        "actions": actions,
    });
    let mut child = Command::new(node)
        .arg(harness)
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn the characterization harness {}", harness.display()))?;
    let mut stdin = child.stdin.take().context("harness stdin")?;
    let mut stdout = child.stdout.take().context("harness stdout")?;
    let mut stderr = child.stderr.take().context("harness stderr")?;
    // Drain both pipes concurrently so a full pipe never blocks the child.
    let out_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let err_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });
    // A harness that stops reading early closes the pipe; its exit status explains why.
    let written = stdin.write_all(request.to_string().as_bytes());
    drop(stdin);

    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().context("poll the harness")? {
            break status;
        }
        if started.elapsed() >= deadline {
            child.kill().context("kill the hung harness")?;
            child.wait().context("reap the hung harness")?;
            anyhow::bail!(
                "characterization harness exceeded its {} ms deadline for {generation} {}",
                deadline.as_millis(),
                asset.display()
            );
        }
        thread::sleep(Duration::from_millis(10));
    };
    let stdout = out_reader
        .join()
        .map_err(|_| anyhow::anyhow!("harness stdout reader panicked"))?
        .context("read the harness stdout")?;
    let stderr = err_reader
        .join()
        .map_err(|_| anyhow::anyhow!("harness stderr reader panicked"))?
        .context("read the harness stderr")?;
    anyhow::ensure!(
        status.success(),
        "characterization harness failed ({status}): {}",
        String::from_utf8_lossy(&stderr)
    );
    written.with_context(|| "write the harness request")?;
    let parsed: Value = serde_json::from_slice(&stdout).with_context(|| {
        format!(
            "parse the harness result of {generation} {}",
            asset.display()
        )
    })?;
    let shells = parsed["shells"]
        .as_array()
        .with_context(|| "harness result lists shells")?
        .iter()
        .map(|shell| shell["env"].clone())
        .collect();
    Ok(Characterization {
        module: parsed["module"].clone(),
        hooks: parsed["hooks"].clone(),
        shells,
        events: parsed["events"]
            .as_array()
            .with_context(|| "harness result lists events")?
            .clone(),
        warnings: parsed["warnings"]
            .as_array()
            .with_context(|| "harness result lists warnings")?
            .iter()
            .map(|warning| warning.as_str().unwrap_or_default().to_owned())
            .collect(),
    })
}
