//! Project-local Git integration installed by `givn init` and skill sync.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::embedded::Assets;

const LAUNCHER_MARKER: &str = "# givn commit-attribution launcher";
const ORIGINAL_HOOK_SUFFIX: &str = ".givn-original";

/// Materialize the shared commit hook and activate it for the current worktree
/// without replacing an existing local commit hook.
pub fn install(project_root: &Path) -> Result<()> {
    if let Ok(crate::scm::Backend::Fossil(_)) = crate::scm::select(project_root) {
        // Fossil has no hook mechanism givn can extend; attribution exists
        // only through `givn commit`.
        println!("  commit attribution: givn commit path only (raw fossil commit is not stamped)");
        return Ok(());
    }
    let shared_dir = project_root.join(".githooks");
    std::fs::create_dir_all(&shared_dir)
        .with_context(|| format!("create {}", shared_dir.display()))?;
    write_executable(&shared_dir.join("commit-msg"), &asset("hooks/commit-msg")?)?;
    println!("  commit attribution: .githooks/commit-msg");

    let hooks_dir = match git_hooks_dir(project_root) {
        Some(path) => path,
        None => {
            println!("  Git activation: unavailable (no Git worktree)");
            return Ok(());
        }
    };
    std::fs::create_dir_all(&hooks_dir)
        .with_context(|| format!("create Git hooks directory {}", hooks_dir.display()))?;

    let launcher = hooks_dir.join("commit-msg");
    let backup = hooks_dir.join(format!("commit-msg{ORIGINAL_HOOK_SUFFIX}"));
    if launcher.exists() && !contains_marker(&launcher)? {
        if !backup.exists() {
            std::fs::rename(&launcher, &backup).with_context(|| {
                format!(
                    "preserve existing Git commit hook {} as {}",
                    launcher.display(),
                    backup.display()
                )
            })?;
        } else {
            // Do not overwrite a hook or an existing backup that Givn cannot
            // identify safely. The checked-in hook remains available for a
            // later explicit activation decision.
            println!("  Git activation: unchanged (existing hook path preserved)");
            return Ok(());
        }
    }

    write_executable(&launcher, &asset("hooks/git-commit-msg-launcher")?)?;
    println!("  Git activation: local commit-msg dispatcher installed");
    Ok(())
}

fn asset(path: &str) -> Result<String> {
    Assets::get_str(path).ok_or_else(|| anyhow::anyhow!("embedded Git asset missing: {path}"))
}

fn git_hooks_dir(project_root: &Path) -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "hooks"])
        .current_dir(project_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_string());
    Some(if path.is_absolute() {
        path
    } else {
        project_root.join(path)
    })
}

fn contains_marker(path: &Path) -> Result<bool> {
    Ok(std::fs::read_to_string(path)
        .with_context(|| format!("read Git hook {}", path.display()))?
        .contains(LAUNCHER_MARKER))
}

fn write_executable(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content).with_context(|| format!("write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions)?;
    }
    Ok(())
}
