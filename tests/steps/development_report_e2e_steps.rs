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

use cucumber::{given, then, when};
use serde_json::{Value, json};

use givn::history::Scope;

use super::GivnWorld;
use super::development_timeline_support as support;
use crate::common;
use crate::common::browser::{self, BrowserDriver};

/// The `@e2e` CLI scenario that generates the report, spelled as the delta
/// feature spells it because the visual gate slugifies exactly this text.
const GENERATE_SCENARIO: &str = "Consumer generates the development report for a change";

/// The `@e2e` browser outline that opens the report in a real browser.
const OPEN_SCENARIO: &str = "Consumer opens the generated development report in a browser";

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
    /// The scope the scenario's report covers, where its Given named one: a
    /// change, an ideation topic, or the project.
    pub(crate) scope: Option<Scope>,
    /// The rendered time range the zoom changed: what the browser showed before
    /// the interaction and what it showed after, so the scenario's assertion
    /// compares two observations of the page's own script rather than a
    /// re-derived value.
    pub(crate) zoomed_range: Option<(Value, Value)>,
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

/// The change workflow's nine phases, in the workflow's own order: what the
/// report's text view must name, top to bottom.
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

/// The phase names one written report draws, in document order: the visible
/// text of every workflow phase line in the text view's stack.
fn rendered_phases(document: &str) -> Vec<String> {
    document
        .split("<p class=\"workflow-phase\" data-phase=\"")
        .skip(1)
        .map(|phase| {
            phase
                .split('"')
                .next()
                .unwrap_or_else(|| panic!("a phase line carries no stage name: {phase}"))
                .to_string()
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

/// Open the report the scenario's own command wrote in the scenario's one
/// browser: the page every browser step then drives. One navigation through the
/// driver's `file://` URL — the report is one self-contained file, so there is
/// no server and no port — and no step opens a second context.
fn open_report(world: &mut GivnWorld) {
    let url = browser::file_url(&report_path(world));
    browser(world).goto(&url);
}

/// The change-wide visual gate's evidence for a CLI scenario: open the report
/// the scenario's own command wrote in the real browser and capture the
/// contract's pair (`mobile.png` 360x780, `desktop.png` 1920x1080) under
/// `evidence/visual/<scenario-slug>/`. Captured after the scenario's final
/// assertion, animations disabled by the driver, from the scenario's one
/// context.
fn commit_evidence(world: &mut GivnWorld, scenario: &str) {
    open_report(world);
    browser(world).capture_evidence(scenario);
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

#[then("the report file exists and names the workflow phases")]
fn report_names_the_workflow_phases(world: &mut GivnWorld) {
    let path = report_path(world);
    assert!(
        path.is_file(),
        "the report file does not exist: {}",
        path.display()
    );
    let document = report_document(world);
    // The text view names the change workflow's nine phases, in the workflow's
    // own order — not only the phases the record happened to reach.
    let phases = rendered_phases(&document);
    assert_eq!(
        phases,
        WORKFLOW_PHASES.to_vec(),
        "the report does not name the nine workflow phases in order: {phases:?}"
    );
    assert!(
        !document.contains("data-phases=\"recorded\""),
        "the report still speaks of recorded phases instead of the workflow's"
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

/// The scope the scenario's report covers: the one its Given named, or the
/// change the shared recorded-history Given created and recorded.
fn report_scope(world: &GivnWorld) -> Scope {
    if let Some(scope) = world
        .development_report_e2e
        .as_ref()
        .and_then(|state| state.scope.clone())
    {
        return scope;
    }
    let change = world
        .last_change_id
        .clone()
        .expect("a change or topic with a recorded history is required first");
    Scope::Change { id: change }
}

/// The scope's own identifier, as the scenario named it.
fn scope_id(scope: &Scope) -> &str {
    match scope {
        Scope::Change { id } | Scope::Ideation { id } | Scope::Project { id } => id,
    }
}

#[given(regex = r#"^a topic "([^"]+)" with a recorded history$"#)]
fn topic_with_a_recorded_history(world: &mut GivnWorld, topic: String) {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let entries = support::seed_ideation_topic(env, &topic);
    assert!(
        !entries.is_empty(),
        "the fixture seeded no topic record at all"
    );
    state(world).scope = Some(Scope::Ideation { id: topic });
}

#[given(regex = r#"^the development report for "([^"]+)" has been generated$"#)]
fn report_has_been_generated(world: &mut GivnWorld, id: String) {
    let scope = report_scope(world);
    assert_eq!(
        scope_id(&scope),
        id,
        "the scenario's report scope is not the scope it named"
    );
    let args: Vec<&str> = match &scope {
        Scope::Change { .. } => vec!["timeline", "--change", &id, "--out", REPORT_FILE],
        Scope::Ideation { .. } => vec!["timeline", "--topic", &id, "--out", REPORT_FILE],
        Scope::Project { .. } => vec!["timeline", "--out", REPORT_FILE],
    };
    let out = common::run_givn(project_root(world), &args);
    assert!(
        out.status.success(),
        "givn {} failed: {}{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let path = project_root(world).path(REPORT_FILE);
    assert!(
        path.is_file(),
        "the command wrote no report at {}",
        path.display()
    );
    let bytes = std::fs::metadata(&path)
        .unwrap_or_else(|error| panic!("read the report {}: {error}", path.display()))
        .len();
    assert!(
        bytes > 0,
        "the command wrote an empty report at {}",
        path.display()
    );
    let state = state(world);
    state.report_path = Some(path);
    state.scope = Some(scope);
}

#[when("the consumer opens the development report in a browser")]
fn consumer_opens_the_report(world: &mut GivnWorld) {
    open_report(world);
}

#[then("the browser shows the report heading and the current workflow state")]
fn browser_shows_the_heading_and_the_current_workflow_state(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const model = JSON.parse(document.getElementById("report-model").textContent);
  const current = document.getElementById("hybrid-current-state");
  const visible = (node) => node !== null && node.getClientRects().length > 0;
  return {
    title: document.title,
    headings: document.querySelectorAll("h1").length,
    header: document.querySelectorAll("header").length,
    main: document.querySelectorAll("main").length,
    heading: document.getElementById("report-title")
      ? document.getElementById("report-title").textContent.trim()
      : null,
    scope: document.getElementById("hybrid-scope") ? document.getElementById("hybrid-scope").textContent.trim() : null,
    currentVisible: visible(current),
    current: current ? current.textContent.trim() : null,
    currentState: model.current_state,
    done: document.getElementById("hybrid-phases-done")
      ? document.getElementById("hybrid-phases-done").textContent.trim()
      : null,
  };
})()"##,
    );
    assert_eq!(
        rendered["title"],
        json!("Development report"),
        "the page's own title is not the report's: {rendered:#}"
    );
    assert_eq!(
        rendered["headings"],
        json!(1),
        "the report does not carry exactly one h1: {rendered:#}"
    );
    assert_eq!(
        rendered["heading"],
        json!("Development report"),
        "the rendered heading is not the report's: {rendered:#}"
    );
    for landmark in ["header", "main"] {
        assert!(
            rendered[landmark].as_u64().unwrap_or_default() >= 1,
            "the report renders no {landmark} landmark: {rendered:#}"
        );
    }
    assert_eq!(
        rendered["currentVisible"],
        json!(true),
        "the browser shows no current workflow state: {rendered:#}"
    );
    let current = rendered["current"].as_str().unwrap_or_default();
    if rendered["currentState"].is_object() {
        let stage = rendered["currentState"]["stage_label"]
            .as_str()
            .unwrap_or_else(|| panic!("the change state names no stage: {rendered:#}"));
        let phase = rendered["currentState"]["phase"]
            .as_str()
            .unwrap_or_else(|| panic!("the change state names no phase: {rendered:#}"));
        assert!(
            current.contains(stage) && current.contains(phase),
            "the header does not name the stage {stage} and the phase {phase}: {current:?}"
        );
        assert!(
            current.contains("stage") && current.contains("phase"),
            "the header does not print the stage label and the phase as separate words: {current:?}"
        );
    } else {
        assert!(
            current.contains("no live change state"),
            "the topic report does not say it has no live change state: {current:?}"
        );
    }
    let done = rendered["done"].as_str().unwrap_or_default();
    assert!(
        done.contains("of 9 phases done"),
        "the header does not count the done phases of nine: {done:?}"
    );
}

#[then("the browser shows the workflow phases top to bottom")]
fn browser_shows_the_workflow_phases_top_to_bottom(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const model = JSON.parse(document.getElementById("report-model").textContent);
  const cards = [...document.querySelectorAll("#hybrid-workflow .phase-stack > .phase-card")];
  return {
    expected: model.workflow.map((phase) => phase.stage),
    drawn: cards.map((card) => card.dataset.phase),
    tops: cards.map((card) => card.getBoundingClientRect().top),
  };
})()"##,
    );
    let expected: Vec<String> = rendered["expected"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no workflow: {rendered:#}"))
        .iter()
        .map(|stage| {
            stage
                .as_str()
                .unwrap_or_else(|| panic!("a workflow phase carries no name: {rendered:#}"))
                .to_string()
        })
        .collect();
    let drawn: Vec<String> = rendered["drawn"]
        .as_array()
        .unwrap_or_else(|| panic!("the browser draws no phase card: {rendered:#}"))
        .iter()
        .map(|stage| {
            stage
                .as_str()
                .unwrap_or_else(|| panic!("a drawn phase carries no name: {rendered:#}"))
                .to_string()
        })
        .collect();
    assert_eq!(
        drawn, expected,
        "the browser's workflow phases are not the model's, in order: {rendered:#}"
    );
    let tops: Vec<f64> = rendered["tops"]
        .as_array()
        .unwrap_or_else(|| panic!("the drawn phases carry no positions: {rendered:#}"))
        .iter()
        .map(|top| top.as_f64().unwrap_or_default())
        .collect();
    assert!(
        tops.windows(2).all(|pair| pair[0] < pair[1]),
        "the workflow phases are not drawn top to bottom: {tops:?}"
    );
}

#[then("the browser shows the longest recorded step with its duration")]
fn browser_shows_the_longest_recorded_step(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const model = JSON.parse(document.getElementById("report-model").textContent);
  const rows = [...document.querySelectorAll("#hybrid-longest .longest-steps li")];
  const steps = model.summary.longest_steps || [];
  return {
    expected: steps.length === 0 ? null : steps[0],
    text: rows.length === 0 ? null : rows[0].textContent.trim(),
    id: rows.length === 0 ? null : rows[0].dataset.entryId,
    durations: rows.map((row) => Number(row.dataset.durationMs)),
  };
})()"##,
    );
    let longest = &rendered["expected"];
    assert!(
        longest.is_object(),
        "the record ranks no measured step: {rendered:#}"
    );
    let id = longest["id"].as_u64().expect("the longest step has an id");
    let label = longest["label"]
        .as_str()
        .unwrap_or_else(|| panic!("the longest step carries no label: {rendered:#}"));
    let duration = givn::cli::timeline::format_duration(
        longest["duration_ms"]
            .as_u64()
            .expect("the longest step was measured"),
    );
    assert_eq!(
        rendered["id"],
        json!(id.to_string()),
        "the first statistics row is not the longest recorded step: {rendered:#}"
    );
    let text = rendered["text"].as_str().unwrap_or_default();
    assert!(
        text.contains(label) && text.contains(&duration),
        "the browser does not show the longest step {label} with its duration {duration}: {text:?}"
    );
    let durations: Vec<u64> = rendered["durations"]
        .as_array()
        .unwrap_or_else(|| panic!("the longest-step list carries no durations: {rendered:#}"))
        .iter()
        .map(|value| value.as_u64().unwrap_or_default())
        .collect();
    assert!(
        durations.windows(2).all(|pair| pair[0] >= pair[1]),
        "the longest-step list is not ranked longest first: {durations:?}"
    );
}

#[then(
    "the browser lays the page out in one column without page-level horizontal scrolling at the mobile viewport"
)]
fn browser_lays_the_page_out_in_one_column_at_the_mobile_viewport(world: &mut GivnWorld) {
    browser(world).viewport(360, 780);
    let layout = browser(world).evaluate(
        r##"(() => {
  const main = document.getElementById("report-main");
  const style = getComputedStyle(main);
  const contentWidth = main.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight);
  const cards = [...document.querySelectorAll("#hybrid-workflow .phase-stack > .phase-card")];
  const kpis = document.querySelector(".kpi-strip");
  const columns = kpis ? getComputedStyle(kpis).gridTemplateColumns.split(" ").filter(Boolean) : [];
  return {
    viewport: window.innerWidth,
    scrollWidth: document.documentElement.scrollWidth,
    kpiColumns: columns.length,
    mainWidth: Math.round(contentWidth),
    cardWidths: cards.map((card) => Math.round(card.getBoundingClientRect().width)),
    cardLefts: cards.map((card) => Math.round(card.getBoundingClientRect().left)),
  };
})()"##,
    );
    assert_eq!(
        layout["viewport"],
        json!(360),
        "the browser is not at the mobile viewport: {layout:#}"
    );
    assert!(
        layout["scrollWidth"].as_u64().unwrap_or_default()
            <= layout["viewport"].as_u64().unwrap_or_default(),
        "the page scrolls horizontally at the mobile viewport: {layout:#}"
    );
    assert_eq!(
        layout["kpiColumns"],
        json!(2),
        "the KPI strip is not two columns at the mobile viewport: {layout:#}"
    );
    let main_width = layout["mainWidth"].as_u64().unwrap_or_default();
    let card_widths = layout["cardWidths"]
        .as_array()
        .unwrap_or_else(|| panic!("the browser draws no phase card: {layout:#}"));
    assert!(
        !card_widths.is_empty()
            && card_widths
                .iter()
                .all(|width| width.as_u64().unwrap_or_default() + 1 >= main_width),
        "the phase cards are not full width at the mobile viewport: {layout:#}"
    );
    let lefts: Vec<u64> = layout["cardLefts"]
        .as_array()
        .unwrap_or_else(|| panic!("the phase cards carry no positions: {layout:#}"))
        .iter()
        .map(|left| left.as_u64().unwrap_or_default())
        .collect();
    assert!(
        lefts.windows(2).all(|pair| pair[0] == pair[1]),
        "the phase cards are not in one column at the mobile viewport: {lefts:?}"
    );
    commit_evidence(world, OPEN_SCENARIO);
}

/// The `@e2e` browser scenario that expands a workflow phase.
const EXPAND_SCENARIO: &str = "Consumer expands a workflow phase and sees its lanes";

/// The selector every expand step drives: the phase's own expander button.
fn phase_expander(phase: &str) -> String {
    format!(".phase-card[data-phase=\"{phase}\"] .phase-expander")
}

/// The expanded phase's region.
fn phase_region(phase: &str) -> String {
    format!(".phase-card[data-phase=\"{phase}\"] .phase-region")
}

#[given("an open development report with a recorded history holding parallel sessions")]
fn open_report_with_parallel_sessions(world: &mut GivnWorld) {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    {
        let env = world
            .env
            .as_ref()
            .expect("a project root is required first");
        support::seed_parallel_sessions(env, "demo");
    }
    state(world).scope = Some(Scope::Change {
        id: "demo".to_string(),
    });
    report_has_been_generated(world, "demo".to_string());
    open_report(world);
}

#[when(regex = r#"^the consumer expands the "([^"]+)" phase$"#)]
fn consumer_expands_the_phase(world: &mut GivnWorld, phase: String) {
    let expander = phase_expander(&phase);
    assert_eq!(
        browser(world)
            .attribute(&expander, "aria-expanded")
            .as_deref(),
        Some("false"),
        "the {phase} phase expander does not start collapsed"
    );
    browser(world).click(&expander);
    assert_eq!(
        browser(world)
            .attribute(&expander, "aria-expanded")
            .as_deref(),
        Some("true"),
        "the {phase} phase expander did not report its expanded state"
    );
    let rendered = browser(world).evaluate(&format!(
        r##"(() => {{
  const region = document.querySelector({region});
  return {{ hidden: region ? region.hidden : null, role: region ? region.getAttribute("role") : null }};
}})()"##,
        region = json!(phase_region(&phase))
    ));
    assert_eq!(
        rendered["hidden"],
        json!(false),
        "the expanded {phase} phase is not visible: {rendered:#}"
    );
    assert_eq!(
        rendered["role"],
        json!("region"),
        "the expanded {phase} phase is not a labelled region: {rendered:#}"
    );
}

#[then("the browser shows that phase's vertical timeline with time flowing down")]
fn browser_shows_the_vertical_timeline(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const region = document.querySelector('.phase-card[data-phase="implement"] .phase-region');
  const ticks = [...region.querySelectorAll(".axis-tick")].map((tick) => tick.getBoundingClientRect().top);
  const tracks = [...region.querySelectorAll(".lane-track")].map((track) => track.getBoundingClientRect().height);
  return {
    axis: region.querySelectorAll(".phase-axis").length,
    ticks,
    tracks,
  };
})()"##,
    );
    assert_eq!(
        rendered["axis"],
        json!(1),
        "the expanded phase draws no vertical axis: {rendered:#}"
    );
    let tops: Vec<f64> = rendered["ticks"]
        .as_array()
        .unwrap_or_else(|| panic!("the vertical axis draws no tick: {rendered:#}"))
        .iter()
        .map(|top| top.as_f64().unwrap_or_default())
        .collect();
    assert!(
        tops.len() >= 2 && tops.windows(2).all(|pair| pair[0] < pair[1]),
        "the axis ticks do not flow downwards in time: {tops:?}"
    );
    let tracks: Vec<f64> = rendered["tracks"]
        .as_array()
        .unwrap_or_else(|| panic!("the phase draws no lane track: {rendered:#}"))
        .iter()
        .map(|height| height.as_f64().unwrap_or_default())
        .collect();
    assert!(
        !tracks.is_empty() && tracks.iter().all(|height| *height > 0.0),
        "the phase's lane tracks have no vertical extent: {tracks:?}"
    );
}

#[then("the browser shows one lane per session beside each other")]
fn browser_shows_one_lane_per_session(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const model = JSON.parse(document.getElementById("report-model").textContent);
  const phase = model.workflow.find((candidate) => candidate.stage === "implement");
  const lanes = [...document.querySelectorAll('.phase-card[data-phase="implement"] .lane')];
  return {
    expected: phase ? phase.lanes.map((lane) => lane.name) : null,
    drawn: lanes.map((lane) => lane.dataset.lane),
    lefts: lanes.map((lane) => lane.getBoundingClientRect().left),
  };
})()"##,
    );
    let expected: Vec<String> = rendered["expected"]
        .as_array()
        .unwrap_or_else(|| panic!("the model carries no implement lanes: {rendered:#}"))
        .iter()
        .map(|name| {
            name.as_str()
                .unwrap_or_else(|| panic!("a lane carries no visible name: {rendered:#}"))
                .to_string()
        })
        .collect();
    let drawn: Vec<String> = rendered["drawn"]
        .as_array()
        .unwrap_or_else(|| panic!("the browser draws no lane: {rendered:#}"))
        .iter()
        .map(|name| {
            name.as_str()
                .unwrap_or_else(|| panic!("a drawn lane carries no name: {rendered:#}"))
                .to_string()
        })
        .collect();
    assert_eq!(
        drawn, expected,
        "the browser does not draw one lane per recorded session: {rendered:#}"
    );
    assert!(
        drawn.len() >= 2,
        "the parallel-session fixture drew fewer than two lanes: {drawn:?}"
    );
    assert!(
        drawn.contains(&support::PARALLEL_SCOUT_LANE.to_string())
            && drawn.contains(&support::PARALLEL_SPAWN_LANE.to_string())
            && drawn.contains(&support::PARALLEL_UNATTRIBUTED_LANE.to_string()),
        "the browser does not name the recorded sessions' lanes and the unattributed lane: {drawn:?}"
    );
    let lefts: Vec<f64> = rendered["lefts"]
        .as_array()
        .unwrap_or_else(|| panic!("the lanes carry no positions: {rendered:#}"))
        .iter()
        .map(|left| left.as_f64().unwrap_or_default())
        .collect();
    assert!(
        lefts.windows(2).all(|pair| pair[0] < pair[1]),
        "the session lanes are not beside each other: {lefts:?}"
    );
}

#[then("the browser shows the Fit, 1 h, and 15 m zoom controls")]
fn browser_shows_the_per_phase_zoom_controls(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const region = document.querySelector('.phase-card[data-phase="implement"] .phase-region');
  const controls = [...region.querySelectorAll(".zoom-control")].map((control) => ({
    text: control.textContent.trim(),
    pressed: control.getAttribute("aria-pressed"),
  }));
  return { controls };
})()"##,
    );
    let controls = rendered["controls"]
        .as_array()
        .unwrap_or_else(|| panic!("the phase draws no zoom control: {rendered:#}"));
    let labels: Vec<String> = controls
        .iter()
        .map(|control| control["text"].as_str().unwrap_or_default().to_string())
        .collect();
    assert_eq!(
        labels,
        vec!["Fit".to_string(), "1 h".to_string(), "15 m".to_string()],
        "the phase's zoom controls do not carry the visible labels the contract names: {labels:?}"
    );
    // The expanded phase survives the mobile layout change: still expanded,
    // still one lane per session, and the lanes scroll horizontally inside the
    // card instead of widening the page.
    browser(world).viewport(360, 780);
    let mobile = browser(world).evaluate(
        r##"(() => {
  const card = document.querySelector('.phase-card[data-phase="implement"]');
  const region = card.querySelector(".phase-region");
  const scroller = card.querySelector(".lane-scroll");
  return {
    expanded: card.querySelector(".phase-expander").getAttribute("aria-expanded"),
    visible: !region.hidden,
    lanes: card.querySelectorAll(".lane").length,
    viewport: window.innerWidth,
    scrollWidth: document.documentElement.scrollWidth,
    laneOverflow: getComputedStyle(scroller).overflowX,
    laneScrollWidth: scroller.scrollWidth,
    laneClientWidth: scroller.clientWidth,
  };
})()"##,
    );
    assert_eq!(
        mobile["expanded"],
        json!("true"),
        "the expanded phase did not survive the mobile layout: {mobile:#}"
    );
    assert_eq!(
        mobile["visible"],
        json!(true),
        "the expanded region is hidden at the mobile viewport: {mobile:#}"
    );
    assert!(
        mobile["lanes"].as_u64().unwrap_or_default() >= 2,
        "the mobile layout lost the session lanes: {mobile:#}"
    );
    assert!(
        mobile["scrollWidth"].as_u64().unwrap_or_default()
            <= mobile["viewport"].as_u64().unwrap_or_default(),
        "the page scrolls horizontally at the mobile viewport: {mobile:#}"
    );
    assert_eq!(
        mobile["laneOverflow"],
        json!("auto"),
        "the lanes do not scroll inside their card: {mobile:#}"
    );
    assert!(
        mobile["laneScrollWidth"].as_u64().unwrap_or_default()
            > mobile["laneClientWidth"].as_u64().unwrap_or_default(),
        "the lanes do not need to scroll inside their card at the mobile viewport: {mobile:#}"
    );
    browser(world).capture_evidence_from(EXPAND_SCENARIO, &phase_region("implement"));
}

/// The `@e2e` browser scenario that selects a timeline element.
const SELECT_SCENARIO: &str = "Consumer selects a timeline element and reads its details";

#[given("an open development report with a recorded history")]
fn open_report_with_a_recorded_history(world: &mut GivnWorld) {
    if world.env.is_none() {
        world.env = Some(common::make_init_root());
    }
    let seeded = {
        let env = world
            .env
            .as_ref()
            .expect("a project root is required first");
        let seeded = support::seed_report_history(env, "demo", "tasks.md");
        // The scenario's own claim: the longest recorded step is a tool call
        // carrying the metadata fact and the bounded content field the panel
        // assertions read.
        support::seed_report_detail_tool(env, "demo");
        seeded
    };
    state(world).seeded = Some(seeded);
    state(world).scope = Some(Scope::Change {
        id: "demo".to_string(),
    });
    report_has_been_generated(world, "demo".to_string());
    open_report(world);
}

#[when(regex = r#"^the consumer selects the "([^"]+)" lane$"#)]
fn consumer_selects_the_lane(world: &mut GivnWorld, lane: String) {
    browser(world).click(&phase_expander("implement"));
    let selector =
        format!(".phase-card[data-phase=\"implement\"] .lane-header[data-lane=\"{lane}\"]");
    browser(world).click(&selector);
    assert_eq!(
        browser(world)
            .attribute(&selector, "aria-pressed")
            .as_deref(),
        Some("true"),
        "the selected lane header does not report its selected state"
    );
    let selected = browser(world).evaluate(
        r##"(() => {
  const lane = document.querySelector('.phase-card[data-phase="implement"] .lane.is-selected');
  return { name: lane ? lane.dataset.lane : null };
})()"##,
    );
    assert_eq!(
        selected["name"],
        json!(lane),
        "the browser did not select the {lane} lane: {selected:#}"
    );
}

/// The selected lane's log, as the browser rendered it.
fn lane_log_state(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r##"(() => {
  const log = document.getElementById("lane-log-implement");
  if (!log) return { present: false };
  return {
    present: true,
    visible: log.getClientRects().length > 0,
    label: log.getAttribute("aria-label"),
    title: log.querySelector(".lane-log-title") ? log.querySelector(".lane-log-title").textContent.trim() : null,
    text: log.innerText,
    facts: log.querySelector(".lane-facts") ? log.querySelector(".lane-facts").innerText : null,
  };
})()"##,
    )
}

#[then("the browser shows that lane's log")]
fn browser_shows_the_lane_log(world: &mut GivnWorld) {
    let log_state = lane_log_state(browser(world));
    assert_eq!(
        log_state["present"],
        json!(true),
        "the browser shows no lane log: {log_state:#}"
    );
    assert_eq!(
        log_state["visible"],
        json!(true),
        "the lane log is not visible: {log_state:#}"
    );
    assert_eq!(
        log_state["label"],
        json!("Lane log"),
        "the lane log is not the labelled region the design system names: {log_state:#}"
    );
    let title = log_state["title"].as_str().unwrap_or_default();
    assert!(
        title.contains(support::PARALLEL_SCOUT_LANE) && title.contains("log"),
        "the lane log is not about the selected lane: {title:?}"
    );
}

#[then("the log names the lane's start, its duration, and its outcome")]
fn log_names_the_lane_start_duration_and_outcome(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const model = JSON.parse(document.getElementById("report-model").textContent);
  const phase = model.workflow.find((candidate) => candidate.stage === "implement");
  const lane = phase ? phase.lanes.find((candidate) => candidate.name === "Architect.Scout") : null;
  const log = document.getElementById("lane-log-implement");
  if (!lane || !log) return null;
  const selected = lane.entries.map((id) => model.entries.find((entry) => entry.id === id)).filter(Boolean);
  const failed = selected.some((entry) => entry.outcome === "failed");
  return {
    first: lane.first_ms,
    duration: lane.duration_ms,
    outcome: failed ? "failed" : (selected[0] ? selected[0].outcome : "unknown"),
    facts: log.querySelector(".lane-facts").innerText,
  };
})()"##,
    );
    assert!(
        rendered.is_object(),
        "the browser renders no facts for the selected lane: {rendered:#}"
    );
    let facts = rendered["facts"].as_str().unwrap_or_default();
    let first = rendered["first"]
        .as_i64()
        .unwrap_or_else(|| panic!("the selected lane records no start: {rendered:#}"));
    let duration = rendered["duration"]
        .as_u64()
        .unwrap_or_else(|| panic!("the selected lane records no duration: {rendered:#}"));
    let outcome = rendered["outcome"].as_str().unwrap_or_default();
    for (label, value) in [
        ("Start", time_of_day(first)),
        ("Duration", givn::cli::timeline::format_duration(duration)),
        ("Outcome", outcome.to_string()),
    ] {
        assert!(
            facts.contains(label) && facts.contains(&value),
            "the log does not name the lane's {label} ({value}): {facts:?}"
        );
    }
}

#[then("the log names the entry's tool call id")]
fn log_names_the_entry_tool_call_id(world: &mut GivnWorld) {
    let log_state = lane_log_state(browser(world));
    let text = log_state["text"].as_str().unwrap_or_default();
    assert!(
        text.contains("Tool call id"),
        "the log names no tool call id fact: {text:?}"
    );
    assert!(
        text.contains(support::PARALLEL_TOOL_CALL_ID),
        "the log does not name the entry's tool call id ({}): {text:?}",
        support::PARALLEL_TOOL_CALL_ID
    );
}

#[then("the log renders the entry's content in its bounded content region")]
fn log_renders_the_bounded_content_region(world: &mut GivnWorld) {
    let rendered = browser(world).evaluate(
        r##"(() => {
  const log = document.getElementById("lane-log-implement");
  const region = log ? log.querySelector(".content-region") : null;
  const field = region ? region.querySelector('.content-field[data-field="tool_input"]') : null;
  const text = field ? field.querySelector(".content-text") : null;
  const style = text ? getComputedStyle(text) : null;
  return {
    inLog: region !== null,
    field: field !== null,
    text: text ? text.textContent : null,
    html: text ? text.innerHTML : null,
    meta: field && field.querySelector(".content-meta") ? field.querySelector(".content-meta").textContent : null,
    maxHeight: style ? style.maxHeight : null,
    overflowY: style ? style.overflowY : null,
  };
})()"##,
    );
    assert_eq!(
        rendered["inLog"],
        json!(true),
        "the log renders no content region: {rendered:#}"
    );
    assert_eq!(
        rendered["field"],
        json!(true),
        "the content region renders no tool-input field: {rendered:#}"
    );
    assert_eq!(
        rendered["text"],
        json!(support::PARALLEL_CONTENT_TEXT),
        "the content region does not render the captured text: {rendered:#}"
    );
    let html = rendered["html"].as_str().unwrap_or_default();
    assert!(
        html.contains("&lt;b&gt;") && !html.contains("<b>"),
        "the content region rendered the captured markup as HTML: {html}"
    );
    let meta = rendered["meta"].as_str().unwrap_or_default();
    assert!(
        meta.contains("64 KiB per field")
            && meta.contains(&format!(
                "original {} bytes",
                support::PARALLEL_CONTENT_TEXT.len()
            ))
            && meta.contains("complete"),
        "the content region does not name its cap, original size, and truncation state: {meta:?}"
    );
    assert_ne!(
        rendered["maxHeight"],
        json!("none"),
        "the content region is not bounded: {rendered:#}"
    );
    assert_eq!(
        rendered["overflowY"],
        json!("auto"),
        "the content region does not scroll inside its bound: {rendered:#}"
    );
    browser(world).capture_evidence_from(SELECT_SCENARIO, "#lane-log-implement");
}
/// One recorded instant as the report's own clock reads it: the time of day of
/// the embedded UTC timestamp, seconds included.
fn time_of_day(at_ms: i64) -> String {
    let seconds = at_ms.div_euclid(1_000).rem_euclid(86_400);
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3_600,
        seconds % 3_600 / 60,
        seconds % 60
    )
}

/// The `@e2e` browser scenario that zooms the timeline.
const ZOOM_SCENARIO: &str = "Consumer zooms the timeline to a shorter time range";

/// The visible range the page's own script rendered for one expanded phase,
/// read from the rendered label and its machine-readable attributes.
fn rendered_phase_range(driver: &mut BrowserDriver, phase: &str) -> Value {
    driver.evaluate(&format!(
        r##"(() => {{
  const label = document.getElementById({id});
  if (!label) return null;
  return {{
    text: label.textContent.trim(),
    start: Number(label.dataset.rangeStartMs),
    end: Number(label.dataset.rangeEndMs),
  }};
}})()"##,
        id = json!(format!("phase-range-{phase}"))
    ))
}

/// The emulated motion preference and the transition the browser computed for
/// the per-phase zoom control.
fn rendered_motion(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r##"(() => {
  const control = document.querySelector('.phase-card[data-phase="implement"] .zoom-control');
  const region = document.querySelector('.phase-card[data-phase="implement"] .phase-region');
  return {
    reduce: matchMedia("(prefers-reduced-motion: reduce)").matches,
    controlTransition: control ? getComputedStyle(control).transitionDuration : null,
    regionAnimation: region ? getComputedStyle(region).animationName : null,
  };
})()"##,
    )
}

#[when("the consumer zooms the expanded phase's timeline in")]
fn consumer_zooms_the_expanded_phase_timeline_in(world: &mut GivnWorld) {
    browser(world).click(&phase_expander("implement"));
    let before = rendered_phase_range(browser(world), "implement");
    assert!(
        before.is_object(),
        "the expanded phase renders no visible time range to zoom"
    );
    assert!(
        before["start"].as_i64().is_some() && before["end"].as_i64().is_some(),
        "the rendered phase range carries no machine-readable window: {before:#}"
    );
    // The real interaction: the per-phase `15 m` control, the next narrower
    // level. The hybrid has no global zoom.
    let control = ".phase-card[data-phase=\"implement\"] .zoom-control[data-zoom=\"15\"]";
    assert!(
        browser(world).text(control).contains("15 m"),
        "the 15 m zoom control carries no visible text label"
    );
    browser(world).click(control);
    let after = rendered_phase_range(browser(world), "implement");
    assert!(
        after.is_object(),
        "the expanded phase renders no visible time range after the zoom"
    );
    let width = |range: &Value| {
        range["end"].as_i64().unwrap_or_default() - range["start"].as_i64().unwrap_or_default()
    };
    assert!(
        width(&after) < width(&before),
        "the zoom did not narrow the rendered phase range: before={before:#} after={after:#}"
    );
    assert_ne!(
        after["text"], before["text"],
        "the rendered phase range label did not change between the two observations"
    );
    state(world)
        .zoomed_range
        .get_or_insert_with(|| (before.clone(), after.clone()));
}

#[then("the browser shows a narrower time range than before")]
fn browser_shows_a_narrower_time_range_than_before(world: &mut GivnWorld) {
    let (before, after) = world
        .development_report_e2e
        .as_ref()
        .and_then(|state| state.zoomed_range.clone())
        .expect("the expanded phase's timeline has not been zoomed yet");
    let observed = rendered_phase_range(browser(world), "implement");
    assert_eq!(
        observed, after,
        "the rendered phase range is not the range the zoom produced"
    );
    let width = |range: &Value| {
        range["end"].as_i64().unwrap_or_default() - range["start"].as_i64().unwrap_or_default()
    };
    assert!(
        width(&after) < width(&before),
        "the browser shows no narrower range than before the zoom: before={before:#} after={after:#}"
    );
    assert!(
        after["start"].as_i64() >= before["start"].as_i64()
            && after["end"].as_i64() <= before["end"].as_i64(),
        "the zoomed range left the phase's own recorded range: before={before:#} after={after:#}"
    );

    // The motion contract: the driver emulates `prefers-reduced-motion: reduce`
    // for the context, so the zoom's transition is removed entirely and the
    // narrowed range above is what the change applied instantly.
    let motion = rendered_motion(browser(world));
    assert_eq!(
        motion["reduce"],
        json!(true),
        "the browser does not emulate the reduced-motion preference the tier captures with: {motion:#}"
    );
    assert!(
        motion["controlTransition"]
            .as_str()
            .unwrap_or_default()
            .split(',')
            .all(|duration| duration.trim() == "0s"),
        "the zoom control still transitions under the reduced-motion preference: {motion:#}"
    );
    assert!(
        motion["regionAnimation"].as_str().unwrap_or_default() == "none",
        "the zoomed region still animates under the reduced-motion preference: {motion:#}"
    );

    // Every zoom level re-applies the one aggregation threshold: each lane's
    // DOM stays bounded instead of drawing one tick per recorded entry.
    let bounded = browser(world).evaluate(
        r##"(() => {
  const lanes = [...document.querySelectorAll('.phase-card[data-phase="implement"] .lane')];
  return lanes.map((lane) => lane.querySelectorAll(".tick").length);
})()"##,
    );
    for count in bounded
        .as_array()
        .unwrap_or_else(|| panic!("the lanes draw no ticks: {bounded:#}"))
    {
        assert!(
            count.as_u64().unwrap_or_default() <= 140,
            "a lane draws more ticks than the bounded threshold permits: {bounded:#}"
        );
    }

    // The zoom state survives the mobile layout change: the narrowed range is
    // still the rendered range.
    browser(world).viewport(360, 780);
    let mobile = rendered_phase_range(browser(world), "implement");
    assert_eq!(
        mobile, after,
        "the zoom state did not survive the mobile layout change"
    );
    browser(world).capture_evidence_from(
        ZOOM_SCENARIO,
        ".phase-card[data-phase=\"implement\"] .phase-region",
    );
}

/// The `@e2e` browser scenario that opens the report of a scope with no
/// recorded activity.
const EMPTY_SCENARIO: &str = "Consumer opens the report of a scope with no recorded activity";

/// The empty state the browser rendered, and the axis furniture it did *not*
/// render — read from the browser's DOM, never from the report file.
fn rendered_empty_state(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r##"(() => {
  const state = document.getElementById("report-empty-state");
  const link = state ? state.querySelector("a[href]") : null;
  return {
    present: state !== null,
    declared: state ? state.dataset.state : null,
    heading: state && state.querySelector("h2") ? state.querySelector("h2").textContent.trim() : null,
    detail: state && state.querySelector(".empty-state-detail")
      ? state.querySelector(".empty-state-detail").textContent.trim()
      : null,
    back: link ? link.textContent.trim() : null,
    backHref: link ? link.getAttribute("href") : null,
    scope: document.getElementById("report-scope")
      ? document.getElementById("report-scope").textContent.trim()
      : null,
    workflow: {
      sections: document.querySelectorAll("#hybrid-workflow, #hybrid-unstaged").length,
      cards: document.querySelectorAll(".phase-card").length,
      phases: document.querySelectorAll("[data-phase]").length,
      stack: document.querySelectorAll(".phase-stack").length,
      axes: document.querySelectorAll(".phase-axis").length,
      lanes: document.querySelectorAll(".lane").length,
      ticks: document.querySelectorAll(".tick").length,
      controls: document.querySelectorAll(".zoom-control").length,
    },
  };
})()"##,
    )
}

#[then(regex = r#"^the browser shows the empty state for "([^"]+)"$"#)]
fn browser_shows_the_empty_state(world: &mut GivnWorld, id: String) {
    let rendered = rendered_empty_state(browser(world));
    assert_eq!(
        rendered["present"],
        json!(true),
        "the browser renders no empty state at all: {rendered:#}"
    );
    assert_eq!(
        rendered["declared"],
        json!("empty"),
        "the rendered empty state does not declare its own state: {rendered:#}"
    );
    assert_eq!(
        rendered["heading"],
        json!("No recorded activity"),
        "the empty state carries no heading: {rendered:#}"
    );
    let detail = rendered["detail"].as_str().unwrap_or_default();
    assert!(
        detail.contains(&id) && detail.ends_with('.'),
        "the empty state carries no sentence explaining the state for {id}: {detail:?}"
    );
    assert!(
        !rendered["back"].as_str().unwrap_or_default().is_empty()
            && rendered["backHref"] == json!("#report-title"),
        "the empty state carries no labelled way back to the report root: {rendered:#}"
    );
    let scope = rendered["scope"].as_str().unwrap_or_default();
    assert!(
        scope.contains(&id),
        "the rendered report does not name the scope it is about: {scope:?}"
    );
}

#[then("the browser draws no workflow phase for it")]
fn browser_draws_no_workflow_phase(world: &mut GivnWorld) {
    let rendered = rendered_empty_state(browser(world));
    for marker in [
        "sections", "cards", "phases", "stack", "axes", "lanes", "ticks", "controls",
    ] {
        assert_eq!(
            rendered["workflow"][marker],
            json!(0),
            "the empty report draws {marker}: {rendered:#}"
        );
    }
    // The model may carry the scaffold, but the browser draws none of it: no
    // phase card, no phase name, and no axis reaches the page.
    let model = browser(world).evaluate(
        r##"(() => JSON.parse(document.getElementById("report-model").textContent).empty)()"##,
    );
    assert_eq!(
        model,
        json!(true),
        "the empty report's model does not declare the empty state"
    );
    // The declared state's captures, beside the final-state pair the visual
    // gate reads for every @e2e scenario.
    let driver = browser(world);
    driver.capture_evidence(EMPTY_SCENARIO);
    driver.capture_state_evidence(EMPTY_SCENARIO, "empty");
}
