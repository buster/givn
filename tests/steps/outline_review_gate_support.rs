//! Fixture builders for the outline review gate scenarios.
//!
//! Every fixture is a real temporary Git or Fossil repository: a production
//! base commit, a change-directory commit (the gate's first touch), and head
//! commits that add, grow, shrink, move, delete, or dirty production sources.
//! The helpers here contain no step bindings; they write files, commit, and
//! call the public gate API.

use crate::common::{self, BackendKind, TestEnv};
use crate::steps::archive_proof_support;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

pub(crate) const CHANGE_ID: &str = "test-change";

const METHODS: usize = 8;

/// One committed fixture repository plus its change id.
pub(crate) struct GateFixture<'a> {
    pub(crate) env: &'a TestEnv,
    pub(crate) change_id: &'a str,
}

impl<'a> GateFixture<'a> {
    pub(crate) fn new(env: &'a TestEnv, change_id: &'a str) -> Self {
        Self { env, change_id }
    }

    pub(crate) fn write(&self, relative: &str, content: &str) {
        write_file(self.env, relative, content);
    }

    pub(crate) fn change_path(&self, relative: &str) -> String {
        format!("givn/changes/{}/{relative}", self.change_id)
    }

    /// Commit everything with the fixture identity on the selected backend.
    pub(crate) fn commit(&self, message: &str) {
        commit(self.env, message);
    }

    /// Write the minimal change directory and make it the first touch.
    pub(crate) fn change_dir_commit(&self) {
        let change = format!("givn/changes/{}", self.change_id);
        self.write(
            &format!("{change}/proposal.md"),
            "# Proposal: test change\n",
        );
        self.write(&format!("{change}/design.md"), "# Design: test change\n");
        self.write(&format!("{change}/tasks.md"), "# Tasks: test change\n");
        self.commit("docs(test-change): add change directory");
    }

    /// The base has one 620-code-line type split across the declaration file
    /// and its parts file; both modules stay under the readable size.
    pub(crate) fn base_god_type_split(&self, total: u32, declaration: &str) {
        let (declaration_source, parts_source) = type_split("Engine", total, &["a", "b", "c"]);
        self.write(declaration, &declaration_source);
        self.write(&parts_path(declaration), &parts_source);
        self.commit("chore(test-change): base god type");
    }

    /// The base has production sources, none of them over the readable size.
    pub(crate) fn base_without_large_units(&self) {
        self.write(
            "src/base.rs",
            "pub struct Base {\n    value: u64,\n}\n\nimpl Base {\n    pub fn value(&self) -> u64 { self.value }\n}\n",
        );
        self.commit("chore(test-change): base sources");
    }

    pub(crate) fn base_data_only_enum(&self, total: u32, path: &str) {
        self.write(path, &data_only_enum("Status", total));
        self.commit("chore(test-change): base status vocabulary");
    }

    pub(crate) fn base_small_type_at(&self, path: &str) {
        self.write(
            path,
            "pub struct Engine {\n    value: u64,\n}\n\nimpl Engine {\n    pub fn value(&self) -> u64 { self.value }\n}\n",
        );
        self.commit("chore(test-change): base type");
    }

    /// Add a split type at head and commit.
    pub(crate) fn add_split_type(
        &self,
        name: &str,
        total: u32,
        declaration: &str,
        fields: &[&str],
    ) {
        let (declaration_source, parts_source) = type_split(name, total, fields);
        self.write(declaration, &declaration_source);
        self.write(&parts_path_from(declaration), &parts_source);
        self.commit(&format!("feat(test-change): add {name}"));
    }

    /// Add explicit declaration and parts paths for one type at head.
    pub(crate) fn add_split_type_at(&self, name: &str, total: u32, declaration: &str, parts: &str) {
        let (declaration_source, parts_source) = type_split(name, total, &["a", "b", "c"]);
        self.write(declaration, &declaration_source);
        self.write(parts, &parts_source);
        self.commit(&format!("feat(test-change): add {name}"));
    }

    /// Add a type that lives entirely in one file at head.
    pub(crate) fn add_one_file_type(&self, name: &str, total: u32, path: &str, fields: &[&str]) {
        self.write(path, &type_one_file(name, total, fields));
        self.commit(&format!("feat(test-change): add {name}"));
    }

    /// Add a cohesive type (one field, all methods touch it) at head.
    pub(crate) fn add_cohesive_type(&self, name: &str, total: u32, declaration: &str) {
        let (declaration_source, parts_source) = type_split(name, total, &["value"]);
        self.write(declaration, &declaration_source);
        self.write(&parts_path_from(declaration), &parts_source);
        self.commit(&format!("feat(test-change): add {name}"));
    }

    /// Add a module of exactly `total` code lines in one file at head.
    pub(crate) fn add_module(&self, total: u32, path: &str) {
        self.write(path, &module_file(total));
        self.commit("feat(test-change): add the module");
    }

    /// Add two types to one file at head.
    pub(crate) fn add_two_types_one_file(&self, total_each: u32, path: &str) {
        let first = type_one_file("First", total_each, &["a", "b", "c"]);
        let second = type_one_file("Second", total_each, &["a", "b", "c"]);
        self.write(path, &format!("{first}\n{second}"));
        self.commit("feat(test-change): add two types");
    }

    /// Add a 420-code-line type with 51 public methods at head.
    pub(crate) fn add_interface(&self, name: &str, total: u32, methods: u32, path: &str) {
        self.write(path, &type_with_public_methods(name, total, methods));
        self.commit(&format!("feat(test-change): add {name}"));
    }

    /// Rewrite the base type at the requested size.
    pub(crate) fn resize_type(&self, total: u32, declaration: &str) {
        let (declaration_source, parts_source) = type_split("Engine", total, &["a", "b", "c"]);
        self.write(declaration, &declaration_source);
        self.write(&parts_path(declaration), &parts_source);
        self.commit("feat(test-change): resize the type");
    }

    /// Append a method with exactly `code_lines` code lines to a type.
    pub(crate) fn add_long_method(&self, path: &str, type_name: &str, code_lines: u32) {
        let mut source = fs::read_to_string(self.env.path(path)).expect("read type source");
        source.push_str(&long_method_impl(type_name, code_lines));
        self.write(path, &source);
        self.commit("feat(test-change): add the long method");
    }

    /// Change a production file without changing its code-line count.
    pub(crate) fn touch_without_growing(&self, path: &str) {
        let mut source = fs::read_to_string(self.env.path(path)).expect("read source");
        source.push_str("// touched by the change\n");
        self.write(path, &source);
        self.commit("feat(test-change): touch the type");
    }

    pub(crate) fn replace_with_malformed(&self, path: &str) {
        self.write(path, "pub struct Engine {\n");
        self.commit("feat(test-change): malformed source");
    }

    pub(crate) fn add_excluded_test_file(&self, path: &str) {
        self.write(path, "pub fn helper() -> u64 { 1 }\n");
        self.commit("feat(test-change): add an analyzer-excluded file");
    }

    pub(crate) fn move_type(&self, from_declaration: &str, to_declaration: &str, to_parts: &str) {
        let declaration = fs::read(self.env.path(from_declaration)).expect("read declaration");
        let parts = fs::read(self.env.path(&parts_path(from_declaration))).expect("read parts");
        fs::remove_file(self.env.path(from_declaration)).expect("remove declaration");
        fs::remove_file(self.env.path(&parts_path(from_declaration))).expect("remove parts");
        self.write(
            to_declaration,
            &String::from_utf8(declaration).expect("utf8"),
        );
        self.write(to_parts, &String::from_utf8(parts).expect("utf8"));
        self.commit("feat(test-change): move the type");
    }

    pub(crate) fn delete_paths(&self, paths: &[&str]) {
        for path in paths {
            let absolute = self.env.path(path);
            if absolute.exists() {
                fs::remove_file(absolute).expect("remove production file");
            }
        }
        self.commit("feat(test-change): delete the source");
    }

    #[cfg(unix)]
    pub(crate) fn replace_with_symlink(&self, path: &str) {
        let absolute = self.env.path(path);
        fs::remove_file(&absolute).expect("remove source");
        let parent = absolute.parent().expect("source parent");
        fs::create_dir_all(parent).expect("create source parent");
        std::os::unix::fs::symlink("parts.rs", &absolute).expect("create symlink");
        self.commit("feat(test-change): replace the source with a symlink");
    }

    /// Append one marker line to the change's review record.
    pub(crate) fn record_review_line(&self, line: &str) {
        let relative = self.change_path("review.md");
        let absolute = self.env.path(&relative);
        let mut content = fs::read_to_string(&absolute).unwrap_or_else(|_| {
            if let Some(parent) = absolute.parent() {
                fs::create_dir_all(parent).expect("create change directory");
            }
            String::new()
        });
        if content.is_empty() {
            content.push_str("# Review\n\n");
        }
        content.push_str(line);
        content.push('\n');
        self.write(&relative, &content);
    }

    pub(crate) fn write_rationale(&self, finding_unit: &str, spec: &RationaleSpec) {
        let relative = self.change_path(&format!(
            "outline/rationales/{}.md",
            rationale_slug(finding_unit)
        ));
        self.write(&relative, &spec.render());
    }
}

/// One rationale artifact. Every field the gate checks is explicit so a
/// scenario can break exactly one of them.
#[derive(Clone, Debug)]
pub(crate) struct RationaleSpec {
    pub(crate) unit: String,
    pub(crate) classification: String,
    pub(crate) finding: String,
    pub(crate) evidence: String,
    pub(crate) why: String,
    pub(crate) alternatives: String,
    pub(crate) owner: Option<String>,
    pub(crate) author: Option<String>,
    pub(crate) reviewed_by: Option<String>,
    pub(crate) review_verdict: Option<String>,
    pub(crate) follow_up: Option<String>,
    pub(crate) expiry: Option<String>,
}

impl RationaleSpec {
    /// A complete rationale that passes the mechanical floor for `unit`.
    pub(crate) fn complete(unit: &str, classification: &str, finding: &str) -> Self {
        let name = unit
            .rsplit("::")
            .next()
            .filter(|candidate| *candidate != unit)
            .unwrap_or_else(|| unit.rsplit('/').next().unwrap_or(unit))
            .trim_end_matches(".rs")
            .to_string();
        Self {
            unit: unit.to_string(),
            classification: classification.to_string(),
            finding: finding.to_string(),
            evidence:
                "620 code lines, 8 public methods; every call site is in the change's production root."
                    .to_string(),
            why: format!(
                "The {name} unit owns its state machine and phase order, so splitting it would move the transition rules into two modules."
            ),
            alternatives:
                "Extract the renderer into a separate module. Rejected: rendering already delegates to the report module, and the extraction would add indirection without removing a responsibility."
                    .to_string(),
            owner: Some("givn-maintainer--owner".to_string()),
            author: Some("agent".to_string()),
            reviewed_by: Some("fresh-context-reviewer".to_string()),
            review_verdict: Some("accept".to_string()),
            follow_up: None,
            expiry: None,
        }
    }

    pub(crate) fn render(&self) -> String {
        let mut text = String::from("# Rationale\n\n");
        text.push_str(&format!("Unit: {}\n", self.unit));
        text.push_str(&format!("Classification: {}\n", self.classification));
        text.push_str(&format!("Finding: {}\n", self.finding));
        text.push_str(&format!("Evidence: {}\n", self.evidence));
        text.push_str(&format!("Why this shape is right: {}\n", self.why));
        text.push_str(&format!("Alternatives considered: {}\n", self.alternatives));
        if let Some(owner) = &self.owner {
            text.push_str(&format!("Owner: {owner}\n"));
        }
        if let Some(author) = &self.author {
            text.push_str(&format!("Author: {author}\n"));
        }
        if let Some(reviewed_by) = &self.reviewed_by {
            text.push_str(&format!("Reviewed-by: {reviewed_by}\n"));
        }
        if let Some(verdict) = &self.review_verdict {
            text.push_str(&format!("Review verdict: {verdict}\n"));
        }
        if let Some(follow_up) = &self.follow_up {
            text.push_str(&format!("Follow-up: {follow_up}\n"));
        }
        if let Some(expiry) = &self.expiry {
            text.push_str(&format!("Expiry: {expiry}\n"));
        }
        text
    }
}

/// The gate's slug rule, mirrored for fixture paths.
pub(crate) fn rationale_slug(unit: &str) -> String {
    let mut slug = String::with_capacity(unit.len());
    let mut separator = false;
    for character in unit.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            separator = false;
        } else if !separator {
            slug.push('-');
            separator = true;
        }
    }
    slug.trim_matches('-').to_string()
}

pub(crate) fn parts_path(declaration: &str) -> String {
    parts_path_from(declaration)
}

fn parts_path_from(declaration: &str) -> String {
    let stem = declaration.strip_suffix(".rs").unwrap_or(declaration);
    format!("{stem}/parts.rs")
}

pub(crate) fn write_file(env: &TestEnv, relative: &str, content: &str) {
    let path = env.path(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create fixture parent");
    }
    fs::write(path, content).expect("write fixture file");
}

pub(crate) fn commit(env: &TestEnv, message: &str) {
    match env.backend() {
        BackendKind::Git => common::git_commit_all(env, message),
        BackendKind::Fossil => common::fossil_commit_all(env, message),
    }
}

/// Commit with a subject and a body, so the scenario-commit check can match
/// the scenario title in the body.
pub(crate) fn commit_with_body(env: &TestEnv, subject: &str, body: &str) {
    match env.backend() {
        BackendKind::Git => {
            let add = common::git(env, &["add", "-A"]);
            assert!(add.status.success(), "git add failed");
            let output = git(env, &["commit", "-q", "-m", subject, "-m", body]);
            assert!(output.status.success(), "git commit failed");
        }
        BackendKind::Fossil => {
            common::fossil_commit_all(env, &format!("{subject}\n\n{body}"));
        }
    }
}

/// The committed head revision of the fixture.
pub(crate) fn revision(env: &TestEnv) -> String {
    match env.backend() {
        BackendKind::Git => {
            String::from_utf8_lossy(&common::git(env, &["rev-parse", "HEAD"]).stdout)
                .trim()
                .to_string()
        }
        BackendKind::Fossil => fossil_current_hash(env),
    }
}

pub(crate) fn git(env: &TestEnv, args: &[&str]) -> Output {
    let output = common::git(env, args);
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

pub(crate) fn git_stdout(env: &TestEnv, args: &[&str]) -> String {
    String::from_utf8_lossy(&git(env, args).stdout)
        .trim()
        .to_string()
}

pub(crate) fn fossil(env: &TestEnv, args: &[&str]) -> Output {
    let output = common::fossil(env, args);
    assert!(
        output.status.success(),
        "fossil {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

pub(crate) fn fossil_stdout(env: &TestEnv, args: &[&str]) -> String {
    String::from_utf8_lossy(&fossil(env, args).stdout)
        .trim()
        .to_string()
}

/// The 40-character hash of the current Fossil check-in.
pub(crate) fn fossil_current_hash(env: &TestEnv) -> String {
    let info = fossil_stdout(env, &["info", "current"]);
    info.lines()
        .find_map(|line| line.trim().strip_prefix("hash:"))
        .and_then(|rest| rest.split_whitespace().next())
        .expect("fossil current hash")
        .to_string()
}

/// A `git clone --depth 1` of the fixture, kept alive by the caller's
/// `TempDir`.
pub(crate) fn shallow_clone(env: &TestEnv) -> tempfile::TempDir {
    let clone = tempfile::tempdir().expect("shallow clone parent");
    let url = format!("file://{}", env.root().display());
    let output = Command::new("git")
        .args(["clone", "-q", "--depth", "1", &url])
        .arg(clone.path())
        .output()
        .expect("run git clone");
    assert!(
        output.status.success(),
        "shallow clone failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    clone
}

/// Rebase the current branch onto `upstream`, keeping the replayed commit's
/// side of any conflict (`-X theirs` during a rebase).
pub(crate) fn git_rebase(env: &TestEnv, upstream: &str) -> Output {
    Command::new("git")
        .args(["rebase", "-X", "theirs", upstream])
        .current_dir(env.root())
        .env("GIT_EDITOR", "true")
        .env("GIT_SEQUENCE_EDITOR", "true")
        .output()
        .expect("run git rebase")
}

/// Merge a Fossil branch into the current checkout. A merge with conflicting
/// files exits non-zero but leaves the merge state for the caller to resolve.
pub(crate) fn fossil_merge(env: &TestEnv, branch: &str) -> Output {
    Command::new("fossil")
        .args(["merge", branch])
        .current_dir(env.root())
        .env("FOSSIL_USER", "fixture")
        .env("USER", "fixture")
        .output()
        .expect("run fossil merge")
}

/// Snapshot every project file except the version-control backend metadata.
pub(crate) fn snapshot(root: &Path) -> Vec<(String, Vec<u8>)> {
    let mut entries = Vec::new();
    collect_files(root, root, &mut entries);
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

fn collect_files(root: &Path, directory: &Path, output: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if matches!(name.as_str(), ".git" | ".fslckout" | "_FOSSIL_") {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, output);
        } else if let Ok(bytes) = fs::read(&path) {
            output.push((
                path.strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/"),
                bytes,
            ));
        }
    }
}

// --- Production source generators -----------------------------------------

/// A type split across its declaration file and one parts file. The sixteen
/// overhead lines (struct, four impl braces, three lines per method) are
/// chosen so the measured code lines equal `total` exactly.
pub(crate) fn type_split(name: &str, total: u32, fields: &[&str]) -> (String, String) {
    assert!(
        !fields.is_empty(),
        "a fixture type needs at least one field"
    );
    let overhead = (fields.len() as u32 + 2) + 4 + (3 * METHODS as u32);
    let body = total.saturating_sub(overhead) / METHODS as u32;
    let remainder = total.saturating_sub(overhead) % METHODS as u32;
    let methods: Vec<String> = (0..METHODS)
        .map(|index| {
            let lines = body + u32::from((index as u32) < remainder);
            method(
                &format!("method_{index}"),
                fields[index % fields.len()],
                lines as usize,
            )
        })
        .collect();
    let struct_fields: String = fields
        .iter()
        .map(|field| format!("    {field}: u64,\n"))
        .collect();
    let declaration = format!(
        "pub struct {name} {{\n{struct_fields}}}\n\nimpl {name} {{\n{}{}{}{}\n}}\n",
        methods[0], methods[1], methods[2], methods[3]
    );
    let parts = format!(
        "impl {name} {{\n{}{}{}{}\n}}\n",
        methods[4], methods[5], methods[6], methods[7]
    );
    (declaration, parts)
}

/// A type with all eight implementations in its declaration file.
pub(crate) fn type_one_file(name: &str, total: u32, fields: &[&str]) -> String {
    let (mut declaration, parts) = type_split(name, total, fields);
    let insertion = declaration
        .rfind("\n}\n")
        .expect("the declaration ends with the impl close");
    let extra = parts
        .lines()
        .filter(|line| !line.starts_with("impl ") && *line != "}")
        .map(|line| format!("{line}\n"))
        .collect::<String>();
    declaration.insert_str(insertion + 1, &extra);
    declaration
}

fn method(name: &str, field: &str, body_lines: usize) -> String {
    let mut source = format!("    pub fn {name}(&self) -> u64 {{\n");
    for _ in 0..body_lines {
        source.push_str(&format!("        let _ = self.{field};\n"));
    }
    source.push_str(&format!("        self.{field}\n    }}\n"));
    source
}

/// A `code_lines`-long method; the signature, the return line, and the close
/// brace count with the body.
pub(crate) fn long_method_impl(type_name: &str, code_lines: u32) -> String {
    let body = code_lines.saturating_sub(3) as usize;
    format!(
        "\nimpl {type_name} {{\n    pub fn long(&self) -> u64 {{\n{}        self.a\n    }}\n}}\n",
        "        let _ = self.a;\n".repeat(body)
    )
}

/// A data-only enum of exactly `total` code lines.
pub(crate) fn data_only_enum(name: &str, total: u32) -> String {
    let variants = total.saturating_sub(2);
    let mut source = format!("pub enum {name} {{\n");
    for index in 0..variants {
        source.push_str(&format!("    Variant{index},\n"));
    }
    source.push_str("}\n");
    source
}

/// A module of exactly `total` code lines: one one-line function per line.
pub(crate) fn module_file(total: u32) -> String {
    let mut source = String::new();
    for index in 0..total {
        source.push_str(&format!("pub fn function_{index}() -> u64 {{ {index} }}\n"));
    }
    source
}

/// A type of exactly `total` code lines carrying `methods` public methods.
pub(crate) fn type_with_public_methods(name: &str, total: u32, methods: u32) -> String {
    let field_count = total.saturating_sub(2) - methods - 2;
    let mut source = format!("pub struct {name} {{\n");
    for index in 0..field_count {
        source.push_str(&format!("    field_{index}: u64,\n"));
    }
    source.push_str("}\n\n");
    source.push_str(&format!("impl {name} {{\n"));
    for index in 0..methods {
        source.push_str(&format!(
            "    pub fn method_{index}(&self) -> u64 {{ {index} }}\n"
        ));
    }
    source.push_str("}\n");
    source
}

// --- Review-ready fixture shared by the CLI scenarios ----------------------

/// The task row and commit body used by the review-ready fixture. It is a
/// real scenario title so the scenario-commit check has something to match.
pub(crate) const READY_TASK_TITLE: &str = "The fixture change is complete";

/// Write the artifact preconditions a review-ready change needs: a delta
/// feature, its step file and registration, a design with a Step Definitions
/// table, a proposal with a routing decision, a permanent use case, and the
/// verification runner. The caller commits the tree.
pub(crate) fn write_review_preconditions(env: &TestEnv, change_id: &str, task_commit: &str) {
    archive_proof_support::disable_coverage(env);
    archive_proof_support::write_result_runner(env, "pass");
    archive_proof_support::configure_commands(env);

    let change = format!("givn/changes/{change_id}");
    write_file(
        env,
        &format!("{change}/proposal.md"),
        "# Proposal: test change\n\n\
         ## Capability Routing\n\n\
         | Proposed capability | Route's recommendation | Decision | Rationale |\n\
         |---|---|---|---|\n\
         | fixture-capability | (fixture) | `NEW in verify-change` | fixture |\n",
    );
    write_file(
        env,
        &format!("{change}/design.md"),
        "# Design: test change\n\n\
         ## Step Definitions\n\n\
         | Capability | Step definition file |\n\
         |---|---|\n\
         | fixture-capability | tests/steps/outline_review_gate_fixture_steps.rs |\n\n\
         ## Visual Design Contract\n\n\
         ```text\n\
         - Interface: none\n\
         - Viewports: not applicable — the capability renders no screen\n\
         - Design system: docs/design/design-system.md (unchanged)\n\
         ```\n",
    );
    write_file(
        env,
        &format!("{change}/specs/verify-change/fixture-capability.feature"),
        "@givn.delta @fixture-capability\n\
         Feature: Fixture capability\n\n\
         \x20 @givn.added\n\
         \x20 Scenario: Fixture behavior\n\
         \x20   Given a fixture precondition\n\
         \x20   Then the fixture result\n",
    );
    write_file(
        env,
        "tests/steps/outline_review_gate_fixture_steps.rs",
        "//! Fixture step file promised by the review-ready change's design.\n",
    );
    let mod_path = env.path("tests/steps/mod.rs");
    let mut registration = fs::read_to_string(&mod_path).unwrap_or_default();
    if !registration.contains("outline_review_gate_fixture_steps") {
        registration.push_str("pub mod outline_review_gate_fixture_steps;\n");
    }
    write_file(env, "tests/steps/mod.rs", &registration);
    write_file(
        env,
        "givn/specs/verify-change/usecase.md",
        &fixture_usecase(),
    );
    write_file(
        env,
        "givn/personas/fixture-persona.md",
        "# Persona: fixture\n",
    );
    write_file(
        env,
        &format!("{change}/review.md"),
        "# Review\n\nREADME-IMPACT: none\n\nPERSONAS: fixture-persona\n",
    );
    write_file(
        env,
        &format!("{change}/tasks.md"),
        &format!(
            "# Tasks\n\n\
             - [x] Scenario: {READY_TASK_TITLE}\n\
             \x20 - [x] COMMIT: {task_commit} -- {READY_TASK_TITLE}\n"
        ),
    );
}

/// Commit everything currently pending with a review-record message.
pub(crate) fn commit_review_record(env: &TestEnv) {
    commit(env, "docs(test-change): record the review decisions");
}

/// Run the real change-scoped test run and assert the receipt is current.
pub(crate) fn run_check_tasks(env: &TestEnv, change_id: &str) {
    let output = common::run_givn(env, &["check", "tasks", "--change", change_id]);
    assert!(
        output.status.success(),
        "givn check tasks failed: stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let receipt = env.path(&format!(
        "givn/changes/{change_id}/verification/tasks-verification.json"
    ));
    let text = fs::read_to_string(&receipt)
        .unwrap_or_else(|error| panic!("read receipt {}: {error}", receipt.display()));
    let value: serde_json::Value = serde_json::from_str(&text).expect("receipt JSON");
    assert_eq!(value["status"], "passed", "receipt is not passed: {text}");
}

fn fixture_usecase() -> String {
    "# Use case: verify-change\n\n\
     ## Level\n\n!\n\n\
     ## Actors\n\n- Developer\n\n\
     ## Goal\n\nCheck a change for structural validity before archive.\n\n\
     ## Trigger\n\nThe developer requests verification of a change.\n\n\
     ## Preconditions\n\n- The change artifacts and active specifications are available.\n\n\
     ## Main flow\n\n1. The developer requests verification; the system reads the active specifications and workflow artifacts.\n\n\
     ## Extensions\n\n- 1: a check fails → completion is blocked; protects the success guarantee.\n\n\
     ## Rules\n\n- Structural failures outrank other findings.\n\n\
     ## Examples\n\n- A duplicate title is reported as a hard error.\n\n\
     ## Minimal guarantee\n\nAn invalid change cannot pass the verification gate.\n\n\
     ## Success guarantee\n\nThe developer receives a complete verification result.\n\n\
     ## Personas\n\n- none\n\n\
     ## Capabilities\n\n- fixture-capability\n\n\
     ## Interactions\n\n\
     | Capability | Consumer action | E2E scenario |\n\
     |---|---|---|\n\
     | fixture-capability | run the fixture capability | Fixture behavior |\n\n\
     ## Includes\n\n- none\n\n\
     ## Extends\n\n- none\n\n\
     ## Out of scope\n\nNothing else.\n\n\
     ## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
        .to_string()
}

/// A rationale map keyed by unit id; scenario steps record the classification
/// and finding reason they expect the gate to compute.
pub(crate) type RationaleMap = BTreeMap<String, (String, String)>;
