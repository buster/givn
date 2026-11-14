//! Fixture builders for the clean-room preparation and verification scenarios.
//!
//! The builders create real Git and Fossil projects, declarations, revisions,
//! dirty states, and write targets; the steps drive the built binary and read
//! the fixture tree. No builder imports the `ise` module.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::common;

/// The tracked file the dirty-checkout fixture modifies.
pub(crate) const MODIFIED_PATH: &str = "src/modified.rs";
/// The untracked file the dirty-checkout fixture adds.
pub(crate) const UNTRACKED_PATH: &str = "notes-untracked.md";

/// The unknown top-level key the malformed declaration carries.
pub(crate) const MALFORMED_KEY: &str = "component";

/// Per-scenario state for the `clean-room-preparation` fixtures.
#[derive(Debug, Default)]
pub(crate) struct IseCleanRoomState {
    /// The container whose child paths are the trial and the receipt; the
    /// container exists so both targets are outside the checkout and absent.
    pub(crate) targets: Option<TempDir>,
    pub(crate) trial_path: Option<PathBuf>,
    pub(crate) receipt_path: Option<PathBuf>,
    /// The revision the When passed to `--at`.
    pub(crate) requested_revision: Option<String>,
    /// The nested project root a refusal must name.
    pub(crate) nested_project_root: Option<PathBuf>,
}

impl IseCleanRoomState {
    /// Create one fresh container and the two absent write targets.
    pub(crate) fn ensure_targets(&mut self) {
        if self.targets.is_some() {
            return;
        }
        let container = TempDir::new().expect("create a trial target container");
        self.trial_path = Some(container.path().join("trial"));
        self.receipt_path = Some(container.path().join("record.yaml"));
        self.targets = Some(container);
    }

    pub(crate) fn trial_path(&self) -> &Path {
        self.trial_path
            .as_deref()
            .expect("the When created the targets")
    }

    pub(crate) fn receipt_path(&self) -> &Path {
        self.receipt_path
            .as_deref()
            .expect("the When created the targets")
    }
}

/// A declaration with the five roles present and the named behavior and
/// source lists. The other roles stay fixed so a mutation is attributable.
pub(crate) fn declaration(behavior: &str, source: &str) -> String {
    format!(
        "behavior: [{behavior}]\n\
         structure: [\"docs/glossary.md\"]\n\
         contracts: [\"docs/reference.md\"]\n\
         limits: [\"docs/arc42/02-architecture-constraints.md\"]\n\
         e2e: [\"tests/e2e\"]\n\
         source: [{source}]\n"
    )
}

/// The declaration the admission fixtures start from.
pub(crate) fn default_declaration() -> String {
    declaration("\"givn/specs/**\"", "\"src/**\"")
}

/// The declaration with one unknown top-level key beside the six valid keys.
pub(crate) fn malformed_declaration() -> String {
    format!("{MALFORMED_KEY}: [\"x\"]\n{}", default_declaration())
}

/// An initialized Git project with `ise` enabled and a committed valid
/// declaration.
pub(crate) fn ise_project() -> common::TestEnv {
    let env = common::make_init_root();
    enable_ise(&env);
    write_declaration(&env, &default_declaration());
    common::git_commit_all(&env, "fixture: ise project");
    env
}

/// The same project without `givn/specification.yaml`.
pub(crate) fn ise_project_without_declaration() -> common::TestEnv {
    let env = ise_project();
    std::fs::remove_file(env.path("givn/specification.yaml"))
        .expect("remove the declaration fixture");
    common::git_commit_all(&env, "fixture: no declaration");
    env
}

/// A Git project whose declaration carries an unknown top-level key.
pub(crate) fn malformed_declaration_project() -> common::TestEnv {
    let env = ise_project();
    write_declaration(&env, &malformed_declaration());
    common::git_commit_all(&env, "fixture: malformed declaration");
    env
}

/// A Git project with `ise` off and a malformed declaration on disk, so a
/// reader that loads the declaration first would report the wrong refusal.
pub(crate) fn ise_off_project() -> common::TestEnv {
    let env = common::make_init_root();
    set_addon(&env, "ise", false);
    write_declaration(&env, &malformed_declaration());
    common::git_commit_all(&env, "fixture: ise off");
    env
}

/// A Git project with `ise` on and its `architecture_principles`
/// prerequisite off (the CLI refuses that combination, so the config is
/// written directly).
pub(crate) fn architecture_principles_off_project() -> common::TestEnv {
    let env = ise_project();
    set_addon(&env, "architecture_principles", false);
    common::git_commit_all(&env, "fixture: architecture principles off");
    env
}

/// A clean project made dirty by one modified tracked file and one untracked,
/// non-ignored file.
pub(crate) fn dirty_project() -> common::TestEnv {
    let env = ise_project();
    std::fs::create_dir_all(env.path("src")).expect("create src");
    std::fs::write(env.path(MODIFIED_PATH), "pub fn before() {}\n")
        .expect("write the tracked file");
    common::git_commit_all(&env, "fixture: tracked file");
    std::fs::write(env.path(MODIFIED_PATH), "pub fn after() {}\n")
        .expect("modify the tracked file");
    std::fs::write(env.path(UNTRACKED_PATH), "untracked\n").expect("write the untracked file");
    env
}

/// A plain directory that is no Git worktree and no Fossil checkout.
pub(crate) fn no_backend_project() -> common::TestEnv {
    let env = common::make_init_root_without_git();
    set_addon(&env, "ise", true);
    write_declaration(&env, &default_declaration());
    std::fs::create_dir_all(env.path("src")).expect("create src");
    std::fs::write(env.path("src/kept.rs"), "pub fn kept() {}\n").expect("write a file");
    env
}

/// A Git worktree whose Givn project root is nested below the top level.
pub(crate) fn nested_project() -> (common::TestEnv, PathBuf) {
    let env = common::make_bare_root();
    let top = env.path("top");
    let project = top.join("nested/project");
    std::fs::create_dir_all(&project).expect("create the nested project root");
    git_in(&top, &["init", "-q"]);
    for args in [
        ["config", "user.email", "fixture@example.com"],
        ["config", "user.name", "Givn Fixture"],
        ["config", "commit.gpgsign", "false"],
    ] {
        git_in(&top, &args);
    }
    let init =
        common::run_givn_with_envs_in(&env, &project, &["init", "--agents", "opencode"], &[]);
    assert!(
        init.status.success(),
        "givn init in the nested project failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    let enable = common::run_givn_with_envs_in(&env, &project, &["addons", "enable", "ise"], &[]);
    assert!(
        enable.status.success(),
        "givn addons enable ise in the nested project failed: {}",
        String::from_utf8_lossy(&enable.stderr)
    );
    std::fs::write(
        project.join("givn/specification.yaml"),
        default_declaration(),
    )
    .expect("write the nested declaration");
    git_in(&top, &["add", "-A"]);
    git_in(&top, &["commit", "-q", "-m", "fixture: nested project"]);
    (env, project)
}

/// A Git project carrying a Fossil checkout marker at the same root.
pub(crate) fn mixed_markers_project() -> common::TestEnv {
    let env = ise_project();
    std::fs::write(env.path(".fslckout"), "").expect("write the Fossil marker");
    env
}

fn enable_ise(env: &common::TestEnv) {
    let output = common::run_givn(env, &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn write_declaration(env: &common::TestEnv, content: &str) {
    std::fs::write(env.path("givn/specification.yaml"), content)
        .expect("write the declaration fixture");
}

/// Replace or insert one add-on key in the `addons:` block, leaving every
/// other config line untouched.
pub(crate) fn set_addon(env: &common::TestEnv, name: &str, value: bool) {
    let path = env.path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let key = format!("  {name}: ");
    if let Some(position) = lines.iter().position(|line| line.starts_with(&key)) {
        lines[position] = format!("  {name}: {value}");
    } else if let Some(position) = lines.iter().position(|line| line == "addons:") {
        lines.insert(position + 1, format!("  {name}: {value}"));
    } else {
        lines.push("addons:".to_string());
        lines.push(format!("  {name}: {value}"));
    }
    std::fs::write(&path, lines.join("\n") + "\n").expect("write givn/config.yaml");
}

fn git_in(directory: &Path, args: &[&str]) -> std::process::Output {
    let output = Command::new("git")
        .args(args)
        .current_dir(directory)
        .output()
        .expect("spawn git");
    assert!(
        output.status.success(),
        "git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}
