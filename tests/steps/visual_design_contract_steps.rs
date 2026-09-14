use cucumber::then;

use crate::steps::GivnWorld;

fn project_root(world: &GivnWorld) -> std::path::PathBuf {
    world
        .env
        .as_ref()
        .map(|env| env.path(""))
        .expect("a project root is required first")
}

#[then(regex = r##"^the ejected design template contains "(.+)"$"##)]
fn ejected_design_template_contains(world: &mut GivnWorld, needle: String) {
    let stdout = crate::steps::stdout(world);
    assert!(
        stdout.contains("givn/artifacts/design/design.md"),
        "eject output did not identify the design template path: {stdout}"
    );

    let path = project_root(world).join("givn/artifacts/design/design.md");
    let content = std::fs::read_to_string(&path).expect("read ejected design template");
    assert!(
        content.contains(&needle),
        "ejected design template missing {needle:?}: {content}"
    );
}

fn ejected_template_contains(world: &mut GivnWorld, artifact: &str, file: &str, needle: &str) {
    let stdout = crate::steps::stdout(world);
    let expected = format!("givn/artifacts/{artifact}/{file}");
    assert!(
        stdout.contains(&expected),
        "eject output did not identify the {artifact} template path: {stdout}"
    );

    let path = project_root(world).join(&expected);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read ejected {artifact} template {}: {error}", path.display()));
    assert!(
        content.contains(needle),
        "ejected {artifact} template missing {needle:?}: {content}"
    );
}

#[then(regex = r##"^the ejected proposal template contains "(.+)"$"##)]
fn ejected_proposal_template_contains(world: &mut GivnWorld, needle: String) {
    ejected_template_contains(world, "proposal", "proposal.md", &needle);
}

#[then(regex = r##"^the ejected design-review template contains "(.+)"$"##)]
fn ejected_design_review_template_contains(world: &mut GivnWorld, needle: String) {
    ejected_template_contains(world, "design-review", "design-review.md", &needle);
}
