//! `@e2e` step definitions for the `agent-activity-reporting` capability
//! (Task E4).
//!
//! The scenario under this file installs the project (`givn init --agents
//! <target>`) and then drives that harness's *generated* reporter through its
//! own real entry point — the shared harness drivers in
//! [`crate::steps::development_timeline_support`] — and asserts on
//! `givn history --change demo --json`. The capability's unit/integration
//! steps live in [`crate::steps::agent_activity_reporting_steps`].
//!
//! This is a CLI (harness integration) interface, so the browser is used for
//! one thing only: the change-wide visual gate's evidence — the report the
//! scenario's own record produced, rendered in the real browser at both
//! contract viewports, captured after the final assertion.

use std::process::Output;

use serde_json::{Value, json};

use cucumber::{given, then, when};

use super::GivnWorld;
use super::development_timeline_support as support;
use super::development_timeline_support::HarnessActivity;
use crate::common;
use crate::common::TestEnv;
use crate::common::browser::{self, BrowserDriver};

/// The `@e2e` outline this file's steps drive; spelled as the delta feature
/// spells it, because the visual gate slugifies exactly this text (an outline's
/// examples share one slug and one evidence pair).
const REPORTER_SCENARIO: &str =
    "Consumer installs the agent reporter and its reported activity lands in the record";

/// The tool call the driver reports: one shell call, whose subject the record's
/// own bounded-label path keeps.
const TOOL_CALL: &str = "toolu_givn_e2e_1";

/// The tool the driver reports that call for.
const TOOL_NAME: &str = "Bash";

/// The command the reported call ran — the entry's label subject.
const TOOL_COMMAND: &str = "cargo test --locked --test features_runner";

/// The question wait the driver reports: the harness prompt the record labels
/// with its first line.
const QUESTION_WAIT: &str = "question_givn_e2e_1";

/// The question the reported wait asked.
const QUESTION_TEXT: &str = "Which repository should the change land in?";

/// The subagent the driver reports, under the identity the harness gave it.
const SUBAGENT: &str = "Architect.Scout";

/// The agent definition that ran the subagent.
const SUBAGENT_AGENT: &str = "Explore";

/// The model the reported turn belongs to: the usage the Claude driver reads
/// back from the session record names it, and the omp and opencode drivers
/// report it inline.
const TURN_MODEL: &str = "claude-sonnet-5";

/// State for the `@e2e` agent-activity-reporting scenarios: the output of the
/// last reporter run the scenario drove, so the assertions read what that
/// process produced rather than a re-derived expectation.
#[derive(Debug, Default)]
pub(crate) struct AgentActivityReportingE2eState {
    /// The reporter process's own output.
    pub(crate) reporter_output: Option<Output>,
}

/// The scenario's project root, as its Givens left it.
fn project_root(world: &GivnWorld) -> &TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// The activity the scenario reports from every harness: one completed tool
/// call, one subagent from its start to its completion, one question wait, the
/// session's turn boundary, and the session's own end. The boundaries are what
/// makes a real reporter deliver — the omp reporter buffers until a turn ends
/// or the session shuts down — and they are the harness's own events, never
/// invented activity: the record they leave behind is the reporter's, and the
/// assertion below reads the three spans out of it.
fn reported_activity() -> Vec<HarnessActivity> {
    vec![
        HarnessActivity::Tool {
            id: TOOL_CALL.to_string(),
            tool: TOOL_NAME.to_string(),
            args: json!({ "command": TOOL_COMMAND }),
        },
        HarnessActivity::Question {
            id: QUESTION_WAIT.to_string(),
            question: QUESTION_TEXT.to_string(),
        },
        HarnessActivity::Subagent {
            id: SUBAGENT.to_string(),
            agent: SUBAGENT_AGENT.to_string(),
        },
        HarnessActivity::Turn {
            index: 1,
            model: TURN_MODEL.to_string(),
            usage: None,
        },
        HarnessActivity::Shutdown,
    ]
}

/// The entries the machine-readable read printed — the read view's own stdout
/// (the `When`'s `givn history --change demo --json`), parsed as the machine
/// contract it is: the scenario asserts on what the command returned, never on
/// the store read behind its back.
fn printed_entries(world: &GivnWorld) -> Vec<Value> {
    let printed = super::stdout(world);
    serde_json::from_str(printed.trim()).unwrap_or_else(|error| {
        panic!("the machine-readable read printed no JSON entries ({error}): {printed:?}")
    })
}

/// The one printed entry of `kind` the assertion can reach, or a failure naming
/// what the read printed instead.
fn entry_of_kind<'a>(entries: &'a [Value], kind: &str) -> &'a Value {
    entries
        .iter()
        .find(|entry| entry["kind"] == json!(kind))
        .unwrap_or_else(|| {
            panic!("the machine-readable record holds no `{kind}` entry: {entries:#?}")
        })
}

/// The harness driver each Examples row is driven through: one entry point per
/// harness, all three with the same signature, so the scenario drives the row's
/// reporter without a per-harness branch of its own.
fn driver_for(target: &str) -> fn(&TestEnv, &[HarnessActivity]) -> Output {
    match target {
        "omp" => support::drive_omp_reporter,
        "opencode" => support::drive_opencode_reporter,
        "claude" => support::drive_claude_reporter,
        other => panic!("no development reporter is known for the agent target `{other}`"),
    }
}

/// The reporter process's own outcome: it exited `0` and the intake named no
/// rejection on it. A reporter that could not deliver says so here, and the
/// intake's rejected events arrive on the same output — the record the
/// assertion reads would be short without either being visible in it.
fn assert_reporter_delivered(reporter: &Output) {
    assert!(
        reporter.status.success(),
        "the reporter run failed (exit {:?}): {}",
        reporter.status.code(),
        String::from_utf8_lossy(&reporter.stderr),
    );
    let complaints = String::from_utf8_lossy(&reporter.stderr);
    assert!(
        !complaints.contains("warning:"),
        "the reporter's run reported an intake complaint: {complaints}"
    );
}

#[given(regex = r#"^the change agent target "([^"]+)" installs the development reporter$"#)]
fn target_installs_the_development_reporter(world: &mut GivnWorld, target: String) {
    // The scenario's first Given already brought the project up; this one is
    // the install under test: the real `givn init --agents <target>`, which
    // writes the target's generated reporter (asserted at drive time by the
    // harness driver, which names the file it needs). `--force` is what makes
    // the real install path run on an already-initialised root: without it,
    // `givn init` keeps the existing `givn/config.yaml` and its recorded
    // `skills.targets`, so `--agents` decides nothing there and no reporter is
    // written.
    let env = project_root(world);
    let out = common::run_givn(env, &["init", "--force", "--agents", target.as_str()]);
    assert!(
        out.status.success(),
        "givn init --force --agents {target} failed (exit {:?}): {}{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

#[when(
    regex = r#"^the installed "([^"]+)" reporter reports a completed tool call, a subagent, and a question wait$"#
)]
fn installed_reporter_reports_activity(world: &mut GivnWorld, target: String) {
    let activities = reported_activity();
    let env = project_root(world);
    // One entry point per harness, all three driven through the same vocabulary
    // above: the generated asset is imported/called (omp, opencode) or executed
    // as the shim (claude), and every one of them reaches the real intake.
    let driver = driver_for(&target);
    let output = driver(env, &activities);
    world
        .agent_activity_reporting_e2e
        .get_or_insert_with(AgentActivityReportingE2eState::default)
        .reporter_output = Some(output);
}

#[then("the machine-readable record names the tool call, the subagent, and the question wait")]
fn record_names_the_reported_activity(world: &mut GivnWorld) {
    // The reporter's own run first: a reporter that could not deliver says so
    // there, and the intake's rejections arrive on that same output.
    let reporter = world
        .agent_activity_reporting_e2e
        .as_ref()
        .and_then(|state| state.reporter_output.as_ref())
        .expect("the installed reporter has not reported yet");
    assert_reporter_delivered(reporter);

    // Then what the real CLI returned: the machine-readable record the reporter
    // itself delivered into.
    let entries = printed_entries(world);
    let scope = json!({ "kind": "change", "id": "demo" });

    let tool = entry_of_kind(&entries, "tool");
    assert_eq!(
        tool["name"],
        json!(TOOL_NAME),
        "the recorded tool call is not the reported one: {tool:#}"
    );
    assert_eq!(
        tool["label"],
        json!(TOOL_COMMAND),
        "the recorded tool call does not name what it ran: {tool:#}"
    );
    assert!(
        tool["duration_ms"].is_u64(),
        "the recorded tool call carries no duration: {tool:#}"
    );

    let subagent = entry_of_kind(&entries, "subagent");
    assert_eq!(
        subagent["name"],
        json!(SUBAGENT),
        "the recorded subagent is not the reported one: {subagent:#}"
    );
    assert_eq!(
        subagent["data"]["agent"],
        json!(SUBAGENT_AGENT),
        "the recorded subagent does not name the agent definition that ran it: {subagent:#}"
    );
    assert!(
        subagent["duration_ms"].is_u64(),
        "the recorded subagent carries no duration: {subagent:#}"
    );

    let question = entry_of_kind(&entries, "question");
    assert_eq!(
        question["label"],
        json!(QUESTION_TEXT),
        "the recorded question wait does not name the question that was asked: {question:#}"
    );
    assert!(
        question["duration_ms"].is_u64(),
        "the recorded question wait carries no duration: {question:#}"
    );

    // Every one of them belongs to the change the scenario reported under, not
    // to the project the reporters also ran in.
    for entry in [tool, subagent, question] {
        assert_eq!(
            entry["scope"], scope,
            "a reported entry left the change's scope: {entry:#}"
        );
    }

    commit_evidence(project_root(world), REPORTER_SCENARIO);
}

/// The change-wide visual gate's evidence for one `@e2e` CLI scenario: render
/// the report the scenario's own record produced with the real `givn timeline`,
/// then open that one file in the real browser and capture the contract's pair
/// (`mobile.png` 360x780, `desktop.png` 1920x1080) under
/// `evidence/visual/<scenario-slug>/`. Captured after the scenario's final
/// assertion, with animations disabled by the driver; the report is rendered
/// from the record's embedded timestamps, so the capture never depends on
/// timing. The outline's three Examples rows write the same two paths, so the
/// committed pair is always a run's own output.
fn commit_evidence(root: &TestEnv, scenario: &str) {
    let out = common::run_givn(
        root,
        &["timeline", "--change", "demo", "--out", "report.html"],
    );
    assert!(
        out.status.success(),
        "givn timeline failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let report = root.path("report.html");
    let mut browser = BrowserDriver::start();
    browser.goto(&browser::file_url(&report));
    browser.capture_evidence(scenario);
    browser.quit();
}
