//! Unit-level step definitions for the run-execution capability.
//!
//! These steps drive the executor through the real givn binary in fixture
//! projects and the probe primitives in-process. `process::exit` never runs
//! inside the runner: the SIGTERM scenario calls the library entry point
//! and asserts the returned exit code.

use std::time::{Duration, Instant};

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

#[given("givn run is executing a product that stays ready")]
fn inproc_start_stays_ready(world: &mut GivnWorld) {
    let port = support::reserve_free_port();
    let port_arg = port.to_string();
    let marker = {
        let env = world.env.get_or_insert_with(common::make_init_root);
        env.path("trap-term-marker")
    };
    let fixture = support::ready_fixture_path().to_string_lossy().into_owned();
    let command = format!(
        "{fixture} serve --port {port_arg} --trap-marker {}",
        marker.display()
    );
    support::write_server_declaration(world, &command, None, Some(port), "/healthz", 60);
    let prev_cwd = std::env::current_dir().expect("runner cwd");
    let root = world
        .env
        .as_ref()
        .expect("fixture project root")
        .root()
        .to_path_buf();
    std::env::set_current_dir(&root).expect("chdir fixture root");
    let handle = std::thread::spawn(|| givn::cli::run::execute(None, false));
    world.run_inproc = Some(support::InProcStart {
        port,
        marker,
        prev_cwd,
        handle: Some(handle),
        joined: None,
    });
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
    // The probe asserts "ready on the first attempt", so its thread must
    // not race the responder's bind.
    support::wait_for_listener(port);
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

#[when("givn receives a terminate request while running the product")]
fn inproc_terminate(world: &mut GivnWorld) {
    let port = world
        .run_inproc
        .as_ref()
        .expect("an in-process start is required")
        .port;
    // The ready line goes to the runner's stdout, so readiness is observed
    // at the HTTP boundary instead.
    let outcome = givn::cli::run::probe_until_ready(port, "/healthz", Duration::from_secs(10));
    assert!(
        matches!(outcome, givn::cli::run::ProbeOutcome::Ready { .. }),
        "the product never became ready: {outcome:?}"
    );
    #[cfg(unix)]
    unsafe {
        libc::kill(std::process::id() as i32, libc::SIGTERM);
    }
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

#[then("givn exits 143")]
fn inproc_exits_143(world: &mut GivnWorld) {
    let code = {
        let job = world
            .run_inproc
            .as_mut()
            .expect("an in-process start is required");
        if job.joined.is_none() {
            let handle = job.handle.take().expect("start thread handle");
            job.joined = Some(handle.join().expect("start thread"));
        }
        job.joined.expect("joined exit code")
    };
    assert_eq!(code, 143, "expected 143 (128+SIGTERM)");
}

#[then("the started product received the terminate signal")]
fn product_received_terminate(world: &mut GivnWorld) {
    let marker = world
        .run_inproc
        .as_ref()
        .expect("an in-process start is required")
        .marker
        .clone();
    // The fixture observes the stored signal on its next serve-loop tick,
    // so the marker may lag the forwarded signal by a few milliseconds.
    let deadline = Instant::now() + Duration::from_secs(2);
    let content = loop {
        match std::fs::read_to_string(&marker) {
            Ok(content) => break content,
            Err(_) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
            Err(e) => panic!("trap marker missing at {}: {e}", marker.display()),
        }
    };
    assert_eq!(content, "15", "fixture must report SIGTERM");
}

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
        !lines[0]
            .split_whitespace()
            .next()
            .unwrap_or("")
            .contains('='),
        "print line {:?} unexpectedly exports a port hand-off",
        lines[0],
    );
}

#[then(regex = r#"^stderr mentions "(.*)"$"#)]
fn stderr_mentions(world: &mut GivnWorld, needle: String) {
    let s = super::stderr(world);
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stderr missing {unescaped:?}: {s}");
}

fn survival_marker(world: &mut GivnWorld) -> String {
    let env = world.env.get_or_insert_with(common::make_init_root);
    env.path("stall-survived").to_string_lossy().into_owned()
}

#[given("a server declaration whose readiness path never succeeds within its declared deadline")]
fn deadline_never_ready(world: &mut GivnWorld) {
    let marker = survival_marker(world);
    let command = support::serve_command(
        "/healthz",
        &[
            "--never-ready",
            "--pid-file",
            "product.pid",
            "--survive-marker",
            marker.as_str(),
        ],
    );
    support::write_server_declaration(
        world,
        &format!("exec {command}"),
        Some("PORT"),
        None,
        "/healthz",
        1,
    );
    let env = world.env.as_ref().unwrap();
    std::fs::write(
        env.path("run-started-ms"),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string(),
    )
    .unwrap();
}

#[given("a server declaration with a finite readiness deadline")]
fn finite_readiness_deadline(world: &mut GivnWorld) {
    deadline_never_ready(world);
}

#[given(
    regex = r#"^the readiness endpoint "(accepts a connection but does not read the request|reads the request but never responds|sends a partial response and leaves the connection open)"$"#
)]
fn stalled_readiness_endpoint(world: &mut GivnWorld, stall: String) {
    let mode = match stall.as_str() {
        "accepts a connection but does not read the request" => "write",
        "reads the request but never responds" => "read",
        "sends a partial response and leaves the connection open" => "partial",
        _ => unreachable!(),
    };
    let marker = survival_marker(world);
    let command = support::serve_command(
        "/healthz",
        &[
            "--stall",
            mode,
            "--pid-file",
            "product.pid",
            "--survive-marker",
            marker.as_str(),
        ],
    );
    support::write_server_declaration(
        world,
        &format!("exec {command}"),
        Some("PORT"),
        None,
        "/healthz",
        1,
    );
}

#[then("the command exits 2 within that deadline plus bounded process cleanup time")]
#[then("the command exits 2 within the readiness deadline plus bounded process cleanup time")]
fn bounded_timeout_exit(world: &mut GivnWorld) {
    assert_eq!(super::output(world).status.code(), Some(2));
    let env = world.env.as_ref().unwrap();
    let started: u128 = std::fs::read_to_string(env.path("run-started-ms"))
        .unwrap()
        .parse()
        .unwrap();
    let elapsed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - started;
    assert!(
        !env.path("stall-survived").exists(),
        "the product survived the bounded cleanup (elapsed {elapsed}ms): {}",
        std::fs::read_to_string(env.path("stall-survived")).unwrap_or_default()
    );
    assert!(
        elapsed <= 15_000,
        "the command did not return after the readiness deadline: {elapsed}ms"
    );
}

#[then("no ready line is printed")]
fn no_ready_line(world: &mut GivnWorld) {
    assert!(!super::stdout(world).contains("givn run: ready at"));
}

#[then("the output names the readiness timeout without reporting ready")]
fn named_readiness_timeout(world: &mut GivnWorld) {
    assert!(super::stderr(world).contains("readiness timeout"));
    no_ready_line(world);
}

#[then("the started product is stopped and reaped")]
fn stopped_product(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    let pid_text = std::fs::read_to_string(env.path("product.pid")).unwrap();
    let pid: i32 = pid_text
        .trim()
        .parse()
        .unwrap_or_else(|error| panic!("invalid product.pid {pid_text:?}: {error}"));
    #[cfg(unix)]
    {
        let deadline = Instant::now() + Duration::from_secs(1);
        while unsafe { libc::kill(pid, 0) } == 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_ne!(
            unsafe { libc::kill(pid, 0) },
            0,
            "product {pid} is still alive or unreaped"
        );
    }
}

#[given(
    "a readiness endpoint whose failing responses consume almost all the declared readiness time"
)]
#[given(
    "a readiness endpoint that delays connection, request consumption, and response across repeated attempts"
)]
fn delayed_failing_endpoint(world: &mut GivnWorld) {
    world.env.get_or_insert_with(common::make_init_root);
    let root = world.env.as_ref().unwrap().root();
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port();
    let peer = std::thread::spawn(move || {
        use std::io::{Read, Write};
        let deadline = Instant::now() + Duration::from_millis(600);
        let mut attempts = 0;
        while Instant::now() < deadline {
            if let Ok((mut stream, _)) = listener.accept() {
                attempts += 1;
                stream
                    .set_read_timeout(Some(Duration::from_millis(50)))
                    .unwrap();
                let mut request = [0; 4096];
                let _ = stream.read(&mut request);
                std::thread::sleep(Duration::from_millis(80));
                let _ = stream.write_all(b"HTTP/1.0 503 Unavailable\r\n\r\n");
            } else {
                std::thread::sleep(Duration::from_millis(2));
            }
        }
        attempts
    });
    let handle = std::thread::spawn(move || {
        let started = Instant::now();
        let outcome =
            givn::cli::run::probe_until_ready(port, "/healthz", Duration::from_millis(220));
        std::fs::write(
            root.join("probe-elapsed-ms"),
            started.elapsed().as_millis().to_string(),
        )
        .unwrap();
        std::fs::write(
            root.join("probe-connections"),
            peer.join().unwrap().to_string(),
        )
        .unwrap();
        outcome
    });
    world.run_probe = Some(super::ProbeJob { port, handle });
}

#[when("the readiness deadline elapses")]
#[when("the consumer starts the product with a declared readiness timeout")]
fn wait_for_shared_deadline(world: &mut GivnWorld) {
    let job = world.run_probe.take().unwrap();
    assert_eq!(
        job.handle.join().unwrap(),
        givn::cli::run::ProbeOutcome::TimedOut
    );
}

#[then("no additional connection attempt receives a fresh full timeout")]
fn no_fresh_probe_timeout(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    let attempts: u32 = std::fs::read_to_string(env.path("probe-connections"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(attempts, 2);
    shared_deadline_elapsed(world);
}

#[then("the probe reports a timeout within the original total deadline")]
#[then(
    "the combined connection, write, read, and retry waits consume no more than the total readiness deadline"
)]
fn shared_deadline_elapsed(world: &mut GivnWorld) {
    let env = world.env.as_ref().unwrap();
    let elapsed: u128 = std::fs::read_to_string(env.path("probe-elapsed-ms"))
        .unwrap()
        .parse()
        .unwrap();
    assert!(elapsed <= 290, "220ms deadline exceeded: {elapsed}ms");
}

// --- prepare-0-9-10: total-deadline readiness -------------------------------

#[given(r#"the readiness endpoint "does not complete a connection""#)]
fn readiness_endpoint_does_not_connect(world: &mut GivnWorld) {
    let marker = survival_marker(world);
    let command = format!("sh -c 'echo $$ > product.pid; (sleep 1.5; touch {marker}) & sleep 10'");
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 1);
}

#[then(
    "an expired deadline triggers the existing product cleanup rather than another full probe wait"
)]
fn expired_deadline_uses_existing_cleanup(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("fixture missing");
    let attempts: u32 = std::fs::read_to_string(env.path("probe-connections"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(
        attempts, 2,
        "an expired deadline started another full probe attempt"
    );
    let elapsed: u128 = std::fs::read_to_string(env.path("probe-elapsed-ms"))
        .unwrap()
        .parse()
        .unwrap();
    assert!(
        elapsed <= 290,
        "cleanup started another full probe wait: {elapsed}ms"
    );
}

fn git_fixture(env: &common::TestEnv, args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .current_dir(env.root())
        .output()
        .expect("run git fixture command")
}

fn git_fixture_stdout(env: &common::TestEnv, args: &[&str]) -> String {
    let output = git_fixture(env, args);
    assert!(
        output.status.success(),
        "git command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[when("the user runs `givn upgrade` again")]
fn upgrade_again(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    world.run_prev_commit = Some(git_fixture_stdout(env, &["rev-parse", "HEAD"]));
    world.last_output = Some(common::run_givn(env, &["upgrade"]));
}

#[then("the working tree is clean")]
fn working_tree_is_clean(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let status = git_fixture_stdout(env, &["status", "--porcelain"]);
    assert!(
        status.trim().is_empty(),
        "the working tree must be clean after the upgrade commit:\n{status}"
    );
}

#[then(regex = r#"^the latest revision contains "(.*)"$"#)]
fn latest_revision_contains(world: &mut GivnWorld, path: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let names = match env.backend() {
        common::BackendKind::Git => {
            git_fixture_stdout(env, &["show", "--name-only", "--format=", "HEAD"])
        }
        common::BackendKind::Fossil => {
            let out = common::fossil(env, &["diff", "--brief", "--checkin", "current"]);
            String::from_utf8_lossy(&out.stdout).into_owned()
        }
    };
    assert!(
        names.lines().any(|line| line.contains(&path)),
        "the latest revision must touch {path}; it touches:\n{names}"
    );
}

#[then("the latest revision is unchanged by the second upgrade")]
fn latest_revision_unchanged(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let before = world
        .run_prev_commit
        .as_deref()
        .expect("the pre-second-upgrade commit hash is required");
    let after = git_fixture_stdout(env, &["rev-parse", "HEAD"]);
    assert_eq!(
        before.trim(),
        after.trim(),
        "the second upgrade created an additional commit"
    );
}
