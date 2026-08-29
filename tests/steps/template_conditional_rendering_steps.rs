//! Non-e2e step definitions for the template-conditional-rendering delta.
//! Fixtures start from `common::make_init_root()` (every addon enabled) and
//! set the scenario's addon state by editing `givn/config.yaml` directly,
//! then `givn skills sync`. The two error scenarios write project-override
//! templates and assert `skills sync` fails closed.

use cucumber::{given, then, when};
use std::process::Output;

use crate::common;
use crate::steps::GivnWorld;
use crate::steps::template_conditional_rendering_support as support;

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

// --- Given ------------------------------------------------------------------

#[given(
    regex = r"^a Givn project with the (.+) and (.+) addons enabled and the skills synchronized$"
)]
fn givn_project_two_addons_enabled(world: &mut GivnWorld, addon_a: String, addon_b: String) {
    world.env = Some(common::make_init_root());
    support::set_addon(world, &addon_a, true);
    support::set_addon(world, &addon_b, true);
    support::sync(world);
    assert_synced_successfully(world);
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
    regex = r"^a Givn project with the ([a-z0-9-]+) addon disabled and the skills synchronized$"
)]
fn givn_project_one_addon_disabled(world: &mut GivnWorld, addon: String) {
    world.env = Some(common::make_init_root());
    support::set_addon(world, &addon, false);
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

#[given(
    "a Givn project with the domain-modeling addon enabled but the arc42 addon disabled and the skills synchronized"
)]
fn givn_project_dm_enabled_arc42_disabled(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    support::set_addon(world, "domain_modeling", true);
    support::set_addon(world, "arc42", false);
    support::sync(world);
    assert_synced_successfully(world);
}

#[given(
    "a Givn project with a project skill template that contains a conditional on an unknown addon"
)]
fn givn_project_unknown_addon_conditional(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let dir = support::env(world).path("givn/skills/givn-review");
    std::fs::create_dir_all(&dir).expect("create skill override dir");
    std::fs::write(
        dir.join("SKILL.md.tmpl"),
        "---\nname: givn-review\ndescription: fixture override\n---\n\n# Title\n\n{{#if addons.bogus_addon}}\nconditional\n{{/if}}\n",
    )
    .expect("write skill override template");
}

#[given("a Givn project with a project command template that contains an unbalanced conditional")]
fn givn_project_unbalanced_command_conditional(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let dir = support::env(world).path("givn/commands");
    std::fs::create_dir_all(&dir).expect("create command override dir");
    std::fs::write(
        dir.join("givn-review.md"),
        "---\ndescription: fixture override\n---\n\nbody\n{{/if}}\n",
    )
    .expect("write command override template");
}

// --- When -------------------------------------------------------------------

#[when("a consumer synchronizes skills through the Givn CLI")]
fn consumer_synchronizes_skills(world: &mut GivnWorld) {
    support::sync(world);
}

// --- Then: section presence/absence -----------------------------------------

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

#[then("the generated review skill contains the Arc42 implementation conformance section")]
fn review_skill_has_arc42(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-review");
    assert!(!texts.is_empty(), "no generated review skill was found");
    for text in texts {
        assert!(
            text.contains("Arc42 implementation conformance"),
            "generated review skill missing the Arc42 section:\n{text}",
        );
    }
}

#[then("the generated review command contains no Arc42 implementation conformance section")]
fn review_command_no_arc42(world: &mut GivnWorld) {
    let text =
        support::command_text(world, "givn-review").expect("no generated review command was found");
    assert!(
        !text.contains("Arc42 implementation conformance"),
        "generated review command unexpectedly contains the Arc42 section:\n{text}",
    );
}

#[then("the generated review command contains the Arc42 implementation conformance section")]
fn review_command_has_arc42(world: &mut GivnWorld) {
    let text =
        support::command_text(world, "givn-review").expect("no generated review command was found");
    assert!(
        text.contains("Arc42 implementation conformance"),
        "generated review command missing the Arc42 section:\n{text}",
    );
}

#[then("the generated review skill contains no Ubiquitous language conformance section")]
fn review_skill_no_ubl(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-review");
    assert!(!texts.is_empty(), "no generated review skill was found");
    for text in texts {
        assert!(
            !text.contains("Ubiquitous language conformance"),
            "generated review skill unexpectedly contains the ubiquitous-language section:\n{text}",
        );
    }
}

#[then("the generated review skill contains the Ubiquitous language conformance section")]
fn review_skill_has_ubl(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-review");
    assert!(!texts.is_empty(), "no generated review skill was found");
    for text in texts {
        assert!(
            text.contains("Ubiquitous language conformance"),
            "generated review skill missing the ubiquitous-language section:\n{text}",
        );
    }
}

#[then("the generated design-review skill contains no Architecture documentation (arc42) branch")]
fn design_review_skill_no_arc42(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-design-review");
    assert!(
        !texts.is_empty(),
        "no generated design-review skill was found"
    );
    for text in texts {
        assert!(
            !text.contains("Architecture documentation (arc42)"),
            "generated design-review skill unexpectedly contains the arc42 branch:\n{text}",
        );
    }
}

#[then("the generated design-review skill contains the Architecture documentation (arc42) branch")]
fn design_review_skill_has_arc42(world: &mut GivnWorld) {
    let texts = support::skill_texts(world, "givn-design-review");
    assert!(
        !texts.is_empty(),
        "no generated design-review skill was found"
    );
    for text in texts {
        assert!(
            text.contains("Architecture documentation (arc42)"),
            "generated design-review skill missing the arc42 branch:\n{text}",
        );
    }
}

#[then("the generated design-review command contains no Architecture documentation (arc42) check")]
fn design_review_command_no_arc42(world: &mut GivnWorld) {
    let text = support::command_text(world, "givn-design-review")
        .expect("no generated design-review command was found");
    assert!(
        !text.contains("Architecture documentation (arc42)"),
        "generated design-review command unexpectedly contains the arc42 check:\n{text}",
    );
}

#[then("the generated design-review command contains the Architecture documentation (arc42) check")]
fn design_review_command_has_arc42(world: &mut GivnWorld) {
    let text = support::command_text(world, "givn-design-review")
        .expect("no generated design-review command was found");
    assert!(
        text.contains("Architecture documentation (arc42)"),
        "generated design-review command missing the arc42 check:\n{text}",
    );
}

// --- Then: domain-modeling skill generation ---------------------------------

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

// --- Then: universal residual scan ------------------------------------------

#[then("no generated skill or command mentions any disabled addon")]
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

// --- Then: fail-closed error scenarios --------------------------------------

#[then("the synchronization fails")]
fn synchronization_fails(world: &mut GivnWorld) {
    let out: &Output = world
        .last_output
        .as_ref()
        .expect("no `givn skills sync` has been run yet");
    assert!(
        !out.status.success(),
        "expected givn skills sync to fail: {}",
        String::from_utf8_lossy(&out.stdout),
    );
}

#[then("the failure names the template with the invalid directive")]
fn failure_names_template_and_directive(world: &mut GivnWorld) {
    let out: &Output = world
        .last_output
        .as_ref()
        .expect("no `givn skills sync` has been run yet");
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        combined.contains("givn-review"),
        "failure must name the failing template, got: {combined}",
    );
    assert!(
        combined.contains("bogus_addon") || combined.contains("{{/if}}"),
        "failure must name the invalid directive, got: {combined}",
    );
}
