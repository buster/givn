//! Regular step definitions for the 0.3.0 release and migration contract.

use crate::steps::GivnWorld;
use cucumber::{given, then, when};

#[given("an installed Givn binary")]
fn installed_givn_binary(_world: &mut GivnWorld) {
    let bin = crate::harness::binary::givn();
    assert!(bin.exists(), "the compiled Givn binary is missing");
}

#[when("the consumer invokes the retired `givn spec index` command")]
fn consumer_runs_spec_index(world: &mut GivnWorld) {
    let root = crate::harness::make_init_root();
    let output = std::process::Command::new(crate::harness::binary::givn())
        .args(["spec", "index"])
        .current_dir(root.root())
        .output()
        .expect("spawn givn spec index");
    world.last_output = Some(output);
}

#[then(expr = "the error reports \"spec index\" is not an available command")]
fn retired_command_error_names_it(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("a command was not run");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand"), "{stderr}");
    assert!(stderr.contains("'index'"), "{stderr}");
}

#[then("the error output stays free of \"embedding\" and \"download\" wording")]
fn retired_command_error_is_engine_free(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("a command was not run");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("embedding"), "{stderr}");
    assert!(!stderr.contains("download"), "{stderr}");
}

#[when("the consumer invokes the retired `givn rewrite-history` command")]
fn consumer_runs_rewrite_history(world: &mut GivnWorld) {
    let root = crate::harness::make_init_root();
    let output = std::process::Command::new(crate::harness::binary::givn())
        .args(["rewrite-history"])
        .current_dir(root.root())
        .output()
        .expect("spawn givn rewrite-history");
    world.last_output = Some(output);
}

#[then(expr = "the error reports \"rewrite-history\" is not an available command")]
fn rewrite_history_error_names_it(world: &mut GivnWorld) {
    let output = world.last_output.as_ref().expect("a command was not run");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand"), "{stderr}");
    assert!(stderr.contains("'rewrite-history'"), "{stderr}");
}
