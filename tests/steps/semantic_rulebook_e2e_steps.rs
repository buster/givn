//! E2E step definitions for the unified semantic rulebook capability.
//!
//! Consumer-flow assertions over the real CLI subprocess: per-stage pair
//! accounting, pair-cache accounting, and the archive gate's stdout-only
//! rendering. Fixtures come from `semantic_rulebook_steps` and the proven
//! semantic-review archive fixture.

use cucumber::then;

use super::GivnWorld;

fn last_stdout(world: &GivnWorld) -> String {
    super::stdout(world)
}

fn report_for(world: &GivnWorld) -> String {
    let env = world.env.as_ref().expect("project root is required");
    std::fs::read_to_string(env.path("givn/changes/running-change/semantic-review.md"))
        .expect("semantic review report exists")
}

#[then(regex = r#"^the review output records exactly ([0-9]+) unique pairs? and ([0-9]+) directional calls per stage$"#)]
fn review_records_pair_counts(world: &mut GivnWorld, pairs: usize, calls: usize) {
    let stdout = last_stdout(world);
    // Both stage lines record the counts: BGE and NLI.
    let expected_pair = format!("unique pairs {pairs},");
    let expected_calls = format!("directional calls {calls},");
    let bge_line = stdout
        .lines()
        .find(|line| line.starts_with("BGE: model"))
        .unwrap_or_else(|| panic!("no BGE stage line:\n{stdout}"));
    let nli_line = stdout
        .lines()
        .find(|line| line.starts_with("NLI status:"))
        .unwrap_or_else(|| panic!("no NLI stage line:\n{stdout}"));
    for line in [bge_line, nli_line] {
        assert!(
            line.contains(&expected_pair),
            "expected `{expected_pair}` in `{line}`"
        );
        assert!(
            line.contains(&expected_calls),
            "expected `{expected_calls}` in `{line}`"
        );
    }
}

#[then("the review output records both second stages enabled")]
fn review_records_both_stages_enabled(world: &mut GivnWorld) {
    let stdout = last_stdout(world);
    assert!(
        stdout.contains("BGE status: BGE_ENABLED"),
        "BGE stage must be enabled:\n{stdout}"
    );
    assert!(
        stdout.contains("NLI status: NLI_ENABLED"),
        "NLI stage must be enabled:\n{stdout}"
    );
}

#[then("the review output records no unscored candidates")]
fn review_records_no_unscored(world: &mut GivnWorld) {
    let stdout = last_stdout(world);
    assert!(
        !stdout.contains("BGE_UNSCORED") && !stdout.contains("unscored 1"),
        "no unscored candidates expected:\n{stdout}"
    );
}

#[then(regex = r#"^the pair cache accounting reports "([^"]+)" reused and "([^"]+)" computed$"#)]
fn pair_cache_accounting(world: &mut GivnWorld, reused: String, computed: String) {
    let expected = format!("pair cache: {reused} reused, {computed} computed");
    let stdout = last_stdout(world);
    assert!(
        stdout.contains(&expected),
        "expected `{expected}` in stdout:\n{stdout}"
    );
}

#[then("the archived rulebook change directory exists")]
fn archived_rulebook_change_directory(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root is required");
    assert!(
        env.path("givn/archive/running-change").is_dir(),
        "the change must have been moved to givn/archive/running-change/"
    );
    assert!(
        !env.path("givn/changes/running-change").exists(),
        "the active change directory must be gone after archiving"
    );
}

#[then("the archive gate output reports the pair cache accounting")]
fn archive_gate_reports_cache_accounting(world: &mut GivnWorld) {
    let stdout = last_stdout(world);
    assert!(
        stdout.contains("pair cache:"),
        "archive gate output must report the pair cache accounting:\n{stdout}"
    );
}

#[then("the archived semantic review report is retained")]
fn archived_report_retained(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("project root is required");
    assert!(
        env
            .path("givn/archive/running-change/semantic-review.md")
            .is_file(),
        "archived change must retain the semantic review report"
    );
    assert!(
        !last_stdout(world).contains("semantic review report saved"),
        "the archive gate must not write the worklist itself"
    );
}

#[then("both reports contain the same candidate ids in the same order")]
fn reports_share_candidate_order(world: &mut GivnWorld) {
    let first = String::from_utf8(
        world
            .semantic_report_snapshot
            .as_ref()
            .expect("first report was snapshotted")
            .clone(),
    )
    .expect("snapshot is utf-8");
    let second = report_for(world);
    let ids = |report: &str| -> Vec<String> {
        report
            .lines()
            .filter_map(|line| line.strip_prefix("- id: "))
            .map(str::to_string)
            .collect()
    };
    assert_eq!(
        ids(&first),
        ids(&second),
        "both runs must list identical candidate ids in identical order"
    );
}
