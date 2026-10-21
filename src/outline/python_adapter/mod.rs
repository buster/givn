//! Python source adapter backed by the standard-library `ast` parser.
//!
//! The interpreter is started only after at least one `.py` file is discovered.
//! A Rust-only project never invokes `python3`.

use super::analyzer::SourceAnalyzer;
use super::model::{
    Analysis, AnalysisDiagnostic, AnalysisRequest, AnalyzedFile, AnalyzerCapabilities, Call,
    Construct, DiagnosticKind, Field, FileStatus, Flow, Language, Limitation, Method, OutlineItem,
    OutlineKind, RepoPath, Resolution, Signature, SourceSpan, SourceUnit, TypeKind, TypeRelation,
    UnitId, UnitKind, UnitOrigin, Visibility,
};
use super::scan;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

const EXTRACTOR: &str = include_str!("extractor.py");

pub struct PythonAnalyzer;

impl SourceAnalyzer for PythonAnalyzer {
    fn capabilities(&self) -> AnalyzerCapabilities {
        AnalyzerCapabilities {
            languages: vec![Language::Python],
            supported_constructs: vec![
                Construct::Class,
                Construct::Function,
                Construct::FileModule,
            ],
            limitations: vec![
                Limitation::PythonDynamicCalls,
                Limitation::PythonTypeInference,
                Limitation::NonUtf8File,
            ],
            resolution_labels: vec![Resolution::SameFile, Resolution::External],
        }
    }

    fn analyze(&self, request: &AnalysisRequest) -> Result<Analysis> {
        if !request.scope.languages.contains(&Language::Python) {
            return Ok(Analysis::default());
        }
        let sources = scan::discover(
            &request.project_root,
            &request.scope.roots,
            &[Language::Python],
        )?;
        if sources.is_empty() {
            return Ok(Analysis::default());
        }
        analyze_sources(&sources)
    }
}

fn analyze_sources(sources: &[scan::ScannedSource]) -> Result<Analysis> {
    let input = json!({
        "files": sources
            .iter()
            .map(|source| json!({
                "path": source.path.as_str(),
                "content": source.content,
            }))
            .collect::<Vec<_>>(),
    });
    let output = (|| -> std::io::Result<std::process::Output> {
        let mut child = Command::new("python3")
            .args(["-I", "-S", "-c", EXTRACTOR])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.to_string().as_bytes())?;
        }
        child.wait_with_output()
    })();
    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let detail = stderr
                .lines()
                .rev()
                .find(|line| !line.trim().is_empty())
                .map(str::trim)
                .unwrap_or("python3 failed to produce an outline");
            return Ok(failed_analysis(
                sources,
                format!("python3 failed: {detail}"),
            ));
        }
        Err(error) => {
            return Ok(failed_analysis(
                sources,
                format!("python3 is not available; install Python 3.10+ ({error})"),
            ));
        }
    };
    let document: Value = match serde_json::from_slice(&output.stdout) {
        Ok(document) => document,
        Err(error) => {
            return Ok(failed_analysis(
                sources,
                format!("python3 returned invalid JSON: {error}"),
            ));
        }
    };
    if let Some(message) = document["error"].as_str() {
        return Ok(failed_analysis(sources, message.to_string()));
    }
    let results = document["files"]
        .as_array()
        .context("python3 output has no files array")?;
    let by_path: std::collections::BTreeMap<&str, &Value> = results
        .iter()
        .filter_map(|result| Some((result["path"].as_str()?, result)))
        .collect();

    let mut analysis = Analysis::default();
    for source in sources {
        let Some(result) = by_path.get(source.path.as_str()).copied() else {
            analysis.diagnostics.push(AnalysisDiagnostic {
                kind: DiagnosticKind::AnalyzerUnavailable,
                path: source.path.clone(),
                line: None,
                message: "python3 returned no result for this file".to_string(),
            });
            analysis.files.push(AnalyzedFile {
                path: source.path.clone(),
                language: Language::Python,
                physical_lines: source.content.lines().count() as u32,
                lines: source.lines.clone(),
                status: FileStatus::Failed,
                unit_ids: Vec::new(),
            });
            continue;
        };
        if result["status"].as_str() == Some("failed") {
            analysis.diagnostics.push(AnalysisDiagnostic {
                kind: DiagnosticKind::ParseFailed,
                path: source.path.clone(),
                line: result["line"].as_u64().map(|line| line as u32),
                message: result["message"]
                    .as_str()
                    .unwrap_or("python parse failed")
                    .to_string(),
            });
            analysis.files.push(AnalyzedFile {
                path: source.path.clone(),
                language: Language::Python,
                physical_lines: source.content.lines().count() as u32,
                lines: source.lines.clone(),
                status: FileStatus::Failed,
                unit_ids: Vec::new(),
            });
            continue;
        }

        let comment_lines: std::collections::BTreeSet<u32> = result["comment_lines"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(result["docstring_lines"].as_array().into_iter().flatten())
            .filter_map(|line| line.as_u64().map(|line| line as u32))
            .collect();
        let lines: Vec<_> = source
            .content
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let number = index as u32 + 1;
                if line.trim().is_empty() {
                    crate::outline::model::LineKind::Blank
                } else if comment_lines.contains(&number) {
                    crate::outline::model::LineKind::Comment
                } else {
                    crate::outline::model::LineKind::Code
                }
            })
            .collect();
        let module_id = UnitId(format!("module:{}", source.path));
        let mut unit_ids = Vec::new();
        for unit in result["units"].as_array().into_iter().flatten() {
            let Some(id) = unit["id"].as_str() else {
                continue;
            };
            let kind = match unit["kind"].as_str() {
                Some("type") => UnitKind::Type(TypeKind::Class),
                Some("module") => UnitKind::Module,
                _ => continue,
            };
            let methods = methods_from(unit, source.path.clone());
            let fields = fields_from(unit, source.path.clone());
            let outline = methods
                .iter()
                .map(|method| OutlineItem {
                    kind: OutlineKind::Method,
                    name: method.name.clone(),
                    visibility: method.visibility,
                    signature: method.signature.text.clone(),
                    order: method.span.start_line,
                    calls: method.calls.clone(),
                })
                .collect();
            let relations = if matches!(kind, UnitKind::Module) {
                Vec::new()
            } else {
                vec![TypeRelation::ContainedIn(module_id.clone())]
            };
            unit_ids.push(UnitId(id.to_string()));
            analysis.units.push(SourceUnit {
                id: UnitId(id.to_string()),
                language: Language::Python,
                kind,
                name: unit["name"].as_str().unwrap_or(id).to_string(),
                module_path: python_module_path(source.path.as_str()),
                declaration_file: source.path.clone(),
                files: vec![source.path.clone()],
                spans: vec![SourceSpan {
                    file: source.path.clone(),
                    start_line: line(unit, "start_line"),
                    end_line: line(unit, "end_line"),
                }],
                visibility: visibility(unit["visibility"].as_str()),
                origin: UnitOrigin::Production,
                methods,
                fields,
                relations,
                outline,
                complete: true,
                unsupported: Vec::new(),
            });
        }
        analysis.files.push(AnalyzedFile {
            path: source.path.clone(),
            language: Language::Python,
            physical_lines: source.content.lines().count() as u32,
            lines,
            status: FileStatus::Analyzed,
            unit_ids,
        });
    }
    analysis.units.sort_by(|left, right| left.id.cmp(&right.id));
    analysis.diagnostics.sort_by(|left, right| {
        (left.path.clone(), left.line).cmp(&(right.path.clone(), right.line))
    });
    Ok(analysis)
}

fn methods_from(unit: &Value, path: RepoPath) -> Vec<Method> {
    let unit_id = unit["id"].as_str().unwrap_or_default();
    let internal: std::collections::BTreeSet<&str> = unit["methods"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|method| method["name"].as_str())
        .chain(
            unit["file_functions"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|name| name.as_str()),
        )
        .collect();
    unit["methods"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|method| {
            let name = method["name"].as_str()?.to_string();
            let calls: Vec<Call> = method["calls"]
                .as_array()
                .into_iter()
                .flatten()
                .enumerate()
                .filter_map(|(order, call)| {
                    let callee = call.as_str()?.to_string();
                    let resolution = if internal.contains(callee.as_str()) {
                        Resolution::SameFile
                    } else {
                        Resolution::External
                    };
                    Some(Call {
                        order: order as u32,
                        callee,
                        resolution,
                        target: None,
                    })
                })
                .collect();
            Some(Method {
                id: format!("{unit_id}::{name}"),
                name,
                visibility: visibility(method["visibility"].as_str()),
                signature: Signature {
                    text: method["signature"].as_str().unwrap_or_default().to_string(),
                },
                span: SourceSpan {
                    file: path.clone(),
                    start_line: line(method, "start_line"),
                    end_line: line(method, "end_line"),
                },
                flow: Flow::Block(Vec::new()),
                field_accesses: method["field_accesses"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|field| field.as_str().map(ToString::to_string))
                    .collect(),
                calls,
                is_trait_default: false,
                origin: UnitOrigin::Production,
            })
        })
        .collect()
}

fn fields_from(unit: &Value, path: RepoPath) -> Vec<Field> {
    unit["fields"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|field| {
            let name = field["name"].as_str()?.to_string();
            Some(Field {
                id: name.clone(),
                name,
                visibility: visibility(field["visibility"].as_str()),
                span: SourceSpan {
                    file: path.clone(),
                    start_line: line(field, "line"),
                    end_line: line(field, "line"),
                },
                type_label: field["type"].as_str().unwrap_or_default().to_string(),
                origin: UnitOrigin::Production,
            })
        })
        .collect()
}

fn failed_analysis(sources: &[scan::ScannedSource], message: String) -> Analysis {
    let mut analysis = Analysis::default();
    for source in sources {
        analysis.files.push(AnalyzedFile {
            path: source.path.clone(),
            language: Language::Python,
            physical_lines: source.content.lines().count() as u32,
            lines: source.lines.clone(),
            status: FileStatus::Failed,
            unit_ids: Vec::new(),
        });
        analysis.diagnostics.push(AnalysisDiagnostic {
            kind: DiagnosticKind::AnalyzerUnavailable,
            path: source.path.clone(),
            line: None,
            message: message.clone(),
        });
    }
    analysis
}

fn line(value: &Value, key: &str) -> u32 {
    value[key]
        .as_u64()
        .map(|line| line.max(1) as u32)
        .unwrap_or(1)
}

fn visibility(value: Option<&str>) -> Visibility {
    match value {
        Some("public") => Visibility::Public,
        Some("private") => Visibility::Private,
        Some("crate") => Visibility::Crate,
        Some("restricted") => Visibility::Restricted,
        _ => Visibility::Private,
    }
}

fn python_module_path(path: &str) -> String {
    path.trim_end_matches(".py").replace('/', ".")
}
