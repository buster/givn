//! Unit-level step definitions for the run-command declaration capability.
//!
//! These steps work directly on the manifest/overlay layer
//! and the validation/render/backfill paths — no
//! subprocess is started.

use super::GivnWorld;
use cucumber::then;

fn status_text(world: &GivnWorld) -> String {
    let out = crate::steps::output(world);
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[then(regex = r#"^stdout reports verification as (current|stale|not checked)$"#)]
fn stdout_reports_verification(world: &mut GivnWorld, state: String) {
    let text = status_text(world);
    let observed = if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        value["verification"]
            .as_str()
            .unwrap_or("missing")
            .to_string()
    } else {
        text.lines()
            .find_map(|line| line.strip_prefix("Verification: "))
            .map(|value| value.trim().to_string())
            .unwrap_or_else(|| "missing".to_string())
    };
    assert_eq!(observed, state, "{text}");
}

#[then("the status reports required artifacts as complete")]
fn status_reports_artifacts_complete(world: &mut GivnWorld) {
    let text = status_text(world);
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        assert_eq!(value["all_required_done"], true, "{text}");
    } else {
        assert!(text.contains("All required artifacts complete."), "{text}");
    }
}

#[then("stdout reports that required artifacts are complete")]
fn stdout_reports_required_complete(world: &mut GivnWorld) {
    status_reports_artifacts_complete(world);
}

#[then("stdout does not claim \"ready-to-archive\" or \"Ready to archive\"")]
fn stdout_does_not_claim_ready(world: &mut GivnWorld) {
    let text = status_text(world);
    assert!(!text.contains("ready-to-archive"), "{text}");
    assert!(!text.contains("Ready to archive"), "{text}");
}
