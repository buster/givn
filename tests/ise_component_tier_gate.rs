//! The component-tier gate's acceptance fixture.
//!
//! A rebuilt project declares ten component scenarios and one `@e2e`
//! scenario. The frozen-input comparison is read-only; only `verify-trial`
//! can report a whole-system pass, and it requires the exact declared counts.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use sha2::{Digest, Sha256};
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

const E2E_SCRIPT: &str = r#"#!/bin/sh
set -eu
names=$(awk '
  /^  @e2e/ { e2e=1; next }
  /^  Scenario: / {
    if (!e2e) next
    e2e=0
    sub(/^  Scenario: /, "")
    printf "%s\"%s\"", sep, $0
    sep=","
  }
' givn/specs/demo.feature)
product=${GIVN_TRIAL_PRODUCT:-${GIVN_BIN:-}}
sha=$(sha256sum "$product" | cut -d' ' -f1)
printf '{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s],"product":"%s","product_sha256":"%s"}\n' "$names" "$product" "$sha" > "$GIVN_RESULT_FILE"
"#;

/// A corpus-reading component runner. It builds the declared product and
/// names every non-`@e2e` title; a count-only script cannot satisfy the gate.
fn component_script(total: usize) -> String {
    format!(
        r#"#!/bin/sh
set -eu
mkdir -p target/debug
printf 'rebuilt gate product\n' > target/debug/gate-product
names=$(awk '
  /^  @e2e/ {{ e2e=1; next }}
  /^  Scenario: / {{
    if (e2e) {{ e2e=0; next }}
    sub(/^  Scenario: /, "")
    printf "%s\"%s\"", sep, $0
    sep=","
  }}
' givn/specs/demo.feature)
printf '{{"scope":"component","total":{total},"passed":{total},"failed":0,"skipped":0,"scenarios":[%s]}}\n' "$names" > "$GIVN_RESULT_FILE"
"#
    )
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
        "verify:\n  command: \"sh ./scripts/verify.sh\"\n  e2e_command: \"sh ./scripts/e2e.sh\"\n  features_path: \"givn/specs\"\n  product_path: \"target/debug/gate-product\"\n  product_env: \"GIVN_BIN\"\n",
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
    write(&project.join("scripts/e2e.sh"), E2E_SCRIPT);
    write(&project.join("scripts/verify.sh"), &component_script(10));
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

const MULTISET_COMPONENT_SCRIPT: &str = r#"#!/bin/sh
set -eu
mkdir -p target/debug
printf 'rebuilt gate product\n' > target/debug/gate-product
printf '{"scope":"component","total":4,"passed":4,"failed":0,"skipped":0,"scenarios":["dup","dup","outline","outline"]}\n' > "$GIVN_RESULT_FILE"
"#;

#[test]
fn duplicate_titles_and_outline_rows_survive_the_running_gate() {
    let (root, project) = fixture();
    write(
        &project.join("givn/specs/demo.feature"),
        "Feature: multiset\n\n  Scenario: dup\n    Given nothing\n\n  Scenario: dup\n    Given nothing\n\n  Scenario Outline: outline\n    Given <value>\n\n    Examples:\n      | value |\n      | one   |\n      | two   |\n",
    );
    git(&project, &["add", "-A"]);
    git(&project, &["commit", "-qm", "fixture: multiset"]);
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(&copy, MULTISET_COMPONENT_SCRIPT);
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
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&result).unwrap()).unwrap();
    assert_eq!(record["component"]["executed"], 4);
    assert_eq!(
        record["component"]["scenarios"],
        serde_json::json!(["dup", "dup", "outline", "outline"])
    );
    assert_eq!(record["clean"], true);
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
    replace_component_script(&copy, &component_script(10));
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
    assert_eq!(record["component"]["executed"], 10);
    assert_eq!(
        record["component"]["scenarios"].as_array().unwrap().len(),
        10
    );
    assert_eq!(record["end_to_end"]["scenarios"][0], "acceptance");
    assert!(
        record["product"]["path"]
            .as_str()
            .unwrap()
            .ends_with("target/debug/gate-product")
    );
    assert!(record["product"]["sha256"].as_str().unwrap().len() == 64);
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
    replace_component_script(&copy, &component_script(9));
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

const MUTATING_E2E_SCRIPT: &str = r#"#!/bin/sh
set -eu
names=$(awk '
  /^  @e2e/ { e2e=1; next }
  /^  Scenario: / {
    if (!e2e) next
    e2e=0
    sub(/^  Scenario: /, "")
    printf "%s\"%s\"", sep, $0
    sep=","
  }
' givn/specs/demo.feature)
product=${GIVN_TRIAL_PRODUCT:-${GIVN_BIN:-}}
sha=$(sha256sum "$product" | cut -d' ' -f1)
printf '{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s],"product":"%s","product_sha256":"%s"}\n' "$names" "$product" "$sha" > "$GIVN_RESULT_FILE"
printf 'mutation\n' >> "$product"
"#;

const BAD_PRODUCT_E2E_SCRIPT: &str = r#"#!/bin/sh
set -eu
names=$(awk '
  /^  @e2e/ { e2e=1; next }
  /^  Scenario: / {
    if (!e2e) next
    e2e=0
    sub(/^  Scenario: /, "")
    printf "%s\"%s\"", sep, $0
    sep=","
  }
' givn/specs/demo.feature)
product=/bin/true
sha=$(sha256sum "$product" | cut -d' ' -f1)
printf '{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s],"product":"%s","product_sha256":"%s"}\n' "$names" "$product" "$sha" > "$GIVN_RESULT_FILE"
"#;

#[test]
fn an_e2e_result_bound_to_another_product_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(&copy, &component_script(10));
    write(&project.join("scripts/e2e.sh"), BAD_PRODUCT_E2E_SCRIPT);
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
        stderr(&output).contains("product_sha256"),
        "stderr={}",
        stderr(&output)
    );
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
}

#[test]
fn a_product_changed_after_binding_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(&copy, &component_script(10));
    write(&project.join("scripts/e2e.sh"), MUTATING_E2E_SCRIPT);
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
        stderr(&output).contains("changed after"),
        "stderr={}",
        stderr(&output)
    );
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
}

#[test]
fn a_product_path_outside_the_copy_refuses() {
    let (root, project) = fixture();
    write(
        &project.join("givn/commands.yaml"),
        "verify:\n  command: \"sh ./scripts/verify.sh\"\n  e2e_command: \"sh ./scripts/e2e.sh\"\n  features_path: \"givn/specs\"\n  product_path: \"../outside-product\"\n  product_env: \"GIVN_BIN\"\n",
    );
    git(&project, &["add", "-A"]);
    git(
        &project,
        &["commit", "-qm", "fixture: escaping product path"],
    );
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    replace_component_script(&copy, &component_script(10));

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
            root.path().join("trial.json").to_str().unwrap(),
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("cannot contain '..'"),
        "stderr={}",
        stderr(&output)
    );
}

#[cfg(unix)]
#[test]
fn a_product_parent_symlink_outside_the_copy_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    let outside = root.path().join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::os::unix::fs::symlink(&outside, copy.join("target")).unwrap();
    replace_component_script(&copy, &component_script(10));

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
            root.path().join("trial.json").to_str().unwrap(),
        ],
    );

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("resolves outside the spec copy"),
        "stderr={}",
        stderr(&output)
    );
}

fn rewrite_contract(manifest: &Path, copy: &Path, mutate: impl FnOnce(&mut serde_yaml_ng::Value)) {
    let path = copy.join("givn/ise-verification.yaml");
    let mut document: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    mutate(&mut document);
    let text = serde_yaml_ng::to_string(&document).unwrap();
    write(&path, &text);

    let mut digest = Sha256::new();
    digest.update(b"givn/ise-verification.yaml\0f\0");
    digest.update(b"0\0");
    digest.update(text.as_bytes());
    let checksum = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();

    let mut manifest_document: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&std::fs::read_to_string(manifest).unwrap()).unwrap();
    manifest_document["verification"]["checksum"] = serde_yaml_ng::Value::String(checksum);
    write(
        manifest,
        &serde_yaml_ng::to_string(&manifest_document).unwrap(),
    );
}

#[test]
fn a_contract_that_hides_the_frozen_corpus_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    rewrite_contract(&manifest, &copy, |document| {
        document["component"]["execution_units"] = serde_yaml_ng::Value::from(0u64);
        document["component"]["scenarios"] = serde_yaml_ng::Value::Sequence(Vec::new());
    });
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
        stderr(&output).contains("does not describe the frozen corpus"),
        "stderr={}",
        stderr(&output)
    );
    assert!(!result.exists(), "the corpus refusal writes no result");
}

#[test]
fn a_contract_from_another_revision_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    rewrite_contract(&manifest, &copy, |document| {
        document["revision"] = serde_yaml_ng::Value::String("deadbeef".to_string());
    });
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
        stderr(&output).contains("must describe the frozen revision"),
        "stderr={}",
        stderr(&output)
    );
}

#[test]
fn a_contract_change_during_the_scripts_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    let mut script = component_script(10);
    script.push_str("\nprintf 'tampered\\n' >> givn/ise-verification.yaml\n");
    replace_component_script(&copy, &script);
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
        stderr(&output).contains("frozen inputs or rebuilt product changed"),
        "stderr={}",
        stderr(&output)
    );
    let record: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
}
