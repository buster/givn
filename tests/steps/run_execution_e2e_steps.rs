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
use crate::common;
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

#[given("verify commands are no-ops with a complete server run declaration")]
fn verify_noops_with_server_run(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    let yaml = "\
verify:
  command: \"./emit-result.sh regular\"
  e2e_command: \"./emit-result.sh e2e\"
coverage:
  measure_command: \"true\"
  merge_command: \"true\"
  non_e2e_path: \"coverage/non-e2e-cobertura.xml\"
  e2e_path: \"coverage/e2e-cobertura.xml\"
  output_path: \"coverage/cobertura-coverage.xml\"
";
    std::fs::write(env.path("givn/commands.yaml"), yaml).expect("write commands.yaml");
    crate::steps::run_declaration_fixtures_support::declare_run(world, "server");
}

#[then(regex = r#"^the archived change directory "([^"]+)" exists$"#)]
fn archived_change_dir_named_exists(world: &mut GivnWorld, id: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    assert!(
        env.path(&format!("givn/archive/{id}")).is_dir(),
        "the change must have been moved to givn/archive/{id}/"
    );
}

#[given("a git-managed project with givn initialised")]
fn git_managed_project(world: &mut GivnWorld) {
    let env = common::make_bare_root();
    let init = common::run_givn(&env, &["init"]);
    assert!(
        init.status.success(),
        "fixture init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    // The committed baseline predates the run declaration: the upgrade
    // scenarios strip it themselves, and `givn upgrade` demands a clean
    // worktree, so the strip must already be part of the baseline commit.
    let commands = std::fs::read_to_string(env.path("givn/commands.yaml"))
        .expect("read fixture commands.yaml");
    let stripped = common::remove_top_level_key(&commands, "run");
    std::fs::write(env.path("givn/commands.yaml"), stripped).expect("write commands.yaml");
    std::fs::write(env.path(".gitignore"), ".opencode/\nAGENTS.md\n")
        .expect("ignore generated agent files");
    git_fixture(&env, &["init"]);
    git_fixture(&env, &["config", "user.name", "Givn Run Test"]);
    git_fixture(&env, &["config", "user.email", "givn-run@example.test"]);
    git_fixture(&env, &["add", "--all"]);
    let commit = git_fixture(&env, &["commit", "-m", "fixture: initialised project"]);
    assert!(
        commit.status.success(),
        "fixture commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );
    world.env = Some(env);
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

#[then(regex = r#"^the file "([^"]+)" contains "(.*)"$"#)]
fn file_contains(world: &mut GivnWorld, rel: String, needle: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content =
        std::fs::read_to_string(env.path(&rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"));
    assert!(
        content.contains(&needle),
        "{rel} lacks {needle:?}:\n{content}"
    );
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

#[then(regex = r#"^the latest commit contains "(.*)"$"#)]
fn latest_commit_contains(world: &mut GivnWorld, path: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let names = git_fixture_stdout(env, &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        names.lines().any(|line| line.trim() == path),
        "latest commit must touch {path}; it touches:\n{names}"
    );
}

#[then("the latest commit is unchanged by the second upgrade")]
fn latest_commit_unchanged(world: &mut GivnWorld) {
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

#[given("a README with a broken run managed block")]
fn broken_run_block_readme(world: &mut GivnWorld) {
    // Insert a run managed block that lost its end marker: exactly one of
    // the two markers present is the damaged state.
    let text = crate::steps::readme_enforcement_steps::scaffolded_readme_text().replace(
        "## Quick start\n",
        "## Quick start\n\n<!-- givn:begin:run -->\nRun instructions.\n",
    );
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    std::fs::write(env.path("README.md"), text).expect("write README.md");
}

#[then("the commands file has run.command set to the sentinel")]
fn commands_file_has_sentinel(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    assert!(
        content.contains(givn::config::manifest::Run::SENTINEL),
        "commands file must carry the run.command sentinel:\n{content}"
    );
}

#[then("the README has the run managed block")]
fn readme_has_run_block(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        readme.contains("<!-- givn:begin:run -->") && readme.contains("<!-- givn:end:run -->"),
        "README must contain the run managed block:\n{readme}"
    );
}

#[then("the README has the run open placeholder")]
fn readme_has_run_placeholder(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme = std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        readme.contains(
            "<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->"
        ),
        "README must carry the run open placeholder:\n{readme}"
    );
}
