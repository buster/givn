//! E2E steps for the real evidence-search CLI.

use cucumber::{given, when};

use super::GivnWorld;

#[given("an end-to-end project with a permanent spec index for evidence search")]
fn e2e_evidence_project(world: &mut GivnWorld) {
    unimplemented!("end-to-end evidence search project");
}

#[given("BGE and NLI evidence are enabled for the end-to-end search")]
fn e2e_evidence_enabled(world: &mut GivnWorld) {
    unimplemented!("BGE/NLI evidence enabled for e2e search");
}

#[given("BGE and NLI evidence recommend the listed candidates")]
fn e2e_evidence_recommends(world: &mut GivnWorld) {
    unimplemented!("BGE/NLI evidence recommends the listed candidates");
}

#[when(regex = r#"^the e2e user searches "([^"]+)" with evidence$"#)]
fn e2e_search_with_evidence(world: &mut GivnWorld, query: String) {
    unimplemented!("e2e search with evidence");
}