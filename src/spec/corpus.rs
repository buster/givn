//! In-memory representation of a specification corpus.
//!
//! The interaction gate reads narratives and features from this corpus instead
//! of walking the filesystem itself. A projection overlays a change's accepted
//! narratives and merged feature text on a `SpecCorpus` without creating a
//! temporary specifications tree or writing a file.
//!
//! Paths are project-relative and include the `givn/specs/` prefix so the same
//! key addresses a permanent file and a projected entry. `read` inventories the
//! direct use-case/fragment directories and their direct feature files; nested
//! feature files and nested narrative documents are outside the capability
//! inventory and are intentionally not read.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// The durable narrative and feature text of a specification corpus.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SpecCorpus {
    pub narratives: BTreeMap<PathBuf, String>,
    pub features: BTreeMap<PathBuf, String>,
}

impl SpecCorpus {
    /// Read the direct use-case/fragment documents and feature files below
    /// `specs_root` into memory.
    ///
    /// A missing root is an empty corpus. Any other read failure names the
    /// path. Nested directories are not traversed; the interaction gate's
    /// capability inventory is direct-child only.
    pub fn read(specs_root: &Path) -> Result<Self> {
        let mut corpus = Self::default();
        if !specs_root.exists() {
            return Ok(corpus);
        }
        let entries = std::fs::read_dir(specs_root).with_context(|| {
            format!("cannot read specifications root '{}'", specs_root.display())
        })?;
        for entry in entries {
            let entry = entry.with_context(|| {
                format!(
                    "cannot read a specifications entry in '{}'",
                    specs_root.display()
                )
            })?;
            let path = entry.path();
            let file_type = entry.file_type().with_context(|| {
                format!("cannot inspect specifications entry '{}'", path.display())
            })?;
            if file_type.is_file() && path.extension().is_some_and(|ext| ext == "feature") {
                corpus.insert_feature(&path, specs_root)?;
                continue;
            }
            if !file_type.is_dir() {
                continue;
            }
            let usecase = entry.file_name();
            let narrative_name = if usecase == "fragments" {
                "fragment.md"
            } else {
                "usecase.md"
            };
            let narrative = path.join(narrative_name);
            if narrative.is_file() {
                corpus.insert_narrative(&narrative, specs_root)?;
            }
            let features = std::fs::read_dir(&path)
                .with_context(|| format!("cannot read use-case directory '{}'", path.display()))?;
            for feature in features {
                let feature = feature.with_context(|| {
                    format!("cannot read a feature entry in '{}'", path.display())
                })?;
                let feature_path = feature.path();
                let is_feature = feature
                    .file_type()
                    .with_context(|| format!("cannot inspect '{}'", feature_path.display()))?
                    .is_file()
                    && feature_path.extension().is_some_and(|ext| ext == "feature");
                if is_feature {
                    corpus.insert_feature(&feature_path, specs_root)?;
                }
            }
        }
        Ok(corpus)
    }

    fn insert_narrative(&mut self, path: &Path, specs_root: &Path) -> Result<()> {
        let key = corpus_key(path, specs_root)?;
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read narrative '{}'", path.display()))?;
        self.narratives.insert(key, content);
        Ok(())
    }

    fn insert_feature(&mut self, path: &Path, specs_root: &Path) -> Result<()> {
        let key = corpus_key(path, specs_root)?;
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read feature '{}'", path.display()))?;
        self.features.insert(key, content);
        Ok(())
    }

    /// Replace a narrative document at a corpus key.
    pub fn overlay_narrative(&mut self, target: &Path, content: String) {
        self.narratives.insert(target.to_path_buf(), content);
    }

    /// Remove a capability feature from a use-case or fragment directory.
    ///
    /// A use-case directory that loses its last capability also loses its
    /// `usecase.md`; the shared `fragments` directory keeps `fragment.md`.
    /// An already-absent feature is a no-op.
    pub fn remove_capability(&mut self, usecase: &Path, capability: &str) {
        let feature = usecase.join(format!("{capability}.feature"));
        self.features.remove(&feature);
        if usecase.file_name().is_some_and(|name| name == "fragments") {
            return;
        }
        let has_feature = self.features.keys().any(|path| path.starts_with(usecase));
        if !has_feature {
            self.narratives.remove(&usecase.join("usecase.md"));
        }
    }
}

fn corpus_key(path: &Path, specs_root: &Path) -> Result<PathBuf> {
    let relative = path.strip_prefix(specs_root).with_context(|| {
        format!(
            "specifications entry '{}' is outside '{}'",
            path.display(),
            specs_root.display()
        )
    })?;
    Ok(PathBuf::from("givn/specs").join(relative))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture() -> tempfile::TempDir {
        let root = tempfile::TempDir::new().unwrap();
        let specs = root.path().join("givn/specs");
        fs::create_dir_all(specs.join("orders/nested")).unwrap();
        fs::create_dir_all(specs.join("fragments")).unwrap();
        fs::write(specs.join("orders/usecase.md"), "# Use case: orders\n").unwrap();
        fs::write(specs.join("orders/order.feature"), "Feature: Order\n").unwrap();
        fs::write(
            specs.join("orders/nested/hidden.feature"),
            "Feature: Hidden\n",
        )
        .unwrap();
        fs::write(specs.join("fragments/fragment.md"), "# Fragment: support\n").unwrap();
        fs::write(
            specs.join("fragments/support.feature"),
            "Feature: Support\n",
        )
        .unwrap();
        fs::write(specs.join("root.feature"), "Feature: Root\n").unwrap();
        root
    }

    #[test]
    fn reads_direct_documents_and_features() {
        let root = fixture();
        let corpus = SpecCorpus::read(&root.path().join("givn/specs")).unwrap();
        assert_eq!(
            corpus
                .narratives
                .get(Path::new("givn/specs/orders/usecase.md")),
            Some(&"# Use case: orders\n".to_string())
        );
        assert!(
            corpus
                .features
                .contains_key(Path::new("givn/specs/orders/order.feature"))
        );
        assert!(
            corpus
                .features
                .contains_key(Path::new("givn/specs/fragments/support.feature"))
        );
        assert!(
            corpus
                .features
                .contains_key(Path::new("givn/specs/root.feature"))
        );
        assert!(
            !corpus
                .features
                .contains_key(Path::new("givn/specs/orders/nested/hidden.feature"))
        );
    }

    #[test]
    fn missing_root_is_empty() {
        let root = tempfile::TempDir::new().unwrap();
        let corpus = SpecCorpus::read(&root.path().join("givn/specs")).unwrap();
        assert!(corpus.narratives.is_empty());
        assert!(corpus.features.is_empty());
    }

    #[test]
    fn a_file_root_is_an_error() {
        let root = tempfile::TempDir::new().unwrap();
        let file = root.path().join("givn/specs");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "not a directory").unwrap();
        assert!(SpecCorpus::read(&file).is_err());
    }

    #[test]
    fn overlay_replaces_a_narrative() {
        let root = fixture();
        let mut corpus = SpecCorpus::read(&root.path().join("givn/specs")).unwrap();
        corpus.overlay_narrative(
            Path::new("givn/specs/orders/usecase.md"),
            "# Use case: orders\n\n## Interactions\n\n- none\n".to_string(),
        );
        assert!(
            corpus
                .narratives
                .get(Path::new("givn/specs/orders/usecase.md"))
                .unwrap()
                .contains("## Interactions")
        );
    }

    #[test]
    fn retirement_removes_a_last_usecase_narrative() {
        let root = fixture();
        let mut corpus = SpecCorpus::read(&root.path().join("givn/specs")).unwrap();
        corpus.remove_capability(Path::new("givn/specs/orders"), "order");
        assert!(
            !corpus
                .features
                .contains_key(Path::new("givn/specs/orders/order.feature"))
        );
        assert!(
            !corpus
                .narratives
                .contains_key(Path::new("givn/specs/orders/usecase.md"))
        );
    }

    #[test]
    fn retirement_keeps_the_fragments_narrative() {
        let root = fixture();
        let mut corpus = SpecCorpus::read(&root.path().join("givn/specs")).unwrap();
        corpus.remove_capability(Path::new("givn/specs/fragments"), "support");
        assert!(
            !corpus
                .features
                .contains_key(Path::new("givn/specs/fragments/support.feature"))
        );
        assert!(
            corpus
                .narratives
                .contains_key(Path::new("givn/specs/fragments/fragment.md"))
        );
    }

    #[test]
    fn retirement_is_idempotent() {
        let root = fixture();
        let mut corpus = SpecCorpus::read(&root.path().join("givn/specs")).unwrap();
        corpus.remove_capability(Path::new("givn/specs/orders"), "order");
        corpus.remove_capability(Path::new("givn/specs/orders"), "order");
        assert!(
            !corpus
                .features
                .contains_key(Path::new("givn/specs/orders/order.feature"))
        );
        assert!(
            !corpus
                .narratives
                .contains_key(Path::new("givn/specs/orders/usecase.md"))
        );
    }
}
