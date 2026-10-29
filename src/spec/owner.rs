use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

pub fn resolve_capability_owner(specs_root: &Path, capability: &str) -> Result<Option<PathBuf>> {
    if !specs_root.exists() {
        return Ok(None);
    }
    let mut owners = Vec::new();
    for entry in std::fs::read_dir(specs_root)
        .with_context(|| format!("cannot read {}", specs_root.display()))?
    {
        let path = entry?.path();
        if path.is_dir() && path.join(format!("{capability}.feature")).is_file() {
            owners.push(path);
        }
    }
    owners.sort();
    if owners.len() > 1 {
        bail!(
            "ambiguous capability owners for '{capability}': {}",
            owners
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(owners.pop())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_requires_exactly_one_usecase_or_fragment() {
        let root = tempfile::TempDir::new().unwrap();
        assert!(
            resolve_capability_owner(root.path(), "help")
                .unwrap()
                .is_none()
        );
        for owner in ["first", "second", "fragments"] {
            let dir = root.path().join(owner);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("help.feature"), "Feature: Help\n").unwrap();
            if owner == "first" {
                assert_eq!(
                    resolve_capability_owner(root.path(), "help").unwrap(),
                    Some(dir)
                );
            } else {
                let error = resolve_capability_owner(root.path(), "help")
                    .unwrap_err()
                    .to_string();
                assert!(error.contains("ambiguous capability owners"));
                assert!(error.contains("first"));
                assert!(error.contains(owner));
            }
        }
    }
}
