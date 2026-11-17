use crate::steps::GivnWorld;
use cucumber::given;

fn change_dir(world: &GivnWorld) -> std::path::PathBuf {
    let root = world
        .env
        .as_ref()
        .map(|e| e.path(""))
        .expect("a project root is required first");
    let id = world
        .last_change_id
        .clone()
        .expect("a change id is required");
    root.join(format!("givn/changes/{id}"))
}

#[given("the change's review.md has no disposition entries")]
fn review_md_no_dispositions(world: &mut GivnWorld) {
    let path = change_dir(world).join("review.md");
    std::fs::write(
        &path,
        "README-IMPACT: none\n\n# Review\n\nNo overlap dispositions recorded.\n\nREVIEW: PASS\n",
    )
    .expect("write review.md without dispositions");
}
