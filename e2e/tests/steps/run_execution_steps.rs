//! Unit-level step definitions for the run-execution capability.
//!
//! These steps drive the executor through the real givn binary in fixture
//! projects and the probe primitives in-process. `process::exit` never runs
//! inside the runner: the SIGTERM scenario calls the library entry point
//! and asserts the returned exit code.

use super::GivnWorld;
use crate::harness as common;
use crate::steps::run_execution_support as support;
use cucumber::{then, when};

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

fn ready_line(stdout: &str) -> &str {
    stdout
        .lines()
        .find(|l| l.starts_with("givn run: ready at http://127.0.0.1:"))
        .unwrap_or_else(|| panic!("no ready line in stdout: {stdout:?}"))
}

#[then("the ready line names the caller-provided port")]
fn ready_line_names_caller_port(world: &mut GivnWorld) {
    let port = world
        .run_caller_port
        .expect("a caller-provided port is required");
    let stdout = super::stdout(world);
    let line = ready_line(&stdout);
    assert!(
        line.contains(&format!(":{port}/")),
        "ready line {line:?} does not name the caller-provided port {port}"
    );
}

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

#[then(regex = r#"^stderr mentions "(.*)"$"#)]
fn stderr_mentions(world: &mut GivnWorld, needle: String) {
    let s = super::stderr(world);
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stderr missing {unescaped:?}: {s}");
}
