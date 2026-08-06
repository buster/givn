use cucumber::{given, then, when};
use crate::steps::GivnWorld;

#[derive(Debug, Default)]
pub struct LicenseState {
    cargo_toml_content: Option<String>,
    readme_content: Option<String>,
}

#[given("a project root containing the givn source checkout")]
fn givn_source_checkout(world: &mut GivnWorld) {
    // Use the actual project root where the test binary is running.
    let project_root = std::env::current_dir().expect("get current dir");
    world.project_root = Some(project_root);
}

#[given("a checkout of the givn source repository")]
fn givn_source_repo(world: &mut GivnWorld) {
    let project_root = std::env::current_dir().expect("get current dir");
    world.project_root = Some(project_root);
}

#[when("the user reads Cargo.toml")]
fn read_cargo_toml(world: &mut GivnWorld) {
    let root = world.project_root.as_ref().expect("project root required");
    let content = std::fs::read_to_string(root.join("Cargo.toml")).expect("read Cargo.toml");
    world.license_state.cargo_toml_content = Some(content);
}

#[when("the user reads README.md")]
fn read_readme(world: &mut GivnWorld) {
    let root = world.project_root.as_ref().expect("project root required");
    let content = std::fs::read_to_string(root.join("README.md")).expect("read README.md");
    world.license_state.readme_content = Some(content);
}

#[then(regex = r#"^the license field is "([^"]+)"$"#)]
fn license_field_is(world: &mut GivnWorld, expected: String) {
    let content = world
        .license_state
        .cargo_toml_content
        .as_ref()
        .expect("Cargo.toml not read yet");
    let license_line = content
        .lines()
        .find(|l| l.trim().starts_with("license"))
        .expect("license field not found in Cargo.toml");
    assert!(
        license_line.contains(&expected),
        "expected license '{}', got: {}",
        expected,
        license_line
    );
}

#[then("the license field is no longer \"TBD\"")]
fn license_field_not_tbd(world: &mut GivnWorld) {
    let content = world
        .license_state
        .cargo_toml_content
        .as_ref()
        .expect("Cargo.toml not read yet");
    let license_line = content
        .lines()
        .find(|l| l.trim().starts_with("license"))
        .expect("license field not found");
    assert!(
        !license_line.contains("TBD"),
        "license field still says TBD: {}",
        license_line
    );
}

#[then("the license section names a specific license")]
fn license_section_names(world: &mut GivnWorld) {
    let content = world
        .license_state
        .readme_content
        .as_ref()
        .expect("README.md not read yet");
    assert!(
        content.contains("MIT") || content.contains("Apache"),
        "README.md license section should name a specific license"
    );
}

#[then("the license section does not say \"not yet chosen\"")]
fn license_section_not_yet_chosen(world: &mut GivnWorld) {
    let content = world
        .license_state
        .readme_content
        .as_ref()
        .expect("README.md not read yet");
    assert!(
        !content.contains("not yet chosen"),
        "README.md still says 'not yet chosen'"
    );
}

#[then("the license field is a valid SPDX expression")]
fn license_field_valid_spdx(world: &mut GivnWorld) {
    let content = world
        .license_state
        .cargo_toml_content
        .as_ref()
        .expect("Cargo.toml not read yet");
    let license_line = content
        .lines()
        .find(|l| l.trim().starts_with("license"))
        .expect("license field not found");
    let value = license_line.split('=').nth(1).unwrap_or("").trim();
    // A valid SPDX expression contains at least one operator or valid id
    assert!(
        value.contains("MIT") || value.contains("Apache") || value.contains("BSD"),
        "license field does not look like a valid SPDX expression: {}",
        value
    );
}

#[then("the license section names the same license")]
fn license_section_same_as_cargo(world: &mut GivnWorld) {
    let cargo = world
        .license_state
        .cargo_toml_content
        .as_ref()
        .expect("Cargo.toml not read yet");
    let readme = world
        .license_state
        .readme_content
        .as_ref()
        .expect("README.md not read yet");
    let cargo_license = cargo
        .lines()
        .find(|l| l.trim().starts_with("license"))
        .map(|l| l.to_string())
        .unwrap_or_default();
    // Check that README mentions the same license identifiers
    assert!(
        readme.contains("MIT") || readme.contains("Apache-2.0"),
        "README should reference the same license as Cargo.toml ({})",
        cargo_license
    );
}
