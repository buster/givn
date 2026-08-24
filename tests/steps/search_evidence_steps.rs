//! Regular black-box steps for optional cross-encoder search evidence.

use cucumber::{given, when};

use super::GivnWorld;

const SEARCH_QUERY: &str = "the command rejects an unknown provider";

#[given("local NLI evidence is enabled")]
fn local_nli_enabled(world: &mut GivnWorld) {
    world.retrieval_reranker = Some("nli-stub".to_string());
}

#[given("every BGE pair exceeds the token limit")]
fn every_bge_pair_over_limit(world: &mut GivnWorld) {
    unimplemented!("every BGE pair exceeds the token limit");
}

#[given("every NLI pair exceeds the token limit")]
fn every_nli_pair_over_limit(world: &mut GivnWorld) {
    unimplemented!("every NLI pair exceeds the token limit");
}

#[given("BGE evidence recommends all listed candidates")]
fn bge_recommends_all(world: &mut GivnWorld) {
    world.retrieval_bge_scores = Some(bge_recommend_fixture().to_string());
}

#[given("NLI evidence recommends all listed candidates")]
fn nli_recommends_all(world: &mut GivnWorld) {
    world.retrieval_nli_scores = Some(nli_recommend_fixture().to_string());
}

#[when(regex = r#"^the user searches with evidence for "([^"]+)"$"#)]
fn search_with_evidence(world: &mut GivnWorld, query: String) {
    let env = world.env.as_ref().expect("retrieval project is required");
    world.last_output = Some(super::run_givn(
        env,
        &["spec", "search", "--evidence", query.as_str()],
        world,
    ));
}

pub(crate) fn bge_recommend_fixture() -> String {
    let query = format!("__query__|{SEARCH_QUERY}");
    let candidates = [
        "givn/specs/retrieval.feature|Provider name is unknown",
        "givn/specs/retrieval.feature|Authentication token is missing",
        "givn/specs/retrieval.feature|Provider configuration is valid",
    ];
    candidates
        .iter()
        .flat_map(|candidate| {
            [
                format!("{query}->{candidate}=3.1000"),
                format!("{candidate}->{query}=3.1000"),
            ]
        })
        .collect::<Vec<_>>()
        .join(";")
}


pub(crate) fn nli_recommend_fixture() -> String {
    let query = format!("__query__|{SEARCH_QUERY}");
    let candidates = [
        "givn/specs/retrieval.feature|Provider name is unknown",
        "givn/specs/retrieval.feature|Authentication token is missing",
        "givn/specs/retrieval.feature|Provider configuration is valid",
    ];
    candidates
        .iter()
        .flat_map(|candidate| {
            [
                format!("{query}->{candidate}=4.0000,0.0000,0.0000"),
                format!("{candidate}->{query}=4.0000,0.0000,0.0000"),
            ]
        })
        .collect::<Vec<_>>()
        .join(";")
}
