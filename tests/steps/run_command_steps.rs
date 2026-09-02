//! Unit-level step definitions for the run-command declaration capability.
//!
//! These steps work directly on the manifest/overlay layer
//! (`givn::config`) and the validation/render/backfill functions — no
//! subprocess is started.

use cucumber::{given, then, when};
use gherkin::Step;

use super::GivnWorld;

// --- Given: declarations on disk ---------------------------------------------

#[given(regex = r#"^a commands file with a run declaration:$"#)]
fn commands_file_with_run_declaration(world: &mut GivnWorld, step: &Step) {
    let env = world.env.get_or_insert_with(crate::common::make_bare_root);
    let content = step
        .docstring
        .as_ref()
        .expect("docstring content for run declaration");
    let path = env.path("givn/commands.yaml");
    std::fs::create_dir_all(path.parent().expect("givn dir")).expect("create givn dir");
    std::fs::write(&path, content).expect("write commands.yaml");
}

// --- When: load ---------------------------------------------------------------

#[when("the commands file is loaded")]
fn commands_file_is_loaded(world: &mut GivnWorld) {
    let env = world.env.get_or_insert_with(crate::common::make_bare_root);
    let base = givn::config::manifest::Manifest::load_embedded().expect("load default manifest");
    let content =
        std::fs::read_to_string(env.path("givn/commands.yaml")).expect("read commands.yaml");
    match givn::config::overlay::apply(base, &content) {
        Ok(m) => {
            world.overlay_manifest = Some(m);
            world.overlay_error = None;
        }
        Err(e) => {
            world.overlay_error = Some(e.to_string());
        }
    }
}

// --- Then ---------------------------------------------------------------------

#[then(regex = r#"^loading fails mentioning "(.*)"$"#)]
fn loading_fails_mentioning(world: &mut GivnWorld, needle: String) {
    let err = world
        .overlay_error
        .as_ref()
        .expect("expected loading to fail");
    assert!(
        err.contains(&needle),
        "load error {err:?} does not mention {needle:?}"
    );
}
