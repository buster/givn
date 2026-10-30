//! Evaluation-only Tree-sitter fact extraction for Rust and Python.
//!
//! This module is compiled only for unit tests. It parses representative
//! sources with Tree-sitter and compares the extracted facts with the current
//! adapters. It is not wired into the outline command and does not change the
//! default report.

use super::model::{Analysis, Construct, Language, TypeKind, UnitKind};
use anyhow::{Context, Result};
use std::collections::BTreeSet;
use tree_sitter::{Node, Parser};

/// Parser-neutral facts used only by the evaluation comparison.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Facts {
    pub units: BTreeSet<String>,
    pub methods: BTreeSet<String>,
    pub fields: BTreeSet<String>,
    pub calls: BTreeSet<String>,
    pub spans: BTreeSet<String>,
    pub unsupported: BTreeSet<String>,
    pub errors: BTreeSet<String>,
}

impl Facts {
    fn insert_unit(&mut self, language: &str, kind: &str, name: &str) {
        self.units.insert(format!("{language}:{kind}:{name}"));
    }

    fn insert_method(&mut self, language: &str, unit: &str, method: &str) {
        self.methods.insert(format!("{language}:{unit}:{method}"));
    }

    fn insert_field(&mut self, language: &str, unit: &str, field: &str) {
        self.fields.insert(format!("{language}:{unit}:{field}"));
    }

    fn insert_call(&mut self, language: &str, unit: &str, method: &str, callee: &str) {
        self.calls
            .insert(format!("{language}:{unit}:{method}:{callee}"));
    }

    fn insert_span(&mut self, path: &str, start: usize, end: usize) {
        self.spans.insert(format!("{path}:{start}:{end}"));
    }
}

/// One comparison row for a fact category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComparisonRow {
    pub category: &'static str,
    pub current: usize,
    pub tree_sitter: usize,
    pub missing_required: usize,
    pub extra_syntax: usize,
    pub classification: &'static str,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Evaluation {
    pub rows: Vec<ComparisonRow>,
    pub errors: Vec<String>,
}

impl Evaluation {
    pub fn decision(&self) -> &'static str {
        if self
            .rows
            .iter()
            .any(|row| row.classification == "missing_required")
        {
            "RETAIN"
        } else {
            "REPLACEMENT_ELIGIBLE"
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::from(
            "Tree-sitter outline evaluation\ncategory | current | tree-sitter | missing | extra | classification\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{} | {} | {} | {} | {} | {}\n",
                row.category,
                row.current,
                row.tree_sitter,
                row.missing_required,
                row.extra_syntax,
                row.classification
            ));
        }
        out.push_str(&format!("decision: {}\n", self.decision()));
        for error in &self.errors {
            out.push_str(&format!("error: {error}\n"));
        }
        out
    }
}

/// Compare the current analyzer with Tree-sitter facts for the same sources.
pub fn evaluate(
    project_root: &std::path::Path,
    roots: &[std::path::PathBuf],
) -> Result<Evaluation> {
    let request = super::model::AnalysisRequest {
        project_root: project_root.to_path_buf(),
        scope: super::model::SourceScope {
            languages: vec![Language::Rust, Language::Python],
            roots: roots.to_vec(),
            include_test_sources: false,
        },
    };
    let current = super::analyzer::outline_analyzer().analyze(&request)?;
    let mut facts = Facts::default();
    for source in super::scan::discover(project_root, roots, &[Language::Rust])? {
        extract_rust(&source.path.as_str(), &source.content, &mut facts)?;
    }
    for source in super::scan::discover(project_root, roots, &[Language::Python])? {
        extract_python(&source.path.as_str(), &source.content, &mut facts)?;
    }

    let current_units: BTreeSet<String> = current.units.iter().map(unit_key).collect();
    let current_methods: BTreeSet<String> = current
        .units
        .iter()
        .flat_map(|unit| {
            unit.methods.iter().map(|method| {
                format!(
                    "{}:{}:{}",
                    language_label(unit.language),
                    unit.name,
                    method.name
                )
            })
        })
        .collect();
    let current_fields: BTreeSet<String> = current
        .units
        .iter()
        .flat_map(|unit| {
            unit.fields.iter().map(|field| {
                format!(
                    "{}:{}:{}",
                    language_label(unit.language),
                    unit.name,
                    field.name
                )
            })
        })
        .collect();
    let current_calls: BTreeSet<String> = current
        .units
        .iter()
        .flat_map(|unit| {
            unit.methods.iter().flat_map(|method| {
                method.calls.iter().map(|call| {
                    format!(
                        "{}:{}:{}:{}",
                        language_label(unit.language),
                        unit.name,
                        method.name,
                        call.callee
                    )
                })
            })
        })
        .collect();
    let current_spans: BTreeSet<String> = current
        .units
        .iter()
        .flat_map(|unit| {
            unit.spans.iter().map(|span| {
                format!(
                    "{}:{}:{}",
                    span.file.as_str(),
                    span.start_line,
                    span.end_line
                )
            })
        })
        .collect();

    let mut rows = vec![
        compare("units", &current_units, &facts.units),
        compare("methods", &current_methods, &facts.methods),
        compare("fields", &current_fields, &facts.fields),
        compare("calls", &current_calls, &facts.calls),
        compare("spans", &current_spans, &facts.spans),
    ];
    rows.push(compare(
        "unsupported",
        &current_unsupported(&current),
        &facts.unsupported,
    ));
    Ok(Evaluation {
        rows,
        errors: Vec::new(),
    })
}

fn compare(
    category: &'static str,
    current: &BTreeSet<String>,
    tree_sitter: &BTreeSet<String>,
) -> ComparisonRow {
    let missing_required = current.difference(tree_sitter).count();
    let extra_syntax = tree_sitter.difference(current).count();
    let classification = if missing_required > 0 {
        "missing_required"
    } else if extra_syntax > 0 {
        "extra_syntax"
    } else {
        "equivalent"
    };
    ComparisonRow {
        category,
        current: current.len(),
        tree_sitter: tree_sitter.len(),
        missing_required,
        extra_syntax,
        classification,
    }
}

fn unit_key(unit: &super::model::SourceUnit) -> String {
    let kind = match unit.kind {
        UnitKind::Type(TypeKind::Struct) => "struct",
        UnitKind::Type(TypeKind::Enum) => "enum",
        UnitKind::Type(TypeKind::Trait) => "trait",
        UnitKind::Type(TypeKind::Union) => "union",
        UnitKind::Type(TypeKind::TypeAlias) => "alias",
        UnitKind::Type(TypeKind::Class) => "class",
        UnitKind::Module => "module",
    };
    format!("{}:{kind}:{}", language_label(unit.language), unit.name)
}

fn current_unsupported(analysis: &Analysis) -> BTreeSet<String> {
    analysis
        .units
        .iter()
        .flat_map(|unit| {
            unit.unsupported
                .iter()
                .map(|note| format!("{}:{:?}", language_label(unit.language), note.construct))
        })
        .collect()
}

fn language_label(language: Language) -> &'static str {
    match language {
        Language::Rust => "rust",
        Language::Python => "python",
    }
}

fn extract_rust(path: &str, source: &str, facts: &mut Facts) -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .context("load the Rust Tree-sitter grammar")?;
    let tree = parser
        .parse(source, None)
        .context("parse the Rust source with Tree-sitter")?;
    if tree.root_node().has_error() {
        facts.errors.insert(path.to_string());
    }
    walk_rust(tree.root_node(), source, path, None, None, facts);
    Ok(())
}

fn extract_python(path: &str, source: &str, facts: &mut Facts) -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .context("load the Python Tree-sitter grammar")?;
    let tree = parser
        .parse(source, None)
        .context("parse the Python source with Tree-sitter")?;
    if tree.root_node().has_error() {
        facts.errors.insert(path.to_string());
    }
    walk_python(tree.root_node(), source, path, None, None, facts);
    Ok(())
}

fn field_name<'a>(node: Node<'a>, field: &str, source: &'a str) -> Option<&'a str> {
    node.child_by_field_name(field)
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
}

fn text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

fn walk_rust(
    node: Node<'_>,
    source: &str,
    path: &str,
    unit: Option<&str>,
    method: Option<&str>,
    facts: &mut Facts,
) {
    let current_unit = match node.kind() {
        "struct_item" | "enum_item" | "trait_item" | "union_item" | "type_item" => {
            let kind = match node.kind() {
                "struct_item" => "struct",
                "enum_item" => "enum",
                "trait_item" => "trait",
                "union_item" => "union",
                _ => "alias",
            };
            if let Some(name) = field_name(node, "name", source) {
                facts.insert_unit("rust", kind, name);
                facts.insert_span(
                    path,
                    node.start_position().row + 1,
                    node.end_position().row + 1,
                );
                Some(name)
            } else {
                unit
            }
        }
        "impl_item" => node
            .child_by_field_name("type")
            .and_then(|child| child.utf8_text(source.as_bytes()).ok())
            .map(str::trim)
            .or(unit),
        "function_item" => {
            if let Some(parent_unit) = unit {
                if let Some(name) = field_name(node, "name", source) {
                    facts.insert_method("rust", parent_unit, name);
                }
            } else if let Some(name) = field_name(node, "name", source) {
                facts.insert_unit("rust", "function", name);
                facts.insert_span(
                    path,
                    node.start_position().row + 1,
                    node.end_position().row + 1,
                );
            }
            unit
        }
        "field_declaration" => {
            if let (Some(parent_unit), Some(name)) = (unit, field_name(node, "name", source)) {
                facts.insert_field("rust", parent_unit, name);
            }
            unit
        }
        "call_expression" => {
            if let (Some(parent_unit), Some(parent_method), Some(callee)) =
                (unit, method, call_name(node, source))
            {
                facts.insert_call("rust", parent_unit, parent_method, callee);
            }
            unit
        }
        "macro_invocation" => {
            if let Some(name) = field_name(node, "macro", source) {
                facts.unsupported.insert(format!("rust:item-macros:{name}"));
            }
            unit
        }
        _ => unit,
    };
    let current_method = if node.kind() == "function_item" {
        field_name(node, "name", source).or(method)
    } else {
        method
    };
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_rust(child, source, path, current_unit, current_method, facts);
    }
}

fn walk_python(
    node: Node<'_>,
    source: &str,
    path: &str,
    unit: Option<&str>,
    method: Option<&str>,
    facts: &mut Facts,
) {
    let current_unit = match node.kind() {
        "class_definition" => {
            if let Some(name) = field_name(node, "name", source) {
                facts.insert_unit("python", "class", name);
                facts.insert_span(
                    path,
                    node.start_position().row + 1,
                    node.end_position().row + 1,
                );
                Some(name)
            } else {
                unit
            }
        }
        "function_definition" => {
            if let Some(parent_unit) = unit {
                if let Some(name) = field_name(node, "name", source) {
                    facts.insert_method("python", parent_unit, name);
                }
                unit
            } else if let Some(name) = field_name(node, "name", source) {
                facts.insert_unit("python", "function", name);
                facts.insert_span(
                    path,
                    node.start_position().row + 1,
                    node.end_position().row + 1,
                );
                unit
            } else {
                unit
            }
        }
        "call" => {
            if let (Some(parent_unit), Some(parent_method), Some(callee)) =
                (unit, method, call_name(node, source))
            {
                facts.insert_call("python", parent_unit, parent_method, callee);
            }
            unit
        }
        _ => unit,
    };
    let current_method = if node.kind() == "function_definition" {
        field_name(node, "name", source).or(method)
    } else {
        method
    };
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_python(child, source, path, current_unit, current_method, facts);
    }
}

fn call_name<'a>(node: Node<'a>, source: &'a str) -> Option<&'a str> {
    let function = node.child_by_field_name("function")?;
    match function.kind() {
        "identifier" | "field_identifier" => function.utf8_text(source.as_bytes()).ok(),
        "field_expression" | "attribute" => function
            .child_by_field_name("field")
            .or_else(|| function.child_by_field_name("attribute"))
            .and_then(|child| child.utf8_text(source.as_bytes()).ok()),
        "scoped_identifier" | "qualified_identifier" => text(function, source).rsplit("::").next(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("temp project");
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(
            dir.path().join("src/lib.rs"),
            "pub struct Widget { pub name: String, size: u32 }\n\
             impl Widget {\n\
                 pub fn new(name: String) -> Self { Self { name, size: 1 } }\n\
                 pub fn area(&self) -> u32 { self.size }\n\
                 fn helper(&self) { self.area(); }\n\
             }\n\
             pub trait Draw { fn draw(&self); }\n\
             impl Draw for Widget { fn draw(&self) { self.helper(); } }\n\
             pub fn main() { let w = Widget::new(\"x\".into()); w.area(); w.draw(); }\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("src/app.py"),
            "class Widget:\n\
                 def __init__(self, name):\n\
                     self.name = name\n\
                 def area(self):\n\
                     return self._size()\n\
                 def _size(self):\n\
                     return 1\n\
             def make():\n\
                 return Widget('x')\n",
        )
        .unwrap();
        dir
    }

    #[test]
    fn evaluation_records_every_required_category_and_a_retain_decision() {
        let dir = fixture();
        let evaluation = evaluate(dir.path(), &[std::path::PathBuf::from("src")]).unwrap();
        let rendered = evaluation.render();
        println!("{rendered}");
        for category in [
            "units",
            "methods",
            "fields",
            "calls",
            "spans",
            "unsupported",
        ] {
            assert!(
                evaluation.rows.iter().any(|row| row.category == category),
                "missing evaluation category {category}:\n{rendered}"
            );
        }
        assert!(
            evaluation
                .rows
                .iter()
                .any(|row| row.classification == "missing_required"),
            "the minimal adapter should record missing required facts, not claim parity:\n{rendered}"
        );
        assert_eq!(evaluation.decision(), "RETAIN", "{rendered}");
        assert!(
            rendered.contains("Tree-sitter outline evaluation"),
            "{rendered}"
        );
    }

    #[test]
    fn the_evaluation_decision_record_matches_the_test_matrix() {
        let document = include_str!("../../../docs/outline-tree-sitter-evaluation.md");
        assert!(document.contains("Decision: RETAIN"), "{document}");
        for category in ["units", "methods", "fields", "calls", "spans"] {
            assert!(
                document.contains(category),
                "decision record omits {category}"
            );
        }
        assert!(document.contains("0.9.14"), "{document}");
        assert!(document.contains("test-only dev dependency"), "{document}");
    }

    #[test]
    fn malformed_tree_sitter_input_is_reported_as_an_error() {
        let mut facts = Facts::default();
        extract_rust("src/broken.rs", "fn broken( {", &mut facts).unwrap();
        assert!(
            facts.errors.iter().any(|path| path == "src/broken.rs"),
            "malformed input was not recorded as an error: {facts:?}"
        );
    }

    #[test]
    fn the_default_analyzer_keeps_the_current_languages() {
        let capabilities = super::super::analyzer::outline_analyzer().capabilities();
        assert_eq!(
            capabilities.languages,
            vec![Language::Rust, Language::Python]
        );
        assert!(
            capabilities
                .supported_constructs
                .contains(&Construct::Struct)
        );
        assert!(
            capabilities
                .supported_constructs
                .contains(&Construct::Class)
        );
    }
}
