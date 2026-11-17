use crate::steps::{GivnWorld, output};
use cucumber::{given, then};
fn env(world: &GivnWorld) -> &crate::harness::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

fn corpus_root(world: &mut GivnWorld) -> &crate::harness::TestEnv {
    if world.env.is_none() {
        world.env = Some(crate::harness::make_init_root());
    }
    let env = world.env.as_ref().expect("fixture project");
    let specs = env.path("givn/specs");
    if specs.exists() {
        std::fs::remove_dir_all(&specs).expect("reset specs");
    }
    std::fs::create_dir_all(&specs).expect("create specs");
    env
}

#[derive(Clone, Copy)]
struct HealthCapability<'a> {
    id: &'a str,
    e2e: bool,
}

fn usecase_document(
    id: &str,
    capabilities: &[HealthCapability<'_>],
    includes: &str,
    extends: &str,
) -> String {
    let capability_list = if capabilities.is_empty() {
        "- none".to_string()
    } else {
        capabilities
            .iter()
            .map(|capability| format!("- {}", capability.id))
            .collect::<Vec<_>>()
            .join(
                "
",
            )
    };
    let interaction_rows = if capabilities.is_empty() {
        "- none".to_string()
    } else {
        let rows: Vec<String> = capabilities
            .iter()
            .map(|capability| {
                format!(
                    "| {} | exercise {} | {} scenario |",
                    capability.id, capability.id, capability.id
                )
            })
            .collect();
        format!(
            "| Capability | Consumer action | E2E scenario |
|---|---|---|
{}",
            rows.join(
                "
"
            )
        )
    };
    format!(
        "# Use case: {id}

         ## Level

!

         ## Actors

- Fixture operator (the first actor is the primary actor)

         ## Goal

Exercise the fixture behavior.

         ## Trigger

The operator asks.

         ## Preconditions

- A project exists.

         ## Main flow

1. The operator acts; System responsibility: respond.
2. The operator waits; System responsibility: respond.

         ## Extensions

- 1: failure → refuse; protects success

         ## Rules

- Run once.

         ## Examples

- A clean run.

         ## Minimal guarantee

The project remains inspectable.

         ## Success guarantee

The result is shown.

         ## Personas

- none

         ## Capabilities

{capability_list}

         ## Interactions

{interaction_rows}

         ## Includes

{includes}

         ## Extends

{extends}

         ## Out of scope

Nothing.

         ## Diagram

```mermaid
flowchart LR
  A --> B
```
"
    )
}

fn fragment_document(id: &str, capabilities: &[HealthCapability<'_>]) -> String {
    let capability_list = capabilities
        .iter()
        .map(|capability| format!("- {}", capability.id))
        .collect::<Vec<_>>()
        .join(
            "
",
        );
    let rows = capabilities
        .iter()
        .map(|capability| {
            format!(
                "| {} | exercise {} | fragment {} scenario |",
                capability.id, capability.id, capability.id
            )
        })
        .collect::<Vec<_>>()
        .join(
            "
",
        );
    format!(
        "# Fragment: {id}

         ## Capabilities

{capability_list}

         ## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
{rows}

         ## Diagram

```mermaid
flowchart LR
  A --> B
```
"
    )
}

fn write_feature(
    env: &crate::harness::TestEnv,
    relative: &str,
    capability: &HealthCapability<'_>,
    prefix: Option<&str>,
) {
    let label = prefix
        .map(|prefix| format!("{prefix} {}", capability.id))
        .unwrap_or_else(|| capability.id.to_string());
    let e2e = if capability.e2e {
        format!("\n  @e2e\n  Scenario: {label} scenario\n    Given a thing\n    Then it works\n")
    } else {
        String::new()
    };
    let content = format!(
        "Feature: {}\n\n  Scenario: Internal {label} scenario\n    Given a thing\n    Then it works\n{e2e}",
        capability.id
    );
    let path = env.root().join(relative);
    std::fs::create_dir_all(path.parent().expect("feature parent")).expect("create feature dir");
    std::fs::write(path, content).expect("write feature");
}

fn write_usecase(
    env: &crate::harness::TestEnv,
    id: &str,
    capabilities: &[HealthCapability<'_>],
    includes: &str,
    extends: &str,
) {
    let dir = env.root().join("givn/specs").join(id);
    std::fs::create_dir_all(&dir).expect("create usecase dir");
    std::fs::write(
        dir.join("usecase.md"),
        usecase_document(id, capabilities, includes, extends),
    )
    .expect("write usecase.md");
    for capability in capabilities {
        write_feature(
            env,
            &format!("givn/specs/{id}/{}.feature", capability.id),
            capability,
            None,
        );
    }
}

fn write_fragment(env: &crate::harness::TestEnv, capabilities: &[HealthCapability<'_>]) {
    let dir = env.root().join("givn/specs/fragments");
    std::fs::create_dir_all(&dir).expect("create fragments dir");
    std::fs::write(
        dir.join("fragment.md"),
        fragment_document("support", capabilities),
    )
    .expect("write fragment.md");
    for capability in capabilities {
        write_feature(
            env,
            &format!("givn/specs/fragments/{}.feature", capability.id),
            capability,
            Some("fragment"),
        );
    }
}

fn combined_output(world: &GivnWorld) -> String {
    let out = output(world);
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

#[given("a structurally valid specification corpus with one clean feature")]
fn clean_corpus(world: &mut GivnWorld) {
    let env = corpus_root(world);
    write_usecase(
        env,
        "clean",
        &[HealthCapability {
            id: "clean",
            e2e: true,
        }],
        "- none",
        "- none",
    );
}

#[given(
    "a structurally valid corpus with a use case named \"support\" and a fragment named \"support\""
)]
fn corpus_with_use_case_and_fragment_support(world: &mut GivnWorld) {
    let env = corpus_root(world);
    write_usecase(
        env,
        "support",
        &[HealthCapability {
            id: "support",
            e2e: true,
        }],
        "- none",
        "- none",
    );
    write_fragment(
        env,
        &[HealthCapability {
            id: "support",
            e2e: true,
        }],
    );
}

#[given(regex = r#"^another use case Includes "([^"]+)"$"#)]
fn another_use_case_includes(world: &mut GivnWorld, target: String) {
    let env = env(world);
    write_usecase(
        env,
        "consumer",
        &[HealthCapability {
            id: "consumer",
            e2e: true,
        }],
        &format!("- {target}"),
        "- none",
    );
}

#[given(
    regex = r#"^a structurally valid corpus containing only a (use case|fragment) named "([^"]+)"$"#
)]
fn corpus_with_only_owner(world: &mut GivnWorld, owner: String, id: String) {
    let env = corpus_root(world);
    if owner == "fragment" {
        write_fragment(
            env,
            &[HealthCapability {
                id: "support",
                e2e: true,
            }],
        );
    } else {
        write_usecase(
            env,
            &id,
            &[HealthCapability {
                id: "support",
                e2e: true,
            }],
            "- none",
            "- none",
        );
    }
}

#[given(regex = r#"^another use case Extends "([^"]+)"$"#)]
fn another_use_case_extends(world: &mut GivnWorld, target: String) {
    let env = env(world);
    write_usecase(
        env,
        "consumer",
        &[HealthCapability {
            id: "consumer",
            e2e: true,
        }],
        "- none",
        &format!("- {target}"),
    );
}

#[given(regex = r#"^structurally valid use cases "([^"]+)" and "([^"]+)"$"#)]
fn two_use_cases(world: &mut GivnWorld, first: String, second: String) {
    let env = corpus_root(world);
    for id in [first, second] {
        write_usecase(
            env,
            &id,
            &[HealthCapability {
                id: "support",
                e2e: true,
            }],
            "- none",
            "- none",
        );
    }
}

fn rewrite_relationship(env: &crate::harness::TestEnv, id: &str, heading: &str, target: &str) {
    let path = env.root().join(format!("givn/specs/{id}/usecase.md"));
    let content = std::fs::read_to_string(&path).expect("read usecase");
    let marker = format!(
        "## {heading}

- none"
    );
    assert!(content.contains(&marker), "fixture lacks {heading}");
    std::fs::write(
        path,
        content.replace(
            &marker,
            &format!(
                "## {heading}

- {target}"
            ),
        ),
    )
    .expect("write relationship");
}

#[given(regex = r#"^"([^"]+)" (Includes|Extends) "([^"]+)"$"#)]
#[given(regex = r#"^the use case "([^"]+)" (Includes|Extends) "([^"]+)"$"#)]
fn use_case_relationship(world: &mut GivnWorld, id: String, relationship: String, target: String) {
    rewrite_relationship(env(world), &id, &relationship, &target);
}

#[given("structurally valid use cases \"first\" and \"support\" and a fragment named \"support\"")]
fn first_support_and_fragment(world: &mut GivnWorld) {
    let env = corpus_root(world);
    for id in ["first", "support"] {
        write_usecase(
            env,
            id,
            &[HealthCapability {
                id: "support",
                e2e: true,
            }],
            "- none",
            "- none",
        );
    }
    write_fragment(
        env,
        &[HealthCapability {
            id: "support",
            e2e: true,
        }],
    );
}

#[given(
    regex = r#"^a structurally valid (use case|fragment) declaring capabilities "([^"]+)" and "([^"]+)"$"#
)]
fn owner_with_two_capabilities(
    world: &mut GivnWorld,
    owner: String,
    first: String,
    second: String,
) {
    let env = corpus_root(world);
    if owner == "fragment" {
        let dir = env.root().join("givn/specs/fragments");
        std::fs::create_dir_all(&dir).expect("create fragments dir");
        let document = format!(
            "# Fragment: support

## Capabilities

- {first}
- {second}

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| {first} | submit | Consumer submits an order |
| {second} | cancel | Consumer cancels an order |

## Diagram

```mermaid
flowchart LR
  A --> B
```
"
        );
        std::fs::write(dir.join("fragment.md"), document).expect("write fragment.md");
        write_feature(
            env,
            &format!("givn/specs/fragments/{first}.feature"),
            &HealthCapability {
                id: &first,
                e2e: false,
            },
            Some("fragment"),
        );
        write_feature(
            env,
            &format!("givn/specs/fragments/{second}.feature"),
            &HealthCapability {
                id: &second,
                e2e: true,
            },
            Some("fragment"),
        );
    } else {
        let dir = env.root().join("givn/specs/orders");
        std::fs::create_dir_all(&dir).expect("create usecase dir");
        let document = usecase_document(
            "orders",
            &[
                HealthCapability {
                    id: &first,
                    e2e: false,
                },
                HealthCapability {
                    id: &second,
                    e2e: true,
                },
            ],
            "- none",
            "- none",
        )
        .replace(
            &format!("| {first} | exercise {first} | {first} scenario |"),
            &format!("| {first} | submit | Consumer submits an order |"),
        )
        .replace(
            &format!("| {second} | exercise {second} | {second} scenario |"),
            &format!("| {second} | cancel | Consumer cancels an order |"),
        );
        std::fs::write(dir.join("usecase.md"), document).expect("write usecase.md");
        write_feature(
            env,
            &format!("givn/specs/orders/{first}.feature"),
            &HealthCapability {
                id: &first,
                e2e: false,
            },
            None,
        );
        write_feature(
            env,
            &format!("givn/specs/orders/{second}.feature"),
            &HealthCapability {
                id: &second,
                e2e: true,
            },
            None,
        );
    }
}

#[given(regex = r#"^its interaction inventory maps "([^"]+)" to "([^"]+)"$"#)]
fn interaction_inventory_maps(_world: &mut GivnWorld, _capability: String, _scenario: String) {
    // The capability fixture already encodes the two interaction rows.
}

#[given(regex = r#"^only "([^"]+)" has an E2E scenario titled "([^"]+)"$"#)]
fn only_capability_has_titled_e2e(world: &mut GivnWorld, capability: String, title: String) {
    // Replace the cancel-order scenario title with the place-order title so
    // the evidence exists in the wrong capability only.
    for path in [
        format!("givn/specs/orders/{capability}.feature"),
        format!("givn/specs/fragments/{capability}.feature"),
    ] {
        let path = env(world).root().join(&path);
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("read capability feature");
            let rewritten = content
                .replace(&format!("{capability} scenario"), &title)
                .replace(
                    &format!("Scenario: Internal {capability} scenario"),
                    &format!("Scenario: Internal {capability}"),
                );
            std::fs::write(&path, rewritten).expect("write capability feature");
        }
    }
}

#[given("a structurally valid fragment declaring capability \"show-help\"")]
fn fragment_show_help(world: &mut GivnWorld) {
    let env = corpus_root(world);
    let dir = env.root().join("givn/specs/fragments");
    std::fs::create_dir_all(&dir).expect("create fragments dir");
    std::fs::write(
        dir.join("fragment.md"),
        "# Fragment: support

## Capabilities

- show-help

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| show-help | ask | Consumer requests help |

## Diagram

```mermaid
flowchart LR
  A --> B
```
",
    )
    .expect("write fragment.md");
    std::fs::write(
        dir.join("show-help.feature"),
        "Feature: show-help

  @e2e
  Scenario: Consumer requests help
    Given a thing
    Then it works
",
    )
    .expect("write show-help feature");
}

#[given(regex = r#"^"([^"]+)" has an E2E scenario titled "([^"]+)"$"#)]
fn capability_has_e2e(world: &mut GivnWorld, capability: String, title: String) {
    let env = env(world);
    for path in [
        format!("givn/specs/fragments/{capability}.feature"),
        format!("givn/specs/orders/{capability}.feature"),
    ] {
        let path = env.root().join(&path);
        if path.exists() {
            let content = std::fs::read_to_string(&path).expect("read capability feature");
            let rewritten = content
                .replace(&format!("{capability} scenario"), &title)
                .replace(
                    &format!("Scenario: Internal {capability} scenario"),
                    &format!("Scenario: Internal {capability}"),
                );
            std::fs::write(&path, rewritten).expect("write capability feature");
        }
    }
}

#[given("two use cases include that fragment without copying its interaction inventory")]
fn two_use_cases_include_fragment(world: &mut GivnWorld) {
    let env = env(world);
    for id in ["caller-a", "caller-b"] {
        write_usecase(
            env,
            id,
            &[HealthCapability {
                id: "caller",
                e2e: true,
            }],
            "- fragment: support",
            "- none",
        );
    }
}

#[given(regex = r#"^an otherwise valid use-case document whose first heading is "([^"]+)"$"#)]
fn malformed_title_document(world: &mut GivnWorld, heading: String) {
    let env = corpus_root(world);
    write_usecase(
        env,
        "support",
        &[HealthCapability {
            id: "support",
            e2e: true,
        }],
        "- none",
        "- none",
    );
    let path = env.root().join("givn/specs/support/usecase.md");
    let content = std::fs::read_to_string(&path).expect("read usecase");
    let body = content
        .split_once('\n')
        .map(|(_, rest)| rest)
        .unwrap_or_default();
    std::fs::write(
        path,
        format!(
            "{heading}
{body}"
        ),
    )
    .expect("write malformed title");
}

#[then("no ambiguous relationship target finding is reported")]
fn no_ambiguous_target(world: &mut GivnWorld) {
    let text = combined_output(world);
    assert!(!text.to_lowercase().contains("ambiguous"), "{text}");
}

#[then(regex = r#"^the relationship resolves to the (use case|fragment) named "([^"]+)"$"#)]
fn relationship_resolves(world: &mut GivnWorld, _owner: String, _id: String) {
    assert_eq!(
        output(world).status.code(),
        Some(0),
        "{}",
        combined_output(world)
    );
    assert!(!combined_output(world).contains("does not resolve"));
}

#[then(regex = r#"^the findings name the unresolved typed target "([^"]+)"$"#)]
fn unresolved_typed_target(world: &mut GivnWorld, target: String) {
    let text = combined_output(world);
    assert!(text.contains(&target), "{text}");
    assert!(text.contains("does not resolve"), "{text}");
}

#[then("the findings say that Extends cannot target a fragment")]
fn extends_fragment_finding(world: &mut GivnWorld) {
    assert!(
        combined_output(world).contains("Extends cannot target a fragment"),
        "{}",
        combined_output(world)
    );
}

#[then(regex = r#"^a use-case cycle finding names "([^"]+)" and "([^"]+)"$"#)]
fn cycle_finding(world: &mut GivnWorld, first: String, second: String) {
    let text = combined_output(world);
    assert!(text.contains("cycle"), "{text}");
    assert!(text.contains(&first) && text.contains(&second), "{text}");
}

#[then("no use-case cycle finding is reported")]
fn no_cycle(world: &mut GivnWorld) {
    assert!(
        !combined_output(world).contains("cycle"),
        "{}",
        combined_output(world)
    );
}

#[then(regex = r#"^the findings report missing E2E evidence for "([^"]+)" and "([^"]+)"$"#)]
fn missing_e2e_evidence(world: &mut GivnWorld, capability: String, scenario: String) {
    let text = combined_output(world);
    assert!(
        text.contains(&format!("capability '{capability}'")),
        "{text}"
    );
    assert!(text.contains("missing @e2e scenario"), "{text}");
    assert!(text.contains(&scenario), "{text}");
}

#[then(regex = r#"^the scenario in "([^"]+)" does not satisfy that interaction$"#)]
fn wrong_capability_scenario(world: &mut GivnWorld, other: String) {
    let text = combined_output(world);
    assert!(text.contains(&other), "{text}");
    let place_order = text
        .lines()
        .find(|line| line.contains("capability 'place-order'"))
        .expect("place-order finding");
    assert!(place_order.contains("0 @e2e scenario(s)"), "{text}");
    assert!(place_order.contains("missing @e2e scenario"), "{text}");
}

#[then(regex = r#"^no missing or unlisted interaction finding is reported for "([^"]+)"$"#)]
fn no_interaction_finding(world: &mut GivnWorld, scenario: String) {
    let text = combined_output(world);
    assert!(!text.contains("interaction inventory mismatch"), "{text}");
    assert!(!text.contains("unlisted @e2e scenario"), "{text}");
    assert!(!text.contains("undeclared capability"), "{text}");
    assert!(!text.contains("interaction"), "{text}");
    let _ = scenario;
}

#[then("the findings identify the malformed use-case title and its document")]
fn malformed_title_finding(world: &mut GivnWorld) {
    let text = combined_output(world);
    assert!(text.contains("usecase.md"), "{text}");
    assert!(
        text.contains("expected '# Use case:")
            || text.contains("use-case title")
            || text.contains("Use case ID"),
        "{text}"
    );
}

#[then("the command exits 3")]
fn exits_3(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(3),
        "expected exit code 3, stderr: {}",
        String::from_utf8_lossy(&output(world).stderr)
    );
}

#[then("stdout is valid JSON")]
fn stdout_valid_json(world: &mut GivnWorld) {
    let stdout = String::from_utf8_lossy(&output(world).stdout);
    serde_json::from_str::<serde_json::Value>(&stdout)
        .unwrap_or_else(|e| panic!("stdout is not valid JSON: {e}\nstdout: {stdout}"));
}
