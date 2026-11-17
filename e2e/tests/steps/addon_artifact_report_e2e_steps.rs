//! `@e2e` steps for the `givn addons list` materialization report (ADR-080).
//!
//! The real interface is the built binary's stdout and exit status. The
//! component tier binds the same phrases in `tests/steps/addon_artifact_report_steps.rs`
//! against the library-driven fixture, so the two trees stay comparable.

use crate::harness as common;
use crate::steps::{GivnWorld, stdout};
use cucumber::{given, then, when};

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// An initialized project with real scaffolding: `givn init` runs every
/// enabled addon's materialization, so `docs/arc42/`, `docs/glossary.md`,
/// `README.md`, and the selected skills all exist.
#[given("an initialized project with scaffolding materialized")]
fn initialized_with_scaffolding(world: &mut GivnWorld) {
    world.env = Some(common::make_init_root());
}

#[when(regex = r"^the (user|consumer) lists the addons$")]
fn lists_addons(world: &mut GivnWorld, _who: String) {
    let out = common::run_givn(env(world), &["addons", "list"]);
    world.last_output = Some(out);
}

#[then(
    regex = r"^the listing reports (\S+) as (enabled|disabled) with artifacts (present|missing|partial|none)(?: (.*))?$"
)]
fn listing_reports(
    world: &mut GivnWorld,
    addon: String,
    state: String,
    artifacts: String,
    detail: String,
) {
    let text = stdout(world);
    let block = addon_block(&text, &addon);
    let state_label = if state == "enabled" {
        "[enabled ]"
    } else {
        "[disabled]"
    };
    assert!(
        block.starts_with(&format!("  {state_label}  {addon}")),
        "listing does not report {addon} as {state}:\n{text}"
    );
    let line = block
        .lines()
        .find(|line| line.trim_start().starts_with("artifacts:"))
        .unwrap_or_else(|| panic!("no artifacts line for {addon}:\n{block}"));
    let observed = line.trim_start().trim_start_matches("artifacts:").trim();
    let expected = if detail.is_empty() {
        artifacts.to_string()
    } else {
        format!("{artifacts} {}", detail.trim())
    };
    assert_eq!(
        observed, expected,
        "unexpected artifacts line for {addon}:\n{block}"
    );
}

/// The report lines belonging to one addon: from its header up to the line
/// before the next listing header or the trailing advice.
fn addon_block<'a>(text: &'a str, addon: &str) -> &'a str {
    let header = format!("]  {addon}");
    let start = text
        .lines()
        .position(|line| {
            line.trim_end() == format!("  {header}") || line.trim_end().ends_with(&header)
        })
        .unwrap_or_else(|| panic!("no listing block for addon {addon}:\n{text}"));
    let lines: Vec<&str> = text.lines().collect();
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| line.starts_with("  [") || line.starts_with("Tip: ") || line.is_empty())
        .map(|(index, _)| index)
        .unwrap_or(lines.len());
    let from = lines[start].as_ptr() as usize - text.as_ptr() as usize;
    let to = if end < lines.len() {
        lines[end].as_ptr() as usize - text.as_ptr() as usize
    } else {
        text.len()
    };
    &text[from..to]
}
