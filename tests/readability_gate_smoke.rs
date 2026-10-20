//! Smoke test for the change-relative readability review gate.
//!
//! Builds a real temporary Git project with a 620-code-line type split across
//! two files, commits a change directory, and drives the public gate API. The
//! gate must analyze the committed head tree, account for the cross-file type
//! as one unit, and pass or fail on the exact contract.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

struct Repo {
    root: tempfile::TempDir,
}

impl Repo {
    fn new() -> Self {
        let repo = Self {
            root: tempfile::tempdir().expect("temp repository"),
        };
        repo.git(&["init", "-q"]);
        repo.git(&["config", "user.email", "gate@example.com"]);
        repo.git(&["config", "user.name", "Gate Test"]);
        repo
    }

    fn root(&self) -> &Path {
        self.root.path()
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.root.path().join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, content).expect("write fixture");
    }

    fn git(&self, args: &[&str]) -> Output {
        let output = Command::new("git")
            .args(args)
            .current_dir(self.root.path())
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    fn commit(&self, message: &str) {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "-m", message]);
    }
}

fn method(name: &str, field: &str, body_lines: usize) -> String {
    let mut source = format!("    pub fn {name}(&self) -> u64 {{\n");
    for _ in 0..body_lines {
        source.push_str(&format!("        let _ = self.{field};\n"));
    }
    source.push_str(&format!("        self.{field}\n    }}\n"));
    source
}

const METHODS: usize = 8;
const FIELDS: [&str; 3] = ["a", "b", "c"];

/// A type split across the declaration file and a parts file. Eight methods
/// keep every method under the 100-code-line method threshold while the type
/// itself reaches `total` code lines. The struct and impl overhead is five
/// struct lines, four impl-brace lines, and three lines per method.
fn engine(total: u32) -> (String, String) {
    named_type("Engine", total)
}

fn named_type(name: &str, total: u32) -> (String, String) {
    let overhead = 5 + 4 + (3 * METHODS) as u32;
    let body = total.saturating_sub(overhead) / METHODS as u32;
    let remainder = total.saturating_sub(overhead) % METHODS as u32;
    let methods: Vec<String> = (0..METHODS)
        .map(|index| {
            let lines = body + u32::from((index as u32) < remainder);
            method(
                &format!("method_{index}"),
                FIELDS[index % FIELDS.len()],
                lines as usize,
            )
        })
        .collect();
    let declaration = format!(
        "pub struct {name} {{\n    a: u64,\n    b: u64,\n    c: u64,\n}}\n\nimpl {name} {{\n{}{}{}{}\n}}\n",
        methods[0], methods[1], methods[2], methods[3]
    );
    let parts = format!(
        "impl {name} {{\n{}{}{}{}\n}}\n",
        methods[4], methods[5], methods[6], methods[7]
    );
    (declaration, parts)
}

fn project(base: u32, head: u32) -> Repo {
    let repo = Repo::new();
    let (declaration, parts) = engine(base);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("chore: base engine");

    repo.write(
        "givn/changes/test-change/proposal.md",
        "# Proposal: test change\n",
    );
    repo.write(
        "givn/changes/test-change/design.md",
        "# Design: test change\n",
    );
    repo.write(
        "givn/changes/test-change/tasks.md",
        "# Tasks: test change\n",
    );
    repo.commit("docs(test-change): add change directory");

    let (declaration, parts) = engine(head);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("feat(test-change): reshape Engine");
    repo
}

fn last_line(text: &str) -> &str {
    text.lines().last().unwrap_or_default()
}

#[test]
fn a_fixed_gate_finding_passes_and_a_missing_decision_fails() {
    let passing = project(620, 480);
    passing.write(
        "givn/changes/test-change/review.md",
        "READABILITY: FIXED type:src/engine.rs::Engine\nREADABILITY: PASS\n",
    );
    let (passed, text) =
        givn::readability::gate::evaluate(passing.root(), "test-change").expect("gate run");
    println!("--- PASS FIXTURE ---\n{text}");
    assert!(passed, "expected PASS, got:\n{text}");
    assert_eq!(last_line(&text), "readability review: PASS");
    assert!(
        text.contains("FINDING   type:src/engine.rs::Engine  fixed at head:"),
        "the cross-file type must be one unit with its declaration-file id:\n{text}"
    );
    assert!(
        text.contains("touched 3 unit(s), 1 finding(s)"),
        "expected the type and its two module units:\n{text}"
    );
    assert!(
        text.contains("DECISION  FIXED      type:src/engine.rs::Engine"),
        "expected the fixed decision:\n{text}"
    );

    let failing = project(620, 700);
    failing.write("givn/changes/test-change/review.md", "READABILITY: PASS\n");
    let (passed, text) =
        givn::readability::gate::evaluate(failing.root(), "test-change").expect("gate run");
    println!("--- FAIL FIXTURE ---\n{text}");
    assert!(!passed, "expected FAIL, got:\n{text}");
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (1 finding without a decision)"
    );
    assert!(
        text.contains("MISSING type:src/engine.rs::Engine"),
        "expected the missing decision:\n{text}"
    );
    assert!(
        text.contains("existing finding grew: 620 -> 700 code lines"),
        "expected the measured base and head sizes:\n{text}"
    );
    assert!(
        text.contains("FAILURE  unaccounted  MISSING type:src/engine.rs::Engine"),
        "expected the unaccounted failure line:\n{text}"
    );

    // The design's complete MISSING block has no PASS marker: the final line
    // stays the missing-decision count instead of adding a second verdict
    // failure.
    let undecided = project(620, 700);
    undecided.write("givn/changes/test-change/review.md", "");
    let (passed, text) =
        givn::readability::gate::evaluate(undecided.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (1 finding without a decision)"
    );
    assert!(
        !text.contains("missing 'READABILITY: PASS'"),
        "an undecided finding is the actionable failure:\n{text}"
    );
}

fn write_change_dir(repo: &Repo) {
    repo.write(
        "givn/changes/test-change/proposal.md",
        "# Proposal: test change\n",
    );
    repo.write(
        "givn/changes/test-change/design.md",
        "# Design: test change\n",
    );
    repo.write(
        "givn/changes/test-change/tasks.md",
        "# Tasks: test change\n",
    );
}

fn small_project() -> Repo {
    let repo = Repo::new();
    repo.write(
        "src/engine.rs",
        "pub struct Engine { value: u64 }\nimpl Engine { pub fn value(&self) -> u64 { self.value } }\n",
    );
    repo.commit("chore: base engine");
    write_change_dir(&repo);
    repo.commit("docs(test-change): add change directory");
    repo
}

#[test]
fn a_root_change_commit_has_an_empty_base() {
    let repo = Repo::new();
    let (declaration, parts) = engine(620);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    write_change_dir(&repo);
    repo.commit("feat(test-change): root change");
    repo.write("givn/changes/test-change/review.md", "READABILITY: PASS\n");

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- ROOT FIXTURE ---\n{text}");
    assert!(!passed);
    assert!(
        text.contains("base none (root change commit; empty base tree)"),
        "expected the root base line:\n{text}"
    );
    assert!(
        text.contains("MISSING type:src/engine.rs::Engine"),
        "every head unit is new in an empty base:\n{text}"
    );
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (1 finding without a decision)"
    );
}

#[test]
fn a_change_directory_without_a_reachable_commit_fails_closed() {
    let repo = Repo::new();
    repo.write("src/engine.rs", "pub struct Engine { value: u64 }\n");
    repo.commit("chore: base engine");
    write_change_dir(&repo);

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (change directory 'givn/changes/test-change/' has no reachable commit; commit the change directory before review)"
    );
}

#[test]
fn dirty_production_paths_fail_before_analysis() {
    let repo = project(620, 480);
    let current = fs::read_to_string(repo.root().join("src/engine.rs")).expect("read source");
    repo.write("src/engine.rs", &format!("{current}\n// uncommitted\n"));

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("production paths are dirty; commit or discard before review: src/engine.rs"),
        "expected the dirty-path message:\n{text}"
    );
    assert!(
        !text.contains("FINDING"),
        "the gate must fail before analysis:\n{text}"
    );
}

#[test]
fn a_shallow_clone_fails_closed() {
    let source = project(620, 480);
    let clone = tempfile::tempdir().expect("clone parent");
    let url = format!("file://{}", source.root().display());
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

    let (passed, text) =
        givn::readability::gate::evaluate(clone.path(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (cannot resolve change base: repository is shallow; fetch the full history before review)"
    );
}

#[test]
fn a_parse_failure_and_an_unanalyzed_file_fail_closed() {
    let parse = small_project();
    parse.write("src/broken.rs", "pub struct Broken {\n");
    parse.commit("feat(test-change): malformed source");
    let (passed, text) =
        givn::readability::gate::evaluate(parse.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (cannot analyze src/broken.rs at head)"
    );

    let skipped = small_project();
    skipped.write("src/tests/support.rs", "pub fn helper() -> u64 { 1 }\n");
    skipped.commit("feat(test-change): analyzer-excluded source");
    let (passed, text) =
        givn::readability::gate::evaluate(skipped.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (changed production file 'src/tests/support.rs' was not analyzed; the analyzer scope or extension filter is wrong)"
    );
}

#[cfg(unix)]
#[test]
fn a_touched_symlink_fails_closed() {
    let repo = small_project();
    fs::remove_file(repo.root().join("src/engine.rs")).expect("remove source");
    std::os::unix::fs::symlink("parts.rs", repo.root().join("src/engine.rs"))
        .expect("create symlink");
    repo.commit("feat(test-change): replace source with a symlink");

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (cannot analyze src/engine.rs at head: symlink)"
    );
}

#[test]
fn a_merge_commit_uses_the_first_parent_of_the_first_touch() {
    let repo = Repo::new();
    let (declaration, parts) = engine(620);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("chore: base engine");
    let main = String::from_utf8(repo.git(&["branch", "--show-current"]).stdout)
        .expect("branch name")
        .trim()
        .to_string();

    repo.git(&["checkout", "-q", "-b", "feature"]);
    write_change_dir(&repo);
    repo.commit("docs(test-change): add change directory");
    let (declaration, parts) = engine(700);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("feat(test-change): grow Engine");

    repo.git(&["checkout", "-q", &main]);
    repo.write("src/unrelated.rs", "pub struct Unrelated;\n");
    repo.commit("chore: unrelated main change");
    repo.git(&["checkout", "-q", "feature"]);
    repo.git(&["merge", "-q", "--no-edit", &main]);
    repo.write("givn/changes/test-change/review.md", "READABILITY: PASS\n");

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("existing finding grew: 620 -> 700 code lines"),
        "the merge's first parent must be the change base:\n{text}"
    );
}

fn write_owner_persona(repo: &Repo) {
    repo.write(
        "givn/personas/givn-maintainer--owner.md",
        "# Persona: Maintainer\n\n- Role: The maintainer who owns the architecture record.\n",
    );
}

fn write_rationale(repo: &Repo, text: &str) {
    repo.write(
        "givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md",
        text,
    );
}

const RATIONALE: &str = "Unit: type:src/engine.rs::Engine\n\
Classification: god-type\n\
Finding: existing-grown\n\
Evidence: 620 -> 700 code lines, 8 public methods; every call site is in src/engine.rs.\n\
Why this shape is right: Engine owns the engine state machine and its phase order, so splitting it would move the transition rules into two modules.\n\
Alternatives considered: Extract the renderer into a separate module. Rejected: rendering already delegates to the report module and the extraction adds indirection.\n\
Owner: givn-maintainer--owner\n";

#[test]
fn a_justified_finding_with_a_complete_rationale_passes() {
    let repo = project(620, 700);
    write_owner_persona(&repo);
    write_rationale(&repo, RATIONALE);
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED type:src/engine.rs::Engine\nREADABILITY: PASS\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- JUSTIFIED FIXTURE ---\n{text}");
    assert!(passed, "expected PASS, got:\n{text}");
    assert!(
        text.contains("DECISION  JUSTIFIED  type:src/engine.rs::Engine  rationale givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md"),
        "expected the rationale decision:\n{text}"
    );
    assert_eq!(last_line(&text), "readability review: PASS");
}

#[test]
fn a_temporary_rationale_without_a_follow_up_fails() {
    let repo = project(620, 700);
    write_owner_persona(&repo);
    write_rationale(&repo, &format!("{RATIONALE}Expiry: 2026-12-01\n"));
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED type:src/engine.rs::Engine\nREADABILITY: PASS\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("missing Follow-up:"),
        "expected the pair check:\n{text}"
    );
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (rationale givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md: missing Follow-up:)"
    );
}

#[test]
fn a_missing_review_record_with_a_finding_fails() {
    let repo = project(620, 700);
    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (readability record is missing; create givn/changes/test-change/review.md and record 'READABILITY: PASS')"
    );
}

#[test]
fn the_decision_contract_rejects_fixed_and_justified_mismatches() {
    let shrunk = project(620, 480);
    shrunk.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED type:src/engine.rs::Engine\nREADABILITY: PASS\n",
    );
    let (passed, text) =
        givn::readability::gate::evaluate(shrunk.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (JUSTIFIED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is not a finding at head; use READABILITY: FIXED type:src/engine.rs::Engine)"
    );

    let grown = project(620, 700);
    grown.write(
        "givn/changes/test-change/review.md",
        "READABILITY: FIXED type:src/engine.rs::Engine\nREADABILITY: PASS\n",
    );
    let (passed, text) =
        givn::readability::gate::evaluate(grown.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (FIXED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is still a finding: god-type 700 code lines (base 620))"
    );
}

#[test]
fn unknown_and_conflicting_markers_fail_closed() {
    let unknown = project(620, 700);
    unknown.write(
        "givn/changes/test-change/review.md",
        "READABILITY: DEFERRED type:src/engine.rs::Engine\n",
    );
    let (passed, text) =
        givn::readability::gate::evaluate(unknown.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (unknown readability marker: READABILITY: DEFERRED type:src/engine.rs::Engine)"
    );

    let conflicting = project(620, 700);
    conflicting.write(
        "givn/changes/test-change/review.md",
        "READABILITY: FIXED type:src/engine.rs::Engine\n\
         READABILITY: JUSTIFIED type:src/engine.rs::Engine\n\
         READABILITY: PASS\n",
    );
    let (passed, text) =
        givn::readability::gate::evaluate(conflicting.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (conflicting decisions for type:src/engine.rs::Engine)"
    );
}

#[test]
fn a_decided_finding_still_requires_the_readability_verdict() {
    let repo = project(620, 700);
    write_owner_persona(&repo);
    write_rationale(&repo, RATIONALE);
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED type:src/engine.rs::Engine\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (missing 'READABILITY: PASS')"
    );
}

#[test]
fn a_fail_marker_blocks_even_a_justified_finding() {
    let repo = project(620, 700);
    write_owner_persona(&repo);
    write_rationale(&repo, RATIONALE);
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED type:src/engine.rs::Engine\n\
         READABILITY: PASS\n\
         FAIL the type has no single responsibility\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (the type has no single responsibility)"
    );
}

#[test]
fn two_units_with_the_same_rationale_slug_fail() {
    let repo = Repo::new();
    repo.write("src/base.rs", "pub struct Base;\n");
    repo.commit("chore: base");
    write_change_dir(&repo);
    repo.commit("docs(test-change): add change directory");

    for (declaration_path, parts_path) in [
        ("src/a_b.rs", "src/a_b/parts.rs"),
        ("src/a-b.rs", "src/a-b/parts.rs"),
    ] {
        let (declaration, parts) = named_type("C", 620);
        repo.write(declaration_path, &declaration);
        repo.write(parts_path, &parts);
    }
    repo.commit("feat(test-change): add two C types");
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED type:src/a_b.rs::C\n\
         READABILITY: JUSTIFIED type:src/a-b.rs::C\n\
         READABILITY: PASS\n",
    );
    write_rationale(
        &repo,
        "Unit: type:src/a_b.rs::C\n\
         Classification: god-type\n\
         Finding: new-type\n\
         Evidence: 620 code lines, 8 public methods.\n\
         Why this shape is right: C owns the C state machine and its phase order, so splitting it would move the transition rules into two modules.\n\
         Alternatives considered: Extract the renderer. Rejected: rendering delegates already and the extraction adds indirection.\n\
         Owner: givn-maintainer--owner\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- SLUG COLLISION FIXTURE ---\n{text}");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (rationale slug collision for type:src/a-b.rs::C and type:src/a_b.rs::C)"
    );
}

#[test]
fn a_file_move_closes_the_old_unit_and_needs_a_decision_for_the_new_unit() {
    let repo = Repo::new();
    let (declaration, parts) = engine(620);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("chore: base engine");
    write_change_dir(&repo);
    repo.commit("docs(test-change): add change directory");

    fs::create_dir_all(repo.root().join("src/core")).expect("create destination");
    repo.git(&["mv", "src/engine.rs", "src/core.rs"]);
    repo.git(&["mv", "src/engine/parts.rs", "src/core/parts.rs"]);
    repo.commit("feat(test-change): move Engine");

    write_owner_persona(&repo);
    repo.write(
        "givn/changes/test-change/readability/rationales/type-src-core-rs-engine.md",
        "Unit: type:src/core.rs::Engine\n\
         Classification: god-type\n\
         Finding: new-type\n\
         Evidence: 620 code lines, 8 public methods; every call site is in src/core.rs.\n\
         Why this shape is right: Engine owns the engine state machine and its phase order, so splitting it would move the transition rules into two modules.\n\
         Alternatives considered: Extract the renderer. Rejected: rendering delegates already and the extraction adds indirection.\n\
         Owner: givn-maintainer--owner\n",
    );
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: FIXED type:src/engine.rs::Engine\n\
         READABILITY: JUSTIFIED type:src/core.rs::Engine\n\
         READABILITY: PASS\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- MOVE FIXTURE ---\n{text}");
    assert!(passed, "expected PASS, got:\n{text}");
    assert!(
        text.contains("removed: base 620 code lines; finding no longer fires"),
        "the old unit is a removal:\n{text}"
    );
    assert!(
        text.contains("FIXED      type:src/engine.rs::Engine"),
        "the old unit needs FIXED:\n{text}"
    );
    assert!(
        text.contains("JUSTIFIED  type:src/core.rs::Engine"),
        "the new unit needs its own decision:\n{text}"
    );
}

#[test]
fn a_data_only_type_is_exempt_and_its_module_can_be_justified() {
    let repo = Repo::new();
    repo.write("src/base.rs", "pub struct Base;\n");
    repo.commit("chore: base");
    write_change_dir(&repo);
    repo.commit("docs(test-change): add change directory");

    let mut status = String::from("pub enum Status {\n");
    for index in 0..615 {
        status.push_str(&format!("    Variant{index},\n"));
    }
    status.push_str("}\n\nimpl Status {\n    pub fn variant_count(&self) -> usize { 615 }\n}\n");
    repo.write("src/status.rs", &status);
    repo.commit("feat(test-change): add the status vocabulary");

    write_owner_persona(&repo);
    repo.write(
        "givn/changes/test-change/readability/rationales/module-src-status-rs.md",
        "Unit: module:src/status.rs\n\
         Classification: module\n\
         Finding: new-module\n\
         Evidence: 620 code lines in one file; the enum carries no behavior.\n\
         Why this shape is right: The status module owns the exhaustive status vocabulary and its ordering, so splitting the variants would create a second source of truth.\n\
         Alternatives considered: Split the enum into per-domain enums. Rejected: every caller matches the full vocabulary and the split would add conversion glue.\n\
         Owner: givn-maintainer--owner\n",
    );
    repo.write(
        "givn/changes/test-change/review.md",
        "READABILITY: JUSTIFIED module:src/status.rs\nREADABILITY: PASS\n",
    );

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- DATA-ONLY FIXTURE ---\n{text}");
    assert!(passed, "expected PASS, got:\n{text}");
    assert!(
        !text.contains("FINDING   type:src/status.rs::Status"),
        "a data-only type is exempt from the size gate:\n{text}"
    );
    assert!(
        text.contains("FINDING   module:src/status.rs"),
        "the module finding is accounted for:\n{text}"
    );
}

struct FossilRepo {
    root: tempfile::TempDir,
    _repository: tempfile::TempDir,
}

impl FossilRepo {
    fn available() -> bool {
        Command::new("fossil")
            .arg("version")
            .output()
            .is_ok_and(|output| output.status.success())
    }

    fn new() -> Self {
        let root = tempfile::tempdir().expect("fossil checkout");
        let repository = tempfile::tempdir().expect("fossil repository");
        let repo_path = repository.path().join("project.fossil");
        let repo = Self {
            root,
            _repository: repository,
        };
        repo.run(&["init", "-q", &repo_path.to_string_lossy()]);
        repo.run(&["open", &repo_path.to_string_lossy()]);
        // The fixture commits run as a named local user. Fossil refuses an
        // unknown user; create it once in the checkout's repository.
        let _ = Command::new("fossil")
            .args(["user", "new", "gate", "gate"])
            .current_dir(repo.root.path())
            .output();
        repo
    }

    fn root(&self) -> &Path {
        self.root.path()
    }

    fn write(&self, relative: &str, content: &str) {
        let path = self.root.path().join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, content).expect("write fixture");
    }

    fn run(&self, args: &[&str]) -> Output {
        let output = Command::new("fossil")
            .args(args)
            .current_dir(self.root.path())
            .env("FOSSIL_USER", "gate")
            .output()
            .expect("run fossil");
        assert!(
            output.status.success(),
            "fossil {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    fn commit(&self, message: &str) {
        let _ = Command::new("fossil")
            .args(["add", "."])
            .current_dir(self.root.path())
            .env("FOSSIL_USER", "gate")
            .output();
        self.run(&["commit", "-m", message, "--user", "gate"]);
    }
}

#[test]
fn a_fossil_checkout_runs_the_gate_against_current() {
    if !FossilRepo::available() {
        eprintln!("skipping the Fossil gate smoke test: fossil is not on PATH");
        return;
    }
    let repo = FossilRepo::new();
    let (declaration, parts) = engine(620);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("base engine");
    repo.write(
        "givn/changes/test-change/proposal.md",
        "# Proposal: test change\n",
    );
    repo.write(
        "givn/changes/test-change/design.md",
        "# Design: test change\n",
    );
    repo.write(
        "givn/changes/test-change/tasks.md",
        "# Tasks: test change\n",
    );
    repo.commit("add change directory");
    let (declaration, parts) = engine(700);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("grow engine");
    repo.write("givn/changes/test-change/review.md", "READABILITY: PASS\n");

    let (passed, text) =
        givn::readability::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- FOSSIL FIXTURE ---\n{text}");
    assert!(!passed);
    assert!(
        text.contains("existing finding grew: 620 -> 700 code lines"),
        "the gate must analyze the Fossil current check-in:\n{text}"
    );
    assert_eq!(
        last_line(&text),
        "readability review: FAIL (1 finding without a decision)"
    );
}
