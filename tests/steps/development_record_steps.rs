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
