//! Step definitions for the `development-record` capability.
//!
//! The scenarios observe the development record through its on-disk contract
//! (`givn/history/events.jsonl`, one JSON object per line) in a black-box
//! project root, so nothing here reads the recorder's in-memory state.

use crate::harness as common;
use crate::steps::GivnWorld;
use crate::steps::development_timeline_support as support;
use cucumber::given;
use serde_json::Value;

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
    /// The Maki session title a reported session entry must carry.
    pub(crate) maki_session_title: Option<String>,
    /// The session a reported event names for itself.
    pub(crate) maki_event_session: Option<String>,
}

pub(crate) const RECORD_DIR: &str = "givn/history";

pub(crate) fn is_record_dir(root: &std::path::Path, path: &std::path::Path) -> bool {
    path == root.join(RECORD_DIR)
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
