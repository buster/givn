//! `@e2e` step definitions for the `development-report` capability (Tasks E5
//! to E9).
//!
//! The CLI scenario generates the report with the real `givn timeline` and
//! asserts on the file it wrote; the browser scenarios drive the real report
//! through [`crate::common::browser::BrowserDriver`] — one browser and one
//! context per scenario — and assert on what the browser rendered, never on
//! the file's raw bytes. The capability's unit/integration steps live in
//! [`crate::steps::development_report_steps`].

use std::path::PathBuf;

use cucumber::{given, then};

use super::GivnWorld;
use super::development_timeline_support as support;
use crate::common;
use crate::common::browser::{self, BrowserDriver};

/// The `@e2e` CLI scenario that generates the report, spelled as the delta
/// feature spells it because the visual gate slugifies exactly this text.
const GENERATE_SCENARIO: &str = "Consumer generates the development report for a change";

/// The report's file name inside the scenario's own fixture root: what the
/// `--out` invocation writes and what the browser scenarios then open.
const REPORT_FILE: &str = "report.html";

/// State for the `@e2e` development-report scenarios: the report the scenario
/// generated, the record its Given seeded, and the browser a browser scenario
/// drives for the scenario's lifetime. The driver owns its browser and one
/// context; dropping this state closes both, so a failing step never leaves a
/// browser behind.
#[derive(Debug, Default)]
pub(crate) struct DevelopmentReportE2eState {
    /// The report the scenario's own command wrote, read back as the written
    /// file rather than as the store it was built from.
    pub(crate) report_path: Option<PathBuf>,
    /// The record the scenario's Given seeded: the steps and artifact
    /// transitions the written report's assertions read their expectations
    /// from.
    pub(crate) seeded: Option<support::SeededReportHistory>,
    /// The scenario's browser, started on the first browser step: one driver,
    /// one browser, one context, so a page state survives across the steps.
    pub(crate) browser: Option<BrowserDriver>,
}

/// The scenario's fixture root, as its Givens left it.
fn project_root(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// The scenario's report state, created on first use.
fn state(world: &mut GivnWorld) -> &mut DevelopmentReportE2eState {
    world
        .development_report_e2e
        .get_or_insert_with(DevelopmentReportE2eState::default)
}

/// The record the scenario's Given seeded.
fn seeded(world: &GivnWorld) -> &support::SeededReportHistory {
    world
        .development_report_e2e
        .as_ref()
        .and_then(|state| state.seeded.as_ref())
        .expect("a seeded development report history is required first")
}

/// The scenario's browser, started on first use: the driver spawns the real
/// browser once, so every browser step of one scenario drives the same context.
fn browser(world: &mut GivnWorld) -> &mut BrowserDriver {
    state(world)
        .browser
        .get_or_insert_with(BrowserDriver::start)
}

/// The report the scenario's own command wrote, read as the written file — the
/// command's output, never the store it was built from.
fn report_document(world: &GivnWorld) -> String {
    let path = report_path(world);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the report {}: {error}", path.display()))
}

/// The report file the scenario's command wrote and its steps read back.
fn report_path(world: &GivnWorld) -> PathBuf {
    world
        .development_report_e2e
        .as_ref()
        .and_then(|state| state.report_path.clone())
        .expect("the development report has not been generated yet")
}

/// The phase names one written report draws, in document order: the visible
/// text of every band row's phase line, split into the stage names it carries.
fn rendered_phases(document: &str) -> Vec<String> {
    document
        .split("<text class=\"band-phases\"")
        .skip(1)
        .flat_map(|phase| {
            let content = phase
                .split('>')
                .nth(1)
                .unwrap_or_else(|| panic!("a phase line carries no text: {phase}"));
            content
                .split("</text")
                .next()
                .unwrap_or_default()
                .split(" · ")
                .map(|name| name.trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

/// One span's own markup, as the written report draws the entry `entry` — the
/// region an assertion reads a step's placement from.
fn span_markup(document: &str, entry: u64) -> String {
    let marker = format!("<g class=\"span\" data-entry-id=\"{entry}\"");
    let start = document
        .find(&marker)
        .unwrap_or_else(|| panic!("the report draws no span for the entry {entry}"));
    let region = &document[start..];
    let end = region.find("</g>").expect("a drawn span is never closed");
    region[..end].to_string()
}

/// One attribute of a markup fragment.
fn attribute(markup: &str, name: &str) -> Option<String> {
    let marker = format!("{name}=\"");
    markup
        .split(&marker)
        .nth(1)
        .and_then(|value| value.split('"').next())
        .map(str::to_string)
}

/// The change-wide visual gate's evidence for a CLI scenario: open the report
/// the scenario's own command wrote in the real browser and capture the
/// contract's pair (`mobile.png` 360x780, `desktop.png` 1920x1080) under
/// `evidence/visual/<scenario-slug>/`. Captured after the scenario's final
/// assertion, animations disabled by the driver, from the scenario's one
/// context.
fn commit_evidence(world: &mut GivnWorld, scenario: &str) {
    let path = report_path(world);
    let driver = browser(world);
    driver.goto(&browser::file_url(&path));
    driver.capture_evidence(scenario);
}

#[given(
    regex = r#"^a change "([^"]+)" with a recorded history whose last recorded artifact transition is "([^"]+)"$"#
)]
fn change_with_artifact_transitions(world: &mut GivnWorld, change: String, artifact: String) {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let seeded = support::seed_report_history(env, &change, &artifact);
    state(world).seeded = Some(seeded);
}

#[then("the command prints the path of the written report")]
fn command_prints_the_report_path(world: &mut GivnWorld) {
    let printed = super::stdout(world).trim().to_string();
    assert!(
        !printed.is_empty(),
        "the command printed no report path at all"
    );
    let path = project_root(world).path(&printed);
    assert_eq!(
        path,
        project_root(world).root().join(REPORT_FILE),
        "the command printed another path than the report it was asked for"
    );
    assert!(
        path.is_file(),
        "the printed path is not a file: {}",
        path.display()
    );
    let bytes = std::fs::metadata(&path)
        .unwrap_or_else(|error| panic!("read the report {}: {error}", path.display()))
        .len();
    assert!(
        bytes > 0,
        "the command printed an empty report: {}",
        path.display()
    );
    state(world).report_path = Some(path);
}

#[then("the report file exists and names the recorded phases")]
fn report_names_the_recorded_phases(world: &mut GivnWorld) {
    let document = report_document(world);
    let phases = rendered_phases(&document);
    let transitions = &seeded(world).transitions;
    // The report names one phase per stage the record reached: the opening
    // stage, plus one stage per artifact transition the record carries.
    assert_eq!(
        phases.len(),
        transitions.len() + 1,
        "the report names {} phases for the record's {} artifact transitions: {phases:?}",
        phases.len(),
        transitions.len(),
    );
    // The stage names themselves are the workflow's: the record's transitions
    // are the workflow's own artifacts, so the phase line reads them in order.
    assert_eq!(
        phases,
        vec![
            "propose",
            "specs",
            "design",
            "arc42",
            "design-review",
            "tasks",
            "implement"
        ],
        "the report's phase names do not follow the record's artifact transitions {transitions:?}"
    );
}

#[then(
    regex = r#"^the report file places the step recorded after that transition in the "([^"]+)" phase$"#
)]
fn report_places_the_post_transition_step(world: &mut GivnWorld, phase: String) {
    let document = report_document(world);
    let step = seeded(world).post_transition_step.clone();
    let span = span_markup(&document, step.id);
    let stage = attribute(&span, "data-stage")
        .unwrap_or_else(|| panic!("the report places the step {} in no phase: {span}", step.id));
    assert_eq!(
        stage, phase,
        "the report places the step recorded after the last transition in another phase: {span}"
    );
    assert!(
        rendered_phases(&document).contains(&phase),
        "the report places the step in a phase its row does not name: {phase}"
    );
    commit_evidence(world, GENERATE_SCENARIO);
}
