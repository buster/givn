//! Gherkin runner for givn's executable specs.
//!
//! This is the single source of truth for the `.feature` files under
//! `givn/specs/`. It is invoked via `verify.command` (`cargo test --test
//! features_runner`). There is no parallel hand-maintained `#[test]` file —
//! the `.feature` file is the cucumber test, and step definitions live in
//! `tests/steps/`.
//!
//! The binary also implements the cargo-nextest custom-harness protocol so
//! the same suite can run under `cargo nextest run`:
//!
//! - `--list [--format terse] [--ignored]` prints one `<unit>: test` line
//!   per scenario matching `GIVN_CUCUMBER_TAGS` (`--ignored` prints the
//!   non-matching complement). Units are named
//!   `<relative feature path>::<scenario name>`.
//! - `<unit> --nocapture --exact` runs exactly that scenario (all examples
//!   of an outline in one process) via a name predicate instead of a tag
//!   expression, because cucumber's `--tags` and `--name` CLI options are
//!   mutually exclusive.
//! - Any other invocation is the plain `cargo test` path and parses argv as
//!   cucumber CLI options (e.g. `--tags`).

mod common;
mod steps;

use std::path::{Path, PathBuf};

use cucumber::{
    World,
    feature::Ext as _,
    gherkin::{GherkinEnv, tagexpr::TagOperation},
    tag::Ext as _,
};
use steps::GivnWorld;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list") {
        nextest_list(&args);
        return;
    }
    if let Some(unit) = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .filter(|_| args.iter().any(|a| a == "--exact"))
    {
        nextest_run(unit).await;
        return;
    }
    let features_path = features_path();
    GivnWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(features_path)
        .await;
}

fn features_path() -> String {
    std::env::var("GIVN_FEATURES").unwrap_or_else(|_| "givn/specs/".to_string())
}

/// cargo-nextest discovery: print `<unit>: test` lines for scenarios that
/// satisfy `GIVN_CUCUMBER_TAGS`, or — with `--ignored` — for those that do
/// not. Tag evaluation mirrors cucumber's own filtering: tags from `Feature`
/// and `Scenario` are merged (specs contain no `Rule`s) on the expanded
/// feature, so the unit set matches what a `--tags` run would execute.
fn nextest_list(args: &[String]) {
    let ignored_phase = args.iter().any(|a| a == "--ignored");
    let tags_expr = std::env::var("GIVN_CUCUMBER_TAGS").unwrap_or_default();
    let tags_filter = if tags_expr.is_empty() {
        None
    } else {
        match tags_expr.parse::<TagOperation>() {
            Ok(op) => Some(op),
            Err(e) => {
                eprintln!("invalid GIVN_CUCUMBER_TAGS expression `{tags_expr}`: {e}");
                std::process::exit(2);
            }
        }
    };

    for (rel, feature) in parsed_features() {
        for scenario in &feature.scenarios {
            let matches = tags_filter
                .as_ref()
                .is_none_or(|tags| tags.eval(feature.tags.iter().chain(&scenario.tags)));
            if matches == ignored_phase {
                continue;
            }
            println!("{}::{}: test", rel, scenario.name);
        }
    }
}

/// cargo-nextest execution: `<unit> --nocapture --exact` runs the single
/// named scenario through a name predicate instead of the CLI `--name`
/// regex, so scenario names never need escaping.
async fn nextest_run(unit: &str) {
    let scenario_name = unit.rsplit("::").next().unwrap_or(unit).to_string();
    GivnWorld::cucumber()
        .with_default_cli()
        .fail_on_skipped()
        .filter_run_and_exit(features_path(), move |_, _, scenario| {
            scenario.name == scenario_name
        })
        .await;
}

fn parsed_features() -> Vec<(String, cucumber::gherkin::Feature)> {
    let root = PathBuf::from(features_path());
    let root = root.canonicalize().unwrap_or(root);
    if root.is_file() {
        return vec![(root.to_string_lossy().into_owned(), parse_feature(&root))];
    }
    let mut files = Vec::new();
    collect_feature_files(&root, &mut files);
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let rel = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            (rel, parse_feature(&path))
        })
        .collect()
}

fn parse_feature(path: &Path) -> cucumber::gherkin::Feature {
    let feature = cucumber::gherkin::Feature::parse_path(path, GherkinEnv::default())
        .unwrap_or_else(|e| {
            eprintln!("failed to parse {}: {e}", path.display());
            std::process::exit(1);
        });
    feature.expand_examples().unwrap_or_else(|e| {
        eprintln!("failed to expand examples in {}: {e}", path.display());
        std::process::exit(1);
    })
}

fn collect_feature_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read dir {}: {e}", dir.display()))
        .map(|e| e.expect("read dir entry"))
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_feature_files(&path, out);
        } else if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("feature"))
        {
            out.push(path);
        }
    }
}
