use gherkin::{Background, Feature, Scenario, Step};
use serde_json::{Value, json};

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
