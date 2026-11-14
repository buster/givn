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
/// The ignored file the revision-read fixture writes into the working tree.
pub(crate) const IGNORED_PATH: &str = "ignored.txt";
/// The retained source file several partition fixtures commit.
pub(crate) const KEPT_PATH: &str = "src/kept.rs";
/// The retained source file's committed content.
pub(crate) const KEPT_CONTENT: &str = "pub fn kept() {}\n";

/// The unknown top-level key the malformed declaration carries.
pub(crate) const MALFORMED_KEY: &str = "component";

/// The declared `contracts` entry a revision can be missing.
pub(crate) const CONTRACT_PATH: &str = "docs/contract.md";
/// The tracked document a `--path` value can name outside the source roots.
pub(crate) const REFERENCE_PATH: &str = "docs/reference.md";
/// The retained feature several refusal fixtures commit.
pub(crate) const DEMO_FEATURE_PATH: &str = "givn/specs/demo.feature";
/// The retained feature's committed content.
pub(crate) const DEMO_FEATURE_CONTENT: &str =
    "Feature: demo\n  Scenario: demo\n    Given nothing\n";
/// The tracked source file a revision commits.
pub(crate) const SOURCE_PATH: &str = "src/one.rs";
/// The tracked source file's committed content.
pub(crate) const SOURCE_CONTENT: &str = "pub fn one() {}\n";

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
    /// The first committed revision of a two-revision fixture.
    pub(crate) first_revision: Option<String>,
    /// The revision whose blob the fixture made unreadable.
    pub(crate) unreadable_revision: Option<String>,
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

/// A declaration with the five retained roles empty and the named source
/// roots, so `--full` replaces every tracked source file.
pub(crate) fn source_only_declaration(source: &str) -> String {
    format!(
        "behavior: []\n\
         structure: []\n\
         contracts: []\n\
         limits: []\n\
         e2e: []\n\
         source: [{source}]\n"
    )
}

/// A declaration whose `structure` role names exactly `retained`, with the
/// other retained roles empty and one source root.
pub(crate) fn structure_declaration(retained: &str, source: &str) -> String {
    format!(
        "behavior: []\n\
         structure: [{retained}]\n\
         contracts: []\n\
         limits: []\n\
         e2e: []\n\
         source: [{source}]\n"
    )
}

/// A declaration whose `behavior` role retains `behavior` and whose source
/// root is `source`.
pub(crate) fn behavior_declaration(behavior: &str, source: &str) -> String {
    format!(
        "behavior: [{behavior}]\n\
         structure: []\n\
         contracts: []\n\
         limits: []\n\
         e2e: []\n\
         source: [{source}]\n"
    )
}

/// A declaration whose `contracts` role names exactly `retained`, with the
/// other retained roles empty and one source root.
pub(crate) fn contracts_declaration(retained: &str, source: &str) -> String {
    format!(
        "behavior: []\n\
         structure: []\n\
         contracts: [{retained}]\n\
         limits: []\n\
         e2e: []\n\
         source: [{source}]\n"
    )
}

/// The declaration with one unknown top-level key beside the six valid keys.
pub(crate) fn malformed_declaration() -> String {
    format!("{MALFORMED_KEY}: [\"x\"]\n{}", default_declaration())
}

/// An initialized Git project with `ise` enabled and a committed valid
/// declaration.
pub(crate) fn ise_project() -> common::TestEnv {
    ise_project_with_declaration(&default_declaration())
}

/// An initialized Git project with `ise` enabled and the supplied committed
/// declaration.
pub(crate) fn ise_project_with_declaration(declaration: &str) -> common::TestEnv {
    let env = common::make_init_root();
    enable_ise(&env);
    write_declaration(&env, declaration);
    common::git_commit_all(&env, "fixture: ise project");
    env
}

/// Rewrite the committed declaration in place; the caller commits it.
pub(crate) fn rewrite_declaration(env: &common::TestEnv, content: &str) {
    write_declaration(env, content);
}

/// A project with one source root and a committed revision carrying a source
/// file and a document outside the source root, so a `--path` value resolves
/// to tracked content that no source root covers.
pub(crate) fn source_root_project() -> common::TestEnv {
    let env = ise_project_with_declaration(&source_only_declaration("\"src/**\""));
    commit_files(
        &env,
        &[
            (REFERENCE_PATH, "reference\n"),
            (SOURCE_PATH, SOURCE_CONTENT),
        ],
        "fixture: source root revision",
    );
    env
}

/// A project whose declaration retains `givn/specs/**` and whose revision
/// commits a retained feature, so the role matches at the revision.
pub(crate) fn retained_behavior_project() -> common::TestEnv {
    let env =
        ise_project_with_declaration(&behavior_declaration("\"givn/specs/**\"", "\"src/**\""));
    commit_demo_feature(&env);
    env
}

/// A project whose `contracts` role names a document the declaration commits
/// nowhere; the revision read is the caller's fixture.
pub(crate) fn contracts_project() -> common::TestEnv {
    ise_project_with_declaration(&contracts_declaration(
        &format!("{CONTRACT_PATH:?}"),
        "\"src/**\"",
    ))
}

/// Commit the retained feature the role fixtures name.
pub(crate) fn commit_demo_feature(env: &common::TestEnv) {
    commit_files(
        env,
        &[(DEMO_FEATURE_PATH, DEMO_FEATURE_CONTENT)],
        "fixture: retained behavior",
    );
}

/// Commit a tracked source file only, so the declared contract stays absent
/// from the revision.
pub(crate) fn commit_revision_without_contract(env: &common::TestEnv) {
    commit_files(
        env,
        &[(SOURCE_PATH, SOURCE_CONTENT)],
        "fixture: revision without the contract",
    );
}

/// Write each file (creating its parent directories) and commit the whole
/// tree. Returns the new revision identifier.
pub(crate) fn commit_files(env: &common::TestEnv, files: &[(&str, &str)], message: &str) -> String {
    for (path, content) in files {
        let target = env.path(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).expect("create the fixture parent directory");
        }
        std::fs::write(&target, content).expect("write the fixture file");
    }
    common::git_commit_all(env, message);
    head_revision(env)
}

/// The current revision identifier.
pub(crate) fn head_revision(env: &common::TestEnv) -> String {
    let output = common::git(env, &["rev-parse", "HEAD"]);
    assert!(
        output.status.success(),
        "git rev-parse HEAD failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Commit `KEPT_PATH` with its retained `structure` role, then delete the
/// loose blob so the revision tree lists a path with no readable object.
pub(crate) fn remove_kept_blob(env: &common::TestEnv) -> String {
    rewrite_declaration(
        env,
        &structure_declaration(&format!("{KEPT_PATH:?}"), "\"src/**\""),
    );
    let revision = commit_files(env, &[(KEPT_PATH, KEPT_CONTENT)], "fixture: kept file");
    let object = common::git(env, &["rev-parse", &format!("{revision}:{KEPT_PATH}")]);
    assert!(
        object.status.success(),
        "git rev-parse for the kept blob failed: {}",
        String::from_utf8_lossy(&object.stderr)
    );
    let id = String::from_utf8_lossy(&object.stdout).trim().to_string();
    let loose = env
        .root()
        .join(".git/objects")
        .join(&id[..2])
        .join(&id[2..]);
    std::fs::remove_file(&loose).expect("remove the loose blob object");
    revision
}

/// Commit a `.gitignore` rule and then create the ignored file untracked.
pub(crate) fn ignore_note(env: &common::TestEnv) {
    std::fs::write(env.path(".gitignore"), format!("{IGNORED_PATH}\n"))
        .expect("write the .gitignore fixture");
    common::git_commit_all(env, "fixture: ignore the note");
    std::fs::write(env.path(IGNORED_PATH), "ignored\n").expect("write the ignored file");
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
