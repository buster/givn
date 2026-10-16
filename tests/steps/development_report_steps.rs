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
