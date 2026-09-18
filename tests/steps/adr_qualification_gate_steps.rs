//! Step definitions for the ADR qualification guidance capability.

use std::{collections::HashSet, fs, path::Path};

use cucumber::{given, then, when};

use crate::steps::GivnWorld;

fn guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("ADR qualification guidance was not loaded")
}

#[given("the ADR qualification guidance is available")]
fn adr_qualification_guidance_is_available(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/addons/arc42-docs/instructions.md",
    ))
    .expect("read canonical Arc42 instruction");
    world.additional_output = Some(content);
}

#[when("an author reads the qualification gate")]
fn author_reads_qualification_gate(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "qualification guidance is empty"
    );
}

#[then("the guidance requires materially different alternatives")]
fn guidance_requires_materially_different_alternatives(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Real alternatives"));
    assert!(content.contains("materially different options"));
}

#[then("the guidance requires architectural impact")]
fn guidance_requires_architectural_impact(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Architectural impact"));
    assert!(content.contains("component/ownership boundary"));
}

#[then("the guidance requires a durable consequence")]
fn guidance_requires_durable_consequence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Durable consequence"));
    assert!(content.contains("migration"));
}

#[then("the guidance requires correct lower-level artifact classification")]
fn guidance_requires_lower_level_classification(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Lower-level artifact classification"));
    assert!(content.contains("complete home"));
}

#[then("the guidance requires a completed existing-ADR or duplicate check")]
fn guidance_requires_existing_adr_check(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Existing-ADR check"));
    assert!(content.contains("NEW_ADR"));
}

#[then("the guidance defines qualification as Boolean AND across all mandatory gates")]
fn guidance_defines_boolean_and_gate(world: &mut GivnWorld) {
    let content = guidance(world);
    for clause in [
        "qualification = alternatives",
        "AND architectural_impact",
        "AND durable_consequence",
        "AND lower_level_artifact",
        "AND existing_adr_check",
    ] {
        assert!(
            content.contains(clause),
            "qualification guidance missing {clause}"
        );
    }
}

#[when("an author reads the qualification criteria")]
fn author_reads_qualification_criteria(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "qualification guidance is empty"
    );
}

#[then("the guidance says `Must be shared` is supporting evidence only")]
fn guidance_says_shared_is_supporting_only(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`Must be shared` is supporting evidence only"));
}

#[then("the guidance says shared evidence cannot qualify a candidate by itself")]
fn guidance_rejects_shared_only_qualification(world: &mut GivnWorld) {
    assert!(guidance(world).contains("It can never qualify a candidate by itself"));
}

#[then("the guidance routes a non-qualifying candidate to one canonical artifact")]
fn guidance_routes_non_qualifying_candidate(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("If any mandatory dimension fails, do not create an ADR"));
    assert!(content.contains("Route the decision once\nto its canonical artifact"));
}

#[when("an author reads the lower-level artifact rules")]
fn author_reads_lower_level_artifact_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "lower-level artifact guidance is empty"
    );
}

#[then("the guidance distinguishes a feature specification from technical design")]
fn guidance_distinguishes_feature_from_design(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("observable behavior with no architectural trade-off"));
    assert!(content.contains("Gherkin specification"));
    assert!(content.contains("local implementation approach or algorithm detail"));
    assert!(content.contains("`design.md` and code"));
}

#[then("the guidance distinguishes Arc42 documentation from implementation code")]
fn guidance_distinguishes_arc42_from_code(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("current component/runtime/deployment fact"));
    assert!(content.contains("corresponding Arc42 chapter"));
}

#[then("the guidance distinguishes process guidance from architecture rationale")]
fn guidance_distinguishes_process_from_architecture(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("prompt, workflow, review, test, coverage, licensing, roadmap"));
    assert!(content.contains("owning process or project document"));
}

#[then("the guidance requires exactly one canonical target for a non-ADR decision")]
fn guidance_requires_one_canonical_target(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`canonical_artifact` naming exactly one destination"));
}

#[when("an author reads the duplicate and relationship rules")]
fn author_reads_duplicate_relationship_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "relationship guidance is empty"
    );
}

#[then("the guidance requires searching existing ADRs before drafting")]
fn guidance_requires_searching_existing_adrs(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("active and archived registers have been searched"));
    assert!(content.contains("before proposing a new ID"));
}

#[then("the guidance requires an explicit new amend supersede or canonical-artifact route")]
fn guidance_requires_explicit_route(world: &mut GivnWorld) {
    let content = guidance(world);
    for route in [
        "NEW_ADR",
        "AMEND_ADR",
        "SUPERSEDE_ADR",
        "CANONICAL_ARTIFACT",
    ] {
        assert!(content.contains(route), "guidance missing route {route}");
    }
}

#[then("the guidance rejects a new ADR when the existing-ADR check is incomplete")]
fn guidance_rejects_incomplete_existing_check(world: &mut GivnWorld) {
    assert!(guidance(world).contains("An incomplete existing-ADR check fails"));
}

#[when("an author reads the structured verdict contract")]
fn author_reads_structured_verdict_contract(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "structured verdict guidance is empty"
    );
}

#[then("the verdict contract names all five mandatory gate fields")]
fn verdict_names_mandatory_gate_fields(world: &mut GivnWorld) {
    let content = guidance(world);
    for field in [
        "alternatives",
        "architectural_impact",
        "durable_consequence",
        "lower_level_artifact",
        "existing_adr_check",
    ] {
        assert!(content.contains(field), "verdict contract missing {field}");
    }
}

#[then("the verdict contract names the supporting-only `must_be_shared` field")]
fn verdict_names_supporting_field(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("must_be_shared"));
    assert!(content.contains("SUPPORTING"));
}

#[then("the verdict contract names `qualification` and `routing`")]
fn verdict_names_qualification_and_routing(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("qualification"));
    assert!(content.contains("routing"));
}

#[then("the verdict contract names `canonical_artifact`, `target_adr`, and `replacement_adr`")]
fn verdict_names_route_targets(world: &mut GivnWorld) {
    let content = guidance(world);
    for field in ["canonical_artifact", "target_adr", "replacement_adr"] {
        assert!(content.contains(field), "verdict contract missing {field}");
    }
}

#[then("the guidance defines the allowed qualification gate and routing values")]
fn guidance_defines_verdict_values(world: &mut GivnWorld) {
    let content = guidance(world);
    for value in [
        "QUALIFIED",
        "NOT_QUALIFIED",
        "NEW_ADR",
        "AMEND_ADR",
        "SUPERSEDE_ADR",
        "CANONICAL_ARTIFACT",
    ] {
        assert!(
            content.contains(value),
            "guidance missing verdict value {value}"
        );
    }
}

#[when("an author reads the route-target rules")]
fn author_reads_route_target_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "route-target guidance is empty"
    );
}

#[then("`CANONICAL_ARTIFACT` requires a `canonical_artifact` target")]
fn canonical_artifact_requires_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("`CANONICAL_ARTIFACT` for a non-qualifying candidate"));
    assert!(content.contains("`canonical_artifact` naming exactly one destination"));
}

#[then("`AMEND_ADR` requires a `target_adr` target")]
fn amend_adr_requires_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("`AMEND_ADR` for a qualified refinement"));
    assert!(content.contains("`target_adr` naming the existing"));
}

#[then("`SUPERSEDE_ADR` requires a `replacement_adr` target")]
fn supersede_adr_requires_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("`SUPERSEDE_ADR` for a qualified replacement"));
    assert!(content.contains("`replacement_adr` naming"));
}

#[then("unused route-target fields are required to remain null")]
fn unused_route_targets_are_null(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("For `CANONICAL_ARTIFACT`"));
    assert!(content.contains("For\n`AMEND_ADR`") || content.contains("For `AMEND_ADR`"));
    assert!(content.contains("For\n`SUPERSEDE_ADR`") || content.contains("For `SUPERSEDE_ADR`"));
    assert!(content.matches("are `null`").count() >= 3);
    assert!(content.contains("`canonical_artifact` and `replacement_adr`"));
    assert!(content.contains("`canonical_artifact` and `target_adr`"));
}

#[when("an author reads the evidence rules")]
fn author_reads_evidence_rules(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "evidence guidance is empty");
}

#[then("the guidance requires evidence supplied by the author or source artifacts")]
fn guidance_requires_supplied_evidence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Evidence arrays contain only facts and"));
    assert!(content.contains("alternatives supplied by the author or source artifacts"));
}

#[then("the guidance says missing evidence remains empty")]
fn guidance_keeps_missing_evidence_empty(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Missing evidence stays"));
    assert!(content.contains("empty and causes its gate to fail"));
}

#[then("the guidance forbids inventing alternatives impact consequences or participants")]
fn guidance_forbids_invented_evidence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("must not invent alternatives"));
    assert!(content.contains("impact, consequences, participants, or provenance"));
}

#[when("an author reads the refinement rules")]
fn author_reads_refinement_rules(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "refinement guidance is empty");
}

#[then("the guidance routes a refinement to `AMEND_ADR`")]
fn guidance_routes_refinement_to_amend(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`AMEND_ADR` for a qualified refinement"));
}

#[then("the guidance requires the target ADR to be named")]
fn guidance_requires_target_adr(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`target_adr` naming the existing"));
}

#[then("the guidance forbids a parallel ADR for the same decision boundary")]
fn guidance_forbids_parallel_adr(world: &mut GivnWorld) {
    assert!(guidance(world).contains("must not create a parallel ADR for"));
}

#[when("an author reads the replacement rules")]
fn author_reads_replacement_rules(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "replacement guidance is empty");
}

#[then("the guidance routes a replacement to `SUPERSEDE_ADR`")]
fn guidance_routes_replacement_to_supersede(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`SUPERSEDE_ADR` for a qualified replacement"));
}

#[then("the guidance requires the replacement ADR to be named")]
fn guidance_requires_replacement_adr(world: &mut GivnWorld) {
    assert!(guidance(world).contains("`replacement_adr` naming"));
}

#[then("the guidance requires the old architectural record to keep an explicit replacement link")]
fn guidance_requires_old_replacement_link(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("old record"));
    assert!(content.contains("explicit replacement link"));
}

#[when("an author reads the Arc42 impact rules")]
fn author_reads_arc42_impact_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "Arc42 impact guidance is empty"
    );
}

#[then("the guidance distinguishes chapter impact from ADR qualification")]
fn guidance_distinguishes_chapter_impact(world: &mut GivnWorld) {
    assert!(
        guidance(world)
            .contains("Arc42 chapter impact and ADR qualification are independent decisions")
    );
}

#[then("the guidance requires an affected chapter to be updated when appropriate")]
fn guidance_requires_affected_chapter_update(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("If chapters are affected"));
    assert!(content.contains("update only the sections affected"));
}

#[then("the guidance does not require a new ADR solely because a chapter is affected")]
fn guidance_does_not_require_adr_for_chapter(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("does not by itself qualify an ADR"));
}

#[given("the change-level Arc42 assessment template is available")]
fn change_level_arc42_template_is_available(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/addons/arc42-docs/impact-assessment-template.md",
    ))
    .expect("read Arc42 impact template");
    world.additional_output = Some(content);
}

#[when("a maintainer reads the ADR disposition marker")]
fn maintainer_reads_adr_disposition_marker(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "Arc42 template is empty");
}

#[then("the marker distinguishes chapter impact from ADR action")]
fn marker_distinguishes_impact_and_action(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("A change may affect chapter 09 without qualifying"));
    assert!(content.contains("ADR action:"));
}

#[then("the marker allows `NONE` with target `none`")]
fn marker_allows_no_adr_action(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("ADR action: NONE"));
    assert!(content.contains("ADR target: none"));
}

#[then("the marker does not duplicate the complete MADR procedure")]
fn marker_does_not_duplicate_madr(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Do not copy the complete qualification procedure"));
    assert!(content.contains("complete ADR procedure and template are owned"));
}

#[given("the consolidated ADR register guidance is available")]
fn consolidated_adr_register_guidance_is_available(world: &mut GivnWorld) {
    let register = std::fs::read_to_string(crate::common::repo_path("docs/arc42/adr/README.md"))
        .expect("read ADR directory register");
    let chapter = std::fs::read_to_string(crate::common::repo_path(
        "docs/arc42/09-architecture-decisions.md",
    ))
    .expect("read Arc42 chapter 9");
    world.additional_output = Some(format!("{register}\n{chapter}"));
}

#[when("a maintainer reads the register rules")]
fn maintainer_reads_register_rules(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "ADR register guidance is empty"
    );
}

#[then("the active register contains accepted non-superseded architectural decisions only")]
fn active_register_contains_architecture_only(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(
        content
            .contains("active register contains only independent durable architecture decisions")
    );
    assert!(content.contains("accepted"));
}

#[then("non-ADR information is not copied into the ADR archive")]
fn non_adr_information_is_not_archived(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("archive contains only genuinely architectural decisions"));
    assert!(content.contains("Non-ADR records are not retained there"));
}

#[then("current architecture facts are owned by Arc42 or the implementation")]
fn current_facts_have_canonical_owner(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Current architecture"));
    assert!(content.contains("facts remain in the relevant Arc42 chapters or implementation"));
}

#[then("an ADR retains rationale and links instead of duplicating current facts")]
fn adr_retains_rationale_and_links(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Individual ADRs preserve decision rationale and links"));
    assert!(content.contains("not a second copy of those facts"));
}

#[when("a maintainer compares the register with the canonical sources")]
fn maintainer_compares_register_and_sources(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "migration sources are empty");
}

#[given("the consolidated ADR register and canonical sources are available")]
fn consolidated_register_and_sources_are_available(world: &mut GivnWorld) {
    let register = fs::read_to_string(crate::common::repo_path("docs/arc42/adr/README.md"))
        .expect("read consolidated ADR register");
    let design = [
        "givn/changes/adr-qualification-gate/design.md",
        "givn/archive/adr-qualification-gate/design.md",
    ]
    .iter()
    .find_map(|path| fs::read_to_string(path).ok())
    .expect("read migration manifest");
    world.additional_output = Some(format!("{register}\n{design}"));
}

fn active_adr_path(id: &str) -> String {
    adr_path("docs/arc42/adr", id)
}

fn archived_adr_path(id: &str) -> String {
    adr_path("docs/arc42/adr/archive", id)
}

fn adr_path(directory: &str, id: &str) -> String {
    let prefix = format!("{id}-");
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            if name == format!("{id}.md") || name.starts_with(&prefix) {
                return path.to_string_lossy().into_owned();
            }
        }
    }
    format!("{directory}/{id}.md")
}

fn file_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

fn migration_ids(content: &str) -> HashSet<String> {
    content
        .lines()
        .filter_map(|line| {
            let id = line.split('|').nth(1)?.trim();
            id.strip_prefix("ADR-")
                .filter(|value| value.len() == 3 && value.chars().all(|ch| ch.is_ascii_digit()))
                .map(|value| format!("ADR-{value}"))
        })
        .collect()
}

#[then("the final register has 35 accepted decisions and two proposed decisions")]
fn final_register_has_expected_count(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 35, "unexpected accepted ADR count");
    assert_eq!(proposed, 2, "unexpected proposed ADR count");
}

#[then("the final register has 36 accepted decisions and three proposed decisions")]
fn final_register_has_three_proposed(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 36, "unexpected accepted ADR count");
    assert_eq!(proposed, 3, "unexpected proposed ADR count");
}

#[then("the final register has 36 accepted decisions and four proposed decisions")]
fn final_register_has_four_proposed(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 36, "unexpected accepted ADR count");
    assert_eq!(proposed, 4, "unexpected proposed ADR count");
}

#[then("the change's migration manifest gives every assigned identifier exactly one disposition")]
fn migration_manifest_covers_all_identifiers(world: &mut GivnWorld) {
    let design = guidance(world);
    let ids = migration_ids(design);
    assert_eq!(
        ids.len(),
        65,
        "migration manifest must contain 65 unique ids"
    );
    for number in 1..=65 {
        assert!(
            ids.contains(&format!("ADR-{number:03}")),
            "missing ADR-{number:03}"
        );
    }
}

#[then("reserved identifiers `ADR-050` and `ADR-051` have no record")]
fn reserved_ids_have_no_record(world: &mut GivnWorld) {
    let register = guidance(world);
    for id in ["ADR-050", "ADR-051"] {
        assert!(!file_exists(&active_adr_path(id)));
        assert!(!file_exists(&archived_adr_path(id)));
        assert!(!register.contains(&format!("[{}]", id)));
    }
}

#[then("non-ADR records are removed from the active register and ADR archive")]
fn non_adr_records_are_removed(world: &mut GivnWorld) {
    let register = guidance(world);
    for id in [
        "ADR-003", "ADR-006", "ADR-007", "ADR-012", "ADR-016", "ADR-018", "ADR-022", "ADR-029",
        "ADR-030", "ADR-031", "ADR-032", "ADR-033", "ADR-034", "ADR-035", "ADR-038", "ADR-040",
        "ADR-043", "ADR-049", "ADR-065",
    ] {
        assert!(
            !file_exists(&active_adr_path(id)),
            "active record remains: {id}"
        );
        assert!(
            !file_exists(&archived_adr_path(id)),
            "archived record remains: {id}"
        );
        assert!(
            !register.contains(&format!("[{id}]")),
            "register link remains: {id}"
        );
    }
}

#[then("absorbed records are absent and their refinements are in the target ADRs")]
fn absorbed_records_are_in_targets(world: &mut GivnWorld) {
    let register = guidance(world);
    for id in ["ADR-021", "ADR-025", "ADR-036", "ADR-062"] {
        assert!(
            !file_exists(&active_adr_path(id)),
            "absorbed record remains: {id}"
        );
        assert!(
            !register.contains(&format!("[{id}]")),
            "register link remains: {id}"
        );
    }

    let runner = fs::read_to_string(active_adr_path("ADR-015")).expect("read ADR-015");
    assert!(runner.contains("missing-testrunner"));
    let specification = fs::read_to_string(active_adr_path("ADR-011")).expect("read ADR-011");
    assert!(specification.contains("implementation-independent"));
    let agent = fs::read_to_string(active_adr_path("ADR-013")).expect("read ADR-013");
    assert!(agent.contains("Target selection") || agent.contains("target selection"));
    let serializer = fs::read_to_string(active_adr_path("ADR-059")).expect("read ADR-059");
    assert!(serializer.contains("Synthetic markers"));
}

#[then(
    "superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links"
)]
fn superseded_records_remain_archived(world: &mut GivnWorld) {
    let _ = world;
    for (id, replacement) in [
        ("ADR-019", "ADR-027"),
        ("ADR-042", "ADR-052"),
        ("ADR-053", "ADR-063"),
    ] {
        let path = archived_adr_path(id);
        let content =
            fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path}: {error}"));
        assert!(content.contains(&format!("status: superseded by {replacement}")));
        assert!(content.contains(replacement));
    }
}

#[then(
    "`ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications"
)]
fn explore_and_audit_adrs_are_spec_owned(world: &mut GivnWorld) {
    let _ = world;
    assert!(!file_exists(&active_adr_path("ADR-043")));
    assert!(!file_exists(&active_adr_path("ADR-049")));
    assert!(file_exists(
        "givn/specs/fragments/find-existing-scenarios.feature"
    ));
    assert!(file_exists(
        "givn/specs/discover-specifications/spec-duplicates.feature"
    ));
}

#[then("independent architecture decisions remain separate")]
fn independent_adrs_remain_separate(world: &mut GivnWorld) {
    let _ = world;
    for id in [
        "ADR-005", "ADR-008", "ADR-009", "ADR-010", "ADR-024", "ADR-039", "ADR-052", "ADR-059",
    ] {
        assert!(
            file_exists(&active_adr_path(id)),
            "independent ADR missing: {id}"
        );
    }
}

#[then("`ADR-063` remains proposed and owned by its active change")]
fn proposed_budget_adr_remains_owned(world: &mut GivnWorld) {
    let content = fs::read_to_string(active_adr_path("ADR-063")).expect("read ADR-063");
    assert!(content.contains("status: proposed"));
    let owned = [
        "givn/changes/semantic-reranker-budget/design.md",
        "givn/archive/semantic-reranker-budget/design.md",
    ]
    .iter()
    .any(|path| file_exists(path));
    assert!(owned, "semantic-reranker-budget design.md must exist");
    assert!(guidance(world).contains("ADR-063"));
}

fn guidance_sources() -> String {
    let paths = [
        "assets/addons/arc42-docs/instructions.md",
        "assets/addons/arc42-docs/impact-assessment-template.md",
        "assets/addons/arc42-docs/chapters/README.md",
        "assets/addons/arc42-docs/chapters/09-architecture-decisions.md",
        "assets/instructions/design-review.md",
        "assets/commands/givn-design-review.md",
        "assets/templates/design-review.md",
        "assets/skills/givn-design-review/SKILL.md.tmpl",
        ".opencode/commands/givn-design-review.md",
        ".opencode/skills/givn-design-review/SKILL.md",
        ".agents/skills/givn-design-review/SKILL.md",
        ".claude/skills/givn-design-review/SKILL.md",
    ];
    paths
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .collect::<Vec<_>>()
        .join("\n")
}

#[given("the canonical Arc42 qualification procedure and design-review guidance are available")]
fn canonical_and_derived_guidance_are_available(world: &mut GivnWorld) {
    let content = guidance_sources();
    assert!(!content.is_empty(), "guidance sources are empty");
    world.additional_output = Some(content);
}

#[when("an author compares the effective and generated guidance sources")]
fn author_compares_effective_and_generated_sources(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "guidance comparison has no sources"
    );
}

#[then("the canonical source uses strict AND qualification semantics")]
fn canonical_source_uses_strict_and(world: &mut GivnWorld) {
    let _ = world;
    let canonical = fs::read_to_string(crate::common::repo_path(
        "assets/addons/arc42-docs/instructions.md",
    ))
    .expect("read canonical qualification instruction");
    for clause in [
        "qualification = alternatives",
        "AND architectural_impact",
        "AND durable_consequence",
        "AND lower_level_artifact",
        "AND existing_adr_check",
    ] {
        assert!(
            canonical.contains(clause),
            "canonical source missing {clause}"
        );
    }
}

#[then("all sources that state qualification treat `Must be shared` as supporting evidence only")]
fn all_qualification_sources_treat_shared_as_supporting(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(!content.contains("at least one clearly justified"));
    for line in content
        .lines()
        .filter(|line| line.contains("Must be shared"))
    {
        assert!(
            line.contains("supporting") || content.contains("supporting evidence only"),
            "source contradicts support-only rule: {line}"
        );
    }
}

#[then("all sources that state routing use the same routing values and route targets")]
fn all_routing_sources_use_same_values(world: &mut GivnWorld) {
    let content = guidance(world);
    for route in [
        "NEW_ADR",
        "AMEND_ADR",
        "SUPERSEDE_ADR",
        "CANONICAL_ARTIFACT",
    ] {
        assert!(
            content.contains(route),
            "guidance source missing route {route}"
        );
    }
    assert!(content.contains("canonical_artifact"));
    assert!(content.contains("target_adr"));
    assert!(content.contains("replacement_adr"));
}

#[then("derived sources defer to the canonical procedure instead of copying its schema")]
fn derived_sources_defer_to_canonical(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("canonical Arc42 addon instruction"));
    assert!(content.contains("must not duplicate the complete procedure"));
    for path in [
        "assets/addons/arc42-docs/impact-assessment-template.md",
        "assets/addons/arc42-docs/chapters/README.md",
        "assets/addons/arc42-docs/chapters/09-architecture-decisions.md",
        "assets/instructions/design-review.md",
        "assets/commands/givn-design-review.md",
        "assets/templates/design-review.md",
        "assets/skills/givn-design-review/SKILL.md.tmpl",
        ".opencode/commands/givn-design-review.md",
        ".opencode/skills/givn-design-review/SKILL.md",
        ".agents/skills/givn-design-review/SKILL.md",
        ".claude/skills/givn-design-review/SKILL.md",
    ] {
        let derived = fs::read_to_string(path).unwrap_or_default();
        assert!(
            !derived.contains("qualification = alternatives"),
            "derived source copied the canonical formula: {path}"
        );
    }
}

#[then("no source claims that runtime validation enforces the policy")]
fn no_source_claims_runtime_enforcement(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("not runtime enforcement") || content.contains("not a Givn runtime"));
    assert!(!content.contains("runtime enforces the policy"));
}

#[then("a project override remains the effective source when configured")]
fn project_override_remains_effective(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("Project overrides may replace this default guidance"));
    assert!(content.contains("normal asset cascade"));
}

#[when("an author prepares an ADR candidate")]
fn author_prepares_adr_candidate(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "ADR candidate guidance is empty"
    );
}

#[then("the guidance requires real alternatives and supplied evidence")]
fn guidance_requires_real_alternatives_and_evidence(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("actual choice between at"));
    assert!(content.contains("least two materially different options"));
    assert!(content.contains("Evidence arrays contain only facts and"));
}

#[then("the guidance requires a decision classification and one canonical target")]
fn guidance_requires_classification_and_target(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("decision classification"));
    assert!(content.contains("exactly one canonical"));
}

#[then("the guidance requires an explicit duplicate amendment or supersession check")]
fn guidance_requires_relationship_check(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("search existing ADRs before proposing a new ID"));
    assert!(content.contains("AMEND_ADR"));
    assert!(content.contains("SUPERSEDE_ADR"));
}

#[then(
    "the MADR checklist is evidence for all mandatory gates rather than an at-least-one escape hatch"
)]
fn madr_checklist_is_evidence_not_escape(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(content.contains("checklist is evidence for those dimensions"));
    assert!(content.contains("not an at-least-one escape"));
}

// Step retained from the retired semantic-remediation fixture set: the ADR
// migration scenario reuses it to carry a distinguishing marker through the
// register fixtures. Validation-only — no fixture state.
#[given(regex = r#"^the remediation case carries the distinguishing marker "([^"]+)"$"#)]
fn remediation_case_marker(_world: &mut GivnWorld, marker: String) {
    assert!(!marker.trim().is_empty());
}

#[then("the final register has 26 accepted decisions and no proposed decisions")]
fn final_register_has_forward_only_adr(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 26, "unexpected accepted ADR count");
    assert_eq!(proposed, 0, "unexpected proposed ADR count");
}

#[then("the final register has 28 accepted decisions and no proposed decisions")]
fn final_register_has_witness_publication_adr(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 28, "unexpected accepted ADR count");
    assert_eq!(proposed, 0, "unexpected proposed ADR count");
}

#[then("the final register has 27 accepted decisions and no proposed decisions")]
fn final_register_has_scm_backend_adr(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 27, "unexpected accepted ADR count");
    assert_eq!(proposed, 0, "unexpected proposed ADR count");
}

#[then("the final register has 24 accepted decisions and no proposed decisions")]
fn final_register_has_retired_lint_adr(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 24, "unexpected accepted ADR count");
    assert_eq!(proposed, 0, "unexpected proposed ADR count");
}

#[then("the final register has 25 accepted decisions and no proposed decisions")]
fn final_register_has_post_engine_counts(world: &mut GivnWorld) {
    let register = guidance(world);
    let accepted = register
        .lines()
        .filter(|line| line.contains("| accepted |"))
        .count();
    let proposed = register
        .lines()
        .filter(|line| line.contains("| proposed |"))
        .count();
    assert_eq!(accepted, 25, "unexpected accepted ADR count");
    assert_eq!(proposed, 0, "unexpected proposed ADR count");
}

#[then("absorbed records are absent and their refinements remain readable")]
fn absorbed_records_remain_readable(world: &mut GivnWorld) {
    let register = guidance(world);
    for id in ["ADR-021", "ADR-025", "ADR-036", "ADR-062"] {
        assert!(
            !file_exists(&active_adr_path(id)),
            "absorbed record remains: {id}"
        );
        assert!(
            !register.contains(&format!("[{id}]")),
            "register link remains: {id}"
        );
    }

    let runner = fs::read_to_string(active_adr_path("ADR-015")).expect("read ADR-015");
    assert!(runner.contains("missing-testrunner"));
    let specification = fs::read_to_string(active_adr_path("ADR-011")).expect("read ADR-011");
    assert!(specification.contains("implementation-independent"));
    let agent = fs::read_to_string(active_adr_path("ADR-013")).expect("read ADR-013");
    assert!(agent.contains("Target selection") || agent.contains("target selection"));
    let serializer = fs::read_to_string(archived_adr_path("ADR-059")).expect("read ADR-059");
    assert!(serializer.contains("Synthetic markers"));
}

#[then(
    "the superseded semantic-engine records remain archived with replacement links to their replacement record"
)]
fn engine_records_remain_archived(world: &mut GivnWorld) {
    let _ = world;
    for id in [
        "ADR-039", "ADR-041", "ADR-046", "ADR-047", "ADR-052", "ADR-055", "ADR-056", "ADR-057",
        "ADR-058", "ADR-059", "ADR-060", "ADR-061", "ADR-063", "ADR-064", "ADR-066", "ADR-068",
    ] {
        let path = archived_adr_path(id);
        let content =
            fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path}: {error}"));
        assert!(content.contains("status: superseded by ADR-069"), "{id}");
        assert!(content.contains("ADR-069"), "{id}");
        assert!(
            !file_exists(&active_adr_path(id)),
            "active record remains: {id}"
        );
    }
    assert!(file_exists(&active_adr_path("ADR-069")));
}

// --- adr-qualification-hardening: falsification and verdict steps ----------

#[when("an author reads the falsification tests")]
fn author_reads_falsification_tests(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "qualification guidance is empty"
    );
}

#[then("the guidance fails a single-change reversible decision")]
fn guidance_fails_single_change_reversible(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("Single-change reversibility"),
        "guidance does not carry the single-change reversibility test"
    );
}

#[then("the guidance fails a decision that constrains no future change")]
fn guidance_fails_no_multi_change_constraint(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("No multi-change constraint"),
        "guidance does not carry the multi-change constraint test"
    );
}

#[then("the guidance fails workflow behavior unless it also moves a boundary")]
fn guidance_fails_product_behavior(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("Product-behavior exclusion"),
        "guidance does not carry the product-behavior exclusion test"
    );
}

#[then("the guidance names the misplaced-decision non-examples")]
fn guidance_names_non_examples(world: &mut GivnWorld) {
    let content = guidance(world);
    for marker in [
        "project identity key split",
        "terminal state value",
        "assumption-and-go-ahead gate",
        "human evaluation gate",
        "change-request entity",
    ] {
        assert!(
            content.contains(marker),
            "guidance is missing the non-example {marker:?}"
        );
    }
}

#[then("the guidance still qualifies a genuine boundary decision")]
fn guidance_still_qualifies_boundary(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("still qualifies"),
        "guidance does not state that a genuine boundary decision still qualifies"
    );
}

#[then("the guidance classifies by kind rather than count")]
fn guidance_classifies_by_kind(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("classify by kind, not count"),
        "guidance does not classify by kind rather than count"
    );
}

#[then("the guidance states there is no numeric ADR cap")]
fn guidance_states_no_numeric_cap(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("no numeric ADR cap"),
        "guidance does not state that there is no numeric ADR cap"
    );
}

#[given("the design template is available")]
fn design_template_is_available(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path("assets/templates/design.md"))
        .expect("read design template");
    world.additional_output = Some(content);
}

#[when("an author reads the Technology Decisions guidance")]
fn author_reads_technology_decisions(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "design template is empty");
}

#[then("the template says justification belongs in the design")]
fn template_says_justification_belongs_in_design(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("belongs in `design.md`"),
        "design template does not keep justification in the design"
    );
}

#[then("the template forbids ADR candidacy unless the falsification tests pass")]
fn template_forbids_adr_candidacy(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("falsification tests pass"),
        "design template does not gate ADR candidacy on the falsification tests"
    );
}

#[when("a maintainer reads the verdict placement rule")]
fn maintainer_reads_verdict_placement(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "Arc42 template is empty");
}

#[then("the marker requires the structured verdict before drafting")]
fn marker_requires_verdict_before_drafting(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("before creating or editing any MADR file"),
        "marker does not require the structured verdict before drafting"
    );
}

#[given("the ADR template is available")]
fn adr_template_is_available(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/addons/arc42-docs/chapters/adr/adr-template.md",
    ))
    .expect("read ADR template");
    world.additional_output = Some(content);
}

#[when("an author prepares to copy the template")]
fn author_prepares_to_copy_template(world: &mut GivnWorld) {
    assert!(!guidance(world).is_empty(), "ADR template is empty");
}

#[then("the template contains no pre-passed qualification verdict")]
fn template_contains_no_prepassed_verdict(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(
        !content.contains("ADR-QUALIFICATION: PASS"),
        "ADR template still ships a pre-passed qualification marker"
    );
    assert!(
        !content.contains("qualification: QUALIFIED"),
        "ADR template still ships a pre-passed qualification value"
    );
}

#[then("the template instructs the author to fill the verdict from the record")]
fn template_instructs_fill_from_record(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("Fill every value from the structured verdict"),
        "ADR template does not instruct filling the verdict from the record"
    );
}

#[given("the design-review instruction is available")]
fn design_review_instruction_is_available(world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/instructions/design-review.md",
    ))
    .expect("read design-review instruction");
    world.additional_output = Some(content);
}

#[when("a reviewer reads the ADR branch")]
fn reviewer_reads_adr_branch(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "design-review instruction is empty"
    );
}

#[then("the instruction requires the cheaper-home challenge for every proposed new ADR")]
fn instruction_requires_cheaper_home(world: &mut GivnWorld) {
    let content = guidance(world);
    assert!(
        content.contains("cheaper home") && content.contains("NEW_ADR"),
        "design-review instruction does not require the cheaper-home challenge"
    );
}

#[then("the design-review command requires the cheaper-home challenge")]
fn command_requires_cheaper_home(_world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/commands/givn-design-review.md",
    ))
    .expect("read design-review command");
    assert!(
        content.contains("cheaper home"),
        "design-review command does not require the cheaper-home challenge"
    );
}

#[then("the design-review skill points at the requirement")]
fn skill_points_at_cheaper_home(_world: &mut GivnWorld) {
    let content = std::fs::read_to_string(crate::common::repo_path(
        "assets/skills/givn-design-review/SKILL.md.tmpl",
    ))
    .expect("read design-review skill template");
    assert!(
        content.contains("cheaper home"),
        "design-review skill does not point at the cheaper-home challenge"
    );
}

#[when("an author reads the chapter-selection guidance")]
fn author_reads_chapter_selection(world: &mut GivnWorld) {
    assert!(
        !guidance(world).is_empty(),
        "qualification guidance is empty"
    );
}

#[then("the guidance treats correlated rows as a double-check")]
fn guidance_treats_correlated_rows_as_double_check(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("treat correlated rows as a prompt to"),
        "guidance does not treat correlated rows as a double-check"
    );
}

#[then("the guidance says an affected chapter does not by itself qualify an ADR")]
fn guidance_affected_chapter_does_not_qualify(world: &mut GivnWorld) {
    assert!(
        guidance(world).contains("does not by itself qualify an ADR"),
        "guidance does not say an affected chapter fails to qualify an ADR"
    );
}

#[then("the guidance does not normalize an ADR per change")]
fn guidance_does_not_normalize_adr_per_change(world: &mut GivnWorld) {
    assert!(
        !guidance(world).contains("a single change routinely triggers rows 3, 5, 6, 9, and 11"),
        "guidance still normalizes a routine row set that includes row 9"
    );
}
