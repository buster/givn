//! Step definitions for the archive step-definition gate (non-E2E).
//!
//! Shared Givens and Thens live here; the `@e2e` companion defines only the
//! archived-file assertion its own scenario needs.

use super::archive_proof_support as support;
use crate::steps::GivnWorld;
use cucumber::given;
use std::fs;
use std::path::{Path, PathBuf};

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

#[given(regex = r#"^the change's spec tree binds "([^"]+)" in "([^"]+)"$"#)]
fn spec_tree_binds(world: &mut GivnWorld, phrase: String, relative: String) {
    write_binding(&change_root(world).join("specs").join(relative), &phrase);
}

#[given(regex = r#"^a permanent step-definition file "([^"]+)" binds "([^"]+)"$"#)]
fn permanent_step_binds(world: &mut GivnWorld, relative: String, phrase: String) {
    let path = project_root(world).join(relative);
    write_binding(&path, &phrase);
    assert!(path.exists(), "permanent step file was not written");
}
