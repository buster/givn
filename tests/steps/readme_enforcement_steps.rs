//! Unit-level step definitions for the README enforcement capability.
//!
//! Fixtures construct deterministic README states on an initialised project
//! root; assertions reuse the shared subprocess harness (`tests/common`).

use cucumber::{given, then};

use super::GivnWorld;

// --- README state fixtures ---------------------------------------------------

/// The new-style scaffold shape: open placeholder markers plus an intact
/// development managed block.
pub(crate) fn scaffolded_readme_text() -> String {
    "\
# readme-demo-project

<!-- givn:todo: One-paragraph description of what this project does and who it is for. -->

## Quick start

<!-- givn:todo: Exact commands to clone, build, and run the project. -->

## Prerequisites

<!-- givn:todo: Required tools, languages, or runtimes. -->

## Usage

<!-- givn:todo: A brief example or --help output showing how to use the project. -->

## Contributing

<!-- givn:todo: PR guidelines, commit conventions, code style, review process. -->

## License

<!-- givn:todo: The project's license (SPDX identifier) — not TBD. -->

<!-- givn:begin:development -->
## Development

Run the project-owned scripts used by `givn/commands.yaml`:

```sh
./run-tests.sh
./run-tests.sh --e2e
```
<!-- givn:end:development -->
"
    .to_string()
}

fn write_readme(world: &mut GivnWorld, content: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    std::fs::write(env.path("README.md"), content).expect("write README.md");
}

#[given("a scaffolded README with open placeholders")]
fn scaffolded_readme(world: &mut GivnWorld) {
    write_readme(world, &scaffolded_readme_text());
}

/// The verbatim previous scaffold template (pre-marker style).
pub(crate) fn legacy_readme_text() -> String {
    "\
# {{PROJECT_NAME}}

<!-- givn:begin:coverage-badge -->
[![Line Coverage: n/a](https://img.shields.io/badge/line%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

<!-- One-paragraph description of what this project does and who it is for. -->

## Quick start

<!--
```
git clone <repo>
cd <project>
make setup
```
-->

## Prerequisites

<!-- List any required tools, languages, or runtimes. -->

## Usage

<!-- Brief example or `--help` output to show how to use the project. -->

## Development

Run the project-owned scripts used by `givn/commands.yaml`:

```sh
./run-tests.sh
./run-tests.sh --e2e
./measure-coverage.sh
./merge-coverages.sh
```

## Contributing

<!--
PR guidelines, commit conventions (Conventional Commits), code style,
review process.
-->

## License

TBD
"
    .to_string()
}

#[given("a README with the previous scaffold style")]
fn legacy_readme(world: &mut GivnWorld) {
    write_readme(world, &legacy_readme_text());
}

#[given("a README with a broken development managed block")]
fn broken_block_readme(world: &mut GivnWorld) {
    // Complete prose, but the development managed block lost its end marker.
    let text = scaffolded_readme_text()
        .replace(
            "<!-- givn:todo: One-paragraph description of what this project does and who it is for. -->",
            "A small command-line tool that prints greetings.",
        )
        .replace("<!-- givn:todo: Exact commands to clone, build, and run the project. -->", "cargo install this-project")
        .replace("<!-- givn:todo: Required tools, languages, or runtimes. -->", "Rust 1.97.1")
        .replace("<!-- givn:todo: A brief example or --help output showing how to use the project. -->", "this-project --help")
        .replace("<!-- givn:todo: PR guidelines, commit conventions, code style, review process. -->", "Conventional Commits.")
        .replace("<!-- givn:todo: The project's license (SPDX identifier) — not TBD. -->", "MIT")
        .replace("<!-- givn:end:development -->\n", "");
    write_readme(world, &text);
}

#[given("a complete README with the development managed block")]
fn complete_readme(world: &mut GivnWorld) {
    unimplemented!("write a complete README with an intact development managed block")
}

#[given("a hand-written README without any givn markers")]
fn hand_written_readme(world: &mut GivnWorld) {
    write_readme(
        world,
        "# my project\n\nJust some prose describing the project.\n",
    );
}

#[given("a complete README whose only managed block is the coverage badge")]
fn badge_only_readme(world: &mut GivnWorld) {
    unimplemented!("write a complete README keeping only the coverage badge managed block")
}

#[given("a complete README whose development managed block is stale")]
fn stale_block_readme(world: &mut GivnWorld) {
    unimplemented!("write a complete README with a stale development managed block")
}

#[given("the README is missing")]
fn readme_missing(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("README.md");
    if path.exists() {
        std::fs::remove_file(&path).expect("remove README.md");
    }
}

// --- Change fixtures ---------------------------------------------------------

#[given(regex = r#"^a reviewable change "([^"]+)"$"#)]
fn reviewable_change(world: &mut GivnWorld, id: String) {
    unimplemented!("create a change with all artifacts and no delta specs")
}

#[given(regex = r#"^the review of change "([^"]+)" records the README impact decision "(.*)"$"#)]
fn review_impact_decision(world: &mut GivnWorld, id: String, decision: String) {
    unimplemented!("append the README-IMPACT line to the change's review.md")
}

// --- Then --------------------------------------------------------------------

#[then("the README still has no development managed block")]
fn readme_has_no_dev_block(world: &mut GivnWorld) {
    unimplemented!("assert the README has no development managed block markers")
}
