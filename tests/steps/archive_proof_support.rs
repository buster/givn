//! Shared fixtures and assertions for the archive-proof scenarios.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Output;

use crate::common;
use crate::steps::GivnWorld;

pub(crate) const RESULT_RUNNER: &str = "./emit-result.sh";

pub(crate) fn env(world: &GivnWorld) -> &common::TestEnv {
    world.env.as_ref().expect("a fixture project is required")
}

pub(crate) fn root(world: &GivnWorld) -> PathBuf {
    env(world).root()
}

/// The project directory the scenario operates on: the nested-project
/// override when set, otherwise the fixture root.
pub(crate) fn project_base(world: &GivnWorld) -> PathBuf {
    world.cwd_override.clone().unwrap_or_else(|| root(world))
}

pub(crate) fn archive_dir(world: &GivnWorld, id: &str) -> PathBuf {
    root(world).join(format!("givn/archive/{id}"))
}

pub(crate) fn receipt(world: &GivnWorld, id: &str) -> serde_json::Value {
    let path = archive_dir(world, id).join("verification.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read receipt {}: {e}", path.display()));
    serde_json::from_str(&text).expect("receipt is valid JSON")
}

pub(crate) fn receipt_status(world: &GivnWorld, id: &str) -> String {
    receipt(world, id)["status"]
        .as_str()
        .unwrap_or("missing")
        .to_string()
}

/// Create the fixture project if the scenario has none.
pub(crate) fn ensure_env(world: &mut GivnWorld) {
    ensure_env_on(world, common::BackendKind::Git);
}

/// Create the fixture project on the named backend if the scenario has none.
pub(crate) fn ensure_env_on(world: &mut GivnWorld, backend: common::BackendKind) {
    if world.env.is_none() {
        world.env = Some(match backend {
            common::BackendKind::Git => common::make_init_root(),
            common::BackendKind::Fossil => common::make_fossil_root(),
        });
    }
}

/// Write the executable result runner. `kind` selects the trustworthiness of
/// the emitted scope result (see the shell script below).
pub(crate) fn write_result_runner(env: &common::TestEnv, kind: &str) {
    let script = format!(
        r#"#!/bin/sh
# archive-proof result runner
kind={kind}
scope=${{1:-regular}}
if [ -z "${{GIVN_RESULT_FILE:-}}" ]; then
    exit 0
fi
printf '%s %s\n' "$scope" "${{GIVN_FEATURES:-}}" >> "$(dirname "$GIVN_RESULT_FILE")/invocations.log"
case "$kind" in
    fail) exit 1 ;;
    missing) exit 0 ;;
    malformed) printf '{{"scope":' > "$GIVN_RESULT_FILE"; exit 0 ;;
    failed) total=2; passed=1; failed=1; skipped=0 ;;
    skipped) total=2; passed=1; failed=0; skipped=1 ;;
    zero) total=0; passed=0; failed=0; skipped=0 ;;
    unreconciled) total=3; passed=2; failed=0; skipped=0 ;;
    wrong-scope)
        if [ "$scope" = regular ]; then scope=e2e; else scope=regular; fi
        total=2; passed=2; failed=0; skipped=0 ;;
    record) total=2; passed=2; failed=0; skipped=0 ;;
    *) total=2; passed=2; failed=0; skipped=0 ;;
esac
printf '{{"scope":"%s","total":%s,"passed":%s,"failed":%s,"skipped":%s}}\n' \
    "$scope" "$total" "$passed" "$failed" "$skipped" > "$GIVN_RESULT_FILE"
"#
    );
    let path = env.path("emit-result.sh");
    fs::write(&path, script).expect("write result runner");
    let mut permissions = fs::metadata(&path)
        .expect("result runner metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("make result runner executable");
}

/// Point `verify.command` and `verify.e2e_command` at the result runner.
pub(crate) fn configure_commands(env: &common::TestEnv) {
    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        &format!(
            "verify:\n  command: \"{RESULT_RUNNER} regular\"\n  e2e_command: \"{RESULT_RUNNER} e2e\"\n"
        ),
    );
}

pub(crate) fn disable_coverage(env: &common::TestEnv) {
    let path = env.path("givn/config.yaml");
    let config = fs::read_to_string(&path).expect("read config");
    fs::write(&path, config.replace("coverage: true", "coverage: false")).expect("write config");
}

pub(crate) fn enable_coverage(env: &common::TestEnv) {
    let path = env.path("givn/config.yaml");
    let config = fs::read_to_string(&path).expect("read config");
    fs::write(&path, config.replace("coverage: false", "coverage: true")).expect("write config");
    let commands = env.path("givn/commands.yaml");
    let content = fs::read_to_string(&commands).expect("read commands");
    if !content.contains("coverage:") {
        let block = "\ncoverage:\n  measure_command: \"./emit-coverage.sh\"\n  \
                     merge_command: \"./merge-coverage.sh\"\n  \
                     non_e2e_path: \"coverage/non-e2e-cobertura.xml\"\n  \
                     e2e_path: \"coverage/e2e-cobertura.xml\"\n  \
                     output_path: \"coverage/cobertura-coverage.xml\"\n";
        fs::write(&commands, format!("{content}{block}")).expect("write commands");
    }
}

/// Coverage scripts. `measure_fails` makes measurement exit non-zero;
/// `undeclared_file` makes the merge write a source file outside the
/// declared evidence paths.
pub(crate) fn write_coverage_scripts(
    env: &common::TestEnv,
    measure_fails: bool,
    undeclared_file: bool,
) {
    let report = |covered: u64| {
        format!(
            r#"<?xml version="1.0"?>
<coverage line-rate="{}" branch-rate="0" lines-covered="{covered}" lines-valid="4" branches-covered="0" branches-valid="0" timestamp="0" version="1">
  <packages />
</coverage>"#,
            covered as f64 / 4.0
        )
    };
    let measure = if measure_fails {
        "#!/bin/sh\nexit 1\n".to_string()
    } else {
        format!(
            "#!/bin/sh\nmkdir -p coverage\ncat > \"$GIVN_COVERAGE_NON_E2E_PATH\" <<'XML'\n{}\nXML\ncat > \"$GIVN_COVERAGE_E2E_PATH\" <<'XML'\n{}\nXML\n",
            report(2),
            report(3)
        )
    };
    let undeclared = if undeclared_file {
        "mkdir -p src\nprintf 'pub fn extra() {}\\n' > src/extra.rs\n"
    } else {
        ""
    };
    let merge = format!(
        "#!/bin/sh\n{undeclared}mkdir -p coverage\ncp \"$GIVN_COVERAGE_NON_E2E_PATH\" \"$GIVN_COVERAGE_OUTPUT_PATH\"\n"
    );
    for (name, content) in [("emit-coverage.sh", measure), ("merge-coverage.sh", merge)] {
        let path = env.path(name);
        fs::write(&path, content).expect("write coverage script");
        let mut permissions = fs::metadata(&path)
            .expect("coverage script metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("make coverage script executable");
    }
}

/// Write every artifact the archive gate requires, plus the delta spec.
pub(crate) fn write_change_artifacts(env: &common::TestEnv, id: &str, with_e2e: bool) {
    write_change_artifacts_at(&env.root(), id, with_e2e);
}

/// Same as [`write_change_artifacts`] but rooted at an arbitrary project
/// directory (the nested-project scenario).
pub(crate) fn write_change_artifacts_at(base: &Path, id: &str, with_e2e: bool) {
    let change = base.join(format!("givn/changes/{id}"));
    fs::create_dir_all(&change).expect("create change dir");
    fs::write(
        change.join("proposal.md"),
        crate::steps::fixture_routing_proposal("demo-cap"),
    )
    .expect("write proposal");
    let specs = change.join("specs/demo-cap");
    fs::create_dir_all(&specs).expect("create delta specs dir");
    let e2e = if with_e2e {
        "\n  @givn.added @e2e\n  Scenario: Demo e2e scenario\n    Given a thing\n    Then it works\n"
    } else {
        ""
    };
    fs::write(
        specs.join("demo-cap.feature"),
        format!(
            "@givn.delta @demo-cap\nFeature: Demo Capability\n\n  @givn.added\n  Scenario: Demo scenario\n    Given a thing\n    Then it works\n{e2e}"
        ),
    )
    .expect("write delta feature");
    fs::write(change.join("design.md"), "# Design\n").expect("write design");
    fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design-review");
    fs::write(change.join("tasks.md"), "- [x] done\n").expect("write tasks");
    fs::write(
        change.join("review.md"),
        "REVIEW: PASS\nREADME-IMPACT: none\n",
    )
    .expect("write review");
    fs::write(change.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").expect("write arc42");
}

/// Full archive-ready fixture: initialized committed project, result runner,
/// commands, and a completed change.
pub(crate) fn ready_change(world: &mut GivnWorld, id: &str, with_e2e: bool, kind: &str) {
    ready_change_on(world, common::BackendKind::Git, id, with_e2e, kind);
}

/// Same as [`ready_change`] but on the named backend.
pub(crate) fn ready_change_on(
    world: &mut GivnWorld,
    backend: common::BackendKind,
    id: &str,
    with_e2e: bool,
    kind: &str,
) {
    ensure_env_on(world, backend);
    let env = env(world);
    disable_coverage(env);
    write_result_runner(env, kind);
    configure_commands(env);
    write_change_artifacts(env, id, with_e2e);
    common::commit_fixture(env);
    world.last_change_id = Some(id.to_string());
}

/// Parse the backend step value used by the backend-parameterized Givens.
pub(crate) fn backend_kind(value: &str) -> common::BackendKind {
    match value {
        "Git" => common::BackendKind::Git,
        "Fossil" => common::BackendKind::Fossil,
        other => panic!("unknown backend step value '{other}'"),
    }
}

/// Paths changed by the current revision, relative to the project root.
pub(crate) fn revision_paths(world: &GivnWorld) -> Vec<String> {
    let env = env(world);
    match env.backend() {
        common::BackendKind::Git => {
            let out = common::git(env, &["show", "--name-only", "--format=", "HEAD"]);
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(str::to_string)
                .collect()
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(env, &["diff", "--brief", "--checkin", "current"]);
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|line| {
                    let rest = line.split_once(char::is_whitespace)?.1.trim();
                    if rest.is_empty() {
                        None
                    } else {
                        Some(rest.to_string())
                    }
                })
                .collect()
        }
    }
}

/// The current revision identifier.
pub(crate) fn current_revision(world: &GivnWorld) -> String {
    let env = env(world);
    match env.backend() {
        common::BackendKind::Git => {
            let out = common::git(env, &["rev-parse", "HEAD"]);
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(env, &["info"]);
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .find_map(|line| line.strip_prefix("checkout:"))
                .and_then(|rest| rest.split_whitespace().next())
                .unwrap_or_default()
                .to_string()
        }
    }
}

/// The revision before the current one (the tested revision when the current
/// revision is the evidence-only finalization).
pub(crate) fn parent_revision(world: &GivnWorld) -> String {
    let env = env(world);
    match env.backend() {
        common::BackendKind::Git => {
            let out = common::git(env, &["rev-parse", "HEAD~1"]);
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(env, &["info"]);
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .find_map(|line| line.strip_prefix("parent:"))
                .and_then(|rest| rest.split_whitespace().next())
                .unwrap_or_default()
                .to_string()
        }
    }
}

/// Changed paths in the checkout, including untracked/`EXTRA` entries.
pub(crate) fn dirty_entries(world: &GivnWorld) -> Vec<String> {
    let env = env(world);
    match env.backend() {
        common::BackendKind::Git => {
            let out = common::git(env, &["status", "--porcelain", "-uall"]);
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|line| line.get(3..).map(str::to_string))
                .collect()
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(env, &["changes", "--differ"]);
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|line| {
                    let rest = line.split_once(char::is_whitespace)?.1.trim();
                    if rest.is_empty() {
                        return None;
                    }
                    Some(
                        rest.split_once("->")
                            .map(|(_, destination)| destination.trim().to_string())
                            .unwrap_or_else(|| rest.to_string()),
                    )
                })
                .collect()
        }
    }
}

/// Whether the current revision contains `path`.
pub(crate) fn revision_contains(world: &GivnWorld, path: &str) -> bool {
    match env(world).backend() {
        common::BackendKind::Git => revision_contains_at(world, "HEAD", path),
        common::BackendKind::Fossil => revision_contains_at(world, "current", path),
    }
}

/// Whether `path` is present in the named revision.
pub(crate) fn revision_contains_at(world: &GivnWorld, revision: &str, path: &str) -> bool {
    let env = env(world);
    match env.backend() {
        common::BackendKind::Git => {
            common::git(env, &["cat-file", "-e", &format!("{revision}:{path}")])
                .status
                .success()
        }
        common::BackendKind::Fossil => common::fossil(env, &["cat", "-r", revision, path])
            .status
            .success(),
    }
}

/// Run `givn archive --change <id>` from the fixture.
pub(crate) fn run_archive(world: &GivnWorld, id: &str) -> Output {
    let env = env(world);
    common::run_givn(env, &["archive", "--change", id])
}

pub(crate) fn staged_paths(world: &GivnWorld) -> Vec<String> {
    let out = common::git(env(world), &["diff", "--cached", "--name-only"]);
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

pub(crate) fn last_commit_paths(world: &GivnWorld) -> Vec<String> {
    let out = common::git(env(world), &["show", "--name-only", "--format=", "HEAD"]);
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_string)
        .collect()
}
