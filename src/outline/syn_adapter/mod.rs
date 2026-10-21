//! `syn` adapter tree: the only modules that import `syn`.
//!
//! It supplies facts only: spans, line kinds, fields, methods, calls, and a
//! neutral flow tree. Metrics and classification stay above the port.
//!
//! `impl_index` carries the cross-file impl and trait-relation facts and is
//! deliberately syn-free; parser types never cross into it.

mod facts;
mod file_builder;
mod impl_index;
mod spans;
mod syntax;

use super::analyzer::SourceAnalyzer;
use super::model::{
    Analysis, AnalysisRequest, AnalyzerCapabilities, CallId, Construct, Language, Limitation,
    Resolution,
};
use super::scan;
use anyhow::Result;
use file_builder::analyze_file;
use impl_index::ImplCollector;

pub struct SynAnalyzer;

impl SourceAnalyzer for SynAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        AnalyzerCapabilities {
            language: Language::Rust,
            supported_constructs: vec![
                Construct::Struct,
                Construct::Enum,
                Construct::Trait,
                Construct::Union,
                Construct::TypeAlias,
                Construct::InherentImpl,
                Construct::TraitImpl,
                Construct::InlineModule,
                Construct::FileModule,
                Construct::FreeFunction,
            ],
            limitations: vec![
                Limitation::ItemMacros,
                Limitation::IncludeMacro,
                Limitation::NonUtf8File,
            ],
            resolution_labels: vec![
                Resolution::SameFile,
                Resolution::ProjectOtherFile,
                Resolution::NameOnly,
                Resolution::Ambiguous,
                Resolution::Dynamic,
                Resolution::External,
            ],
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        let sources = scan::discover(&request.project_root, &request.scope.roots)?;
        let mut analysis = Analysis::default();
        let mut impls = ImplCollector::default();

        for source in sources {
            analyze_file(source, &mut analysis, &mut impls);
        }

        impl_index::attach_impls(&mut analysis, impls);
        analysis.units.sort_by(|a, b| a.id.cmp(&b.id));
        analysis.diagnostics.sort_by(|a, b| {
            (a.path.clone(), a.line, a.message.clone()).cmp(&(
                b.path.clone(),
                b.line,
                b.message.clone(),
            ))
        });
        Ok(analysis)
    }
}

#[allow(dead_code)]
fn _call_id(_id: CallId) {}

#[cfg(test)]
mod tests {
    //! Adapter behavior tests: unit identity, cross-file impls, and relations.

    use super::super::model::SourceScope;
    use super::SynAnalyzer;
    use super::*;
    use crate::outline::model::{
        Analysis, AnalysisRequest, DiagnosticKind, Language, RepoPath, TypeRelation, UnitId,
        UnitKind,
    };

    fn analyze_sources(files: &[(&str, &str)]) -> Analysis {
        let root = tempfile::tempdir().expect("temp project");
        for (path, content) in files {
            let target = root.path().join(path);
            std::fs::create_dir_all(target.parent().unwrap()).unwrap();
            std::fs::write(target, content).unwrap();
        }
        let request = AnalysisRequest {
            project_root: root.path().to_path_buf(),
            scope: SourceScope {
                language: Language::Rust,
                roots: vec![std::path::PathBuf::from("src")],
                include_test_sources: false,
            },
        };
        SynAnalyzer.analyze(&request).expect("analyze")
    }

    #[test]
    fn type_ids_use_the_declaration_file_and_impls_merge_across_files() {
        let analysis = analyze_sources(&[
            (
                "src/engine.rs",
                "pub struct Engine { a: u64, b: u64 }\nimpl Engine {\n    pub fn first(&self) -> u64 { self.a }\n}\n",
            ),
            (
                "src/engine/parts.rs",
                "impl Engine {\n    pub fn second(&self) -> u64 { self.b }\n    pub fn third(&self) -> u64 { self.a }\n}\n",
            ),
        ]);

        let engine = analysis
            .units
            .iter()
            .find(|unit| unit.id == UnitId("type:src/engine.rs::Engine".to_string()))
            .expect("the type is identified by its declaration file");
        assert_eq!(engine.name, "Engine");
        assert_eq!(
            engine.files,
            vec![
                RepoPath::new("src/engine.rs"),
                RepoPath::new("src/engine/parts.rs")
            ]
        );
        let methods: Vec<&str> = engine
            .methods
            .iter()
            .map(|method| method.name.as_str())
            .collect();
        assert_eq!(methods, vec!["first", "second", "third"]);
        assert!(
            engine
                .spans
                .iter()
                .any(|span| span.file == RepoPath::new("src/engine/parts.rs"))
        );
        assert!(
            analysis
                .units
                .iter()
                .any(|unit| unit.id == UnitId("module:src/engine/parts.rs".to_string()))
        );
        assert!(
            analysis
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.kind != DiagnosticKind::AmbiguousType)
        );
    }

    #[test]
    fn an_ambiguous_type_name_is_not_merged() {
        let analysis = analyze_sources(&[
            ("src/a.rs", "pub struct C;\n"),
            ("src/b.rs", "pub struct C;\n"),
            (
                "src/c.rs",
                "impl C {\n    pub fn only(&self) -> u64 { 1 }\n}\n",
            ),
        ]);

        assert!(analysis.diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == DiagnosticKind::AmbiguousType
                && diagnostic.path == RepoPath::new("src/c.rs")
        }));
        for unit in analysis
            .units
            .iter()
            .filter(|unit| unit.name == "C" && matches!(unit.kind, UnitKind::Type(_)))
        {
            assert!(
                unit.files
                    .iter()
                    .all(|file| file != &RepoPath::new("src/c.rs")),
                "an ambiguous impl was merged into {}",
                unit.id
            );
        }
    }

    #[test]
    fn a_duplicate_name_resolves_by_the_impl_module_tree() {
        let analysis = analyze_sources(&[
            (
                "src/a_b.rs",
                "pub struct C;\nimpl C {\n    pub fn one(&self) {}\n}\n",
            ),
            (
                "src/a_b/parts.rs",
                "impl C {\n    pub fn two(&self) {}\n}\n",
            ),
            (
                "src/a-b.rs",
                "pub struct C;\nimpl C {\n    pub fn one(&self) {}\n}\n",
            ),
            (
                "src/a-b/parts.rs",
                "impl C {\n    pub fn two(&self) {}\n}\n",
            ),
        ]);
        assert!(
            analysis
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.kind != DiagnosticKind::AmbiguousType),
            "each impl belongs to its own module tree"
        );
        for id in ["type:src/a_b.rs::C", "type:src/a-b.rs::C"] {
            let unit = analysis
                .units
                .iter()
                .find(|unit| unit.id == UnitId(id.to_string()))
                .expect("type unit");
            let methods: Vec<&str> = unit
                .methods
                .iter()
                .map(|method| method.name.as_str())
                .collect();
            assert_eq!(methods, vec!["one", "two"]);
            assert_eq!(unit.files.len(), 2);
        }
    }

    #[test]
    fn a_cross_file_trait_impl_records_both_relations() {
        let analysis = analyze_sources(&[
            (
                "src/api.rs",
                "pub trait Api {\n    fn call(&self) -> u64;\n}\n",
            ),
            (
                "src/engine.rs",
                "pub struct Engine;\nimpl Api for Engine {\n    fn call(&self) -> u64 { 1 }\n}\n",
            ),
        ]);

        let api = analysis
            .units
            .iter()
            .find(|unit| unit.id == UnitId("type:src/api.rs::Api".to_string()))
            .expect("trait unit");
        assert!(api.relations.contains(&TypeRelation::ImplementedBy(UnitId(
            "type:src/engine.rs::Engine".to_string()
        ))));
        let engine = analysis
            .units
            .iter()
            .find(|unit| unit.id == UnitId("type:src/engine.rs::Engine".to_string()))
            .expect("implementor unit");
        assert!(
            engine
                .relations
                .contains(&TypeRelation::ImplementsUnit(UnitId(
                    "type:src/api.rs::Api".to_string()
                )))
        );
    }
}
