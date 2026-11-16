use super::GivnWorld;
use crate::harness as common;
use cucumber::{then, when};

#[when("the consumer runs the migration coverage check")]
fn consumer_runs_migration_coverage_check(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("migration fixture project");
    world.last_output = Some(common::run_givn(
        env,
        &["check", "migration", "--change", "migration-test"],
    ));
}

fn write_transcript(world: &GivnWorld, title: &str, argv: &str) {
    let output = world
        .last_output
        .as_ref()
        .expect("the scenario must run the real CLI first");
    let dir = common::terminal_evidence::transcript_dir(title);
    std::fs::create_dir_all(&dir).expect("create transcript directory");
    let cwd = world
        .env
        .as_ref()
        .map(|env| env.root().display().to_string())
        .unwrap_or_else(|| "<unknown>".to_string());
    let text = format!(
        "Scenario: {title}\nargv: {argv}\ncwd (fixture project root): {cwd}\nexit code: {}\n--- stdout ---\n{}\n--- stderr ---\n{}\n",
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    std::fs::write(dir.join("transcript.txt"), text).expect("write transcript");
}

#[then("the terminal transcript is captured for this scenario")]
fn capture_review_transcript(world: &mut GivnWorld) {
    let title = world
        .scenario_title
        .clone()
        .unwrap_or_else(|| "unnamed scenario".to_string());
    let argv = world
        .last_command
        .clone()
        .map(|command| format!("givn {command}"))
        .unwrap_or_else(|| "givn (scenario invocation)".to_string());
    write_transcript(world, &title, &argv);
}
