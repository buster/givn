//! Step definitions for the `development-report` capability.
//!
//! The scenarios generate a report through `givn timeline` and assert on the
//! file the command wrote — the JSON model in its data slot and the text it
//! renders — never on the store alone; the record fixtures live in
//! [`crate::steps::development_timeline_support`].

use crate::harness as common;
use crate::harness::history_record::HistoryEntry;
use crate::steps::GivnWorld;
use crate::steps::development_timeline_support as support;
use cucumber::given;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub(crate) struct DevelopmentReportState {
    /// The report the last generation step wrote.
    pub(crate) report_path: Option<PathBuf>,
    /// The entries the scenario's Given seeded, ids included.
    pub(crate) seeded: Vec<HistoryEntry>,
    /// The metadata-rich tool entry the details-panel fixture seeded.
    pub(crate) metadata_entry: Option<HistoryEntry>,
    /// The content entry ids the details-panel fixture seeded.
    pub(crate) content_entry_ids: Vec<u64>,
    /// The store's bytes before the report ran, so the store-completeness
    /// assertion compares against the seeded record.
    pub(crate) store_before: Option<Vec<u8>>,
    /// The ids of the recorder's own command entries the scenario seeded, so
    /// the report's omission and the store's retention address the same rows.
    pub(crate) recorder_entry_ids: Vec<u64>,
    /// The record's entry count as seeded: the report run and the read view
    /// must each leave it unchanged.
    pub(crate) record_entry_count: Option<usize>,
    /// The ideation topic a linked change report's proposal names, so the
    /// second Given can seed its record.
    pub(crate) linked_topic: Option<String>,
}

fn state(world: &mut GivnWorld) -> &mut DevelopmentReportState {
    world
        .development_report
        .get_or_insert_with(DevelopmentReportState::default)
}

fn seeded(world: &GivnWorld) -> &[HistoryEntry] {
    world
        .development_report
        .as_ref()
        .map(|state| state.seeded.as_slice())
        .unwrap_or_default()
}

fn project_root(world: &mut GivnWorld) -> &common::TestEnv {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn run_fixture_command(env: &common::TestEnv, args: &[&str]) {
    let out = common::run_givn(env, args);
    assert!(
        out.status.success(),
        "givn {} failed: {}{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

#[given(regex = r#"^a change "([^"]+)" with no recorded activity$"#)]
fn change_with_no_recorded_activity(world: &mut GivnWorld, id: String) {
    let env = project_root(world);
    run_fixture_command(env, &["new", &id]);
    // The fixture's own claim: the change exists and its record holds nothing —
    // the setup commands' own entries are removed, so the scope's record is
    // empty rather than filtered by accident.
    support::clear_record(env);
    assert!(
        !env.root()
            .join("givn")
            .join("history")
            .join("events.jsonl")
            .exists(),
        "the fixture's record is empty"
    );
    world.last_change_id = Some(id);
}

fn recorder_entry_ids(world: &GivnWorld) -> Vec<u64> {
    world
        .development_report
        .as_ref()
        .map(|state| state.recorder_entry_ids.clone())
        .unwrap_or_default()
}

#[given(regex = r#"^a change "([^"]+)" whose proposal links the ideation topic "([^"]+)"$"#)]
fn change_whose_proposal_links_the_topic(world: &mut GivnWorld, change: String, topic: String) {
    {
        let env = project_root(world);
        support::create_linked_ideation_change(env, &change, &topic);
    }
    let state = state(world);
    state.linked_topic = Some(topic);
    world.last_change_id = Some(change);
}
