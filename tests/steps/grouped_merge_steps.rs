use std::io::Write;

use cucumber::given;

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
