//! Step definitions for the `development-record` capability.
//!
//! The scenarios observe the development record through its on-disk contract
//! (`givn/history/events.jsonl`, one JSON object per line) in a black-box
//! project root, so nothing here reads the recorder's in-memory state.

use std::collections::BTreeSet;

use cucumber::{given, then, when};
use serde_json::{Value, json};

use crate::common;
use crate::steps::development_timeline_support as support;
use crate::steps::{GivnWorld, stderr};

/// State for the development-record scenarios: the entry the last reading
/// step identified, so a following step can assert on "that entry", plus the
/// store snapshot the append-only scenario takes before its second command.
#[derive(Debug, Default)]
pub(crate) struct DevelopmentRecordState {
    pub(crate) last_entry: Option<Value>,
    /// The store's bytes as they stood before this scenario's next command.
    pub(crate) store_before: Option<Vec<u8>>,
    /// The earlier entry's own field values, read the way a consumer reads
    /// them.
    pub(crate) earlier_entry: Option<Value>,
    /// The entries the store held before the two-writer race started.
    pub(crate) entries_before: Option<usize>,
    /// The racing `givn` processes the last Given started, waiting to be
    /// waited for.
    pub(crate) racing: Option<Vec<std::process::Child>>,
    /// The ideation topic the fixture declared, for the step that asserts on
    /// "the ideation topic".
    pub(crate) topic: Option<String>,
    /// The ids the record already held when the off-mode fixture started: the
    /// earlier entries the developer can still read after the mode is lowered.
    pub(crate) earlier_ids: Vec<u64>,
    /// The question text the Maki transcript fixture carries, joined the way
    /// the reporter joins it.
    pub(crate) question_text: Option<String>,
    /// The wait title (the label's bounded source) the Maki fixture carries.
    pub(crate) question_title: Option<String>,
    /// The title of the residual wait whose transcript holds no question.
    pub(crate) residual_title: Option<String>,
}

/// The verification command the bounded-label fixture declares: it really runs
/// (so its step entry is measured) and its complete text is longer than the
/// record's label limit, so the complete text must not survive into the store.
pub(crate) const LONG_VERIFY_COMMAND: &str = "sleep 1 # 012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789";

/// The byte-level form of "nothing earlier was rewritten": the store still
/// begins with the bytes it held before the scenario's next command. Both
/// append-only steps state the property through this one assertion.
fn assert_store_prefix_unchanged(before: &[u8], after: &[u8]) {
    assert!(
        after.starts_with(before),
        "the store's earlier bytes were rewritten:\nbefore: {}\nafter:  {}",
        String::from_utf8_lossy(before),
        String::from_utf8_lossy(after)
    );
}

/// The one `command` entry the scenario's own command recorded, found by the
/// command name the `the user runs …` step captured — never by position, so
/// the fixture's own setup commands are never mistaken for it.
fn recorded_command_entry<'a>(world: &GivnWorld, entries: &'a [Value]) -> &'a Value {
    let command = world
        .last_command
        .clone()
        .expect("no `givn` command has been run yet");
    let recorded: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("command") && entry["name"] == json!(command))
        .collect();
    assert_eq!(
        recorded.len(),
        1,
        "expected exactly one `command` entry for `{command}`; the record holds: {entries:#?}"
    );
    recorded[0]
}

#[then("the development record holds one entry for that command")]
fn record_holds_one_entry_for_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let entry = recorded_command_entry(world, &entries).clone();
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(entry);
}

#[given(regex = r#"^a change "([^"]+)" with a verification command that takes measurable time$"#)]
fn change_with_a_slow_verification_command(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {} failed: {}",
        id,
        String::from_utf8_lossy(&out.stderr),
    );
    support::declare_verify_command(env, support::SLOW_VERIFY_COMMAND);
    world.last_change_id = Some(id);
}

#[then("the development record holds one entry for the verification run")]
fn record_holds_one_entry_for_the_verification_run(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let step = verified_step_entry(world, &entries).clone();
    let label = step["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the verification step carries no label: {step}"));
    assert!(
        label.contains(support::SLOW_VERIFY_COMMAND),
        "the label does not name the verification run: {step}"
    );
    assert!(
        label.chars().count() <= 120,
        "the label is not bounded to 120 characters: {step}"
    );
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(step);
}

#[then("that entry carries a duration of at least one second")]
fn entry_carries_at_least_one_second(world: &mut GivnWorld) {
    let entry = world
        .development_record
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no record entry was identified by the previous step");
    let duration_ms = entry["duration_ms"]
        .as_u64()
        .unwrap_or_else(|| panic!("the entry carries no measured duration: {entry}"));
    assert!(
        duration_ms >= 1_000,
        "the verification step is not measured as at least one second: {entry}"
    );
    let outcome = entry["outcome"]
        .as_str()
        .unwrap_or_else(|| panic!("the entry carries no outcome: {entry}"));
    assert!(
        ["ok", "failed", "skipped", "unknown"].contains(&outcome),
        "unexpected outcome `{outcome}`: {entry}"
    );
}

#[then("that entry carries a measured duration and its outcome")]
fn entry_carries_a_measured_duration_and_its_outcome(world: &mut GivnWorld) {
    let entry = world
        .development_record
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no record entry was identified by the previous step");
    let duration_ms = entry["duration_ms"]
        .as_u64()
        .unwrap_or_else(|| panic!("the entry carries no measured duration: {entry}"));
    assert!(
        duration_ms < 600_000,
        "implausible measured duration {duration_ms} ms: {entry}"
    );
    assert_eq!(entry["outcome"], json!("ok"), "unexpected outcome: {entry}");
}

#[given("the development record cannot be written")]
fn record_cannot_be_written(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // The store path itself is blocked (a directory where the file belongs), so
    // the append fails the way a full or read-only disk fails it — a real
    // failure of the real write path, never a simulated one.
    support::make_record_unwritable(env);
}

#[then("stderr warns that the development record could not be written")]
fn stderr_warns_that_the_record_could_not_be_written(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let store = support::store_file(&env.root());
    let lines = stderr(world);
    let warnings: Vec<&str> = lines
        .lines()
        .filter(|line| line.contains("development record"))
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "expected exactly one warning naming the development record (no retry loop): {lines:?}"
    );
    let warning = warnings[0];
    assert!(
        warning.starts_with("warning:") && warning.contains("could not be written"),
        "the line is not a warning that the record could not be written: {warning}"
    );
    assert!(
        warning.contains(&store.display().to_string()),
        "the warning does not name the store path {}: {warning}",
        store.display()
    );
    // The warning is the only observable effect: no silent fallback wrote the
    // entry elsewhere, so the store path is still the blocking directory and
    // the record holds no entry.
    assert!(
        store.is_dir(),
        "the append wrote to the blocked store path {}",
        store.display()
    );
}

#[given("the development record holds an entry from an earlier command")]
fn record_holds_an_entry_from_an_earlier_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["status"]);
    assert!(
        out.status.success(),
        "the earlier command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let entries = support::record_entries(&env.root());
    let earlier = entries
        .last()
        .cloned()
        .expect("the earlier command wrote no entry");
    assert_eq!(
        (earlier["kind"].clone(), earlier["name"].clone()),
        (json!("command"), json!("status")),
        "the last entry is not the earlier command's own entry: {earlier}"
    );
    let state = world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default);
    state.store_before = Some(support::store_bytes(&env.root()));
    state.earlier_entry = Some(earlier);
}

#[then("the earlier entry is still present unchanged")]
fn earlier_entry_is_still_present_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let state = world
        .development_record
        .as_ref()
        .expect("the earlier-command fixture ran first");
    let before = state
        .store_before
        .clone()
        .expect("the fixture took no store snapshot");
    let earlier = state
        .earlier_entry
        .clone()
        .expect("the fixture identified no earlier entry");
    let after = support::store_bytes(&env.root());
    assert_store_prefix_unchanged(&before, &after);
    let entries = support::record_entries(&env.root());
    let read_back = entries
        .iter()
        .find(|entry| entry["id"] == earlier["id"])
        .unwrap_or_else(|| panic!("the earlier entry is gone: {entries:#?}"));
    assert_eq!(
        read_back, &earlier,
        "the earlier entry's field values changed"
    );
}

#[then("the new entry is appended after it without rewriting the store")]
fn new_entry_is_appended_after_it_without_rewriting_the_store(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = world
        .development_record
        .as_ref()
        .and_then(|state| state.store_before.clone())
        .expect("the earlier-command fixture ran first");
    let after = support::store_bytes(&env.root());
    assert_store_prefix_unchanged(&before, &after);
    let appended = &after[before.len()..];
    let appended = std::str::from_utf8(appended)
        .unwrap_or_else(|error| panic!("the appended bytes are not UTF-8: {error}"));
    let appended_lines: Vec<&str> = appended.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        appended_lines.len(),
        1,
        "the command appended {} lines, not one: {appended:?}",
        appended_lines.len()
    );
    let entry: Value = serde_json::from_str(appended_lines[0])
        .unwrap_or_else(|error| panic!("the appended line is not a JSON object: {error}"));
    assert_eq!(
        (entry["kind"].clone(), entry["name"].clone()),
        (json!("command"), json!("status")),
        "the appended entry is not this command's own entry: {entry}"
    );
}

#[given("no change and no ideation is active for this project")]
fn no_change_and_no_ideation_is_active(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    support::assert_bare_project(env);
}

#[then("the recorded entry names the project as its scope")]
fn recorded_entry_names_the_project_as_its_scope(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let scope = &recorded_command_entry(world, &entries)["scope"];
    assert_eq!(
        scope["kind"],
        json!("project"),
        "the entry is not scoped to the project: {scope}"
    );
    assert_eq!(
        scope["id"],
        json!(support::project_name(env)),
        "the entry's scope does not name the fixture's own project: {scope}"
    );
}

#[given("two givn commands record at the same time")]
fn two_givn_commands_record_at_the_same_time(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["new", "demo"]);
    assert!(
        out.status.success(),
        "givn new demo failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let before = support::record_entries(&env.root()).len();
    let racing = support::start_racing_commands(env);
    let state = world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default);
    state.entries_before = Some(before);
    state.racing = Some(racing);
}

#[when("both commands finish")]
fn both_commands_finish(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let racing = world
        .development_record
        .as_mut()
        .and_then(|state| state.racing.take())
        .expect("the previous step started no racing commands");
    assert_eq!(
        racing.len(),
        support::RACING_ROUNDS * 2,
        "the fixture did not start two commands per round"
    );
    support::finish_racing_commands(env, racing);
}

#[then("every entry in the development record has a distinct id")]
fn every_entry_has_a_distinct_id(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = world
        .development_record
        .as_ref()
        .and_then(|state| state.entries_before)
        .expect("the two-writer fixture ran first");
    let entries = support::record_entries(&env.root());
    // Every racer appended its own run and no line was lost or invented: one
    // `command` entry plus its two children per racing command.
    let expected = before + support::RACING_ROUNDS * 2 * 3;
    assert_eq!(
        entries.len(),
        expected,
        "the record holds {} entries, expected {expected}: {entries:#?}",
        entries.len()
    );
    let mut seen = BTreeSet::new();
    let duplicates: Vec<u64> = entries
        .iter()
        .map(|entry| {
            entry["id"]
                .as_u64()
                .unwrap_or_else(|| panic!("an entry carries no numeric id: {entry}"))
        })
        .filter(|id| !seen.insert(*id))
        .collect();
    assert!(
        duplicates.is_empty(),
        "the record minted the same id twice: {duplicates:?}"
    );
}

#[then("every parent link names an entry that exists in the record")]
fn every_parent_link_names_an_entry_in_the_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let ids: BTreeSet<u64> = entries
        .iter()
        .map(|entry| {
            entry["id"]
                .as_u64()
                .unwrap_or_else(|| panic!("an entry carries no numeric id: {entry}"))
        })
        .collect();
    let dangling: Vec<&Value> = entries
        .iter()
        .filter(
            |entry| match entry.get("parent").and_then(|parent| parent.as_u64()) {
                Some(parent) => !ids.contains(&parent),
                None => false,
            },
        )
        .collect();
    assert!(
        dangling.is_empty(),
        "these entries name a parent that is not in the record: {dangling:#?}"
    );
    // Not a vacuous claim: every racing run's children carry the link.
    let linked = entries
        .iter()
        .filter(|entry| entry.get("parent").is_some())
        .count();
    assert_eq!(
        linked,
        support::RACING_ROUNDS * 2 * 2,
        "the record does not carry the racing runs' child links: {entries:#?}"
    );
}

#[then("the repository reports no pending change for the development record")]
fn repository_reports_no_pending_change_for_the_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let store = support::store_file(&env.root());
    assert!(
        store.is_file(),
        "the scenario's own command wrote no record at {}: the assertion below would be vacuous",
        store.display()
    );
    let status = common::git(env, &["status", "--porcelain", "-uall"]);
    assert!(
        status.status.success(),
        "git status failed: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    let porcelain = String::from_utf8_lossy(&status.stdout);
    let named: Vec<&str> = porcelain
        .lines()
        .filter(|line| line.contains("givn/history/events.jsonl"))
        .collect();
    assert!(
        named.is_empty(),
        "the repository reports the development record as a pending change:\n{porcelain}"
    );
    // Not a vacuous claim: the record is ignored, not merely committed.
    let ignored = common::git(env, &["check-ignore", "-q", "givn/history/events.jsonl"]);
    assert!(
        ignored.status.success(),
        "the development record is not ignored by the repository:\n{porcelain}"
    );
}

/// The record's directory, relative to the project root.
pub(crate) const RECORD_DIR: &str = "givn/history";

/// Whether `path` is the project's record directory. The acceptance suite's
/// project-state comparisons exclude it because the record is Givn runtime
/// state, not project content; the comparison is by path, never by bare entry
/// name, so an unrelated directory called `history` is never dropped.
pub(crate) fn is_record_dir(root: &std::path::Path, path: &std::path::Path) -> bool {
    path == root.join(RECORD_DIR)
}

#[then(regex = r#"^the directory "([^"]+)" exists$"#)]
fn named_directory_exists(world: &mut GivnWorld, rel: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path(&rel);
    assert!(
        path.is_dir(),
        "expected directory {} to exist",
        path.display()
    );
}

#[then("the project root holds no development record")]
fn project_root_holds_no_development_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let root = env.root();
    let store = support::store_file(&root);
    assert!(
        !store.exists(),
        "the project root holds a development record at {}",
        store.display()
    );
    assert!(
        !root.join(RECORD_DIR).exists(),
        "the recorder created the record directory {}",
        root.join(RECORD_DIR).display()
    );
    // The engagement rule gates everything a record write would create,
    // including the scaffold-time `.gitignore` fallback of a root with no
    // repository.
    assert!(
        !root.join(".gitignore").exists(),
        "the recorder wrote {} in a root it must not engage in",
        root.join(".gitignore").display()
    );
}

#[then("the project root holds a development record")]
fn project_root_holds_a_development_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    assert!(
        entries
            .iter()
            .any(|entry| entry["kind"] == json!("command") && entry["name"] == json!("init")),
        "the successful init did not record its own command entry: {entries:#?}"
    );
}

#[given("a project root with givn/ initialised in a Fossil checkout")]
fn project_root_initialised_in_a_fossil_checkout(world: &mut GivnWorld) {
    // A real Fossil checkout first and `givn init` INSIDE it: the managed
    // ignore file then carries the record path before the fixture's first
    // commit, so its `addremove` cannot record the store (F3).
    let env = common::make_fossil_root_bare();
    let out = common::run_givn(&env, &["init", "--agents", "opencode"]);
    assert!(
        out.status.success(),
        "givn init in the Fossil checkout failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    common::fossil_commit_all(&env, "fixture: initial state");
    world.env = Some(env);
}

/// The glob entries of a Fossil ignore file: its non-comment lines. The fence
/// markers are not globs.
fn fossil_glob_entries(path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

#[then("the repository's ignore glob names the development record")]
fn repository_ignore_glob_names_the_development_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert_ignore_glob_names_record(env);
}

/// The development record is named by both effective ignore sources of a
/// Fossil checkout — the committed managed file and the repository-level
/// setting — and it is neither a pending nor an extra change.
fn assert_ignore_glob_names_record(env: &common::TestEnv) {
    let store = support::store_file(&env.root());
    assert!(
        store.is_file(),
        "the scenario's own command wrote no record at {}: the assertion below would be vacuous",
        store.display()
    );
    // A committed managed file overrides the repository-level setting, so both
    // sources are read: the step never asserts on the file it happened to
    // write.
    let managed = env.path(".fossil-settings/ignore-glob");
    assert!(
        managed.is_file(),
        "the managed checkout must carry {}: the effective ignore source would otherwise be the setting",
        managed.display()
    );
    let entries = fossil_glob_entries(&managed);
    assert!(
        entries.iter().any(|entry| entry == "givn/history/*"),
        "the managed ignore file does not name the development record:\n{entries:#?}"
    );
    let setting = common::fossil(env, &["settings", "--value", "ignore-glob"]);
    assert!(
        setting.status.success(),
        "read the repository ignore-glob setting: {}",
        String::from_utf8_lossy(&setting.stderr)
    );
    let setting = String::from_utf8_lossy(&setting.stdout);
    assert!(
        setting
            .split([',', '\n'])
            .any(|entry| entry.trim() == "givn/history/*"),
        "the effective ignore-glob setting does not name the development record: {setting:?}"
    );
    // The record is neither a pending nor an extra change: the committed file
    // keeps it out of the checkout without the working tree becoming dirty.
    for command in ["changes", "extras"] {
        let out = common::fossil(env, &[command]);
        assert!(
            out.status.success(),
            "fossil {command} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let listing = String::from_utf8_lossy(&out.stdout);
        assert!(
            !listing.contains("givn/history/events.jsonl"),
            "the repository reports the development record as a {command} entry:\n{listing}"
        );
    }
}

#[given("a project root with givn/ initialised under Git")]
fn project_root_initialised_under_git(world: &mut GivnWorld) {
    let env = common::make_init_root();
    assert!(
        common::is_git_repo(&env),
        "the fixture must be a real Git worktree: {}",
        env.root().display()
    );
    world.env = Some(env);
}

#[given(regex = r#"^a change "([^"]+)" with a recorded history$"#)]
fn change_with_a_recorded_history(world: &mut GivnWorld, id: String) {
    // When this is the scenario's first Given the scenario did not name a
    // project root itself (the agent-activity-reporting and development-report
    // scenarios that start from a recorded history), the fixture brings up the
    // same init root the root Given builds.
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    for args in [
        vec!["new".to_string(), id.clone()],
        vec!["status".to_string(), "--change".to_string(), id.clone()],
    ] {
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let out = common::run_givn(env, &argv);
        assert!(
            out.status.success(),
            "givn {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
    }
    let entries = support::record_entries(&env.root());
    assert!(
        !entries.is_empty(),
        "the fixture recorded no history at all: {entries:#?}"
    );
    // The store's bytes as they stand before the scenario's next command: the
    // reading commands must leave exactly these bytes behind.
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .store_before = Some(support::store_bytes(&env.root()));
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" with a command whose full text is longer than 120 characters$"#
)]
fn change_with_a_command_longer_than_the_label_limit(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        LONG_VERIFY_COMMAND.chars().count() > 120,
        "the fixture's command text must exceed the record's label limit: {LONG_VERIFY_COMMAND}"
    );
    let out = common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    support::declare_verify_command(env, LONG_VERIFY_COMMAND);
    world.last_change_id = Some(id);
}

#[when("the user runs that command")]
fn user_runs_that_command(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the long-command fixture ran first");
    let out = common::run_givn(env, &["check", "tasks", "--change", &change]);
    world.last_command = Some("check".to_string());
    world.last_output = Some(out);
}

/// The one `step` entry the fixture's own run recorded as the child of the
/// `check` command's entry — found through the command's minted id, never by
/// position.
fn verified_step_entry<'a>(world: &GivnWorld, entries: &'a [Value]) -> &'a Value {
    let command = recorded_command_entry(world, entries);
    let steps: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("step") && entry["parent"] == command["id"])
        .collect();
    assert_eq!(
        steps.len(),
        1,
        "expected exactly one `step` entry under the check command; the record holds: {entries:#?}"
    );
    steps[0]
}

#[then("the recorded entry's label is at most 120 characters and marks the truncation")]
fn recorded_label_is_at_most_120_characters_and_marks_the_truncation(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let step = verified_step_entry(world, &entries).clone();
    let label = step["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the step entry carries no label: {step}"));
    assert!(
        label.chars().count() <= 120,
        "the label is not bounded to 120 characters ({}): {step}",
        label.chars().count()
    );
    let truncated = label
        .strip_suffix('…')
        .unwrap_or_else(|| panic!("the label does not mark its truncation: {step}"));
    assert!(
        LONG_VERIFY_COMMAND.starts_with(truncated),
        "the label is not derived from the command text: {step}"
    );
    assert_ne!(
        label, LONG_VERIFY_COMMAND,
        "the label kept the complete command text: {step}"
    );
    // The bound is a character count, not a byte count: the label is measured
    // the way a consumer measures it.
    assert_eq!(
        truncated.chars().count() + 1,
        label.chars().count(),
        "the truncation mark is not part of the bounded label: {step}"
    );
}

#[then("the record holds no complete command text")]
fn record_holds_no_complete_command_text(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let bytes = support::store_bytes(&env.root());
    let text = String::from_utf8_lossy(&bytes);
    assert!(
        !text.contains(LONG_VERIFY_COMMAND),
        "the store carries the complete command text: {text}"
    );
    // Not a vacuous claim: the fixture's own run is in the store, with the
    // truncated label as its only trace of the command.
    let entries = support::record_entries(&env.root());
    let step = verified_step_entry(world, &entries).clone();
    let label = step["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the step entry carries no label: {step}"));
    assert!(
        text.lines().any(|line| line.contains(label)),
        "the record does not carry the step's bounded label: {text}"
    );
}

#[then("the development record holds no new entry")]
fn record_holds_no_new_entry(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = world
        .development_record
        .as_ref()
        .and_then(|state| state.store_before.clone())
        .expect("the recorded-history fixture ran first");
    let after = support::store_bytes(&env.root());
    assert_eq!(
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&after),
        "the reading commands changed the record"
    );
    // The reads really happened: the report was written from the record, so
    // the assertion above is about a render that ran, not about a no-op.
    let out = world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet");
    assert!(
        out.status.success(),
        "the report run failed: {}",
        stderr(world)
    );
    let report = env.path("report.html");
    let document = std::fs::read_to_string(&report).unwrap_or_else(|error| {
        panic!(
            "the report was not written to {}: {error}",
            report.display()
        )
    });
    assert!(
        !document.contains("__GIVN_REPORT_DATA__"),
        "the report still carries its unfilled data slot: {}",
        report.display()
    );
    assert!(
        document.contains("demo"),
        "the report does not name the scope it renders: {}",
        report.display()
    );
}

#[given(regex = r#"^an ideation topic "([^"]+)" whose session is unfinished$"#)]
fn an_ideation_topic_whose_session_is_unfinished(world: &mut GivnWorld, topic: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // The fixture is a project that has no change yet: the topic is what the
    // project is working on, and only the topic can scope the activity.
    support::assert_no_active_change(env);
    support::declare_unfinished_ideation_topic(env, &topic);
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .topic = Some(topic);
}

#[then("the recorded entry names the ideation topic as its scope")]
fn recorded_entry_names_the_ideation_topic_as_its_scope(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let topic = world
        .development_record
        .as_ref()
        .and_then(|state| state.topic.clone())
        .expect("the ideation fixture ran first");
    let entries = support::record_entries(&env.root());
    let scope = &recorded_command_entry(world, &entries)["scope"];
    assert_eq!(
        scope,
        &json!({ "kind": "ideation", "id": topic }),
        "the entry's scope does not name the unfinished ideation topic: {scope}"
    );
}

/// The full-content marker the record-hygiene fixture writes through the real
/// intake. A real `data.content` entry is what makes "the store is ignored" a
/// claim that can fail: an ignore rule that misses the store would show up as a
/// pending change (Git) or an extra file (Fossil) carrying this text.
const HYGIENE_CONTENT: &str =
    "full-mode content written through the real intake: the ignore property must cover the store";

/// The `#[...]` block of a fenced ignore file, or `None` when the file carries
/// no record block.
fn history_block(text: &str) -> Option<String> {
    let mut block = String::new();
    let mut inside = false;
    let mut found = false;
    for line in text.lines() {
        if line.trim() == givn::history::HISTORY_BLOCK_BEGIN {
            inside = true;
            found = true;
            continue;
        }
        if line.trim() == givn::history::HISTORY_BLOCK_END {
            inside = false;
            continue;
        }
        if inside {
            block.push_str(line);
            block.push('\n');
        }
    }
    found.then_some(block)
}

/// Remove the record's managed block from a fenced ignore file, leaving the
/// rest of the file intact. The scaffold-time safety net is then the
/// scenario's own full-content write to re-assert, not a fixture's own earlier
/// command.
fn remove_history_block(path: &std::path::Path) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return;
    };
    let mut kept: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.trim() == givn::history::HISTORY_BLOCK_BEGIN {
            inside = true;
            continue;
        }
        if line.trim() == givn::history::HISTORY_BLOCK_END {
            inside = false;
            continue;
        }
        if !inside {
            kept.push(line);
        }
    }
    let mut content = kept.join("\n");
    if !content.is_empty() {
        content.push('\n');
    }
    std::fs::write(path, content).expect("rewrite the ignore file without the record block");
}

#[given("a project root with givn/ initialised without a repository")]
fn project_root_initialised_without_a_repository(world: &mut GivnWorld) {
    let env = common::make_init_root_without_git();
    assert!(
        !common::is_git_repo(&env),
        "the fixture must start without a repository: {}",
        env.root().display()
    );
    // The scenario asserts that the full-content write leaves the
    // scaffold-time block behind, so the fixture starts from a file that does
    // not already carry it.
    remove_history_block(&env.path(".gitignore"));
    assert!(
        history_block(&std::fs::read_to_string(env.path(".gitignore")).unwrap_or_default())
            .is_none(),
        "the fixture could not clear the record block it must observe being written"
    );
    world.env = Some(env);
}

#[when("the project records a full-content entry in the full mode")]
fn project_records_a_full_content_entry(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let payload = support::intake_payload(
        Some("demo"),
        None,
        vec![json!({
            "kind": "tool",
            "at": "2026-10-17T08:00:00.000Z",
            "tool": "Bash",
            "label": "cargo build --locked",
            "duration_ms": 1_200,
            "outcome": "ok",
            "content": {
                "tool_input": {
                    "text": "cargo build --locked",
                    "truncated": false,
                    "bytes": 19
                },
                "tool_output": {
                    "text": HYGIENE_CONTENT,
                    "truncated": false,
                    "bytes": HYGIENE_CONTENT.len()
                }
            }
        })],
    );
    let before = support::record_entries(&env.root()).len();
    let out = support::report_intake_with_args(env, &payload, &["--with-full-content"]);
    assert!(
        out.status.success(),
        "the full-content intake failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    // The write really carried content, so the ignore assertions that follow
    // are about a store that could fail them: the reported entry and the
    // intake invocation's own command entry are both appended.
    let entries = support::record_entries(&env.root());
    assert_eq!(
        entries.len(),
        before + 2,
        "the full-content write did not append its entry and command: {entries:#?}"
    );
    let appended = &entries[before..];
    let stored: Vec<&Value> = appended
        .iter()
        .filter(|entry| {
            entry
                .pointer("/data/content/tool_output/text")
                .and_then(Value::as_str)
                == Some(HYGIENE_CONTENT)
        })
        .collect();
    assert_eq!(
        stored.len(),
        1,
        "the record did not keep the full-mode content entry: {entries:#?}"
    );
}

#[then(regex = r#"^the project's "\.gitignore" names the development record$"#)]
fn gitignore_names_the_development_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path(".gitignore");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let block = history_block(&text).unwrap_or_else(|| {
        panic!(
            "{} carries no record block: the scaffold-time safety net was not written",
            path.display()
        )
    });
    assert!(
        block.lines().any(|line| line.trim() == "givn/history/"),
        "the record block does not name the store: {block:?}"
    );
}

#[when("the project becomes a Git repository")]
fn project_becomes_a_git_repository(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    common::git_init(env);
    assert!(
        common::is_git_repo(env),
        "the fixture did not become a Git worktree: {}",
        env.root().display()
    );
}

#[then("the repository reports no pending change under the record directory")]
fn repository_reports_no_pending_change_under_the_record_directory(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let store = support::store_file(&env.root());
    let report = env.path("givn/history/reports/demo.html");
    assert!(
        store.is_file() && report.is_file(),
        "the scenario wrote no record and report to ignore ({}, {})",
        store.display(),
        report.display()
    );
    let status = common::git(env, &["status", "--porcelain", "-uall"]);
    assert!(
        status.status.success(),
        "git status failed: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    let porcelain = String::from_utf8_lossy(&status.stdout);
    let named: Vec<&str> = porcelain
        .lines()
        .filter(|line| line.contains("givn/history/"))
        .collect();
    assert!(
        named.is_empty(),
        "the repository reports a pending change under the record directory:\n{porcelain}"
    );
    // Not a vacuous claim: both the store and the generated report are
    // ignored, not merely absent from the listing.
    for path in [
        "givn/history/events.jsonl",
        "givn/history/reports/demo.html",
    ] {
        let ignored = common::git(env, &["check-ignore", "-q", path]);
        assert!(
            ignored.status.success(),
            "the repository does not ignore {path}:\n{porcelain}"
        );
    }
}

#[then("the Fossil checkout reports no extra file under the record directory")]
fn fossil_checkout_reports_no_extra_file_under_the_record_directory(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let store = support::store_file(&env.root());
    assert!(
        store.is_file(),
        "the scenario's own command wrote no record at {}: the assertion below would be vacuous",
        store.display()
    );
    let extras = common::fossil(env, &["extras"]);
    assert!(
        extras.status.success(),
        "fossil extras failed: {}",
        String::from_utf8_lossy(&extras.stderr)
    );
    let listing = String::from_utf8_lossy(&extras.stdout);
    let named: Vec<&str> = listing
        .lines()
        .filter(|line| line.contains("givn/history/"))
        .collect();
    assert!(
        named.is_empty(),
        "the Fossil checkout reports an extra file under the record directory:\n{listing}"
    );
}

#[when("a lifecycle command runs")]
fn a_lifecycle_command_runs(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["status", "--change", "demo"]);
    world.last_command = Some("status".to_string());
    world.last_output = Some(out);
}

#[then("the repository's ignore glob still names the development record")]
fn repository_ignore_glob_still_names_the_development_record(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert_ignore_glob_names_record(env);
}

#[then("the repository's ignore glob covers the generated reports")]
fn repository_ignore_glob_covers_the_generated_reports(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let report = env.path("givn/history/reports/demo.html");
    assert!(
        report.is_file(),
        "the timeline run wrote no report at {}: the assertion below would be vacuous",
        report.display()
    );
    let entries = fossil_glob_entries(&env.path(".fossil-settings/ignore-glob"));
    assert!(
        entries.iter().any(|entry| entry == "givn/history/*"),
        "the managed ignore file does not name the record directory the reports live in:\n{entries:#?}"
    );
    let setting = common::fossil(env, &["settings", "--value", "ignore-glob"]);
    assert!(
        setting.status.success(),
        "read the repository ignore-glob setting: {}",
        String::from_utf8_lossy(&setting.stderr)
    );
    let setting = String::from_utf8_lossy(&setting.stdout);
    assert!(
        setting
            .split([',', '\n'])
            .any(|entry| entry.trim() == "givn/history/*"),
        "the effective ignore-glob setting does not cover the report directory: {setting:?}"
    );
    let extras = common::fossil(env, &["extras"]);
    assert!(
        extras.status.success(),
        "fossil extras failed: {}",
        String::from_utf8_lossy(&extras.stderr)
    );
    let listing = String::from_utf8_lossy(&extras.stdout);
    assert!(
        !listing.contains("givn/history/reports/demo.html"),
        "the Fossil checkout reports the generated report as an extra file:\n{listing}"
    );
}

/// Write the project-owned `record:` block into `givn/config.yaml`, replacing
/// an existing block rather than duplicating a top-level key.
fn set_record_mode(env: &common::TestEnv, mode: &str) {
    let path = env.path("givn/config.yaml");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let mut kept: Vec<String> = Vec::new();
    let mut skip = false;
    let mut replaced = false;
    for line in text.lines() {
        let top_level_record = !line.starts_with([' ', '\t', '#']) && line.starts_with("record:");
        if top_level_record {
            kept.push("record:".to_string());
            kept.push(format!("  mode: {mode}"));
            skip = true;
            replaced = true;
            continue;
        }
        if skip {
            if line.starts_with("  ") || line.starts_with('\t') || line.trim().is_empty() {
                continue;
            }
            skip = false;
        }
        kept.push(line.to_string());
    }
    if !replaced {
        kept.push("record:".to_string());
        kept.push(format!("  mode: {mode}"));
    }
    std::fs::write(&path, kept.join("\n") + "\n")
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

/// Remove every record-side ignore block the fixture's earlier commands wrote,
/// so the off-mode assertion observes the lifecycle command's own behaviour
/// instead of a block that was already there.
fn clear_record_ignore_blocks(env: &common::TestEnv) {
    for path in [env.path(".git/info/exclude"), env.path(".gitignore")] {
        remove_history_block(&path);
        assert!(
            history_block(&std::fs::read_to_string(&path).unwrap_or_default()).is_none(),
            "the fixture could not clear the record block in {}",
            path.display()
        );
    }
}

#[given(regex = r#"^a change "([^"]+)" whose record already holds entries$"#)]
fn change_whose_record_already_holds_entries(world: &mut GivnWorld, id: String) {
    // A real Git worktree, so the record-side ignore write the off mode must
    // skip has a destination the assertion can observe.
    let env = common::make_init_root();
    for args in [
        vec!["new".to_string(), id.clone()],
        vec!["status".to_string(), "--change".to_string(), id.clone()],
    ] {
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let out = common::run_givn(&env, &argv);
        assert!(
            out.status.success(),
            "givn {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
    }
    let entries = support::record_entries(&env.root());
    assert!(
        !entries.is_empty(),
        "the fixture recorded no entries at all: {entries:#?}"
    );
    let state = world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default);
    let scoped: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["scope"] == json!({ "kind": "change", "id": id }))
        .collect();
    assert!(
        !scoped.is_empty(),
        "the fixture recorded no entry for the change: {entries:#?}"
    );
    state.earlier_ids = scoped
        .iter()
        .map(|entry| {
            entry["id"]
                .as_u64()
                .unwrap_or_else(|| panic!("an entry carries no numeric id: {entry}"))
        })
        .collect();
    state.store_before = Some(support::store_bytes(&env.root()));
    world.last_change_id = Some(id);
    world.env = Some(env);
}

#[when(regex = r#"^the project sets the record mode to "([^"]+)"$"#)]
fn project_sets_the_record_mode(world: &mut GivnWorld, mode: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    set_record_mode(env, &mode);
    // The fixture clears the blocks earlier commands wrote, so "no ignore
    // entry is written" is about the command under observation.
    clear_record_ignore_blocks(env);
}

#[when(regex = r#"^a lifecycle command runs for "([^"]+)"$"#)]
fn a_lifecycle_command_runs_for(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["status", "--change", &id]);
    world.last_command = Some("status".to_string());
    world.last_output = Some(out);
}

#[then("the record gains no new entry")]
fn the_record_gains_no_new_entry(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = world
        .development_record
        .as_ref()
        .and_then(|state| state.store_before.clone())
        .expect("the recorded-entries fixture ran first");
    let after = support::store_bytes(&env.root());
    assert_eq!(
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&after),
        "the command appended to the record although the mode is off"
    );
}

#[then("the record directory gains no pending state")]
fn the_record_directory_gains_no_pending_state(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let pending = env.path("givn/history/pending");
    let entries = std::fs::read_dir(&pending)
        .map(|entries| entries.count())
        .unwrap_or(0);
    assert_eq!(
        entries,
        0,
        "the off-mode command created pending state under {}",
        pending.display()
    );
    // Not vacuous: the record's own directory still holds the earlier entries.
    assert!(
        support::store_file(&env.root()).is_file(),
        "the fixture's earlier entries are gone: the assertion above would be vacuous"
    );
}

#[then("no record-side ignore entry is written")]
fn no_record_side_ignore_entry_is_written(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    for path in [env.path(".git/info/exclude"), env.path(".gitignore")] {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            history_block(&text).is_none(),
            "the off-mode command wrote the record's ignore block to {}",
            path.display()
        );
    }
}

#[then(regex = r#"^the developer still reads the earlier entries for "([^"]+)"$"#)]
fn developer_still_reads_the_earlier_entries(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let earlier = world
        .development_record
        .as_ref()
        .map(|state| state.earlier_ids.clone())
        .expect("the recorded-entries fixture ran first");
    assert!(
        !earlier.is_empty(),
        "the fixture recorded no earlier entries to read back"
    );
    let out = common::run_givn(env, &["history", "--change", &id, "--json"]);
    assert!(
        out.status.success(),
        "reading the earlier entries failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let entries: Vec<Value> = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|error| panic!("the read view did not print JSON: {error}"));
    let ids: BTreeSet<u64> = entries
        .iter()
        .filter_map(|entry| entry["id"].as_u64())
        .collect();
    for id in &earlier {
        assert!(
            ids.contains(id),
            "lowering the mode hid the earlier entry {id}: {entries:#?}"
        );
    }
    // The lowering is forward-looking: the read view appended nothing.
    let before = world
        .development_record
        .as_ref()
        .and_then(|state| state.store_before.clone())
        .expect("the recorded-entries fixture ran first");
    assert_eq!(
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&support::store_bytes(&env.root())),
        "the read view changed the record"
    );
    world.last_output = Some(out);
}

#[when("the agent reports a tool call with its complete input through the intake")]
fn agent_reports_a_tool_call_with_complete_input(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let payload = support::intake_payload(
        Some("demo"),
        None,
        vec![json!({
            "kind": "tool",
            "at": "2026-10-17T09:00:00.000Z",
            "tool": "Bash",
            "label": "cargo test --locked",
            "outcome": "ok",
            "content": {
                "tool_input": {
                    "text": "cargo test --locked",
                    "truncated": false,
                    "bytes": 19
                }
            }
        })],
    );
    // The one-shot override would raise a live project to `full`; under `off`
    // it must be ignored with the same one warning.
    let out = support::report_intake_with_args(env, &payload, &["--with-full-content"]);
    world.last_output = Some(out);
}

#[then("the intake exits 0 and warns once that the record is off")]
fn intake_exits_zero_and_warns_once_that_the_record_is_off(world: &mut GivnWorld) {
    let out = world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet");
    assert!(
        out.status.success(),
        "the off-mode intake did not exit 0: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let lines = String::from_utf8_lossy(&out.stderr);
    let warnings: Vec<&str> = lines
        .lines()
        .filter(|line| line.contains("record is off"))
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "expected exactly one warning that the record is off: {lines:?}"
    );
    assert!(
        warnings[0].starts_with("warning:"),
        "the line is not a warning: {}",
        warnings[0]
    );
}

#[given(regex = r#"^a change "([^"]+)" whose project sets an unknown record mode$"#)]
fn change_whose_project_sets_an_unknown_record_mode(world: &mut GivnWorld, id: String) {
    let env = common::make_init_root();
    let out = common::run_givn(&env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    // The project-owned key carries a word no mode names: the record's own
    // resolver must warn and continue as the default, never fail the config.
    set_record_mode(&env, "sometimes");
    world.last_change_id = Some(id);
    world.env = Some(env);
}

#[then("the command warns once that the record falls back to the default mode")]
fn command_warns_once_that_the_record_falls_back_to_the_default(world: &mut GivnWorld) {
    let out = world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet");
    assert!(
        out.status.success(),
        "the lifecycle command failed under an unknown mode: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let lines = String::from_utf8_lossy(&out.stderr);
    let warnings: Vec<&str> = lines
        .lines()
        .filter(|line| line.starts_with("warning:"))
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "expected exactly one warning for the unknown mode (never one per resolver): {lines:?}"
    );
    let warning = warnings[0];
    assert!(
        warning.contains("record.mode"),
        "the warning does not name the key: {warning}"
    );
    assert!(
        warning.contains("falls back to the default mode") && warning.contains("(on)"),
        "the warning does not name the fallback: {warning}"
    );
}

#[then("the record holds the command's bounded metadata")]
fn record_holds_the_commands_bounded_metadata(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let entry = recorded_command_entry(world, &entries).clone();
    let change = world
        .last_change_id
        .clone()
        .expect("the unknown-mode fixture ran first");
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "the fallback command was not recorded as a run: {entry}"
    );
    assert_eq!(
        entry["scope"],
        json!({ "kind": "change", "id": change }),
        "the entry is not scoped to the change: {entry}"
    );
    let label = entry["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the command entry carries no label: {entry}"));
    assert!(
        label.chars().count() <= 120,
        "the label is not bounded to 120 characters ({}): {entry}",
        label.chars().count()
    );
    assert!(
        entry["duration_ms"].as_u64().is_some(),
        "the command entry carries no measured duration: {entry}"
    );
    assert_eq!(
        entry["data"]["exit_code"],
        json!(0),
        "the command entry does not carry its exit code: {entry}"
    );
}

#[then(regex = r#"^`givn history settings --json` reports mode "([^"]+)" with source "([^"]+)"$"#)]
fn settings_reports_mode_and_source(world: &mut GivnWorld, mode: String, source: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = support::store_bytes(&env.root());
    let out = common::run_givn(env, &["history", "settings", "--json"]);
    assert!(
        out.status.success(),
        "the settings query failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let payload: Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|error| panic!("the settings query did not print JSON: {error}"));
    assert_eq!(
        payload["mode"],
        json!(mode),
        "the settings query reported the wrong mode: {payload}"
    );
    assert_eq!(
        payload["source"],
        json!(source),
        "the settings query reported the wrong source: {payload}"
    );
    // The query is a read view: it appends no `command` entry.
    assert_eq!(
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&support::store_bytes(&env.root())),
        "the settings query changed the record"
    );
}

/// The turn metadata the default-capture fixture reports: the model, the
/// harness's stop reason, its cost, and its context reading.
const CAPTURE_TURN_INDEX: u64 = 12;
const CAPTURE_TURN_MODEL: &str = "deepseek/deepseek-flash";
const CAPTURE_STOP_REASON: &str = "end_turn";
const CAPTURE_COST: f64 = 0.0123;
const CAPTURE_CONTEXT_SIZE: u64 = 48_000;
const CAPTURE_CONTEXT_WINDOW: u64 = 128_000;
const CAPTURE_REASONING: u64 = 64;
const CAPTURE_TOOL_CALL: &str = "call-1";
const CAPTURE_OUTPUT_BYTES: u64 = 4_096;

/// The failed tool's error text: its first line is longer than the record's
/// error bound, so the intake must keep only the bounded first line.
fn overlong_tool_error() -> String {
    format!(
        "{}\nthis second line must never reach the record",
        "E".repeat(givn::history::ERROR_TEXT_LIMIT + 500)
    )
}

#[given("the project records in the default mode")]
fn project_records_in_the_default_mode(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let config =
        std::fs::read_to_string(env.path("givn/config.yaml")).expect("read givn/config.yaml");
    assert!(
        !config.lines().any(|line| {
            !line.starts_with([' ', '\t', '#']) && line.trim_start().starts_with("record:")
        }),
        "the fixture must start without an active record.mode key: {config}"
    );
    let out = common::run_givn(env, &["history", "settings", "--json"]);
    assert!(
        out.status.success(),
        "the settings query failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let payload: Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|error| panic!("the settings query did not print JSON: {error}"));
    assert_eq!(
        payload["mode"],
        json!("on"),
        "the fixture is not on: {payload}"
    );
    assert_eq!(
        payload["source"],
        json!("default"),
        "the fixture's mode does not come from the default: {payload}"
    );
    // The scenario drives the generated omp reporter, so the fixture targets
    // omp: its reporter asset is installed by the real `givn init`.
    support::initialize_with_omp_reporter(env);
}

#[when(
    regex = r#"^the omp reporter reports a completed turn and a failed tool call for "([^"]+)"$"#
)]
fn omp_reporter_reports_turn_and_failed_tool(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = support::drive_omp_reporter(
        env,
        &[
            support::HarnessActivity::TurnMetadata {
                index: CAPTURE_TURN_INDEX,
                model: CAPTURE_TURN_MODEL.to_string(),
                stop_reason: CAPTURE_STOP_REASON.to_string(),
                cost: json!(CAPTURE_COST),
                context_size: CAPTURE_CONTEXT_SIZE,
                context_window: CAPTURE_CONTEXT_WINDOW,
                usage: Some(json!({
                    "input": 12_043,
                    "output": 812,
                    "reasoning": CAPTURE_REASONING,
                    "cache_read": 9_000,
                    "cache_write": 0,
                    "total_tokens": 21_855,
                })),
            },
            support::HarnessActivity::FailedTool {
                id: CAPTURE_TOOL_CALL.to_string(),
                tool: "Bash".to_string(),
                args: json!({ "command": "cargo test --locked" }),
                error: overlong_tool_error(),
                output_bytes: CAPTURE_OUTPUT_BYTES,
            },
            support::HarnessActivity::Shutdown,
        ],
    );
    assert!(
        out.status.success(),
        "the omp reporter driver failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_change_id = Some(id);
    world.last_output = Some(out);
}

#[then("the turn entry names its model, its cost, its context size, and its stop reason")]
fn turn_entry_names_its_metadata(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the metadata fixture ran first");
    let entries = support::record_entries(&env.root());
    let turns: Vec<&Value> = entries
        .iter()
        .filter(|entry| {
            entry["kind"] == json!("turn")
                && entry["source"] == json!("agent")
                && entry["scope"] == json!({ "kind": "change", "id": change })
        })
        .collect();
    let turn = turns
        .last()
        .unwrap_or_else(|| panic!("the reporter recorded no turn entry: {entries:#?}"));
    let data = &turn["data"];
    assert_eq!(
        data["model"],
        json!(CAPTURE_TURN_MODEL),
        "the turn does not name its model: {turn}"
    );
    assert_eq!(
        data["cost"],
        json!(CAPTURE_COST),
        "the turn does not name its cost: {turn}"
    );
    assert_eq!(
        data["context_size"],
        json!(CAPTURE_CONTEXT_SIZE),
        "the turn does not name its context size: {turn}"
    );
    assert_eq!(
        data["context_window"],
        json!(CAPTURE_CONTEXT_WINDOW),
        "the turn does not name its context window: {turn}"
    );
    assert_eq!(
        data["stop_reason"],
        json!(CAPTURE_STOP_REASON),
        "the turn does not name its stop reason: {turn}"
    );
    assert_eq!(
        data["index"],
        json!(CAPTURE_TURN_INDEX),
        "the turn does not name its index: {turn}"
    );
    assert_eq!(
        data["usage"]["reasoning"],
        json!(CAPTURE_REASONING),
        "the turn's usage does not carry its reasoning count: {turn}"
    );
    assert_eq!(
        turn["session"],
        json!(support::DRIVER_SESSION),
        "the entry envelope does not carry the harness session: {turn}"
    );
}

#[then(
    "the failed tool entry keeps its error text within the documented bound, its tool call id, and its output size"
)]
fn failed_tool_entry_keeps_bounded_metadata(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the metadata fixture ran first");
    let entries = support::record_entries(&env.root());
    let tools: Vec<&Value> = entries
        .iter()
        .filter(|entry| {
            entry["kind"] == json!("tool")
                && entry["source"] == json!("agent")
                && entry["scope"] == json!({ "kind": "change", "id": change })
        })
        .collect();
    let tool = tools
        .last()
        .unwrap_or_else(|| panic!("the reporter recorded no tool entry: {entries:#?}"));
    assert_eq!(
        tool["outcome"],
        json!("failed"),
        "the failed tool call is not recorded as failed: {tool}"
    );
    assert_eq!(
        tool["data"]["tool_call_id"],
        json!(CAPTURE_TOOL_CALL),
        "the tool entry does not carry its harness call id: {tool}"
    );
    assert_eq!(
        tool["data"]["output_bytes"],
        json!(CAPTURE_OUTPUT_BYTES),
        "the tool entry does not carry its output size: {tool}"
    );
    assert_eq!(
        tool["data"]["duration_source"],
        json!("reporter"),
        "the tool entry does not name where its duration came from: {tool}"
    );
    let error = tool["data"]["error"]
        .as_str()
        .unwrap_or_else(|| panic!("the failed tool entry carries no error text: {tool}"));
    assert!(
        error.chars().count() <= givn::history::ERROR_TEXT_LIMIT,
        "the error text exceeds the documented bound: {tool}"
    );
    assert!(
        error.ends_with('…'),
        "the bounded error text does not mark its truncation: {tool}"
    );
    let kept = error.trim_end_matches('…');
    assert!(
        overlong_tool_error().starts_with(kept),
        "the bounded error text is not the first line of the reported error: {tool}"
    );
    assert!(
        !error.contains("second line"),
        "the intake kept a later error line: {tool}"
    );
}

/// The question the Maki transcript fixture carries: longer than both the entry
/// label bound and the 4 000-character question cap.
fn maki_question() -> String {
    format!(
        "{} — choose the next step",
        "Q".repeat(givn::history::QUESTION_TEXT_LIMIT + 500)
    )
}

/// The option labels the fixture's question carries.
const MAKI_OPTIONS: [&str; 2] = ["Continue with the change", "Stop here"];

/// The text the reporter joins and the intake stores: every question and option
/// label, one per line, in the reporter's order.
fn maki_joined_question() -> String {
    std::iter::once(maki_question())
        .chain(MAKI_OPTIONS.iter().map(|option| option.to_string()))
        .collect::<Vec<String>>()
        .join("\n")
}

/// The last question entry the Maki reporter recorded for the scenario's
/// change: the wait's own entry, never a fixture-written line.
fn last_answered_question(world: &GivnWorld) -> Value {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the question fixture ran first");
    let entries = support::record_entries(&env.root());
    let questions: Vec<&Value> = entries
        .iter()
        .filter(|entry| {
            entry["kind"] == json!("question")
                && entry["source"] == json!("agent")
                && entry["scope"] == json!({ "kind": "change", "id": change })
        })
        .collect();
    questions
        .last()
        .map(|entry| (*entry).clone())
        .unwrap_or_else(|| panic!("the Maki reporter recorded no question entry: {entries:#?}"))
}

#[given(
    regex = r#"^a change "([^"]+)" whose Maki reporter reads a question from its session transcript$"#
)]
fn change_with_maki_question_transcript(world: &mut GivnWorld, id: String) {
    // The reporter is installed by the real `givn init --agents maki` in a
    // fresh root, never a hand-written file, and driven through the real
    // `maki` binary.
    let env = common::make_bare_root();
    support::initialize_with_maki_reporter(&env);
    let out = common::run_givn(&env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let state = world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default);
    state.question_text = Some(maki_joined_question());
    state.question_title = Some(maki_question());
    world.last_change_id = Some(id);
    world.env = Some(env);
}

#[when("the reported question is longer than the entry label bound")]
fn reported_question_longer_than_the_label_bound(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let question = maki_question();
    let transcript = support::maki_question_transcript(Some(&question), &MAKI_OPTIONS);
    let out = support::drive_maki_question_wait(env, &question, &transcript);
    world.last_output = Some(out);
}

#[then("the record holds the complete question text up to the 4 000-character cap")]
fn record_holds_the_question_text_capped(world: &mut GivnWorld) {
    let state = world
        .development_record
        .as_ref()
        .expect("the question fixture ran first");
    let expected = state
        .question_text
        .clone()
        .expect("the question fixture carries no text");
    assert!(
        expected.chars().count() > givn::history::QUESTION_TEXT_LIMIT,
        "the fixture's question must exceed the cap: {} chars",
        expected.chars().count()
    );
    let entry = last_answered_question(world);
    let question = entry
        .pointer("/data/question")
        .unwrap_or_else(|| panic!("the question entry carries no full text: {entry}"));
    let text = question["text"]
        .as_str()
        .unwrap_or_else(|| panic!("the question text is not a string: {entry}"));
    assert!(
        text.chars().count() <= givn::history::QUESTION_TEXT_LIMIT,
        "the stored question text exceeds the cap ({} chars): {entry}",
        text.chars().count()
    );
    let capped: String = expected
        .chars()
        .take(givn::history::QUESTION_TEXT_LIMIT)
        .collect();
    assert_eq!(
        text, capped,
        "the stored question text is not the reported text up to the cap: {entry}"
    );
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(entry);
}

#[then("the entry marks the text truncated with its original size")]
fn entry_marks_the_question_truncated(world: &mut GivnWorld) {
    let state = world
        .development_record
        .as_ref()
        .expect("the question fixture ran first");
    let expected = state
        .question_text
        .clone()
        .expect("the question fixture carries no text");
    let entry = state
        .last_entry
        .clone()
        .expect("no question entry was identified by the previous step");
    let question = &entry["data"]["question"];
    assert_eq!(
        question["truncated"],
        json!(true),
        "the entry does not mark the text truncated: {entry}"
    );
    assert_eq!(
        question["bytes"],
        json!(expected.len()),
        "the entry does not keep the original byte size: {entry}"
    );
}

#[then("the entry's label stays within the label bound")]
fn entry_label_stays_within_the_label_bound(world: &mut GivnWorld) {
    let state = world
        .development_record
        .as_ref()
        .expect("the question fixture ran first");
    let title = state
        .question_title
        .clone()
        .expect("the question fixture carries no title");
    let entry = state
        .last_entry
        .clone()
        .expect("no question entry was identified by the previous step");
    let label = entry["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the question entry carries no label: {entry}"));
    assert!(
        label.chars().count() <= givn::history::LABEL_LIMIT,
        "the label is not bounded to {} characters ({}): {entry}",
        givn::history::LABEL_LIMIT,
        label.chars().count()
    );
    assert!(
        label.ends_with('…'),
        "the label does not mark its truncation: {entry}"
    );
    let kept = label.trim_end_matches('…');
    assert!(
        title.starts_with(kept),
        "the label is not the wait title's bounded first line: {entry}"
    );
    let metadata = entry["data"]["title"]
        .as_str()
        .unwrap_or_else(|| panic!("the question entry carries no bounded title: {entry}"));
    assert!(
        metadata.chars().count() <= givn::history::TEXT_METADATA_LIMIT,
        "the title metadata exceeds its documented bound: {entry}"
    );
}

#[when("the Maki session transcript holds no matching question")]
fn maki_transcript_holds_no_matching_question(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let title = "Waiting for the operator";
    let transcript = support::maki_question_transcript(None, &[]);
    let out = support::drive_maki_question_wait(env, title, &transcript);
    world.last_output = Some(out);
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .residual_title = Some(title.to_string());
}

#[then("the record keeps the bounded title label and no full question text")]
fn record_keeps_the_title_label_without_question_text(world: &mut GivnWorld) {
    let state = world
        .development_record
        .as_ref()
        .expect("the residual fixture ran first");
    let title = state
        .residual_title
        .clone()
        .expect("the residual fixture carries no title");
    let entry = last_answered_question(world);
    assert_eq!(
        entry["label"],
        json!(title),
        "the residual wait does not keep its bounded title label: {entry}"
    );
    assert!(
        entry["data"].get("question").is_none(),
        "the residual wait invented a full question text: {entry}"
    );
    assert_eq!(
        entry["data"]["title"],
        json!(title),
        "the residual wait does not keep its bounded title metadata: {entry}"
    );
    assert!(
        entry["duration_ms"].as_u64().is_some(),
        "the residual wait lost its measured duration: {entry}"
    );
}

/// The complete content the default-mode fixture reports: the markers must not
/// survive into the store, while the tool call's metadata must.
const DEFAULT_CONTENT_INPUT: &str = "cargo test --locked --test features_runner";
const DEFAULT_CONTENT_OUTPUT: &str = "test result: ok. 272 passed; 0 failed";
const DEFAULT_TOOL_MS: u64 = 1_240;

#[given(regex = r#"^a change "([^"]+)" whose project has no record mode configured$"#)]
fn change_without_record_mode(world: &mut GivnWorld, id: String) {
    let env = common::make_init_root();
    let out = common::run_givn(&env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let config =
        std::fs::read_to_string(env.path("givn/config.yaml")).expect("read givn/config.yaml");
    assert!(
        !config.lines().any(|line| {
            !line.starts_with([' ', '\t', '#']) && line.trim_start().starts_with("record:")
        }),
        "the fixture must set no record mode: {config}"
    );
    world.last_change_id = Some(id);
    world.env = Some(env);
}

#[when("the agent reports a tool call with its complete input and output")]
fn agent_reports_a_tool_call_with_complete_content(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the default-mode fixture ran first");
    let payload = support::intake_payload(
        Some(&change),
        None,
        vec![json!({
            "kind": "tool",
            "at": "2026-10-17T10:00:00.000Z",
            "tool": "Bash",
            "label": "cargo test --locked",
            "duration_ms": DEFAULT_TOOL_MS,
            "outcome": "ok",
            "tool_call_id": "default-content-call",
            "content": {
                "tool_input": {
                    "text": DEFAULT_CONTENT_INPUT,
                    "truncated": false,
                    "bytes": DEFAULT_CONTENT_INPUT.len()
                },
                "tool_output": {
                    "text": DEFAULT_CONTENT_OUTPUT,
                    "truncated": false,
                    "bytes": DEFAULT_CONTENT_OUTPUT.len()
                }
            }
        })],
    );
    let out = support::report_intake(env, &payload);
    assert!(
        out.status.success(),
        "the intake failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_output = Some(out);
}

#[then("the record drops the complete content")]
fn record_drops_the_complete_content(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let bytes = support::store_bytes(&env.root());
    let store = String::from_utf8_lossy(&bytes);
    for marker in [DEFAULT_CONTENT_INPUT, DEFAULT_CONTENT_OUTPUT] {
        assert!(
            !store.contains(marker),
            "the store carries the complete content: {store}"
        );
    }
    // Not vacuous: the tool call itself was recorded.
    let entries = support::record_entries(&env.root());
    assert!(
        entries
            .iter()
            .any(|entry| entry["kind"] == json!("tool") && entry["name"] == json!("Bash")),
        "the default-mode intake dropped the event instead of its content: {entries:#?}"
    );
}

#[then("no entry carries a data.content object")]
fn no_entry_carries_a_content_object(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let carrying: Vec<&Value> = entries
        .iter()
        .filter(|entry| entry["data"].get("content").is_some())
        .collect();
    assert!(
        carrying.is_empty(),
        "the default mode stored a content object: {carrying:#?}"
    );
}

#[then("the record keeps the tool call's metadata")]
fn record_keeps_the_tool_calls_metadata(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the default-mode fixture ran first");
    let entries = support::record_entries(&env.root());
    let tool = entries
        .iter()
        .find(|entry| {
            entry["kind"] == json!("tool")
                && entry["source"] == json!("agent")
                && entry["scope"] == json!({ "kind": "change", "id": change })
        })
        .unwrap_or_else(|| panic!("the intake recorded no tool entry: {entries:#?}"));
    assert_eq!(tool["name"], json!("Bash"), "the tool name is gone: {tool}");
    assert_eq!(
        tool["label"],
        json!("cargo test --locked"),
        "the bounded label is gone: {tool}"
    );
    assert_eq!(
        tool["duration_ms"],
        json!(DEFAULT_TOOL_MS),
        "the measured duration is gone: {tool}"
    );
    assert_eq!(
        tool["outcome"],
        json!("ok"),
        "the reported outcome is gone: {tool}"
    );
    assert_eq!(
        tool["session"],
        json!("fixture-session"),
        "the payload's session envelope is gone: {tool}"
    );
    assert_eq!(
        tool["data"]["tool_call_id"],
        json!("default-content-call"),
        "the tool call id is gone: {tool}"
    );
}

/// The complete input the full-mode fixture reports: larger than the per-field
/// content cap by design, so the intake's cut is observable.
fn over_cap_input() -> String {
    "I".repeat(givn::history::CONTENT_FIELD_LIMIT + 4_096)
}

/// The last agent tool entry the scenario's intake recorded.
fn last_recorded_tool(world: &GivnWorld) -> Value {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the full-mode fixture ran first");
    let entries = support::record_entries(&env.root());
    let tools: Vec<&Value> = entries
        .iter()
        .filter(|entry| {
            entry["kind"] == json!("tool")
                && entry["source"] == json!("agent")
                && entry["scope"] == json!({ "kind": "change", "id": change })
        })
        .collect();
    tools
        .last()
        .map(|entry| (*entry).clone())
        .unwrap_or_else(|| panic!("the intake recorded no tool entry: {entries:#?}"))
}

#[given(regex = r#"^a change "([^"]+)" whose project sets the record mode to "([^"]+)"$"#)]
fn change_whose_project_sets_the_record_mode(world: &mut GivnWorld, id: String, mode: String) {
    let env = common::make_init_root();
    let out = common::run_givn(&env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    set_record_mode(&env, &mode);
    let query = common::run_givn(&env, &["history", "settings", "--json"]);
    assert!(
        query.status.success(),
        "the settings query failed: {}",
        String::from_utf8_lossy(&query.stderr)
    );
    let payload: Value = serde_json::from_slice(&query.stdout)
        .unwrap_or_else(|error| panic!("the settings query did not print JSON: {error}"));
    assert_eq!(
        payload,
        json!({ "mode": mode, "source": "config" }),
        "the fixture's mode is not the configured one"
    );
    world.last_change_id = Some(id);
    world.env = Some(env);
}

#[when("the agent reports a tool call whose complete input exceeds the per-field cap")]
fn agent_reports_a_tool_call_over_the_field_cap(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = world
        .last_change_id
        .clone()
        .expect("the full-mode fixture ran first");
    let input = over_cap_input();
    let output = "O".repeat(givn::history::CONTENT_FIELD_LIMIT + 4_096);
    let assistant = "A".repeat(givn::history::CONTENT_FIELD_LIMIT);
    let thinking = "T".repeat(givn::history::CONTENT_FIELD_LIMIT);
    let prompt = "U".repeat(givn::history::CONTENT_FIELD_LIMIT);
    let payload = support::intake_payload(
        Some(&change),
        None,
        vec![json!({
            "kind": "tool",
            "at": "2026-10-17T10:30:00.000Z",
            "tool": "Bash",
            "label": "large input",
            "duration_ms": 5,
            "outcome": "ok",
            "content": {
                "tool_input": { "text": input, "truncated": false, "bytes": over_cap_input().len() },
                "tool_output": { "text": output, "truncated": false, "bytes": output.len() },
                "assistant_text": { "text": assistant, "truncated": false, "bytes": assistant.len() },
                "thinking_text": { "text": thinking, "truncated": false, "bytes": thinking.len() },
                "user_prompt": { "text": prompt, "truncated": false, "bytes": prompt.len() }
            }
        })],
    );
    let out = support::report_intake(env, &payload);
    assert!(
        out.status.success(),
        "the full-mode intake failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_output = Some(out);
}

#[then("the record keeps the input up to the 64 KiB per-field cap")]
fn record_keeps_the_input_up_to_the_field_cap(world: &mut GivnWorld) {
    let entry = last_recorded_tool(world);
    let content = entry
        .pointer("/data/content")
        .unwrap_or_else(|| panic!("the full-mode entry carries no content: {entry}"));
    let field = &content["tool_input"];
    let text = field["text"]
        .as_str()
        .unwrap_or_else(|| panic!("the tool input field carries no text: {entry}"));
    assert_eq!(
        text.len(),
        givn::history::CONTENT_FIELD_LIMIT,
        "the stored input is not the per-field cap: {entry}"
    );
    assert_eq!(
        text,
        over_cap_input()
            .chars()
            .take(givn::history::CONTENT_FIELD_LIMIT)
            .collect::<String>(),
        "the stored input is not the reported text up to the cap: {entry}"
    );
    assert_eq!(
        field["truncated"],
        json!(true),
        "the cut input is not marked truncated: {entry}"
    );
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(entry);
}

#[then("the content field marks truncation and records its original size")]
fn content_field_marks_truncation_and_original_size(world: &mut GivnWorld) {
    let entry = world
        .development_record
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no full-mode entry was identified by the previous step");
    let field = &entry["data"]["content"]["tool_input"];
    assert_eq!(
        field["truncated"],
        json!(true),
        "the content field does not mark its truncation: {entry}"
    );
    assert_eq!(
        field["bytes"],
        json!(over_cap_input().len()),
        "the content field does not record its original byte size: {entry}"
    );
}

#[given(regex = r#"^a change "([^"]+)" whose proposal\.md appears while a recorded command runs$"#)]
fn change_proposal_appears_during_a_recorded_command(world: &mut GivnWorld, id: String) {
    let env = common::make_init_root();
    let out = support::create_change_during_a_recorded_command(&env, &id);
    world.env = Some(env);
    world.last_change_id = Some(id);
    world.last_command = Some("new".to_string());
    world.last_output = Some(out);
}

#[when("the command finishes")]
fn the_recorded_command_finishes(world: &mut GivnWorld) {
    let out = world
        .last_output
        .as_ref()
        .expect("no recorded command has run yet");
    assert!(
        out.status.success(),
        "the recorded command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let id = world
        .last_change_id
        .clone()
        .expect("the fixture created no change");
    let proposal = env.path(&format!("givn/changes/{id}/proposal.md"));
    assert!(
        proposal.is_file(),
        "the artifact the command was supposed to create does not exist: {}",
        proposal.display()
    );
}

#[then(regex = r#"^the record holds one phase entry labelled "([^"]+)" for "([^"]+)"$"#)]
fn record_holds_one_phase_entry(world: &mut GivnWorld, label: String, change: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::record_entries(&env.root());
    let transitions = support::transitions_for(&entries, &change, &label);
    assert_eq!(
        transitions.len(),
        1,
        "expected exactly one `{label}` transition for `{change}`; the record holds: {entries:#?}"
    );
    let entry = transitions[0];
    assert_eq!(
        entry["name"],
        json!(label),
        "the transition's name is not the artifact filename: {entry}"
    );
    assert_eq!(
        entry["source"],
        json!("givn"),
        "the transition was not recorded by Givn: {entry}"
    );
    assert_eq!(
        entry["outcome"],
        json!("ok"),
        "the transition does not record the observation as ok: {entry}"
    );
    assert!(
        entry.get("duration_ms").is_none(),
        "the transition invents a duration: {entry}"
    );
    assert!(
        entry.get("data").is_none(),
        "the transition carries data: {entry}"
    );
    let command = entries
        .iter()
        .find(|candidate| {
            candidate["kind"] == json!("command") && candidate["name"] == json!("new")
        })
        .unwrap_or_else(|| {
            panic!("the command that observed the artifact recorded no entry: {entries:#?}")
        });
    assert_eq!(
        entry["parent"], command["id"],
        "the transition is not parented to the observing command's own entry: {entry}"
    );
    world
        .development_record
        .get_or_insert_with(DevelopmentRecordState::default)
        .last_entry = Some(entry.clone());
}

#[then("a later recorded command that sees the same artifact records no second transition")]
fn later_recorded_command_records_no_second_transition(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let id = world
        .last_change_id
        .clone()
        .expect("the fixture created no change");
    let out = common::run_givn(env, &["status", "--change", &id]);
    assert!(
        out.status.success(),
        "the later recorded command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    world.last_command = Some("status".to_string());
    world.last_output = Some(out);
    let entries = support::record_entries(&env.root());
    let later_commands = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("command") && entry["name"] == json!("status"))
        .count();
    assert_eq!(
        later_commands, 1,
        "the later command was not recorded, so its observation proves nothing: {entries:#?}"
    );
    assert_eq!(
        support::transitions_for(&entries, &id, "proposal.md").len(),
        1,
        "the later command recorded a second transition for the same artifact: {entries:#?}"
    );
}

#[then("a gate check for the same artifact keeps its artifact id label and is not a transition")]
fn gate_check_keeps_its_artifact_id_label(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let id = world
        .last_change_id
        .clone()
        .expect("the fixture created no change");
    let out = common::run_givn(env, &["check", "proposal", "--change", &id]);
    assert!(
        out.status.success(),
        "givn check --artifact proposal failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let entries = support::record_entries(&env.root());
    let gates = entries
        .iter()
        .filter(|entry| entry["kind"] == json!("phase") && entry["label"] == json!("proposal"))
        .count();
    assert_eq!(
        gates, 1,
        "expected exactly one gate phase labelled with the artifact id `proposal`: {entries:#?}"
    );
    assert_eq!(
        support::transitions_for(&entries, &id, "proposal.md").len(),
        1,
        "the gate check stood in for or deduped away the artifact transition: {entries:#?}"
    );
}

#[then("the entry's content never exceeds the 256 KiB per-entry cap")]
fn entry_content_never_exceeds_the_entry_cap(world: &mut GivnWorld) {
    let entry = world
        .development_record
        .as_ref()
        .and_then(|state| state.last_entry.clone())
        .expect("no full-mode entry was identified by the previous step");
    let content = entry
        .pointer("/data/content")
        .unwrap_or_else(|| panic!("the full-mode entry carries no content: {entry}"));
    let object = content
        .as_object()
        .unwrap_or_else(|| panic!("the content is not an object: {entry}"));
    let mut total = 0usize;
    for (name, field) in object {
        if name == "truncated" {
            continue;
        }
        let text = field["text"]
            .as_str()
            .unwrap_or_else(|| panic!("the `{name}` field carries no text: {entry}"));
        assert!(
            text.len() <= givn::history::CONTENT_FIELD_LIMIT,
            "the `{name}` field exceeds the per-field cap ({} bytes): {entry}",
            text.len()
        );
        total += text.len();
    }
    assert!(
        total <= givn::history::CONTENT_ENTRY_LIMIT,
        "the entry's content exceeds the per-entry cap ({total} bytes): {entry}"
    );
    // Not vacuous: the reported bundle exceeded the cap, so the aggregate cut
    // really happened and the bundle-level marker is present.
    assert_eq!(
        total,
        givn::history::CONTENT_ENTRY_LIMIT,
        "the entry did not fill its content budget exactly: {entry}"
    );
    assert_eq!(
        content["truncated"],
        json!(true),
        "the cut bundle is not marked truncated: {entry}"
    );
}
