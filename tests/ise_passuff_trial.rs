//! A passuff-shaped trial: two rebuilt binaries in one directory, a
//! `PASSUFF_BIN_DIR` environment variable, a frozen e2e package absent from the
//! copy, and a coverage script that writes only the component report.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

const GIVN: &str = env!("CARGO_BIN_EXE_givn");

const VERIFY_SCRIPT: &str = r#"#!/bin/sh
set -eu
if [ "${1:-}" = "--coverage" ]; then
  mkdir -p coverage
  printf '%s' '<?xml version="1.0"?><coverage line-rate="0.5" branch-rate="0" lines-covered="5" lines-valid="10" branches-covered="0" branches-valid="0" timestamp="0" version="1"><packages /></coverage>' > "$GIVN_COVERAGE_NON_E2E_PATH"
  exit 0
fi
mkdir -p target/debug
printf '#!/bin/sh\necho passuff\n' > target/debug/passuff
chmod +x target/debug/passuff
printf '#!/bin/sh\necho passuff-tui\n' > target/debug/passuff-tui
chmod +x target/debug/passuff-tui
names=""
for feature in givn/specs/*.feature; do
  part=$(awk '
    /^@e2e/ { e2e=1; next }
    /^  Scenario: / {
      if (e2e) next
      sub(/^  Scenario: /, "")
      printf "%s\"%s\"", sep, $0
      sep=","
    }
  ' "$feature")
  if [ -n "$part" ]; then
    if [ -n "$names" ]; then names="$names,$part"; else names="$part"; fi
  fi
done
printf '{"scope":"component","total":2,"passed":2,"failed":0,"skipped":0,"scenarios":[%s]}\n' "$names" > "$GIVN_RESULT_FILE"
"#;

const E2E_SCRIPT: &str = r#"#!/bin/sh
set -eu
[ -x "$PASSUFF_BIN_DIR/passuff" ]
[ -x "$PASSUFF_BIN_DIR/passuff-tui" ]
"$PASSUFF_BIN_DIR/passuff" >/dev/null
"$PASSUFF_BIN_DIR/passuff-tui" >/dev/null
names=$(awk '/^  Scenario: / { sub(/^  Scenario: /, ""); printf "%s\"%s\"", sep, $0; sep="," }' givn/specs/acceptance.feature)
digest=$(cd "$PASSUFF_BIN_DIR" && find . -type f -printf '%P\n' | sort | while IFS= read -r file; do
  printf '%s\0' "$file"
  if [ -x "$file" ]; then printf '1\0'; else printf '0\0'; fi
  cat "$file"
  printf '\0'
done | sha256sum | cut -d' ' -f1)
printf '{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s],"product":"%s","product_sha256":"%s"}\n' "$names" "$PASSUFF_BIN_DIR" "$digest" > "$GIVN_RESULT_FILE"
"#;

const WRONG_PRODUCT_E2E_SCRIPT: &str = r#"#!/bin/sh
set -eu
wrong="$PWD/target/debug"
mkdir -p "$wrong"
printf '#!/bin/sh\necho checkout\n' > "$wrong/passuff"
chmod +x "$wrong/passuff"
printf '#!/bin/sh\necho checkout-tui\n' > "$wrong/passuff-tui"
chmod +x "$wrong/passuff-tui"
names=$(awk '/^  Scenario: / { sub(/^  Scenario: /, ""); printf "%s\"%s\"", sep, $0; sep="," }' givn/specs/acceptance.feature)
digest=$(cd "$wrong" && find . -type f -printf '%P\n' | sort | while IFS= read -r file; do
  printf '%s\0' "$file"
  if [ -x "$file" ]; then printf '1\0'; else printf '0\0'; fi
  cat "$file"
  printf '\0'
done | sha256sum | cut -d' ' -f1)
printf '{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s],"product":"%s","product_sha256":"%s"}\n' "$names" "$wrong" "$digest" > "$GIVN_RESULT_FILE"
"#;

fn git(project: &Path, args: &[&str]) {
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
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).expect("write fixture");
}

fn fixture() -> (TempDir, PathBuf) {
    let root = tempfile::TempDir::new().unwrap();
    let project = root.path().join("project");
    std::fs::create_dir_all(&project).unwrap();
    git(&project, &["init", "-q"]);
    write(
        &project.join("givn/config.yaml"),
        "addons:\n  ise: true\n  coverage: true\n",
    );
    write(
        &project.join("givn/commands.yaml"),
        "verify:\n  command: \"sh ./scripts/verify.sh\"\n  e2e_command: \"sh ./scripts/e2e.sh\"\n  features_path: \"givn/specs\"\n  product_path: \"target/debug\"\n  product_env: \"PASSUFF_BIN_DIR\"\ncoverage:\n  measure_command: \"sh ./scripts/verify.sh --coverage\"\n  merge_command: \"true\"\n  non_e2e_path: \"coverage/non-e2e.xml\"\n  e2e_path: \"coverage/e2e.xml\"\n  output_path: \"coverage/coverage.xml\"\n  e2e_subprocess: true\n",
    );
    write(
        &project.join("givn/specification.yaml"),
        "behavior: [\"givn/specs/**\"]\nstructure: []\ncontracts: []\nlimits: []\ne2e: [\"crates/passuff-e2e\"]\nsource: [\"crates/**\", \"scripts/**\"]\n",
    );
    write(
        &project.join("givn/specs/component.feature"),
        "Feature: component\n\n  Scenario: one\n    Given nothing\n\n  Scenario: two\n    Given nothing\n",
    );
    write(
        &project.join("givn/specs/acceptance.feature"),
        "@e2e\nFeature: acceptance\n\n  Scenario: acceptance\n    Given nothing\n",
    );
    write(
        &project.join("crates/passuff-e2e/Cargo.toml"),
        "[package]\nname = \"passuff-e2e\"\n",
    );
    write(
        &project.join("crates/passuff-e2e/steps.rs"),
        "// e2e steps\n",
    );
    write(&project.join("scripts/e2e.sh"), E2E_SCRIPT);
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
    assert!(output.status.success(), "{}", stderr(&output));
    manifest
}

fn prepare(copy: &Path) {
    let path = copy.join("scripts/verify.sh");
    write(&path, VERIFY_SCRIPT);
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
fn a_passuff_shaped_copy_binds_two_binaries_and_accepts_a_missing_e2e_report() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    prepare(&copy);
    assert!(!copy.join("crates/passuff-e2e").exists());
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
    assert_eq!(record["clean"], true);
    assert_eq!(record["component"]["executed"], 2);
    assert_eq!(record["end_to_end"]["executed"], 1);
    assert_eq!(
        record["coverage"]["end_to_end"]["status"],
        "subprocess-uninstrumented"
    );
    assert_eq!(
        record["product"]["path"].as_str(),
        Some(copy.join("target/debug").to_string_lossy().as_ref())
    );
    assert_eq!(record["product"]["sha256"].as_str().map(str::len), Some(64));
}

#[test]
fn a_passuff_shaped_e2e_run_against_the_checkout_binaries_refuses() {
    let (root, project) = fixture();
    let manifest = rebuild(&project, root.path());
    let copy = root.path().join("copy");
    prepare(&copy);
    write(&project.join("scripts/e2e.sh"), WRONG_PRODUCT_E2E_SCRIPT);
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
        serde_json::from_str(&std::fs::read_to_string(&result).unwrap()).unwrap();
    assert_eq!(record["clean"], false);
}
