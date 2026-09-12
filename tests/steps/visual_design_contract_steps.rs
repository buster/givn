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
