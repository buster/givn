//! Shared fixtures for the `inspect-development-history` capabilities.
//!
//! Owns the record fixtures, the seeded-history builder, and the three
//! harness drivers, so the per-capability step files bind steps while this
//! module owns how a scenario reaches the record:
//!
//! - the record fixtures read `givn/history/events.jsonl` the way a consumer
//!   does (parse the JSONL, never the recorder's in-memory state) through the
//!   existing black-box seam `common::run_givn` (the compiled binary in a
//!   `tempfile` project root);
//! - the seeded-history builder writes a fixture record with known entries;
//! - the harness drivers are a fake `pi` extension context for omp, a fake
//!   plugin context capturing `session.hook` / `tool.hook` / `shell.hook` plus
//!   the event subscription for opencode, and a payload-on-stdin invocation of
//!   the generated Claude shim with a fixture transcript.

use crate::harness as common;
use crate::harness::history_record::{
    CONTENT_FIELD_LIMIT, HistoryEntry, HistoryStore, SCHEMA, Scope,
};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub const SLOW_VERIFY_COMMAND: &str = "sleep 2";

pub fn declare_verify_command(env: &common::TestEnv, command: &str) {
    // Single-quote the YAML scalar so shell double quotes stay literal.
    let yaml = format!("verify:\n  command: '{}'\n", command.replace('\'', "''"));
    common::write_commands_yaml(&env.path("givn/commands.yaml"), &yaml);
}

pub fn project_name(env: &common::TestEnv) -> String {
    env.root()
        .file_name()
        .and_then(|name| name.to_str())
        .expect("the fixture project root has a name")
        .to_string()
}

pub fn create_change_during_a_recorded_command(
    env: &common::TestEnv,
    id: &str,
) -> std::process::Output {
    common::run_givn(env, &["new", id])
}

pub fn assert_no_active_change(env: &common::TestEnv) {
    let changes = read_dir_names(&env.path("givn/changes"));
    assert!(
        changes.is_empty(),
        "the fixture project has changes: {changes:?}"
    );
}

pub fn assert_bare_project(env: &common::TestEnv) {
    assert_no_active_change(env);
    for topic in read_dir_names(&env.path("givn/ideation")) {
        let session = env.path(&format!("givn/ideation/{topic}/session.md"));
        assert!(
            !session.exists(),
            "the fixture project has an ideation session: {}",
            session.display()
        );
    }
}

pub fn declare_unfinished_ideation_topic(env: &common::TestEnv, topic: &str) {
    let session = env.path(&format!("givn/ideation/{topic}/session.md"));
    let dir = session
        .parent()
        .expect("the session path has a directory")
        .to_path_buf();
    std::fs::create_dir_all(&dir)
        .unwrap_or_else(|error| panic!("create the topic {}: {error}", dir.display()));
    let body = "\
# Ideation session: timeline-ideas

## Phase

frame

## Progress

- frame written
- questions answered

## Next steps

- elaborate the use cases
";
    std::fs::write(&session, body)
        .unwrap_or_else(|error| panic!("write the session {}: {error}", session.display()));
}

const IDEATION_STEPS: [(&str, &str, u64); 3] = [
    ("ideation", "givn instructions ideation", 20_000),
    ("eventstorming", "givn instructions eventstorming", 600_000),
    ("elaboration", "givn instructions usecase", 120_000),
];

const IDEATION_STEP_AT: [&str; 3] = [
    "2026-10-16T08:00:00.000Z",
    "2026-10-16T08:30:00.000Z",
    "2026-10-16T09:00:00.000Z",
];

pub fn seed_ideation_topic(env: &common::TestEnv, topic: &str) -> Vec<HistoryEntry> {
    declare_unfinished_ideation_topic(env, topic);
    let scope = Scope::Ideation {
        id: topic.to_string(),
    };
    let mut history = SeededHistory::new();
    for (index, (name, label, duration_ms)) in IDEATION_STEPS.into_iter().enumerate() {
        let entry = history.span(&scope, "step", name, IDEATION_STEP_AT[index], duration_ms);
        history.label(entry, label);
    }
    history.seed(env)
}

fn ideation_fixture_at(base_minutes: u32, seconds: u32) -> String {
    format!(
        "2026-10-16T{:02}:{:02}:{:02}.000Z",
        base_minutes / 60,
        base_minutes % 60,
        seconds
    )
}

pub fn seed_ideation_topic_markers(
    env: &common::TestEnv,
    topic: &str,
    markers: &[&str],
) -> Vec<HistoryEntry> {
    declare_unfinished_ideation_topic(env, topic);
    let mut history = SeededHistory::new();
    push_ideation_topic_entries(&mut history, topic, markers);
    history.seed(env)
}

fn push_ideation_topic_entries(history: &mut SeededHistory, topic: &str, markers: &[&str]) {
    let scope = Scope::Ideation {
        id: topic.to_string(),
    };
    // The work the opening frame ran, before any marker exists.
    let frame = history.span(
        &scope,
        "step",
        "ideation",
        "2026-10-16T08:00:00.000Z",
        20_000,
    );
    history.label(frame, "givn instructions ideation");
    for (index, marker) in markers.iter().enumerate() {
        let base_minutes = 8 * 60 + 30 + index as u32 * 30;
        let started = history.point(
            &scope,
            "phase",
            marker,
            &ideation_fixture_at(base_minutes, 0),
        );
        history.label(started, marker);
        let work = history.span(
            &scope,
            "step",
            marker,
            &ideation_fixture_at(base_minutes, 30),
            600_000,
        );
        history.label(work, &format!("givn instructions {marker}"));
    }
}

pub fn create_linked_ideation_change(
    env: &common::TestEnv,
    change: &str,
    topic: &str,
) -> std::path::PathBuf {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let proposal = env
        .root()
        .join("givn")
        .join("changes")
        .join(change)
        .join("proposal.md");
    std::fs::write(
        &proposal,
        format!("# Proposal: {change}\n\n- Ideation topic: {topic}\n"),
    )
    .unwrap_or_else(|error| panic!("write the proposal {}: {error}", proposal.display()));
    declare_unfinished_ideation_topic(env, topic);
    proposal
}

pub fn create_archived_linked_ideation_change(
    env: &common::TestEnv,
    change: &str,
    topic: &str,
) -> std::path::PathBuf {
    let archived = env.root().join("givn").join("archive").join(change);
    std::fs::create_dir_all(&archived)
        .unwrap_or_else(|error| panic!("create the archived change directory: {error}"));
    let proposal = archived.join("proposal.md");
    std::fs::write(
        &proposal,
        format!("# Proposal: {change}\n\n- Ideation topic: {topic}\n"),
    )
    .unwrap_or_else(|error| panic!("write the archived proposal: {error}"));
    std::fs::write(archived.join("review.md"), "REVIEW: PASS\n")
        .unwrap_or_else(|error| panic!("write the archived review: {error}"));
    declare_unfinished_ideation_topic(env, topic);
    proposal
}

pub const UNLINKED_TOPIC: &str = "ideas";

pub fn seed_unlinked_change(env: &common::TestEnv, change: &str) -> Vec<HistoryEntry> {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let proposal = env
        .root()
        .join("givn")
        .join("changes")
        .join(change)
        .join("proposal.md");
    std::fs::write(
        &proposal,
        format!("# Proposal: {change}\n\n- Ideation topic: none\n"),
    )
    .unwrap_or_else(|error| panic!("write the proposal {}: {error}", proposal.display()));
    declare_unfinished_ideation_topic(env, UNLINKED_TOPIC);
    seed_change_record(env, change)
}

fn push_change_entries(history: &mut SeededHistory, change: &str) {
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let transition = history.point(&scope, "phase", "proposal.md", "2026-10-16T10:00:00.000Z");
    history.label(transition, "proposal.md");
    for (label, at, duration_ms) in [
        (
            "givn instructions specs",
            "2026-10-16T10:30:00.000Z",
            200_000u64,
        ),
        (
            "cargo test --locked --test features_runner",
            "2026-10-16T11:30:00.000Z",
            400_000,
        ),
    ] {
        let step = history.span(&scope, "step", label, at, duration_ms);
        history.label(step, label);
    }
}

pub fn seed_linked_ideation_record(
    env: &common::TestEnv,
    change: &str,
    topic: &str,
    markers: &[&str],
) -> Vec<HistoryEntry> {
    let mut history = SeededHistory::new();
    push_ideation_topic_entries(&mut history, topic, markers);
    push_change_entries(&mut history, change);
    history.seed(env)
}

pub const IDEATION_SCOUT_LANE: &str = "Ideation.Scout";

pub const IDEATION_WRITER_LANE: &str = "Ideation.Writer";

const IDEATION_SCOUT_SESSION: &str = "ideation-scout-1a2b3c4d";

const IDEATION_WRITER_SESSION: &str = "ideation-writer-5e6f7a8b";

pub fn seed_linked_ideation_sessions(
    env: &common::TestEnv,
    change: &str,
    topic: &str,
    markers: &[&str],
) -> Vec<HistoryEntry> {
    let mut history = SeededHistory::new();
    push_ideation_topic_entries(&mut history, topic, markers);
    push_ideation_persona_sessions(&mut history, topic);
    push_change_entries(&mut history, change);
    history.seed(env)
}

fn push_ideation_persona_sessions(history: &mut SeededHistory, topic: &str) {
    let scope = Scope::Ideation {
        id: topic.to_string(),
    };
    let scout = history.point(
        &scope,
        "session",
        "scout-session",
        "2026-10-16T08:31:00.000Z",
    );
    history.session(scout, IDEATION_SCOUT_SESSION);
    history.data(scout, serde_json::json!({ "title": IDEATION_SCOUT_LANE }));
    let scout_turn = history.span(&scope, "turn", "turn", "2026-10-16T08:32:00.000Z", 60_000);
    history.session(scout_turn, IDEATION_SCOUT_SESSION);
    let writer = history.point(
        &scope,
        "session",
        "writer-session",
        "2026-10-16T08:33:00.000Z",
    );
    history.session(writer, IDEATION_WRITER_SESSION);
    history.data(writer, serde_json::json!({ "title": IDEATION_WRITER_LANE }));
    let writer_tool = history.span(&scope, "tool", "Write", "2026-10-16T08:34:00.000Z", 15_000);
    history.label(writer_tool, "personas/developer.md");
    history.session(writer_tool, IDEATION_WRITER_SESSION);
}

pub fn seed_change_record(env: &common::TestEnv, change: &str) -> Vec<HistoryEntry> {
    let mut history = SeededHistory::new();
    push_change_entries(&mut history, change);
    history.seed(env)
}

const OUT_OF_ORDER_STEPS: [(&str, &str, &str, u64); 3] = [
    (
        "verification",
        "cargo test --locked --test features_runner",
        "2026-10-16T10:00:00.000Z",
        60_000,
    ),
    (
        "lint",
        "cargo clippy --locked --all-targets -- -D warnings",
        "2026-10-16T09:00:00.000Z",
        120_000,
    ),
    (
        "coverage",
        "cargo llvm-cov nextest --locked --test features_runner",
        "2026-10-16T11:00:00.000Z",
        300_000,
    ),
];

pub fn seed_out_of_order_steps(env: &common::TestEnv, scope: &Scope) -> Vec<HistoryEntry> {
    let mut history = SeededHistory::new();
    for (name, label, at, duration_ms) in OUT_OF_ORDER_STEPS {
        let entry = history.span(scope, "step", name, at, duration_ms);
        history.label(entry, label);
    }
    history.seed(env)
}

pub const WORKFLOW_ARTIFACTS: [&str; 7] = [
    "proposal.md",
    "specs",
    "design.md",
    "arc42.md",
    "design-review.md",
    "tasks.md",
    "review.md",
];

const STAGE_TRANSITIONS: [(&str, &str); 7] = [
    ("propose", "proposal.md"),
    ("specs", "specs"),
    ("design", "design.md"),
    ("arc42", "arc42.md"),
    ("design-review", "design-review.md"),
    ("tasks", "tasks.md"),
    ("review", "review.md"),
];

pub fn stage_transition(stage: &str) -> &'static str {
    STAGE_TRANSITIONS
        .iter()
        .find(|(known, _)| *known == stage)
        .map(|(_, artifact)| *artifact)
        .unwrap_or_else(|| panic!("the workflow has no stage `{stage}`"))
}

pub fn seed_archived_change(
    env: &common::TestEnv,
    change: &str,
    last_artifact: &str,
) -> Vec<HistoryEntry> {
    let archived = env.root().join("givn").join("archive").join(change);
    std::fs::create_dir_all(&archived)
        .unwrap_or_else(|error| panic!("create the archived change directory: {error}"));
    std::fs::write(archived.join("proposal.md"), "# Proposal\n")
        .unwrap_or_else(|error| panic!("write the archived proposal: {error}"));
    std::fs::write(archived.join("review.md"), "REVIEW: PASS\n")
        .unwrap_or_else(|error| panic!("write the archived review: {error}"));
    let last = WORKFLOW_ARTIFACTS
        .iter()
        .position(|artifact| *artifact == last_artifact)
        .unwrap_or_else(|| {
            panic!("the workflow records no artifact `{last_artifact}`: {WORKFLOW_ARTIFACTS:?}")
        });
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    for (index, artifact) in WORKFLOW_ARTIFACTS[..=last].iter().enumerate() {
        let transition = history.point(&scope, "phase", artifact, TRANSITION_AT[index]);
        history.label(transition, artifact);
    }
    history.seed(env)
}

pub fn write_change_artifacts_through_tasks(env: &common::TestEnv, change: &str) {
    let change_dir = env.root().join("givn").join("changes").join(change);
    let specs = change_dir.join("specs");
    std::fs::create_dir_all(&specs)
        .unwrap_or_else(|error| panic!("create the change spec directory: {error}"));
    std::fs::write(
        specs.join(format!("{change}.feature")),
        format!("Feature: {change}\n"),
    )
    .unwrap_or_else(|error| panic!("write the change spec: {error}"));
    std::fs::write(change_dir.join("design.md"), "# Design\n")
        .unwrap_or_else(|error| panic!("write the change design: {error}"));
    std::fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .unwrap_or_else(|error| panic!("write the change design review: {error}"));
    std::fs::write(
        change_dir.join("tasks.md"),
        "- [x] write the specification\n- [x] verify the change\n",
    )
    .unwrap_or_else(|error| panic!("write the change tasks: {error}"));
}

const TRANSITION_AT: [&str; 7] = [
    "2026-10-16T08:00:00.000Z",
    "2026-10-16T09:00:00.000Z",
    "2026-10-16T10:00:00.000Z",
    "2026-10-16T11:00:00.000Z",
    "2026-10-16T12:00:00.000Z",
    "2026-10-16T13:00:00.000Z",
    "2026-10-16T14:00:00.000Z",
];

const STAGE_WORK: [(&str, &str, u64); 7] = [
    (
        "givn instructions propose",
        "2026-10-16T07:50:00.000Z",
        300_000,
    ),
    (
        "givn instructions specs",
        "2026-10-16T08:30:00.000Z",
        600_000,
    ),
    (
        "givn instructions design",
        "2026-10-16T09:30:00.000Z",
        900_000,
    ),
    (
        "givn instructions arc42",
        "2026-10-16T10:30:00.000Z",
        120_000,
    ),
    (
        "givn skills design-review",
        "2026-10-16T11:30:00.000Z",
        600_000,
    ),
    (
        "givn instructions tasks",
        "2026-10-16T12:30:00.000Z",
        300_000,
    ),
    (
        "cargo test --locked --test features_runner",
        "2026-10-16T13:30:00.000Z",
        1_800_000,
    ),
];

#[derive(Debug, Clone)]
pub struct SeededReportHistory {
    /// The step recorded after the record's last transition: the one the
    /// `implement` phase's assertion names.
    pub post_transition_step: HistoryEntry,
}

pub fn seed_report_history(
    env: &common::TestEnv,
    change: &str,
    last_artifact: &str,
) -> SeededReportHistory {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let last = WORKFLOW_ARTIFACTS
        .iter()
        .position(|artifact| *artifact == last_artifact)
        .unwrap_or_else(|| {
            panic!("the workflow records no artifact `{last_artifact}`: {WORKFLOW_ARTIFACTS:?}")
        });
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    for (index, artifact) in WORKFLOW_ARTIFACTS[..=last].iter().enumerate() {
        let transition = history.point(&scope, "phase", artifact, TRANSITION_AT[index]);
        history.label(transition, artifact);
    }
    let work = last + 1;
    assert!(
        work < STAGE_WORK.len(),
        "the fixture records no step for the stage `{last_artifact}` opens"
    );
    let mut steps = Vec::new();
    for (label, at, duration_ms) in STAGE_WORK[..=work].iter() {
        let step = history.span(&scope, "step", label, at, *duration_ms);
        steps.push(history.label(step, label));
    }
    let entries = history.seed(env);
    SeededReportHistory {
        post_transition_step: entries[steps[work]].clone(),
    }
}

pub struct SeededDetailsHistory {
    /// The tool entry carrying every captured metadata fact the panel names,
    /// plus a small content bundle.
    pub metadata_entry: HistoryEntry,
    /// The entries whose content bundles fill the 8 MiB report budget.
    pub content_entry_ids: Vec<u64>,
    /// The store's bytes as seeded, so the report run is shown not to change a
    /// single content byte.
    pub store_bytes: Vec<u8>,
}

fn captured_metadata() -> Value {
    serde_json::json!({
        "title": "Development report details",
        "status": "needs_input",
        "stop_reason": "end_turn",
        "model": "deepseek/deepseek-flash",
        "cost": 0.42,
        "list_cost": 0.55,
        "context_size": 48_000,
        "context_window": 128_000,
        "tool_call_id": "call-25b",
        "output_bytes": 4_096,
        "error": "the command failed: exit status 101",
        "agent": "omp",
        "attempt": 2,
        "max_attempts": 3,
        "delay_ms": 1_500,
        "fallback_from": "model-a",
        "fallback_to": "model-b",
        "permission_mode": "acceptEdits",
        "content": {
            "tool_input": {
                "text": "cargo test --locked",
                "truncated": false,
                "bytes": 19
            },
            "tool_output": {
                "text": "test result: ok.",
                "truncated": false,
                "bytes": 16
            }
        }
    })
}

fn full_content_bundle(marker: char) -> Value {
    let text: String = std::iter::repeat_n(marker, CONTENT_FIELD_LIMIT).collect();
    let field = serde_json::json!({
        "text": text,
        "truncated": false,
        "bytes": text.len()
    });
    serde_json::json!({
        "tool_input": field.clone(),
        "tool_output": field.clone(),
        "assistant_text": field.clone(),
        "thinking_text": field
    })
}

pub fn seed_captured_metadata_and_content(
    env: &common::TestEnv,
    change: &str,
) -> SeededDetailsHistory {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    let turn = history.span(
        &scope,
        "turn",
        "turn 3",
        "2026-10-16T09:00:00.000Z",
        600_000,
    );
    let metadata = history.span(
        &scope,
        "tool",
        "Bash",
        "2026-10-16T09:10:00.000Z",
        2_400_000,
    );
    history.label(metadata, "cargo test --locked");
    history.session(metadata, "session-25b");
    history.data(metadata, captured_metadata());
    history.nested_in(metadata, turn);
    let mut content_indices = Vec::new();
    for index in 0..34usize {
        let at = format!("2026-10-16T10:{index:02}:00.000Z");
        let entry = history.span(&scope, "tool", "Bash", &at, 5_000);
        history.label(entry, &format!("content call {index}"));
        let marker = (b'a' + (index % 26) as u8) as char;
        history.data(
            entry,
            serde_json::json!({ "content": full_content_bundle(marker) }),
        );
        content_indices.push(entry);
    }
    let entries = history.seed(env);
    let content_entry_ids = content_indices
        .iter()
        .map(|index| entries[*index].id)
        .collect();
    SeededDetailsHistory {
        metadata_entry: entries[metadata].clone(),
        content_entry_ids,
        store_bytes: std::fs::read(store_file(&env.root()))
            .unwrap_or_else(|error| panic!("read the seeded store: {error}")),
    }
}

pub fn seed_report_detail_tool(env: &common::TestEnv, change: &str) -> HistoryEntry {
    let mut entry = HistoryEntry::reported(
        Scope::Change {
            id: change.to_string(),
        },
        "tool",
        "Bash",
        "2026-10-16T14:30:00.000Z",
    );
    entry.label = Some("cargo test --locked".to_string());
    entry.duration_ms = Some(2_400_000);
    entry.outcome = "ok".to_string();
    entry.session = Some("e7-session".to_string());
    entry.data = Some(serde_json::json!({
        "tool_call_id": "call-e7",
        "output_bytes": 16,
        "content": {
            "tool_input": {
                "text": "<b>cargo test --locked</b>",
                "truncated": false,
                "bytes": 25
            }
        }
    }));
    let store = HistoryStore::at(store_file(&env.root()));
    store
        .append(std::slice::from_mut(&mut entry))
        .unwrap_or_else(|error| panic!("append the E7 detail tool entry: {error}"));
    entry
}

pub const RECORDER_LABELS: [&str; 3] = [
    "history ingest",
    "history ingest --with-full-content",
    "history harness-hook --harness claude",
];

pub struct SeededRecorderHistory {
    /// The one reported tool call: measured, so the ranking and the tool count
    /// discriminate.
    pub tool: HistoryEntry,
    /// The one other recorded command the report keeps.
    pub other_command: HistoryEntry,
    /// The recorder's own entries — the unflagged intake, the flagged intake,
    /// and the Claude-style hook mapper — in store order.
    pub recorder: Vec<HistoryEntry>,
    /// The record's entry count as seeded.
    pub entry_count: usize,
    /// The store's bytes as seeded.
    pub store_bytes: Vec<u8>,
}

pub fn seed_recorder_omission_history(
    env: &common::TestEnv,
    change: &str,
) -> SeededRecorderHistory {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    let tool = history.span(
        &scope,
        "tool",
        "Bash",
        "2026-10-16T09:00:00.000Z",
        2_400_000,
    );
    history.label(tool, "cargo test --locked");
    history.session(tool, "session-h12");
    let other = history.span(
        &scope,
        "command",
        "status",
        "2026-10-16T09:10:00.000Z",
        90_000,
    );
    let other_label = format!("status --change {change}");
    history.label(other, &other_label);
    let times = [
        "2026-10-16T09:20:00.000Z",
        "2026-10-16T09:21:00.000Z",
        "2026-10-16T09:22:00.000Z",
    ];
    let durations = [40u64, 40, 60];
    let mut recorder_indices = Vec::new();
    for ((label, at), duration_ms) in RECORDER_LABELS.into_iter().zip(times).zip(durations) {
        let entry = history.span(&scope, "command", "history", at, duration_ms);
        history.label(entry, label);
        recorder_indices.push(entry);
    }
    let entries = history.seed(env);
    SeededRecorderHistory {
        tool: entries[tool].clone(),
        other_command: entries[other].clone(),
        recorder: recorder_indices
            .iter()
            .map(|index| entries[*index].clone())
            .collect(),
        entry_count: record_entries(&env.root()).len(),
        store_bytes: store_bytes(&env.root()),
    }
}

pub const PARALLEL_SPAWN_LANE: &str = "Opencode main";

pub const PARALLEL_SCOUT_LANE: &str = "Architect.Scout";

pub const PARALLEL_UNATTRIBUTED_LANE: &str = "unattributed";

pub const PARALLEL_TOOL_CALL_ID: &str = "call-hybrid-scout";

pub const PARALLEL_CONTENT_TEXT: &str = "<b>cargo test --locked</b>";

pub fn seed_parallel_sessions(env: &common::TestEnv, change: &str) -> Vec<HistoryEntry> {
    let out = common::run_givn(env, &["new", change]);
    assert!(
        out.status.success(),
        "givn new {change} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let scope = Scope::Change {
        id: change.to_string(),
    };
    let mut history = SeededHistory::new();
    let transition = history.point(&scope, "phase", "tasks.md", "2026-10-16T13:00:00.000Z");
    history.label(transition, "tasks.md");
    let spawn_session = history.point(
        &scope,
        "session",
        "opencode-session",
        "2026-10-16T13:05:00.000Z",
    );
    history.session(spawn_session, SPAWN_SESSION_ID);
    history.data(
        spawn_session,
        serde_json::json!({ "title": PARALLEL_SPAWN_LANE }),
    );
    let child_session = history.point(
        &scope,
        "session",
        "subagent-session",
        "2026-10-16T13:06:00.000Z",
    );
    history.session(child_session, CHILD_SESSION_ID);
    history.data(
        child_session,
        serde_json::json!({ "title": "Subagent transcript" }),
    );
    let child_turn = history.span(&scope, "turn", "turn", "2026-10-16T13:07:00.000Z", 30_000);
    history.session(child_turn, CHILD_SESSION_ID);
    let child_span = history.point(
        &scope,
        "subagent",
        "Architect.Scout",
        "2026-10-16T13:08:00.000Z",
    );
    history.session(child_span, CHILD_SESSION_ID);
    history.data(
        child_span,
        serde_json::json!({ "subagent": CHILD_SESSION_ID, "agent": PARALLEL_SCOUT_LANE }),
    );
    let tool = history.span(&scope, "tool", "Bash", "2026-10-16T13:09:00.000Z", 12_000);
    history.label(tool, "cargo test --locked");
    history.session(tool, CHILD_SESSION_ID);
    history.data(
        tool,
        serde_json::json!({
            "tool_call_id": PARALLEL_TOOL_CALL_ID,
            "content": {
                "tool_input": {
                    "text": PARALLEL_CONTENT_TEXT,
                    "truncated": false,
                    "bytes": PARALLEL_CONTENT_TEXT.len()
                }
            }
        }),
    );
    history.nested_in(tool, child_span);
    let spawn_turn = history.span(&scope, "turn", "turn", "2026-10-16T13:10:00.000Z", 45_000);
    history.session(spawn_turn, SPAWN_SESSION_ID);
    let spawn_span = history.point(&scope, "subagent", "Reviewer", "2026-10-16T13:11:00.000Z");
    history.session(spawn_span, SPAWN_SESSION_ID);
    history.data(
        spawn_span,
        serde_json::json!({ "subagent": "ghost-111213141516", "agent": "Reviewer" }),
    );
    history.span(&scope, "step", "verify", "2026-10-16T13:12:00.000Z", 20_000);
    history.seed(env)
}

const SPAWN_SESSION_ID: &str = "spawn-0a1b2c3d4e5f";

const CHILD_SESSION_ID: &str = "child-9f8e7d6c5b4a";

fn read_dir_names(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

const SHIM_BIN_DIR: &str = ".givn-test-bin";

pub const HARNESS_SESSION: &str = "claude-timeline-fixture";

pub fn initialize_with_omp_reporter(env: &common::TestEnv) {
    let out = common::run_givn(env, &["skills", "targets", "add", "omp"]);
    assert!(
        out.status.success(),
        "givn skills targets add omp failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        env.path(".omp/hooks/pre/givn-timeline.js").is_file(),
        "the managed sync did not install the omp development reporter"
    );
}

pub fn shim_givn_on_path(env: &common::TestEnv) -> PathBuf {
    let bin = env.path(SHIM_BIN_DIR);
    std::fs::create_dir_all(&bin).unwrap_or_else(|error| {
        panic!("create the PATH shim directory {}: {error}", bin.display())
    });
    let givn = bin.join("givn");
    std::fs::write(
        &givn,
        format!(
            "#!/bin/sh\nexec '{}' \"$@\"\n",
            crate::harness::binary::givn().display()
        ),
    )
    .unwrap_or_else(|error| panic!("write the PATH shim {}: {error}", givn.display()));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&givn)
            .expect("the PATH shim's metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&givn, permissions).expect("make the PATH shim executable");
    }
    bin
}

fn driver_path(dirs: &[&Path]) -> String {
    let mut parts: Vec<String> = dirs.iter().map(|dir| dir.display().to_string()).collect();
    if let Some(existing) = std::env::var_os("PATH") {
        parts.push(existing.to_string_lossy().into_owned());
    }
    parts.join(":")
}

pub fn report_harness_hook(env: &common::TestEnv, payload: &Value) -> std::process::Output {
    let shim = env.path(".claude/hooks/givn-timeline.sh");
    assert!(
        shim.is_file(),
        "the generated Claude shim is missing: {}",
        shim.display()
    );
    let bin = shim_givn_on_path(env);
    let mut child = std::process::Command::new(&shim)
        .current_dir(env.root())
        .env("PATH", driver_path(&[&bin]))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("run the generated shim {}: {error}", shim.display()));
    use std::io::Write as _;
    child
        .stdin
        .as_mut()
        .expect("the generated shim's stdin")
        .write_all(payload.to_string().as_bytes())
        .expect("write the hook payload");
    child
        .wait_with_output()
        .unwrap_or_else(|error| panic!("wait for the generated shim: {error}"))
}

#[derive(Debug)]
pub struct TranscriptFixture {
    /// The transcript file the `Stop` payload names.
    pub path: PathBuf,
    /// The usage the record must carry, in the record's own `data` keys.
    pub counts: Value,
    /// The model that produced the usage.
    pub model: String,
}

pub fn write_transcript_with_usage(env: &common::TestEnv, model: &str) -> TranscriptFixture {
    let path = env.path("transcript.jsonl");
    let input = 12_043u64;
    let output = 812u64;
    let cache_read = 9_000u64;
    let cache_write = 240u64;
    let text = format!(
        "{}\n{}\n",
        serde_json::json!({
            "type": "user",
            "sessionId": HARNESS_SESSION,
            "message": {
                "role": "user",
                "content": [{"type": "text", "text": "Implement the change"}],
            },
        }),
        serde_json::json!({
            "type": "assistant",
            "sessionId": HARNESS_SESSION,
            "message": {
                "role": "assistant",
                "model": model,
                "usage": {
                    "input_tokens": input,
                    "output_tokens": output,
                    "cache_read_input_tokens": cache_read,
                    "cache_creation_input_tokens": cache_write,
                    "output_tokens_details": {"thinking_tokens": 160},
                },
            },
        }),
    );
    std::fs::write(&path, text)
        .unwrap_or_else(|error| panic!("write the fixture transcript {}: {error}", path.display()));
    TranscriptFixture {
        path,
        counts: serde_json::json!({
            "input": input,
            "output": output,
            "cache_read": cache_read,
            "cache_write": cache_write,
            "total_tokens": input + output + cache_read + cache_write,
        }),
        model: model.to_string(),
    }
}

pub const RACING_ROUNDS: usize = 6;

pub const RACE_BARRIER_COMMAND: &str = "until [ -e givn/history/givn-race-go ]; do :; done; printf '{\"total\":1,\"passed\":1,\"failed\":0,\"skipped\":0}' > $GIVN_RESULT_FILE.$$; mv $GIVN_RESULT_FILE.$$ $GIVN_RESULT_FILE";

const RACE_MARKER: &str = "givn/history/givn-race-go";

const RACE_ARMED: std::time::Duration = std::time::Duration::from_millis(300);

pub fn start_racing_commands(env: &common::TestEnv) -> Vec<std::process::Child> {
    declare_verify_command(env, RACE_BARRIER_COMMAND);
    common::commit_fixture(env);
    let marker = env.path(RACE_MARKER);
    std::fs::create_dir_all(marker.parent().expect("race marker parent"))
        .expect("create race marker directory");
    let _ = std::fs::remove_file(&marker);
    let children: Vec<std::process::Child> = (0..RACING_ROUNDS * 2)
        .map(|_| spawn_recording_command(env))
        .collect();
    std::thread::sleep(RACE_ARMED);
    std::fs::write(&marker, b"").expect("release the racing commands");
    children
}

pub fn finish_racing_commands(env: &common::TestEnv, children: Vec<std::process::Child>) {
    for (index, child) in children.into_iter().enumerate() {
        let out = child
            .wait_with_output()
            .expect("wait for a racing givn process");
        assert!(
            out.status.success(),
            "racing command {index} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let _ = std::fs::remove_file(env.path(RACE_MARKER));
}

fn spawn_recording_command(env: &common::TestEnv) -> std::process::Child {
    std::process::Command::new(crate::harness::binary::givn())
        .args(["check", "tasks", "--change", "demo"])
        .current_dir(env.root())
        .env_remove("GIVN_FEATURES")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn a racing givn process")
}

pub fn clear_record(env: &common::TestEnv) {
    let path = store_file(&env.root());
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("clear the store {}: {error}", path.display()),
    }
}

pub fn store_file(root: &Path) -> PathBuf {
    root.join("givn/history/events.jsonl")
}

pub fn make_record_unwritable(env: &common::TestEnv) {
    let path = store_file(&env.root());
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("clear the store {}: {error}", path.display()),
    }
    std::fs::create_dir_all(&path)
        .unwrap_or_else(|error| panic!("create the unwritable store {}: {error}", path.display()));
}

pub fn intake_payload(change: Option<&str>, topic: Option<&str>, events: Vec<Value>) -> Value {
    serde_json::json!({
        "schema": 1,
        "reporter": "fixture",
        "session": "fixture-session",
        "change": change,
        "topic": topic,
        "events": events,
    })
}

pub fn report_intake(env: &common::TestEnv, payload: &Value) -> std::process::Output {
    report_intake_with_args(env, payload, &[])
}

pub fn report_intake_with_args(
    env: &common::TestEnv,
    payload: &Value,
    args: &[&str],
) -> std::process::Output {
    let mut argv = vec!["history", "ingest"];
    argv.extend_from_slice(args);
    common::run_givn_with_stdin_and_envs(env, &argv, &payload.to_string(), &[])
}

pub fn report_intake_raw(env: &common::TestEnv, payload: &str) -> std::process::Output {
    common::run_givn_with_stdin_and_envs(env, &["history", "ingest"], payload, &[])
}

pub fn record_entries(root: &Path) -> Vec<Value> {
    entries_of(&store_bytes(root))
}

pub fn store_bytes(root: &Path) -> Vec<u8> {
    let path = store_file(root);
    std::fs::read(&path)
        .unwrap_or_else(|error| panic!("read the development record {}: {error}", path.display()))
}

pub fn entries_of(bytes: &[u8]) -> Vec<Value> {
    let text = std::str::from_utf8(bytes)
        .unwrap_or_else(|error| panic!("the development record is not UTF-8: {error}"));
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .unwrap_or_else(|error| panic!("store line is not a JSON object: {error}\n{line}"))
        })
        .collect()
}

#[derive(Debug, Default)]
pub struct SeededHistory {
    entries: Vec<HistoryEntry>,
    parents: Vec<Option<usize>>,
}

impl SeededHistory {
    /// A builder for a record with no entries yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// One measured span: the entry kind, its name, its recorded end
    /// timestamp, and the duration a fixture measured. Returns the entry's
    /// index in the seeded batch, which the setters and [`Self::nested_in`]
    /// address.
    pub fn span(
        &mut self,
        scope: &Scope,
        kind: &str,
        name: &str,
        at: &str,
        duration_ms: u64,
    ) -> usize {
        let mut entry = seeded_entry(scope, kind, name, at);
        entry.duration_ms = Some(duration_ms);
        self.push(entry)
    }

    /// One point entry: the kind records an instant (a reported usage, a
    /// minted revision, an activity the record does not time), so it carries
    /// no duration.
    pub fn point(&mut self, scope: &Scope, kind: &str, name: &str, at: &str) -> usize {
        self.push(seeded_entry(scope, kind, name, at))
    }

    /// The bounded subject label of the entry at `index`.
    pub fn label(&mut self, index: usize, label: &str) -> usize {
        self.entry(index).label = Some(label.to_string());
        index
    }

    /// The entry's `data`, where the record carries a non-generic value.
    pub fn data(&mut self, index: usize, data: Value) -> usize {
        self.entry(index).data = Some(data);
        index
    }

    /// The harness session the entry belongs to (the envelope's `session`).
    pub fn session(&mut self, index: usize, session: &str) -> usize {
        self.entry(index).session = Some(session.to_string());
        index
    }

    /// The entry's outcome (`ok`, `failed`, `skipped`, or `unknown`).
    pub fn outcome(&mut self, index: usize, outcome: &str) -> usize {
        self.entry(index).outcome = outcome.to_string();
        index
    }

    /// Nest the entry at `index` inside the entry at `parent` (an earlier
    /// index of this same batch), the link the report nests spans by.
    pub fn nested_in(&mut self, index: usize, parent: usize) -> usize {
        assert!(
            parent < index,
            "a parent is only ever an entry an earlier append minted: {parent} >= {index}"
        );
        self.parents[index] = Some(parent);
        index
    }

    /// Write the record: clear the fixture's store and append these entries
    /// through the store's own append path. Returns the written entries — ids
    /// minted, parent links resolved — so the scenario asserts on the record it
    /// seeded.
    pub fn seed(mut self, env: &common::TestEnv) -> Vec<HistoryEntry> {
        let path = store_file(&env.root());
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("clear the record {}: {error}", path.display()),
        }
        let store = HistoryStore::at(path);
        store
            .append_linked(&mut self.entries, &self.parents)
            .unwrap_or_else(|error| panic!("seed the development record: {error}"));
        self.entries
    }

    /// The entry at `index`, for the setters.
    fn entry(&mut self, index: usize) -> &mut HistoryEntry {
        self.entries
            .get_mut(index)
            .unwrap_or_else(|| panic!("the seeded record has no entry {index}"))
    }

    /// Append one entry with no parent link yet.
    fn push(&mut self, entry: HistoryEntry) -> usize {
        self.entries.push(entry);
        self.parents.push(None);
        self.entries.len() - 1
    }
}

fn seeded_entry(scope: &Scope, kind: &str, name: &str, at: &str) -> HistoryEntry {
    HistoryEntry {
        schema: SCHEMA,
        id: 0,
        at: at.to_string(),
        scope: scope.clone(),
        source: "givn".to_string(),
        kind: kind.to_string(),
        name: name.to_string(),
        label: None,
        duration_ms: None,
        outcome: "ok".to_string(),
        parent: None,
        session: None,
        data: None,
    }
}

#[derive(Debug, Clone)]
pub enum HarnessActivity {
    /// One completed tool call.
    Tool {
        /// The harness's own call id (the key both reporters pair a call by).
        id: String,
        /// The tool the harness names.
        tool: String,
        /// The call's input, in the harness's own shape.
        args: Value,
    },
    /// One question wait: the question tool's call, from its start to its
    /// completion.
    Question {
        /// The harness's own call id.
        id: String,
        /// The question text the wait's label is bounded from.
        question: String,
    },
    /// One subagent, from its start to its completion.
    Subagent {
        /// The subagent's own identity.
        id: String,
        /// The agent definition that ran it.
        agent: String,
    },
    /// One completed turn, with the harness's own usage where it reports one.
    Turn {
        /// The turn's own index.
        index: u64,
        /// The model that produced the turn.
        model: String,
        /// The usage the harness reports for the turn, in its own shape.
        usage: Option<Value>,
    },
    /// One completed turn with the default-capture metadata a driven reporter
    /// reports: the harness's stop reason, its cost, and its context reading.
    TurnMetadata {
        /// The turn's own index.
        index: u64,
        /// The model that produced the turn.
        model: String,
        /// The harness's own stop reason (a raw word).
        stop_reason: String,
        /// The cost the harness reports (real and list price where present).
        cost: Value,
        /// The context size the harness measured.
        context_size: u64,
        /// The model's context window.
        context_window: u64,
        /// The usage the harness reports for the turn.
        usage: Option<Value>,
    },
    /// One failed tool call with the harness's own error text and output size.
    FailedTool {
        /// The harness's own call id.
        id: String,
        /// The tool the harness names.
        tool: String,
        /// The call's input, in the harness's own shape.
        args: Value,
        /// The harness's own error text (a stale reporter may send it unbounded).
        error: String,
        /// The output's size in bytes.
        output_bytes: u64,
    },
    /// The session's end: the reporter materializes whatever never closed and
    /// flushes.
    Shutdown,
}

impl HarnessActivity {
    /// The activity as the driver scripts consume it.
    fn as_json(&self) -> Value {
        match self {
            HarnessActivity::Tool { id, tool, args } => serde_json::json!({
                "kind": "tool",
                "id": id,
                "tool": tool,
                "args": args,
            }),
            HarnessActivity::Question { id, question } => serde_json::json!({
                "kind": "question",
                "id": id,
                "question": question,
            }),
            HarnessActivity::Subagent { id, agent } => serde_json::json!({
                "kind": "subagent",
                "id": id,
                "agent": agent,
            }),
            HarnessActivity::Turn {
                index,
                model,
                usage,
            } => serde_json::json!({
                "kind": "turn",
                "index": index,
                "model": model,
                "usage": usage,
            }),
            HarnessActivity::TurnMetadata {
                index,
                model,
                stop_reason,
                cost,
                context_size,
                context_window,
                usage,
            } => serde_json::json!({
                "kind": "turn_metadata",
                "index": index,
                "model": model,
                "stop_reason": stop_reason,
                "cost": cost,
                "context_size": context_size,
                "context_window": context_window,
                "usage": usage,
            }),
            HarnessActivity::FailedTool {
                id,
                tool,
                args,
                error,
                output_bytes,
            } => serde_json::json!({
                "kind": "failed_tool",
                "id": id,
                "tool": tool,
                "args": args,
                "error": error,
                "output_bytes": output_bytes,
            }),
            HarnessActivity::Shutdown => serde_json::json!({ "kind": "shutdown" }),
        }
    }
}

pub const DRIVER_SESSION: &str = "givn-timeline-driver";

pub fn drive_omp_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let reporter = env.path(".omp/hooks/pre/givn-timeline.js");
    assert!(
        reporter.is_file(),
        "the generated omp reporter is missing: {}",
        reporter.display()
    );
    run_node_driver(
        env,
        "invoke-omp-reporter.mjs",
        OMP_DRIVER,
        &reporter,
        activities,
    )
}

pub fn drive_opencode_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let reporter = env.path(".opencode/plugins/givn-timeline.js");
    assert!(
        reporter.is_file(),
        "the generated opencode reporter is missing: {}",
        reporter.display()
    );
    run_node_driver(
        env,
        "invoke-opencode-reporter.mjs",
        OPENCODE_DRIVER,
        &reporter,
        activities,
    )
}

pub fn drive_claude_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let shim = env.path(".claude/hooks/givn-timeline.sh");
    assert!(
        shim.is_file(),
        "the generated Claude reporter is missing: {}",
        shim.display()
    );
    let mut last = None;
    for activity in activities {
        for payload in claude_payloads(env, activity) {
            let event = payload["hook_event_name"]
                .as_str()
                .unwrap_or("unknown event")
                .to_string();
            let out = report_harness_hook(env, &payload);
            assert!(
                out.status.success(),
                "the generated Claude reporter failed on `{event}` (exit {:?}): {}{}",
                out.status.code(),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );
            last = Some(out);
        }
    }
    last.expect("the driver reported no harness event")
}

fn claude_payloads(env: &common::TestEnv, activity: &HarnessActivity) -> Vec<Value> {
    let tool_call = |event: &str, tool: &str, id: &str, input: Value| {
        serde_json::json!({
            "hook_event_name": event,
            "session_id": DRIVER_SESSION,
            "tool_name": tool,
            "tool_use_id": id,
            "tool_input": input,
        })
    };
    match activity {
        HarnessActivity::Tool { id, tool, args } => {
            let mut completion = tool_call("PostToolUse", tool, id, args.clone());
            completion["duration_ms"] = Value::from(CLAUDE_TOOL_MS);
            completion["tool_response"] = serde_json::json!({});
            vec![tool_call("PreToolUse", tool, id, args.clone()), completion]
        }
        HarnessActivity::Question { id, question } => {
            let input = serde_json::json!({ "questions": [{ "question": question }] });
            let mut completion = tool_call("PostToolUse", CLAUDE_QUESTION_TOOL, id, input.clone());
            completion["duration_ms"] = Value::from(CLAUDE_TOOL_MS);
            completion["tool_response"] = serde_json::json!({});
            vec![
                tool_call("PreToolUse", CLAUDE_QUESTION_TOOL, id, input),
                completion,
            ]
        }
        HarnessActivity::Subagent { id, agent } => vec![
            serde_json::json!({
                "hook_event_name": "SubagentStart",
                "session_id": DRIVER_SESSION,
                "agent_id": id,
                "agent_type": agent,
            }),
            serde_json::json!({
                "hook_event_name": "SubagentStop",
                "session_id": DRIVER_SESSION,
                "agent_id": id,
                "agent_type": agent,
            }),
        ],
        HarnessActivity::Turn { model, .. } => {
            let transcript = write_transcript_with_usage(env, model);
            vec![serde_json::json!({
                "hook_event_name": "Stop",
                "session_id": DRIVER_SESSION,
                "transcript_path": transcript.path.display().to_string(),
            })]
        }
        HarnessActivity::TurnMetadata { model, cost, .. } => {
            let transcript = write_transcript_with_usage(env, model);
            if let Some(total) = cost.as_f64() {
                let mut text = std::fs::read_to_string(&transcript.path)
                    .unwrap_or_else(|error| panic!("read the transcript fixture: {error}"));
                text.push_str(
                    &serde_json::json!({
                        "type": "cost-state",
                        "totalCostUSD": total,
                    })
                    .to_string(),
                );
                text.push('\n');
                std::fs::write(&transcript.path, text)
                    .expect("append the cost-state rollup to the transcript fixture");
            }
            vec![serde_json::json!({
                "hook_event_name": "Stop",
                "session_id": DRIVER_SESSION,
                "transcript_path": transcript.path.display().to_string(),
            })]
        }
        HarnessActivity::FailedTool {
            id,
            tool,
            args,
            error,
            output_bytes,
        } => {
            let mut completion = tool_call("PostToolUseFailure", tool, id, args.clone());
            completion["duration_ms"] = Value::from(CLAUDE_TOOL_MS);
            completion["error"] = Value::String(error.clone());
            completion["tool_response"] = Value::String("x".repeat(*output_bytes as usize));
            vec![tool_call("PreToolUse", tool, id, args.clone()), completion]
        }
        HarnessActivity::Shutdown => vec![serde_json::json!({
            "hook_event_name": "SessionEnd",
            "session_id": DRIVER_SESSION,
        })],
    }
}

const CLAUDE_QUESTION_TOOL: &str = "AskUserQuestion";

const CLAUDE_TOOL_MS: u64 = 40;

pub(crate) fn maki_binary() -> PathBuf {
    if let Some(dir) = std::env::var_os("GIVN_E2E_BIN") {
        let candidate = PathBuf::from(dir).join("maki");
        if candidate.is_file() {
            return candidate;
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        let candidate = PathBuf::from(home).join(".cache/givn-e2e/bin/maki");
        if candidate.is_file() {
            return candidate;
        }
    }
    PathBuf::from("maki")
}

pub fn initialize_with_maki_reporter(env: &common::TestEnv) {
    let out = common::run_givn(env, &["init", "--agents", "maki"]);
    assert!(
        out.status.success(),
        "givn init --agents maki failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        env.path(".maki/lua/givn-timeline.lua").is_file(),
        "givn init did not install the Maki development reporter"
    );
}

pub type MakiEvent = (&'static str, Value);

fn lua_literal(value: &Value) -> String {
    match value {
        Value::Null => "nil".to_string(),
        Value::Bool(boolean) => boolean.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(text) => format!("{text:?}"),
        Value::Array(items) => format!(
            "{{ {} }}",
            items.iter().map(lua_literal).collect::<Vec<_>>().join(", ")
        ),
        Value::Object(map) => format!(
            "{{ {} }}",
            map.iter()
                .map(|(key, value)| format!(
                    "[{}] = {}",
                    lua_literal(&Value::String(key.clone())),
                    lua_literal(value)
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn maki_driver_script(events: &[MakiEvent]) -> String {
    maki_driver_script_with("", events)
}

fn maki_driver_script_with(prelude: &str, events: &[MakiEvent]) -> String {
    let mut scripts = String::new();
    for (event, data) in events {
        scripts.push_str(&format!("  {{ {event:?}, {} }},\n", lua_literal(data)));
    }
    format!(
        r#"-- Givn timeline test driver: the shipped fence above ran verbatim, then the
-- scripted events are fired through the harness's own dispatch.
local marker = maki.fs.write(".maki-driver-loaded", "loaded\n")
if not marker then
  error("the driver's load marker could not be written")
end
{prelude}
local events = {{
{scripts}}}
for _, event in ipairs(events) do
  local fired, fire_err = pcall(function()
    maki.api.exec_autocmds(event[1], {{ data = event[2] }})
  end)
  if not fired then
    maki.log.error("givn-timeline driver: " .. event[1] .. " failed: " .. tostring(fire_err))
  end
  -- No host event carries a timestamp: give the paired stamps a real gap.
  local until_t = os.clock() + 0.02
  while os.clock() < until_t do end
end
local reporter = require("givn-timeline")
local flushed, flush_err = pcall(reporter.run)
if not flushed then
  maki.log.error("givn-timeline driver: the flush failed: " .. tostring(flush_err))
end
"#
    )
}

pub fn run_maki_driver(env: &common::TestEnv, events: &[MakiEvent]) -> std::process::Output {
    assert_maki_reporter(env);
    run_maki_init(env, &maki_driver_script(events))
}

fn lua_model_return(model: &MakiModel<'_>) -> String {
    match (model.spec, model.reasoning) {
        (Some(spec), Some(level)) => format!(
            "  return {{ spec = {}, thinking_shown = {} }}",
            lua_literal(&Value::String(spec.to_string())),
            lua_literal(&Value::String(level.to_string()))
        ),
        (Some(spec), None) => format!(
            "  return {{ spec = {} }}",
            lua_literal(&Value::String(spec.to_string()))
        ),
        (None, _) => "  return nil".to_string(),
    }
}

fn lua_session_return(model: MakiModel<'_>) -> String {
    match (model.spec, model.reasoning) {
        (Some(spec), Some(level)) => format!(
            "  return {{ model = {}, thinking = {} }}",
            lua_literal(&Value::String(spec.to_string())),
            lua_literal(&Value::String(level.to_string()))
        ),
        (Some(spec), None) => format!(
            "  return {{ model = {} }}",
            lua_literal(&Value::String(spec.to_string()))
        ),
        (None, _) => "  return nil".to_string(),
    }
}

pub fn run_maki_attribution(
    env: &common::TestEnv,
    command: &str,
    focused: MakiModel<'_>,
) -> String {
    let module = env.path(".maki/lua/givn-attribution.lua");
    assert!(
        module.is_file(),
        "the generated Maki attribution module is missing: {}",
        module.display()
    );
    let focused_lua = if focused.headless {
        "  return nil, \"no interactive UI attached\"".to_string()
    } else {
        lua_model_return(&focused)
    };
    let session_lua = lua_session_return(focused);
    let command = lua_literal(&Value::String(command.to_string()));
    let driver = format!(
        r#"-- Givn Maki attribution driver: the shipped fences above ran verbatim;
-- the focused model is the scenario fixture and the module is the generated one.
local marker = maki.fs.write(".maki-driver-loaded", "loaded\n")
if not marker then
  error("the driver's load marker could not be written")
end
local attribution = require("givn-attribution")
maki.model.get = function()
{focused_lua}
end
maki.session.read = function()
{session_lua}
end
local slots = maki.api.get_slots()
local entry = slots["tool.bash.input"]
if type(entry) ~= "table" or type(entry.fillers) ~= "table" then
  error("the attribution layer did not register tool.bash.input")
end
local registered = false
for _, filler in ipairs(entry.fillers) do
  if filler == "project/init.lua" then
    registered = true
  end
end
if not registered then
  error("the attribution layer is not owned by project/init.lua")
end
maki.fs.write(".maki-attribution-prepared", attribution.prepare({command}) .. "\n")
"#
    );
    run_maki_init(env, &driver);
    let prepared = std::fs::read_to_string(env.path(".maki-attribution-prepared"))
        .expect("read the prepared Maki command");
    prepared.trim_end_matches('\n').to_string()
}

fn assert_maki_reporter(env: &common::TestEnv) {
    let reporter = env.path(".maki/lua/givn-timeline.lua");
    assert!(
        reporter.is_file(),
        "the generated Maki reporter is missing: {}",
        reporter.display()
    );
    let fence = env.path(".maki/init.lua");
    assert!(
        fence.is_file(),
        "the generated Maki fence is missing: {}",
        fence.display()
    );
}

fn run_maki_init(env: &common::TestEnv, driver: &str) -> std::process::Output {
    // The driver file starts with the shipped fence verbatim, so the fence text
    // itself is what loads the reporter in the real harness; the generated file
    // is restored by the next `givn init`/`givn skills sync`.
    let fence = env.path(".maki/init.lua");
    let generated = std::fs::read_to_string(&fence)
        .unwrap_or_else(|error| panic!("read the generated Maki fence: {error}"));
    std::fs::write(&fence, format!("{generated}\n{driver}"))
        .expect("write the Maki driver init.lua");
    let marker = env.path(".maki-driver-loaded");
    let _ = std::fs::remove_file(&marker);
    let maki = maki_binary();
    let shim = shim_givn_on_path(env);
    let mut path_parts: Vec<String> = Vec::new();
    if let Some(parent) = maki
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        path_parts.push(parent.display().to_string());
    }
    path_parts.push(shim.display().to_string());
    if let Some(existing) = std::env::var_os("PATH") {
        path_parts.push(existing.to_string_lossy().into_owned());
    }
    let output = std::process::Command::new(&maki)
        .args(["--trust", "prompt"])
        .current_dir(env.root())
        .env("PATH", path_parts.join(":"))
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "run the real maki binary {} (run ./setup-e2e-harness.sh): {error}",
                maki.display()
            )
        });
    assert!(
        output.status.success(),
        "maki --trust prompt failed (exit {:?}): {}{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(
        marker.is_file(),
        "the real maki binary did not load the project plugin (no load marker): {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

pub const MAKI_QUESTION_SESSION: &str = "maki-question-fixture";

pub fn maki_question_transcript(question: Option<&str>, options: &[&str]) -> Value {
    match question {
        Some(question) => serde_json::json!([{
            "role": "assistant",
            "content": [{
                "type": "tool_use",
                "name": "question",
                "input": {
                    "questions": [{
                        "question": question,
                        "options": options
                            .iter()
                            .map(|label| serde_json::json!({ "label": label }))
                            .collect::<Vec<_>>(),
                    }],
                },
            }],
        }]),
        None => serde_json::json!([
            {
                "role": "assistant",
                "content": [{ "type": "text", "text": "Working on the change." }],
            },
        ]),
    }
}

pub fn drive_maki_question_wait(
    env: &common::TestEnv,
    title: &str,
    transcript: &Value,
) -> std::process::Output {
    assert_maki_reporter(env);
    let title = lua_literal(&Value::String(title.to_string()));
    let transcript = lua_literal(transcript);
    let driver = format!(
        r#"-- Givn question-wait driver: the shipped fence above ran verbatim; the
-- live transcript is the fixture, and the harness's own status pair opens and
-- closes the wait.
local marker = maki.fs.write(".maki-driver-loaded", "loaded\n")
if not marker then
  error("the driver's load marker could not be written")
end
maki.session.messages = function()
  return {transcript}
end
local status = function(value)
  maki.api.exec_autocmds("SessionStatusChanged", {{
    data = {{
      session_id = {MAKI_QUESTION_SESSION:?},
      status = value,
      title = {title},
      focused = true,
    }},
  }})
  local until_t = os.clock() + 0.02
  while os.clock() < until_t do end
end
status("needs_input")
status("working")
local reporter = require("givn-timeline")
local flushed, flush_err = pcall(reporter.run)
if not flushed then
  maki.log.error("givn-timeline driver: the flush failed: " .. tostring(flush_err))
end
"#
    );
    run_maki_init(env, &driver)
}

pub fn maki_events_for(activities: &[HarnessActivity]) -> Vec<MakiEvent> {
    let mut events: Vec<MakiEvent> = Vec::new();
    for activity in activities {
        match activity {
            HarnessActivity::Tool { id, tool, args } => {
                events.push((
                    "ToolStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                    }),
                ));
                events.push((
                    "ToolDone",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                        "is_error": false,
                        "bytes": 0,
                        "duration_ms": MAKI_TOOL_MS,
                        "input": args,
                    }),
                ));
            }
            HarnessActivity::Question { id, question } => {
                let _ = id;
                events.push((
                    "SessionStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "status": "needs_input",
                        "title": question,
                        "focused": true,
                    }),
                ));
                events.push((
                    "SessionStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "status": "working",
                        "title": question,
                        "focused": true,
                    }),
                ));
            }
            HarnessActivity::Subagent { id, agent } => {
                events.push((
                    "TaskStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "id": id,
                        "name": agent,
                        "status": "working",
                    }),
                ));
                events.push((
                    "TaskStatusChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "id": id,
                        "name": agent,
                        "status": "done",
                    }),
                ));
            }
            HarnessActivity::Turn {
                index,
                model,
                usage,
            } => {
                events.push((
                    "ModelChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "model": { "id": model },
                    }),
                ));
                events.push((
                    "TurnStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "text": "Implement the change",
                    }),
                ));
                let usage = usage.as_ref().map(|usage| {
                    serde_json::json!({
                        "input_tokens": usage["input"],
                        "output_tokens": usage["output"],
                        "cache_read_tokens": usage["cache_read"],
                        "cache_write_tokens": usage["cache_write"],
                    })
                });
                events.push((
                    "TurnEnd",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "reason": "finished",
                        "num_turns": index,
                        "usage": usage,
                    }),
                ));
            }
            HarnessActivity::TurnMetadata {
                index,
                model,
                stop_reason,
                cost,
                context_size,
                context_window,
                usage,
            } => {
                events.push((
                    "ModelChanged",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "model": { "id": model },
                    }),
                ));
                events.push((
                    "TurnStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "text": "Implement the change",
                    }),
                ));
                let usage = usage.as_ref().map(|usage| {
                    serde_json::json!({
                        "input_tokens": usage["input"],
                        "output_tokens": usage["output"],
                        "cache_read_tokens": usage["cache_read"],
                        "cache_write_tokens": usage["cache_write"],
                    })
                });
                events.push((
                    "TurnEnd",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "reason": stop_reason,
                        "num_turns": index,
                        "usage": usage,
                        "cost": cost,
                        "context_size": context_size,
                        "context_window": context_window,
                    }),
                ));
            }
            HarnessActivity::FailedTool {
                id,
                tool,
                args,
                output_bytes,
                ..
            } => {
                events.push((
                    "ToolStart",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                    }),
                ));
                events.push((
                    "ToolDone",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "tool_id": id,
                        "tool": tool,
                        "is_error": true,
                        "bytes": output_bytes,
                        "duration_ms": MAKI_TOOL_MS,
                        "input": args,
                    }),
                ));
            }
            HarnessActivity::Shutdown => {
                events.push((
                    "SessionEnd",
                    serde_json::json!({
                        "session_id": DRIVER_SESSION,
                        "reason": "shutdown",
                    }),
                ));
            }
        }
    }
    events
}

const MAKI_TOOL_MS: u64 = 40;

pub fn drive_maki_reporter(
    env: &common::TestEnv,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let events = maki_events_for(activities);
    run_maki_driver(env, &events)
}

fn run_node_driver(
    env: &common::TestEnv,
    script_name: &str,
    script: &str,
    reporter: &Path,
    activities: &[HarnessActivity],
) -> std::process::Output {
    let root = env.root();
    let bin = shim_givn_on_path(env);
    let script_path = root.join(script_name);
    std::fs::write(&script_path, script)
        .unwrap_or_else(|error| panic!("write the driver {}: {error}", script_path.display()));
    let activities: Vec<Value> = activities.iter().map(HarnessActivity::as_json).collect();
    let output = std::process::Command::new("node")
        .arg(&script_path)
        .current_dir(&root)
        .env("PATH", driver_path(&[&bin]))
        .env("GIVN_DRIVER_REPORTER", reporter)
        .env("GIVN_DRIVER_SESSION", DRIVER_SESSION)
        .env(
            "GIVN_DRIVER_ACTIVITIES",
            serde_json::Value::Array(activities).to_string(),
        )
        .output()
        .unwrap_or_else(|error| panic!("run the driver {}: {error}", script_path.display()));
    assert!(
        output.status.success(),
        "the driver {} failed (exit {:?}): {}{}",
        script_path.display(),
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

const OMP_DRIVER: &str = r#"import { spawnSync } from "node:child_process"
import { pathToFileURL } from "node:url"

const activities = JSON.parse(process.env.GIVN_DRIVER_ACTIVITIES)
const handlers = new Map()
const eventHandlers = new Map()
const pi = {
  on: (event, handler) => handlers.set(event, handler),
  events: { on: (event, handler) => eventHandlers.set(event, handler) },
  getThinkingLevel: () => undefined,
  exec: async (command, args) => {
    const result = spawnSync(command, args, { env: process.env, stdio: ["ignore", "inherit", "inherit"] })
    if (result.status !== 0) throw new Error(`${command} exited ${result.status}`)
  },
}
const module = await import(pathToFileURL(process.env.GIVN_DRIVER_REPORTER).href)
module.default(pi)
const fire = async (event, payload, ctx = {}) => {
  const handler = handlers.get(event)
  if (!handler) throw new Error(`the reporter registered no ${event} handler`)
  await handler(payload, ctx)
}
await fire("session_start", {}, { sessionManager: { getSessionId: () => process.env.GIVN_DRIVER_SESSION } })
for (const activity of activities) {
  if (activity.kind === "tool") {
    await fire("tool_execution_start", { toolCallId: activity.id, toolName: activity.tool, args: activity.args })
    await fire("tool_execution_end", { toolCallId: activity.id, toolName: activity.tool, args: activity.args, isError: false })
  } else if (activity.kind === "question") {
    const args = { questions: [{ question: activity.question }] }
    await fire("tool_execution_start", { toolCallId: activity.id, toolName: "ask", args })
    await fire("tool_execution_end", { toolCallId: activity.id, toolName: "ask", args, isError: false })
  } else if (activity.kind === "subagent") {
    const handler = eventHandlers.get("task:subagent:lifecycle")
    if (!handler) throw new Error("the reporter subscribed to no subagent lifecycle")
    handler({ id: activity.id, agent: activity.agent, status: "started" })
    handler({ id: activity.id, agent: activity.agent, status: "completed" })
  } else if (activity.kind === "turn") {
    await fire("turn_start", { turnIndex: activity.index })
    await fire("turn_end", { turnIndex: activity.index, message: { model: activity.model, usage: activity.usage } })
  } else if (activity.kind === "turn_metadata") {
    await fire("turn_start", { turnIndex: activity.index })
    const usage = {
      ...(activity.usage ?? {}),
      cost: activity.cost,
      contextTokens: activity.context_size,
      contextWindow: activity.context_window,
    }
    await fire(
      "turn_end",
      { turnIndex: activity.index, message: { model: activity.model, stopReason: activity.stop_reason, usage } },
      { getContextUsage: () => ({ tokens: activity.context_size, contextWindow: activity.context_window }) },
    )
  } else if (activity.kind === "failed_tool") {
    await fire("tool_execution_start", { toolCallId: activity.id, toolName: activity.tool, args: activity.args })
    await fire("tool_execution_end", {
      toolCallId: activity.id,
      toolName: activity.tool,
      args: activity.args,
      isError: true,
      result: { error: activity.error, bytes: activity.output_bytes, exit_code: 101 },
    })
  } else if (activity.kind === "shutdown") {
    await fire("session_shutdown", {})
  }
}
"#;

const OPENCODE_DRIVER: &str = r#"import { pathToFileURL } from "node:url"

const activities = JSON.parse(process.env.GIVN_DRIVER_ACTIVITIES)
const session = process.env.GIVN_DRIVER_SESSION
const hooks = new Map()
const queued = []
let wake = null
let closed = false
const stream = {
  async *[Symbol.asyncIterator]() {
    while (!closed) {
      if (queued.length > 0) {
        yield queued.shift()
        continue
      }
      await new Promise((resolve) => { wake = resolve })
      wake = null
    }
  },
}
const registration = (domain) => (kind, handler) => hooks.set(`${domain}:${kind}`, handler)
const ctx = {
  session: { hook: registration("session") },
  tool: { hook: registration("tool") },
  shell: { hook: registration("shell") },
  permission: { hook: registration("permission") },
  event: { subscribe: () => stream },
  signal: undefined,
}
const module = await import(pathToFileURL(process.env.GIVN_DRIVER_REPORTER).href)
await module.default.setup(ctx)
let clock = Date.now()
const stamp = () => (clock += 50)
const emit = async (event) => {
  queued.push(event)
  if (wake) {
    const resolve = wake
    wake = null
    resolve()
  }
  // The consumer handles one event at a time; give it the turn it needs before
  // the next event of the same session.
  await new Promise((resolve) => setTimeout(resolve, 25))
}
// The session's own boundary event: it is what makes the reporter materialize
// and flush, so the driver's activities and its session end share one shape.
const boundary = async () => {
  await emit({ type: "session.execution.succeeded", created: stamp(), data: { sessionID: session } })
  await new Promise((resolve) => setTimeout(resolve, 200))
}
for (const activity of activities) {
  if (activity.kind === "tool") {
    await emit({ type: "session.tool.called", created: stamp(), data: { sessionID: session, id: activity.id, name: activity.tool, input: activity.args } })
    await emit({ type: "session.tool.success", created: stamp(), data: { sessionID: session, id: activity.id, name: activity.tool } })
  } else if (activity.kind === "question") {
    await emit({ type: "permission.asked", created: stamp(), data: { sessionID: session, id: activity.id, tool: activity.question } })
    await emit({ type: "permission.replied", created: stamp(), data: { sessionID: session, id: activity.id } })
  } else if (activity.kind === "subagent") {
    await emit({ type: "session.created", created: stamp(), data: { sessionID: session, id: activity.id, parentID: session, agent: activity.agent } })
    await emit({ type: "session.execution.succeeded", created: stamp(), data: { sessionID: activity.id } })
  } else if (activity.kind === "turn") {
    await emit({ type: "session.step.started", created: stamp(), data: { sessionID: session, assistantMessageID: `step-${activity.index}` } })
    await emit({ type: "session.step.ended", created: stamp(), data: { sessionID: session, assistantMessageID: `step-${activity.index}`, model: activity.model, tokens: activity.usage } })
  } else if (activity.kind === "turn_metadata") {
    await emit({ type: "session.step.started", created: stamp(), data: { sessionID: session, assistantMessageID: `step-${activity.index}` } })
    await emit({ type: "session.step.ended", created: stamp(), data: {
      sessionID: session,
      assistantMessageID: `step-${activity.index}`,
      model: activity.model,
      finish: activity.stop_reason,
      rawFinish: activity.stop_reason,
      cost: activity.cost,
      tokens: activity.usage,
    } })
  } else if (activity.kind === "failed_tool") {
    await emit({ type: "session.tool.called", created: stamp(), data: { sessionID: session, id: activity.id, name: activity.tool, input: activity.args } })
    await emit({ type: "session.tool.failed", created: stamp(), data: {
      sessionID: session,
      id: activity.id,
      name: activity.tool,
      error: activity.error,
      bytes: activity.output_bytes,
    } })
  } else if (activity.kind === "shutdown") {
    await boundary()
  }
}
// The session's own end: a driver whose activities did not report it still
// closes with one final boundary — the reporter materializes whatever never
// closed and flushes the last payload — so the record receives the buffer
// either way.
if (!activities.some((activity) => activity.kind === "shutdown")) await boundary()
closed = true
if (wake) {
  const resolve = wake
  wake = null
  resolve()
}
await new Promise((resolve) => setTimeout(resolve, 200))
"#;

/// The model a fake Maki plugin reports through its UI or its session snapshot.
pub struct MakiModel<'a> {
    pub spec: Option<&'a str>,
    pub reasoning: Option<&'a str>,
    /// The real harness has no UI, so `maki.model.get` fails and the layer has
    /// to read the session snapshot instead.
    pub headless: bool,
}

/// The record entries whose label or name carries a topic marker.
pub fn markers_for<'a>(
    entries: &'a [serde_json::Value],
    topic: &str,
    label: &str,
) -> Vec<&'a serde_json::Value> {
    entries
        .iter()
        .filter(|entry| entry["kind"].as_str() == Some("phase"))
        .filter(|entry| entry["scope"]["kind"].as_str() == Some("ideation"))
        .filter(|entry| entry["scope"]["id"].as_str() == Some(topic))
        .filter(|entry| entry["label"].as_str() == Some(label))
        .collect()
}
