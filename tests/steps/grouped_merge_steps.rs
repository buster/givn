use std::io::Write;

use cucumber::{given, then};

use crate::common;
use crate::steps::GivnWorld;

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

fn write_delta(change_dir: &std::path::Path, group: &str, cap: &str) {
    let specs_dir = change_dir.join("specs").join(group);
    std::fs::create_dir_all(&specs_dir).expect("create change specs dir");
    std::fs::write(
        specs_dir.join(format!("{cap}.feature")),
        format!(
            "@givn.delta @{cap}\nFeature: {cap}\n\n  @givn.added\n  Scenario: {cap} merges\n    Given a placeholder step\n"
        ),
    )
    .expect("write delta feature");
}

fn write_change_usecase(change_dir: &std::path::Path, group: &str, cap: &str) {
    let usecase_dir = change_dir.join("specs").join(group);
    std::fs::create_dir_all(&usecase_dir).expect("create use-case dir");
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", group)
        .replace("<capability-id>", cap)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", &format!("{cap} merges"));
    std::fs::write(usecase_dir.join("usecase.md"), document).expect("write change usecase");
}

fn prepare_change(env: &common::TestEnv, id: &str, proposal: &str, group: &str) {
    let out = common::run_givn(env, &["new", id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let change_dir = env.path(&format!("givn/changes/{id}"));
    std::fs::write(change_dir.join("proposal.md"), proposal).expect("write proposal");
    write_passing_artifacts(&change_dir);
    write_delta(&change_dir, group, id);
    write_change_usecase(&change_dir, group, id);
}

const ROUTED_PROPOSAL: &str = "\
# Proposal: {ID}

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| {ID} | (fixture) | `NEW in {GROUP}` | fixture |
";

const UNROUTED_PROPOSAL: &str = "\
# Proposal: {ID}

No capability routing decisions were recorded for this change.
";

#[given(
    regex = r#"^a completed change "([^"]+)" whose proposal declares the capability NEW in "([^"]+)" and whose delta lives under specs/([^/]+)/$"#
)]
fn grouped_new_change(world: &mut GivnWorld, id: String, group: String, delta_group: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let proposal = ROUTED_PROPOSAL
        .replace("{ID}", &id)
        .replace("{GROUP}", &group);
    prepare_change(env, &id, &proposal, &delta_group);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a completed change "([^"]+)" whose proposal records no routing decision and whose delta lives under specs/([^/]+)/$"#
)]
fn unrouted_new_change(world: &mut GivnWorld, id: String, delta_group: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let proposal = UNROUTED_PROPOSAL.replace("{ID}", &id);
    prepare_change(env, &id, &proposal, &delta_group);
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a completed change "([^"]+)" with two deltas: one routed NEW in "([^"]+)" and one unrouted$"#
)]
fn mixed_routing_change(world: &mut GivnWorld, id: String, group: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let proposal = ROUTED_PROPOSAL
        .replace("{ID}", &id)
        .replace("{GROUP}", &group);
    // First delta: capability `<id>` under the declared group (routed).
    prepare_change(env, &id, &proposal, &group);
    // Second delta: a different capability with no routing decision.
    let change_dir = env.path(&format!("givn/changes/{id}"));
    write_delta(&change_dir, "unrouted-group", "unrouted-cap");
    world.last_change_id = Some(id);
}

#[given(
    regex = r#"^a givn project with a permanent use case "([^"]+)" whose only capability is "([^"]+)"$"#
)]
fn permanent_usecase_with_only_capability(world: &mut GivnWorld, usecase: String, cap: String) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().expect("env set above");
    // Archive pipeline fixtures: a passing result runner, the coverage
    // command pair and reports, and the complete addon set — the same shape
    // the enforce-post-merge-e2e-gate archive fixtures use.
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let commands_path = env.path("givn/commands.yaml");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"givn missing-e2e-runner\"
coverage:
  measure_command: \"./measure-coverage.sh\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
";
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    configure_retirement_policy(env);

    // The permanent use case: a valid narrative plus its only capability.
    let usecase_dir = env.path(&format!("givn/specs/{usecase}"));
    std::fs::create_dir_all(&usecase_dir).expect("create use-case dir");
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", &usecase)
        .replace("<capability-id>", &cap)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", &format!("{cap} works"))
        .replace(
            &format!(
                "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| {cap} | exercise the capability | {cap} works |\n"
            ),
            "- none\n",
        );
    std::fs::write(usecase_dir.join("usecase.md"), document).expect("write use-case narrative");
    std::fs::write(
        usecase_dir.join(format!("{cap}.feature")),
        format!("Feature: {cap}\n\n  Scenario: {cap} works\n    Given a placeholder step\n"),
    )
    .expect("write permanent capability spec");
}

#[given(regex = r#"^a completed change "([^"]+)" whose delta retires capability "([^"]+)"$"#)]
fn retiring_change(world: &mut GivnWorld, id: String, cap: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let out = common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let change_dir = env.path(&format!("givn/changes/{id}"));
    std::fs::write(
        change_dir.join("proposal.md"),
        crate::steps::fixture_routing_proposal(&cap),
    )
    .expect("write proposal");
    write_passing_artifacts(&change_dir);
    // The retirement delta: zero scenarios, capability tag matching the
    // permanent capability.
    let specs_dir = change_dir.join("specs").join(&cap);
    std::fs::create_dir_all(&specs_dir).expect("create change specs dir");
    std::fs::write(
        specs_dir.join(format!("{cap}.feature")),
        format!("@givn.delta @givn.retired @{cap}\nFeature: Retire {cap}\n"),
    )
    .expect("write retirement delta");
    world.last_change_id = Some(id);
}

#[then(regex = r#"^the tombstone "([^"]+)" records capability "([^"]+)"$"#)]
fn tombstone_records_capability(world: &mut GivnWorld, path: String, cap: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = std::fs::read_to_string(env.path(&path)).expect("read tombstone");
    assert!(
        content.contains(&cap),
        "tombstone {path} missing capability '{cap}':\n{content}"
    );
}

use crate::steps::archive_proof_support as support;
use cucumber::when;

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
    let document = givn::gherkin::usecase_md::usecase_md_template()
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
        givn::gherkin::parse::parse_supported_source(&text, &path).expect("parse feature");
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
    let slug = givn::integrity_hook::scenario_slug(title);
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

#[given("a completed change introduces a new capability without a declared routing decision")]
fn new_capability_without_routing(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    grouped_commands(world);
    let env = world.env.as_ref().unwrap();
    let proposal = UNROUTED_PROPOSAL.replace("{ID}", "flat-new");
    prepare_change(env, "flat-new", &proposal, "flat-new");
    commit_and_refresh(world, "flat-new");
}

#[then("the command exits non-zero and explains the missing NEW routing decision")]
fn explains_missing_new_routing(world: &mut GivnWorld) {
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("NEW in")
            || stderr.contains("routing decision")
            || stderr.contains("undeclared"),
        "stderr lacks routing explanation: {stderr}"
    );
}

#[then("the permanent capability is not created and the active change is not moved")]
fn no_permanent_and_active_kept(world: &mut GivnWorld) {
    let root = support::root(world);
    let id = world.last_change_id.clone().expect("change id");
    assert!(!root.join(format!("givn/specs/{id}")).exists());
    assert!(root.join(format!("givn/changes/{id}")).is_dir());
}

#[given(
    regex = r#"^a completed change has two deltas with one routed NEW in "([^"]+)" and one unrouted$"#
)]
fn two_deltas_one_unrouted(world: &mut GivnWorld, group: String) {
    world.env = Some(common::make_init_root());
    grouped_commands(world);
    let env = world.env.as_ref().unwrap();
    let proposal = ROUTED_PROPOSAL
        .replace("{ID}", "mixed-routing")
        .replace("{GROUP}", &group);
    prepare_change(env, "mixed-routing", &proposal, &group);
    let change_dir = env.path("givn/changes/mixed-routing");
    write_delta(&change_dir, "unrouted-group", "unrouted-cap");
    commit_and_refresh(world, "mixed-routing");
}

#[then("the command exits non-zero and names the missing routing decision")]
fn names_missing_routing(world: &mut GivnWorld) {
    let out = world.last_output.as_ref().expect("archive ran");
    assert!(!out.status.success(), "archive unexpectedly passed");
    let stderr = crate::steps::stderr(world);
    assert!(
        stderr.contains("unrouted-cap") || stderr.contains("routing") || stderr.contains("NEW in"),
        "stderr lacks missing routing decision: {stderr}"
    );
}

#[then("neither delta is merged and the active change is not moved")]
fn neither_delta_merged(world: &mut GivnWorld) {
    let root = support::root(world);
    assert!(
        !root
            .join("givn/specs/mixed-group/mixed-routing.feature")
            .exists()
    );
    assert!(
        !root
            .join("givn/specs/unrouted-group/unrouted-cap.feature")
            .exists()
    );
    assert!(root.join("givn/changes/mixed-routing").is_dir());
}

fn owner_proposal(routing: &str) -> String {
    let decision = if routing == "EXTEND" {
        "EXTEND shared-capability".to_string()
    } else {
        "NEW in new-group".to_string()
    };
    format!(
        "# Proposal: fixture\n\n## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| shared-capability | (fixture) | {decision} | fixture |\n"
    )
}

#[given("an otherwise valid completed change has a delta for capability \"shared-capability\"")]
fn owner_change_delta(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    grouped_commands(world);
    let env = world.env.as_ref().unwrap();
    let out = common::run_givn(env, &["new", "owner-demo"]);
    assert!(out.status.success());
    let change_dir = env.path("givn/changes/owner-demo");
    write_passing_artifacts(&change_dir);
    let delta_dir = change_dir.join("specs/usecase-owner");
    std::fs::create_dir_all(&delta_dir).expect("create delta dir");
    std::fs::write(
        delta_dir.join("shared-capability.feature"),
        "@givn.delta @shared-capability\nFeature: Shared capability\n\n  @givn.modified\n  Scenario: Shared capability works\n    Then updated\n",
    )
    .expect("write delta");
    world.last_change_id = Some("owner-demo".to_string());
}

fn write_owner_narrative(root: &std::path::Path, usecase: &str, cap: &str) {
    let dir = root.join("givn/specs").join(usecase);
    std::fs::create_dir_all(&dir).expect("create owner narrative dir");
    if usecase == "fragments" {
        let document = givn::gherkin::usecase_md::fragment_md_template()
            .replace("<fragment-id>", usecase)
            .replace("<capability-id>", cap);
        std::fs::write(dir.join("fragment.md"), document).expect("write fragment narrative");
        return;
    }
    let document = givn::gherkin::usecase_md::usecase_md_template()
        .replace("<usecase-id>", usecase)
        .replace("<capability-id>", cap)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", "No E2E interaction")
        .replace(
            &format!(
                "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n| {cap} | exercise the capability | No E2E interaction |\n"
            ),
            "- none\n",
        );
    std::fs::write(dir.join("usecase.md"), document).expect("write owner narrative");
}

#[given(regex = r#"^"shared-capability" has (.+)$"#)]
fn shared_capability_owners(world: &mut GivnWorld, owners: String) {
    let root = support::root(world);
    let change_dir = root.join("givn/changes/owner-demo");
    let mut write_usecase_owner = false;
    let mut write_fragment_owner = false;
    match owners.as_str() {
        "one use-case owner" => write_usecase_owner = true,
        "one fragment owner" => write_fragment_owner = true,
        "both a use-case owner and a fragment owner" => {
            write_usecase_owner = true;
            write_fragment_owner = true;
        }
        other => panic!("unknown owner shape '{other}'"),
    }
    let delta = change_dir.join("specs/usecase-owner/shared-capability.feature");
    if write_usecase_owner && write_fragment_owner {
        for (dir, title) in [
            ("usecase-owner", "Shared capability works in usecase"),
            ("fragments", "Shared capability works in fragment"),
        ] {
            let dir = root.join("givn/specs").join(dir);
            std::fs::create_dir_all(&dir).expect("create owner dir");
            std::fs::write(
                dir.join("shared-capability.feature"),
                format!("Feature: Shared capability\n\n  Scenario: {title}\n    Then original\n"),
            )
            .expect("write owner feature");
            write_owner_narrative(
                &root,
                dir.file_name().and_then(|name| name.to_str()).unwrap(),
                "shared-capability",
            );
        }
        std::fs::write(
            &delta,
            "@givn.delta @shared-capability\nFeature: Shared capability\n\n  @givn.added\n  Scenario: Shared capability adds behavior\n    Then updated\n",
        )
        .expect("write ambiguous delta");
    } else if write_usecase_owner {
        let dir = root.join("givn/specs/usecase-owner");
        std::fs::create_dir_all(&dir).expect("create usecase owner");
        std::fs::write(
            dir.join("shared-capability.feature"),
            "Feature: Shared capability\n\n  Scenario: Shared capability works in usecase\n    Then original\n",
        )
        .expect("write usecase owner feature");
        write_owner_narrative(&root, "usecase-owner", "shared-capability");
        std::fs::write(
            &delta,
            "@givn.delta @shared-capability\nFeature: Shared capability\n\n  @givn.modified\n  Scenario: Shared capability works in usecase\n    Then updated\n",
        )
        .expect("write usecase delta");
    } else if write_fragment_owner {
        let dir = root.join("givn/specs/fragments");
        std::fs::create_dir_all(&dir).expect("create fragment owner");
        std::fs::write(
            dir.join("shared-capability.feature"),
            "Feature: Shared capability\n\n  Scenario: Shared capability works in fragment\n    Then original\n",
        )
        .expect("write fragment owner feature");
        write_owner_narrative(&root, "fragments", "shared-capability");
        let target = change_dir.join("specs/fragments/shared-capability.feature");
        std::fs::create_dir_all(target.parent().unwrap()).expect("create fragment delta dir");
        std::fs::rename(&delta, &target).expect("move delta to fragment owner");
        std::fs::write(
            &target,
            "@givn.delta @shared-capability\nFeature: Shared capability\n\n  @givn.modified\n  Scenario: Shared capability works in fragment\n    Then updated\n",
        )
        .expect("write fragment delta");
    }
}

#[given(regex = r#"^the change declares (EXTEND|NEW) for "shared-capability"$"#)]
fn declares_routing(world: &mut GivnWorld, routing: String) {
    let root = support::root(world);
    let change_dir = root.join("givn/changes/owner-demo");
    std::fs::write(change_dir.join("proposal.md"), owner_proposal(&routing))
        .expect("write proposal");
    if routing == "NEW" {
        let source = if change_dir
            .join("specs/fragments/shared-capability.feature")
            .exists()
        {
            change_dir.join("specs/fragments/shared-capability.feature")
        } else {
            change_dir.join("specs/usecase-owner/shared-capability.feature")
        };
        let target = change_dir.join("specs/new-group/shared-capability.feature");
        std::fs::create_dir_all(target.parent().unwrap()).expect("create new-group delta dir");
        std::fs::rename(source, target).expect("move delta to new group");
    }
}

#[given("all other applicable review and archive evidence passes")]
fn other_evidence_passes(world: &mut GivnWorld) {
    let id = "owner-demo";
    commit_and_refresh(world, id);
    world.verification_log_before =
        Some(serde_json::to_string(&archive_snapshot(world, id)).expect("snapshot JSON"));
}

fn archive_snapshot(world: &GivnWorld, id: &str) -> Vec<(String, Vec<u8>)> {
    let root = support::root(world);
    let mut out = Vec::new();
    for (prefix, path) in [
        ("specs", root.join("givn/specs")),
        ("change", root.join(format!("givn/changes/{id}"))),
        ("archive", root.join(format!("givn/archive/{id}"))),
    ] {
        collect_snapshot(prefix, &path, &mut out);
    }
    out.sort();
    out
}

fn collect_snapshot(prefix: &str, path: &std::path::Path, out: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_snapshot(prefix, &path, out);
        } else {
            let relative = path
                .strip_prefix(path.ancestors().last().unwrap_or(&path))
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            out.push((
                format!("{prefix}:{relative}"),
                std::fs::read(&path).unwrap_or_default(),
            ));
        }
    }
}

#[then(
    regex = r"^the command (succeeds|exits non-zero) and reports (the existing use-case owner|the existing fragment owner|a NEW conflict with the existing use-case owner|a NEW conflict with the existing fragment owner|ambiguous capability owners)$"
)]
fn command_outcome_and_owner(world: &mut GivnWorld, outcome: String, resolution: String) {
    let out = world.last_output.as_ref().expect("command ran");
    let succeeded = out.status.success();
    assert_eq!(
        succeeded,
        outcome == "succeeds",
        "unexpected outcome: status={:?} stderr={}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = crate::steps::stdout(world);
    let stderr = crate::steps::stderr(world);
    match resolution.as_str() {
        "the existing use-case owner" => {
            assert!(
                stdout.contains("usecase-owner") || stdout.contains("shared-capability"),
                "success output does not report the use-case owner: {stdout}"
            );
        }
        "the existing fragment owner" => {
            assert!(
                stdout.contains("fragments") || stdout.contains("shared-capability"),
                "success output does not report the fragment owner: {stdout}"
            );
        }
        "a NEW conflict with the existing use-case owner" => {
            assert!(
                stderr.contains("NEW") && stderr.contains("already exists"),
                "stderr lacks NEW conflict: {stderr}"
            );
        }
        "a NEW conflict with the existing fragment owner" => {
            assert!(
                stderr.contains("NEW") && stderr.contains("already exists"),
                "stderr lacks NEW conflict: {stderr}"
            );
        }
        "ambiguous capability owners" => {
            assert!(
                stderr.contains("ambiguous capability owners"),
                "stderr lacks ambiguity: {stderr}"
            );
        }
        other => panic!("unknown owner resolution '{other}'"),
    }
}

#[then("owner resolution completes before any permanent-spec mutation or active-change move")]
fn owner_before_mutation(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let out = world.last_output.as_ref().expect("command ran");
    if !out.status.success() {
        let before: Vec<(String, Vec<u8>)> =
            serde_json::from_str(world.verification_log_before.as_deref().expect("snapshot"))
                .expect("parse snapshot");
        assert_eq!(
            before,
            archive_snapshot(world, &id),
            "a rejected command mutated permanent state"
        );
    } else {
        assert!(
            support::root(world)
                .join(format!("givn/archive/{id}"))
                .is_dir()
                || support::root(world).join("givn/specs").is_dir(),
            "owner resolution left no archive result"
        );
    }
}

#[then("a rejected command leaves permanent specs and the active change unchanged")]
fn rejected_leaves_unchanged(world: &mut GivnWorld) {
    let id = world.last_change_id.clone().expect("change id");
    let out = world.last_output.as_ref().expect("command ran");
    if out.status.success() {
        return;
    }
    let before: Vec<(String, Vec<u8>)> =
        serde_json::from_str(world.verification_log_before.as_deref().expect("snapshot"))
            .expect("parse snapshot");
    assert_eq!(
        before,
        archive_snapshot(world, &id),
        "a rejected command mutated permanent specs or the active change"
    );
}

#[given("a completed change routes a new capability into its declared use case")]
fn new_capability_preserves_structure(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    grouped_commands(world);
    let env = world.env.as_ref().unwrap();
    let out = common::run_givn(env, &["new", "structure-new"]);
    assert!(out.status.success());
    let change_dir = env.path("givn/changes/structure-new");
    std::fs::write(
        change_dir.join("proposal.md"),
        ROUTED_PROPOSAL
            .replace("{ID}", "structure-new")
            .replace("{GROUP}", "structure-group"),
    )
    .expect("write proposal");
    write_passing_artifacts(&change_dir);
    write_supported_delta(&change_dir, "structure-group", "structure-new");
    world.last_change_id = Some("structure-new".to_string());
}

#[given(
    "the supported feature contains Background setup, inherited tags, docstrings, data tables and multiple tagged Examples blocks"
)]
fn supported_feature_structure(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root");
    let content = std::fs::read_to_string(
        env.path("givn/changes/structure-new/specs/structure-group/structure-new.feature"),
    )
    .expect("read delta");
    assert!(content.contains("Background:"));
    assert!(content.contains("@e2e"));
    assert!(content.contains("\"\"\"json"));
    assert!(content.contains("Examples: First"));
    assert!(content.contains("Examples: Second"));
}

#[then("the new permanent feature retains the same supported setup and execution scope")]
fn new_feature_keeps_scope(world: &mut GivnWorld) {
    let root = support::root(world);
    let path = root.join("givn/specs/structure-group/structure-new.feature");
    let text = std::fs::read_to_string(&path).expect("read permanent feature");
    let feature =
        givn::gherkin::parse::parse_supported_source(&text, &path).expect("parse feature");
    assert!(feature.tags.iter().any(|tag| tag == "e2e"));
    assert!(feature.background.is_some());
    assert_eq!(feature.scenarios.len(), 1);
}

#[then(
    "every step argument and Examples value remains attached to its original executable element"
)]
fn arguments_attached(world: &mut GivnWorld) {
    let root = support::root(world);
    let path = root.join("givn/specs/structure-group/structure-new.feature");
    let text = std::fs::read_to_string(&path).expect("read permanent feature");
    let feature =
        givn::gherkin::parse::parse_supported_source(&text, &path).expect("parse feature");
    let outline = &feature.scenarios[0];
    assert!(outline.steps[0].docstring().is_some());
    assert!(outline.steps[1].table().is_some());
    assert_eq!(outline.examples.len(), 2);
    assert_eq!(outline.examples[0].table.as_ref().unwrap().rows.len(), 3);
    assert_eq!(outline.examples[1].table.as_ref().unwrap().rows.len(), 2);
}

#[then("only delta-operation tags are removed")]
fn delta_tags_removed(world: &mut GivnWorld) {
    let root = support::root(world);
    let path = root.join("givn/specs/structure-group/structure-new.feature");
    let text = std::fs::read_to_string(&path).expect("read permanent feature");
    assert!(!text.contains("@givn.added"));
    assert!(!text.contains("@givn.modified"));
    assert!(text.contains("@fast") && text.contains("@api") && text.contains("@slow"));
}

#[given("a completed change has two correctly routed new capabilities")]
fn two_routed_capabilities(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    grouped_commands(world);
    let env = world.env.as_ref().unwrap();
    let out = common::run_givn(env, &["new", "two-new"]);
    assert!(out.status.success());
    let change_dir = env.path("givn/changes/two-new");
    std::fs::write(
        change_dir.join("proposal.md"),
        "# Proposal: fixture\n\n## Capability Routing\n\n| Proposed capability | Route's recommendation | Decision | Rationale |\n|---|---|---|---|\n| supported-cap | (fixture) | NEW in group-a | fixture |\n| rule-cap | (fixture) | NEW in group-b | fixture |\n",
    )
    .expect("write proposal");
    write_passing_artifacts(&change_dir);
    let group_a = change_dir.join("specs/group-a");
    std::fs::create_dir_all(&group_a).expect("create group a");
    std::fs::write(
        group_a.join("supported-cap.feature"),
        "@givn.delta @supported-cap\nFeature: Supported capability\n\n  @givn.added\n  Scenario: Supported works\n    Then visible\n",
    )
    .expect("write supported delta");
    write_usecase_document(&group_a, "group-a", "supported-cap", "Supported works");
    let group_b = change_dir.join("specs/group-b");
    std::fs::create_dir_all(&group_b).expect("create group b");
    std::fs::write(
        group_b.join("rule-cap.feature"),
        "@givn.delta @rule-cap\nFeature: Rule capability\n\n  Rule: Restricted\n    Scenario: Nested\n      Then visible\n",
    )
    .expect("write rule delta");
    write_usecase_document(&group_b, "group-b", "rule-cap", "Nested");
    commit_and_refresh(world, "two-new");
}

#[given("one delta is supported while the other contains a Rule")]
fn one_delta_contains_rule(world: &mut GivnWorld) {
    let root = support::root(world);
    let supported = std::fs::read_to_string(
        root.join("givn/changes/two-new/specs/group-a/supported-cap.feature"),
    )
    .expect("read supported delta");
    let rule =
        std::fs::read_to_string(root.join("givn/changes/two-new/specs/group-b/rule-cap.feature"))
            .expect("read rule delta");
    assert!(!supported.contains("Rule:"));
    assert!(rule.contains("Rule:"));
}

#[then("neither delta is merged and neither permanent capability is created")]
fn no_new_capabilities_created(world: &mut GivnWorld) {
    let root = support::root(world);
    assert!(!root.join("givn/specs/group-a").exists());
    assert!(!root.join("givn/specs/group-b").exists());
}

#[then("the active change is not moved and no archive receipt is written")]
fn change_not_moved_no_receipt(world: &mut GivnWorld) {
    let root = support::root(world);
    assert!(root.join("givn/changes/two-new").is_dir());
    assert!(!root.join("givn/archive/two-new/verification.json").exists());
}

/// Addon policy belongs in `givn/config.yaml`; the coverage gate removes stale
/// reports before measuring, so the fixture measure command writes them.
fn configure_retirement_policy(env: &common::TestEnv) {
    common::replace_config_addons(
        env,
        "addons:\n  arc42: true\n  readme: true\n  coverage: true\n  domain_modeling: true\n  dev_principles: true\n",
    );
    common::write_fixture_coverage_measure(
        env,
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    );
}
