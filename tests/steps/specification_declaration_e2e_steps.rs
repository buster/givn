//! E2E step definitions for the `specification-declaration` capability.
//!
//! The scenario drives the real `givn lint` against a project-owned
//! declaration and reads its recorded output; it never imports
//! `config::declaration`.

use cucumber::{given, then};

use crate::common;
use crate::steps::{GivnWorld, stderr};

const DECLARATION_PATH: &str = "givn/specification.yaml";

/// The declaration the project owns: the five roles plus two replaceable
/// source roots. Portability is the only boundary under test here, so a path
/// that does not exist yet stays an advisory.
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

/// A schema error is the blocking declaration finding, rendered with the
/// `DECL ` level prefix. Advisory placement findings use `SPEC ` and stay
/// allowed.
#[then("stderr contains no declaration-schema error")]
fn stderr_contains_no_declaration_schema_error(world: &mut GivnWorld) {
    let observed = stderr(world);
    assert!(
        !observed.lines().any(|line| line.starts_with("DECL ")),
        "stderr reports a declaration-schema error:\n{observed}"
    );
}
