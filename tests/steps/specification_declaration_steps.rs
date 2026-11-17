//! Step definitions for the `specification-declaration` capability.
//!
//! Every step drives the built binary (`common::run_givn`) and reads the
//! fixture tree; none imports `config::declaration`. The declaration fixture is
//! one valid six-key file that each scenario breaks in exactly one way, so a
//! refusal is attributable to the mutation the scenario names.

use cucumber::{given, then};

use crate::common;
use crate::steps::{GivnWorld, output, stderr};

const DECLARATION_PATH: &str = "givn/specification.yaml";

/// Per-scenario state for the `specification-declaration` scenarios.
#[derive(Debug, Default)]
pub(crate) struct SpecificationDeclarationState {
    /// The declaration bytes the Given recorded after preparing the fixture.
    pub(crate) recorded: Option<String>,
}

/// The declaration every scenario starts from: the six required keys, one
/// project-relative path each.
const DECLARATION_ENTRIES: [(&str, &str); 6] = [
    ("behavior", r#"["givn/specs/**"]"#),
    ("structure", r#"["docs/glossary.md"]"#),
    ("contracts", r#"["docs/reference.md"]"#),
    ("limits", r#"["docs/arc42/02-architecture-constraints.md"]"#),
    ("e2e", r#"["tests/e2e"]"#),
    ("source", r#"["src/**"]"#),
];

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// An initialized project selecting opencode with `ise` effective.
fn ise_enabled_project(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let output = common::run_givn(&env, &["addons", "enable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    world.env = Some(env);
}

fn write_declaration(world: &GivnWorld, declaration: &str) {
    std::fs::write(env(world).path(DECLARATION_PATH), declaration)
        .expect("write the declaration fixture");
}

fn entry_mut<'a>(
    entries: &'a mut Vec<(&'static str, &'static str)>,
    key: &str,
) -> &'a mut (&'static str, &'static str) {
    entries
        .iter_mut()
        .find(|(candidate, _)| *candidate == key)
        .expect("the declaration fixture declares every key")
}

/// The declaration fixture with `mutation` applied. Every phrase comes from the
/// scenario's Given or Examples row; an unmapped phrase fails the step instead
/// of silently writing a valid file.
fn declaration_for(mutation: &str) -> String {
    let mut entries: Vec<(&'static str, &'static str)> = DECLARATION_ENTRIES.to_vec();
    match mutation {
        r#"adds the key "components""# => entries.push(("components", "[]")),
        r#"misspells the role "behavior" as "behaviour""# => {
            let (declared, _) = entry_mut(&mut entries, "behavior");
            *declared = "behaviour";
        }
        r#"repeats the role "e2e""# => entries.push(("e2e", r#"["tests/e2e"]"#)),
        r#"omits the role "limits""# => entries.retain(|(key, _)| *key != "limits"),
        "omits the source list" => entries.retain(|(key, _)| *key != "source"),
        r#"declares the source root "../other/crates/**""# => {
            let (_, value) = entry_mut(&mut entries, "source");
            *value = r#"["../other/crates/**"]"#;
        }
        r#"declares the structure path "/opt/specs/**""# => {
            let (_, value) = entry_mut(&mut entries, "structure");
            *value = r#"["/opt/specs/**"]"#;
        }
        "is not valid YAML" => {
            let (_, value) = entry_mut(&mut entries, "behavior");
            *value = r#"["givn/specs/**"#;
        }
        r#"holds a scalar for the role "behavior""# => {
            let (_, value) = entry_mut(&mut entries, "behavior");
            *value = r#""givn/specs/**""#;
        }
        other => panic!("unhandled declaration mutation '{other}'"),
    }
    let mut declaration = entries
        .iter()
        .map(|(key, value)| format!("{key}: {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    declaration.push('\n');
    declaration
}

// --- Given -----------------------------------------------------------------

#[given(
    regex = r"^an ise-enabled project selecting opencode whose declaration (.+) and whose bytes are recorded$"
)]
fn ise_project_with_recorded_declaration(world: &mut GivnWorld, mutation: String) {
    let declaration = declaration_for(&mutation);
    ise_enabled_project(world);
    write_declaration(world, &declaration);
    world.specification_declaration.recorded = Some(declaration);
}

#[given(
    "an ise-enabled project selecting opencode whose declaration places build inputs under the four content roles"
)]
fn ise_project_with_build_input_declaration(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &crate::steps::ise_clean_room_support::build_input_declaration("\"tests/e2e\""),
    );
    crate::steps::ise_clean_room_support::write_build_inputs(env(world));
    let manifest = env(world).path("tests/e2e/Cargo.toml");
    std::fs::create_dir_all(manifest.parent().expect("the manifest has a parent"))
        .expect("create the End-to-end package directory");
    std::fs::write(&manifest, "[package]\nname = \"e2e\"\n")
        .expect("write the End-to-end package manifest");
}

#[given("an ise-enabled project selecting opencode whose declaration is not valid YAML")]
fn ise_project_with_unparsable_declaration(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_declaration(world, &declaration_for("is not valid YAML"));
}

#[given(
    "an ise-enabled project selecting opencode whose declaration holds a scalar for the role \"behavior\""
)]
fn ise_project_with_scalar_role(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &declaration_for(r#"holds a scalar for the role "behavior""#),
    );
}

#[given("an ise-enabled project selecting opencode whose declaration path is a directory")]
fn ise_project_with_directory_declaration(world: &mut GivnWorld) {
    ise_enabled_project(world);
    // Enabling `ise` scaffolds the declaration, so the path is occupied by a
    // file before the directory replaces it.
    let path = env(world).path(DECLARATION_PATH);
    if path.is_file() {
        std::fs::remove_file(&path).expect("remove the scaffolded declaration");
    }
    std::fs::create_dir(&path).expect("create the directory occupying the declaration path");
}

#[given(
    "an ise-enabled project selecting opencode whose declaration misspells the role \"behavior\" as \"behaviour\""
)]
fn ise_project_with_misspelled_role(world: &mut GivnWorld) {
    ise_enabled_project(world);
    write_declaration(
        world,
        &declaration_for(r#"misspells the role "behavior" as "behaviour""#),
    );
}

#[given("an initialized project selecting opencode with ise disabled and a malformed declaration")]
fn ise_disabled_project_with_malformed_declaration(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
    let output = common::run_givn(env(world), &["addons", "disable", "ise"]);
    assert!(
        output.status.success(),
        "givn addons disable ise failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    write_declaration(world, &declaration_for(r#"adds the key "components""#));
}

// --- Then ------------------------------------------------------------------

#[then("the declaration bytes equal the recorded bytes")]
fn declaration_bytes_equal_recorded(world: &mut GivnWorld) {
    let recorded = world
        .specification_declaration
        .recorded
        .clone()
        .expect("the Given records the declaration bytes first");
    let after = std::fs::read_to_string(env(world).path(DECLARATION_PATH))
        .expect("read the declaration after the lint run");
    assert_eq!(
        after, recorded,
        "the declaration was written by the lint run"
    );
}

#[then(regex = r#"^stderr does not report the omitted role "(.*)"$"#)]
fn stderr_does_not_report_the_omitted_role(world: &mut GivnWorld, role: String) {
    let observed = stderr(world);
    let missing_role = format!("missing role '{role}'");
    assert!(
        !observed.contains(&missing_role),
        "stderr reports the omitted role '{role}' besides the unknown key:\n{observed}"
    );
}

#[then(regex = r#"^the command exits 1 and names "([^"]+)"$"#)]
fn exits_one_and_names(world: &mut GivnWorld, name: String) {
    let out = output(world);
    assert_eq!(out.status.code(), Some(1), "stderr={}", stderr(world));
    let observed = stderr(world);
    assert!(
        observed.contains(&name),
        "stderr does not name {name:?}:\n{observed}"
    );
}

/// A declaration schema refusal is the blocking `[DECL ]` finding: the
/// declaration is absent, unreadable, or not valid YAML.
#[then("stderr contains no declaration-schema error")]
fn stderr_contains_no_declaration_schema_error(world: &mut GivnWorld) {
    let observed = stderr(world);
    assert!(
        !observed.contains("[DECL ]"),
        "stderr carries a declaration refusal:\n{observed}"
    );
}

/// The fixture declares every build input the whole-product rebuild reads, so
/// no advisory may report a declared path as missing, duplicated, or inside
/// the End-to-end boundary.
#[then("stderr contains no declared-path advisory for the build inputs")]
fn stderr_contains_no_declared_path_advisory_for_the_build_inputs(world: &mut GivnWorld) {
    const BUILD_INPUT_SUBJECTS: [&str; 5] = [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "assets/",
        "e2e",
    ];
    let observed = stderr(world);
    for line in observed
        .lines()
        .filter(|line| line.starts_with("[SPEC ] advisory"))
    {
        for subject in BUILD_INPUT_SUBJECTS {
            assert!(
                !line.contains(subject),
                "the declared-path advisory names the build input {subject:?}:\n{line}"
            );
        }
    }
}

#[then("stderr contains no declaration finding")]
fn stderr_contains_no_declaration_finding(world: &mut GivnWorld) {
    let observed = stderr(world);
    assert!(
        !observed.contains(DECLARATION_PATH),
        "stderr reports the declaration although ise is off:\n{observed}"
    );
    assert!(
        !observed.contains("DECL"),
        "stderr carries a declaration finding although ise is off:\n{observed}"
    );
}
