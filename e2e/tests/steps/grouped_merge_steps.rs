use crate::harness as common;
use crate::steps::GivnWorld;
use crate::steps::archive_proof_support as support;
use cucumber::{given, then, when};
use std::io::Write;

fn write_passing_artifacts(change_dir: &std::path::Path) {
    for file in [
        "design.md",
        "design-review.md",
        "tasks.md",
        "review.md",
        "arc42.md",
    ] {
        let mut f = std::fs::File::create(change_dir.join(file)).expect("create artifact");
        match file {
            "design-review.md" => writeln!(f, "DESIGN-REVIEW: PASS").ok(),
            "review.md" => writeln!(f, "README-IMPACT: none\n\nRETIRE: only-cap — retiring its last capability\n\nREVIEW: PASS").ok(),
            "tasks.md" => writeln!(f, "- [x] all done\n- [x] COMMIT: a1b2c3d4 -- fixture").ok(),
            "arc42.md" => writeln!(f, "## Status\n\nSTATUS: DONE").ok(),
            _ => writeln!(f, "# Design").ok(),
        };
    }
}

const ROUTED_PROPOSAL: &str = "\
# Proposal: {ID}

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| {ID} | (fixture) | `NEW in {GROUP}` | fixture |
";

fn grouped_commands(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    support::write_result_runner(env, "pass");
    support::configure_commands(env);
    support::disable_coverage(env);
}

fn commit_and_refresh(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    common::commit_fixture(env);
    support::refresh_change_receipt(world, id);
    world.last_change_id = Some(id.to_string());
}

fn write_usecase_document(dir: &std::path::Path, usecase: &str, cap: &str, scenario: &str) {
    std::fs::create_dir_all(dir).expect("create use-case dir");
    let document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", usecase)
        .replace("<capability-id>", cap)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", scenario);
    std::fs::write(dir.join("usecase.md"), document).expect("write usecase");
}

fn write_supported_delta(change_dir: &std::path::Path, group: &str, cap: &str) {
    let dir = change_dir.join("specs").join(group);
    std::fs::create_dir_all(&dir).expect("create delta dir");
    std::fs::write(
        dir.join(format!("{cap}.feature")),
        format!(
            "@givn.delta @{cap} @e2e\nFeature: {cap} capability\n\n  Background: Shared session\n    Given a signed-in operator\n      \"\"\"json\n      {{\"role\": \"operator\"}}\n      \"\"\"\n    And the mapping\n      | key | value |\n      | mode | strict |\n\n  @givn.added @fast\n  Scenario Outline: {cap} reads\n    Given payload <name>\n      \"\"\"json\n      {{\"name\": \"<name>\"}}\n      \"\"\"\n    And mapping\n      | key | value |\n      | name | <name> |\n    Then visible\n\n    @api\n    Examples: First\n      | name |\n      | alpha |\n      | beta |\n\n    @slow\n    Examples: Second\n      | name |\n      | gamma |\n"
        ),
    )
    .expect("write supported delta");
    write_usecase_document(&dir, group, cap, &format!("{cap} reads"));
}

#[given(
    regex = r#"^a givn project with a completed change declaring a capability NEW in "([^"]+)"$"#
)]
fn project_with_new_capability(world: &mut GivnWorld, group: String) {
    world.env = Some(common::make_init_root());
    grouped_commands(world);
    let env = world.env.as_ref().unwrap();
    let out = common::run_givn(env, &["new", "grouped-new"]);
    assert!(out.status.success());
    let change_dir = env.path("givn/changes/grouped-new");
    std::fs::write(
        change_dir.join("proposal.md"),
        ROUTED_PROPOSAL
            .replace("{ID}", "grouped-new")
            .replace("{GROUP}", &group),
    )
    .expect("write proposal");
    write_passing_artifacts(&change_dir);
    write_supported_delta(&change_dir, &group, "grouped-new");
}

#[given(
    "its supported delta feature has Background setup, inherited execution tags and a Scenario Outline with a docstring, table and Examples"
)]
fn delta_has_supported_structure(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let content = std::fs::read_to_string(
        env.path("givn/changes/grouped-new/specs/some-group/grouped-new.feature"),
    )
    .expect("read delta");
    for needle in ["Background:", "@e2e", "Scenario Outline:", "Examples:"] {
        assert!(content.contains(needle), "delta lacks {needle:?}");
    }
    assert!(content.contains("\"\"\"json"));
}

#[given("all applicable archive verification passes")]
fn archive_verification_passes(world: &mut GivnWorld) {
    commit_and_refresh(world, "grouped-new");
}

#[when(regex = r#"^the consumer invokes `givn archive --change ([^`]+)`$"#)]
fn consumer_archives(world: &mut GivnWorld, id: String) {
    let out = support::run_archive(world, &id);
    world.last_change_id = Some(id);
    world.last_output = Some(out);
}

#[then("the command succeeds and terminal output reports a proven archive")]
fn succeeds_and_reports_proven(world: &mut GivnWorld) {
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(
        out.status.success(),
        "archive failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("archived successfully") || stdout.contains("Archive proven"),
        "stdout does not report the archive: {stdout}"
    );
    let id = world.last_change_id.clone().expect("change id");
    assert_eq!(support::receipt_status(world, &id), "proven");
}

#[then(
    regex = r#"^the feature is published under "([^"]+)" rather than a capability-named use-case directory$"#
)]
fn published_under_group(world: &mut GivnWorld, group: String) {
    let root = support::root(world);
    let id = world.last_change_id.clone().expect("change id");
    let expected = root.join(&group).join(format!("{id}.feature"));
    assert!(
        expected.is_file(),
        "published feature missing at {}; stdout={} stderr={}",
        expected.display(),
        crate::steps::stdout(world),
        crate::steps::stderr(world)
    );
    assert!(
        !root.join(format!("givn/specs/{id}")).exists(),
        "capability-named use-case directory was created"
    );
}

#[then(
    "the published feature retains its complete supported executable content and tag inheritance"
)]
fn published_content_retained(world: &mut GivnWorld) {
    let root = support::root(world);
    let id = world.last_change_id.clone().expect("change id");
    let path = root.join(format!("givn/specs/some-group/{id}.feature"));
    let text = std::fs::read_to_string(&path).expect("read published feature");
    let feature =
        gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default()).expect("parse feature");
    assert!(feature.tags.iter().any(|tag| tag == "e2e"));
    let background = feature.background.as_ref().expect("background kept");
    assert_eq!(background.steps.len(), 2);
    assert!(background.steps[0].docstring().is_some());
    assert!(background.steps[1].table().is_some());
    let outline = &feature.scenarios[0];
    assert_eq!(outline.keyword, "Scenario Outline");
    assert!(outline.tags.iter().any(|tag| tag == "fast"));
    assert!(outline.steps[0].docstring().is_some());
    assert!(outline.steps[1].table().is_some());
    assert_eq!(outline.examples.len(), 2);
    assert!(!text.contains("@givn.added"));
    grouped_transcript_captured(world);
}

fn grouped_transcript_captured(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("archive ran");
    let title =
        "Consumer archives a new capability and the merged spec lands in its declared use case";
    let slug = crate::harness::timing::scenario_slug(title);
    let dir = crate::harness::binary::product_root()
        .join("target/prepare-0-9-10-evidence")
        .join(&slug);
    std::fs::create_dir_all(&dir).expect("create transcript dir");
    let id = world.last_change_id.clone().expect("change id");
    std::fs::write(
        dir.join("transcript.txt"),
        format!(
            "Scenario: {title}\nargv: givn archive --change {id}\nexit code: {}\n--- stdout ---\n{}\n--- stderr ---\n{}\n",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ),
    )
    .expect("write transcript");
}
