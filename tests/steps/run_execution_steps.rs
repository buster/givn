//! Unit-level step definitions for the run-execution capability.
//!
//! These steps drive the executor through the real givn binary in fixture
//! projects and the probe primitives in-process. `process::exit` never runs
//! inside the runner: the SIGTERM scenario calls the library entry point
//! and asserts the returned exit code.

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;
use crate::steps::run_execution_support as support;

// --- Given: declarations -----------------------------------------------------

#[given("a project with a server run declaration with a fixed port")]
fn server_declaration_fixed_port(world: &mut GivnWorld) {
    support::write_fixed_port_server_declaration(world, "./serve-fixture.sh");
}

#[given("a project with a library run declaration")]
fn library_declaration(world: &mut GivnWorld) {
    world.env.get_or_insert_with(common::make_init_root);
    super::run_declaration_fixtures_support::declare_run(world, "library");
}

#[given("a project with a library run declaration with a real start command")]
fn library_declaration_real_command(world: &mut GivnWorld) {
    support::write_library_declaration(world, "cargo build --lib");
}

#[given("a project with an unconfigured run declaration")]
fn unconfigured_declaration(world: &mut GivnWorld) {
    world.env.get_or_insert_with(common::make_init_root);
    super::run_declaration_fixtures_support::strip_run(world);
}

// --- When: actions -----------------------------------------------------------

#[when("the user runs givn run with a caller-provided port")]
fn run_with_caller_port(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    world.run_caller_port = Some(port);
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let port_arg = port.to_string();
    world.last_output = Some(common::run_givn(env, &["run", "--port", &port_arg]));
}

#[then(regex = r#"^stderr mentions "(.*)"$"#)]
fn stderr_mentions(world: &mut GivnWorld, needle: String) {
    let s = super::stderr(world);
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stderr missing {unescaped:?}: {s}");
}

// --- Then: print-mode contract ------------------------------------------------

#[then(
    regex = r#"^stdout is one line that exports the port hand-off and names the start command$"#
)]
fn stdout_print_line_with_port(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "expected exactly one stdout line: {stdout:?}"
    );
    let command = world
        .run_declared_command
        .as_deref()
        .expect("a declared start command is required");
    assert!(
        lines[0].contains(command),
        "print line {:?} does not name the start command {command:?}",
        lines[0],
    );
    assert!(
        lines[0].contains("PORT="),
        "print line {:?} does not export the port hand-off",
        lines[0],
    );
}

#[then(regex = r#"^stdout is one line that names the start command without exporting a port$"#)]
fn stdout_print_line_without_port(world: &mut GivnWorld) {
    let stdout = super::stdout(world);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "expected exactly one stdout line: {stdout:?}"
    );
    let command = world
        .run_declared_command
        .as_deref()
        .expect("a declared start command is required");
    assert!(
        lines[0].contains(command),
        "print line {:?} does not name the start command {command:?}",
        lines[0],
    );
    assert!(
        !lines[0].contains('=')
            || !lines[0]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .contains('='),
        "print line {:?} unexpectedly exports a port hand-off",
        lines[0],
    );
}
