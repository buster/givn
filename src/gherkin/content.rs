use gherkin::{Background, Feature, Scenario, Step};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn tags<'a>(tags: impl Iterator<Item = &'a String>) -> Vec<&'a str> {
    let mut tags: Vec<_> = tags
        .map(String::as_str)
        .filter(|tag| !tag.starts_with("givn.") && *tag != "wip")
        .collect();
    tags.sort_unstable();
    tags.dedup();
    tags
}

fn steps(steps: &[Step]) -> Value {
    json!(
        steps
            .iter()
            .map(|step| {
                json!([
                    step.keyword.trim(),
                    format!("{:?}", step.ty),
                    step.value,
                    step.docstring,
                    step.table.as_ref().map(|table| &table.rows)
                ])
            })
            .collect::<Vec<_>>()
    )
}

pub fn background_content(background: Option<&Background>) -> Value {
    background.map_or(Value::Null, |background| steps(&background.steps))
}

pub fn executable_content(feature: &Feature, scenario: &Scenario) -> String {
    json!([
        scenario.keyword,
        background_content(feature.background.as_ref()),
        tags(feature.tags.iter().chain(&scenario.tags)),
        steps(&scenario.steps),
        scenario
            .examples
            .iter()
            .map(|examples| json!([
                tags(examples.tags.iter()),
                examples.table.as_ref().map(|table| &table.rows)
            ]))
            .collect::<Vec<_>>()
    ])
    .to_string()
}

/// The digest of the scenarios tagged `@e2e` in one feature file, or `None`
/// when the file selects none.
///
/// Layout: `sha256(feature_path NUL title NUL executable_content NUL ...)`
/// over the selected scenarios in file order. A feature-level `@e2e` tag
/// applies to every scenario, as it does for tag filtering; scenarios nested
/// under `Rule:` are outside the selection.
pub fn e2e_scenario_digest(feature_path: &str, source: &str) -> anyhow::Result<Option<String>> {
    let feature = Feature::parse(source, gherkin::GherkinEnv::default())
        .map_err(|error| anyhow::anyhow!("parse the feature: {error}"))?;
    let feature_tagged = feature.tags.iter().any(|tag| tag == "e2e");
    let selected: Vec<&Scenario> = feature
        .scenarios
        .iter()
        .filter(|scenario| feature_tagged || scenario.tags.iter().any(|tag| tag == "e2e"))
        .collect();
    if selected.is_empty() {
        return Ok(None);
    }
    let mut hasher = Sha256::new();
    hasher.update(feature_path.as_bytes());
    hasher.update(b"\0");
    for scenario in selected {
        hasher.update(scenario.name.as_bytes());
        hasher.update(b"\0");
        hasher.update(executable_content(&feature, scenario).as_bytes());
        hasher.update(b"\0");
    }
    Ok(Some(
        hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
    ))
}
