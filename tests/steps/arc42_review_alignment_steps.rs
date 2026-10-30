use crate::steps::GivnWorld;
use cucumber::{given, then, when};

fn project_root(world: &GivnWorld) -> std::path::PathBuf {
    world
        .env
        .as_ref()
        .map(|env| env.path(""))
        .expect("a project root is required first")
}

fn last_stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[then(regex = r##"^the ejected review template contains "(.+)"$"##)]
fn ejected_review_template_contains(world: &mut GivnWorld, needle: String) {
    let stdout = last_stdout(world);
    assert!(
        stdout.contains("givn/artifacts/review/review.md"),
        "eject output did not identify the review template path: {stdout}"
    );

    let path = project_root(world).join("givn/artifacts/review/review.md");
    let content = std::fs::read_to_string(&path).expect("read ejected review template");
    assert!(
        content.contains(&needle),
        "ejected review template missing {needle:?}: {content}"
    );
}

#[then(regex = r##"^the generated review guidance contains "(.+)"$"##)]
fn generated_review_guidance_contains(world: &mut GivnWorld, needle: String) {
    let root = project_root(world);
    let config = std::fs::read_to_string(root.join("givn/config.yaml")).unwrap_or_default();
    let mut targets: Vec<String> = vec!["opencode".to_string()];
    let mut in_skills = false;
    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed == "skills:" {
            in_skills = true;
            continue;
        }
        if in_skills {
            if !line.is_empty() && !line.starts_with(' ') {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("targets:") {
                targets = rest
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }

    for target in &targets {
        let paths = [
            format!(".{target}/skills/givn-review/SKILL.md"),
            ".opencode/commands/givn-review.md".to_string(),
        ];
        for relative in paths {
            let path = root.join(&relative);
            if !path.exists() {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap_or_else(|error| {
                panic!("read generated review guidance {relative}: {error}")
            });
            assert!(
                content.contains(&needle),
                "generated review guidance {relative} missing {needle:?}"
            );
        }
    }
}

// --- prepare-0-9-10: architecture documentation and review ------------------

fn guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("architecture guidance was not loaded")
}

fn normalized_guidance(world: &GivnWorld) -> String {
    guidance(world)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn load_asset(world: &mut GivnWorld, relative: &str) {
    let content = std::fs::read_to_string(crate::common::repo_path(relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"));
    world.additional_output = Some(content);
}

#[given("a project using the Arc42 addon without a completed architecture baseline")]
fn project_without_completed_arc42_baseline(world: &mut GivnWorld) {
    load_asset(world, "assets/addons/arc42-docs/instructions.md");
}

#[when("the author reads the architecture documentation guidance")]
fn author_reads_architecture_documentation_guidance(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "architecture guidance is empty"
    );
}

#[then(
    "the first completed change must establish all twelve chapters proportionate to the real project"
)]
fn first_change_establishes_twelve_chapters(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("first completed change must establish all twelve chapters"),
        "guidance does not require the twelve-chapter baseline"
    );
    assert!(
        content.contains("proportionate to the real"),
        "guidance does not require proportionate content"
    );
}

#[then("unfinished scaffold and fabricated completeness do not satisfy the baseline")]
fn scaffold_does_not_satisfy_baseline(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("Unfinished scaffold or fabricated completeness does not satisfy"),
        "guidance accepts scaffold as a baseline"
    );
}

#[then("specific explicit unknowns and explained inapplicability are acceptable content")]
fn explicit_unknowns_are_acceptable(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("Specific explicit unknowns and explained inapplicability are")
            && content.contains("acceptable content"),
        "guidance does not allow explicit unknowns or inapplicability"
    );
}

#[given("a project with a completed twelve-chapter Arc42 baseline")]
fn project_with_completed_baseline(world: &mut GivnWorld) {
    load_asset(world, "assets/addons/arc42-docs/instructions.md");
}

#[when("the author reads the architecture maintenance guidance for a later change")]
fn author_reads_architecture_maintenance_guidance(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "maintenance guidance is empty"
    );
}

#[then("affected chapters must reflect the implemented change")]
fn affected_chapters_reflect_change(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("maintain only the chapters affected by the implemented change"),
        "maintenance guidance does not require affected chapters to reflect the change"
    );
}

#[then("unchanged chapters do not require repeated whole-baseline approval")]
fn unchanged_chapters_need_no_reapproval(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("unchanged chapters do not require repeated whole-baseline approval"),
        "maintenance guidance re-approves the whole baseline"
    );
}

#[given("a change's first implementation is ready for review")]
fn first_implementation_ready_for_review(world: &mut GivnWorld) {
    load_asset(world, "assets/instructions/review.md");
}

#[when("the reviewer reads the architecture conformance guidance")]
fn reviewer_reads_architecture_conformance(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "review guidance is empty"
    );
}

#[then(
    "review checks cohesive responsibilities narrow consumer contracts external adapters separate composition and explicit dependency direction"
)]
fn review_checks_substantive_boundaries(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    for marker in [
        "cohesive responsibilities",
        "narrow consumer contracts",
        "external adapters separate from the domain",
        "separate composition root",
        "explicit dependency direction",
    ] {
        assert!(
            content.contains(marker),
            "architecture review is missing {marker:?}"
        );
    }
}

#[then("review confirms actual imports and calls rather than inferring boundaries from filenames")]
fn review_confirms_imports_and_calls(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "confirmed actual imports\n  and calls rather than inferring boundaries from filenames"
        ) || normalized_guidance(world).contains(
            "confirmed actual imports and calls rather than inferring boundaries from filenames"
        ),
        "architecture review infers boundaries from filenames"
    );
}

#[then(
    "narrow module APIs may satisfy a contract while explicit interfaces are required where isolation or substitution needs them"
)]
fn narrow_module_apis_may_satisfy_contract(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("narrow module API may satisfy a contract unless isolation or substitution"),
        "architecture review demands an interface everywhere"
    );
}

#[then(
    "confirmed Personas remain stakeholder review lenses without invented identities or an internal-change exemption"
)]
fn personas_remain_review_lenses(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("confirmed Personas remain stakeholder review lenses"),
        "architecture review does not keep Personas as review lenses"
    );
    assert!(
        content.contains("without invented") && content.contains("internal-change exemption"),
        "architecture review invents Personas or exempts internal changes"
    );
}

#[given("one substantive review has recorded material findings and minor naming observations")]
fn substantive_review_with_findings(world: &mut GivnWorld) {
    load_asset(world, "assets/instructions/review.md");
}

#[when("the reviewer reads the closure guidance")]
fn reviewer_reads_closure_guidance(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "closure guidance is empty"
    );
}

#[then(
    "one targeted closure checks the findings and material regressions introduced by their fixes"
)]
fn closure_checks_findings_and_regressions(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("One targeted closure checks"),
        "review guidance does not define one targeted closure"
    );
    assert!(
        normalized_guidance(world).contains("material regressions introduced by their fixes"),
        "closure does not check fix regressions"
    );
}

#[then("critical defects remain repairable blockers")]
fn critical_defects_remain_blockers(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("Critical defects remain repairable blockers"),
        "closure treats critical defects as non-blocking"
    );
}

#[then(
    "minor naming style polish and unrelated findings go to the existing backlog or a separate follow-up batch"
)]
fn minor_findings_go_to_backlog(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("Minor naming, style polish, and")
            && normalized_guidance(world)
                .contains("existing backlog or a separate follow-up batch"),
        "closure absorbs minor or unrelated findings into the audit"
    );
}

#[then("closure does not initiate another open-ended audit or redundant full-suite run")]
fn closure_is_bounded(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("not another open-ended audit")
            && content.contains("does not require a redundant full-suite run"),
        "closure opens another audit or reruns the suite"
    );
}

#[given("an outline report is available for changed components")]
fn outline_report_available(world: &mut GivnWorld) {
    load_asset(world, "assets/instructions/review.md");
}

#[when("the reviewer reads the final architecture review guidance")]
fn reviewer_reads_final_architecture_guidance(world: &mut GivnWorld) {
    assert!(
        !normalized_guidance(world).is_empty(),
        "final review guidance is empty"
    );
}

#[then(
    "final review reuses relevant outline evidence from implementation unless structural changes have made it stale"
)]
fn final_review_reuses_outline_evidence(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("reused unless structural changes have made its relevant evidence stale"),
        "final review does not reuse outline evidence until stale"
    );
    assert!(
        normalized_guidance(world).contains("only structurally stale"),
        "final review reruns the outline for nonstructural edits"
    );
}

#[then("only structurally stale relevant outline evidence requires rerunning the outline")]
fn only_stale_outline_requires_rerun(world: &mut GivnWorld) {
    let content = normalized_guidance(world).to_lowercase();
    assert!(
        content.contains("only structurally stale")
            && content.contains("nonstructural edits alone do not require a rerun"),
        "outline rerun is not tied to structural staleness"
    );
}

#[then("the outline supports substantive source-confirmed architecture review")]
fn outline_supports_source_confirmed_review(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world)
            .contains("supports substantive, source-confirmed architecture review"),
        "outline is not connected to source-confirmed review"
    );
}

#[then(
    "mixed-language coverage is disclosed without forcing outline analysis of unsupported components"
)]
fn unsupported_coverage_is_disclosed(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("Disclose which languages and components it covers and which it omits"),
        "outline coverage is not disclosed"
    );
    assert!(
        content.contains("Unsupported components receive direct source assessment"),
        "unsupported components are forced through the outline"
    );
}

#[then("the reviewer must reconstruct responsibility and ordered call flow from the outline alone")]
fn reviewer_reconstructs_unit_from_outline(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("reconstruct each changed unit from its outline")
            && content
                .contains("state its responsibility in one sentence and its ordered call flow")
            && content.contains("without reading bodies or comments"),
        "review guidance does not require responsibility and call flow from the outline"
    );
}

#[then("names parameters return types and type names must state the domain operation")]
fn review_names_state_domain_operation(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "unit and type names, method names, parameter names and types, return types, and called type names must state the domain operation in the project's vocabulary"
        ),
        "review guidance does not require domain names on the outline"
    );
}

#[then("a generic or machinery name is a substantive readability finding")]
fn review_generic_name_is_finding(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains(
            "A generic or machinery name where a domain noun exists is a readability finding"
        ),
        "review guidance accepts generic machinery names"
    );
}

#[then("an unreadable changed unit rejects PASS until made readable")]
fn review_unreadable_unit_rejects_pass(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("record a substantive readability finding")
            && content.contains("reject PASS until the unit is made readable"),
        "review guidance does not reject PASS for an unreadable unit"
    );
}

#[then("the report itself supplies no verdict metric waiver or second review context")]
fn report_supplies_no_verdict(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    for marker in [
        "no verdict",
        "no metric waiver",
        "no forced reviewer context",
    ] {
        assert!(
            content.contains(marker),
            "outline report guidance still supplies {marker:?}"
        );
    }
    assert!(
        !content.contains("OUTLINE: PASS") && !content.contains("OUTLINE: FAIL"),
        "outline report guidance requires an outline verdict"
    );
}

#[then(
    "unsupported components receive direct source assessment using the same architecture questions"
)]
fn unsupported_components_source_assessed(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    assert!(
        content.contains("same responsibility, interface, dependency-direction, and decomposition"),
        "unsupported components do not use the same architecture questions"
    );
    assert!(
        content.contains("not a failure and needs no waiver"),
        "unsupported components are treated as failure or waived"
    );
}

#[then(
    "no outline verdict metric waiver forced reviewer context or independent outline review is required"
)]
fn no_outline_verdict_or_waiver(world: &mut GivnWorld) {
    let content = normalized_guidance(world);
    for marker in [
        "no verdict",
        "no metric waiver",
        "no forced reviewer context",
        "no authority to block review or archive",
    ] {
        assert!(
            content.contains(marker),
            "outline guidance still requires {marker:?}"
        );
    }
}

#[then("the outline alone cannot block review or archive")]
fn outline_cannot_block(world: &mut GivnWorld) {
    assert!(
        normalized_guidance(world).contains("The outline alone cannot block review or archive"),
        "outline is still a blocking verdict"
    );
}

// --- @e2e steps -------------------------------------------------------------

#[then(
    "the returned guidance compares Arc42 design tasks and completed implementation against available evidence"
)]
fn returned_guidance_compares_design_tasks_and_implementation(world: &mut GivnWorld) {
    let content = last_stdout(world);
    assert!(
        content.contains("Arc42 design tasks and the completed implementation against available"),
        "review guidance does not compare design tasks and the completed implementation: {content}"
    );
    assert!(
        content.contains("not an unwritten review verdict or a future archive receipt"),
        "review guidance demands an unwritten verdict or future receipt: {content}"
    );
}

#[then("the guidance requires recording and resolving material conformance mismatches")]
fn guidance_requires_resolving_arc42_mismatches(world: &mut GivnWorld) {
    assert!(
        last_stdout(world).contains("record and resolve every material conformance mismatch"),
        "review guidance does not require resolving conformance mismatches"
    );
}

#[then(
    "the guidance does not require an unwritten review verdict or future archive receipt as evidence"
)]
fn guidance_rejects_unwritten_verdict(world: &mut GivnWorld) {
    assert!(
        last_stdout(world).contains("not an unwritten review verdict or a future archive receipt"),
        "review guidance accepts an unwritten verdict or future receipt"
    );
}

#[then(
    "the ejected template records Arc42 implementation conformance against design tasks and completed implementation"
)]
fn ejected_template_records_arc42_conformance(world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(project_root(world).join("givn/artifacts/review/review.md"))
            .expect("read ejected review template");
    assert!(
        content.contains("Arc42 design tasks and the completed")
            && content.contains("implementation against available evidence"),
        "ejected review template does not record implementation conformance: {content}"
    );
}

#[then("the template records evidence and material mismatches")]
fn template_records_evidence_and_mismatches(world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(project_root(world).join("givn/artifacts/review/review.md"))
            .expect("read ejected review template");
    let normalized = content.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains("records evidence and material mismatches"),
        "ejected review template lacks evidence/mismatch wording: {content}"
    );
}

#[then("the template contains no outline verdict phrase")]
fn template_contains_no_outline_verdict(world: &mut GivnWorld) {
    let content =
        std::fs::read_to_string(project_root(world).join("givn/artifacts/review/review.md"))
            .expect("read ejected review template");
    assert!(
        !content.to_lowercase().contains("outline verdict"),
        "ejected review template still carries a retired-gate phrase: {content}"
    );
}
