//! Step definitions for the `development-report` capability.
//!
//! The scenarios generate a report through `givn timeline` and assert on the
//! file the command wrote — the JSON model in its data slot and the text it
//! renders — never on the store alone; the record fixtures live in
//! [`crate::steps::development_timeline_support`].

use std::path::PathBuf;

use cucumber::{given, then, when};
use givn::cli::timeline::format_duration;
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
