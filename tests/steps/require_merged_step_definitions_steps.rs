//! Step definitions for the archive step-definition gate (non-E2E).
//!
//! Shared Givens and Thens live here; the `@e2e` companion defines only the
//! archived-file assertion its own scenario needs.

use std::fs;
use std::path::{Path, PathBuf};

use cucumber::{given, then};

use super::archive_proof_support as support;
use crate::steps::{GivnWorld, stderr};

fn project_root(world: &GivnWorld) -> PathBuf {
    support::root(world)
}

fn change_root(world: &GivnWorld) -> PathBuf {
    let id = world.last_change_id.clone().expect("a change id");
    project_root(world).join("givn/changes").join(id)
}

fn write_binding(path: &Path, phrase: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create step directory");
    }
    fs::write(
        path,
        format!("@given(\"{phrase}\")\ndef step():\n    raise NotImplementedError(\"TODO\")\n"),
    )
    .expect("write step-definition file");
}

fn is_step_source(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if name.ends_with("_steps.py") || name.ends_with("_steps.rs") {
        return true;
    }
    path.parent().is_some_and(|parent| {
        parent.components().any(|component| {
            let name = component.as_os_str().to_string_lossy();
            name == "steps" || name.ends_with("_steps")
        })
    })
}

fn collect_permanent_step_text(root: &Path) -> String {
    fn walk(root: &Path, directory: &Path, text: &mut String) {
        let Ok(entries) = fs::read_dir(directory) else {
            return;
        };
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if matches!(
                name.as_str(),
                ".git" | "target" | "__pycache__" | "node_modules" | ".venv" | "venv"
            ) {
                continue;
            }
            if path.is_dir() {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                if relative.starts_with("givn/changes")
                    || relative.starts_with("givn/archive")
                    || relative.starts_with("givn/history")
                {
                    continue;
                }
                walk(root, &path, text);
            } else if is_step_source(&path)
                && let Ok(content) = fs::read_to_string(&path)
            {
                text.push_str(&content);
                text.push('\n');
            }
        }
    }
    let mut text = String::new();
    walk(root, root, &mut text);
    text
}

#[given(regex = r#"^the change's spec tree binds "([^"]+)" in "([^"]+)"$"#)]
fn spec_tree_binds(world: &mut GivnWorld, phrase: String, relative: String) {
    write_binding(&change_root(world).join("specs").join(relative), &phrase);
}

#[given(regex = r#"^no permanent step-definition file binds "([^"]+)"$"#)]
fn no_permanent_step_binds(world: &mut GivnWorld, phrase: String) {
    let text = collect_permanent_step_text(&project_root(world));
    assert!(
        !text.contains(&phrase),
        "fixture permanent step tree unexpectedly binds {phrase:?}: {text}"
    );
}

#[given(regex = r#"^the change's spec tree binds 21 distinct step phrases in "([^"]+)"$"#)]
fn spec_tree_binds_21(_world: &mut GivnWorld, _relative: String) {
    todo!("TODO: write 21 delta step phrases");
}

#[given("no permanent step-definition file binds any of those step phrases")]
fn no_permanent_binds_any(_world: &mut GivnWorld) {
    todo!("TODO: assert the permanent tree lacks the phrases");
}

#[given(regex = r#"^the candidate's spec tree binds "([^"]+)" in "([^"]+)"$"#)]
fn candidate_spec_tree_binds(_world: &mut GivnWorld, _phrase: String, _relative: String) {
    todo!("TODO: write the candidate's delta step file");
}

#[given(regex = r#"^a permanent step-definition file "([^"]+)" binds "([^"]+)"$"#)]
fn permanent_step_binds(_world: &mut GivnWorld, _relative: String, _phrase: String) {
    todo!("TODO: write the permanent step-definition file");
}

#[then("stderr says the definition must be merged into the permanent per-capability step file")]
fn stderr_says_merge(world: &mut GivnWorld) {
    let text = stderr(world);
    assert!(
        text.contains("permanent per-capability step file"),
        "stderr does not name the repair: {text}"
    );
}

#[then("stderr names the alphabetically first 20 missing step phrases")]
fn stderr_names_first_20(_world: &mut GivnWorld) {
    todo!("TODO: assert the first 20 phrases are named");
}

#[then("stderr does not name the twenty-first missing step phrase")]
fn stderr_omits_21st(_world: &mut GivnWorld) {
    todo!("TODO: assert the 21st phrase is omitted");
}

#[then(regex = r#"^stderr says "([^"]+)"$"#)]
fn stderr_says(_world: &mut GivnWorld, _text: String) {
    todo!("TODO: assert the exact refusal fragment");
}

#[then("the candidate is still unproven")]
fn candidate_still_unproven(_world: &mut GivnWorld) {
    todo!("TODO: assert the candidate stays unproven");
}

#[then("no regular scope result exists for the candidate")]
fn no_regular_result(_world: &mut GivnWorld) {
    todo!("TODO: assert the record runner did not run");
}
