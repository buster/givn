//! Shared fixture/read helpers for the template-conditional-rendering steps.
//! The non-e2e steps build fixtures by editing `givn/config.yaml` directly and
//! then running `givn skills sync`; both the full and e2e scenarios read the
//! generated files through these helpers.

use std::path::PathBuf;

use crate::common::{TestEnv, run_givn};
use crate::steps::GivnWorld;

/// The fixture project root, panicking if none was set.
pub fn env(world: &GivnWorld) -> &TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

/// Run `givn skills sync` in the fixture and record the output.
pub fn sync(world: &mut GivnWorld) {
    let out = run_givn(env(world), &["skills", "sync"]);
    world.last_output = Some(out);
}

/// Set an addon's enabled flag directly in the fixture's `givn/config.yaml`
/// (mirrors the `givn addon "..." is "..."` step, but keyed on a bool).
/// The config keys use underscores (`domain_modeling`), while the step
/// language spells the addon with a hyphen (`domain-modeling`), so the
/// hyphenated form is normalized to the config key.
pub fn set_addon(world: &mut GivnWorld, name: &str, enabled: bool) {
    let key = name.replace('-', "_");
    let path = env(world).path("givn/config.yaml");
    let config = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let value = if enabled { "true" } else { "false" };
    let target_line = format!("  {key}: {value}");
    let mut lines: Vec<String> = config.lines().map(|l| l.to_string()).collect();
    let mut found = false;
    for line in &mut lines {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{key}:")) && !line.trim_start().starts_with('#') {
            *line = target_line.clone();
            found = true;
            break;
        }
    }
    if !found {
        let mut inserted = false;
        for i in 0..lines.len() {
            if lines[i].trim() == "addons:" {
                lines.insert(i + 1, target_line.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            lines.push("addons:".to_string());
            lines.push(target_line);
        }
    }
    std::fs::write(&path, lines.join("\n") + "\n").expect("write givn/config.yaml");
}

/// The target directories (`.opencode`, `.agents`, `.claude`) present in the
/// fixture. Imported from the givn/install-... templates; `givn init` defaults
/// to opencode only.
pub fn targets(world: &GivnWorld) -> Vec<String> {
    let mut out = Vec::new();
    for dir in [".opencode", ".agents", ".claude"] {
        if env(world).path(dir).is_dir() {
            out.push(dir.to_string());
        }
    }
    out
}

/// Candidate generated skill files across the present targets.
pub fn skill_paths(world: &GivnWorld, name: &str) -> Vec<PathBuf> {
    targets(world)
        .iter()
        .map(|t| env(world).path(&format!("{t}/skills/{name}/SKILL.md")))
        .collect()
}

/// Contents of every existing generated copy of a skill. Empty when the skill
/// was not generated at all.
pub fn skill_texts(world: &GivnWorld, name: &str) -> Vec<String> {
    skill_paths(world, name)
        .iter()
        .filter(|p| p.exists())
        .map(|p| std::fs::read_to_string(p).expect("read generated skill"))
        .collect()
}

/// The generated opencode command path for a command name.
pub fn command_path(world: &GivnWorld, name: &str) -> PathBuf {
    env(world).path(&format!(".opencode/commands/{name}.md"))
}

/// Contents of a generated opencode command, if present.
pub fn command_text(world: &GivnWorld, name: &str) -> Option<String> {
    let path = command_path(world, name);
    path.exists()
        .then(|| std::fs::read_to_string(&path).expect("read generated command file"))
}

/// Every generated skill and command markdown file across the present targets.
pub fn all_generated_files(world: &GivnWorld) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let root = env(world).root();
    for target in targets(world) {
        let skills = root.join(&target).join("skills");
        if skills.is_dir() {
            let mut stack = vec![skills];
            while let Some(dir) = stack.pop() {
                for entry in std::fs::read_dir(&dir).expect("read skills dir") {
                    let entry = entry.expect("skills dir entry");
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                        out.push(path);
                    }
                }
            }
        }
    }
    let commands = root.join(".opencode/commands");
    if commands.is_dir() {
        for entry in std::fs::read_dir(&commands).expect("read commands dir") {
            let path = entry.expect("commands dir entry").path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                out.push(path);
            }
        }
    }
    out
}
