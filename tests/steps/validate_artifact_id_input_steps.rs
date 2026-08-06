use crate::steps::{GivnWorld, stderr};
use cucumber::then;

#[then("stderr lists known artifact ids")]
fn stderr_lists_known_artifacts(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.contains("proposal") || s.contains("tasks") || s.contains("review"),
        "expected stderr to list known artifact ids, got: {}",
        s,
    );
}

#[then(regex = r#"^stderr says the artifact is not in the effective manifest$"#)]
fn stderr_says_not_in_manifest(world: &mut GivnWorld) {
    let s = stderr(world);
    assert!(
        s.contains("not in") || s.contains("unknown artifact"),
        "expected stderr to mention 'not in' or 'unknown artifact', got: {}",
        s,
    );
}
