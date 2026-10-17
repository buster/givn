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

/// One scope's recorded kind, as the record names it.
fn scope_kind(scope: &Scope) -> &'static str {
    match scope {
        Scope::Change { .. } => "change",
        Scope::Ideation { .. } => "ideation",
        Scope::Project { .. } => "project",
    }
}

/// One scope's record, read back from the store as the fixture left it: the
/// ground truth the browser's rendered text is asserted against, for the
/// scenarios whose record another Given built.
fn recorded_entries(world: &GivnWorld, scope: &Scope) -> Vec<Value> {
    support::record_entries(&project_root(world).root())
        .into_iter()
        .filter(|entry| entry["scope"]["kind"] == json!(scope_kind(scope)))
        .filter(|entry| entry["scope"]["id"] == json!(scope_id(scope)))
        .collect()
}

/// The scope's longest measured entry: the step the record's own durations rank
/// first, never a fixture constant.
fn longest_measured(entries: &[Value]) -> Value {
    entries
        .iter()
        .filter(|entry| entry["duration_ms"].is_u64())
        .max_by_key(|entry| entry["duration_ms"].as_u64().unwrap_or_default())
        .unwrap_or_else(|| panic!("the scope's record holds no measured entry: {entries:#?}"))
        .clone()
}

/// One recorded entry's visible name: its bounded label, or its name where the
/// record carries no label — the same derivation the report renders.
fn visible_name(entry: &Value) -> String {
    entry["label"]
        .as_str()
        .or_else(|| entry["name"].as_str())
        .unwrap_or_else(|| panic!("the recorded entry names nothing: {entry:#}"))
        .to_string()
}

/// One recorded duration as the report renders it.
fn rendered_duration(entry: &Value) -> String {
    givn::cli::timeline::format_duration(
        entry["duration_ms"]
            .as_u64()
            .unwrap_or_else(|| panic!("the record measures no duration for the entry: {entry:#}")),
    )
}

/// The structure the browser rendered: the document's own title, its headings
/// and landmarks, the timeline's accessible name, and its drawn spans — read
/// from the browser's DOM, never from the report file.
fn rendered_structure(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r#"(() => {
  const timeline = document.querySelector("svg.timeline");
  return {
    title: document.title,
    headings: document.querySelectorAll("h1").length,
    header: document.querySelectorAll("header").length,
    main: document.querySelectorAll("main").length,
    aside: document.querySelectorAll("aside").length,
    timelines: document.querySelectorAll("svg.timeline").length,
    timelineName: timeline ? timeline.getAttribute("aria-label") : null,
    controls: [...document.querySelectorAll(".controls button")].map((button) => ({
      id: button.id,
      label: button.textContent.trim(),
      disabled: button.disabled,
    })),
    range: document.getElementById("timeline-range")
      ? {
          text: document.getElementById("timeline-range").textContent,
          start: Number(document.getElementById("timeline-range").dataset.rangeStartMs),
          end: Number(document.getElementById("timeline-range").dataset.rangeEndMs),
        }
      : null,
    spans: [...document.querySelectorAll("svg.timeline .span")].map((span) => ({
      id: span.dataset.entryId,
      name: span.getAttribute("aria-label"),
      tabindex: span.getAttribute("tabindex"),
    })),
  };
})()"#,
    )
}

/// What the browser focused for one `Tab` press, and the ring it rendered for
/// that element: the assertion reads the element the browser really focused and
/// the style it really applied, never a class name.
fn focus_probe(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r##"(() => {
  const active = document.activeElement;
  if (!active || active === document.body) return { marker: "", outline: "none", stroke: "none" };
  const marker = active.dataset && active.dataset.entryId
    ? "span:" + active.dataset.entryId
    : active.tagName.toLowerCase() + "#" + active.id;
  const target = active.classList && active.classList.contains("span")
    ? active.querySelector(".span-bar") || active
    : active;
  const style = getComputedStyle(target);
  return {
    marker: marker,
    outline: style.outlineStyle + " " + style.outlineWidth,
    stroke: style.stroke + " " + style.strokeWidth,
  };
})()"##,
    )
}

/// Every expected control, and every drawn span, is reachable by a real `Tab`
/// press in the browser's own focus order, and the focused element carries a
/// visible ring — an outline for a control, the accent stroke for a span. The
/// markers come from the browser's own `document.activeElement`, so the
/// assertion never trusts a class name the page wrote.
fn assert_every_control_is_tabbable(driver: &mut BrowserDriver, expected: &[String]) {
    let mut seen: Vec<String> = Vec::new();
    for _ in 0..expected.len() + 2 {
        driver.press(None, "Tab");
        let focused = focus_probe(driver);
        let marker = focused["marker"].as_str().unwrap_or_default().to_string();
        if marker.is_empty() {
            continue;
        }
        let outline = focused["outline"].as_str().unwrap_or_default().to_string();
        let stroke = focused["stroke"].as_str().unwrap_or_default().to_string();
        let outlined = outline.starts_with("solid") && !outline.ends_with("0px");
        assert!(
            outlined || stroke.ends_with(" 2px"),
            "the focused element {marker} draws no visible focus ring: outline={outline:?} stroke={stroke:?}"
        );
        if !seen.contains(&marker) {
            seen.push(marker);
        }
        if seen.len() == expected.len() {
            break;
        }
    }
    for control in expected {
        assert!(
            seen.contains(control),
            "the control {control} is not reachable by Tab: {seen:?}"
        );
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

#[then("the browser shows the report heading and the development timeline")]
fn browser_shows_the_heading_and_the_timeline(world: &mut GivnWorld) {
    let structure = rendered_structure(browser(world));
    assert_eq!(
        structure["title"],
        json!("Development report"),
        "the page's own title is not the report's: {structure:#}"
    );
    assert_eq!(
        structure["headings"],
        json!(1),
        "the report does not carry exactly one h1: {structure:#}"
    );
    assert_eq!(
        browser(world).text("#report-title"),
        "Development report",
        "the rendered heading is not the report's"
    );
    for landmark in ["header", "main", "aside"] {
        assert!(
            structure[landmark].as_u64().unwrap_or_default() >= 1,
            "the report renders no {landmark} landmark: {structure:#}"
        );
    }
    assert_eq!(
        structure["timelines"],
        json!(1),
        "the report renders no single timeline svg: {structure:#}"
    );
    assert_eq!(
        structure["timelineName"],
        json!("Development timeline"),
        "the timeline svg carries no accessible name: {structure:#}"
    );

    // Every span the browser drew is exposed with its own accessible name —
    // the design's baseline at every assertion — and is reachable by Tab with a
    // visible focus ring.
    let scope = report_scope(world);
    let entries = recorded_entries(world, &scope);
    let spans = structure["spans"]
        .as_array()
        .unwrap_or_else(|| panic!("the browser drew no span list: {structure:#}"));
    assert!(
        !spans.is_empty(),
        "the browser drew no span at all for the scope's {} recorded entries: {structure:#}",
        entries.len()
    );
    // Every control the browser rendered carries a visible text label, and
    // every control that is not disabled at its own limit is reachable by Tab.
    let mut expected: Vec<String> = Vec::new();
    for control in structure["controls"]
        .as_array()
        .unwrap_or_else(|| panic!("the browser rendered no control list: {structure:#}"))
    {
        let id = control["id"]
            .as_str()
            .unwrap_or_else(|| panic!("a rendered control carries no id: {control:#}"));
        let label = control["label"].as_str().unwrap_or_default();
        assert!(
            !label.is_empty(),
            "the rendered control {id} carries no visible label: {control:#}"
        );
        if control["disabled"] != json!(true) {
            expected.push(format!("button#{id}"));
        }
    }
    for span in spans {
        let id: u64 = span["id"]
            .as_str()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| panic!("a drawn span names no entry id: {span:#}"));
        let entry = entries
            .iter()
            .find(|entry| entry["id"].as_u64() == Some(id))
            .unwrap_or_else(|| panic!("a drawn span names no recorded entry: {span:#}"));
        assert_eq!(
            span["name"],
            json!(format!(
                "{}, {}",
                visible_name(entry),
                rendered_duration(entry)
            )),
            "a drawn span does not expose its name and duration: {span:#}"
        );
        assert_eq!(
            span["tabindex"],
            json!("0"),
            "a drawn span is not reachable by keyboard: {span:#}"
        );
        expected.push(format!("span:{id}"));
    }
    assert_every_control_is_tabbable(browser(world), &expected);
}

#[then("the browser shows the longest recorded step with its duration")]
fn browser_shows_the_longest_recorded_step(world: &mut GivnWorld) {
    let scope = report_scope(world);
    let entries = recorded_entries(world, &scope);
    let longest = longest_measured(&entries);
    let id = longest["id"].as_u64().expect("the longest step has an id");
    let name = visible_name(&longest);
    let duration = rendered_duration(&longest);
    let rendered = browser(world).evaluate(&format!(
        r#"(() => {{
  const span = [...document.querySelectorAll("svg.timeline .span")]
    .find((candidate) => candidate.dataset.entryId === "{id}");
  if (!span) return null;
  return {{ name: span.getAttribute("aria-label"), text: span.textContent.trim() }};
}})()"#
    ));
    assert!(
        !rendered.is_null(),
        "the browser draws no span for the record's longest step {id} ({name}, {duration})"
    );
    assert_eq!(
        rendered["name"],
        json!(format!("{name}, {duration}")),
        "the longest recorded step's accessible name is not its name and duration: {rendered:#}"
    );
    let text = rendered["text"].as_str().unwrap_or_default();
    assert!(
        text.contains(&duration),
        "the browser's own text for the longest recorded step names no duration: {text:?}"
    );
    assert!(
        text.contains(&name),
        "the browser's own text for the longest recorded step names no step: {text:?}"
    );
    commit_evidence(world, OPEN_SCENARIO);
}

/// The `@e2e` browser scenario that selects a timeline element.
const SELECT_SCENARIO: &str = "Consumer selects a timeline element and reads its details";

/// The span the browser draws for the scope's longest measured entry, with its
/// own selector — the element both activation paths then drive.
fn longest_span(world: &mut GivnWorld) -> (Value, String) {
    let scope = report_scope(world);
    let entries = recorded_entries(world, &scope);
    let longest = longest_measured(&entries);
    let id = longest["id"].as_u64().expect("the longest step has an id");
    let drawn = browser(world).evaluate(
        r#"(() => {
  const spans = [...document.querySelectorAll("svg.timeline .span")];
  const widths = spans.map((span) => Number(span.dataset.durationMs || -1));
  const longest = spans[widths.indexOf(Math.max(...widths))];
  return longest ? { id: longest.dataset.entryId } : null;
})()"#,
    );
    assert!(
        !drawn.is_null(),
        "the browser draws no span for the record's longest step {id}"
    );
    assert_eq!(
        drawn["id"],
        json!(id.to_string()),
        "the browser's longest drawn span is not the record's longest measured entry: {drawn:#} vs {longest:#}"
    );
    (
        longest,
        format!("svg.timeline .span[data-entry-id=\"{id}\"]"),
    )
}

/// The details panel the browser rendered: whether it is open, the label it
/// carries, the close control's own visible label, the facts it shows, and the
/// bounded content region — read from the browser's DOM, never from the report
/// file.
fn details_panel(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r#"(() => {
  const panel = document.getElementById("details-panel");
  const close = document.getElementById("details-close");
  const content = document.getElementById("details-content");
  const fields = content
    ? [...content.querySelectorAll(".content-field")].map((block) => ({
        field: block.dataset.field,
        text: block.querySelector(".content-text")?.innerText ?? "",
        html: block.querySelector(".content-text")?.innerHTML ?? "",
        meta: block.querySelector(".content-meta")?.innerText ?? "",
      }))
    : [];
  if (!panel) return { open: false, label: null, close: null, text: "", content: { hidden: true, fields: [] } };
  return {
    open: !panel.hidden,
    label: panel.getAttribute("aria-label"),
    role: panel.getAttribute("role"),
    close: close ? close.textContent.trim() : null,
    text: panel.innerText,
    content: {
      hidden: content ? content.hidden : true,
      text: content ? content.innerText : "",
      fields,
    },
  };
})()"#,
    )
}

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

#[when("the consumer selects the longest recorded step")]
fn consumer_selects_the_longest_recorded_step(world: &mut GivnWorld) {
    let (_, selector) = longest_span(world);
    // The pointer's own path: a real click on the span opens the panel.
    browser(world).click(&selector);
    let opened = details_panel(browser(world));
    assert_eq!(
        opened["open"],
        json!(true),
        "a real click on the step's span did not open the details panel: {opened:#}"
    );
    // The keyboard's own path: `Escape` closes the panel, and focusing the span
    // and pressing `Enter` opens the same panel again.
    browser(world).press(None, "Escape");
    let closed = details_panel(browser(world));
    assert_eq!(
        closed["open"],
        json!(false),
        "Escape did not close the details panel: {closed:#}"
    );
    browser(world).press(Some(&selector), "Enter");
    let reopened = details_panel(browser(world));
    assert_eq!(
        reopened["open"],
        json!(true),
        "Enter on the focused step's span did not open the details panel: {reopened:#}"
    );
}

#[then("the browser shows a details panel for that step")]
fn browser_shows_a_details_panel(world: &mut GivnWorld) {
    let (longest, _) = longest_span(world);
    let panel = details_panel(browser(world));
    assert_eq!(
        panel["open"],
        json!(true),
        "the browser shows no open details panel: {panel:#}"
    );
    assert_eq!(
        panel["role"],
        json!("region"),
        "the details panel is not the labelled region the design system names: {panel:#}"
    );
    assert_eq!(
        panel["label"],
        json!("Details"),
        "the details panel carries no accessible label: {panel:#}"
    );
    assert_eq!(
        panel["close"],
        json!("Close"),
        "the details panel's close control carries no visible label: {panel:#}"
    );
    let text = panel["text"].as_str().unwrap_or_default();
    assert!(
        text.contains(&visible_name(&longest)),
        "the details panel is not about the step the consumer selected: {text:?}"
    );
}

#[then("the panel names its start, its duration, and its outcome")]
fn panel_names_start_duration_and_outcome(world: &mut GivnWorld) {
    let (longest, _) = longest_span(world);
    let panel = details_panel(browser(world));
    let text = panel["text"].as_str().unwrap_or_default();
    let end = givn::history::parse_rfc3339_ms(longest["at"].as_str().unwrap_or_default())
        .expect("the selected step records a readable end");
    let duration = longest["duration_ms"]
        .as_u64()
        .expect("the selected step was measured");
    let start_ms = end - i64::try_from(duration).expect("the duration fits in milliseconds");
    let start = time_of_day(start_ms);
    assert!(
        text.contains(&start),
        "the panel names no start ({start}) for the selected step: {text:?}"
    );
    let duration = givn::cli::timeline::format_duration(duration);
    assert!(
        text.contains(&duration),
        "the panel names no duration ({duration}) for the selected step: {text:?}"
    );
    let outcome = longest["outcome"]
        .as_str()
        .expect("the recorded step carries its outcome");
    assert!(
        text.contains(outcome),
        "the panel names no outcome ({outcome}) for the selected step: {text:?}"
    );
    assert!(
        text.contains("Duration") && text.contains("Start") && text.contains("Outcome"),
        "the panel's facts carry no written labels: {text:?}"
    );
    // The visual gate's evidence for this scenario: the panel open, at both
    // contract viewports, with the page brought to the panel the assertion above
    // read (it sits below the fold at the mobile viewport).
    browser(world).capture_evidence_from(SELECT_SCENARIO, "#details-panel");
}

#[then("the panel names the entry's tool call id")]
fn panel_names_the_tool_call_id(world: &mut GivnWorld) {
    let (longest, _) = longest_span(world);
    let call = longest["data"]["tool_call_id"].as_str().unwrap_or_else(|| {
        panic!("the fixture's longest step carries no tool call id: {longest:#}")
    });
    let panel = details_panel(browser(world));
    let text = panel["text"].as_str().unwrap_or_default();
    assert!(
        text.contains("Tool call id"),
        "the panel names no tool call id fact: {text:?}"
    );
    assert!(
        text.contains(call),
        "the panel does not name the entry's tool call id ({call}): {text:?}"
    );
}

#[then("the panel renders the entry's content in its bounded content region")]
fn panel_renders_bounded_content(world: &mut GivnWorld) {
    let (longest, _) = longest_span(world);
    let field = &longest["data"]["content"]["tool_input"];
    let expected = field["text"]
        .as_str()
        .unwrap_or_else(|| panic!("the fixture's tool entry carries no content text: {longest:#}"));
    let original = field["bytes"]
        .as_u64()
        .unwrap_or_else(|| panic!("the fixture's content field carries no size: {longest:#}"));
    let panel = details_panel(browser(world));
    let region = &panel["content"];
    assert_eq!(
        region["hidden"],
        json!(false),
        "the panel renders no content region for the entry: {panel:#}"
    );
    let fields = region["fields"]
        .as_array()
        .unwrap_or_else(|| panic!("the content region carries no fields: {region:#}"));
    let rendered = fields
        .iter()
        .find(|field| field["field"] == json!("tool_input"))
        .unwrap_or_else(|| panic!("the content region does not render the tool input: {region:#}"));
    assert_eq!(
        rendered["text"],
        json!(expected),
        "the content region does not render the captured text: {rendered:#}"
    );
    let meta = rendered["meta"].as_str().unwrap_or_default();
    assert!(
        meta.contains("64 KiB per field"),
        "the content region names no cap: {meta:?}"
    );
    assert!(
        meta.contains(&format!("original {original} bytes")),
        "the content region does not name the original size: {meta:?}"
    );
    assert!(
        meta.contains("complete"),
        "the content region does not state the field's truncation state: {meta:?}"
    );
    // Escaped: the captured text carries markup, and the region renders it as
    // text, never as an element.
    let html = rendered["html"].as_str().unwrap_or_default();
    assert!(
        !html.contains("<b>") && html.contains("&lt;b&gt;"),
        "the content region rendered the captured markup as HTML: {html}"
    );
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

/// The range the browser rendered: the visible window the page's own script
/// produced, read from the rendered label and its machine-readable attributes.
fn rendered_range(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r#"(() => {
  const label = document.getElementById("timeline-range");
  if (!label) return null;
  return {
    text: label.textContent.trim(),
    start: Number(label.dataset.rangeStartMs),
    end: Number(label.dataset.rangeEndMs),
  };
})()"#,
    )
}

/// The rendered motion state: the emulated preference and the durations the
/// browser computed for the timeline's own transition and animation.
fn rendered_motion(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r#"(() => {
  const timeline = document.querySelector("svg.timeline");
  const style = getComputedStyle(timeline);
  return {
    reduce: matchMedia("(prefers-reduced-motion: reduce)").matches,
    animation: style.animationName + " " + style.animationDuration,
    transition: style.transitionDuration,
  };
})()"#,
    )
}

#[when("the consumer zooms the timeline in")]
fn consumer_zooms_the_timeline_in(world: &mut GivnWorld) {
    let before = rendered_range(browser(world));
    assert!(
        !before.is_null(),
        "the report renders no visible time range to zoom"
    );
    assert!(
        before["start"].as_i64().is_some() && before["end"].as_i64().is_some(),
        "the rendered range carries no machine-readable window: {before:#}"
    );
    // The real interaction: a click on the rendered zoom-in control.
    let control = browser(world).evaluate(
        r#"(() => {
  const button = document.getElementById("zoom-in");
  const out = document.getElementById("zoom-out");
  return button
    ? {
        label: button.textContent.trim(),
        disabled: button.disabled,
        outDisabled: out ? out.disabled : null,
      }
    : null;
})()"#,
    );
    assert_eq!(
        control["label"],
        json!("Zoom in"),
        "the zoom-in control carries no visible text label: {control:#}"
    );
    assert_eq!(
        control["outDisabled"],
        json!(true),
        "the zoom-out control is not disabled while the record's whole range is shown: {control:#}"
    );
    browser(world).click("#zoom-in");
    // The control at the limit reveals itself once the range narrows: zooming
    // out is reachable again exactly when there is something to zoom out to.
    let widened = browser(world).evaluate(
        r#"(() => {
  const out = document.getElementById("zoom-out");
  return out ? { disabled: out.disabled } : null;
})()"#,
    );
    assert_eq!(
        widened["disabled"],
        json!(false),
        "the zoom-out control stays disabled after the range narrowed: {widened:#}"
    );
    let after = rendered_range(browser(world));
    assert!(
        !after.is_null(),
        "the report renders no visible time range after the zoom"
    );
    // The rendered state changed between the two observations: the page's own
    // script produced a narrower window, and the label it rendered says so.
    let width = |range: &Value| {
        range["end"].as_i64().unwrap_or_default() - range["start"].as_i64().unwrap_or_default()
    };
    assert!(
        width(&after) < width(&before),
        "the zoom did not narrow the rendered time range: before={before:#} after={after:#}"
    );
    assert_ne!(
        after["text"], before["text"],
        "the rendered range label did not change between the two observations"
    );
    state(world)
        .zoomed_range
        .get_or_insert_with(|| (before.clone(), after.clone()));
}

#[then("the browser shows a narrower time range than before")]
fn browser_shows_a_narrower_time_range(world: &mut GivnWorld) {
    let (before, after) = world
        .development_report_e2e
        .as_ref()
        .and_then(|state| state.zoomed_range.clone())
        .expect("the timeline has not been zoomed yet");
    let observed = rendered_range(browser(world));
    assert_eq!(
        observed, after,
        "the rendered range the zoom produced is not the range the browser shows now"
    );
    let width = |range: &Value| {
        range["end"].as_i64().unwrap_or_default() - range["start"].as_i64().unwrap_or_default()
    };
    assert!(
        width(&after) < width(&before),
        "the browser shows no narrower range than before the zoom: before={before:#} after={after:#}"
    );
    // The window's own bounds: the narrowed range stays inside the record's
    // range, so the axis never leaves the timestamps the report embeds.
    assert!(
        after["start"].as_i64() >= before["start"].as_i64()
            && after["end"].as_i64() <= before["end"].as_i64(),
        "the zoomed range left the record's own range: before={before:#} after={after:#}"
    );

    // The motion contract: the driver emulates `prefers-reduced-motion: reduce`
    // for the context, so the zoom's transition is removed entirely and the
    // range above — already the narrowed one — is what the change applied
    // instantly.
    let motion = rendered_motion(browser(world));
    assert_eq!(
        motion["reduce"],
        json!(true),
        "the browser does not emulate the reduced-motion preference the tier captures with: {motion:#}"
    );
    assert!(
        motion["animation"]
            .as_str()
            .unwrap_or_default()
            .starts_with("none"),
        "the zoom's animation still runs under the reduced-motion preference: {motion:#}"
    );
    // The visual gate's evidence for this scenario: the narrowed range at both
    // contract viewports, captured after the assertions above.
    browser(world).capture_evidence(ZOOM_SCENARIO);
}

/// The `@e2e` browser scenario that opens the report of a scope with no
/// recorded activity.
const EMPTY_SCENARIO: &str = "Consumer opens the report of a scope with no recorded activity";

/// The empty state the browser rendered, and the axis furniture it did *not*
/// render — read from the browser's DOM, never from the report file.
fn rendered_empty_state(driver: &mut BrowserDriver) -> Value {
    driver.evaluate(
        r#"(() => {
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
    axis: {
      svg: document.querySelectorAll("svg").length,
      timelines: document.querySelectorAll("svg.timeline").length,
      bands: document.querySelectorAll(".band").length,
      spans: document.querySelectorAll(".span").length,
      ticks: document.querySelectorAll(".tick").length,
      aggregates: document.querySelectorAll(".aggregate").length,
      controls: document.querySelectorAll(".controls button").length,
    },
  };
})()"#,
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

#[then("the browser draws no timeline axis for it")]
fn browser_draws_no_timeline_axis(world: &mut GivnWorld) {
    let rendered = rendered_empty_state(browser(world));
    for marker in [
        "svg",
        "timelines",
        "bands",
        "spans",
        "ticks",
        "aggregates",
        "controls",
    ] {
        assert_eq!(
            rendered["axis"][marker],
            json!(0),
            "the empty report draws {marker}: {rendered:#}"
        );
    }
    // The declared state's captures, beside the final-state pair the visual gate
    // reads for every @e2e scenario.
    let driver = browser(world);
    driver.capture_evidence(EMPTY_SCENARIO);
    driver.capture_state_evidence(EMPTY_SCENARIO, "empty");
}
