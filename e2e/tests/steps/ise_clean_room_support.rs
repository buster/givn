//! Fixture builders for the clean-room preparation and verification scenarios.
//!
//! The builders create real Git and Fossil projects, declarations, revisions,
//! dirty states, and write targets; the steps drive the built binary and read
//! the fixture tree. No builder imports the `ise` module.

use crate::harness as common;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

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
/// The declaration bytes the non-UTF-8 fixture commits at its first revision.
pub(crate) const NON_UTF8_DECLARATION_BYTES: &[u8] = b"behavior: [\"givn/specs/**\"]\n\xff\n";

/// The declaration every trial retains as an implicit entry.
pub(crate) const DECLARATION_PATH: &str = "givn/specification.yaml";

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
/// The bytes the existing-target-file fixture writes.
pub(crate) const TARGET_FILE_CONTENT: &str = "existing target\n";
/// The bytes the existing-manifest fixture writes; the scope is the settled
/// one so a create-new refusal, not a parse failure, is under test.
pub(crate) const MANIFEST_CONTENT: &str = "scope: ise-manifest/v1\n";
/// The declared `structure` entry a repository-link revision carries.
pub(crate) const VENDOR_LINK_PATH: &str = "vendor/lib";
/// The nested repository marker a revision can carry.
pub(crate) const NESTED_MARKER_PATH: &str = "nested/_FOSSIL_";

/// The executable script the entry-kind fixtures commit.
pub(crate) const RUN_SCRIPT_PATH: &str = "src/run.sh";
/// The symlink the entry-kind fixtures commit.
pub(crate) const SYMLINK_PATH: &str = "src/link";
/// The symlink's recorded target, relative to its directory.
pub(crate) const SYMLINK_TARGET: &str = "run.sh";
/// The behavior feature the Fossil entry-kind fixture retains.
pub(crate) const FOSSIL_FEATURE_PATH: &str = "givn/specs/demo/spec.feature";
/// The behavior feature's content; one scenario is tagged `@e2e`.
pub(crate) const FOSSIL_FEATURE_CONTENT: &str =
    "Feature: demo\n\n  @e2e\n  Scenario: demo\n    Given nothing\n";
/// The kept source file the Fossil entry-kind fixture commits.
pub(crate) const KEPT_SOURCE_PATH: &str = "src/other.rs";
/// The replaced source file the Fossil entry-kind fixture commits.
pub(crate) const REPLACED_SOURCE_PATH: &str = "src/keys/parser.rs";

/// Per-scenario state for the `clean-room-preparation` fixtures.
#[derive(Debug, Default)]
pub(crate) struct IseCleanRoomState {
    /// The container whose child paths are the spec copy and the manifest;
    /// the container exists so both targets are outside the checkout and
    /// absent.
    pub(crate) targets: Option<TempDir>,
    pub(crate) trial_path: Option<PathBuf>,
    pub(crate) manifest_path: Option<PathBuf>,
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
        self.manifest_path = Some(container.path().join("manifest.yaml"));
        self.targets = Some(container);
    }

    pub(crate) fn trial_path(&self) -> &Path {
        self.trial_path
            .as_deref()
            .expect("the When created the targets")
    }

    pub(crate) fn manifest_path(&self) -> &Path {
        self.manifest_path
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
/// declaration, plus one tracked file per declared retained role so every
/// entry matches at the revision.
pub(crate) fn ise_project() -> common::TestEnv {
    let env = ise_project_with_declaration(&default_declaration());
    commit_files(
        &env,
        &[
            (DEMO_FEATURE_PATH, DEMO_FEATURE_CONTENT),
            ("docs/glossary.md", "glossary\n"),
            (REFERENCE_PATH, "reference\n"),
            ("docs/arc42/02-architecture-constraints.md", "constraints\n"),
            ("tests/e2e/Cargo.toml", "[package]\nname = \"e2e\"\n"),
        ],
        "fixture: retained roles",
    );
    env
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

/// A minimal Git project with `ise` enabled and the supplied declaration
/// uncommitted: the `givn init` scaffold is absent, so only the paths the
/// scenario commits enter the revision and the placement counts stay exact.
pub(crate) fn bare_ise_project(declaration: &str) -> common::TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    std::fs::write(env.path(".git/info/exclude"), "givn/config.yaml\n")
        .expect("write the Git exclude fixture");
    std::fs::create_dir_all(env.path("givn")).expect("create givn/");
    std::fs::write(env.path("givn/config.yaml"), "addons:\n  ise: true\n")
        .expect("write the fixture config");
    std::fs::write(env.path("givn/specification.yaml"), declaration)
        .expect("write the fixture declaration");
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

/// Create the trial target as an existing empty directory.
pub(crate) fn existing_target_directory(state: &mut IseCleanRoomState) {
    state.ensure_targets();
    std::fs::create_dir(state.trial_path()).expect("create the existing target directory");
}

/// Create the trial target as an existing regular file holding known bytes.
pub(crate) fn existing_target_file(state: &mut IseCleanRoomState) {
    state.ensure_targets();
    std::fs::write(state.trial_path(), TARGET_FILE_CONTENT)
        .expect("write the existing target file");
}

/// Create the manifest as an existing regular file holding known bytes.
pub(crate) fn existing_manifest(state: &mut IseCleanRoomState) {
    state.ensure_targets();
    std::fs::write(state.manifest_path(), MANIFEST_CONTENT).expect("write the existing manifest");
}

/// Point the trial at `<container>/link/trial`, where `link` is a symlink to
/// the checkout root, so a prefix check over the supplied path would pass.
#[cfg(unix)]
pub(crate) fn symlinked_target(state: &mut IseCleanRoomState, checkout: &Path) {
    state.ensure_targets();
    let container = state
        .targets
        .as_ref()
        .expect("the targets exist")
        .path()
        .to_path_buf();
    let link = container.join("link");
    std::os::unix::fs::symlink(checkout, &link).expect("create the symlink into the checkout");
    state.trial_path = Some(link.join("trial"));
}

/// The symlinked-target fixture needs symlink support; refuse on a host
/// without it rather than assert a path the fixture could not build.
#[cfg(not(unix))]
pub(crate) fn symlinked_target(_state: &mut IseCleanRoomState, _checkout: &Path) {
    panic!("the symlinked-target fixture requires symlink support");
}

/// A project whose `structure` role retains the repository-link path.
pub(crate) fn repository_link_project() -> common::TestEnv {
    ise_project_with_declaration(&structure_declaration(
        &format!("{VENDOR_LINK_PATH:?}"),
        "\"src/**\"",
    ))
}

/// Record a `160000` index entry at the repository-link path and commit it,
/// without cloning a second repository.
pub(crate) fn commit_repository_link(env: &common::TestEnv) {
    let target = head_revision(env);
    let info = format!("160000,{target},{VENDOR_LINK_PATH}");
    let index = common::git(env, &["update-index", "--add", "--cacheinfo", &info]);
    assert!(
        index.status.success(),
        "git update-index for the repository link failed: {}",
        String::from_utf8_lossy(&index.stderr)
    );
    let commit = common::git(env, &["commit", "-q", "-m", "fixture: repository link"]);
    assert!(
        commit.status.success(),
        "git commit of the repository link failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
}

/// A project whose `structure` role retains the nested repository marker.
pub(crate) fn nested_marker_project() -> common::TestEnv {
    ise_project_with_declaration(&structure_declaration(
        &format!("{NESTED_MARKER_PATH:?}"),
        "\"src/**\"",
    ))
}

/// Commit the nested marker as a regular file.
pub(crate) fn commit_nested_marker(env: &common::TestEnv) {
    commit_files(
        env,
        &[(NESTED_MARKER_PATH, "nested marker fixture\n")],
        "fixture: nested marker",
    );
}

/// Commit an executable script and a symlink to it, so one Git revision
/// carries both entry kinds.
#[cfg(unix)]
pub(crate) fn commit_executable_and_symlink(env: &common::TestEnv) {
    use std::os::unix::fs::PermissionsExt;

    let script = env.path(RUN_SCRIPT_PATH);
    std::fs::create_dir_all(script.parent().expect("the script has a parent"))
        .expect("create the script directory");
    std::fs::write(&script, "#!/bin/sh\necho fixture\n").expect("write the executable script");
    let mut permissions = std::fs::metadata(&script)
        .expect("read the script mode")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&script, permissions).expect("set the executable bit");
    std::os::unix::fs::symlink(SYMLINK_TARGET, env.path(SYMLINK_PATH))
        .expect("create the fixture symlink");
    common::git_commit_all(env, "fixture: executable and symlink");
}

/// The entry-kind fixture needs symlink support; refuse on a host without it
/// rather than assert a kind the fixture could not build.
#[cfg(not(unix))]
pub(crate) fn commit_executable_and_symlink(_env: &common::TestEnv) {
    panic!("the entry-kind fixture requires symlink support");
}

/// A Fossil checkout with `ise` enabled, `allow-symlinks on`, and a
/// declaration that retains `givn/specs/**` and sources `src/**`. The
/// setting runs before any symlink is added, so Fossil records the `l` flag.
pub(crate) fn fossil_kind_project() -> common::TestEnv {
    let env = common::make_fossil_root();
    enable_ise(&env);
    write_declaration(
        &env,
        &behavior_declaration("\"givn/specs/**\"", "\"src/**\""),
    );
    let settings = common::fossil(&env, &["settings", "allow-symlinks", "on"]);
    assert!(
        settings.status.success(),
        "fossil settings allow-symlinks on failed: {}",
        String::from_utf8_lossy(&settings.stderr)
    );
    common::fossil_commit_all(&env, "fixture: fossil declaration");
    env
}

/// Commit the retained feature, kept source, replaced source, executable
/// script, and symlink the Fossil entry-kind scenario reads.
#[cfg(unix)]
pub(crate) fn commit_fossil_kinds(env: &common::TestEnv) {
    use std::os::unix::fs::PermissionsExt;

    for (path, content) in [
        (FOSSIL_FEATURE_PATH, FOSSIL_FEATURE_CONTENT),
        (KEPT_SOURCE_PATH, "pub fn other() {}\n"),
        (REPLACED_SOURCE_PATH, "pub fn parse() {}\n"),
    ] {
        let target = env.path(path);
        std::fs::create_dir_all(target.parent().expect("the file has a parent"))
            .expect("create the fixture parent directory");
        std::fs::write(&target, content).expect("write the fixture file");
    }
    let script = env.path(RUN_SCRIPT_PATH);
    std::fs::write(&script, "#!/bin/sh\necho fossil\n").expect("write the executable script");
    let mut permissions = std::fs::metadata(&script)
        .expect("read the script mode")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&script, permissions).expect("set the executable bit");
    std::os::unix::fs::symlink(SYMLINK_TARGET, env.path(SYMLINK_PATH))
        .expect("create the fixture symlink");
    common::fossil_commit_all(env, "fixture: fossil kinds");
}

/// The entry-kind fixture needs symlink support; refuse on a host without it.
#[cfg(not(unix))]
pub(crate) fn commit_fossil_kinds(_env: &common::TestEnv) {
    panic!("the entry-kind fixture requires symlink support");
}

/// The behavior feature the prepare `@e2e` fixture retains.
pub(crate) const E2E_FEATURE_PATH: &str = "givn/specs/demo.feature";
/// The behavior feature's content; its scenario is tagged `@e2e`.
pub(crate) const E2E_FEATURE_CONTENT: &str =
    "Feature: demo\n\n  @e2e\n  Scenario: demo\n    Given nothing\n";

/// A declaration whose `e2e` role names `entry` and whose other retained
/// roles are empty, with one source root, so only the `e2e` entry can match
/// the revision.
pub(crate) fn e2e_declaration(entry: &str, source: &str) -> String {
    format!(
        "behavior: []\n\
         structure: []\n\
         contracts: []\n\
         limits: []\n\
         e2e: [{entry}]\n\
         source: [{source}]\n"
    )
}

/// A declaration with the named `behavior` and `e2e` roles and one source
/// root, so no other declared entry can match nothing at the revision.
pub(crate) fn behavior_e2e_declaration(behavior: &str, e2e: &str, source: &str) -> String {
    format!(
        "behavior: [{behavior}]\n\
         structure: []\n\
         contracts: []\n\
         limits: []\n\
         e2e: [{e2e}]\n\
         source: [{source}]\n"
    )
}

/// A project whose `contracts` role names `givn/**`. The config and the
/// command file are excluded from the repository, so the revision's `givn/`
/// holds only the declaration and the document the role matches; the retained
/// count therefore isolates the union of the declaration and the role.
pub(crate) fn contracts_wildcard_project() -> common::TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    std::fs::write(
        env.path(".git/info/exclude"),
        "givn/config.yaml\ngivn/commands.yaml\n",
    )
    .expect("write the Git exclude fixture");
    std::fs::create_dir_all(env.path("givn")).expect("create givn/");
    std::fs::write(env.path("givn/config.yaml"), "addons:\n  ise: true\n")
        .expect("write the fixture config");
    std::fs::write(
        env.path(DECLARATION_PATH),
        contracts_declaration("\"givn/**\"", "\"src/**\""),
    )
    .expect("write the fixture declaration");
    env
}

/// A project whose `e2e` role names `givn/**`. The config and the command file
/// are excluded from the repository, so the revision's `givn/` holds only the
/// declaration and the document the role matches; the resolved package
/// therefore covers the declaration itself and one other file.
pub(crate) fn e2e_wildcard_project() -> common::TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    std::fs::write(
        env.path(".git/info/exclude"),
        "givn/config.yaml\ngivn/commands.yaml\n",
    )
    .expect("write the Git exclude fixture");
    std::fs::create_dir_all(env.path("givn")).expect("create givn/");
    std::fs::write(env.path("givn/config.yaml"), "addons:\n  ise: true\n")
        .expect("write the fixture config");
    std::fs::write(
        env.path(DECLARATION_PATH),
        e2e_declaration("\"givn/**\"", "\"src/**\""),
    )
    .expect("write the fixture declaration");
    env
}

/// A Git project whose first revision predates the declaration and whose
/// `HEAD` records it, so the working tree is clean while the selected revision
/// has no declaration. Returns the project and the first revision identifier.
pub(crate) fn declaration_late_project() -> (common::TestEnv, String) {
    let env = common::make_init_root();
    enable_ise(&env);
    // The add-on scaffold writes a declaration; the first revision must
    // predate it, so remove the scaffold before that commit.
    std::fs::remove_file(env.path(DECLARATION_PATH)).expect("remove the scaffolded declaration");
    let first = commit_files(
        &env,
        &[(KEPT_PATH, KEPT_CONTENT)],
        "fixture: revision predating the declaration",
    );
    write_declaration(&env, &source_only_declaration("\"src/**\""));
    common::git_commit_all(&env, "fixture: declaration");
    (env, first)
}

/// A Git project whose first revision declares the malformed declaration and
/// whose `HEAD` declares the valid one, so the working tree is clean while the
/// selected revision's declaration is malformed. Returns the project and the
/// first revision identifier.
pub(crate) fn malformed_first_revision_project() -> (common::TestEnv, String) {
    let env = common::make_init_root();
    enable_ise(&env);
    write_declaration(&env, &malformed_declaration());
    let first = commit_files(
        &env,
        &[(KEPT_PATH, KEPT_CONTENT)],
        "fixture: malformed declaration",
    );
    write_declaration(&env, &source_only_declaration("\"src/**\""));
    common::git_commit_all(&env, "fixture: valid declaration");
    (env, first)
}

/// A Git project whose first revision commits declaration bytes that are not
/// UTF-8 and whose `HEAD` records the valid one, so the working tree is clean
/// while the selected revision's declaration cannot be decoded. Returns the
/// project and the first revision identifier.
pub(crate) fn non_utf8_first_revision_project() -> (common::TestEnv, String) {
    let env = common::make_init_root();
    enable_ise(&env);
    std::fs::write(env.path(DECLARATION_PATH), NON_UTF8_DECLARATION_BYTES)
        .expect("write the non-UTF-8 declaration");
    let first = commit_files(
        &env,
        &[(KEPT_PATH, KEPT_CONTENT)],
        "fixture: non-UTF-8 declaration",
    );
    write_declaration(&env, &source_only_declaration("\"src/**\""));
    common::git_commit_all(&env, "fixture: valid declaration");
    (env, first)
}

/// Rewrite one role line of the checkout's declaration in place. The caller
/// leaves the file uncommitted: `ise compare` reads the working tree for the
/// `e2e` set and the `behavior` selection.
pub(crate) fn rewrite_checkout_role(env: &common::TestEnv, role: &str, value: &str) {
    let path = env.path(DECLARATION_PATH);
    let content = std::fs::read_to_string(&path).expect("read the checkout declaration");
    let key = format!("{role}:");
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let position = lines
        .iter()
        .position(|line| line.starts_with(&key))
        .unwrap_or_else(|| panic!("the checkout declaration has no '{key}' line"));
    lines[position] = format!("{role}: {value}");
    std::fs::write(&path, lines.join("\n") + "\n").expect("rewrite the checkout declaration");
}

/// The bytes of one path at one revision, read with `git show`.
pub(crate) fn revision_content(env: &common::TestEnv, revision: &str, path: &str) -> Vec<u8> {
    let output = Command::new("git")
        .args(["show", &format!("{revision}:{path}")])
        .current_dir(env.root())
        .output()
        .expect("spawn git show");
    assert!(
        output.status.success(),
        "git show {revision}:{path} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

/// A minimal Git project with `ise` enabled and the prepare scenario's
/// declaration. The config is excluded through `.git/info/exclude`; the
/// declaration is committed with the seven fixture files, so the revision
/// records it and the trial carries it.
pub(crate) fn e2e_trial_project() -> common::TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    std::fs::write(env.path(".git/info/exclude"), "givn/config.yaml\n")
        .expect("write the Git exclude fixture");
    std::fs::create_dir_all(env.path("givn")).expect("create givn/");
    std::fs::write(env.path("givn/config.yaml"), "addons:\n  ise: true\n")
        .expect("write the fixture config");
    std::fs::write(
        env.path("givn/specification.yaml"),
        behavior_e2e_declaration("\"givn/specs/**\"", "\"tests/e2e\"", "\"src/**\""),
    )
    .expect("write the fixture declaration");
    env
}

/// Commit the seven files the prepare `@e2e` scenario counts. Returns the
/// revision identifier.
pub(crate) fn commit_e2e_trial_revision(env: &common::TestEnv) -> String {
    commit_files(
        env,
        &[
            (
                DECLARATION_PATH,
                &behavior_e2e_declaration("\"givn/specs/**\"", "\"tests/e2e\"", "\"src/**\""),
            ),
            (E2E_FEATURE_PATH, E2E_FEATURE_CONTENT),
            ("tests/e2e/Cargo.toml", "[package]\nname = \"e2e\"\n"),
            ("tests/e2e/steps/demo.rs", "// e2e steps\n"),
            ("src/keys/parser.rs", "pub fn parse() {}\n"),
            ("src/keys/mod.rs", "pub mod parser;\n"),
            ("src/other.rs", "pub fn other() {}\n"),
            ("notes.md", "notes\n"),
        ],
        "fixture: e2e trial revision",
    )
}

/// The e2e package's manifest file the verify fixtures freeze.
pub(crate) const VERIFY_E2E_MANIFEST_PATH: &str = "tests/e2e/Cargo.toml";
/// The e2e package's step file the verify fixtures freeze.
pub(crate) const VERIFY_E2E_STEP_PATH: &str = "tests/e2e/steps/keys.rs";
/// An e2e package file the checkout gains after preparation.
pub(crate) const ADDED_E2E_STEP_PATH: &str = "tests/e2e/steps/extra.rs";
/// The manifest a checkout declaration's added `e2e` root brings.
pub(crate) const ADDED_E2E_ROOT_MANIFEST_PATH: &str = "tests/extra-e2e/Cargo.toml";
/// A behavior feature the checkout gains after preparation; one scenario is
/// tagged `@e2e`.
pub(crate) const ADDED_E2E_FEATURE_PATH: &str = "givn/specs/demo/extra.feature";
/// The gained feature's content: one `@e2e` scenario.
pub(crate) const ADDED_E2E_FEATURE_CONTENT: &str =
    "Feature: added demo\n\n  @e2e\n  Scenario: added demo\n    Given nothing\n";
/// The retained source file the new `--full` count fixture commits inside a
/// source root.
pub(crate) const RETAINED_SOURCE_PATH: &str = "src/keep.rs";
/// The kept source file the verify fixtures commit: inside `src/**` and
/// outside every frozen input.
pub(crate) const VERIFY_KEPT_PATH: &str = "src/keys/steps.rs";

/// A Git project with `ise` enabled and the verify scenarios' declaration:
/// `givn/specs/**` behavior, `docs/**` contracts, `tests/e2e` as the e2e
/// package, and `src/**` source. The config is excluded through
/// `.git/info/exclude`; the declaration is committed with the revision, so the
/// trial and its record always carry it.
pub(crate) fn verify_trial_project() -> common::TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    std::fs::write(env.path(".git/info/exclude"), "givn/config.yaml\n")
        .expect("write the Git exclude fixture");
    std::fs::create_dir_all(env.path("givn")).expect("create givn/");
    std::fs::write(
        env.path("givn/config.yaml"),
        "addons:\n  ise: true\n  coverage: false\n",
    )
    .expect("write the fixture config");
    std::fs::write(env.path("givn/specification.yaml"), verify_declaration())
        .expect("write the fixture declaration");
    env
}

fn verify_declaration() -> String {
    "behavior: [\"givn/specs/**\"]\n\
     structure: []\n\
     contracts: [\"docs/**\"]\n\
     limits: []\n\
     e2e: [\"tests/e2e\"]\n\
     source: [\"src/**\"]\n"
        .to_string()
}

/// The verification fixture's feature: one component scenario and one
/// `@e2e` scenario, so the component tier has a corpus to execute.
pub(crate) const TRIAL_FEATURE_CONTENT: &str = "Feature: demo\n\n  Scenario: component\n    Given nothing\n\n  @e2e\n  Scenario: demo\n    Given nothing\n";

/// The commands the verification fixture declares: a component runner inside
/// the copy, a frozen End-to-end runner in the checkout, and a product file.
pub(crate) const TRIAL_COMMANDS: &str = "verify:\n  command: \"sh ./scripts/verify.sh\"\n  e2e_command: \"sh ./scripts/e2e.sh\"\n  features_path: \"givn/specs\"\n  product_path: \"target/debug/gate-product\"\n  product_env: \"GIVN_BIN\"\n";

/// The component runner the builder writes into the copy: it builds the
/// declared product, parses the frozen feature, and executes the
/// `Given nothing` step for every non-`@e2e` scenario.
pub(crate) const TRIAL_COMPONENT_SCRIPT: &str = r#"#!/bin/sh
set -eu
mkdir -p target/debug
printf 'rebuilt gate product\n' > target/debug/gate-product
names=""
sep=""
scenario=""
e2e=0
skip=0
step_nothing() {
  :
}
while IFS= read -r line; do
  case "$line" in
    "  @e2e")
      e2e=1
      ;;
    "  Scenario: "*)
      if [ "$e2e" -eq 1 ]; then
        e2e=0
        skip=1
        scenario=""
        continue
      fi
      skip=0
      title=${line#  Scenario: }
      scenario=$title
      names="$names$sep\"$title\""
      sep=","
      ;;
    "    Given nothing")
      if [ "$skip" -eq 1 ]; then
        continue
      fi
      [ -n "$scenario" ] || { echo "step outside a scenario" >&2; exit 1; }
      step_nothing
      ;;
    "    "*)
      echo "unbound step: $line" >&2
      exit 1
      ;;
  esac
done < givn/specs/demo/spec.feature
printf '{"scope":"component","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s]}\n' "$names" > "$GIVN_RESULT_FILE"
"#;

/// The frozen End-to-end runner: it names the `@e2e` scenario and records the
/// rebuilt product digest it drove.
pub(crate) const TRIAL_E2E_SCRIPT: &str = r#"#!/bin/sh
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
' givn/specs/demo/spec.feature)
product=${GIVN_TRIAL_PRODUCT:-${GIVN_BIN:-}}
sha=$(sha256sum "$product" | cut -d' ' -f1)
printf '{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0,"scenarios":[%s],"product":"%s","product_sha256":"%s"}\n' "$names" "$product" "$sha" > "$GIVN_RESULT_FILE"
"#;

/// Commit the verification fixture's frozen inputs and commands. Returns the
/// revision identifier.
pub(crate) fn commit_verify_trial_revision(env: &common::TestEnv) -> String {
    commit_files(
        env,
        &[
            (DECLARATION_PATH, &verify_declaration()),
            ("givn/commands.yaml", TRIAL_COMMANDS),
            (FOSSIL_FEATURE_PATH, TRIAL_FEATURE_CONTENT),
            (REFERENCE_PATH, "reference\n"),
            (VERIFY_E2E_MANIFEST_PATH, "[package]\nname = \"e2e\"\n"),
            (VERIFY_E2E_STEP_PATH, "// e2e steps\n"),
            (VERIFY_KEPT_PATH, "pub fn steps() {}\n"),
            (REPLACED_SOURCE_PATH, "pub fn parse() {}\n"),
        ],
        "fixture: verify trial revision",
    )
}

/// Install the trial scripts after preparation: the component runner inside
/// the copy and the frozen End-to-end runner in the checkout.
pub(crate) fn write_trial_scripts(env: &common::TestEnv, copy: &Path) {
    for (path, content) in [
        (copy.join("scripts/verify.sh"), TRIAL_COMPONENT_SCRIPT),
        (env.path("scripts/e2e.sh"), TRIAL_E2E_SCRIPT),
    ] {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create script parent");
        }
        std::fs::write(&path, content).expect("write trial script");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&path).unwrap().permissions();
            permissions.set_mode(0o755);
            std::fs::set_permissions(&path, permissions).expect("make the trial script executable");
        }
    }
}

/// The e2e step files the broad-package fixture commits beside its manifest,
/// so the resolved package holds twenty-five paths.
pub(crate) const BROAD_PACKAGE_STEP_FILES: usize = 24;

/// The e2e package directory the broad-package fixture resolves to one root.
pub(crate) const BROAD_PACKAGE_PATH: &str = "tests/e2e";

/// A Git project with `ise` enabled whose committed `tests/e2e` package holds
/// twenty-five files: the manifest beside twenty-four step files committed in
/// one revision, so `e2e_file_set` resolves the directory to that manifest.
pub(crate) fn broad_package_project() -> common::TestEnv {
    let env = verify_trial_project();
    let mut owned: Vec<(String, String)> = vec![
        (DECLARATION_PATH.to_string(), verify_declaration()),
        (
            FOSSIL_FEATURE_PATH.to_string(),
            FOSSIL_FEATURE_CONTENT.to_string(),
        ),
        (REFERENCE_PATH.to_string(), "reference\n".to_string()),
        (
            VERIFY_E2E_MANIFEST_PATH.to_string(),
            "[package]\nname = \"e2e\"\n".to_string(),
        ),
        (
            VERIFY_KEPT_PATH.to_string(),
            "pub fn steps() {}\n".to_string(),
        ),
        (
            REPLACED_SOURCE_PATH.to_string(),
            "pub fn parse() {}\n".to_string(),
        ),
    ];
    for index in 0..BROAD_PACKAGE_STEP_FILES {
        owned.push((
            format!("{BROAD_PACKAGE_PATH}/steps/f{index:02}.rs"),
            format!("// e2e steps {index}\n"),
        ));
    }
    let files: Vec<(&str, &str)> = owned
        .iter()
        .map(|(path, content)| (path.as_str(), content.as_str()))
        .collect();
    commit_files(&env, &files, "fixture: broad e2e package");
    env
}

/// Copy one tracked checkout path into the trial, creating its parent
/// directories: preparation never writes a frozen path, so a scenario that
/// needs one plants the checkout's own bytes.
pub(crate) fn copy_into_trial(env: &common::TestEnv, trial: &Path, path: &str) {
    let source = env.path(path);
    let target = trial.join(path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).expect("create the planted path's parent");
    }
    std::fs::copy(&source, &target).unwrap_or_else(|error| {
        panic!("copy {} into the trial: {error}", source.display());
    });
}

/// Copy every checkout file under `directory` into the trial, so a scenario
/// plants a whole frozen package.
pub(crate) fn copy_tree_into_trial(env: &common::TestEnv, trial: &Path, directory: &str) {
    let root = env.path(directory);
    let mut directories = vec![root];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        {
            let path = entry.expect("read the checkout entry").path();
            let relative = path
                .strip_prefix(env.root())
                .expect("the walked path is inside the checkout")
                .to_string_lossy()
                .into_owned();
            if path.is_dir() {
                directories.push(path);
            } else {
                copy_into_trial(env, trial, &relative);
            }
        }
    }
}

/// Record a receipt's content rewrite as an older record: every `e2e_files`
/// identity also appears in `retained_files` and `excluded_roles` is absent.
/// The trial already holds the package's own bytes, so the identities stay
/// correct and the retained comparison covers the planted paths.
pub(crate) fn rewrite_record_without_excluded_roles(path: &Path) {
    let mut mapping = record_mapping(path);
    let e2e = mapping
        .get(serde_yaml_ng::Value::String("e2e_files".to_string()))
        .cloned()
        .unwrap_or_else(|| panic!("the record has no e2e_files mapping"));
    let retained_key = serde_yaml_ng::Value::String("retained_files".to_string());
    let mut retained = match mapping.remove(&retained_key) {
        Some(serde_yaml_ng::Value::Mapping(map)) => map,
        other => panic!("the record's retained_files is not a mapping: {other:?}"),
    };
    match e2e {
        serde_yaml_ng::Value::Mapping(map) => retained.extend(map),
        other => panic!("the record's e2e_files is not a mapping: {other:?}"),
    }
    mapping.insert(retained_key, serde_yaml_ng::Value::Mapping(retained));
    mapping.remove(serde_yaml_ng::Value::String("excluded_roles".to_string()));
    write_record_mapping(path, mapping);
}

/// The record parsed into a mapping a scenario can rewrite before the
/// re-check, so one prepared trial proves every record refusal.
pub(crate) fn record_mapping(path: &Path) -> serde_yaml_ng::Mapping {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read the record {}: {error}", path.display()));
    let document: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
        .unwrap_or_else(|error| panic!("parse the record {}: {error}", path.display()));
    match document {
        serde_yaml_ng::Value::Mapping(mapping) => mapping,
        other => panic!("the record is not a mapping: {other:?}"),
    }
}

/// Replace the record with a mutated mapping.
pub(crate) fn write_record_mapping(path: &Path, mapping: serde_yaml_ng::Mapping) {
    let text = serde_yaml_ng::to_string(&serde_yaml_ng::Value::Mapping(mapping))
        .expect("serialize the rewritten record");
    std::fs::write(path, text).expect("rewrite the record");
}

/// Append bytes to one file, so its identity changes while its entry kind
/// stays the same.
pub(crate) fn append_bytes(path: &Path) {
    use std::io::Write as _;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap_or_else(|error| panic!("open {} to append: {error}", path.display()));
    file.write_all(b"\n").expect("append to the fixture file");
}

/// Write a file the checkout gained after preparation. The file stays
/// untracked; `git ls-files --others` still lists it, so the re-check sees it.
pub(crate) fn add_checkout_file(env: &common::TestEnv, path: &str, content: &str) {
    let target = env.path(path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).expect("create the added file's parent directory");
    }
    std::fs::write(&target, content).expect("write the added file");
}

/// Point the manifest at `<container>/blocker/manifest.yaml`, where `blocker`
/// is a regular file, so the manifest write fails after the spec copy exists.
pub(crate) fn blocked_manifest(state: &mut IseCleanRoomState) {
    state.ensure_targets();
    let container = state
        .targets
        .as_ref()
        .expect("the targets exist")
        .path()
        .to_path_buf();
    let blocker = container.join("blocker");
    std::fs::write(&blocker, "not a directory\n").expect("write the manifest blocker");
    state.manifest_path = Some(blocker.join("manifest.yaml"));
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

/// A Git project with `ise` on, its `architecture_principles` prerequisite
/// off, and a malformed declaration on disk, so a reader that loads the
/// declaration before the add-on check would report the wrong refusal.
pub(crate) fn architecture_principles_off_project() -> common::TestEnv {
    let env = ise_project();
    set_addon(&env, "architecture_principles", false);
    write_declaration(&env, &malformed_declaration());
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
