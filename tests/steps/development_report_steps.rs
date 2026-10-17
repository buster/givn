//! Step definitions for the `development-report` capability.
//!
//! The scenarios generate a report through `givn timeline` and assert on the
//! file the command wrote — the JSON model in its data slot and the text it
//! renders — never on the store alone; the record fixtures live in
//! [`crate::steps::development_timeline_support`].

use std::path::PathBuf;

use cucumber::{given, then, when};
use givn::cli::timeline::{format_count, format_duration};
use givn::history::{HistoryEntry, Scope};
use serde_json::{Value, json};

use crate::common;
use crate::steps::GivnWorld;
use crate::steps::development_timeline_support as support;

/// One entry the ranking fixture's record holds.
#[derive(Debug, Clone, Copy)]
struct FixtureEntry {
    /// The recorded entry kind.
    kind: &'static str,
    /// The entry's domain name.
    name: &'static str,
    /// The entry's bounded subject label.
    label: &'static str,
    /// The entry's recorded end timestamp.
    at: &'static str,
    /// The measured duration; `None` is an activity the record does not time.
    duration: Option<u64>,
    /// The recorded exit code (the `data` a step entry carries).
    exit_code: i64,
}

/// The steps the ranking fixture's record holds, in the order the store holds
/// them — deliberately neither the duration order the ranking must produce nor,
/// for the untimed tool call, a measured duration at all.
const RANKING_FIXTURE: [FixtureEntry; 4] = [
    FixtureEntry {
        kind: "step",
        name: "lint",
        label: "cargo clippy --locked --all-targets -- -D warnings",
        at: "2026-10-16T09:16:00.000Z",
        duration: Some(120_000),
        exit_code: 1,
    },
    FixtureEntry {
        kind: "step",
        name: "verification",
        label: "cargo test --locked --test features_runner",
        at: "2026-10-16T09:55:00.000Z",
        duration: Some(2_460_000),
        exit_code: 0,
    },
    FixtureEntry {
        kind: "step",
        name: "coverage",
        label: "cargo llvm-cov nextest --locked --test features_runner",
        at: "2026-10-16T10:30:00.000Z",
        duration: Some(780_000),
        exit_code: 0,
    },
    FixtureEntry {
        kind: "tool",
        name: "Bash",
        label: "cargo llvm-cov report --cobertura",
        at: "2026-10-16T10:20:00.000Z",
        duration: None,
        exit_code: 0,
    },
];

/// The index (in the fixture batch above) of the step the untimed tool call
/// belongs to: the coverage step, whose span the call falls inside.
const COVERAGE_STEP_INDEX: usize = 2;

/// The waits the question fixture's record holds: the question the report must
/// total, its recorded end, and its measured duration.
const WAIT_FIXTURE: [(&str, &str, u64); 2] = [
    (
        "Which report layout should the timeline use?",
        "2026-10-16T09:05:00.000Z",
        300_000,
    ),
    (
        "Should the axis label every day boundary?",
        "2026-10-16T09:15:00.000Z",
        180_000,
    ),
];

/// The turn the fixture's waits belong to. Its own recorded duration is the
/// scenario's control: a wait is its own span, so the wait total must be the
/// waits' durations and never the enclosing turn's.
const WAIT_TURN: (&str, u64) = ("2026-10-16T09:20:00.000Z", 1_200_000);

/// The token counts one reported usage carries, in the record's own `data` keys.
#[derive(Debug, Clone, Copy)]
struct UsageCounts {
    input: u64,
    output: u64,
    cache_read: u64,
    cache_write: u64,
}

impl UsageCounts {
    /// The usage's own total — the count the report's roll-ups add up.
    fn total(&self) -> u64 {
        self.input + self.output + self.cache_read + self.cache_write
    }

    /// The `data` a `usage` entry carries: every count the harness reported, the
    /// model that produced it, and the subagent whose span owns it.
    fn data(&self, model: &str, subagent: Option<&str>) -> Value {
        let mut data = json!({
            "input": self.input,
            "output": self.output,
            "cache_read": self.cache_read,
            "cache_write": self.cache_write,
            "total_tokens": self.total(),
            "model": model,
        });
        if let Some(subagent) = subagent {
            data["subagent"] = json!(subagent);
        }
        data
    }
}

/// One reported usage: the model that produced it, its counts, and the subagent
/// whose span owns it where the usage belongs to a subagent.
#[derive(Debug, Clone, Copy)]
struct UsageFixture {
    model: &'static str,
    counts: UsageCounts,
    subagent: Option<&'static str>,
}

/// One turn the fixture's record holds: its recorded end, its measured duration,
/// and the usage the harness reported for it. `None` is the turn whose usage the
/// harness never reported — the record marks that turn's usage unavailable, and
/// the report must state it rather than fold it in as zero.
#[derive(Debug, Clone, Copy)]
struct TurnFixture {
    at: &'static str,
    duration_ms: u64,
    usage: Option<UsageFixture>,
}

/// One subagent the fixture's record holds: its identity, the agent definition
/// that ran it, its recorded end and measured duration, and its own usage where
/// the harness reports one.
#[derive(Debug, Clone, Copy)]
struct SubagentFixture {
    identity: &'static str,
    agent: &'static str,
    at: &'static str,
    duration_ms: u64,
    usage: Option<UsageFixture>,
}

/// The first model's counts: one turn's usage.
const SONNET_COUNTS: UsageCounts = UsageCounts {
    input: 12_043,
    output: 812,
    cache_read: 9_000,
    cache_write: 240,
};

/// The second model's counts: a second model really produced usage.
const CODEX_COUNTS: UsageCounts = UsageCounts {
    input: 4_000,
    output: 600,
    cache_read: 400,
    cache_write: 0,
};

/// The subagent's own counts: the usage its own span owns.
const SCOUT_COUNTS: UsageCounts = UsageCounts {
    input: 2_400,
    output: 300,
    cache_read: 300,
    cache_write: 0,
};

/// The turns the token fixture's record holds, in the order the store holds
/// them: two turns with reported usage for two different models, and one turn
/// whose usage the harness never reported.
const TOKEN_TURNS: [TurnFixture; 3] = [
    TurnFixture {
        at: "2026-10-16T09:20:00.000Z",
        duration_ms: 600_000,
        usage: None,
    },
    TurnFixture {
        at: "2026-10-16T09:40:00.000Z",
        duration_ms: 900_000,
        usage: Some(UsageFixture {
            model: "claude-sonnet-5",
            counts: SONNET_COUNTS,
            subagent: None,
        }),
    },
    TurnFixture {
        at: "2026-10-16T10:00:00.000Z",
        duration_ms: 300_000,
        usage: Some(UsageFixture {
            model: "gpt-5-codex",
            counts: CODEX_COUNTS,
            subagent: None,
        }),
    },
];

/// The subagents the token fixture's record holds: one whose own span carries a
/// usage entry, and one the harness reports no usage for at all.
const TOKEN_SUBAGENTS: [SubagentFixture; 2] = [
    SubagentFixture {
        identity: "Scout",
        agent: "Architect",
        at: "2026-10-16T10:10:00.000Z",
        duration_ms: 240_000,
        usage: Some(UsageFixture {
            model: "claude-sonnet-5",
            counts: SCOUT_COUNTS,
            subagent: Some("Scout"),
        }),
    },
    SubagentFixture {
        identity: "Helper",
        agent: "Scout",
        at: "2026-10-16T10:12:00.000Z",
        duration_ms: 60_000,
        usage: None,
    },
];

/// The subagents the fan-out fixture's record holds: the identity each span ran
/// under, the agent definition that ran it, its recorded end, and the duration
/// the harness measured for it. `None` is a subagent the record does not time —
/// it still counts toward the fan-out and never toward the combined duration.
const FAN_OUT_SUBAGENTS: [(&str, &str, &str, Option<u64>); 6] = [
    ("Scout", "explore", "2026-10-16T09:02:00.000Z", Some(20_000)),
    ("Scout", "explore", "2026-10-16T09:05:00.000Z", Some(15_000)),
    (
        "Architect",
        "design",
        "2026-10-16T09:08:00.000Z",
        Some(12_000),
    ),
    ("Scout", "explore", "2026-10-16T09:11:00.000Z", Some(8_000)),
    ("Reviewer", "review", "2026-10-16T09:14:00.000Z", None),
    (
        "Helper",
        "implement",
        "2026-10-16T09:17:00.000Z",
        Some(5_000),
    ),
];

/// The turn the fan-out fixture's subagents were spawned from: its recorded end
/// and the duration it was measured with. The turn's label names no subagent
/// and no subagent's label names the turn, so the record's own `parent` link is
/// the only spawn relationship in the fixture — a report that inferred the
/// fan-out from label text cannot pass.
const FAN_OUT_TURN: (&str, u64) = ("2026-10-16T09:20:00.000Z", 1_200_000);

/// The turn's bounded subject label, the text a label-scanning fan-out would
/// have to match to find its spawned subagents.
const FAN_OUT_TURN_LABEL: &str = "implement the fan-out summary";

/// The short agent activities the dense fixture's record holds: how many there
/// are, the spacing between their recorded ends, and the duration each was
/// measured with. Together they make every activity far shorter than the time
/// one pixel of the initial zoom spends on (the fixture spreads the activities
/// over a quarter of an hour), so a report that drew one shape per entry would
/// be unreadable — the scenario's own claim, asserted from the seeded record.
const DENSE_ACTIVITIES: usize = 240;
const DENSE_SPACING_MS: i64 = 4_000;
const DENSE_DURATION_MS: u64 = 250;

/// The `at` of the dense fixture's `index`-th activity: the fixture's own base
/// instant (09:00:00 UTC on the record's date) plus its offset, in the record's
/// RFC 3339 form. The offsets are whole seconds inside the base's hour, so the
/// helper renders the time of day and the date stays the fixture's.
fn dense_activity_at(index: usize) -> String {
    let second_of_day = 9 * 3_600 + index as i64 * DENSE_SPACING_MS / 1_000;
    format!(
        "2026-10-16T{:02}:{:02}:{:02}.000Z",
        second_of_day / 3_600,
        second_of_day % 3_600 / 60,
        second_of_day % 60,
    )
}

/// One aggregate marker as the written report draws it: the bucket it covers,
/// the count and combined duration it names in its own machine-readable
/// attributes, and its accessible name — the three ways the marker names what
/// it stands for.
#[derive(Debug)]
struct DrawnAggregate {
    bucket_start_ms: i64,
    bucket_end_ms: i64,
    count: usize,
    duration_ms: Option<u64>,
    name: String,
}

/// One measured entry the widened-ranking fixture's record holds: the entry
/// kind, its name and bounded label, its recorded end, and its measured
/// duration. The agent activity is an ordinary `tool` entry — the kind an agent
/// reporter produces — never a step in disguise.
#[derive(Debug, Clone, Copy)]
struct MeasuredEntry {
    kind: &'static str,
    name: &'static str,
    label: &'static str,
    at: &'static str,
    duration_ms: u64,
}

/// The steps and the agent tool call the widened-ranking fixture's record holds,
/// in the order the store holds them: two measured `step` entries Givn ran, and
/// one measured `tool` entry an agent reported that is longer than both — so the
/// ranking's first row is a non-step entry, and a ranking restricted to `step`
/// entries cannot pass.
const MEASURED_ACTIVITY_FIXTURE: [MeasuredEntry; 3] = [
    MeasuredEntry {
        kind: "step",
        name: "lint",
        label: "cargo clippy --locked --all-targets -- -D warnings",
        at: "2026-10-16T09:16:00.000Z",
        duration_ms: 120_000,
    },
    MeasuredEntry {
        kind: "step",
        name: "verification",
        label: "cargo test --locked --test features_runner",
        at: "2026-10-16T09:55:00.000Z",
        duration_ms: 600_000,
    },
    MeasuredEntry {
        kind: "tool",
        name: "Bash",
        label: "cargo llvm-cov nextest --locked --test features_runner",
        at: "2026-10-16T10:30:00.000Z",
        duration_ms: 2_460_000,
    },
];

/// State for the development-report scenarios: the report the last generation
/// step wrote, so a following step reads the file the command produced rather
/// than the store, and the record the scenario's Given seeded, so the
/// assertions derive from the fixture the scenario claims.
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
}

/// The capability's state, created on first use.
fn state(world: &mut GivnWorld) -> &mut DevelopmentReportState {
    world
        .development_report
        .get_or_insert_with(DevelopmentReportState::default)
}

/// The record the scenario's Given seeded.
fn seeded(world: &GivnWorld) -> &[HistoryEntry] {
    world
        .development_report
        .as_ref()
        .map(|state| state.seeded.as_slice())
        .unwrap_or_default()
}

/// The fixture root the scenario's first Given brought up: an initialised
/// project root, exactly the one the other capabilities' fixtures build.
fn project_root(world: &mut GivnWorld) -> &common::TestEnv {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// Run one fixture command and fail the scenario on a non-zero exit — a
/// fixture step's own setup is a precondition, never an assertion under test.
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

/// The attributes a document loads another resource through.
const LOADING_ATTRIBUTES: [&str; 6] = ["src", "href", "srcset", "poster", "action", "formaction"];

/// The inline forms that fetch: a document carrying any of these reaches
/// outside its own file even without a loading attribute.
const REMOTE_FETCH_FORMS: [&str; 5] = ["@import", "fetch(", "XMLHttpRequest", "url(http", "url(//"];

/// The remote-resource reference a report must never carry, named as the
/// document spells it. The scan reads the written file for the two ways a
/// document reaches outside itself: a loading attribute
/// ([`LOADING_ATTRIBUTES`]) whose value is an absolute or protocol-relative
/// URL, and an inline form that fetches ([`REMOTE_FETCH_FORMS`]). Nothing else
/// is a reference: a `data:` URL, a fragment (`href="#…"`), and a relative path
/// stay inside the file, and the SVG namespace declaration
/// (`xmlns="http://www.w3.org/2000/svg"`) is a namespace URI the browser never
/// resolves to a request.
fn remote_resource_reference(document: &str) -> Option<String> {
    for attribute in LOADING_ATTRIBUTES {
        let marker = format!("{attribute}=\"");
        let mut rest = document;
        while let Some(at) = rest.find(&marker) {
            rest = &rest[at + marker.len()..];
            let value = rest.split('"').next().unwrap_or_default();
            if value.starts_with("//") || value.starts_with("http:") || value.starts_with("https:")
            {
                return Some(format!("{attribute}=\"{value}\""));
            }
        }
    }
    REMOTE_FETCH_FORMS
        .into_iter()
        .find(|marker| document.contains(marker))
        .map(str::to_string)
}

/// The report the last generation step wrote, read as the written file — the
/// command's own output, never the store it was built from.
fn report_text(world: &GivnWorld) -> String {
    let path = world
        .development_report
        .as_ref()
        .and_then(|state| state.report_path.clone())
        .expect("the development report has not been generated yet");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the report {}: {error}", path.display()))
}

/// The JSON model the written report carries in its data slot.
fn report_model(document: &str) -> Value {
    let slot = document
        .find("<script id=\"report-model\"")
        .expect("the report carries no data slot");
    let content = document[slot..]
        .find('>')
        .map(|offset| slot + offset + 1)
        .expect("the data slot's element is never closed");
    let end = document[content..]
        .find("</script>")
        .map(|offset| content + offset)
        .expect("the data slot is never closed");
    serde_json::from_str(&document[content..end])
        .unwrap_or_else(|error| panic!("the data slot does not hold the JSON model: {error}"))
}

/// The `data-entry-id` values of a generated list, in document order.
fn list_entry_ids(document: &str, list_id: &str) -> Vec<u64> {
    let marker = format!("id=\"{list_id}\"");
    let start = document
        .find(&marker)
        .unwrap_or_else(|| panic!("the report holds no #{list_id} list"));
    let region = &document[start..];
    let end = region.find("</ol>").expect("the list is never closed");
    region[..end]
        .split("<li ")
        .skip(1)
        .map(|item| {
            item.split("data-entry-id=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .and_then(|value| value.parse().ok())
                .unwrap_or_else(|| panic!("a listed row carries no entry id: {item}"))
        })
        .collect()
}

/// The summary tile named `name` as the report renders it: its whole markup, so
/// an assertion reads the value the screen shows rather than a re-derivation.
fn tile(document: &str, name: &str) -> String {
    let marker = format!("data-tile=\"{name}\"");
    let start = document
        .find(&marker)
        .unwrap_or_else(|| panic!("the report holds no `{name}` tile"));
    let region = &document[start..];
    let end = region.find("</div>").expect("the tile is never closed");
    region[..end].to_string()
}

#[given(regex = r#"^a change "([^"]+)" whose record holds steps of different durations$"#)]
fn change_with_steps_of_different_durations(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        for (index, fixture) in RANKING_FIXTURE.into_iter().enumerate() {
            let entry = match fixture.duration {
                Some(duration_ms) => {
                    history.span(&scope, fixture.kind, fixture.name, fixture.at, duration_ms)
                }
                None => history.point(&scope, fixture.kind, fixture.name, fixture.at),
            };
            assert_eq!(entry, index, "the fixture seeds its entries in order");
            history.label(entry, fixture.label);
            history.data(entry, json!({ "exit_code": fixture.exit_code }));
            history.outcome(
                entry,
                if fixture.exit_code == 0 {
                    "ok"
                } else {
                    "failed"
                },
            );
            if fixture.kind == "tool" {
                history.nested_in(entry, COVERAGE_STEP_INDEX);
            }
        }
        history.seed(env)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a change "([^"]+)" whose record holds two question waits$"#)]
fn change_with_two_question_waits(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        let turn = history.span(&scope, "turn", "turn", WAIT_TURN.0, WAIT_TURN.1);
        for (index, (question, at, duration_ms)) in WAIT_FIXTURE.into_iter().enumerate() {
            let wait = history.span(&scope, "question", "question", at, duration_ms);
            history.label(wait, question);
            history.nested_in(wait, turn);
            assert_eq!(
                wait,
                turn + 1 + index,
                "the fixture nests every wait in its turn"
            );
        }
        history.seed(env)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[given(regex = r#"^a change "([^"]+)" whose record holds reported token usage$"#)]
fn change_with_reported_token_usage(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        for turn in TOKEN_TURNS {
            let reported = history.span(&scope, "turn", "turn", turn.at, turn.duration_ms);
            let Some(usage) = turn.usage else {
                // The turn the harness reported no usage for: the record marks
                // the usage unavailable, never as a zero-filled report.
                history.data(reported, json!({ "usage": "unavailable" }));
                continue;
            };
            let entry = history.point(&scope, "usage", usage.model, turn.at);
            history.data(entry, usage.counts.data(usage.model, usage.subagent));
            history.nested_in(entry, reported);
        }
        for subagent in TOKEN_SUBAGENTS {
            let span = history.span(
                &scope,
                "subagent",
                subagent.identity,
                subagent.at,
                subagent.duration_ms,
            );
            history.data(
                span,
                json!({ "agent": subagent.agent, "subagent": subagent.identity }),
            );
            let Some(usage) = subagent.usage else {
                continue;
            };
            let entry = history.point(&scope, "usage", usage.model, subagent.at);
            history.data(entry, usage.counts.data(usage.model, usage.subagent));
            history.nested_in(entry, span);
        }
        history.seed(env)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[given(regex = r#"^an ideation topic "([^"]+)" whose record holds the topic's own steps$"#)]
fn ideation_topic_with_its_own_steps(world: &mut GivnWorld, topic: String) {
    let seeded = {
        let env = project_root(world);
        // The scenario's own claim: the topic's work exists before any change
        // directory does.
        support::assert_no_active_change(env);
        support::seed_ideation_topic(env, &topic)
    };
    state(world).seeded = seeded;
}

#[when(regex = r#"^the developer generates the development report for the topic "([^"]+)"$"#)]
fn generate_report_for_topic(world: &mut GivnWorld, topic: String) {
    let report = {
        let env = project_root(world);
        run_fixture_command(
            env,
            &["timeline", "--topic", &topic, "--out", "report.html"],
        );
        env.path("report.html")
    };
    assert!(
        report.is_file(),
        "the command wrote no report at {}",
        report.display()
    );
    state(world).report_path = Some(report);
}

#[given(
    regex = r#"^a change "([^"]+)" whose record holds an entry appended after an entry with a later end time$"#
)]
fn change_with_an_entry_appended_after_a_later_end(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        support::seed_out_of_order_steps(env, &scope)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" whose record holds an agent tool call longer than any step$"#
)]
fn change_with_an_agent_tool_call_longer_than_any_step(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        for fixture in MEASURED_ACTIVITY_FIXTURE {
            let entry = history.span(
                &scope,
                fixture.kind,
                fixture.name,
                fixture.at,
                fixture.duration_ms,
            );
            history.label(entry, fixture.label);
        }
        history.seed(env)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a change "([^"]+)" whose record holds captured metadata and full-mode content$"#
)]
fn change_with_captured_metadata_and_full_content(world: &mut GivnWorld, id: String) {
    let seeded = {
        let env = project_root(world);
        support::seed_captured_metadata_and_content(env, &id)
    };
    let state = state(world);
    state.store_before = Some(seeded.store_bytes.clone());
    state.metadata_entry = Some(seeded.metadata_entry.clone());
    state.content_entry_ids = seeded.content_entry_ids.clone();
    world.last_change_id = Some(id);
}

/// The embedded text bytes of one `data.content` object, summed the way the
/// report budget counts them.
fn content_text_bytes(content: &Value) -> u64 {
    content
        .as_object()
        .map(|object| {
            object
                .iter()
                .filter(|(name, _)| name.as_str() != "truncated")
                .filter_map(|(_, field)| field["text"].as_str())
                .map(|text| text.len() as u64)
                .sum()
        })
        .unwrap_or_default()
}

#[then(
    "the report model names the session, its title, the status, the stop reason, the cost, the context size and window, the tool call id, the output size, the error, the parent, the retry, and the permission mode"
)]
fn report_model_names_the_captured_metadata(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let metadata = state(world)
        .metadata_entry
        .clone()
        .expect("the details-panel fixture ran first");
    let entry = model["entries"]
        .as_array()
        .unwrap_or_else(|| panic!("the report model carries no entries: {model:#?}"))
        .iter()
        .find(|entry| entry["id"] == json!(metadata.id))
        .unwrap_or_else(|| {
            panic!(
                "the model lost the metadata entry {}: {model:#?}",
                metadata.id
            )
        });
    let data = &entry["data"];
    assert_eq!(
        entry["session"],
        json!("session-25b"),
        "the model does not name the harness session: {entry}"
    );
    assert_eq!(data["title"], json!("Development report details"));
    assert_eq!(data["status"], json!("needs_input"));
    assert_eq!(data["stop_reason"], json!("end_turn"));
    assert_eq!(data["model"], json!("deepseek/deepseek-flash"));
    assert_eq!(data["cost"], json!(0.42));
    assert_eq!(data["list_cost"], json!(0.55));
    assert_eq!(data["context_size"], json!(48_000));
    assert_eq!(data["context_window"], json!(128_000));
    assert_eq!(data["tool_call_id"], json!("call-25b"));
    assert_eq!(data["output_bytes"], json!(4_096));
    assert_eq!(data["error"], json!("the command failed: exit status 101"));
    assert_eq!(
        entry["parent"],
        json!(
            metadata
                .parent
                .expect("the fixture nests the metadata entry in a turn")
        ),
        "the model does not name the entry's parent: {entry}"
    );
    assert_eq!(data["agent"], json!("omp"));
    assert_eq!(data["attempt"], json!(2));
    assert_eq!(data["max_attempts"], json!(3));
    assert_eq!(data["delay_ms"], json!(1_500));
    assert_eq!(data["fallback_from"], json!("model-a"));
    assert_eq!(data["fallback_to"], json!("model-b"));
    assert_eq!(data["permission_mode"], json!("acceptEdits"));
    // The panel's fact labels for every value are in the rendered document.
    for label in [
        "Session",
        "Session title",
        "Status",
        "Stop reason",
        "Model",
        "Cost",
        "List cost",
        "Context size",
        "Context window",
        "Tool call id",
        "Output bytes",
        "Error",
        "Parent",
        "Agent",
        "Retry",
        "Permission mode",
    ] {
        assert!(
            document.contains(label),
            "the report does not render the `{label}` fact: {document:.0}"
        );
    }
}

#[then("the report embeds the content within the 8 MiB documented budget")]
fn report_embeds_content_within_the_budget(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let budget = 8_388_608u64;
    assert_eq!(
        model["content_budget_bytes"],
        json!(budget),
        "the model does not state the documented budget: {model:#?}"
    );
    let used = model["content_bytes"]
        .as_u64()
        .unwrap_or_else(|| panic!("the model does not state the embedded bytes: {model:#?}"));
    assert!(
        used > 0 && used <= budget,
        "the embedded content is not within the budget ({used} bytes): {model:#?}"
    );
    let mut embedded = 0usize;
    let mut omitted = 0usize;
    let mut total = 0u64;
    for entry in model["entries"]
        .as_array()
        .unwrap_or_else(|| panic!("the report model carries no entries: {model:#?}"))
    {
        let content = entry["data"].get("content");
        let marker = entry["data"].get("content_omitted");
        match (content, marker) {
            (Some(content), None) => {
                embedded += 1;
                total += content_text_bytes(content);
            }
            (None, Some(marker)) => {
                assert_eq!(
                    marker,
                    &json!(true),
                    "an omitted entry is not marked content_omitted: {entry}"
                );
                omitted += 1;
            }
            _ => {}
        }
    }
    assert!(
        embedded > 0,
        "the fixture's content was not embedded at all: {model:#?}"
    );
    assert!(
        omitted > 0,
        "the fixture's content did not exceed the budget: {model:#?}"
    );
    assert_eq!(
        total, used,
        "the model's content_bytes does not match the embedded content: {model:#?}"
    );
}

#[then(regex = r#"^the report notes "([^"]+)" beyond the budget$"#)]
fn report_notes_content_omitted(world: &mut GivnWorld, note: String) {
    let document = report_text(world);
    let model = report_model(&document);
    assert_eq!(
        model["content_omitted_note"],
        json!(note),
        "the model does not carry the omission note: {model:#?}"
    );
    assert!(
        document.contains(&note),
        "the report text view does not carry the omission note: {note:?}"
    );
    let omitted = model["omitted_content"]
        .as_array()
        .unwrap_or_else(|| panic!("the model lists no omitted content: {model:#?}"));
    assert!(
        !omitted.is_empty(),
        "the fixture does not exceed the content budget: {model:#?}"
    );
    for id in omitted {
        let id = id
            .as_u64()
            .unwrap_or_else(|| panic!("an omitted id is not a number: {id}"));
        assert!(
            document.contains(&format!("data-entry-id=\"{id}\"")),
            "the text view names no omission for entry {id}"
        );
    }
}

#[then("the development record still holds every content byte")]
fn record_still_holds_every_content_byte(world: &mut GivnWorld) {
    let root = project_root(world).root();
    let (before, seeded_entries) = {
        let state = state(world);
        (
            state
                .store_before
                .clone()
                .expect("the details-panel fixture ran first"),
            state.content_entry_ids.len(),
        )
    };
    let after = std::fs::read(support::store_file(&root))
        .unwrap_or_else(|error| panic!("read the development record: {error}"));
    assert_eq!(
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&after),
        "the report run changed the development record"
    );
    // Not vacuous: the store still carries every seeded content bundle.
    let store = String::from_utf8_lossy(&after);
    let bundles = store.matches("\"content\":").count();
    assert!(
        bundles > seeded_entries,
        "the store lost content bundles: {bundles} <= {seeded_entries}"
    );
}

#[when(regex = r#"^the developer generates the development report for "([^"]+)"$"#)]
fn generate_report_for_change(world: &mut GivnWorld, id: String) {
    let report = {
        let env = project_root(world);
        run_fixture_command(env, &["timeline", "--change", &id, "--out", "report.html"]);
        env.path("report.html")
    };
    assert!(
        report.is_file(),
        "the command wrote no report at {}",
        report.display()
    );
    state(world).report_path = Some(report);
}

#[then("the report ranks the longest step first")]
fn report_ranks_the_longest_step_first(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let steps: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.kind == "step" && entry.duration_ms.is_some())
        .collect();
    assert!(
        steps.len() >= 3,
        "the fixture must hold several measured steps: {steps:#?}"
    );
    // The fixture's own claim: the store's line order is not the duration
    // order, so a report that reused the record's order cannot pass.
    let mut expected: Vec<&HistoryEntry> = steps.clone();
    expected.sort_by_key(|entry| std::cmp::Reverse(entry.duration_ms));
    let expected_ids: Vec<u64> = expected.iter().map(|entry| entry.id).collect();
    let file_order: Vec<u64> = steps.iter().map(|entry| entry.id).collect();
    assert_ne!(
        expected_ids, file_order,
        "the fixture's file order must differ from its duration order"
    );

    let ranked: Vec<u64> = model["summary"]["longest_steps"]
        .as_array()
        .unwrap_or_else(|| panic!("the model ranks no steps: {model:#?}"))
        .iter()
        .map(|step| {
            step["id"]
                .as_u64()
                .unwrap_or_else(|| panic!("a ranked step carries no id: {step}"))
        })
        .collect();
    assert_eq!(
        ranked, expected_ids,
        "the model's ranking is not the recorded steps, longest first: {model:#?}"
    );
    let longest = &expected[0];
    let longest_duration = longest
        .duration_ms
        .expect("the fixture's longest step is measured");
    assert_eq!(
        model["summary"]["longest_steps"][0]["duration_ms"],
        json!(longest_duration),
        "the first ranked step is not the longest recorded step: {model:#?}"
    );

    let rendered = list_entry_ids(&document, "longest-steps");
    assert_eq!(
        rendered, expected_ids,
        "the rendered ranking disagrees with the model's: {document}"
    );
    let longest_label = longest.label.clone().expect("the fixture labels its steps");
    assert!(
        document.contains(&longest_label),
        "the rendered ranking does not name the longest step: {document}"
    );
    assert!(
        document.contains(&format_duration(longest_duration)),
        "the rendered ranking does not name the longest step's duration: {document}"
    );

    // A duration the record does not carry is excluded from the ranking and
    // stated in words, never summed as zero.
    let unmeasured: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.duration_ms.is_none())
        .collect();
    assert!(
        !unmeasured.is_empty(),
        "the fixture holds an untimed entry on purpose"
    );
    for entry in &unmeasured {
        assert!(
            !ranked.contains(&entry.id),
            "an untimed entry was ranked: {entry:#?}"
        );
    }
    assert_eq!(
        model["summary"]["unmeasured_durations"],
        json!(unmeasured.len()),
        "the model does not state the durations the record lacks: {model:#?}"
    );
    assert!(
        document.contains("not measured"),
        "the report does not state a missing duration as not measured: {document}"
    );
}

/// The generated list named `id` as the report renders it: its whole markup.
fn list(document: &str, id: &str) -> String {
    let marker = format!("id=\"{id}\"");
    let start = document
        .find(&marker)
        .unwrap_or_else(|| panic!("the report holds no `{id}` list"));
    let region = &document[start..];
    let end = region
        .find("</ul>")
        .or_else(|| region.find("</ol>"))
        .expect("the list is never closed");
    region[..end].to_string()
}

/// The seeded usage entries, read the way the report reads them.
fn seeded_usage(world: &GivnWorld) -> Vec<&HistoryEntry> {
    seeded(world)
        .iter()
        .filter(|entry| entry.kind == "usage")
        .collect()
}

/// The tokens one seeded entry reports (`data.total_tokens`).
fn reported_tokens(entry: &HistoryEntry) -> Option<u64> {
    entry.data.as_ref()?.get("total_tokens")?.as_u64()
}

/// The model one seeded `usage` entry names as the producer of its counts.
fn reported_model(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("model")?.as_str()
}

/// The subagent one seeded `usage` entry names as the owner of its counts.
fn named_subagent(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("subagent")?.as_str()
}

/// Whether a seeded usage entry belongs to one seeded subagent span — the same
/// two links the record carries: the span's own id as the entry's parent, or
/// the span's identity in the entry's `data.subagent`.
fn belongs_to(usage: &HistoryEntry, span: &HistoryEntry) -> bool {
    usage.parent == Some(span.id) || named_subagent(usage) == Some(span.name.as_str())
}

/// Whether one seeded turn is the turn whose token usage the harness never
/// reported.
fn turn_usage_unavailable(entry: &HistoryEntry) -> bool {
    entry.kind == "turn"
        && entry
            .data
            .as_ref()
            .and_then(|data| data.get("usage"))
            .and_then(|value| value.as_str())
            == Some("unavailable")
}

/// The `usage` rows of a generated token list, as `(name, tokens)` pairs.
fn token_rows(document: &str, id: &str, attribute: &str) -> Vec<(String, Option<u64>)> {
    let marker = format!("{attribute}=\"");
    list(document, id)
        .split("<li ")
        .skip(1)
        .map(|row| {
            let name = row
                .split(&marker)
                .nth(1)
                .and_then(|value| value.split('"').next())
                .unwrap_or_else(|| panic!("a token row names no account: {row}"));
            let value = row
                .split("class=\"token-value\">")
                .nth(1)
                .and_then(|value| value.split('<').next())
                .unwrap_or_else(|| panic!("a token row carries no value: {row}"));
            (name.to_string(), parse_token_value(value))
        })
        .collect()
}

/// The tokens a rendered row names, or `None` for the "unavailable" marker.
fn parse_token_value(value: &str) -> Option<u64> {
    if value == "unavailable" {
        return None;
    }
    Some(
        value
            .replace(',', "")
            .parse()
            .unwrap_or_else(|error| panic!("a token row's value is not a count: {value}: {error}")),
    )
}

#[then("the report names the total tokens for the change")]
fn report_names_the_total_tokens_for_the_change(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let total: u64 = seeded_usage(world)
        .iter()
        .filter_map(|entry| reported_tokens(entry))
        .sum();
    assert_eq!(
        model["summary"]["tokens_total"],
        json!(total),
        "the model's token total is not the usage the record reports: {model:#?}"
    );
    let rendered = tile(&document, "tokens");
    assert!(
        rendered.contains(&format_count(total)),
        "the tokens tile does not name the change's total ({total}): {rendered}"
    );

    // The turn whose usage the harness never reported is stated, never folded
    // into the totals as a zero-filled usage.
    let unavailable = seeded(world)
        .iter()
        .filter(|entry| turn_usage_unavailable(entry))
        .count();
    assert_eq!(
        unavailable, 1,
        "the fixture holds exactly one turn without reported usage"
    );
    assert_eq!(
        model["summary"]["unmeasured_turns"],
        json!(unavailable),
        "the model does not count the turns without reported usage: {model:#?}"
    );
    assert!(
        document.contains("unavailable"),
        "the report does not state the unmeasured turn's usage as unavailable: {document}"
    );
}

#[then("the report names the tokens of every subagent")]
fn report_names_the_tokens_of_every_subagent(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let usage = seeded_usage(world);
    let spans: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.kind == "subagent")
        .collect();
    assert_eq!(
        spans.len(),
        2,
        "the fixture holds two recorded subagents: {spans:#?}"
    );

    let mut expected: Vec<(String, Option<u64>)> = Vec::new();
    for span in &spans {
        let owned = usage
            .iter()
            .filter(|entry| belongs_to(entry, span))
            .filter_map(|entry| reported_tokens(entry))
            .reduce(|total, tokens| total + tokens);
        assert!(
            !expected.iter().any(|(name, _)| name == &span.name.clone()),
            "the fixture's subagent identities are distinct"
        );
        expected.push((span.name.clone(), owned));
    }
    let rows = token_rows(&document, "tokens-by-subagent", "data-subagent");
    let mut sorted = expected.clone();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(
        rows, sorted,
        "the rendered subagent rows disagree with the usage the record reports: {document}"
    );
    let modelled = model["summary"]["tokens_by_subagent"]
        .as_array()
        .unwrap_or_else(|| panic!("the model holds no subagent tokens: {model:#?}"));
    assert_eq!(
        modelled.len(),
        expected.len(),
        "the model invents or drops a subagent row: {model:#?}"
    );
    for (name, tokens) in &expected {
        let row = modelled
            .iter()
            .find(|row| row["name"] == json!(name))
            .unwrap_or_else(|| panic!("the model holds no row for {name}: {model:#?}"));
        assert_eq!(
            row["tokens"].as_u64(),
            *tokens,
            "the model's row for {name} is not the usage its span owns: {row}"
        );
    }
    assert!(
        document.contains("unavailable"),
        "a subagent the harness reports no usage for does not read unavailable: {document}"
    );
}

#[then("the report names the token total for every model that produced the usage")]
fn report_names_the_token_total_for_every_model(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let mut expected: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    for entry in seeded_usage(world) {
        let (Some(model_name), Some(tokens)) = (reported_model(entry), reported_tokens(entry))
        else {
            continue;
        };
        *expected.entry(model_name.to_string()).or_default() += tokens;
    }
    assert_eq!(
        expected.len(),
        2,
        "the fixture reports usage for two models: {expected:#?}"
    );

    let modelled = model["summary"]["tokens_by_model"]
        .as_array()
        .unwrap_or_else(|| panic!("the model holds no per-model tokens: {model:#?}"));
    assert_eq!(
        modelled.len(),
        expected.len(),
        "the model invents or drops a model row: {model:#?}"
    );
    for (name, tokens) in &expected {
        let row = modelled
            .iter()
            .find(|row| row["name"] == json!(name))
            .unwrap_or_else(|| panic!("the model holds no row for {name}: {model:#?}"));
        assert_eq!(
            row["tokens"].as_u64(),
            Some(*tokens),
            "the model's total for {name} is not the usage that model produced: {row}"
        );
    }

    let rendered = list(&document, "tokens-by-model");
    for (name, tokens) in &expected {
        assert!(
            rendered.contains(name.as_str()),
            "the rendered per-model list does not name {name}: {rendered}"
        );
        assert!(
            rendered.contains(&format_count(*tokens)),
            "the rendered per-model list does not name {name}'s total: {rendered}"
        );
    }
}

/// The timeline section of the written report: the region the band rows live
/// in, so an assertion reads the steps *under* their band rather than anywhere
/// in the document.
fn timeline_region(document: &str) -> String {
    let start = document
        .find("<section class=\"timeline-section\"")
        .expect("the report draws no timeline");
    let region = &document[start..];
    let end = region
        .find("</section>")
        .expect("the timeline section is never closed");
    region[..end].to_string()
}

#[then("the report names the topic's steps under the topic scope")]
fn report_names_the_topics_steps_under_the_topic_scope(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let steps: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.kind == "step")
        .collect();
    assert_eq!(
        steps.len(),
        3,
        "the topic fixture holds its own steps: {steps:#?}"
    );

    assert_eq!(
        model["scope"]["kind"],
        json!("ideation"),
        "the report is not scoped to the topic: {model:#?}"
    );
    let topic = model["scope"]["id"]
        .as_str()
        .expect("the report names the topic")
        .to_string();
    let bands = model["bands"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no bands: {model:#?}"));
    assert_eq!(
        bands.len(),
        1,
        "the topic's report is the topic's own band: {model:#?}"
    );
    let band = &bands[0];
    assert_eq!(
        band["kind"],
        json!("ideation"),
        "the band is not the topic's"
    );
    assert_eq!(band["scope"]["id"], json!(topic));
    let band_label = band["label"]
        .as_str()
        .expect("the band carries its visible label")
        .to_string();
    assert!(
        band_label.contains(&topic),
        "the band's label does not name the topic: {band_label}"
    );
    let band_entries: Vec<u64> = band["entries"]
        .as_array()
        .unwrap_or_else(|| panic!("the band covers no entries: {band:#?}"))
        .iter()
        .map(|entry| entry.as_u64().expect("a band entry id"))
        .collect();
    let step_ids: Vec<u64> = steps.iter().map(|step| step.id).collect();
    assert_eq!(
        band_entries, step_ids,
        "the band does not cover exactly the topic's own steps: {model:#?}"
    );

    // The rendered timeline names the band and, inside it, every step: its
    // label and its recorded duration.
    let timeline = timeline_region(&document);
    assert!(
        timeline.contains(&band_label),
        "the timeline does not name the topic band: {timeline}"
    );
    let band_at = timeline
        .find(&format!("data-band=\"{topic}\""))
        .unwrap_or_else(|| panic!("the timeline draws no band for the topic: {timeline}"));
    for step in &steps {
        let label = step.label.clone().expect("the fixture labels its steps");
        let duration = format_duration(step.duration_ms.expect("the fixture measures its steps"));
        let at = timeline
            .find(&label)
            .unwrap_or_else(|| panic!("the timeline does not name the step {label}: {timeline}"));
        assert!(
            at > band_at,
            "the step {label} is not drawn under the topic band: {timeline}"
        );
        assert!(
            timeline.contains(&duration),
            "the timeline does not name the step's duration {duration}: {timeline}"
        );
    }
}

#[then("the report names no change that does not exist yet")]
fn report_names_no_change_that_does_not_exist_yet(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let topic = model["scope"]["id"].clone();
    for entry in model["entries"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no entries: {model:#?}"))
    {
        assert_eq!(
            entry["scope"]["kind"],
            json!("ideation"),
            "the report carries an entry outside the topic: {entry}"
        );
        assert_eq!(
            entry["scope"]["id"], topic,
            "the report carries an entry outside the topic: {entry}"
        );
    }
    for band in model["bands"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no bands: {model:#?}"))
    {
        assert_ne!(
            band["kind"],
            json!("change"),
            "the report invents a change band: {band}"
        );
        assert_ne!(
            band["scope"]["kind"],
            json!("change"),
            "the report invents a change scope: {band}"
        );
    }
    assert!(
        !document.contains("Change:"),
        "the report names a change that does not exist: {document}"
    );

    // The claim's other half: the fixture project really holds no change, so a
    // change band could only have been invented.
    support::assert_no_active_change(
        world
            .env
            .as_ref()
            .expect("a project root is required first"),
    );
}

/// The spans the written report draws, in document order: each one's entry id
/// and its recorded end timestamp (as milliseconds), read from the markup the
/// screen renders.
fn rendered_spans(document: &str) -> Vec<(u64, i64)> {
    timeline_region(document)
        .split("<g class=\"span\"")
        .skip(1)
        .map(|span| {
            let id = span
                .split("data-entry-id=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .and_then(|value| value.parse().ok())
                .unwrap_or_else(|| panic!("a drawn span carries no entry id: {span}"));
            let at = span
                .split("data-at=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .and_then(givn::history::parse_rfc3339_ms)
                .unwrap_or_else(|| panic!("a drawn span carries no readable `at`: {span}"));
            (id, at)
        })
        .collect()
}

/// The axis ticks the written report draws, in document order, as recorded
/// instants in milliseconds.
fn rendered_ticks(document: &str) -> Vec<i64> {
    timeline_region(document)
        .split("<text class=\"tick\"")
        .skip(1)
        .map(|tick| {
            tick.split("data-at-ms=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .and_then(|value| value.parse().ok())
                .unwrap_or_else(|| panic!("a drawn tick carries no instant: {tick}"))
        })
        .collect()
}

#[then("the report orders the entries by their recorded end time")]
fn report_orders_the_entries_by_their_recorded_end_time(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let entries = seeded(world);

    // The fixture's own claim: the store's line order is not the record's time
    // order, so a report that reused the store's order (or the ids, which are
    // minted in that order) cannot pass.
    let file_order: Vec<u64> = entries.iter().map(|entry| entry.id).collect();
    let mut by_time: Vec<(i64, u64)> = entries
        .iter()
        .map(|entry| {
            let at = givn::history::parse_rfc3339_ms(&entry.at)
                .unwrap_or_else(|| panic!("the fixture's `at` does not parse: {}", entry.at));
            (at, entry.id)
        })
        .collect();
    by_time.sort();
    let time_order: Vec<u64> = by_time.iter().map(|(_, id)| *id).collect();
    assert_ne!(
        file_order, time_order,
        "the fixture's file order must contradict its time order"
    );

    let ordered: Vec<u64> = model["entries"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no entries: {model:#?}"))
        .iter()
        .map(|entry| {
            entry["id"]
                .as_u64()
                .unwrap_or_else(|| panic!("an entry carries no id: {entry}"))
        })
        .collect();
    assert_eq!(
        ordered, time_order,
        "the model's spans are not in recorded-time order: {model:#?}"
    );

    // The rendered timeline agrees: its spans are drawn in the model's order,
    // and their recorded times never run backwards.
    let spans = rendered_spans(&document);
    let drawn: Vec<u64> = spans.iter().map(|(id, _)| *id).collect();
    assert_eq!(
        drawn, time_order,
        "the drawn spans are not in recorded-time order: {document}"
    );
    assert!(
        spans.windows(2).all(|pair| pair[0].1 <= pair[1].1),
        "the drawn spans' recorded times are not ascending: {document}"
    );

    // The rendered axis agrees too: its ticks run forwards in time.
    let ticks = rendered_ticks(&document);
    assert!(
        !ticks.is_empty(),
        "the report draws no axis ticks: {document}"
    );
    assert!(
        ticks.windows(2).all(|pair| pair[0] < pair[1]),
        "the drawn axis ticks are not ascending in time: {ticks:?}"
    );
}

#[given(regex = r#"^a change "([^"]+)" whose record holds many short subagents$"#)]
fn change_with_many_short_subagents(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        let turn = history.span(&scope, "turn", "turn", FAN_OUT_TURN.0, FAN_OUT_TURN.1);
        history.label(turn, FAN_OUT_TURN_LABEL);
        for (index, (identity, agent, at, duration)) in FAN_OUT_SUBAGENTS.into_iter().enumerate() {
            let subagent = match duration {
                Some(duration_ms) => history.span(&scope, "subagent", identity, at, duration_ms),
                None => history.point(&scope, "subagent", identity, at),
            };
            history.label(subagent, &format!("{agent} subagent {index}"));
            history.data(subagent, json!({ "agent": agent, "subagent": identity }));
            history.nested_in(subagent, turn);
            assert_eq!(
                subagent,
                turn + 1 + index,
                "the fixture nests every subagent in its spawning turn"
            );
        }
        history.seed(env)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[then("the report names the number of subagents and their combined duration")]
fn report_names_the_number_of_subagents_and_their_combined_duration(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let subagents: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.kind == "subagent")
        .collect();
    assert!(
        subagents.len() >= 3,
        "the fixture must hold a fan-out of several subagents: {subagents:#?}"
    );
    let combined: u64 = subagents.iter().filter_map(|entry| entry.duration_ms).sum();
    let unmeasured = subagents
        .iter()
        .filter(|entry| entry.duration_ms.is_none())
        .count();
    assert_eq!(
        unmeasured, 1,
        "the fixture holds one subagent the record does not time"
    );

    // The fan-out is the recorded spans: every subagent is a span the record
    // nests in its spawning turn, and no subagent's label names that turn (nor
    // does the turn's label name a subagent), so a report that inferred the
    // spawn relationship from label text would find none of them.
    let turn = seeded(world)
        .iter()
        .find(|entry| entry.kind == "turn")
        .expect("the fixture nests its subagents in a turn");
    for subagent in &subagents {
        assert_eq!(
            subagent.parent,
            Some(turn.id),
            "a subagent is not nested in the recorded turn: {subagent:#?}"
        );
        let label = subagent.label.as_deref().unwrap_or_default();
        assert!(
            !label.contains(FAN_OUT_TURN_LABEL),
            "a subagent's label names its spawning turn: {label}"
        );
    }

    assert_eq!(
        model["summary"]["subagents"],
        json!(subagents.len()),
        "the model does not count the recorded subagents: {model:#?}"
    );
    assert_eq!(
        model["summary"]["subagent_ms"],
        json!(combined),
        "the model's combined subagent duration is not the durations the record measured: {model:#?}"
    );
    assert_eq!(
        model["summary"]["unmeasured_subagents"],
        json!(unmeasured),
        "the model does not state the subagents the record does not time: {model:#?}"
    );
    assert_ne!(
        combined, FAN_OUT_TURN.1,
        "the fixture must make the subagents' combined duration and the turn's distinguishable"
    );

    let rendered = tile(&document, "subagents");
    assert!(
        rendered.contains(&format!("{} subagents", subagents.len())),
        "the subagents tile does not name the fan-out's size: {rendered}"
    );
    assert!(
        rendered.contains(&format_duration(combined)),
        "the subagents tile does not name the subagents' combined duration: {rendered}"
    );
    assert!(
        !rendered.contains(&format_duration(FAN_OUT_TURN.1)),
        "the subagents tile shows the spawning turn's duration instead of the subagents': {rendered}"
    );
    assert!(
        rendered.contains("not measured"),
        "the untimed subagent is not stated as not measured: {rendered}"
    );
}

#[then("the report references no remote resource")]
fn report_references_no_remote_resource(world: &mut GivnWorld) {
    let document = report_text(world);

    // The scan discriminates: a document that reaches outside its own file is
    // detected by the same function the assertion below runs, so a clean report
    // is never confused with a scan that does not look.
    for reference in [
        r#"<script src="https://cdn.example.com/report.js"></script>"#,
        r#"<link href="//cdn.example.com/report.css" rel="stylesheet">"#,
        r#"<style>@import url("https://cdn.example.com/report.css");</style>"#,
        r#"<script>fetch("https://example.test/report.json")</script>"#,
        r#"<script>new XMLHttpRequest()</script>"#,
    ] {
        assert!(
            remote_resource_reference(reference).is_some(),
            "the scan does not detect a remote reference: {reference}"
        );
    }
    assert_eq!(
        remote_resource_reference(&document),
        None,
        "the report references a resource outside its own file: {document}"
    );

    // One self-contained file: the styles and the model are inline, and the
    // only scripts are the model payload element plus the asset's own inline
    // interaction layer — `design.md` §Report Generation has the inline script
    // build the axis from that model, so the document carries both and neither
    // loads anything.
    assert!(
        document.starts_with("<!doctype html>") && document.trim_end().ends_with("</html>"),
        "the report is not one complete document: {document}"
    );
    assert!(
        document.contains("<style>"),
        "the report's styles are not inline: {document}"
    );
    assert_eq!(
        document.matches("<script id=\"report-model\"").count(),
        1,
        "the report does not carry exactly one inline model payload element: {document}"
    );
    assert!(
        !document.contains("<script src") && !document.contains("src=\"http"),
        "the report loads a script from outside its own file: {document}"
    );

    // The command's half of the claim: it wrote the file and printed its path,
    // with no service to reach — the fixture project root holds no server, no
    // port, and no external service, and the generation runs as a subprocess in
    // it (the Local Runnability statement).
    let id = world
        .last_change_id
        .clone()
        .expect("the scenario names a change");
    let (printed, root) = {
        let env = project_root(world);
        let out = common::run_givn(env, &["timeline", "--change", &id, "--out", "report.html"]);
        (out, env.root().to_path_buf())
    };
    assert!(
        printed.status.success(),
        "the report generation failed: {}",
        String::from_utf8_lossy(&printed.stderr)
    );
    let path = world
        .development_report
        .as_ref()
        .and_then(|state| state.report_path.clone())
        .expect("the development report has not been generated yet");
    let printed_path = String::from_utf8_lossy(&printed.stdout).trim().to_string();
    assert!(
        !printed_path.is_empty(),
        "the command printed no path for the report it wrote"
    );
    assert_eq!(
        root.join(&printed_path)
            .canonicalize()
            .unwrap_or_else(|error| panic!("resolve the printed path {printed_path}: {error}")),
        path.canonicalize()
            .unwrap_or_else(|error| panic!("resolve the report {}: {error}", path.display())),
        "the command does not print the path of the report it wrote"
    );
    assert_eq!(
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read the report {}: {error}", path.display())),
        document,
        "the file the command wrote is not the report the browser loads"
    );
}

/// The empty state the written report renders: the region between its own
/// section start and close, so an assertion reads the state's own markup rather
/// than a phrase that happens to appear anywhere in the document.
fn empty_state_region(document: &str) -> String {
    let start = document
        .find("<section class=\"empty-state\"")
        .unwrap_or_else(|| panic!("the report renders no empty state: {document}"));
    let region = &document[start..];
    let end = region
        .find("</section>")
        .expect("the empty state section is never closed");
    region[..end].to_string()
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

#[then(regex = r#"^the report shows the empty state for "([^"]+)"$"#)]
fn report_shows_the_empty_state_for(world: &mut GivnWorld, id: String) {
    let document = report_text(world);
    let model = report_model(&document);

    // The fixture's own claim: the requested scope recorded nothing, so the
    // empty state is the report of a really empty record, not of a filtered one.
    let scope = Scope::Change { id: id.clone() };
    let recorded: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.scope == scope)
        .collect();
    assert!(
        recorded.is_empty(),
        "the fixture's scope recorded activity: {recorded:#?}"
    );
    assert_eq!(
        model["empty"],
        json!(true),
        "the model does not mark the scope empty: {model:#?}"
    );
    assert_eq!(
        model["summary"]["entries"],
        json!(0),
        "the empty report still counts recorded entries: {model:#?}"
    );

    // The rendered state: a heading, one sentence naming the scope, and a way
    // back to the report root.
    let empty = empty_state_region(&document);
    assert!(
        empty.contains("<h2"),
        "the empty state carries no heading: {empty}"
    );
    assert!(
        empty.contains("No recorded activity"),
        "the empty state's heading does not name the state: {empty}"
    );
    assert!(
        empty.contains(&id),
        "the empty state does not name the scope it is about: {empty}"
    );
    assert!(
        empty.contains("Back to the report root") && empty.contains("href=\"#report-title\""),
        "the empty state offers no way back to the report root: {empty}"
    );
    assert!(
        document.contains("<h1 id=\"report-title\">"),
        "the report has no root heading to return to: {document}"
    );
}

#[then("the report draws no workflow phase for it")]
fn report_draws_no_workflow_phase_for_it(world: &mut GivnWorld) {
    let document = report_text(world);
    // The empty state is the whole report root: no phase card, no axis, no
    // lane, and no KPI strip is drawn for a scope that recorded nothing.
    for marker in [
        "<svg",
        "class=\"timeline-section\"",
        "class=\"axis\"",
        "class=\"tick\"",
        "class=\"band\"",
        "class=\"span\"",
        "class=\"workflow-stack\"",
        "data-phase=",
        "data-status=",
        "data-lane=",
        "data-tile=",
    ] {
        assert!(
            !document.contains(marker),
            "the empty report draws {marker}: {document}"
        );
    }
    // The empty state names no workflow phase: no phase line and no phase name
    // reaches the rendered state, and the state itself is the whole root.
    let empty = empty_state_region(&document);
    assert!(
        !empty.contains("workflow-phase") && !empty.contains("phase "),
        "the empty state names a workflow phase: {empty}"
    );
    // The document is still one complete report, its root heading intact.
    assert!(
        document.starts_with("<!doctype html>") && document.trim_end().ends_with("</html>"),
        "the report is not one complete document: {document}"
    );
    // The model agrees: the empty report carries no rows to draw and its
    // workflow scaffold draws no phase.
    let model = report_model(&document);
    assert_eq!(
        model["bands"].as_array().map(Vec::len),
        Some(0),
        "the empty report still carries bands: {model:#?}"
    );
    assert!(
        !document.contains(&format!("phase {}", "propose")),
        "the empty report prints a phase line: {document}"
    );
}

/// The report's nine workflow phases, in workflow order: the names the report
/// may carry, and the only ones.
const WORKFLOW_PHASES: [&str; 9] = [
    "propose",
    "specs",
    "design",
    "arc42",
    "design-review",
    "tasks",
    "implement",
    "review",
    "archive",
];

#[given(regex = r#"^a change "([^"]+)" whose record holds the workflow phases up to "([^"]+)"$"#)]
fn change_whose_record_holds_workflow_phases_up_to(
    world: &mut GivnWorld,
    id: String,
    stage: String,
) {
    {
        let env = project_root(world);
        // The fixture's project carries no arc42 artifact: the scaffold's
        // seven-step workflow is the one its artifacts derive.
        run_fixture_command(env, &["addons", "disable", "arc42"]);
        support::seed_report_history(env, &id, support::stage_transition(&stage));
    }
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^the change's artifacts make its derived state "([^"]+)" with the next required artifact "([^"]+)"$"#
)]
fn change_artifacts_make_its_derived_state(world: &mut GivnWorld, stage: String, next: String) {
    let id = world
        .last_change_id
        .clone()
        .expect("the change Given names a change");
    let env = project_root(world);
    support::write_change_artifacts_through_tasks(env, &id);
    // The fixture's own claim: the production state derives the scenario's
    // stage and next artifact from the artifact files, so the report reads a
    // real change state rather than a label a step supplied.
    let out = common::run_givn(env, &["status", "--change", &id, "--json"]);
    assert!(
        out.status.success(),
        "givn status failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let status: Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|error| panic!("the status output is not JSON: {error}"));
    assert_eq!(
        status["stage"],
        json!(stage),
        "the fixture's artifacts do not derive the claimed stage: {status:#?}"
    );
    assert_eq!(
        status["next_required"]["id"],
        json!(next),
        "the fixture's artifacts do not derive the claimed next artifact: {status:#?}"
    );
}

/// The written report's model, with its workflow stack.
fn report_workflow(model: &Value) -> &[Value] {
    model["workflow"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_else(|| panic!("the report carries no workflow stack: {model:#?}"))
}

/// The workflow phase named `stage` as the written report's model carries it.
fn report_phase<'a>(model: &'a Value, stage: &str) -> &'a Value {
    report_workflow(model)
        .iter()
        .find(|phase| phase["stage"] == json!(stage))
        .unwrap_or_else(|| panic!("the report names no phase {stage}: {model:#?}"))
}

#[then(regex = r#"^the report names the current stage "([^"]+)"$"#)]
fn report_names_the_current_stage(world: &mut GivnWorld, stage: String) {
    let document = report_text(world);
    let model = report_model(&document);
    let state = &model["current_state"];
    assert_eq!(
        state["stage_label"],
        json!(stage),
        "the report's current state does not name the stage: {model:#?}"
    );
    assert!(
        document.contains(&format!("data-stage-label=\"{stage}\"")),
        "the text view does not print the stage label {stage}: {document}"
    );
    // The header names the stage and the current phase as separate words: the
    // stage label is not the active phase's name.
    let phase = state["phase"]
        .as_str()
        .unwrap_or_else(|| panic!("the current state names no phase: {state:#?}"));
    assert!(
        document.contains(&format!("<span class=\"state-phase\">{phase}</span>")),
        "the header does not name the current phase {phase} separately: {document}"
    );
}

#[then(regex = r#"^the report marks the phases before "([^"]+)" done$"#)]
fn report_marks_the_phases_before_done(world: &mut GivnWorld, stage: String) {
    let document = report_text(world);
    let model = report_model(&document);
    let workflow = report_workflow(&model);
    let current = workflow
        .iter()
        .position(|phase| phase["stage"] == json!(stage))
        .unwrap_or_else(|| panic!("the report names no phase {stage}: {model:#?}"));
    assert!(current > 0, "the phase {stage} has no phase before it");
    for phase in &workflow[..current] {
        let name = phase["stage"].as_str().expect("a phase name");
        assert_eq!(
            phase["status"],
            json!("done"),
            "the report does not mark {name} done: {model:#?}"
        );
        assert!(
            document.contains(&format!("data-phase=\"{name}\" data-status=\"done\"")),
            "the text view does not mark {name} done: {document}"
        );
    }
}

#[then(regex = r#"^the report marks "([^"]+)" active$"#)]
fn report_marks_phase_active(world: &mut GivnWorld, stage: String) {
    let document = report_text(world);
    let model = report_model(&document);
    assert_eq!(
        report_phase(&model, &stage)["status"],
        json!("active"),
        "the report does not mark {stage} active: {model:#?}"
    );
    assert!(
        document.contains(&format!("data-phase=\"{stage}\" data-status=\"active\"")),
        "the text view does not mark {stage} active: {document}"
    );
}

#[then(regex = r#"^the report marks "([^"]+)" queued$"#)]
fn report_marks_phase_queued(world: &mut GivnWorld, stage: String) {
    let document = report_text(world);
    let model = report_model(&document);
    assert_eq!(
        report_phase(&model, &stage)["status"],
        json!("queued"),
        "the report does not mark {stage} queued: {model:#?}"
    );
    assert!(
        document.contains(&format!("data-phase=\"{stage}\" data-status=\"queued\"")),
        "the text view does not mark {stage} queued: {document}"
    );
}

#[then("the report names no phase the workflow does not have")]
fn report_names_no_phase_the_workflow_does_not_have(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let workflow = report_workflow(&model);
    let stages: Vec<&str> = workflow
        .iter()
        .map(|phase| {
            phase["stage"]
                .as_str()
                .unwrap_or_else(|| panic!("a workflow phase carries no name: {phase:#?}"))
        })
        .collect();
    assert_eq!(
        stages, WORKFLOW_PHASES,
        "the report's workflow is not the nine phases in workflow order: {model:#?}"
    );
    let printed: Vec<&str> = document
        .split("data-phase=\"")
        .skip(1)
        .filter_map(|rest| rest.split('"').next())
        .collect();
    for name in &printed {
        assert!(
            WORKFLOW_PHASES.contains(name),
            "the report invents the phase {name}: {document}"
        );
    }
    assert_eq!(
        printed, WORKFLOW_PHASES,
        "the text view does not name exactly the nine phases: {document}"
    );
}

/// The two recorded sessions of the parallel-lane fixture: the spawning
/// session, the subagent's own child session, and the session id its span names
/// without any recorded entries — the span no tool call is attributable to.
const SPAWN_SESSION: &str = "spawn-0a1b2c3d4e5f";
const CHILD_SESSION: &str = "child-9f8e7d6c5b4a";
const GHOST_SESSION: &str = "ghost-111213141516";

/// The subagent span's own child session (`data.subagent`).
fn subagent_child(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("subagent")?.as_str()
}

/// The subagent span's agent definition (`data.agent`).
fn subagent_agent(entry: &HistoryEntry) -> Option<&str> {
    entry.data.as_ref()?.get("agent")?.as_str()
}

#[given(regex = r#"^a change "([^"]+)" whose record holds one phase with two sessions$"#)]
fn change_whose_record_holds_one_phase_with_two_sessions(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        let transition = history.point(&scope, "phase", "tasks.md", "2026-10-16T13:00:00.000Z");
        history.label(transition, "tasks.md");
        let spawn_session = history.point(
            &scope,
            "session",
            "opencode-session",
            "2026-10-16T13:05:00.000Z",
        );
        history.session(spawn_session, SPAWN_SESSION);
        history.data(spawn_session, json!({ "title": "Opencode main" }));
        let child_session = history.point(
            &scope,
            "session",
            "subagent-session",
            "2026-10-16T13:06:00.000Z",
        );
        history.session(child_session, CHILD_SESSION);
        history.data(child_session, json!({ "title": "Subagent transcript" }));
        let child_turn = history.span(&scope, "turn", "turn", "2026-10-16T13:07:00.000Z", 30_000);
        history.session(child_turn, CHILD_SESSION);
        let child_span = history.point(
            &scope,
            "subagent",
            "Architect.Scout",
            "2026-10-16T13:08:00.000Z",
        );
        history.session(child_span, CHILD_SESSION);
        history.data(
            child_span,
            json!({ "subagent": CHILD_SESSION, "agent": "Architect.Scout" }),
        );
        let tool = history.span(&scope, "tool", "Bash", "2026-10-16T13:09:00.000Z", 12_000);
        history.session(tool, CHILD_SESSION);
        history.nested_in(tool, child_span);
        let spawn_turn = history.span(&scope, "turn", "turn", "2026-10-16T13:10:00.000Z", 45_000);
        history.session(spawn_turn, SPAWN_SESSION);
        let spawn_span = history.point(&scope, "subagent", "Reviewer", "2026-10-16T13:11:00.000Z");
        history.session(spawn_span, SPAWN_SESSION);
        history.data(
            spawn_span,
            json!({ "subagent": GHOST_SESSION, "agent": "Reviewer" }),
        );
        history.span(&scope, "step", "verify", "2026-10-16T13:12:00.000Z", 20_000);
        history.seed(env)
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[given(regex = r#"^one session is a subagent's own recorded session named "([^"]+)"$"#)]
fn one_session_is_a_subagents_own_recorded_session(world: &mut GivnWorld, name: String) {
    let entries = seeded(world);
    let span = entries
        .iter()
        .find(|entry| {
            entry.kind == "subagent"
                && subagent_agent(entry) == Some(name.as_str())
                && subagent_child(entry) == entry.session.as_deref()
        })
        .unwrap_or_else(|| {
            panic!("the record holds no subagent session named {name}: {entries:#?}")
        });
    assert!(
        entries
            .iter()
            .any(|entry| entry.session == span.session && entry.id != span.id),
        "the subagent's session records no entries of its own: {entries:#?}"
    );
}

#[given("one session holds a subagent span the record places in the spawning session")]
fn one_session_holds_a_span_in_the_spawning_session(world: &mut GivnWorld) {
    let entries = seeded(world);
    let span = entries
        .iter()
        .find(|entry| {
            entry.kind == "subagent"
                && entry
                    .session
                    .as_deref()
                    .is_some_and(|session| subagent_child(entry) != Some(session))
        })
        .unwrap_or_else(|| {
            panic!("the record holds no subagent span in a spawning session: {entries:#?}")
        });
    assert!(
        entries
            .iter()
            .any(|entry| entry.session == span.session && entry.kind != "subagent"),
        "the spawning session records no entries of its own: {entries:#?}"
    );
}

/// The written report's phase named `stage`, its lanes' session keys included.
fn workflow_phase_lanes<'a>(model: &'a Value, stage: &str) -> &'a Vec<Value> {
    report_phase(model, stage)["lanes"]
        .as_array()
        .unwrap_or_else(|| panic!("the report's {stage} phase carries no lanes: {model:#?}"))
}

#[then("the report draws one lane per session beside each other in that phase")]
fn report_draws_one_lane_per_session_in_that_phase(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let lanes = workflow_phase_lanes(&model, "implement");
    let entries = seeded(world);

    // The fixture's own claim: the record names two sessions in this phase, one
    // of them the subagent's own, plus an entry the record names no session for.
    let mut expected: Vec<Option<&str>> = Vec::new();
    for entry in entries {
        let key = entry.session.as_deref();
        if !expected.contains(&key) {
            expected.push(key);
        }
    }
    assert_eq!(
        expected.iter().filter(|key| key.is_some()).count(),
        2,
        "the fixture must record two sessions: {entries:#?}"
    );
    assert!(
        expected.contains(&None),
        "the fixture records an entry with no session: {entries:#?}"
    );

    let drawn: Vec<Option<&str>> = lanes.iter().map(|lane| lane["session"].as_str()).collect();
    assert_eq!(
        drawn.len(),
        expected.len(),
        "the report draws {} lanes for {} recorded session keys: {model:#?}",
        drawn.len(),
        expected.len()
    );
    for key in &expected {
        assert!(
            drawn.contains(key),
            "the report draws no lane for the session key {key:?}: {model:#?}"
        );
    }
    // The lanes sit beside each other in one phase: every entry is a member of
    // exactly one lane, and no lane is empty.
    let mut members: Vec<u64> = Vec::new();
    for lane in lanes {
        let ids: Vec<u64> = lane["entries"]
            .as_array()
            .unwrap_or_else(|| panic!("a lane carries no entries: {lane:#?}"))
            .iter()
            .map(|id| id.as_u64().expect("a lane member id"))
            .collect();
        assert!(!ids.is_empty(), "the report draws an empty lane: {lane:#?}");
        let name = lane["name"].as_str().expect("a lane name");
        assert!(
            document.contains(name),
            "the text view does not print the lane {name}: {document}"
        );
        members.extend(ids);
    }
    let mut unique = members.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(
        unique.len(),
        members.len(),
        "an entry appears in more than one lane: {model:#?}"
    );
}

#[then(regex = r#"^the lane named "([^"]+)" carries that subagent's own entries$"#)]
fn lane_named_carries_the_subagents_own_entries(world: &mut GivnWorld, name: String) {
    let document = report_text(world);
    let model = report_model(&document);
    let lanes = workflow_phase_lanes(&model, "implement");
    let entries = seeded(world);
    let span = entries
        .iter()
        .find(|entry| {
            entry.kind == "subagent"
                && subagent_agent(entry) == Some(name.as_str())
                && subagent_child(entry) == entry.session.as_deref()
        })
        .unwrap_or_else(|| panic!("the record holds no subagent session named {name}"));
    let lane = lanes
        .iter()
        .find(|lane| lane["name"] == json!(name))
        .unwrap_or_else(|| panic!("the report draws no lane named {name}: {model:#?}"));
    assert_eq!(
        lane["session"].as_str(),
        span.session.as_deref(),
        "the lane named {name} is not the subagent's own session: {model:#?}"
    );
    let ids: Vec<u64> = lane["entries"]
        .as_array()
        .expect("the lane's members")
        .iter()
        .map(|id| id.as_u64().expect("a lane member id"))
        .collect();
    assert!(
        ids.contains(&span.id),
        "the lane named {name} does not carry the subagent's own span: {model:#?}"
    );
    for id in ids {
        let entry = entries
            .iter()
            .find(|entry| entry.id == id)
            .unwrap_or_else(|| panic!("the lane holds an entry outside the record: {id}"));
        assert_eq!(
            entry.session.as_deref(),
            span.session.as_deref(),
            "the lane named {name} carries an entry of another session: {entry:#?}"
        );
    }
}

#[then("the spawning session's lane keeps its own name and carries the subagent span")]
fn spawning_sessions_lane_keeps_its_own_name(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let lanes = workflow_phase_lanes(&model, "implement");
    let entries = seeded(world);
    let span = entries
        .iter()
        .find(|entry| {
            entry.kind == "subagent"
                && entry
                    .session
                    .as_deref()
                    .is_some_and(|session| subagent_child(entry) != Some(session))
        })
        .unwrap_or_else(|| panic!("the record holds no spawned subagent span: {entries:#?}"));
    let lane = lanes
        .iter()
        .find(|lane| lane["session"].as_str() == span.session.as_deref())
        .unwrap_or_else(|| panic!("the spawning session has no lane: {model:#?}"));
    let title = entries
        .iter()
        .filter(|entry| entry.kind == "session")
        .filter(|entry| entry.session == span.session)
        .find_map(|entry| entry.data.as_ref()?.get("title")?.as_str())
        .expect("the spawning session records a title");
    assert_eq!(
        lane["name"].as_str(),
        Some(title),
        "the spawning session's lane was renamed by the span it contains: {model:#?}"
    );
    let ids: Vec<u64> = lane["entries"]
        .as_array()
        .expect("the lane's members")
        .iter()
        .map(|id| id.as_u64().expect("a lane member id"))
        .collect();
    assert!(
        ids.contains(&span.id),
        "the spawning session's lane does not carry the subagent span: {model:#?}"
    );
    assert!(
        !lanes
            .iter()
            .any(|lane| { subagent_agent(span).is_some_and(|agent| lane["name"] == json!(agent)) }),
        "the report created a subagent lane for the spawning session's span: {model:#?}"
    );
}

#[then("an entry whose session the record does not name stays in the unattributed lane")]
fn unnamed_session_entry_stays_unattributed(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let lanes = workflow_phase_lanes(&model, "implement");
    let entry = seeded(world)
        .iter()
        .find(|entry| entry.session.is_none())
        .unwrap_or_else(|| panic!("the fixture records an entry with no session"));
    let lane = lanes
        .iter()
        .find(|lane| lane["session"].is_null())
        .unwrap_or_else(|| panic!("the report draws no unattributed lane: {model:#?}"));
    assert_eq!(
        lane["name"].as_str(),
        Some("unattributed"),
        "the unattributed lane carries another name: {model:#?}"
    );
    let ids: Vec<u64> = lane["entries"]
        .as_array()
        .expect("the lane's members")
        .iter()
        .map(|id| id.as_u64().expect("a lane member id"))
        .collect();
    assert!(
        ids.contains(&entry.id),
        "the entry with no session is not in the unattributed lane: {model:#?}"
    );
}

#[then("the report never assigns an entry to a subagent the record does not name")]
fn report_never_assigns_an_entry_to_an_unrecorded_subagent(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let lanes = workflow_phase_lanes(&model, "implement");
    let entries = seeded(world);
    for lane in lanes {
        let session = lane["session"].as_str();
        for id in lane["entries"].as_array().expect("the lane's members") {
            let id = id.as_u64().expect("a lane member id");
            let entry = entries
                .iter()
                .find(|entry| entry.id == id)
                .unwrap_or_else(|| panic!("a lane holds an entry outside the record: {id}"));
            assert_eq!(
                entry.session.as_deref(),
                session,
                "the report assigns the entry {id} to a session the record does not name: {entry:#?}"
            );
        }
    }
    // No lane exists for a session the record never delivered: the ghost child
    // session its span names stays a statement, never a lane.
    assert!(
        !lanes
            .iter()
            .any(|lane| lane["session"].as_str() == Some(GHOST_SESSION)),
        "the report invented a lane for a session it does not carry: {model:#?}"
    );
    assert!(
        document.contains("the record names no per-subagent tool identity"),
        "the text view does not state the attribution limit: {document}"
    );
}

#[then("the report states when no tool call was attributable to a subagent span")]
fn report_states_when_no_tool_call_was_attributable(world: &mut GivnWorld) {
    let document = report_text(world);
    let entries = seeded(world);
    let ghost = entries
        .iter()
        .find(|entry| {
            entry.kind == "subagent"
                && subagent_child(entry).is_some_and(|child| {
                    !entries.iter().any(|candidate| {
                        candidate.kind == "tool"
                            && (candidate.parent == Some(entry.id)
                                || candidate.session.as_deref() == Some(child))
                    })
                })
        })
        .unwrap_or_else(|| {
            panic!("the fixture holds no subagent span without tool calls: {entries:#?}")
        });
    assert!(
        document.contains("no tool call was attributable"),
        "the report does not state the attribution limit: {document}"
    );
    assert!(
        document.contains(&format!(
            "class=\"attribution-limit\" data-subagent=\"{}\"",
            ghost.name
        )),
        "the report does not name the span no tool call was attributable to: {document}"
    );
    // The span whose child session flushed a tool call carries no such
    // statement: the limit is stated where it holds, not everywhere.
    let attributable = entries
        .iter()
        .find(|entry| entry.kind == "subagent" && subagent_child(entry) == entry.session.as_deref())
        .expect("the fixture records the subagent's own session");
    assert!(
        !document.contains(&format!(
            "class=\"attribution-limit\" data-subagent=\"{}\"",
            attributable.name
        )),
        "the report states an attribution limit for a subagent whose tool call it carries: {document}"
    );
}

#[given(regex = r#"^the change skips the "([^"]+)" artifact$"#)]
fn the_change_skips_the_artifact(world: &mut GivnWorld, artifact: String) {
    let id = world
        .last_change_id
        .clone()
        .expect("the change Given names a change");
    let env = project_root(world);
    // The fixture's project enables the arc42 artifact graph, then records the
    // artifact as intentionally not produced, so the production state derives
    // it `skipped` — never a label the step writes into the report.
    run_fixture_command(env, &["addons", "enable", "arc42"]);
    let change_dir = env.root().join("givn").join("changes").join(&id);
    std::fs::write(change_dir.join(".givn-skip"), format!("{artifact}\n"))
        .unwrap_or_else(|error| panic!("write the change's skip file: {error}"));
    let out = common::run_givn(env, &["status", "--change", &id, "--json"]);
    assert!(
        out.status.success(),
        "givn status failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let status: Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|error| panic!("the status output is not JSON: {error}"));
    let skipped = status["artifacts"]
        .as_array()
        .and_then(|artifacts| {
            artifacts
                .iter()
                .find(|candidate| candidate["id"] == json!(artifact))
        })
        .unwrap_or_else(|| panic!("the state carries no artifact {artifact}: {status:#?}"));
    assert_eq!(
        skipped["status"],
        json!("skipped"),
        "the fixture's artifact is not skipped: {status:#?}"
    );
    // Skipping the optional artifact leaves the state where it was: the
    // skipped artifact is never the next required one.
    assert_eq!(
        status["stage"],
        json!("tasks-done"),
        "skipping the artifact moved the stage: {status:#?}"
    );
    assert_eq!(
        status["next_required"]["id"],
        json!("review"),
        "skipping the artifact moved the next required artifact: {status:#?}"
    );
}

#[then(regex = r#"^the report marks "([^"]+)" skipped$"#)]
fn report_marks_phase_skipped(world: &mut GivnWorld, stage: String) {
    let document = report_text(world);
    let model = report_model(&document);
    let phase = report_phase(&model, &stage);
    assert_eq!(
        phase["status"],
        json!("skipped"),
        "the report does not mark {stage} skipped: {model:#?}"
    );
    assert!(
        document.contains(&format!("data-phase=\"{stage}\" data-status=\"skipped\"")),
        "the text view does not mark {stage} skipped: {document}"
    );
    // The skipped phase's wall time is the record's own measured span (zero
    // when it recorded nothing), never an invented duration.
    assert!(
        phase["wall_ms"].is_u64(),
        "the skipped phase carries no measured wall time: {model:#?}"
    );
}

#[then(regex = r#"^the report does not mark "([^"]+)" done$"#)]
fn report_does_not_mark_phase_done(world: &mut GivnWorld, stage: String) {
    let document = report_text(world);
    let model = report_model(&document);
    assert_ne!(
        report_phase(&model, &stage)["status"],
        json!("done"),
        "the report marks {stage} done as well as skipped: {model:#?}"
    );
    assert!(
        !document.contains(&format!("data-phase=\"{stage}\" data-status=\"done\"")),
        "the text view marks {stage} done as well as skipped: {document}"
    );
}

#[given(
    regex = r#"^an archived change "([^"]+)" whose record holds the workflow phases up to "([^"]+)"$"#
)]
fn archived_change_whose_record_holds_workflow_phases(
    world: &mut GivnWorld,
    id: String,
    stage: String,
) {
    let seeded = {
        let env = project_root(world);
        let seeded = support::seed_archived_change(env, &id, support::stage_transition(&stage));
        // The fixture's own claim: the change is archived, never live.
        assert!(
            env.root().join("givn").join("archive").join(&id).is_dir(),
            "the fixture's archived change directory does not exist"
        );
        assert!(
            !env.root().join("givn").join("changes").join(&id).exists(),
            "the fixture still holds a live change directory"
        );
        seeded
    };
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[then("the report names no live change state")]
fn report_names_no_live_change_state(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    assert!(
        model["current_state"].is_null(),
        "the archived report carries a live current state: {model:#?}"
    );
    assert!(
        document.contains("no live change state"),
        "the header does not state the absent live state: {document}"
    );
    for marker in ["data-stage-label=", "state-phase", "state-next="] {
        assert!(
            !document.contains(marker),
            "the archived header names the live state's {marker}: {document}"
        );
    }
}

#[then("the report marks every workflow phase done")]
fn report_marks_every_workflow_phase_done(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let workflow = report_workflow(&model);
    assert_eq!(
        workflow.len(),
        WORKFLOW_PHASES.len(),
        "the archived report does not carry the nine phases: {model:#?}"
    );
    for phase in workflow {
        let stage = phase["stage"].as_str().expect("a phase name");
        assert_eq!(
            phase["status"],
            json!("done"),
            "the archived report does not mark {stage} done: {model:#?}"
        );
        assert!(
            document.contains(&format!("data-phase=\"{stage}\" data-status=\"done\"")),
            "the text view does not mark {stage} done: {document}"
        );
        // A phase with no recorded entries is still done and records zero
        // minutes — no status is invented from the record.
        if phase["lanes"].as_array().is_some_and(Vec::is_empty) {
            assert_eq!(
                phase["wall_ms"],
                json!(0),
                "the empty archived phase {stage} invents a duration: {model:#?}"
            );
        }
    }
}

#[then("the report names no next required artifact")]
fn report_names_no_next_required_artifact(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    assert!(
        model["current_state"].is_null(),
        "the archived report carries a next required artifact: {model:#?}"
    );
    assert!(
        !document.contains("state-next=") && !document.contains("next artifact"),
        "the archived header names a next required artifact: {document}"
    );
}

#[given(regex = r#"^a change "([^"]+)" whose record holds no artifact transition$"#)]
fn change_whose_record_holds_no_artifact_transition(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        let step = history.span(
            &scope,
            "step",
            "verify",
            "2026-10-16T09:10:00.000Z",
            120_000,
        );
        history.label(step, "cargo test --locked");
        let tool = history.span(&scope, "tool", "Bash", "2026-10-16T09:20:00.000Z", 5_000);
        history.label(tool, "git status");
        history.seed(env)
    };
    // The fixture's own claim: the record carries work and no artifact
    // transition at all.
    assert_eq!(seeded.len(), 2, "the fixture records its own work");
    assert!(
        seeded.iter().all(|entry| entry.kind != "phase"),
        "the fixture's record carries an artifact transition: {seeded:#?}"
    );
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

#[then("the report groups the recorded work under no workflow phase")]
fn report_groups_the_recorded_work_under_no_workflow_phase(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let recorded = seeded(world);
    let unstaged = &model["unstaged"];
    assert!(
        !unstaged.is_null(),
        "the report carries no no-stage group: {model:#?}"
    );
    assert!(
        unstaged["stage"].is_null(),
        "the no-stage group carries a phase name: {model:#?}"
    );
    assert!(
        unstaged["status"].is_null(),
        "the no-stage group carries a status: {model:#?}"
    );
    let mut members: Vec<u64> = unstaged["lanes"]
        .as_array()
        .unwrap_or_else(|| panic!("the no-stage group carries no lanes: {model:#?}"))
        .iter()
        .flat_map(|lane| {
            lane["entries"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|id| id.as_u64().expect("a lane member id"))
        })
        .collect();
    members.sort_unstable();
    let mut expected: Vec<u64> = recorded.iter().map(|entry| entry.id).collect();
    expected.sort_unstable();
    assert_eq!(
        members, expected,
        "the no-stage group does not hold exactly the recorded work: {model:#?}"
    );
    // The group is rendered below the phase stack, with its entry count.
    assert!(
        document.contains("class=\"unstaged-group\"")
            && document.contains(&format!("data-entries=\"{}\"", expected.len())),
        "the text view does not print the no-stage group: {document}"
    );
    // No workflow phase draws the recorded work: the groups never overlap.
    for phase in report_workflow(&model) {
        for lane in phase["lanes"].as_array().into_iter().flatten() {
            for id in lane["entries"].as_array().into_iter().flatten() {
                assert!(
                    !expected.contains(&id.as_u64().expect("a lane member id")),
                    "the report places the recorded work under the phase {:?}: {model:#?}",
                    phase["stage"]
                );
            }
        }
    }
}

#[then("the report states that the record carries no stage transition")]
fn report_states_that_the_record_carries_no_stage_transition(world: &mut GivnWorld) {
    let document = report_text(world);
    assert!(
        document.contains(
            "entries carry no recorded stage transition; the record assigns them to no workflow phase."
        ),
        "the report does not state the gap: {document}"
    );
}

#[then("the report names no phase for that recorded work")]
fn report_names_no_phase_for_that_recorded_work(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let recorded = seeded(world);
    for entry in recorded {
        let assignment = model["stages"]
            .as_array()
            .and_then(|assignments| {
                assignments
                    .iter()
                    .find(|assignment| assignment["entry"] == json!(entry.id))
            })
            .unwrap_or_else(|| {
                panic!("the entry {} has no stage assignment: {model:#?}", entry.id)
            });
        assert!(
            assignment["stage"].is_null(),
            "the report names the phase {:?} for the entry {}: {model:#?}",
            assignment["stage"],
            entry.id
        );
    }
    assert!(
        !document.contains("data-stage="),
        "the text view names a phase for the recorded work: {document}"
    );
}

#[given(regex = r#"^a change "([^"]+)" whose record holds hundreds of short agent activities$"#)]
fn change_with_hundreds_of_short_agent_activities(world: &mut GivnWorld, id: String) {
    let scope = Scope::Change { id: id.clone() };
    let seeded = {
        let env = project_root(world);
        run_fixture_command(env, &["new", &id]);
        let mut history = support::SeededHistory::new();
        for index in 0..DENSE_ACTIVITIES {
            let entry = history.span(
                &scope,
                "tool",
                "Bash",
                &dense_activity_at(index),
                DENSE_DURATION_MS,
            );
            history.label(entry, &format!("tool call {}", index + 1));
        }
        history.seed(env)
    };
    assert_eq!(
        seeded.len(),
        DENSE_ACTIVITIES,
        "the fixture seeds every one of its short activities"
    );
    state(world).seeded = seeded;
    world.last_change_id = Some(id);
}

/// The aggregate markers the written report draws, in document order, read from
/// the timeline region's own markup.
fn rendered_aggregates(document: &str) -> Vec<DrawnAggregate> {
    timeline_region(document)
        .split("<g class=\"aggregate\"")
        .skip(1)
        .map(|marker| DrawnAggregate {
            bucket_start_ms: aggregate_attribute(marker, "data-bucket-start-ms"),
            bucket_end_ms: aggregate_attribute(marker, "data-bucket-end-ms"),
            count: aggregate_attribute(marker, "data-count") as usize,
            duration_ms: marker
                .split("data-duration-ms=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .and_then(|value| value.parse().ok()),
            name: marker
                .split("aria-label=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .unwrap_or_else(|| {
                    panic!("an aggregate marker carries no accessible name: {marker}")
                })
                .to_string(),
        })
        .collect()
}

/// One numeric attribute of an aggregate marker's markup.
fn aggregate_attribute(marker: &str, attribute: &str) -> i64 {
    let marker_attribute = format!("{attribute}=\"");
    marker
        .split(&marker_attribute)
        .nth(1)
        .and_then(|value| value.split('"').next())
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| panic!("an aggregate marker carries no {attribute}: {marker}"))
}

#[then("the report collapses the short agent activities into aggregate markers")]
fn report_collapses_the_short_agent_activities_into_aggregate_markers(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let activities = seeded(world);
    assert!(
        activities.len() >= 200,
        "the fixture holds hundreds of short activities: {}",
        activities.len()
    );

    // The fixture's own claim: every activity is shorter than the time one
    // pixel of the initial zoom spends on — each is under a hundredth of the
    // window it is drawn across — so a report that drew one shape per entry
    // could not be read. The claim is derived from the seeded record, not
    // assumed.
    let times: Vec<i64> = activities
        .iter()
        .map(|entry| {
            givn::history::parse_rfc3339_ms(&entry.at)
                .unwrap_or_else(|| panic!("the fixture's `at` does not parse: {}", entry.at))
        })
        .collect();
    let window_span = times.iter().max().expect("the fixture records activities")
        - times.iter().min().expect("the fixture records activities");
    for entry in activities {
        let duration = entry
            .duration_ms
            .expect("the fixture measures every activity") as i64;
        assert!(
            (duration as f64) < window_span as f64 / 100.0,
            "the fixture must make every activity far shorter than its window: {duration} of {window_span}"
        );
    }

    // The model's markers cover exactly the recorded activities, each of them
    // once — the entries the markers stand for are the entries the record holds.
    let markers = model["aggregates"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no aggregates: {model:#?}"));
    assert!(
        !markers.is_empty(),
        "a dense record draws no aggregate markers: {model:#?}"
    );
    let mut covered: Vec<u64> = markers
        .iter()
        .flat_map(|marker| {
            marker["entries"]
                .as_array()
                .unwrap_or_else(|| panic!("a marker covers no entries: {marker}"))
                .iter()
                .map(|entry| entry.as_u64().expect("a covered entry id"))
        })
        .collect();
    let mut expected: Vec<u64> = activities.iter().map(|entry| entry.id).collect();
    covered.sort_unstable();
    expected.sort_unstable();
    assert_eq!(
        covered, expected,
        "the aggregate markers do not cover exactly the recorded activities: {model:#?}"
    );

    // No individual span is drawn for a covered entry, and the axis draws far
    // fewer shapes than the record holds entries — the aggregate replaced the
    // ticks instead of joining them.
    let drawn_ids: Vec<u64> = rendered_spans(&document)
        .iter()
        .map(|(id, _)| *id)
        .collect();
    for entry in activities {
        assert!(
            !drawn_ids.contains(&entry.id),
            "the report draws an individual span for a collapsed entry: {entry:#?}"
        );
    }
    let drawn = rendered_aggregates(&document);
    assert_eq!(
        drawn.len(),
        markers.len(),
        "the drawn markers disagree with the model's: {document}"
    );
    assert!(
        drawn.len() < activities.len() / 2,
        "the axis draws one shape per entry rather than markers: {} markers for {} entries",
        drawn.len(),
        activities.len()
    );
}

#[then("each aggregate marker names how many entries it covers and their total duration")]
fn each_aggregate_marker_names_its_coverage_and_duration(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let entries = seeded(world);
    let markers = model["aggregates"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no aggregates: {model:#?}"));
    let drawn = rendered_aggregates(&document);
    let mut covered = 0;
    for marker in markers {
        let ids: Vec<u64> = marker["entries"]
            .as_array()
            .unwrap_or_else(|| panic!("a marker covers no entries: {marker}"))
            .iter()
            .map(|entry| entry.as_u64().expect("a covered entry id"))
            .collect();
        assert!(!ids.is_empty(), "a marker covers no entry: {marker}");
        let combined: u64 = ids
            .iter()
            .map(|id| {
                entries
                    .iter()
                    .find(|entry| entry.id == *id)
                    .unwrap_or_else(|| panic!("a marker covers an entry outside the record: {id}"))
                    .duration_ms
                    .expect("the fixture measures every activity")
            })
            .sum();
        assert_eq!(
            marker["duration_ms"].as_u64(),
            Some(combined),
            "the model's marker does not carry the covered entries' combined duration: {marker}"
        );
        covered += ids.len();

        // The marker itself names both: the count and the combined duration in
        // its machine-readable attributes, and both in its accessible name.
        let bucket_start_ms = marker["bucket_start_ms"]
            .as_i64()
            .unwrap_or_else(|| panic!("a marker carries no bucket start: {marker}"));
        let bucket_end_ms = marker["bucket_end_ms"]
            .as_i64()
            .unwrap_or_else(|| panic!("a marker carries no bucket end: {marker}"));
        let rendered = drawn
            .iter()
            .find(|drawn| drawn.bucket_start_ms == bucket_start_ms)
            .unwrap_or_else(|| panic!("the model's marker is not drawn: {marker}"));
        assert_eq!(
            (rendered.bucket_end_ms, rendered.count, rendered.duration_ms),
            (bucket_end_ms, ids.len(), Some(combined)),
            "the drawn marker does not name what the model's does: {rendered:#?}"
        );
        let covered_entries = if ids.len() == 1 {
            "1 entry".to_string()
        } else {
            format!("{} entries", ids.len())
        };
        assert!(
            rendered.name.contains(&covered_entries),
            "the marker's accessible name does not name how many entries it covers: {rendered:#?}"
        );
        assert!(
            rendered.name.contains(&format_duration(combined)),
            "the marker's accessible name does not name its combined duration: {rendered:#?}"
        );
        assert!(
            document.contains(&format!(
                "{count} · {duration}",
                count = ids.len(),
                duration = format_duration(combined)
            )),
            "the marker's visible text does not name its coverage and duration: {rendered:#?}"
        );
    }
    assert_eq!(
        covered,
        entries.len(),
        "the markers do not cover every recorded entry: {model:#?}"
    );
}

#[then("the report summary still counts every recorded entry")]
fn report_summary_still_counts_every_recorded_entry(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let recorded = seeded(world).len();

    // The summary covers the full record, never only the drawn shapes.
    assert_eq!(
        model["summary"]["entries"].as_u64(),
        Some(recorded as u64),
        "the summary does not count every recorded entry: {model:#?}"
    );
    assert_eq!(
        model["entries"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        recorded,
        "the model dropped entries in favour of its markers: {model:#?}"
    );
    let entries_tile = tile(&document, "entries");
    assert!(
        entries_tile.contains(&format_count(recorded as u64)),
        "the entries tile does not name the record's entry count ({recorded}): {entries_tile}"
    );
    assert!(
        rendered_aggregates(&document).len() < recorded,
        "the axis draws no fewer shapes than the record holds entries: {document}"
    );
}

/// One row of a generated list as the report renders it: the entry it names,
/// the visible kind it shows, the visible label, and the rendered duration.
#[derive(Debug, PartialEq)]
struct RenderedRow {
    id: u64,
    kind: String,
    label: String,
    duration: String,
}

/// The text of one `<span class="…">` element inside a rendered row.
fn row_element(row: &str, class: &str) -> String {
    let marker = format!("class=\"{class}\">");
    row.split(&marker)
        .nth(1)
        .and_then(|rest| rest.split('<').next())
        .unwrap_or_else(|| panic!("the row carries no `.`{class}`` element: {row}"))
        .to_string()
}

/// The rows of a generated list as the report renders them, in document order.
fn list_rows(document: &str, list_id: &str) -> Vec<RenderedRow> {
    let marker = format!("id=\"{list_id}\"");
    let start = document
        .find(&marker)
        .unwrap_or_else(|| panic!("the report holds no #{list_id} list"));
    let region = &document[start..];
    let end = region.find("</ol>").expect("the list is never closed");
    region[..end]
        .split("<li ")
        .skip(1)
        .map(|row| RenderedRow {
            id: row
                .split("data-entry-id=\"")
                .nth(1)
                .and_then(|value| value.split('"').next())
                .and_then(|value| value.parse().ok())
                .unwrap_or_else(|| panic!("a listed row carries no entry id: {row}")),
            kind: row_element(row, "step-kind"),
            label: row_element(row, "step-label"),
            duration: row_element(row, "step-duration"),
        })
        .collect()
}

/// The ranking the model carries: every measured entry, longest first, as
/// `(id, kind, label, duration_ms)`.
fn modelled_ranking(model: &Value) -> Vec<(u64, String, String, u64)> {
    model["summary"]["longest_steps"]
        .as_array()
        .unwrap_or_else(|| panic!("the model ranks no entries: {model:#?}"))
        .iter()
        .map(|ranked| {
            (
                ranked["id"]
                    .as_u64()
                    .unwrap_or_else(|| panic!("a ranked entry carries no id: {ranked}")),
                ranked["kind"]
                    .as_str()
                    .unwrap_or_else(|| panic!("a ranked entry carries no kind: {ranked}"))
                    .to_string(),
                ranked["label"]
                    .as_str()
                    .unwrap_or_else(|| panic!("a ranked entry carries no label: {ranked}"))
                    .to_string(),
                ranked["duration_ms"]
                    .as_u64()
                    .unwrap_or_else(|| panic!("a ranked entry carries no duration: {ranked}")),
            )
        })
        .collect()
}

#[then("the report ranks the agent tool call first")]
fn report_ranks_the_agent_tool_call_first(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let entries = seeded(world);
    let tool = entries
        .iter()
        .find(|entry| entry.kind == "tool")
        .expect("the fixture seeds the agent's tool call");
    let tool_duration = tool
        .duration_ms
        .expect("the fixture measures its tool call");

    // The fixture's own claim: the agent's call outlasts every step, so a
    // ranking restricted to `step` entries cannot put it first.
    let steps: Vec<&HistoryEntry> = entries
        .iter()
        .filter(|entry| entry.kind == "step")
        .collect();
    assert!(!steps.is_empty(), "the fixture seeds Givn's own steps too");
    for step in &steps {
        assert!(
            step.duration_ms.unwrap_or_default() < tool_duration,
            "the fixture's tool call must outlast every step: {step:#?}"
        );
    }

    // The ranking covers every measured entry of the record, longest first.
    let mut expected: Vec<&HistoryEntry> = entries
        .iter()
        .filter(|entry| entry.duration_ms.is_some())
        .collect();
    expected.sort_by(|a, b| {
        b.duration_ms
            .cmp(&a.duration_ms)
            .then_with(|| a.at.cmp(&b.at))
            .then_with(|| a.id.cmp(&b.id))
    });
    let expected_ids: Vec<u64> = expected.iter().map(|entry| entry.id).collect();
    let ranked = modelled_ranking(&model);
    let ranked_ids: Vec<u64> = ranked.iter().map(|(id, ..)| *id).collect();
    assert_eq!(
        ranked_ids, expected_ids,
        "the ranking is not every measured entry of the record, longest first: {model:#?}"
    );

    // The first row is the agent's tool call, and the ranking says which layer
    // recorded it.
    let (id, kind, label, duration) = &ranked[0];
    assert_eq!(*id, tool.id, "the agent's tool call is not ranked first");
    assert_eq!(kind, "tool", "the first ranked entry is not the tool call");
    assert_eq!(
        label,
        &tool
            .label
            .clone()
            .expect("the fixture labels its tool call"),
        "the first ranked entry does not carry the tool call's own label"
    );
    assert_eq!(*duration, tool_duration);
}

#[then("the ranking names the kind of every entry it lists")]
fn the_ranking_names_the_kind_of_every_entry_it_lists(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let ranked = modelled_ranking(&model);
    let ranked_ids: Vec<u64> = ranked.iter().map(|(id, ..)| *id).collect();
    assert_eq!(
        list_entry_ids(&document, "longest-steps"),
        ranked_ids,
        "the rendered ranking is not the model's: {document}"
    );

    let rows = list_rows(&document, "longest-steps");
    assert_eq!(
        rows.len(),
        ranked.len(),
        "the rendered ranking has a different number of rows than the model: {document}"
    );
    for (row, (id, kind, label, duration_ms)) in rows.iter().zip(&ranked) {
        assert_eq!(row.id, *id, "a rendered row names another entry: {row:?}");
        assert_eq!(
            row.kind, *kind,
            "the rendered row does not name its entry's kind: {row:?}"
        );
        assert_eq!(
            row.label, *label,
            "the rendered row does not name its entry's label: {row:?}"
        );
        assert_eq!(
            row.duration,
            format_duration(*duration_ms),
            "the rendered row does not name its entry's duration: {row:?}"
        );
    }

    // The kinds the fixture's own record carries are all named, and the agent's
    // activity is visibly not a Givn step.
    assert_eq!(
        rows[0].kind, "tool",
        "the first row is not the agent's call: {rows:?}"
    );
    assert!(
        rows.iter().any(|row| row.kind == "step"),
        "the ranking does not name Givn's own steps: {rows:?}"
    );
    let mut kinds: Vec<&str> = rows.iter().map(|row| row.kind.as_str()).collect();
    kinds.sort_unstable();
    kinds.dedup();
    let mut seeded_kinds: Vec<&str> = seeded(world)
        .iter()
        .filter(|entry| entry.duration_ms.is_some())
        .map(|entry| entry.kind.as_str())
        .collect();
    seeded_kinds.sort_unstable();
    seeded_kinds.dedup();
    assert_eq!(
        kinds, seeded_kinds,
        "the ranking's kinds are not the measured entries' kinds: {rows:?}"
    );
}

#[then("the report names the total wait duration and the number of waits")]
fn report_names_the_total_wait_duration_and_the_number_of_waits(world: &mut GivnWorld) {
    let document = report_text(world);
    let model = report_model(&document);
    let waits: Vec<&HistoryEntry> = seeded(world)
        .iter()
        .filter(|entry| entry.kind == "question")
        .collect();
    assert_eq!(
        waits.len(),
        2,
        "the fixture must hold two question waits: {waits:#?}"
    );
    let total: u64 = waits.iter().filter_map(|entry| entry.duration_ms).sum();
    assert_eq!(
        waits
            .iter()
            .filter(|entry| entry.duration_ms.is_none())
            .count(),
        0,
        "the fixture's waits carry known durations"
    );

    // The wait total is the waits' own recorded span — never the duration of the
    // turn the waits sit inside, which is the requester's "waiting is
    // indistinguishable from working" problem.
    let turn = seeded(world)
        .iter()
        .find(|entry| entry.kind == "turn")
        .expect("the fixture nests its waits in a turn");
    let turn_duration = turn.duration_ms.expect("the fixture's turn is measured");
    assert_ne!(
        total, turn_duration,
        "the fixture must make the wait total and the turn's duration distinguishable"
    );
    assert_eq!(
        model["summary"]["wait_ms"],
        json!(total),
        "the model's wait total is not the waits' own recorded durations: {model:#?}"
    );
    assert_eq!(
        model["summary"]["waits"],
        json!(waits.len()),
        "the model does not count the recorded waits: {model:#?}"
    );
    assert_eq!(
        model["summary"]["unmeasured_waits"],
        json!(0),
        "the fixture's waits are all measured: {model:#?}"
    );

    // The rendered text and the embedded model agree.
    let rendered = tile(&document, "waits");
    assert!(
        rendered.contains(&format_duration(total)),
        "the waiting tile does not name the total wait duration: {rendered}"
    );
    assert!(
        rendered.contains(&format!("{} waits", waits.len())),
        "the waiting tile does not name the number of waits: {rendered}"
    );
    assert!(
        !rendered.contains(&format_duration(turn_duration)),
        "the waiting tile shows the enclosing turn's duration instead of the waits': {rendered}"
    );
}
