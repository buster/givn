//! Unit-level step definitions for the run-execution capability.
//!
//! These steps drive the executor through the real givn binary in fixture
//! projects and the probe primitives in-process. `process::exit` never runs
//! inside the runner: the SIGTERM scenario calls the library entry point
//! and asserts the returned exit code.

use std::time::Duration;

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;
use crate::steps::run_execution_support as support;

// --- Given: declarations -----------------------------------------------------

#[given("a project with a server run declaration with a fixed port")]
fn server_declaration_fixed_port(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    let port_arg = port.to_string();
    let command = support::serve_command("/healthz", &["--port", &port_arg, "--exit-after", "1"]);
    support::write_server_declaration(world, &command, None, Some(port), "/healthz", 60);
}

#[given("a project with a server run declaration whose readiness path never succeeds")]
fn server_declaration_never_ready(world: &mut GivnWorld) {
    let command = support::serve_command("/healthz", &["--never-ready"]);
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 1);
}

#[given("a project with a server run declaration whose product exits 3 before readiness")]
fn server_declaration_early_exit(world: &mut GivnWorld) {
    let fixture = support::ready_fixture_path().to_string_lossy().into_owned();
    let command = format!("{fixture} fail --code 3");
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 5);
}

#[given("a project with a CLI run declaration that exits 7")]
fn cli_declaration_exit_7(world: &mut GivnWorld) {
    let fixture = support::ready_fixture_path().to_string_lossy().into_owned();
    let command = format!("{fixture} fail --code 7");
    let declaration = format!("run:\n  command: \"{command}\"\n  product_type: cli\n");
    let env = world.env.get_or_insert_with(common::make_init_root);
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = crate::common::replace_top_level_key(&content, "run", &declaration);
    std::fs::write(&path, updated).expect("write commands.yaml");
    world.run_declared_command = Some(command);
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

// --- Given: in-process probe mechanics ---------------------------------------

#[given("a readiness probe against a port with no listener yet")]
fn probe_against_silent_port(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    let handle = std::thread::spawn(move || {
        givn::cli::run::probe_until_ready(port, "/healthz", Duration::from_secs(5))
    });
    world.run_probe = Some(super::ProbeJob { port, handle });
}

#[given("a readiness probe against a listener that answers failure first")]
fn probe_against_failing_listener(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    world.run_responder = Some(support::spawn_responder(port, 404));
    let handle = std::thread::spawn(move || {
        givn::cli::run::probe_until_ready(port, "/healthz", Duration::from_secs(5))
    });
    world.run_probe = Some(super::ProbeJob { port, handle });
}

#[given("a readiness probe against a listener that answers 204")]
fn probe_against_204_listener(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    world.run_responder = Some(support::spawn_responder(port, 204));
    let handle = std::thread::spawn(move || {
        givn::cli::run::probe_until_ready(port, "/healthz", Duration::from_secs(5))
    });
    world.run_probe = Some(super::ProbeJob { port, handle });
}

#[given("a readiness probe against a listener that never answers success")]
fn probe_against_never_ready_listener(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    world.run_responder = Some(support::spawn_responder(port, 404));
    let handle = std::thread::spawn(move || {
        givn::cli::run::probe_until_ready(port, "/healthz", Duration::from_millis(300))
    });
    world.run_probe = Some(super::ProbeJob { port, handle });
}

// --- When ---------------------------------------------------------------------

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

#[when("the listener starts answering success after a short delay")]
fn listener_starts_answering(world: &mut GivnWorld) {
    let port = world.run_probe.as_ref().expect("a probe is required").port;
    std::thread::sleep(Duration::from_millis(300));
    world.run_responder = Some(support::spawn_responder(port, 200));
}

#[when("the listener switches to answering success")]
fn listener_switches_to_success(world: &mut GivnWorld) {
    let responder = world
        .run_responder
        .as_ref()
        .expect("a responder is required");
    // Ensure the probe has actually seen the failing status before the
    // flip, so "more than one attempt" is guaranteed, not racy.
    responder.wait_for_connections(1, Duration::from_secs(5));
    responder.set_status(200);
}

#[when("the readiness timeout elapses")]
fn readiness_timeout_elapses(_world: &mut GivnWorld) {
    // The probe's own deadline governs; joining happens in the Then step.
}

// --- Then: probe outcomes ------------------------------------------------------

#[then("the probe reports ready after more than one attempt")]
fn probe_ready_after_retries(world: &mut GivnWorld) {
    let job = world.run_probe.take().expect("a probe is required");
    let outcome = job.handle.join().expect("probe thread");
    assert!(
        matches!(
            outcome,
            givn::cli::run::ProbeOutcome::Ready { attempts } if attempts > 1
        ),
        "expected ready after more than one attempt: {outcome:?}"
    );
}

#[then("the probe reports ready on the first attempt")]
fn probe_ready_first_attempt(world: &mut GivnWorld) {
    let job = world.run_probe.take().expect("a probe is required");
    let outcome = job.handle.join().expect("probe thread");
    assert_eq!(
        outcome,
        givn::cli::run::ProbeOutcome::Ready { attempts: 1 },
        "expected ready on the first attempt: {outcome:?}"
    );
}

#[then("the probe reports a timeout")]
fn probe_reports_timeout(world: &mut GivnWorld) {
    let job = world.run_probe.take().expect("a probe is required");
    let outcome = job.handle.join().expect("probe thread");
    assert_eq!(outcome, givn::cli::run::ProbeOutcome::TimedOut);
}

// --- Then: ready line ------------------------------------------------------------

fn ready_line(stdout: &str) -> &str {
    stdout
        .lines()
        .find(|l| l.starts_with("givn run: ready at http://127.0.0.1:"))
        .unwrap_or_else(|| panic!("no ready line in stdout: {stdout:?}"))
}

#[then("the ready line names the fixed declared port")]
fn ready_line_names_fixed_port(world: &mut GivnWorld) {
    let port = world
        .run_fixed_port
        .expect("a fixed declared port is required");
    let stdout = super::stdout(world);
    let line = ready_line(&stdout);
    assert!(
        line.contains(&format!(":{port}/")),
        "ready line {line:?} does not name the fixed port {port}"
    );
}

#[then(regex = r#"^the ready line names the readiness path "(.*)"$"#)]
fn ready_line_names_readiness_path(world: &mut GivnWorld, path: String) {
    let stdout = super::stdout(world);
    let line = ready_line(&stdout);
    assert!(
        line.ends_with(&path),
        "ready line {line:?} does not name the readiness path {path:?}"
    );
}

// --- Then: stderr ------------------------------------------------------------------

#[then("the command exits 7")]
fn command_exits_seven(world: &mut GivnWorld) {
    let out = super::output(world);
    assert_eq!(
        out.status.code(),
        Some(7),
        "stderr={}",
        super::stderr(world)
    );
}

#[then(regex = r#"^stderr mentions "(.*)"$"#)]
fn stderr_mentions(world: &mut GivnWorld, needle: String) {
    let s = super::stderr(world);
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stderr missing {unescaped:?}: {s}");
}
