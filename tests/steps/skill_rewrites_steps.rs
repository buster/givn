use cucumber::{then, when};

use crate::common;
use crate::steps::GivnWorld;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

#[when("the user reads the design and steps instructions for skill-rewrites")]
fn read_design_and_steps_instructions(world: &mut GivnWorld) {
    let design = common::run_givn(
        env(world),
        &["instructions", "design", "--change", "skill-rewrites"],
    );
    let steps = common::run_givn(
        env(world),
        &["instructions", "tasks", "--change", "skill-rewrites"],
    );
    assert!(
        design.status.success(),
        "design instructions failed: {:?}",
        design
    );
    assert!(
        steps.status.success(),
        "tasks instructions failed: {:?}",
        steps
    );
    let mut combined = String::from_utf8_lossy(&design.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&steps.stdout));
    world.additional_output = Some(combined);
}

fn installed_skill_texts(world: &GivnWorld) -> Vec<(std::path::PathBuf, String)> {
    let env = common::make_init_root();
    let root = env.root();
    // The scenario creates its own project; reuse it when present.
    let root = world.env.as_ref().map(|env| env.root()).unwrap_or(root);
    let targets =
        givn::config::overlay::load_skills_targets(&root).expect("load configured skills targets");
    let mut out = Vec::new();
    for target in targets {
        let dir = root.join(format!(".{target}/skills"));
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path().join("SKILL.md");
            if path.is_file() {
                out.push((
                    path.clone(),
                    std::fs::read_to_string(&path).expect("read generated skill"),
                ));
            }
        }
    }
    out
}

#[then("synchronization reports skills rather than skills and agent commands")]
fn sync_reports_skills_only(world: &mut GivnWorld) {
    let out = crate::steps::stdout(world);
    assert!(
        out.contains("synced skill:")
            && !out.contains("synced command:")
            && !out.contains("agent command"),
        "the synchronization report still names agent commands: {out}"
    );
}

#[then("generated review design and steps skills direct readers to their canonical instructions")]
fn generated_skills_direct_to_canonical_instructions(world: &mut GivnWorld) {
    let skills = installed_skill_texts(world);
    assert!(!skills.is_empty(), "no generated skills were installed");
    for (path, content) in skills {
        for (expected, marker) in [
            ("givn-review", "givn instructions review"),
            ("givn-design", "givn instructions design"),
            ("givn-steps", "givn instructions steps"),
        ] {
            if path.to_string_lossy().contains(expected) {
                assert!(
                    content.contains(marker),
                    "{} does not direct readers to {marker}",
                    path.display()
                );
            }
        }
    }
}

#[then(
    "the served review instruction retains split-or-keep while treating shape matches as advisory"
)]
fn review_instruction_retains_gates(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    let output = common::run_givn(
        env,
        &["instructions", "review", "--change", "skill-rewrites"],
    );
    assert!(output.status.success(), "review instruction failed");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("split-or-keep") && text.contains("advisory"),
        "the served review instruction lost its deterministic gates:\n{text}"
    );
}

#[then(
    "the served design and steps instructions retain black-box behavior and distinct requirement coverage"
)]
fn design_and_steps_instructions_retain_policy(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("sync fixture is missing");
    let design = common::run_givn(
        env,
        &["instructions", "design", "--change", "skill-rewrites"],
    );
    let steps = common::run_givn(
        env,
        &["instructions", "steps", "--change", "skill-rewrites"],
    );
    assert!(design.status.success() && steps.status.success());
    assert!(
        String::from_utf8_lossy(&design.stdout).contains("Black-Box-First"),
        "the design instruction lost the black-box policy"
    );
    assert!(
        String::from_utf8_lossy(&steps.stdout).contains("which case this test covers"),
        "the steps instruction lost distinct requirement coverage"
    );
}

#[then("generated skills contain no repeated dialog-loading or delegation policy blocks")]
fn generated_skills_repeat_no_policy(world: &mut GivnWorld) {
    for (path, content) in installed_skill_texts(world) {
        for marker in [
            "givn instructions dialog",
            "When work is handed to another context",
            "**Dialog policy:**",
            "pass the obligation explicitly",
        ] {
            assert!(
                !content.contains(marker),
                "{} repeats {marker:?}:\n{content}",
                path.display()
            );
        }
    }
}

#[then("the guidance requires split-or-keep and a brief reason for keeping a long scenario")]
fn guidance_requires_split_with_reason(world: &mut GivnWorld) {
    let out = crate::steps::stdout(world);
    assert!(
        out.contains("split-or-keep")
            && out.contains("brief keep reason")
            && out.contains("completed before sign-off"),
        "the review guidance does not require split-or-keep with a keep reason"
    );
}

#[then("the guidance requires completing a chosen split before sign-off")]
fn guidance_requires_split_completion(world: &mut GivnWorld) {
    let out = crate::steps::stdout(world);
    assert!(
        out.contains("A chosen split must be") && out.contains("completed before sign-off"),
        "the review guidance does not require completing a chosen split before sign-off"
    );
}

#[then("shape-match findings do not require blocking dispositions")]
fn shape_matches_are_not_blocking(world: &mut GivnWorld) {
    let out = crate::steps::stdout(world);
    assert!(
        out.contains("advisory") && !out.contains("shape match must block"),
        "shape-match findings are treated as blocking"
    );
}

#[then("every generated skill carries trigger metadata and a direction to its served instruction")]
fn generated_skills_carry_metadata_and_direction(world: &mut GivnWorld) {
    let skills = installed_skill_texts(world);
    assert!(!skills.is_empty(), "no generated skills were installed");
    for (path, content) in skills {
        assert!(
            content.starts_with("---\n") && content.contains("description:"),
            "{} has no trigger metadata",
            path.display()
        );
        assert!(
            content.contains("givn instructions "),
            "{} has no served-instruction direction",
            path.display()
        );
    }
}

#[then("no generated skill copies built-in instruction rules")]
fn generated_skills_copy_no_rules(world: &mut GivnWorld) {
    for (path, content) in installed_skill_texts(world) {
        for marker in [
            "## Rule 1",
            "Artifact-specific rules",
            "VISUAL-REVIEW: PASS",
            "Overlap dispositions",
            "which case this test covers",
        ] {
            assert!(
                !content.contains(marker),
                "{} copies the built-in rule {marker:?}",
                path.display()
            );
        }
    }
}

#[then("dialog policy and delegation boilerplate are not repeated in thin skills")]
fn thin_skills_omit_policy_and_delegation(world: &mut GivnWorld) {
    generated_skills_repeat_no_policy(world);
}

#[then(
    "the generated givn-implement skill names both givn instructions tasks and givn instructions steps"
)]
fn generated_implement_skill_names_tasks_and_steps(world: &mut GivnWorld) {
    let mut checked = 0;
    for (path, content) in installed_skill_texts(world) {
        if !path.to_string_lossy().contains("givn-implement") {
            continue;
        }
        checked += 1;
        assert!(
            content.contains("givn instructions tasks"),
            "{} does not name the tasks instruction:\n{content}",
            path.display()
        );
        assert!(
            content.contains("givn instructions steps"),
            "{} does not name the steps instruction:\n{content}",
            path.display()
        );
    }
    assert!(checked > 0, "no generated givn-implement skill was found");
}

#[then("the served tasks instruction names givn instructions steps")]
fn served_tasks_instruction_names_steps(world: &mut GivnWorld) {
    let env = env(world);
    let output = common::run_givn(
        env,
        &["instructions", "tasks", "--change", "skill-rewrites"],
    );
    assert!(
        output.status.success(),
        "tasks instruction failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("givn instructions steps"),
        "the served tasks instruction does not name the steps instruction:\n{text}"
    );
}
