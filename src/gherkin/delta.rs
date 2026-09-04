use anyhow::{Result, bail};
use gherkin::{Feature, Scenario};

/// The delta operation for a scenario (derived from its tags).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeltaOp {
    /// `@givn.added` or no delta tag: append to permanent spec.
    Added,
    /// `@givn.modified`: replace scenario by title.
    Modified,
    /// `@givn.removed`: delete scenario by title.
    Removed,
}

/// A scenario paired with its delta operation.
#[derive(Debug, Clone)]
pub struct DeltaScenario<'a> {
    pub scenario: &'a Scenario,
    pub op: DeltaOp,
}

/// Returns true if the feature is tagged as a givn delta (`@givn.delta`).
pub fn is_delta_feature(feature: &Feature) -> bool {
    feature.tags.iter().any(|t| t == "givn.delta")
}

/// Returns true if the feature is a capability-retirement marker
/// (`@givn.retired`). A retirement delta has zero scenarios — it exists
/// only to tell `givn archive` to delete a whole capability directory
/// instead of merging.
pub fn is_retired_feature(feature: &Feature) -> bool {
    feature.tags.iter().any(|t| t == "givn.retired")
}

/// Extract the capability tag from a feature (first tag that isn't a `@givn.*` tag).
pub fn capability_tag(feature: &Feature) -> Option<&str> {
    feature.tags.iter().find_map(|t| {
        if !t.starts_with("givn.") {
            Some(t.as_str())
        } else {
            None
        }
    })
}

/// Reject a capability tag that is not a safe, unambiguous path segment.
/// `capability_tag` returns an arbitrary string taken from a feature tag;
/// `archive.rs` interpolates it into a filesystem path twice (merge target)
/// and now also uses it to select a directory to delete (retirement). A
/// tag containing `..`, `/`, or an empty string must never reach either.
pub fn validate_capability_tag(tag: &str) -> Result<()> {
    let ok = !tag.is_empty()
        && tag
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !tag.starts_with('-')
        && !tag.ends_with('-');
    if !ok {
        bail!(
            "invalid capability tag '{tag}': must be non-empty lowercase \
             kebab-case ([a-z0-9-]+), no leading or trailing '-'"
        );
    }
    Ok(())
}

/// Parse the delta operation from a scenario's tags.
/// Default (no delta tag) is `Added`.
pub fn delta_op(scenario: &Scenario) -> DeltaOp {
    for tag in &scenario.tags {
        match tag.as_str() {
            "givn.added" => return DeltaOp::Added,
            "givn.modified" => return DeltaOp::Modified,
            "givn.removed" => return DeltaOp::Removed,
            _ => {}
        }
    }
    DeltaOp::Added // default
}

/// Extract all delta scenarios from a feature.
pub fn delta_scenarios(feature: &Feature) -> Vec<DeltaScenario<'_>> {
    feature
        .scenarios
        .iter()
        .map(|s| DeltaScenario {
            scenario: s,
            op: delta_op(s),
        })
        .collect()
}

/// Returns true if the scenario is a `@givn.removed` placeholder.
/// These should not trigger @wip warnings in lint.
pub fn is_removed_placeholder(scenario: &Scenario) -> bool {
    scenario.tags.iter().any(|t| t == "givn.removed")
}

/// Returns Some(message) if the feature contains any non-removed @wip scenarios.
pub fn find_wip_in_feature(feature: &Feature) -> Option<String> {
    for scenario in &feature.scenarios {
        if !is_removed_placeholder(scenario) && scenario.tags.iter().any(|t| t == "wip") {
            return Some(format!("scenario '{}' is still tagged @wip", scenario.name));
        }
    }
    None
}

/// Strip all `@givn.*` tags from a list of tags.
pub fn strip_givn_tags(tags: &[String]) -> Vec<String> {
    tags.iter()
        .filter(|t| !t.starts_with("givn."))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gherkin::parse::parse_str;

    const DELTA_FEATURE: &str = r#"
@givn.delta @user-auth
Feature: User authentication

  @givn.added @wip
  Scenario: User logs in
    Given a registered user
    When they provide valid credentials
    Then they are authenticated

  @givn.modified
  Scenario: User logs out
    Given a logged-in user
    When they log out
    Then their session is ended

  @givn.removed
  Scenario: Old login flow
    Given the old flow exists
"#;

    #[test]
    fn detect_delta_feature() {
        let feature = parse_str(DELTA_FEATURE).unwrap();
        assert!(is_delta_feature(&feature));
    }

    #[test]
    fn extract_capability_tag() {
        let feature = parse_str(DELTA_FEATURE).unwrap();
        assert_eq!(capability_tag(&feature), Some("user-auth"));
    }

    #[test]
    fn parse_delta_ops() {
        let feature = parse_str(DELTA_FEATURE).unwrap();
        let deltas = delta_scenarios(&feature);
        assert_eq!(deltas.len(), 3);
        assert_eq!(deltas[0].op, DeltaOp::Added);
        assert_eq!(deltas[1].op, DeltaOp::Modified);
        assert_eq!(deltas[2].op, DeltaOp::Removed);
    }

    #[test]
    fn strip_givn_tags_removes_givn_prefix() {
        let tags = vec![
            "givn.delta".to_string(),
            "givn.added".to_string(),
            "wip".to_string(),
            "user-auth".to_string(),
        ];
        let stripped = strip_givn_tags(&tags);
        assert_eq!(stripped, vec!["wip", "user-auth"]);
    }

    #[test]
    fn default_delta_op_is_added() {
        let input = r#"
Feature: Plain feature
  Scenario: No delta tag
    Given something
    When it happens
    Then it works
"#;
        let feature = parse_str(input).unwrap();
        let deltas = delta_scenarios(&feature);
        assert_eq!(deltas[0].op, DeltaOp::Added);
    }

    #[test]
    fn find_wip_detects_non_removed_wip() {
        let input = r#"
Feature: Test

  @wip
  Scenario: Not done yet
    Given x
    Then y
"#;
        let feature = parse_str(input).unwrap();
        let msg = find_wip_in_feature(&feature);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("Not done yet"));
    }

    #[test]
    fn find_wip_allows_wip_on_removed_placeholder() {
        let input = r#"
Feature: Test

  @givn.removed @wip
  Scenario: Old
    Given x
"#;
        let feature = parse_str(input).unwrap();
        assert!(find_wip_in_feature(&feature).is_none());
    }

    #[test]
    fn find_wip_clean_when_no_wip() {
        let input = r#"
Feature: Test

  Scenario: Complete
    Given x
    Then y
"#;
        let feature = parse_str(input).unwrap();
        assert!(find_wip_in_feature(&feature).is_none());
    }

    #[test]
    fn detect_retired_feature() {
        let feature =
            parse_str("@givn.delta @givn.retired @old-capability\nFeature: Old capability\n")
                .unwrap();
        assert!(is_retired_feature(&feature));
    }

    #[test]
    fn validate_capability_tag_rejects_traversal() {
        assert!(validate_capability_tag("../../etc").is_err());
        assert!(validate_capability_tag("foo/bar").is_err());
        assert!(validate_capability_tag("").is_err());
        assert!(validate_capability_tag("-leading").is_err());
    }

    #[test]
    fn validate_capability_tag_accepts_kebab_case() {
        assert!(validate_capability_tag("semantic-review").is_ok());
        assert!(validate_capability_tag("abc123").is_ok());
    }
}
