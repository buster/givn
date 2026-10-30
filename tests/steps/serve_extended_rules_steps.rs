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

/// The fixture's single active change id, when exactly one exists.
fn single_active_change(env: &common::TestEnv) -> Option<String> {
    let dir = env.path("givn/changes");
    let mut ids: Vec<String> = std::fs::read_dir(&dir)
        .ok()?
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
    (ids.len() == 1).then(|| ids.remove(0))
}

/// The fixture's single active change id, requiring exactly one.
fn active_change_id(env: &common::TestEnv) -> String {
    single_active_change(env).unwrap_or_else(|| {
        panic!(
            "expected exactly one active change in the fixture under {}",
            env.path("givn/changes").display()
        )
    })
}

/// Run `givn instructions <id>` in the fixture and return its stdout. Manifest
/// artifact ids get a `--change` context: the fixture's single active change
/// when one exists, otherwise the requested id itself — artifact instructions
/// are not change-specific and the CLI validates the context by shape, which
/// the no-change sync fixtures (the all-targets fixture) rely on.
pub(crate) fn served_instruction(env: &common::TestEnv, id: &str) -> String {
    let mut args = vec!["instructions", id];
    let change;
    if MANIFEST_ARTIFACT_IDS.contains(&id) {
        change = single_active_change(env).unwrap_or_else(|| id.to_string());
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

/// The exact bytes of the extension block a recording step wrote, for the
/// byte-for-byte preservation comparison across an upgrade.
pub(crate) fn record_extension_block(env: &common::TestEnv, block: &str) {
    let dir = extension_fixture_dir(env);
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join("extension-block"), block).expect("record the extension block");
}

/// The recorded extension block bytes.
pub(crate) fn recorded_extension_block(env: &common::TestEnv) -> String {
    let path = extension_fixture_dir(env).join("extension-block");
    fs::read_to_string(&path).expect("the fixture recorded the extension block")
}

/// The active `extensions:` block of the project's config as text.
pub(crate) fn config_extension_block(env: &common::TestEnv) -> String {
    let config = fs::read_to_string(env.path("givn/config.yaml")).expect("read givn/config.yaml");
    let mut block: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in config.lines() {
        if !inside {
            if line == "extensions:" {
                inside = true;
                block.push(line);
            }
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            block.push(line);
        } else {
            break;
        }
    }
    assert!(
        !block.is_empty(),
        "the project config carries no active extensions block:\n{config}"
    );
    format!("{}\n", block.join("\n"))
}

#[given("the project records an extension for the proposal instruction")]
fn project_records_proposal_extension(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // Capture the built-in served content before the extension exists, so the
    // ordering assertions compare against the same request path. Only the
    // fixtures with a change context resolve an artifact instruction; the
    // upgrade fixtures carry no change and need no capture.
    let changes_dir = env.path("givn/changes");
    let single_change = fs::read_dir(&changes_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .count()
        })
        .unwrap_or(0)
        == 1;
    if single_change {
        let builtin = request_instruction(env, "proposal");
        assert!(
            builtin.status.success(),
            "givn instructions proposal failed: {}",
            String::from_utf8_lossy(&builtin.stderr)
        );
        record_builtin_content(env, "proposal", &builtin.stdout);
    }
    let block = format!("extensions:\n  proposal:\n    - {RECORDED_EXTENSION_RULE}\n");
    crate::steps::instruction_extension_steps::write_extensions(
        env,
        &format!("  proposal:\n    - {RECORDED_EXTENSION_RULE}\n"),
    );
    record_extension_block(env, &block);
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

// --- serve-extended-rules: project-owned replacement ------------------------

/// The distinctive marker of the project-owned replacement text.
pub(crate) const REPLACEMENT_MARKER: &str = "REPLACEMENT-MARKER";

/// The project-owned replacement of the proposal instruction facet.
pub(crate) const PROJECT_REPLACEMENT_TEXT: &str = "\
# Instruction: proposal (project-owned replacement)

REPLACEMENT-MARKER: follow the house proposal template.
";

/// Record the exact bytes of the project-owned replacement of `id`.
pub(crate) fn record_replacement(env: &common::TestEnv, id: &str, bytes: &[u8]) {
    let dir = extension_fixture_dir(env).join("replacement");
    fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
    fs::write(dir.join(id), bytes)
        .unwrap_or_else(|error| panic!("record the replacement of {id}: {error}"));
}

/// The recorded bytes of the project-owned replacement of `id`.
pub(crate) fn recorded_replacement(env: &common::TestEnv, id: &str) -> String {
    let path = extension_fixture_dir(env).join("replacement").join(id);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no replacement recorded for {id}: {error}"))
}

#[given("a project-owned replacement for the proposal instruction")]
fn project_owned_replacement_for_proposal(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/artifacts/proposal/instructions.md");
    fs::create_dir_all(path.parent().expect("replacement parent directory"))
        .expect("create the replacement directory");
    fs::write(&path, PROJECT_REPLACEMENT_TEXT).expect("write the project-owned replacement");
    record_replacement(env, "proposal", PROJECT_REPLACEMENT_TEXT.as_bytes());
    // The upgrade scenario needs the replacement committed to keep the
    // checkout clean; a plain request fixture is unaffected by the commit.
    common::commit_fixture(env);
}

#[then("the served content carries the replacement text")]
fn served_content_carries_the_replacement_text(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let replacement = recorded_replacement(env, "proposal");
    let content = recorded_stdout(env, "proposal");
    assert!(
        content.contains(replacement.trim_end()),
        "the served proposal content does not carry the project-owned replacement: {content}"
    );
}

// --- serve-extended-rules: the request path never mutates --------------------

/// Every project file as (relative path, bytes), excluding the version-control
/// store and the fixture's own state directory. Sorted by path.
pub(crate) fn project_files(env: &common::TestEnv) -> Vec<(String, Vec<u8>)> {
    let mut files = Vec::new();
    collect_project_files(&env.root(), &env.root(), &mut files);
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

fn collect_project_files(
    root: &std::path::Path,
    dir: &std::path::Path,
    out: &mut Vec<(String, Vec<u8>)>,
) {
    let mut entries: Vec<std::path::PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            if name == ".git"
                || path == root.join(EXTENSION_FIXTURE_DIR)
                || crate::steps::development_record_steps::is_record_dir(root, &path)
            {
                continue;
            }
            collect_project_files(root, &path, out);
        } else {
            let relative = path
                .strip_prefix(root)
                .expect("project file under the project root");
            out.push((
                relative.to_string_lossy().into_owned(),
                fs::read(&path).unwrap_or_default(),
            ));
        }
    }
}

#[when("the agent requests the instructions for every served step")]
fn agent_requests_every_served_step(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    // The no-mutation witness is taken before the first request; the fixture's
    // own state directory is excluded so the witness itself never counts.
    let snapshot = serde_json::to_vec(&project_files(env)).expect("serialize the project snapshot");
    fs::write(
        extension_fixture_dir(env).join("project-snapshot"),
        snapshot,
    )
    .expect("record the pre-request project snapshot");

    // Every instruction asset the binary serves in this Givn version, so the
    // request path is exercised across the whole served corpus.
    let assets_dir = common::repo_path("assets/instructions");
    let mut ids: Vec<String> = fs::read_dir(&assets_dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", assets_dir.display()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".md").map(str::to_string)
        })
        .collect();
    ids.sort();
    assert!(!ids.is_empty(), "no served instruction assets found");

    for id in &ids {
        let mut args = vec!["instructions", id.as_str()];
        let change;
        if MANIFEST_ARTIFACT_IDS.contains(&id.as_str()) {
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
    }
}

#[then("no project file changed")]
fn no_project_file_changed(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let raw = fs::read(extension_fixture_dir(env).join("project-snapshot"))
        .expect("the request step records the pre-request snapshot");
    let before: Vec<(String, Vec<u8>)> =
        serde_json::from_slice(&raw).expect("the recorded snapshot is valid");
    let after = project_files(env);
    let mut changed: Vec<String> = Vec::new();
    for (path, bytes) in &before {
        match after.iter().find(|(other, _)| other == path) {
            Some((_, other_bytes)) if other_bytes == bytes => {}
            _ => changed.push(path.clone()),
        }
    }
    for (path, _) in &after {
        if !before.iter().any(|(other, _)| other == path) {
            changed.push(path.clone());
        }
    }
    assert!(
        changed.is_empty(),
        "the request path changed project files: {changed:?}"
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

#[then("the served content carries the project's rule in the marked extension section")]
fn served_content_carries_the_project_rule_in_the_extension_section(world: &mut GivnWorld) {
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

// --- prepare-0-9-10: behavior-batch, RED, REFACTOR and cleanup guidance ----

fn normalized_guidance(world: &GivnWorld) -> String {
    world
        .additional_output
        .as_deref()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn batch_guidance(world: &mut GivnWorld) {
    let env = world.env.get_or_insert_with(common::make_init_root);
    let output = common::run_givn(env, &["new", "serve-extended-rules"]);
    assert!(
        output.status.success(),
        "create the served guidance fixture change: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let design = served_instruction(env, "design");
    let steps = served_instruction(env, "steps");
    world.additional_output = Some(format!("{design}\n{steps}"));
}

#[given(
    "a coherent named behavior batch is planned in tasks with scenarios validation and an intended commit boundary"
)]
fn coherent_behavior_batch_planned(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[when("the author requests the implementation and steps guidance")]
fn author_requests_implementation_and_steps(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "implementation and steps guidance is empty"
    );
}

#[then("tests and bindings must run before production edits for new behavior")]
fn tests_run_before_production_edits(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("Tests and bindings must run before production edits for new behavior"),
        "guidance does not require tests and bindings before production edits"
    );
}

#[then(
    "the observed RED must be a meaningful behavioral failure rather than missing bindings syntax registration fixtures or environment failure"
)]
fn observed_red_is_behavioral(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "observed RED is a meaningful behavioral failure rather than missing bindings, syntax, registration, fixtures, or environment failure"
        ),
        "guidance accepts a non-behavioral RED"
    );
}

#[then(
    "the actual command and short observed failure are recorded compactly in the batch task as they occur"
)]
fn red_evidence_recorded_compactly(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Record the actual command and a short observed failure compactly in the batch task as they occur"
        ),
        "guidance does not require contemporaneous compact RED evidence"
    );
}

#[then("minimum implementation is followed by actual GREEN and REFACTOR checks recorded briefly")]
fn green_and_refactor_recorded(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "minimum implementation is followed by actual GREEN and REFACTOR checks recorded briefly"
        ),
        "guidance does not require actual GREEN and REFACTOR checks"
    );
}

#[given("a distinct requirement already holds before its regression test is written")]
fn distinct_requirement_already_holds(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[when("the author reads the implementation guidance for an immediately passing test")]
fn author_reads_immediately_passing_guidance(world: &mut GivnWorld) {
    assert!(!normalized_guidance(world).is_empty());
}

#[then("the requirement is explicitly recorded as existing behavior protected")]
fn requirement_recorded_as_existing_behavior(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("existing behavior protected"),
        "guidance does not name an immediately passing requirement as protected existing behavior"
    );
}

#[then("no production edit is made for that requirement")]
fn no_production_edit_for_passing_requirement(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("make no production edit for that requirement"),
        "guidance still requires a production edit for a passing requirement"
    );
}

#[then(
    "another requirement in the same coherent batch may still require behavioral RED and production edits"
)]
fn other_requirement_may_still_require_red(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Another requirement in the same coherent batch may still require behavioral RED and production edits"
        ),
        "guidance lets one passing test exempt the whole batch"
    );
}

#[given("production behavior was edited before the requirement's behavioral test was run")]
fn production_edited_before_test(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[when("the author reads the test-first evidence guidance")]
fn author_reads_test_first_evidence_guidance(world: &mut GivnWorld) {
    assert!(!normalized_guidance(world).is_empty());
}

#[then("the skipped RED must be admitted")]
fn skipped_red_is_admitted(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("admit the skipped RED"),
        "guidance does not admit a skipped RED"
    );
}

#[then(
    "backdated evidence history rewriting and deliberately breaking finished behavior are forbidden"
)]
fn backdating_and_rewriting_forbidden(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Never backdate evidence, rewrite history, or deliberately break finished behavior"
        ),
        "guidance permits backdating or history rewriting"
    );
}

#[then("mutation or discrimination tests cannot be claimed as historical RED")]
fn mutation_tests_not_historical_red(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("Mutation or discrimination tests are not claimed as historical RED"),
        "guidance claims mutation tests as historical RED"
    );
}

#[then(
    "separate RED GREEN and REFACTOR commits or retrospective phase-shaped history gates are not required"
)]
fn phase_commits_not_required(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Separate RED, GREEN, and REFACTOR commits and retrospective phase-shaped history gates are not required"
        ),
        "guidance requires phase-shaped commits"
    );
}

#[given("a behavior batch has completed its actual GREEN and REFACTOR checks")]
fn batch_completed_green_and_refactor(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[when("the author reads the batch completion guidance")]
fn author_reads_batch_completion_guidance(world: &mut GivnWorld) {
    assert!(!normalized_guidance(world).is_empty());
}

#[then("the intended explicit-path diff is inspected before a coherent completed batch commit")]
fn explicit_path_diff_inspected(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Inspect the intended explicit-path diff before a coherent completed batch commit"
        ),
        "guidance does not inspect the explicit-path diff before committing"
    );
}

#[then("the actual revision is recorded before scenario tasks are completed")]
fn revision_recorded_before_tasks_complete(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("Record the actual reachable revision before scenario tasks are completed"),
        "guidance checks tasks before recording the revision"
    );
}

#[then("commit selections name the batch files explicitly rather than implicitly staging all work")]
fn commit_selections_are_explicit(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Commit selections name the batch files explicitly rather than implicitly staging all work"
        ),
        "guidance permits implicit staging of all work"
    );
}

#[then("no production diff is required for each scenario that protects existing behavior")]
fn no_per_scenario_production_diff(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "No production diff is required for each scenario that protects existing behavior"
        ),
        "guidance still requires a production diff per scenario"
    );
}

#[given("a behavior batch changes supported and unsupported components")]
fn supported_and_unsupported_batch(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[when("the author reads the REFACTOR guidance")]
fn author_reads_refactor_guidance(world: &mut GivnWorld) {
    assert!(!normalized_guidance(world).is_empty());
}

#[then("supported changed components receive one advisory outline inspection per batch")]
fn supported_components_receive_outline_inspection(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "inspect its advisory report once per batch for supported changed components and relevant neighbors"
        ),
        "guidance does not define one advisory outline inspection per batch"
    );
}

#[then(
    "the assessment discloses which languages and components the outline covers and which it omits"
)]
fn assessment_discloses_outline_coverage(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Disclose which languages and components the outline covers and which it omits"
        ),
        "guidance does not disclose outline coverage"
    );
}

#[then("unsupported components receive direct source assessment without forced outline analysis")]
fn unsupported_components_receive_direct_assessment(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Unsupported components receive direct source assessment using the same responsibility, interface, dependency-direction, and decomposition questions, without forced outline analysis"
        ),
        "guidance forces unsupported components through the outline"
    );
}

#[then(
    "the assessment asks whether responsibilities are cohesive public interfaces narrow dependencies correctly directed and decomposition understandable"
)]
fn assessment_asks_architecture_questions(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Ask whether responsibilities are cohesive, public interfaces narrow, dependencies correctly directed, and decomposition understandable"
        ),
        "REFACTOR guidance does not ask the architecture questions"
    );
}

#[then("suspected problems are confirmed against source before conclusions or edits")]
fn suspected_problems_confirmed_against_source(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("Confirm suspected problems against source before conclusions or edits"),
        "REFACTOR guidance accepts outline-only conclusions"
    );
}

#[then("structural call order is not represented as a complete runtime trace")]
fn call_order_is_not_runtime_trace(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("Structural call order is not a complete runtime trace"),
        "guidance overstates the outline as a runtime trace"
    );
}

#[then("uncertain or omitted edges limit the conclusions")]
fn uncertain_edges_limit_conclusions(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("uncertain or omitted edges limit the conclusions"),
        "guidance does not bound conclusions from uncertain edges"
    );
}

#[then(
    "a later structural edit is the reason to regenerate a stale outline rather than every small edit"
)]
fn structural_edit_regenerates_outline(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "a later structural edit is the reason to regenerate it, not every small edit"
        ),
        "guidance regenerates the outline for every edit"
    );
}

#[then("the report is reused until structural changes make its relevant evidence stale")]
fn report_reused_until_stale(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("Reuse the report until structural changes make its relevant evidence stale"),
        "guidance does not reuse the outline report"
    );
}

#[then("nonstructural edits alone do not require rerunning the outline")]
fn nonstructural_edits_do_not_rerun_outline(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("nonstructural edits alone do not require rerunning the outline"),
        "guidance reruns the outline for nonstructural edits"
    );
}

#[given("a behavior batch changes unsupported components")]
fn unsupported_components_only_batch(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[given("a behavior batch changes a unit whose outline cannot carry its story")]
fn unreadable_unit_batch(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[then(
    "the same responsibility interface dependency and decomposition questions are assessed directly against source"
)]
fn unsupported_same_questions_directly(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Unsupported components receive direct source assessment using the same responsibility, interface, dependency-direction, and decomposition questions"
        ),
        "unsupported components do not get the same questions from source"
    );
}

#[then("unavailable outline support is not failure or a reason for a waiver")]
fn unavailable_outline_not_failure(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains(
            "Unavailable outline support is not a failure and is not a reason for a waiver"
        ) || content
            .contains("Unavailable outline support is not failure or a reason for a waiver"),
        "guidance treats unavailable outline support as failure or waiver"
    );
}

#[then("no outline verdict metric waiver or forced reviewer context is required")]
fn no_outline_verdict_or_context(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    for marker in [
        "no blocking verdict",
        "metric waiver",
        "forced reviewer context",
    ] {
        assert!(
            content.contains(marker),
            "REFACTOR guidance still requires {marker:?}"
        );
    }
    assert!(
        !content.contains("OUTLINE: PASS"),
        "REFACTOR guidance still requires an outline verdict"
    );
}

#[then(
    "the outline must state the unit's responsibility and ordered call flow without bodies or comments"
)]
fn outline_states_responsibility_and_call_flow(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content
            .contains("State the unit's responsibility in one sentence and its ordered call flow")
            && content.contains("without bodies or comments"),
        "REFACTOR guidance does not require responsibility and call flow from the outline"
    );
}

#[then("unit type method parameter return and called-type names must state the domain operation")]
fn names_state_the_domain_operation(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "The unit and type names, method names, parameter names and types, return types, and called type names must state the domain operation in the project's vocabulary"
        ),
        "REFACTOR guidance does not require domain names on the outline"
    );
}

#[then("generic machinery names are readability findings")]
fn generic_machinery_names_are_findings(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("A generic or machinery name where a domain noun exists"),
        "REFACTOR guidance accepts generic machinery names"
    );
    assert!(
        normalized_guidance(world).contains("is a readability finding"),
        "REFACTOR guidance does not classify generic names as findings"
    );
}

#[then("metrics and classifications never overrule an unreadable outline")]
fn metrics_never_overrule_readability(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "a clean metric never excuses an unreadable outline, and no metric waives it"
        ),
        "REFACTOR guidance lets metrics overrule an unreadable outline"
    );
}

#[then("source confirms a suspected problem rather than substituting for the outline story")]
fn source_confirms_without_substituting(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "Source confirms a suspected problem; it never substitutes for the outline story"
        ),
        "REFACTOR guidance substitutes source reading for the outline story"
    );
}

#[then(
    "an unreadable changed unit is fixed in REFACTOR or recorded as a substantive review finding"
)]
fn unreadable_unit_is_fixed_or_recorded(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("fix it in REFACTOR")
            && content.contains("record a substantive review finding"),
        "REFACTOR guidance neither fixes nor records an unreadable unit"
    );
}

#[when("the author requests the tasks instruction")]
fn author_requests_tasks_instruction(world: &mut GivnWorld) {
    let env = world.env.get_or_insert_with(common::make_init_root);
    world.additional_output = Some(served_instruction(env, "tasks"));
}

#[then("the served guidance names givn instructions steps as the step mechanics")]
fn tasks_guidance_names_step_mechanics(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("Step mechanics: `givn instructions steps`")
            && content.contains("Read that instruction before writing or running any step"),
        "the tasks instruction does not route to the shared step mechanics"
    );
}

#[given("the author encounters a concrete cleanup opportunity while implementing agreed behavior")]
fn cleanup_opportunity_encountered(world: &mut GivnWorld) {
    batch_guidance(world);
}

#[when("the author reads the cleanup guidance")]
fn author_reads_cleanup_guidance(world: &mut GivnWorld) {
    assert!(!normalized_guidance(world).is_empty());
}

#[then(
    "the cleanup is made in a separate behavior-preserving refactoring commit with explicit paths and actual GREEN checks"
)]
fn cleanup_is_separate_green_commit(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "separate behavior-preserving refactoring commit with explicit paths and actual GREEN checks"
        ),
        "cleanup guidance does not require a separate behavior-preserving commit"
    );
}

#[then("the cleanup does not authorize speculative features or a broad abstraction rewrite")]
fn cleanup_does_not_authorize_scope(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("does not authorize speculative features or a broad abstraction rewrite"),
        "cleanup guidance permits speculative scope"
    );
}

#[given("a project is about to make its first production implementation")]
fn first_production_implementation(world: &mut GivnWorld) {
    let env = world.env.get_or_insert_with(common::make_init_root);
    world.additional_output = Some(served_instruction(env, "dev-principles"));
}

#[when("the author requests the development principles guidance")]
fn author_requests_development_principles(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "development principles guidance is empty"
    );
}

#[then(
    "the guidance requires cohesive responsibility modules narrow consumer contracts external adapters separate composition and explicit dependency direction"
)]
fn development_guidance_requires_modular_boundaries(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    for marker in [
        "cohesive responsibility modules",
        "narrow consumer contracts",
        "external adapters separate",
        "compose them in one explicit root",
        "dependency direction",
    ] {
        assert!(
            content.contains(marker),
            "development principles are missing {marker:?}"
        );
    }
}

#[then(
    "a narrow module API is acceptable unless isolation or substitution needs an explicit interface"
)]
fn narrow_module_api_is_acceptable(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("narrow module API is acceptable unless isolation or substitution needs an explicit interface"),
        "development principles demand an explicit interface everywhere"
    );
}

#[then("the guidance does not demand hypothetical providers or plugin registries")]
fn no_hypothetical_providers(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("do not invent hypothetical providers or plugin registries"),
        "development principles demand hypothetical providers"
    );
}

// --- start entry point ------------------------------------------------------

#[given("a Givn project with a project-owned explore instruction override")]
fn project_with_explore_override(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let dir = env.path("givn/artifacts/explore");
    std::fs::create_dir_all(&dir).expect("create explore override dir");
    std::fs::write(dir.join("instructions.md"), "# Project explore override\n")
        .expect("write explore override");
    world.env = Some(env);
}
