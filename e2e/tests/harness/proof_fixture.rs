//! Change-receipt fixtures.
//!
//! A scenario that needs a *passed* change verification receipt before the
//! product writes one itself writes the receipt here, in the product's
//! `change-proof/v1` shape. This is the record's file format, not its rule:
//! every digest below is computed exactly as the product computes it
//! (`sha2` over the same byte streams), and the product still validates the
//! receipt it reads.
//!
//! `tested_content_digest` is deliberately absent, which the product's
//! freshness rule accepts when the recorded revision is the current one. A
//! fixture that needs the stronger content binding runs the product's check
//! instead.

use std::path::Path;

use sha2::{Digest as _, Sha256};

/// Write a passed change verification receipt for `change`.
pub fn write_change_receipt(change_dir: &Path, change: &str, head: &str, verify_command: &str) {
    let receipt = serde_json::json!({
        "schema": 2,
        "change": change,
        "status": "passed",
        "head": head,
        "features_path": change_dir.join("specs").to_string_lossy(),
        "features_digest": digest_features(&change_dir.join("specs")),
        "command_digest": digest_text(verify_command),
        "command": verify_command,
        "content_scope": "change-proof/v1",
        "runner_exit": 0,
        "result": {
            "scope": null,
            "total": 2,
            "passed": 2,
            "failed": 0,
            "skipped": 0,
        },
        "finished_at_unix": 1,
    });
    let dir = change_dir.join("verification");
    std::fs::create_dir_all(&dir).expect("create the verification directory");
    let path = dir.join("tasks-verification.json");
    std::fs::write(&path, format!("{receipt}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

/// SHA-256 over the sorted feature files' relative paths and contents.
pub fn digest_features(dir: &Path) -> String {
    let mut files = Vec::new();
    collect_feature_files(dir, &mut files);
    files.sort();
    let mut hasher = Sha256::new();
    for path in files {
        let relative = path.strip_prefix(dir).unwrap_or(&path);
        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update(b"\0");
        let bytes =
            std::fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        hasher.update(&bytes);
        hasher.update(b"\0");
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// SHA-256 of a text input, hex-encoded.
pub fn digest_text(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn collect_feature_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_feature_files(&path, out);
        } else if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("feature"))
        {
            out.push(path);
        }
    }
}
