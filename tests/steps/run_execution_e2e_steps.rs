//! @e2e step definitions for the run-execution capability.
//!
//! Real subprocess invocation of the givn binary in fixture projects; the
//! declared products are the in-tree ready fixture.

use std::io::BufRead;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::steps::run_execution_support as support;

#[given("a project with a server run declaration that becomes ready and exits cleanly")]
fn server_becomes_ready_exits_cleanly(world: &mut GivnWorld) {
    let command = support::serve_command("/healthz", &["--exit-after", "1"]);
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 60);
}

#[given("a project with a server run declaration that stays ready")]
fn server_stays_ready(world: &mut GivnWorld) {
    let marker = {
        let env = world.env.get_or_insert_with(crate::common::make_init_root);
        env.path("trap-int-marker")
    };
    let fixture = support::ready_fixture_path().to_string_lossy().into_owned();
    let command = format!(
        "{} serve --ready-path /healthz --trap-marker {}",
        fixture,
        marker.display()
    );
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 60);
    world.run_trap_marker = Some(marker);
}

#[when("the user interrupts givn run after the ready line")]
fn interrupt_after_ready_line(world: &mut GivnWorld) {
    let root = world
        .env
        .as_ref()
        .expect("a project root is required first")
        .root()
        .to_path_buf();
    let mut child = Command::new(env!("CARGO_BIN_EXE_givn"))
        .arg("run")
        .current_dir(&root)
        .env_remove("GIVN_FEATURES")
        .env_remove("GIVN_SPEC_MAX_RERANK_PAIRS")
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn givn run");
    let pid = child.id() as i32;

    // Watchdog: if the ready line never arrives within 30 s, kill the
    // subprocess so this step can fail instead of blocking forever.
    let running = Arc::new(AtomicBool::new(true));
    let watchdog_flag = running.clone();
    let watchdog = std::thread::spawn(move || {
        for _ in 0..300 {
            if !watchdog_flag.load(Ordering::SeqCst) {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        }
    });

    let stdout = child.stdout.take().expect("piped stdout");
    let mut reader = std::io::BufReader::new(stdout);
    let mut ready_seen = false;
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) if line.starts_with("givn run: ready at ") => {
                ready_seen = true;
                break;
            }
            Ok(_) => continue,
            Err(_) => break,
        }
    }
    assert!(ready_seen, "the ready line never appeared on stdout");

    unsafe {
        libc::kill(pid, libc::SIGINT);
    }
    let out = child.wait_with_output().expect("wait givn run");
    running.store(false, Ordering::SeqCst);
    watchdog.join().expect("watchdog thread");
    world.last_output = Some(out);
}

#[then("the command exits 130")]
fn command_exits_130(world: &mut GivnWorld) {
    let out = super::output(world);
    assert_eq!(
        out.status.code(),
        Some(130),
        "stderr={}",
        super::stderr(world)
    );
}

#[then("the started product received the interrupt")]
fn product_received_interrupt(world: &mut GivnWorld) {
    let marker = world
        .run_trap_marker
        .clone()
        .expect("a trap marker is required");
    // The fixture observes the stored signal on its next serve-loop tick,
    // so the marker may lag the forwarded signal by a few milliseconds.
    let deadline = Instant::now() + Duration::from_secs(2);
    let content = loop {
        match std::fs::read_to_string(&marker) {
            Ok(content) => break content,
            Err(_) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(e) => panic!("trap marker missing at {}: {e}", marker.display()),
        }
    };
    assert_eq!(content, "2", "fixture must report SIGINT");
}

#[given("a project with a server run declaration with a port hand-off")]
fn server_declaration_with_port_handoff(world: &mut GivnWorld) {
    let command = support::serve_command("/healthz", &[]);
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 60);
}
