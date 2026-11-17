use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::then;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn installed_skill_texts(world: &GivnWorld) -> Vec<(std::path::PathBuf, String)> {
    let env = common::make_init_root();
    let root = env.root();
    // The scenario creates its own project; reuse it when present.
    let root = world.env.as_ref().map(|env| env.root()).unwrap_or(root);
    let targets = crate::harness::skills_targets(&root).expect("load configured skills targets");
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
