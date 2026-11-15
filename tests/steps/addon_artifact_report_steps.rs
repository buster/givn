//! Steps for the `givn addons list` materialization report (ADR-076).
//!
//! The capability is `enable-optional-addons` under `initialize-project`. The
//! real interface is a subprocess `givn addons list`; every Then step asserts
//! that subprocess's stdout or exit status.

use crate::common;
use crate::steps::{GivnWorld, output, stdout};
use cucumber::{given, then, when};

use crate::steps::template_conditional_rendering_support::set_addon;

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

#[given(regex = r#"^the managed communication block in "([^"]+)" is damaged$"#)]
fn communication_block_damaged(world: &mut GivnWorld, file: String) {
    let path = env(world).path(&file);
    let content = std::fs::read_to_string(&path).expect("read the file holding the block");
    let damaged = content.replace("<!-- givn:end:communication-policy -->", "");
    assert_ne!(
        damaged, content,
        "the fixture had no communication block to damage"
    );
    std::fs::write(&path, damaged).expect("write the damaged file");
}

#[when(regex = r"^the (user|consumer) lists the addons$")]
fn lists_addons(world: &mut GivnWorld, _who: String) {
    let out = common::run_givn(env(world), &["addons", "list"]);
    world.set_last_output(out);
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

#[then(regex = r"^the listing suggests `([^`]+)`$")]
fn listing_suggests(world: &mut GivnWorld, command: String) {
    let text = stdout(world);
    let suggestions: Vec<&str> = text
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("suggestion: "))
        .collect();
    assert!(
        suggestions
            .iter()
            .any(
                |suggestion| suggestion.strip_prefix(&command).is_some_and(|rest| {
                    rest.is_empty() || rest.starts_with("  (refused while change")
                })
            ),
        "no suggestion `{command}` in:\n{text}"
    );
}

#[then(regex = r"^the listing suggests for (\S+) `([^`]+)`$")]
fn listing_suggests_for(world: &mut GivnWorld, addon: String, command: String) {
    let text = stdout(world);
    for block in text.split("\n\n") {
        if block.contains(&format!("]  {addon}\n"))
            || block.starts_with(&format!("  [enabled ]  {addon}"))
        {
            assert!(
                block.contains(&format!("suggestion: {command}")),
                "the {addon} block does not suggest `{command}`:\n{block}"
            );
            return;
        }
    }
    panic!("no block for addon {addon} in:\n{text}");
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

#[then(regex = r#"^the suggestion names "([^"]+)" as the change holding the add-on lock$"#)]
fn suggestion_names_lock(world: &mut GivnWorld, change: String) {
    let text = stdout(world);
    assert!(
        text.contains(&format!(
            "(refused while change {change} holds the add-on lock)"
        )),
        "no lock suffix naming {change} in:\n{text}"
    );
}

#[given(regex = r"^the user runs skills sync$")]
fn user_runs_skills_sync(world: &mut GivnWorld) {
    let out = common::run_givn(env(world), &["skills", "sync"]);
    world.set_last_output(out);
    assert!(
        output(world).status.success(),
        "givn skills sync failed: {}",
        String::from_utf8_lossy(&output(world).stderr)
    );
}

#[then(regex = r#"^stderr names "([^"]+)" as retired$"#)]
fn stderr_names_retired(world: &mut GivnWorld, key: String) {
    let text = crate::steps::stderr(world);
    assert!(
        text.contains(&format!("retired addon setting 'addons.{key}'")),
        "stderr does not name {key} as retired:\n{text}"
    );
}

#[then(regex = r#"^stderr names "([^"]+)" as the replacement$"#)]
fn stderr_names_replacement(world: &mut GivnWorld, replacement: String) {
    let text = crate::steps::stderr(world);
    assert!(
        text.contains(&format!("Rename it manually to 'addons.{replacement}'")),
        "stderr does not name {replacement} as the replacement:\n{text}"
    );
}

#[then("the listing reports no missing skill for a selected target")]
fn no_missing_skill(world: &mut GivnWorld) {
    let text = stdout(world);
    assert!(
        !text.contains("artifacts: partial"),
        "the listing reports a partial addon:\n{text}"
    );
    let _ = world;
}

#[then("the project files are unchanged")]
fn project_files_unchanged(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(1),
        "the fixture must be a broken listing to prove nothing was repaired"
    );
    let arc42 = env(world).path("docs/arc42");
    assert!(!arc42.exists(), "the listing re-materialized docs/arc42");
    let config = std::fs::read_to_string(env(world).path("givn/config.yaml"))
        .expect("read givn/config.yaml");
    assert!(
        !config.contains("arc42: false"),
        "the listing changed the stored addon setting:\n{config}"
    );
}

#[given(regex = r#"^the directory "([^"]+)" no longer exists$"#)]
fn directory_removed(world: &mut GivnWorld, rel: String) {
    let path = env(world).path(&rel);
    if path.exists() {
        std::fs::remove_dir_all(&path).expect("remove fixture directory");
    }
}

#[given(regex = r#"^the skill "([^"]+)" is removed from the "([^"]+)" target only$"#)]
fn skill_removed_from_target(world: &mut GivnWorld, skill: String, target: String) {
    let path = env(world).path(&format!(".{target}/skills/{skill}"));
    assert!(
        path.exists(),
        "the fixture lacks the skill to remove: {}",
        path.display()
    );
    std::fs::remove_dir_all(&path).expect("remove fixture skill directory");
}

#[given(regex = r#"^the skill "([^"]+)" is removed from every target$"#)]
fn skill_removed(world: &mut GivnWorld, skill: String) {
    for target in [".opencode", ".agents", ".claude", ".omp", ".maki"] {
        let path = env(world).path(&format!("{target}/skills/{skill}"));
        if path.exists() {
            std::fs::remove_dir_all(&path).expect("remove fixture skill directory");
        }
    }
}

#[given(regex = r"^the readme addon is disabled$")]
fn readme_disabled(world: &mut GivnWorld) {
    set_addon(world, "readme", false);
}

#[given(regex = r#"^the givn config contains the retired key "([^"]+)"$"#)]
fn retired_key_present(world: &mut GivnWorld, key: String) {
    let path = env(world).path("givn/config.yaml");
    let content = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let entry = format!("  {key}: true\n");
    let addons = content
        .find("\naddons:\n")
        .map(|index| index + "\naddons:\n".len())
        .expect("the fixture config has an addons block");
    let mut updated = String::from(&content[..addons]);
    updated.push_str(&entry);
    updated.push_str(&content[addons..]);
    std::fs::write(path, updated).expect("write givn/config.yaml");
}

#[then("the listing writes nothing to the project")]
fn listing_writes_nothing(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(1),
        "the fixture must be a broken listing to prove nothing was repaired"
    );
    let arc42 = env(world).path("docs/arc42");
    assert!(!arc42.exists(), "the listing re-materialized docs/arc42");
}
