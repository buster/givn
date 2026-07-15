//! Gherkin runner for givn's executable specs.
//!
//! This is the single source of truth for the `.feature` files under
//! `givn/specs/`. It is invoked via `verify.command` (`cargo test --test
//! features_runner`). There is no parallel hand-maintained `#[test]` file —
//! the `.feature` file is the cucumber test, and step definitions live in
//! `tests/steps/`.

mod common;
mod steps;

use cucumber::World;
use steps::GivnWorld;

#[tokio::main]
async fn main() {
    let features_path = std::env::var("GIVN_FEATURES").unwrap_or_else(|_| "givn/specs/".to_string());
    GivnWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(features_path)
        .await;
}
