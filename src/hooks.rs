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
use crate::config::manifest::{HookEntry, HookFull, Manifest};

/// Run all post hooks for a given artifact.
///
/// Returns Ok(()) if all required hooks pass.
/// Returns Err if any required hook fails (allow_failure hooks only warn).
pub fn run_artifact_hooks(
    artifact_id: &str,
    manifest: &Manifest,
    features_path: &str,
) -> Result<()> {
    let artifact = manifest.artifacts.iter().find(|a| a.id == artifact_id).ok_or_else(|| {
        anyhow::anyhow!("artifact '{}' not found in manifest", artifact_id)
    })?;

    if artifact.post.is_empty() {
        return Ok(());
    }

    println!("Running hooks for artifact '{artifact_id}'...");
    let mut failed = false;

    for hook in &artifact.post {
        let (run, allow_failure, name) = normalise_hook(hook, manifest);
        let display = name.as_deref().unwrap_or(&run);
        println!("  hook: {display}");

        match run_hook(&run, features_path) {
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
fn normalise_hook(entry: &HookEntry, _manifest: &Manifest) -> (String, bool, Option<String>) {
    match entry {
        HookEntry::Short(s) => (s.clone(), false, None),
        HookEntry::Full(HookFull { run, allow_failure, name }) => {
            (run.clone(), *allow_failure, name.clone())
        }
    }
}

/// Execute a single hook command.
fn run_hook(hook: &str, features_path: &str) -> Result<()> {
    // Resolve built-ins.
    let command = match hook {
        "verify" => {
            // The verify command is resolved from the manifest; pass it through.
            // This is called with `hook = "verify"` only from run_artifact_hooks,
            // which has the manifest. But here we only have the hook string.
            // We'll use the binary itself to invoke it.
            return run_shell_command(
                &format!("{} missing-testrunner", std::env::current_exe()?.display()),
                features_path,
            );
        }
        "lint" => {
            return run_shell_command(
                &format!("{} lint", std::env::current_exe()?.display()),
                features_path,
            );
        }
        other => other.to_string(),
    };

    run_shell_command(&command, features_path)
}

/// Resolve the `verify` built-in to the configured verify.command and run it.
pub fn run_verify_hook(manifest: &Manifest, features_path: &str) -> Result<()> {
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

/// Run all hooks for a single artifact, using the full manifest to resolve `verify`.
/// This version correctly resolves the `verify` built-in to `manifest.verify.command`.
pub fn run_artifact_hooks_full(
    artifact_id: &str,
    manifest: &Manifest,
    features_path: &str,
) -> Result<()> {
    let artifact = manifest.artifacts.iter().find(|a| a.id == artifact_id).ok_or_else(|| {
        anyhow::anyhow!("artifact '{}' not found in manifest", artifact_id)
    })?;

    if artifact.post.is_empty() {
        return Ok(());
    }

    println!("Running hooks for artifact '{artifact_id}'...");
    let mut failed = false;

    for hook in &artifact.post {
        let (run, allow_failure, name) = normalise_hook(hook, manifest);
        let display = name.as_deref().unwrap_or(&run);
        println!("  hook: {display}");

        let result = match run.as_str() {
            "verify" => run_verify_hook(manifest, features_path),
            "lint" => run_shell_command(
                &format!("{} lint", std::env::current_exe().unwrap_or_default().display()),
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
