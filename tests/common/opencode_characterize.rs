//! Test-only driver for `tests/fixtures/opencode/characterize.mjs`.
//!
//! It runs the generated OpenCode attribution plugin through its public hooks
//! with controlled payloads in a fresh Node process per case. It is not
//! evidence of real-loader behavior.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Context;
use serde_json::{Value, json};

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
}

/// Import `asset` afresh and replay `actions` through its public hooks.
pub fn run(
    asset: &Path,
    generation: &str,
    initial_env: &serde_json::Map<String, Value>,
    actions: &[Value],
) -> anyhow::Result<Characterization> {
    let node = std::env::var_os("GIVN_OPENCODE_NODE").context(
        "OpenCode setup failure: GIVN_OPENCODE_NODE is not exported by the development shell",
    )?;
    let harness =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/opencode/characterize.mjs");
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
        .context("spawn the characterization harness")?;
    child
        .stdin
        .take()
        .context("harness stdin")?
        .write_all(request.to_string().as_bytes())
        .context("write the harness request")?;
    let output = child.wait_with_output().context("wait for the harness")?;
    anyhow::ensure!(
        output.status.success(),
        "characterization harness failed ({}): {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let parsed: Value =
        serde_json::from_slice(&output.stdout).context("parse the harness result")?;
    let shells = parsed["shells"]
        .as_array()
        .context("harness result lists shells")?
        .iter()
        .map(|shell| shell["env"].clone())
        .collect();
    Ok(Characterization {
        module: parsed["module"].clone(),
        hooks: parsed["hooks"].clone(),
        shells,
        events: parsed["events"]
            .as_array()
            .context("harness result lists events")?
            .clone(),
    })
}
