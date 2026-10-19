//! `givn test-result`: write the four-count runner result contract.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

/// Write the runner result for the current invocation to `GIVN_RESULT_FILE`.
///
/// The runner contract is four integers and a process exit code. `scope` is
/// never required: Givn labels the invocation that supplied the result path.
pub fn run(total: u64, passed: u64, failed: u64, skipped: u64) -> Result<()> {
    let path = std::env::var("GIVN_RESULT_FILE").map(PathBuf::from).context(
        "GIVN_RESULT_FILE is not set — run this helper from a Givn verification command \
         (givn check tasks, givn archive, or a configured hook)",
    )?;
    write_result(&path, total, passed, failed, skipped)?;
    println!(
        "givn test-result: total {total}, passed {passed}, failed {failed}, skipped {skipped}"
    );
    Ok(())
}

fn write_result(path: &Path, total: u64, passed: u64, failed: u64, skipped: u64) -> Result<()> {
    if passed + failed + skipped != total {
        bail!(
            "result counts do not reconcile: passed {passed} + failed {failed} + skipped {skipped} \
             != total {total}"
        );
    }
    let payload = serde_json::json!({
        "total": total,
        "passed": passed,
        "failed": failed,
        "skipped": skipped,
    });
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create result directory {}", parent.display()))?;
    }
    let temporary = path.with_file_name(format!(
        ".{}.{}.givn-tmp",
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "result.json".to_string()),
        std::process::id()
    ));
    std::fs::write(&temporary, format!("{payload}\n"))
        .with_context(|| format!("write result {}", temporary.display()))?;
    std::fs::rename(&temporary, path)
        .with_context(|| format!("publish result {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_reconciling_counts_without_scope() {
        let dir = tempfile::TempDir::new().unwrap();
        let result = dir.path().join("result.json");
        write_result(&result, 3, 3, 0, 0).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&result).unwrap()).unwrap();
        assert_eq!(value["total"], 3);
        assert_eq!(value["passed"], 3);
        assert!(value.get("scope").is_none());
        assert!(!dir.path().join(".result.json.givn-tmp").exists());
    }

    #[test]
    fn writes_failed_and_skipped_counts_for_a_red_run() {
        let dir = tempfile::TempDir::new().unwrap();
        let result = dir.path().join("result.json");
        write_result(&result, 3, 1, 1, 1).unwrap();
        let value: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&result).unwrap()).unwrap();
        assert_eq!(value["failed"], 1);
        assert_eq!(value["skipped"], 1);
    }

    #[test]
    fn rejects_non_reconciling_counts() {
        let dir = tempfile::TempDir::new().unwrap();
        let result = dir.path().join("result.json");
        let err = write_result(&result, 2, 1, 0, 0).unwrap_err();
        assert!(err.to_string().contains("do not reconcile"), "{err}");
    }
}
