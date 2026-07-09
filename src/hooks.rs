use crate::config::manifest::{HookEntry, HookFull, Manifest};
/// Hook execution engine (ADR-028, arc42 ch.8.12).
///
/// Resolves and executes artifact `post` hooks:
/// - Named built-ins: `verify` (run `verify.command`) or `lint` (run `givn lint`).
/// - Shell command strings.
/// - Object form with `allow_failure` / `name`.
///
/// Hooks run with `GIVN_FEATURES` set to the appropriate features path.
/// Failure blocks the gate unless `allow_failure: true`.
use anyhow::Result;

/// Run all post hooks for a given artifact.
///
/// The `verify` built-in is resolved to `manifest.verify.command` — never to the
/// sentinel. This is the single correct implementation; use it everywhere.
///
/// Returns Ok(()) if all required hooks pass.
/// Returns Err if any required hook fails (allow_failure hooks only warn).
pub fn run_artifact_hooks(
    artifact_id: &str,
    manifest: &Manifest,
    features_path: &str,
) -> Result<()> {
    let artifact = manifest
        .artifacts
        .iter()
        .find(|a| a.id == artifact_id)
        .ok_or_else(|| anyhow::anyhow!("artifact '{}' not found in manifest", artifact_id))?;

    if artifact.post.is_empty() {
        return Ok(());
    }

    println!("Running hooks for artifact '{artifact_id}'...");
    let mut failed = false;

    for hook in &artifact.post {
        let (run, allow_failure, name) = normalise_hook(hook);
        let display = name.as_deref().unwrap_or(&run);
        println!("  hook: {display}");

        let result = match run.as_str() {
            "verify" => run_verify_hook(manifest, features_path),
            "lint" => run_shell_command(
                &format!(
                    "{} lint",
                    std::env::current_exe().unwrap_or_default().display()
                ),
                features_path,
            ),
            other => run_shell_command(other, features_path),
        };

        match result {
            Ok(()) => {
                println!("  ✓ {display}");
            }
            Err(e) if allow_failure => {
                eprintln!("  ⚠ {display} (failed, allow_failure=true): {e}");
            }
            Err(e) => {
                eprintln!("  ✗ {display}: {e}");
                failed = true;
            }
        }
    }

    if failed {
        anyhow::bail!("one or more required hooks failed for artifact '{artifact_id}'");
    }
    Ok(())
}

/// Run hooks for all `apply.requires` artifacts as the archive gate.
pub fn run_archive_gate_hooks(manifest: &Manifest, features_path: &str) -> Result<()> {
    for req in &manifest.apply.requires {
        run_artifact_hooks(req, manifest, features_path)?;
    }
    Ok(())
}

/// Normalise a HookEntry into (command_or_builtin, allow_failure, display_name).
fn normalise_hook(entry: &HookEntry) -> (String, bool, Option<String>) {
    match entry {
        HookEntry::Short(s) => (s.clone(), false, None),
        HookEntry::Full(HookFull {
            run,
            allow_failure,
            name,
        }) => (run.clone(), *allow_failure, name.clone()),
    }
}

/// Resolve the `verify` built-in to `manifest.verify.command` and run it.
fn run_verify_hook(manifest: &Manifest, features_path: &str) -> Result<()> {
    let command = &manifest.verify.command;
    println!("  Running: {command}");
    run_shell_command(command, features_path)
}

/// Execute a shell command with GIVN_FEATURES set.
fn run_shell_command(command: &str, features_path: &str) -> Result<()> {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .env("GIVN_FEATURES", features_path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!(
            "command '{}' exited with status: {}",
            command,
            status.code().unwrap_or(-1)
        )
    }
}
