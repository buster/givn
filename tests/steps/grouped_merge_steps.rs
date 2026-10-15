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
            "review.md" => writeln!(f, "REVIEW: PASS\nREADME-IMPACT: none").ok(),
            "tasks.md" => writeln!(f, "- [x] all done").ok(),
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
  measure_command: \"true\"
  merge_command: \"cp coverage/non-e2e-cobertura.xml coverage/cobertura-coverage.xml\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
addons:
  arc42: true
  absolute_mode: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  grill_me: true
  grill_with_docs: true
";
    let yaml = format!("{yaml}{}", common::complete_run_declaration("cli"));
    std::fs::write(&commands_path, yaml).expect("write commands.yaml");
    let cov_dir = env.path("coverage");
    std::fs::create_dir_all(&cov_dir).expect("create coverage dir");
    std::fs::write(
        cov_dir.join("non-e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.85" branch-rate="0.72" lines-covered="340" lines-valid="400" branches-covered="36" branches-valid="50" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write non-e2e xml");
    std::fs::write(
        cov_dir.join("e2e-cobertura.xml"),
        r#"<?xml version="1.0"?>
<coverage line-rate="0.9" branch-rate="0.8" lines-covered="90" lines-valid="100" branches-covered="16" branches-valid="20" timestamp="0" version="1">
  <packages />
</coverage>"#,
    )
    .expect("write e2e xml");

    // The permanent use case: a narrative plus its only capability.
    let usecase_dir = env.path(&format!("givn/specs/{usecase}"));
    std::fs::create_dir_all(&usecase_dir).expect("create use-case dir");
    std::fs::write(
        usecase_dir.join("usecase.md"),
        format!(
            "# Use case: {usecase}\n\n## Narrative\n\nA use case whose only capability is `{cap}`.\n"
        ),
    )
    .expect("write use-case narrative");
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
