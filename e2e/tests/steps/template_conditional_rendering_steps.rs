//! Non-e2e step definitions for the template-conditional-rendering delta.
//! Fixtures start from `common::make_init_root()` (every addon enabled) and
//! set the scenario's addon state by editing `givn/config.yaml` directly,
//! then `givn skills sync`. The two error scenarios write project-override
//! templates and assert `skills sync` fails closed.

use crate::harness as common;
use crate::steps::GivnWorld;
use crate::steps::template_conditional_rendering_support as support;
use cucumber::{given, then, when};
use std::process::Output;

fn assert_synced_successfully(world: &GivnWorld) {
    let out: &Output = world
        .last_output
        .as_ref()
        .expect("no `givn skills sync` has been run yet");
    assert!(
        out.status.success(),
        "givn skills sync failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

#[given(
    regex = r"^a Givn project with the ([a-z0-9-]+) addon enabled and the skills synchronized$"
)]
fn givn_project_one_addon_enabled(world: &mut GivnWorld, addon: String) {
    world.env = Some(common::make_init_root());
    support::set_addon(world, &addon, true);
    support::sync(world);
    assert_synced_successfully(world);
}

#[given(
    regex = r"^a Givn project with the ([a-z0-9-]+) and ([a-z0-9-]+) addons disabled and the skills synchronized$"
)]
fn givn_project_two_addons_disabled(world: &mut GivnWorld, addon_a: String, addon_b: String) {
    world.env = Some(common::make_init_root());
    support::set_addon(world, &addon_a, false);
    support::set_addon(world, &addon_b, false);
    support::sync(world);
    assert_synced_successfully(world);
}

#[given(
    regex = r"^a Givn project with the (.+), (.+), and (.+) addons disabled and the skills synchronized$"
)]
fn givn_project_three_addons_disabled(
    world: &mut GivnWorld,
    addon_a: String,
    addon_b: String,
    addon_c: String,
) {
    world.env = Some(common::make_init_root());
    for addon in [addon_a, addon_b, addon_c] {
        support::set_addon(world, &addon, false);
    }
    support::sync(world);
    assert_synced_successfully(world);
}

#[then("no Givn command files are generated")]
fn then_no_command_files(world: &mut GivnWorld) {
    let root = support::env(world).root();
    for target in [".opencode", ".agents", ".claude", ".omp", ".maki"] {
        let commands = root.join(target).join("commands");
        let generated: Vec<_> = std::fs::read_dir(&commands)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
                    .collect()
            })
            .unwrap_or_default();
        assert!(
            generated.is_empty(),
            "generated command files found under {target}: {generated:?}"
        );
    }
}

#[then("no generated review command exists")]
fn then_no_review_command(world: &mut GivnWorld) {
    assert!(
        support::command_text(world, "givn-review").is_none(),
        "a generated review command exists"
    );
}

#[when("a consumer synchronizes skills through the Givn CLI")]
fn consumer_synchronizes_skills(world: &mut GivnWorld) {
    support::sync(world);
}

#[then("the served review instruction omits the Arc42 implementation conformance section")]
fn served_review_instruction_omits_arc42(world: &mut GivnWorld) {
    let content = support::served_artifact_instruction(world, "review");
    assert!(
        !content.contains("Arc42 implementation conformance"),
        "the served review instruction carries the Arc42 section:\n{content}"
    );
}

#[then("the generated review skill carries no Arc42 implementation conformance section")]
fn review_skill_carries_no_arc42(world: &mut GivnWorld) {
    review_skill_no_arc42(world);
}

#[then("the generated review skill directs the reviewer to the review instruction")]
fn generated_review_skill_directs_to_instruction(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-review");
    assert!(!texts.is_empty(), "no generated review skill was found");
    for text in &texts {
        assert!(
            text.contains("givn instructions review"),
            "the generated review skill does not direct the reviewer to the review instruction:\n{text}"
        );
    }
}

#[then("the generated review skill contains no Arc42 implementation conformance section")]
fn review_skill_no_arc42(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-review");
    assert!(!texts.is_empty(), "no generated review skill was found");
    for text in texts {
        assert!(
            !text.contains("Arc42 implementation conformance"),
            "generated review skill unexpectedly contains the Arc42 section:\n{text}",
        );
    }
}

#[then("no generated domain-modeling skill exists")]
fn no_generated_domain_skill(world: &mut GivnWorld) {
    let paths = support::skill_paths(world, "givn-domain-modeling");
    for path in &paths {
        assert!(
            !path.exists(),
            "generated domain-modeling skill should not exist: {path:?}"
        );
    }
}

#[then("a generated domain-modeling skill exists")]
fn generated_domain_skill_exists(world: &mut GivnWorld) {
    let paths = support::skill_paths(world, "givn-domain-modeling");
    assert!(
        paths.iter().any(|p| p.exists()),
        "expected a generated domain-modeling skill, found none at {paths:?}",
    );
}

#[then("no generated skill or command mentions any disabled addon")]
#[then("no generated skill mentions any disabled addon")]
fn no_generated_mentions_disabled_addon(world: &mut GivnWorld) {
    let files = support::all_generated_files(world);
    assert!(
        !files.is_empty(),
        "expected generated skills/commands to scan for addon references",
    );
    let markers_exact = [
        "Coverage badge",
        "coverage.measure_command",
        "coverage.output_path",
        "givn:begin:coverage",
    ];
    let markers_case_insensitive = ["arc42", "glossary", "ubiquitous", "domain-modeling"];
    for path in files {
        let content = std::fs::read_to_string(&path).expect("read generated file");
        for marker in markers_exact {
            assert!(
                !content.contains(marker),
                "{path:?} still mentions disabled-addon marker {marker:?}:\n{content}",
            );
        }
        let lower = content.to_lowercase();
        for marker in markers_case_insensitive {
            assert!(
                !lower.contains(marker),
                "{path:?} still mentions disabled addon {marker:?} (case-insensitive)",
            );
        }
    }
}
