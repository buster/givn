//! E2E step definitions for the `specification-declaration` capability.
//!
//! The scenario drives the real `givn lint` against a project-owned
//! declaration and reads its recorded output; it never imports
//! `config::declaration`.

use crate::harness as common;
use crate::steps::{GivnWorld, stderr};
use cucumber::{given, then};

const DECLARATION_PATH: &str = "givn/specification.yaml";

const DECLARED_ROLES: &str = "\
behavior: [\"givn/specs/**\"]
structure: [\"docs/glossary.md\"]
contracts: [\"docs/reference.md\"]
limits: [\"docs/arc42/02-architecture-constraints.md\"]
e2e: [\"tests/e2e\"]
source: [\"src/**\", \"tests/**\"]
";

#[given(
    "an ise-enabled project selecting opencode with a declaration naming the five roles and two source roots"
)]
fn ise_enabled_project_with_declared_roles(world: &mut GivnWorld) {
    let env = common::make_init_root();
    let enable = common::run_givn(&env, &["addons", "enable", "ise"]);
    assert!(
        enable.status.success(),
        "givn addons enable ise failed: {}",
        String::from_utf8_lossy(&enable.stderr)
    );
    std::fs::write(env.path(DECLARATION_PATH), DECLARED_ROLES)
        .expect("write the declaration fixture");
    world.env = Some(env);
}

#[then("stderr contains no declaration-schema error")]
fn stderr_contains_no_declaration_schema_error(world: &mut GivnWorld) {
    let observed = stderr(world);
    assert!(
        !observed.lines().any(|line| line.starts_with("DECL ")),
        "stderr reports a declaration-schema error:\n{observed}"
    );
}

#[then("stderr contains the declared-path advisory for the contracts role")]
fn stderr_contains_the_contracts_role_advisory(world: &mut GivnWorld) {
    let observed = stderr(world);
    let expected = "the 'contracts' role declares 'docs/reference.md', which matches no path in \
                    the project";
    assert!(
        observed
            .lines()
            .any(|line| line.starts_with("[SPEC ] advisory") && line.ends_with(expected)),
        "stderr reports no contracts-role advisory:\n{observed}"
    );
}
