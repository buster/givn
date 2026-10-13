//! Step definitions for the `serve-extended-rules` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). The
//! shared served-instruction content step and the `served_instruction` helper
//! live here.

use std::fs;
use std::process::Output;

use cucumber::{given, then, when};

use crate::common;
use crate::steps::GivnWorld;

/// Instruction ids that are manifest artifacts and therefore need a change
/// context when served. Policy and step ids resolve without one.
const MANIFEST_ARTIFACT_IDS: [&str; 6] = [
    "proposal",
    "specs",
    "design",
    "design-review",
    "tasks",
    "review",
];

/// The fixture's single active change id.
fn active_change_id(env: &common::TestEnv) -> String {
    let dir = env.path("givn/changes");
    let mut ids: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|error| panic!("read changes dir {}: {error}", dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_dir() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    ids.sort();
    assert_eq!(
        ids.len(),
        1,
        "expected exactly one active change in the fixture, got {ids:?}"
    );
    ids.remove(0)
}

/// Run `givn instructions <id>` in the fixture and return its stdout. Manifest
/// artifact ids get `--change <active>`; policy and step ids do not.
pub(crate) fn served_instruction(env: &common::TestEnv, id: &str) -> String {
    let mut args = vec!["instructions", id];
    let change;
    if MANIFEST_ARTIFACT_IDS.contains(&id) {
        change = active_change_id(env);
        args.push("--change");
        args.push(change.as_str());
    }
    let output = common::run_givn(env, &args);
    assert!(
        output.status.success(),
        "givn instructions {id} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// The default fixture for the ideation guidance scenarios: a Givn project
/// with every addon enabled, so the addon-gated instruction fragments render.
#[given("a Givn project")]
fn givn_project(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[then(regex = r#"^the served ([a-z-]+) instruction contains "([^"]+)"$"#)]
fn served_instruction_contains(world: &mut GivnWorld, id: String, text: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = served_instruction(env, &id);
    assert!(
        content.contains(&text),
        "the served {id} instruction is missing {text:?}: {content}"
    );
}

#[then(regex = r#"^the served ([a-z-]+) instruction does not contain "([^"]+)"$"#)]
fn served_instruction_does_not_contain(world: &mut GivnWorld, id: String, text: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = served_instruction(env, &id);
    assert!(
        !content.contains(&text),
        "the served {id} instruction must not contain {text:?}: {content}"
    );
}

// --- serve-extended-rules: extension recording and ordering assertions ------

/// Default rule the recording fixture writes for the proposal instruction.
pub(crate) const RECORDED_EXTENSION_RULE: &str = "Keep proposals under 500 words";

/// The built-in release-note rule of the proposal instruction: the marker the
/// contradiction scenario asserts beside the project's rule.
pub(crate) const BUILTIN_RELEASE_NOTE_RULE: &str =
    "carries the release note as its message subject";

/// Record the built-in served content of `id` before an extension exists, for
/// the ordering assertions.
pub(crate) fn record_builtin_content(env: &common::TestEnv, id: &str, content: &[u8]) {
    let dir = extension_fixture_dir(env).join("builtin");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), content)
        .unwrap_or_else(|error| panic!("record the built-in content of {id}: {error}"));
}

/// The recorded built-in served content of `id`.
pub(crate) fn recorded_builtin_content(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("builtin").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no built-in content recorded for {id}: {error}"))
}

/// The `## Project extension` section of a served content, panicking when the
/// section is absent (the shared helper used by the serving assertions).
pub(crate) fn marked_extension_section<'a>(id: &str, content: &'a str) -> (&'a str, usize) {
    let at = content.find(EXTENSION_SECTION_MARKER).unwrap_or_else(|| {
        panic!("the served {id} content carries no marked extension section: {content}")
    });
    (&content[at..], at)
}

#[given("the project records an extension for the proposal instruction")]
fn project_records_proposal_extension(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // Capture the built-in served content before the extension exists, so the
    // ordering assertions compare against the same request path.
    let builtin = request_instruction(env, "proposal");
    assert!(
        builtin.status.success(),
        "givn instructions proposal failed: {}",
        String::from_utf8_lossy(&builtin.stderr)
    );
    record_builtin_content(env, "proposal", &builtin.stdout);
    crate::steps::instruction_extension_steps::write_extensions(
        env,
        &format!("  proposal:\n    - {RECORDED_EXTENSION_RULE}\n"),
    );
    record_expected_rule(env, "proposal", RECORDED_EXTENSION_RULE);
    // The upgrade scenarios need a clean checkout; the fixture's own state
    // directory travels with the fixture commit.
    common::commit_fixture(env);
}

#[given(regex = r#"^the extension carries the rule "([^"]+)"$"#)]
fn extension_carries_the_rule(world: &mut GivnWorld, rule: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::instruction_extension_steps::replace_extensions(
        env,
        &format!("  proposal:\n    - {rule}\n"),
    );
    record_expected_rule(env, "proposal", &rule);
}

/// The served text with the trailing skill-routing line removed: both the
/// built-in capture and the extended request carry it, and it is not rule
/// content.
pub(crate) fn without_skill_route(content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    if lines
        .last()
        .is_some_and(|line| line.starts_with("Skill to load now:"))
    {
        lines.pop();
    }
    lines.join("\n")
}

#[then("every built-in rule of the proposal instruction remains in the served content")]
fn every_builtin_rule_remains(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let builtin = without_skill_route(&recorded_builtin_content(env, "proposal"));
    let content = without_skill_route(&recorded_stdout(env, "proposal"));
    assert!(
        content.starts_with(&builtin),
        "the served proposal content no longer carries every built-in rule \
         (the built-in content is not the served prefix): {content}"
    );
}

#[then("the built-in rules appear before the marked extension section")]
fn builtin_rules_appear_before_the_extension_section(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let builtin = without_skill_route(&recorded_builtin_content(env, "proposal"));
    let content = without_skill_route(&recorded_stdout(env, "proposal"));
    let (_, marker_at) = marked_extension_section("proposal", &content);
    let builtin_at = content.find(&builtin).unwrap_or_else(|| {
        panic!("the served proposal content does not carry the built-in rules: {content}")
    });
    assert!(
        builtin_at < marker_at,
        "the built-in rules appear after the marked extension section: {content}"
    );
}

#[then("the contradicting project rule appears beside the built-in release-note rule")]
fn contradicting_rule_appears_beside_the_builtin_release_note_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let (section, _) = marked_extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing {rule:?}: {content}"
    );
    assert!(
        content.contains(BUILTIN_RELEASE_NOTE_RULE),
        "the served proposal content is missing the built-in release-note rule \
         ({BUILTIN_RELEASE_NOTE_RULE:?}) beside {rule:?}: {content}"
    );
}

// --- serve-extended-rules: machine-readable serving --------------------------

/// The parsed JSON response of the last `givn instructions --json` request.
fn json_response(world: &GivnWorld) -> serde_json::Value {
    let output = world
        .last_output
        .as_ref()
        .expect("the JSON instructions request was not made");
    assert!(
        output.status.success(),
        "givn instructions --json failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "the instruction response is not JSON ({error}): {}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}

#[when("the agent requests the proposal instructions as JSON through the Givn CLI")]
fn agent_requests_proposal_as_json(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change = active_change_id(env);
    let output = common::run_givn(
        env,
        &[
            "instructions",
            "proposal",
            "--change",
            change.as_str(),
            "--json",
        ],
    );
    world.last_output = Some(output);
}

#[then("the response identifies the artifact and its generated paths and prerequisites")]
fn response_identifies_the_artifact(world: &mut GivnWorld) {
    let value = json_response(world);
    assert_eq!(
        value["id"], "proposal",
        "the response does not identify the proposal artifact: {value}"
    );
    let generates = value["generates"]
        .as_array()
        .unwrap_or_else(|| panic!("the response carries no generated paths array: {value}"));
    assert!(
        generates.iter().any(|path| path
            .as_str()
            .is_some_and(|path| path.ends_with("proposal.md"))),
        "the response does not name the proposal path it generates: {value}"
    );
    assert!(
        value["requires"].is_array(),
        "the response carries no prerequisites array: {value}"
    );
}

#[then("the response carries the same instruction content as the text path")]
fn response_carries_the_same_instruction_content(world: &mut GivnWorld) {
    let instruction = json_response(world)["instruction"]
        .as_str()
        .unwrap_or_else(|| panic!("the response carries no instruction text"))
        .to_string();
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let text = without_skill_route(&served_instruction(env, "proposal"));
    assert_eq!(
        text, instruction,
        "the machine-readable instruction differs from the text path"
    );
}

#[then("the response carries the project's rule in the marked extension section")]
fn response_carries_the_project_rule_in_the_extension_section(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let value = json_response(world);
    let instruction = value["instruction"]
        .as_str()
        .unwrap_or_else(|| panic!("the response carries no instruction text"));
    let (section, _) = marked_extension_section("proposal", instruction);
    let rule = expected_rule(env, "proposal");
    assert!(
        section.contains(&rule),
        "the response's extension section is missing {rule:?}: {instruction}"
    );
}

// --- instruction-extension serving fixtures ---------------------------------

/// Fixture directory the instruction-extension steps use to record the
/// expected rules and the raw guidance outputs.
pub(crate) const EXTENSION_FIXTURE_DIR: &str = ".givn-extension-fixture";

/// The marked section the served extension appends.
pub(crate) const EXTENSION_SECTION_MARKER: &str = "## Project extension";

pub(crate) fn extension_fixture_dir(env: &common::TestEnv) -> std::path::PathBuf {
    env.path(EXTENSION_FIXTURE_DIR)
}

/// Record the rule the extension scenarios expect the served content of `id`
/// to carry in the marked extension section.
pub(crate) fn record_expected_rule(env: &common::TestEnv, id: &str, rule: &str) {
    let dir = extension_fixture_dir(env).join("rules");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), format!("{rule}\n"))
        .unwrap_or_else(|error| panic!("record the expected rule for {id}: {error}"));
}

/// Record the rule the extension scenarios expect the served content of `id`
/// NOT to carry (the earlier of two entries for one instruction key).
pub(crate) fn record_excluded_rule(env: &common::TestEnv, id: &str, rule: &str) {
    let dir = extension_fixture_dir(env).join("excluded");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), format!("{rule}\n"))
        .unwrap_or_else(|error| panic!("record the excluded rule for {id}: {error}"));
}

/// The rule recorded for `id` by a fixture step.
pub(crate) fn expected_rule(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("rules").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no expected rule recorded for {id}: {error}"))
        .trim()
        .to_string()
}

/// The rule a fixture step recorded as excluded for `id`.
pub(crate) fn excluded_rule(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("excluded").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no excluded rule recorded for {id}: {error}"))
        .trim()
        .to_string()
}

/// Run `givn instructions <id>` — with `--change <active>` for manifest
/// artifact ids — and record stdout, stderr, and the exit code for the
/// instruction-extension assertions.
pub(crate) fn request_instruction(env: &common::TestEnv, id: &str) -> Output {
    let mut args = vec!["instructions", id];
    let change;
    if MANIFEST_ARTIFACT_IDS.contains(&id) {
        change = active_change_id(env);
        args.push("--change");
        args.push(change.as_str());
    }
    let output = common::run_givn(env, &args);
    let dir = extension_fixture_dir(env).join("served");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(format!("{id}.stdout")), &output.stdout).expect("record served stdout");
    fs::write(dir.join(format!("{id}.stderr")), &output.stderr).expect("record served stderr");
    fs::write(
        dir.join(format!("{id}.status")),
        output.status.code().unwrap_or(-1).to_string(),
    )
    .expect("record served status");
    output
}

/// The recorded served stdout of `id`.
pub(crate) fn recorded_stdout(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env)
        .join("served")
        .join(format!("{id}.stdout"));
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no served output recorded for {id}: {error}"))
}

/// The instruction ids with a recorded served output, in id order.
pub(crate) fn recorded_served_ids(env: &common::TestEnv) -> Vec<String> {
    let dir = extension_fixture_dir(env).join("served");
    let mut ids: Vec<String> = fs::read_dir(&dir)
        .unwrap_or_else(|error| {
            panic!(
                "no guidance request was recorded ({}): {error}",
                dir.display()
            )
        })
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".stdout").map(str::to_string)
        })
        .collect();
    ids.sort();
    assert!(!ids.is_empty(), "no guidance request was recorded");
    ids
}

/// The recorded served stderr of `id`.
pub(crate) fn recorded_stderr(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env)
        .join("served")
        .join(format!("{id}.stderr"));
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no served stderr recorded for {id}: {error}"))
}

/// The marked extension section of a served content, panicking when the
/// section is absent.
pub(crate) fn extension_section<'a>(id: &str, content: &'a str) -> &'a str {
    content
        .split_once(EXTENSION_SECTION_MARKER)
        .unwrap_or_else(|| {
            panic!("the served {id} content carries no marked extension section: {content}")
        })
        .1
}

// The alternation lists exactly the ids this delta's generic serving step is
// asked for. Two pre-existing literal bindings own the `specs`
// (`fix_domain_modeling_skill_trigger_steps.rs`) and `design`
// (`fix_domain_modeling_skill_trigger_e2e_steps.rs`) sentences; a broad
// `([a-z-]+)` pattern would make every `specs` request an ambiguous step match,
// and Rust's regex crate has no negative lookahead.
#[when(
    regex = r"^the agent requests the (proposal|dialog|ideation|explore) (?:instructions|policy) through the Givn CLI$"
)]
fn agent_requests_instruction(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let output = request_instruction(env, &id);
    world.last_output = Some(output);
}

#[then("the served content carries the changed rule")]
fn served_content_carries_the_changed_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let section = extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing {rule:?}: {content}"
    );
}

#[then("the served content is the built-in rules alone")]
fn served_content_is_the_builtin_rules_alone(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = recorded_stdout(env, "proposal");
    assert!(
        !content.contains(EXTENSION_SECTION_MARKER),
        "the served proposal content carries an extension section: {content}"
    );
    assert!(
        content.contains("proposed by"),
        "the served proposal content is missing the built-in proposal rules: {content}"
    );
}

#[then("the served content carries the built-in proposal rules")]
fn served_content_carries_the_builtin_proposal_rules(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = recorded_stdout(env, "proposal");
    assert!(
        content.contains("proposed by"),
        "the served proposal content is missing the built-in proposal rules: {content}"
    );
}

/// Built-in rule marker of the specs instruction: the canonical policy line
/// the served specs content carries.
const BUILTIN_SPECS_RULE: &str = "Primary assertion rule";

#[then("the served content carries the built-in specs rules")]
fn served_content_carries_the_builtin_specs_rules(world: &mut GivnWorld) {
    // The specs request is bound by the pre-existing specific step
    // (`fix_domain_modeling_skill_trigger_steps.rs`), which captures the
    // output in `last_output`; the fixture recordings are not written for it.
    let output = world
        .last_output
        .as_ref()
        .expect("the specs instructions were not requested first");
    assert!(
        output.status.success(),
        "the specs instructions request failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let content = String::from_utf8_lossy(&output.stdout);
    assert!(
        content.contains(BUILTIN_SPECS_RULE),
        "the served specs content is missing the built-in specs rules \
         ({BUILTIN_SPECS_RULE:?}): {content}"
    );
}

#[then("the served content carries no extension section")]
fn served_content_carries_no_extension_section(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let mut checked = false;
    if let Some(output) = world.last_output.as_ref() {
        let content = String::from_utf8_lossy(&output.stdout);
        assert!(
            !content.contains(EXTENSION_SECTION_MARKER),
            "the served content carries an extension section: {content}"
        );
        checked = true;
    }
    if extension_fixture_dir(env).join("served").exists() {
        for id in recorded_served_ids(env) {
            let content = recorded_stdout(env, &id);
            assert!(
                !content.contains(EXTENSION_SECTION_MARKER),
                "the served {id} content carries an extension section: {content}"
            );
            checked = true;
        }
    }
    assert!(
        checked,
        "no served content was requested for this assertion"
    );
}

#[then("the served content carries the rule from the last entry")]
fn served_content_carries_the_last_entry_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = expected_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    let section = extension_section("proposal", &content);
    assert!(
        section.contains(&rule),
        "the served proposal extension section is missing the last entry's rule {rule:?}: {content}"
    );
}

#[then("the served content does not carry the rule from the earlier entry")]
fn served_content_does_not_carry_the_earlier_entry_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let rule = excluded_rule(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    assert!(
        !content.contains(&rule),
        "the served proposal content carries the earlier entry's rule {rule:?}: {content}"
    );
}

#[then("each served content carries its project's rule in the marked extension section")]
fn each_served_content_carries_its_project_rule(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let ids = recorded_served_ids(env);
    assert!(
        ids.len() > 1,
        "expected more than one served content, got {ids:?}"
    );
    for id in ids {
        let rule = expected_rule(env, &id);
        let content = recorded_stdout(env, &id);
        let section = extension_section(&id, &content);
        assert!(
            section.contains(&rule),
            "the served {id} extension section is missing {rule:?}: {content}"
        );
    }
}
