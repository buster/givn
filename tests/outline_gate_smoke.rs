//! Smoke tests for the change-relative outline review gate.
//!
//! The gate is outline-first: every touched unit with methods needs a
//! `OUTLINE: PASS|FAIL` verdict from a fresh-context reviewer.
//! Metrics are hints and do not decide the gate.

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
    let (declaration, parts) = named_type("Engine", base);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("chore: base engine");

    write_change_dir(&repo);
    repo.commit("docs(test-change): add change directory");

    let (declaration, parts) = named_type("Engine", head);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("feat(test-change): reshape Engine");
    repo
}

fn last_line(text: &str) -> &str {
    text.lines().last().unwrap_or_default()
}

fn outline_review(unit: &str) -> String {
    format!(
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: REVIEWER fresh-context-reviewer\n\
         OUTLINE: PASS {unit} | one responsibility; the methods call in a clear order\n\
         OUTLINE: PASS\n"
    )
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

fn add_small_type(repo: &Repo) {
    repo.write(
        "src/small.rs",
        "pub struct Small;\nimpl Small {\n    pub fn run(&self) {}\n}\n",
    );
    repo.commit("feat(test-change): add Small");
}

#[test]
fn a_readable_outline_passes_and_metrics_are_hints() {
    let repo = project(620, 700);
    repo.write(
        "givn/changes/test-change/review.md",
        &outline_review("type:src/engine.rs::Engine"),
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- READABLE OUTLINE ---\n{text}");
    assert!(passed, "expected PASS, got:\n{text}");
    assert_eq!(last_line(&text), "outline review: PASS");
    assert!(
        text.contains("touched 3 unit(s), 1 outline(s), 1 metric hint(s)"),
        "the header must separate outlines from metric hints:\n{text}"
    );
    assert!(
        text.contains("OUTLINE   type:src/engine.rs::Engine"),
        "the type outline must be printed:\n{text}"
    );
    assert!(
        text.contains("CHECK     PASS"),
        "the outline verdict must be printed:\n{text}"
    );
    assert!(
        text.contains("HINT      type:src/engine.rs::Engine"),
        "the metric must be reported as a hint:\n{text}"
    );
    assert!(
        !text.contains("FINDING"),
        "metric findings no longer gate the outline:\n{text}"
    );
}

#[test]
fn an_unreadable_outline_fails_when_metrics_are_clean() {
    let repo = small_project();
    add_small_type(&repo);
    repo.write(
        "givn/changes/test-change/review.md",
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: REVIEWER fresh-context-reviewer\n\
         OUTLINE: FAIL type:src/small.rs::Small | the call order is not explainable\n\
         OUTLINE: PASS\n",
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    println!("--- UNREADABLE OUTLINE ---\n{text}");
    assert!(!passed, "expected FAIL, got:\n{text}");
    assert!(
        text.contains(
            "outline review failed for type:src/small.rs::Small: the call order is not explainable"
        ),
        "the outline failure must be explicit:\n{text}"
    );
    assert!(
        !text.contains("HINT"),
        "the fixture has no metric finding:\n{text}"
    );
}

#[test]
fn a_missing_outline_verdict_fails() {
    let repo = small_project();
    add_small_type(&repo);
    repo.write(
        "givn/changes/test-change/review.md",
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: REVIEWER fresh-context-reviewer\n\
         OUTLINE: PASS\n",
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("MISSING outline type:src/small.rs::Small"),
        "the missing verdict must name the unit:\n{text}"
    );
    assert_eq!(
        last_line(&text),
        "outline review: FAIL (1 outline without a verdict)"
    );
}

#[test]
fn an_empty_pass_explanation_fails() {
    let repo = small_project();
    add_small_type(&repo);
    repo.write(
        "givn/changes/test-change/review.md",
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: REVIEWER fresh-context-reviewer\n\
         OUTLINE: PASS type:src/small.rs::Small | \n\
         OUTLINE: PASS\n",
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains(
            "outline PASS for type:src/small.rs::Small has no responsibility/call-order explanation"
        ),
        "an empty PASS must fail:\n{text}"
    );
}

#[test]
fn a_missing_reviewer_fails() {
    let repo = small_project();
    add_small_type(&repo);
    repo.write(
        "givn/changes/test-change/review.md",
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: PASS type:src/small.rs::Small | one responsibility; the calls are clear\n\
         OUTLINE: PASS\n",
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("missing 'OUTLINE: REVIEWER <id>'"),
        "a reviewer is required:\n{text}"
    );
}

#[test]
fn the_reviewer_must_differ_from_the_author() {
    let repo = small_project();
    add_small_type(&repo);
    repo.write(
        "givn/changes/test-change/review.md",
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: REVIEWER agent\n\
         OUTLINE: PASS type:src/small.rs::Small | one responsibility; the calls are clear\n\
         OUTLINE: PASS\n",
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("outline reviewer 'agent' must differ from the author 'agent'"),
        "self-review must fail:\n{text}"
    );
}

#[test]
fn conflicting_outline_verdicts_fail() {
    let repo = small_project();
    add_small_type(&repo);
    repo.write(
        "givn/changes/test-change/review.md",
        "OUTLINE: AUTHOR agent\n\
         OUTLINE: REVIEWER fresh-context-reviewer\n\
         OUTLINE: PASS type:src/small.rs::Small | one responsibility; the calls are clear\n\
         OUTLINE: FAIL type:src/small.rs::Small | the calls are not clear\n\
         OUTLINE: PASS\n",
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("conflicting outline verdicts for type:src/small.rs::Small"),
        "two verdicts for one unit must fail:\n{text}"
    );
}

#[test]
fn a_root_change_commit_has_an_empty_base() {
    let repo = Repo::new();
    let (declaration, parts) = named_type("Engine", 620);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    write_change_dir(&repo);
    repo.commit("feat(test-change): root change");
    repo.write(
        "givn/changes/test-change/review.md",
        &outline_review("type:src/engine.rs::Engine"),
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(passed, "expected PASS, got:\n{text}");
    assert!(
        text.contains("base none (root change commit; empty base tree)"),
        "expected the root base line:\n{text}"
    );
    assert!(
        text.contains("OUTLINE   type:src/engine.rs::Engine"),
        "every head unit with methods gets an outline:\n{text}"
    );
}

#[test]
fn a_change_directory_without_a_reachable_commit_fails_closed() {
    let repo = Repo::new();
    repo.write("src/engine.rs", "pub struct Engine { value: u64 }\n");
    repo.commit("chore: base engine");
    write_change_dir(&repo);

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "outline review: FAIL (change directory 'givn/changes/test-change/' has no reachable commit; commit the change directory before review)"
    );
}

#[test]
fn dirty_production_paths_fail_before_analysis() {
    let repo = project(620, 480);
    let current = fs::read_to_string(repo.root().join("src/engine.rs")).expect("read source");
    repo.write("src/engine.rs", &format!("{current}\n// uncommitted\n"));

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert!(
        text.contains("production paths are dirty; commit or discard before review: src/engine.rs"),
        "expected the dirty-path message:\n{text}"
    );
    assert!(
        !text.contains("OUTLINE"),
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
        givn::outline::gate::evaluate(clone.path(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "outline review: FAIL (cannot resolve change base: repository is shallow; fetch the full history before review)"
    );
}

#[test]
fn a_parse_failure_and_an_unanalyzed_file_fail_closed() {
    let parse = small_project();
    parse.write("src/broken.rs", "pub struct Broken {\n");
    parse.commit("feat(test-change): malformed source");
    let (passed, text) =
        givn::outline::gate::evaluate(parse.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "outline review: FAIL (cannot analyze src/broken.rs at head)"
    );

    let skipped = small_project();
    skipped.write("src/tests/support.rs", "pub fn helper() -> u64 { 1 }\n");
    skipped.commit("feat(test-change): analyzer-excluded source");
    let (passed, text) =
        givn::outline::gate::evaluate(skipped.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "outline review: FAIL (changed production file 'src/tests/support.rs' was not analyzed; the analyzer scope or extension filter is wrong)"
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
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(!passed);
    assert_eq!(
        last_line(&text),
        "outline review: FAIL (cannot analyze src/engine.rs at head: symlink)"
    );
}

#[test]
fn a_merge_commit_uses_the_first_parent_of_the_first_touch() {
    let repo = Repo::new();
    let (declaration, parts) = named_type("Engine", 620);
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
    let (declaration, parts) = named_type("Engine", 700);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("feat(test-change): grow Engine");

    repo.git(&["checkout", "-q", &main]);
    repo.write("src/unrelated.rs", "pub struct Unrelated;\n");
    repo.commit("chore: unrelated main change");
    repo.git(&["checkout", "-q", "feature"]);
    repo.git(&["merge", "-q", "--no-edit", &main]);
    repo.write(
        "givn/changes/test-change/review.md",
        &outline_review("type:src/engine.rs::Engine"),
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(passed, "expected PASS, got:\n{text}");
    assert!(
        text.contains("existing finding grew: 620 -> 700 code lines"),
        "the merge's first parent must be the change base:\n{text}"
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
    let (declaration, parts) = named_type("Engine", 620);
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
    let (declaration, parts) = named_type("Engine", 700);
    repo.write("src/engine.rs", &declaration);
    repo.write("src/engine/parts.rs", &parts);
    repo.commit("grow engine");
    repo.write(
        "givn/changes/test-change/review.md",
        &outline_review("type:src/engine.rs::Engine"),
    );

    let (passed, text) =
        givn::outline::gate::evaluate(repo.root(), "test-change").expect("gate run");
    assert!(passed, "expected PASS, got:\n{text}");
    assert!(
        text.contains("existing finding grew: 620 -> 700 code lines"),
        "the gate must analyze the Fossil current check-in:\n{text}"
    );
}
