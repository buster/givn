//! Step definitions for the `trial-input-verification` capability.
//!
//! Every step drives the built binary; fixtures live in
//! `tests/steps/ise_clean_room_support.rs`. Unimplemented phrases use the
//! not-implemented stub pattern from `design.md` "Strict Mode": RED is a
//! failed step that names the phrase, never an unbound or skipped step.

use cucumber::{given, then, when};

use crate::steps::GivnWorld;

#[given(regex = r#"^a prepared Git trial with a corrupted record$"#)]
fn not_implemented_0001(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a prepared Git trial with a corrupted record"#
    ))
}

#[given(regex = r#"^a prepared Git trial with a receipt$"#)]
fn not_implemented_0002(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a prepared Git trial with a receipt"#
    ))
}

#[given(regex = r#"^a prepared Git trial with a receipt carrying an unknown field$"#)]
fn not_implemented_0003(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a prepared Git trial with a receipt carrying an unknown field"#
    ))
}

#[given(regex = r#"^a prepared Git trial with a receipt missing the counts field$"#)]
fn not_implemented_0004(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a prepared Git trial with a receipt missing the counts field"#
    ))
}

#[given(regex = r#"^a prepared Git trial with a record that is not a trial record$"#)]
fn not_implemented_0005(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: a prepared Git trial with a record that is not a trial record"#
    ))
}

#[when(
    regex = r#"^the developer changes a scenario tagged "e2e" in "givn/specs/demo/spec\.feature"$"#
)]
fn not_implemented_0006(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer changes a scenario tagged "e2e" in "givn/specs/demo/spec.feature""#
    ))
}

#[when(
    regex = r#"^the developer changes the e2e file "tests/e2e/steps/keys\.rs" in the checkout$"#
)]
fn not_implemented_0007(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the developer changes the e2e file "tests/e2e/steps/keys.rs" in the checkout"#
    ))
}

#[given(regex = r#"^the project disables architecture principles after preparation$"#)]
fn not_implemented_0008(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the project disables architecture principles after preparation"#
    ))
}

#[given(regex = r#"^the project disables ise after preparation$"#)]
fn not_implemented_0009(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the project disables ise after preparation"#
    ))
}

#[when(regex = r#"^the trial loses the retained file "givn/specs/demo/spec\.feature"$"#)]
fn not_implemented_0010(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the trial loses the retained file "givn/specs/demo/spec.feature""#
    ))
}

#[when(regex = r#"^the verifier changes the e2e file "tests/e2e/steps/keys\.rs" in the trial$"#)]
fn not_implemented_0011(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier changes the e2e file "tests/e2e/steps/keys.rs" in the trial"#
    ))
}

#[when(regex = r#"^the verifier changes the kept source file "src/keys/steps\.rs" in the trial$"#)]
fn not_implemented_0012(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier changes the kept source file "src/keys/steps.rs" in the trial"#
    ))
}

#[when(
    regex = r#"^the verifier changes the retained file "givn/specs/demo/spec\.feature" in the trial$"#
)]
fn not_implemented_0013(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier changes the retained file "givn/specs/demo/spec.feature" in the trial"#
    ))
}

#[when(
    regex = r#"^the verifier changes the retained files "givn/specs/demo/spec\.feature" and "docs/reference\.md" and the e2e file "tests/e2e/steps/keys\.rs" in the trial$"#
)]
fn not_implemented_0014(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier changes the retained files "givn/specs/demo/spec.feature" and "docs/reference.md" and the e2e file "tests/e2e/steps/keys.rs" in the trial"#
    ))
}

#[when(regex = r#"^the verifier copies the record inside the trial and re-checks it there$"#)]
fn not_implemented_0015(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier copies the record inside the trial and re-checks it there"#
    ))
}

#[when(regex = r#"^the verifier re-checks the trial against the record$"#)]
fn not_implemented_0016(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier re-checks the trial against the record"#
    ))
}

#[when(regex = r#"^the verifier rewrites the record's backend to "fossil" and re-checks it$"#)]
fn not_implemented_0017(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier rewrites the record's backend to "fossil" and re-checks it"#
    ))
}

#[when(
    regex = r#"^the verifier rewrites the record's written path and re-checks it against the trial$"#
)]
fn not_implemented_0018(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: the verifier rewrites the record's written path and re-checks it against the trial"#
    ))
}

#[then(regex = r#"^verification exits zero$"#)]
fn not_implemented_0019(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification exits zero"#
    ))
}

#[then(regex = r#"^verification fails and names "architecture_principles"$"#)]
fn not_implemented_0020(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names "architecture_principles""#
    ))
}

#[then(regex = r#"^verification fails and names "givn/specs/demo/spec\.feature"$"#)]
fn not_implemented_0021(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names "givn/specs/demo/spec.feature""#
    ))
}

#[then(regex = r#"^verification fails and names "ise"$"#)]
fn not_implemented_0022(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names "ise""#
    ))
}

#[then(regex = r#"^verification fails and names "tests/e2e/steps/keys\.rs"$"#)]
fn not_implemented_0023(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names "tests/e2e/steps/keys.rs""#
    ))
}

#[then(regex = r#"^verification fails and names all three paths$"#)]
fn not_implemented_0024(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names all three paths"#
    ))
}

#[then(regex = r#"^verification fails and names the record$"#)]
fn not_implemented_0025(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names the record"#
    ))
}

#[then(regex = r#"^verification fails and names the record and the backend$"#)]
fn not_implemented_0026(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names the record and the backend"#
    ))
}

#[then(regex = r#"^verification fails and names the record and the missing field$"#)]
fn not_implemented_0027(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names the record and the missing field"#
    ))
}

#[then(regex = r#"^verification fails and names the record and the trial path$"#)]
fn not_implemented_0028(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names the record and the trial path"#
    ))
}

#[then(regex = r#"^verification fails and names the record and the unknown field$"#)]
fn not_implemented_0029(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names the record and the unknown field"#
    ))
}

#[then(regex = r#"^verification fails and names the record path$"#)]
fn not_implemented_0030(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification fails and names the record path"#
    ))
}

#[then(regex = r#"^verification names the expected record scope$"#)]
fn not_implemented_0031(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification names the expected record scope"#
    ))
}

#[then(regex = r#"^verification reports no record error$"#)]
fn not_implemented_0032(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification reports no record error"#
    ))
}

#[then(regex = r#"^verification reports the backend mismatch$"#)]
fn not_implemented_0033(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification reports the backend mismatch"#
    ))
}

#[then(regex = r#"^verification reports the record is inside the trial$"#)]
fn not_implemented_0034(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification reports the record is inside the trial"#
    ))
}

#[then(regex = r#"^verification reports the retained and e2e identities clean$"#)]
fn not_implemented_0035(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification reports the retained and e2e identities clean"#
    ))
}

#[then(regex = r#"^verification reports the written path mismatch$"#)]
fn not_implemented_0036(_world: &mut GivnWorld) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!(
        r#"not implemented: verification reports the written path mismatch"#
    ))
}
