//! The component-tier gate's acceptance fixture.
//!
//! A rebuilt project declares ten component scenarios and one `@e2e`
//! scenario. The frozen-input comparison is read-only; only `verify-trial`
//! can report a whole-system pass, and it requires the exact declared counts.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const GIVN: &str = env!("CARGO_BIN_EXE_givn");

fn git(project: &Path, args: &[&str]) -> Output {
    let output = Command::new("git")
        .args(args)
        .current_dir(project)
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.com")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.com")
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).expect("write fixture file");
}

/// A committed project with ten component scenarios, one `@e2e` scenario, and
/// verify/e2e commands that the builder can replace inside the copy.
fn fixture() -> (TempDir, PathBuf) {
    let root = tempfile::TempDir::new().expect("tempdir");
    let project = root.path().join("project");
    std::fs::create_dir_all(&project).expect("create project");
    git(&project, &["init", "-q"]);

    write(
        &project.join("givn/config.yaml"),
        "addons:\n  ise: true\n  coverage: false\n",
    );
    write(
        &project.join("givn/commands.yaml"),
        "verify:\n  command: \"./scripts/verify.sh\"\n  e2e_command: \"./scripts/e2e.sh\"\n  features_path: \"givn/specs\"\n",
    );
    write(
        &project.join("givn/specification.yaml"),
        "behavior: [\"givn/specs/**\"]\nstructure: []\ncontracts: []\nlimits: []\ne2e: [\"tests/e2e\"]\nsource: [\"src/**\", \"scripts/**\"]\n",
    );

    let mut feature = String::from("Feature: gate probe\n\n");
    for index in 1..=10 {
        feature.push_str(&format!(
            "  Scenario: component {index}\n    Given nothing\n\n"
        ));
    }
    feature.push_str("  @e2e\n  Scenario: acceptance\n    Given nothing\n");
    write(&project.join("givn/specs/demo.feature"), &feature);

    write(
        &project.join("tests/e2e/Cargo.toml"),
        "[package]\nname = \"e2e\"\n",
    );
    write(&project.join("tests/e2e/steps/demo.rs"), "// e2e steps\n");
    write(&project.join("src/one.rs"), "pub fn one() {}\n");
    write(
        &project.join("scripts/e2e.sh"),
        "#!/bin/sh\nprintf '{\"scope\":\"e2e\",\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\n",
    );
    write(
        &project.join("scripts/verify.sh"),
        "#!/bin/sh\nprintf '{\"scope\":\"component\",\"total\":10,\"passed\":10,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\n",
    );
    for script in ["scripts/e2e.sh", "scripts/verify.sh"] {
        let path = project.join(script);
        let mut permissions = std::fs::metadata(&path).unwrap().permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            permissions.set_mode(0o755);
        }
        #[cfg(not(unix))]
        {
            let _ = &mut permissions;
        }
        std::fs::set_permissions(path, permissions).unwrap();
    }

    git(&project, &["add", "-A"]);
    git(&project, &["commit", "-qm", "fixture"]);
    (root, project)
}

fn run(project: &Path, args: &[&str]) -> Output {
    Command::new(GIVN)
        .args(args)
        .current_dir(project)
        .output()
        .expect("run givn")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn rebuild(project: &Path, root: &Path) -> PathBuf {
    let copy = root.join("copy");
    let manifest = root.join("manifest.yaml");
    let output = run(
        project,
        &[
            "ise",
            "rebuild-all",
            "--into",
            copy.to_str().unwrap(),
            "--manifest",
            manifest.to_str().unwrap(),
        ],
    );
    assert!(
        output.status.success(),
        "rebuild failed: {}",
        stderr(&output)
    );
    manifest
}

fn replace_component_script(copy: &Path, body: &str) {
    let path = copy.join("scripts/verify.sh");
    write(&path, body);
    let mut permissions = std::fs::metadata(&path).unwrap().permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.set_mode(0o755);
    }
    #[cfg(not(unix))]
    {
        let _ = &mut permissions;
    }
    std::fs::set_permissions(path, permissions).unwrap();
}

#[test]
fn a_missing_component_runner_is_not_a_green_whole_system_trial() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(&copy, "#!/bin/sh\ntrue\n");
    let result = root.path().join("trial.json");

    let output = run(
        &project,
        &[
            "ise",
            "verify-trial",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
            "--result",
            result.to_str().unwrap(),
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("component_executed=0/10"),
        "stderr={}",
        stderr(&output)
    );
    assert!(
        !stdout(&output).contains("Trial result: passed"),
        "stdout={}",
        stdout(&output)
    );
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
    assert_eq!(record["component"]["executed"], 0);
    assert_eq!(record["component"]["declared"], 10);
}

#[test]
fn a_passing_component_and_e2e_result_records_the_counts() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(
        &copy,
        "#!/bin/sh\nprintf '{\"scope\":\"component\",\"total\":10,\"passed\":10,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\n",
    );
    let result = root.path().join("trial.json");

    let output = run(
        &project,
        &[
            "ise",
            "verify-trial",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
            "--result",
            result.to_str().unwrap(),
        ],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("Component scenarios: 10/10 executed"),
        "stdout={}",
        stdout(&output)
    );
    assert!(
        stdout(&output).contains("End-to-end tests: 1/1 executed"),
        "stdout={}",
        stdout(&output)
    );
    assert!(stdout(&output).contains("Trial result: passed"));
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], true);
    assert_eq!(record["scope"], "ise-trial-result/v1");
}

#[test]
fn compare_is_read_only_and_prints_unverified() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(&copy, "#!/bin/sh\nexit 99\n");

    let output = run(
        &project,
        &[
            "ise",
            "compare",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
        ],
    );

    assert!(
        output.status.success(),
        "compare must stay read-only: {}",
        stderr(&output)
    );
    assert!(stdout(&output).contains("Specification files: match"));
    assert!(stdout(&output).contains("End-to-end tests: match"));
    assert!(stdout(&output).contains("Trial result: unverified"));
    assert!(!stdout(&output).contains("Trial result: passed"));
}

#[test]
fn an_exact_count_mismatch_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(
        &copy,
        "#!/bin/sh\nprintf '{\"scope\":\"component\",\"total\":9,\"passed\":9,\"failed\":0,\"skipped\":0}\\n' > \"$GIVN_RESULT_FILE\"\n",
    );
    let result = root.path().join("trial.json");

    let output = run(
        &project,
        &[
            "ise",
            "verify-trial",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
            "--result",
            result.to_str().unwrap(),
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("component_executed=9/10"),
        "stderr={}",
        stderr(&output)
    );
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
}

#[test]
fn a_not_run_decision_cannot_replace_declared_units() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    let result = root.path().join("trial.json");

    let output = run(
        &project,
        &[
            "ise",
            "verify-trial",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
            "--result",
            result.to_str().unwrap(),
            "--component-not-run",
            "out of scope",
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("cannot replace their execution"),
        "stderr={}",
        stderr(&output)
    );
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
}

#[test]
fn a_revision_tracking_the_reserved_contract_path_refuses() {
    let (root, project) = fixture();
    write(
        &project.join("givn/ise-verification.yaml"),
        "scope: project-owned\n",
    );
    git(&project, &["add", "-A"]);
    git(&project, &["commit", "-qm", "fixture: reserved path"]);

    let copy = root.path().join("copy");
    let manifest = root.path().join("manifest.yaml");
    let output = run(
        &project,
        &[
            "ise",
            "rebuild-all",
            "--into",
            copy.to_str().unwrap(),
            "--manifest",
            manifest.to_str().unwrap(),
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("reserved for the generated verification contract"),
        "stderr={}",
        stderr(&output)
    );
    assert!(!copy.exists(), "the copy must not be written");
}

#[test]
fn a_result_path_escaping_into_the_copy_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");

    let output = run(
        &project,
        &[
            "ise",
            "verify-trial",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
            "--result",
            "../copy/trial.json",
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("inside the spec copy"),
        "stderr={}",
        stderr(&output)
    );
}

#[cfg(unix)]
#[test]
fn a_symlink_result_path_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    let target = copy.join("symlink-result.json");
    let link = project.join("escape-result.json");
    std::os::unix::fs::symlink(&target, &link).expect("create the broken symlink");

    let output = run(
        &project,
        &[
            "ise",
            "verify-trial",
            "--manifest",
            manifest.to_str().unwrap(),
            "--copy",
            copy.to_str().unwrap(),
            "--result",
            "escape-result.json",
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("is a symlink"),
        "stderr={}",
        stderr(&output)
    );
    assert!(
        !target.exists(),
        "the result must not be written inside the copy"
    );
}
