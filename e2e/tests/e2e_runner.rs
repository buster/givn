//! The `@e2e` acceptance runner.
//!
//! This binary is the E2E scope's single entry point. It selects every
//! scenario tagged `@e2e` from the shared feature corpus (`GIVN_FEATURES`),
//! skips `@givn.removed` merge records, and drives the built product binary
//! through `GIVN_BIN`. It links no product crate.
//!
//! Preflight, before any scenario runs:
//!
//! - `e2e/Cargo.lock` must name no `givn`, `givn-config`, or `givn-core`
//!   package. The product's static manifest reader cannot see a registry, git,
//!   or renamed edge; this check can, and it runs on every invocation.
//! - `GIVN_BIN` must name an executable file.
//!
//! Strict mode: `fail_on_skipped()` plus the exit rule below. An undefined
//! step fails its scenario, a skipped or pending step becomes a failure, and
//! the runner exits nonzero when `failed > 0`, `skipped > 0`, or no scenario
//! ran. The scoped result file (`GIVN_RESULT_FILE`, scope from
//! `GIVN_RESULT_SCOPE`) is written before every exit, so Givn reads the real
//! counts instead of an absent file.

mod harness;
// The step tree is a port of the acceptance corpus: a helper no @e2e scenario
// reaches is dead in this package but stays so the port remains reviewable
// against the capability module it came from. The scenarios, not the item
// list, are the acceptance authority.
#[allow(dead_code)]
mod steps;

use std::path::{Path, PathBuf};

use cucumber::World as _;
use cucumber::gherkin::tagexpr::TagOperation;
use cucumber::tag::Ext as _;
use steps::GivnWorld;

const PRODUCT_PACKAGES: [&str; 3] = ["givn", "givn-config", "givn-core"];

#[tokio::main]
async fn main() {
    preflight();
    let features_path = features_path();
    let filter = e2e_filter();
    let scope = std::env::var("GIVN_RESULT_SCOPE").unwrap_or_else(|_| "e2e".to_string());
    let writer = GivnWorld::cucumber()
        .fail_on_skipped()
        .before(|_, _, scenario, world| {
            world.scenario_title = Some(scenario.name.clone());
            Box::pin(async {})
        })
        .filter_run(features_path, move |feature, _, scenario| {
            filter(feature, scenario)
        })
        .await;
    let stats = writer.scenarios_stats();
    write_result_file(&scope, stats);
    if stats.failed > 0 || stats.skipped > 0 || stats.total() == 0 {
        std::process::exit(1);
    }
}

/// Refuse the package before a scenario runs.
fn preflight() {
    refuse_product_lockfile();
    let bin = std::env::var_os("GIVN_BIN").unwrap_or_else(|| {
        eprintln!(
            "GIVN_BIN is unset: the @e2e package drives the built product. Run \
             ./run-tests.sh --e2e --package e2e, which builds and exports it."
        );
        std::process::exit(2);
    });
    let bin = PathBuf::from(bin);
    if !is_executable(&bin) {
        eprintln!(
            "GIVN_BIN points at {} which is not an executable file: run \
             ./run-tests.sh --e2e --package e2e, which builds and exports it.",
            bin.display()
        );
        std::process::exit(2);
    }
}

/// Read `Cargo.lock` and refuse a product package in it.
///
/// The static boundary reader admits a dependency edge that carries no project
/// path, so the committed lockfile is the permanent check: adding
/// `givn = "9"` or a git edge shows up here on the next run.
fn refuse_product_lockfile() {
    let lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    let text = std::fs::read_to_string(&lock).unwrap_or_else(|error| {
        eprintln!("cannot read {}: {error}", lock.display());
        std::process::exit(2);
    });
    for package in lockfile_packages(&text) {
        if PRODUCT_PACKAGES.contains(&package.as_str()) {
            eprintln!(
                "the @e2e package links the product crate `{package}` (see {}): the \
                 acceptance package must reach the product only by executing GIVN_BIN.",
                lock.display()
            );
            std::process::exit(2);
        }
    }
}

/// The `name = "…"` values of every `[[package]]` entry in a lockfile.
fn lockfile_packages(text: &str) -> Vec<String> {
    let mut packages = Vec::new();
    let mut in_package = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[[package]]";
            continue;
        }
        if in_package && let Some(rest) = trimmed.strip_prefix("name = ") {
            packages.push(rest.trim_matches('"').to_string());
        }
    }
    packages
}

fn features_path() -> String {
    std::env::var("GIVN_FEATURES").unwrap_or_else(|_| {
        eprintln!(
            "GIVN_FEATURES is unset: the @e2e package reads the shared feature corpus. \
             Run ./run-tests.sh --e2e --package e2e."
        );
        std::process::exit(2);
    })
}

/// The scope filter: `@e2e` and not `@givn.removed`.
///
/// `GIVN_CUCUMBER_TAGS` overrides it for a scoped run (for example
/// `@e2e and @some-capability`). Tags from the feature and the scenario are
/// merged, which is what cucumber does for a `--tags` run; the corpus contains
/// no `Rule`s.
fn e2e_filter() -> impl Fn(&gherkin::Feature, &gherkin::Scenario) -> bool {
    let expression = std::env::var("GIVN_CUCUMBER_TAGS").unwrap_or_default();
    let explicit = if expression.is_empty() {
        None
    } else {
        match expression.parse::<TagOperation>() {
            Ok(op) => Some(op),
            Err(error) => {
                eprintln!("invalid GIVN_CUCUMBER_TAGS expression `{expression}`: {error}");
                std::process::exit(2);
            }
        }
    };
    move |feature: &gherkin::Feature, scenario: &gherkin::Scenario| {
        let tags = feature.tags.iter().chain(&scenario.tags);
        let tags: Vec<&String> = tags.collect();
        if tags.iter().any(|tag| *tag == "givn.removed") {
            return false;
        }
        match &explicit {
            Some(op) => op.eval(tags.iter().copied()),
            None => tags.iter().any(|tag| *tag == "e2e"),
        }
    }
}

fn write_result_file(scope: &str, stats: &cucumber::writer::summarize::Stats) {
    let Ok(path) = std::env::var("GIVN_RESULT_FILE") else {
        return;
    };
    let payload = serde_json::json!({
        "scope": scope,
        "total": stats.total(),
        "passed": stats.passed,
        "failed": stats.failed,
        "skipped": stats.skipped,
    });
    if let Err(error) = std::fs::write(&path, format!("{payload}\n")) {
        eprintln!("failed to write GIVN_RESULT_FILE {path}: {error}");
        std::process::exit(2);
    }
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0 && meta.is_file())
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
