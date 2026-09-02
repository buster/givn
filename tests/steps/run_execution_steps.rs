//! Unit-level step definitions for the run-execution capability.
//!
//! These steps drive the executor through its library entry point and the
//! probe primitives — no givn subprocess is started (that is the @e2e
//! file's job). `process::exit` never runs inside the runner.

use cucumber::then;

use super::GivnWorld;

#[then(regex = r#"^stderr mentions "(.*)"$"#)]
fn stderr_mentions(world: &mut GivnWorld, needle: String) {
    let s = super::stderr(world);
    let unescaped = needle.replace("\\\"", "\"");
    assert!(s.contains(&unescaped), "stderr missing {unescaped:?}: {s}");
}
