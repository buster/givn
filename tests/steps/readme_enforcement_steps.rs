//! Unit-level step definitions for the README enforcement capability.
//!
//! Fixtures construct deterministic README states on an initialised project
//! root; assertions reuse the shared subprocess harness (`tests/common`).

use cucumber::{given, then};

use super::GivnWorld;
use crate::common;

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

#[given("a scaffolded README")]
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

#[given("a legacy scaffold README")]
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

#[given("a complete README")]
fn complete_readme(world: &mut GivnWorld) {
    let text = scaffolded_readme_text()
        .replace(
            "<!-- givn:todo: One-paragraph description of what this project does and who it is for. -->",
            "A small command-line tool that prints greetings.",
        )
        .replace("<!-- givn:todo: Exact commands to clone, build, and run the project. -->", "cargo install this-project")
        .replace("<!-- givn:todo: Required tools, languages, or runtimes. -->", "Rust 1.97.1")
        .replace("<!-- givn:todo: A brief example or --help output showing how to use the project. -->", "this-project --help")
        .replace("<!-- givn:todo: PR guidelines, commit conventions, code style, review process. -->", "Conventional Commits.")
        .replace("<!-- givn:todo: The project's license (SPDX identifier) — not TBD. -->", "MIT");
    write_readme(world, &text);
}

#[given("a hand-written README")]
fn hand_written_readme(world: &mut GivnWorld) {
    write_readme(
        world,
        "# my project\n\nJust some prose describing the project.\n",
    );
}

#[given("a badge-only README")]
fn badge_only_readme(world: &mut GivnWorld) {
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
        .replace(
            "<!-- givn:begin:development -->\n## Development\n\nRun the project-owned scripts used by `givn/commands.yaml`:\n\n```sh\n./run-tests.sh\n./run-tests.sh --e2e\n```\n<!-- givn:end:development -->\n",
            "<!-- givn:begin:coverage-badge -->\n[![Line Coverage: n/a](https://img.shields.io/badge/line%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)\n[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)\n<!-- givn:end:coverage-badge -->\n",
        );
    write_readme(world, &text);
}

#[given("a stale development managed block")]
fn stale_block_readme(world: &mut GivnWorld) {
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
        .replace(
            "Run the project-owned scripts used by `givn/commands.yaml`:\n\n```sh\n./run-tests.sh\n./run-tests.sh --e2e\n```",
            "Run the project-owned scripts:\n\n```sh\nstale-command-listing\n```",
        );
    write_readme(world, &text);
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

#[given("verify commands are no-ops")]
fn verify_commands_noops(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let yaml = "\
verify:
  command: \"true\"
  e2e_command: \"true\"
coverage:
  measure_command: \"true\"
  merge_command: \"true\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
  output_path: \"coverage/cobertura-coverage.xml\"
";
    std::fs::write(env.path("givn/commands.yaml"), yaml).expect("write commands.yaml");
}

#[given("the semantic gate is offline")]
fn semantic_gate_offline(world: &mut GivnWorld) {
    // The subprocess drives the blocking semantic gate during archive; these
    // world fields flow into `world_envs` so the gate runs stubbed and
    // offline instead of downloading model artifacts.
    world.retrieval_embedder = Some("stub".to_string());
    world.retrieval_bge = Some("stub".to_string());
    world.retrieval_reranker = Some("disabled".to_string());
}

// --- Change fixtures ---------------------------------------------------------

/// Write a change directory with every artifact complete and no delta specs,
/// so `givn check review` exercises the mandatory checks without the
/// retrieval/overlap gates.
pub(crate) fn write_reviewable_change(world: &mut GivnWorld, id: &str) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{id}"));
    std::fs::create_dir_all(&change_dir).expect("create change dir");
    std::fs::write(change_dir.join("proposal.md"), "# Proposal\n").expect("write proposal");
    std::fs::write(change_dir.join("design.md"), "# Design\n").expect("write design");
    std::fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design-review");
    std::fs::write(
        change_dir.join("tasks.md"),
        "- [x] COMMIT: a1b2c3d4 -- all scenarios done\n",
    )
    .expect("write tasks");
    std::fs::write(change_dir.join("review.md"), "REVIEW: PASS\n").expect("write review");
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n")
        .expect("write arc42");
    world.last_change_id = Some(id.to_string());
}

#[given(regex = r#"^a reviewable change "([^"]+)"$"#)]
fn reviewable_change(world: &mut GivnWorld, id: String) {
    write_reviewable_change(world, &id);
}

#[given(regex = r#"^a completed change "([^"]+)"$"#)]
fn completed_change_short(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let change_dir = env.path(&format!("givn/changes/{id}"));
    let out = common::run_givn(env, &["new", &id]);
    assert!(
        out.status.success(),
        "givn new {id} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::write(
        change_dir.join("proposal.md"),
        crate::steps::fixture_routing_proposal("example"),
    )
    .expect("write proposal");
    std::fs::write(change_dir.join("design.md"), "# Design\n").expect("write design");
    std::fs::write(change_dir.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design-review");
    std::fs::write(change_dir.join("tasks.md"), "- [x] all done\n").expect("write tasks");
    std::fs::write(change_dir.join("review.md"), "REVIEW: PASS\n").expect("write review");
    std::fs::write(change_dir.join("arc42.md"), "## Status\n\nSTATUS: DONE\n")
        .expect("write arc42");
    let specs_dir = change_dir.join("specs").join("example");
    std::fs::create_dir_all(&specs_dir).expect("create specs dir");
    std::fs::write(
        specs_dir.join("example.feature"),
        "@givn.delta\nFeature: example\n  Scenario: stub\n    Given a placeholder step\n",
    )
    .expect("write delta spec");
    world.last_change_id = Some(id);
}

#[given(regex = r#"^its review records the impact "(.*)"$"#)]
fn review_records_impact(world: &mut GivnWorld, decision: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let id = world
        .last_change_id
        .as_deref()
        .expect("a change id is required first");
    let path = env.path(&format!("givn/changes/{id}/review.md"));
    let existing = std::fs::read_to_string(&path).expect("read review.md");
    std::fs::write(&path, format!("{existing}README-IMPACT: {decision}\n"))
        .expect("write README-IMPACT line");
}

// --- Then --------------------------------------------------------------------

#[then("the development block stays deleted")]
fn readme_has_no_dev_block(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        !content.contains("<!-- givn:begin:development -->")
            && !content.contains("<!-- givn:end:development -->"),
        "README unexpectedly contains development managed block markers"
    );
}
