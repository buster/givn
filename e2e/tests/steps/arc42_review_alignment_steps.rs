use crate::steps::GivnWorld;
use cucumber::then;

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

fn guidance(world: &GivnWorld) -> &str {
    world
        .additional_output
        .as_deref()
        .expect("architecture guidance was not loaded")
}

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
