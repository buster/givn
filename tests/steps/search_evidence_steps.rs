//! Regular steps for optional cross-encoder search evidence.

use cucumber::{given, when};

use super::GivnWorld;

#[given("local NLI evidence is enabled")]
fn local_nli_enabled(world: &mut GivnWorld) {
    unimplemented!("local NLI evidence is enabled");
}

#[given("every BGE pair exceeds the token limit")]
fn every_bge_pair_over_limit(world: &mut GivnWorld) {
    unimplemented!("every BGE pair exceeds the token limit");
}

#[given("every NLI pair exceeds the token limit")]
fn every_nli_pair_over_limit(world: &mut GivnWorld) {
    unimplemented!("every NLI pair exceeds the token limit");
}

#[given("BGE evidence recommends the listed candidates")]
fn bge_recommends_all(world: &mut GivnWorld) {
    unimplemented!("BGE evidence recommends the listed candidates");
}

#[given("NLI evidence recommends the listed candidates")]
fn nli_recommends_all(world: &mut GivnWorld) {
    unimplemented!("NLI evidence recommends the listed candidates");
}

#[when(regex = r#"^the user searches with evidence for "([^"]+)"$"#)]
fn search_with_evidence(world: &mut GivnWorld, query: String) {
    unimplemented!("the search with evidence");
}