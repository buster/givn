//! Step definitions for the `clean-room-preparation (E2E)` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. Unimplemented phrases use the
//! not-implemented stub pattern from `design.md` "Strict Mode": RED is a
//! failed step that names the phrase, never an unbound or skipped step.

use cucumber::{given, then, when};

use crate::steps::GivnWorld;

#[given(
    regex = r#"^a Git project with ise enabled and a declaration that retains "givn/specs/\*\*", declares "tests/e2e", and replaces "src/\*\*"$"#
)]
fn not_implemented_0001(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a Git project with ise enabled and a declaration that retains "givn/specs/**", declares "tests/e2e", and replaces "src/**""#
    ))
}

#[given(
    regex = r#"^a committed revision with "givn/specs/demo\.feature", "tests/e2e/Cargo\.toml", "tests/e2e/steps/demo\.rs", "src/keys/parser\.rs", "src/keys/mod\.rs", "src/other\.rs", and "notes\.md"$"#
)]
fn not_implemented_0002(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", "src/keys/parser.rs", "src/keys/mod.rs", "src/other.rs", and "notes.md""#
    ))
}

#[when(
    regex = r#"^the developer prepares a trial from "HEAD" with the path "src/keys/parser\.rs" and a receipt$"#
)]
fn not_implemented_0003(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs" and a receipt"#
    ))
}

#[then(
    regex = r#"^the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version$"#
)]
fn not_implemented_0004(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version"#
    ))
}

#[then(
    regex = r#"^the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis$"#
)]
fn not_implemented_0005(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis"#
    ))
}

#[then(
    regex = r#"^the report prints "Backend: git" and a "Revision:" line holding the resolved 40-hex commit$"#
)]
fn not_implemented_0006(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the report prints "Backend: git" and a "Revision:" line holding the resolved 40-hex commit"#
    ))
}

#[then(
    regex = r#"^the report prints "Retained: 3", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1"$"#
)]
fn not_implemented_0007(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the report prints "Retained: 3", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1""#
    ))
}

#[then(regex = r#"^the report prints the written trial path and the receipt path$"#)]
fn not_implemented_0008(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the report prints the written trial path and the receipt path"#
    ))
}

#[then(
    regex = r#"^the trial holds the retained and kept source files, including "src/keys/mod\.rs"$"#
)]
fn not_implemented_0009(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial holds the retained and kept source files, including "src/keys/mod.rs""#
    ))
}

#[then(regex = r#"^the trial omits "src/keys/parser\.rs" and "notes\.md"$"#)]
fn not_implemented_0010(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial omits "src/keys/parser.rs" and "notes.md""#
    ))
}
