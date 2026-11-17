use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::gherkin::Step;
use cucumber::{given, then};
use std::io::Write;

fn write_command_overrides(env: &common::TestEnv) {
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    crate::steps::archive_proof_support::configure_commands(env);
    crate::steps::archive_proof_support::disable_coverage(env);
}

fn delta_narrative(usecase: &str, cap: &str, delta: &str, permanent: Option<&str>) -> String {
    let mut titles: Vec<String> = Vec::new();
    for text in [permanent, Some(delta)].into_iter().flatten() {
        if let Ok(feature) = gherkin::Feature::parse(text, gherkin::GherkinEnv::default()) {
            let inherited_e2e = feature.tags.iter().any(|tag| tag == "e2e");
            titles.extend(
                feature
                    .scenarios
                    .iter()
                    .filter(|scenario| {
                        inherited_e2e || scenario.tags.iter().any(|tag| tag == "e2e")
                    })
                    .map(|scenario| scenario.name.clone()),
            );
        }
    }
    titles.sort();
    titles.dedup();
    let first = titles
        .first()
        .map(String::as_str)
        .unwrap_or("No E2E interaction");
    let mut document = crate::harness::usecase_md_template()
        .replace("<usecase-id>", usecase)
        .replace("<capability-id>", cap)
        .replace("<actor role>", "Fixture operator")
        .replace("<the domain goal>", "Exercise the fixture behavior.")
        .replace("<consumer action>", "exercise the capability")
        .replace("<scenario title>", first);
    let header = "| Capability | Consumer action | E2E scenario |\n|---|---|---|\n";
    let first_row = format!("| {cap} | exercise the capability | {first} |\n");
    if titles.is_empty() {
        document = document.replace(&format!("{header}{first_row}"), "- none\n");
    } else {
        let rows = titles
            .iter()
            .map(|title| format!("| {cap} | exercise the capability | {title} |\n"))
            .collect::<String>();
        document = document.replace(&first_row, &rows);
    }
    document
}

fn complete_change_with_delta(
    env: &common::TestEnv,
    id: &str,
    delta_content: &str,
) -> std::path::PathBuf {
    let change_dir = env.path(&format!("givn/changes/{id}"));
    if !change_dir.exists() {
        let out = common::run_givn(env, &["new", id]);
        assert!(
            out.status.success(),
            "givn new {id} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
    for file in &[
        "proposal.md",
        "design.md",
        "design-review.md",
        "tasks.md",
        "review.md",
    ] {
        let mut f = std::fs::File::create(change_dir.join(file)).expect("create artifact");
        if *file == "design-review.md" {
            writeln!(f, "DESIGN-REVIEW: PASS").ok();
        } else if *file == "review.md" {
            writeln!(f, "README-IMPACT: none\n\nREVIEW: PASS").ok();
        } else if *file == "tasks.md" {
            writeln!(f, "- [x] COMMIT: a1b2c3d4 -- all done").ok();
        }
    }
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n").unwrap();
    write_command_overrides(env);
    let specs_dir = change_dir.join("specs");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    let capability = delta_content
        .split_whitespace()
        .find_map(|token| {
            token
                .strip_prefix('@')
                .filter(|tag| !tag.starts_with("givn."))
        })
        .unwrap_or("delta")
        .to_string();
    let owner = crate::harness::resolve_capability_owner(&env.path("givn/specs"), &capability);
    let delta_dir = match owner {
        Some(owner) => {
            let dir = specs_dir.join(owner.file_name().expect("owner directory name"));
            std::fs::create_dir_all(&dir).expect("create owned delta dir");
            dir
        }
        None => specs_dir.clone(),
    };
    std::fs::write(delta_dir.join(format!("delta-{id}.feature")), delta_content)
        .expect("write delta feature");
    common::declare_fixture_routing(env, id);
    let usecase = if delta_dir == specs_dir {
        capability.clone()
    } else {
        delta_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&capability)
            .to_string()
    };
    let narrative_dir = if delta_dir == specs_dir {
        specs_dir.join(&usecase)
    } else {
        delta_dir
    };
    std::fs::create_dir_all(&narrative_dir).expect("create narrative dir");
    let permanent_feature = env.path(&format!("givn/specs/{usecase}/{capability}.feature"));
    let permanent_text = std::fs::read_to_string(&permanent_feature).ok();
    std::fs::write(
        narrative_dir.join("usecase.md"),
        delta_narrative(
            &usecase,
            &capability,
            delta_content,
            permanent_text.as_deref(),
        ),
    )
    .expect("write delta usecase");
    change_dir
}

fn docstring_block(content: &str) -> String {
    let mut out = String::from("      \"\"\"\n");
    for line in content.lines() {
        if line.is_empty() {
            out.push('\n');
        } else {
            out.push_str(&format!("      {line}\n"));
        }
    }
    out.push_str("      \"\"\"\n");
    out
}

#[given(
    regex = r#"^a givn project with a completed change "([^"]+)" whose delta modifies scenario "([^"]+)" within capability "([^"]+)" and gives a step the docstring:$"#
)]
fn e2e_project_with_completed_change(
    world: &mut GivnWorld,
    id: String,
    title: String,
    cap: String,
    step: &Step,
) {
    world.env = Some(common::make_init_root());
    let env = world.env.as_ref().expect("env set above");
    let dir = env.path(&format!("givn/specs/{cap}"));
    std::fs::create_dir_all(&dir).expect("create specs dir");
    std::fs::write(
        dir.join(format!("{cap}.feature")),
        format!("Feature: {cap}\n\n  Scenario: {title}\n    Given  a baseline step exists\n"),
    )
    .expect("write permanent spec");
    let doc = step.docstring.as_ref().expect("docstring required");
    let delta = format!(
        "@givn.delta @{cap}\nFeature: Delta\n\n  @givn.modified\n  Scenario: {title}\n    Given  a baseline step exists\n    And  a step uses the payload:\n{docstring}",
        docstring = docstring_block(doc),
    );
    complete_change_with_delta(env, &id, &delta);
    world.last_change_id = Some(id);
}

#[then(regex = r#"^the permanent spec file for "([^"]+)" contains the text "([^"]+)"$"#)]
fn permanent_spec_contains(world: &mut GivnWorld, cap: String, needle: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path(&format!("givn/specs/{cap}/{cap}.feature"));
    let content = std::fs::read_to_string(&path).expect("read permanent spec file");
    assert!(
        content.contains(&needle),
        "permanent spec {:?} missing {:?}:\n{}",
        path,
        needle,
        content,
    );
}
