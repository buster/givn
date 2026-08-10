use crate::steps::GivnWorld;
use cucumber::then;

fn project_root(world: &GivnWorld) -> std::path::PathBuf {
    world
        .env
        .as_ref()
        .map(|env| env.path(""))
        .expect("a project root is required first")
}

fn last_stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no `givn` command has been run yet");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[then(regex = r##"^the ejected review template contains "(.+)"$"##)]
fn ejected_review_template_contains(world: &mut GivnWorld, needle: String) {
    let stdout = last_stdout(world);
    assert!(
        stdout.contains("givn/artifacts/review/review.md"),
        "eject output did not identify the review template path: {stdout}"
    );

    let path = project_root(world).join("givn/artifacts/review/review.md");
    let content = std::fs::read_to_string(&path).expect("read ejected review template");
    assert!(
        content.contains(&needle),
        "ejected review template missing {needle:?}: {content}"
    );
}

#[then(regex = r##"^the generated review guidance contains "(.+)"$"##)]
fn generated_review_guidance_contains(world: &mut GivnWorld, needle: String) {
    let root = project_root(world);
    let config = std::fs::read_to_string(root.join("givn/config.yaml")).unwrap_or_default();
    let mut targets: Vec<String> = vec!["opencode".to_string()];
    let mut in_skills = false;
    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed == "skills:" {
            in_skills = true;
            continue;
        }
        if in_skills {
            if !line.is_empty() && !line.starts_with(' ') {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("targets:") {
                targets = rest
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
    }

    for target in &targets {
        let paths = [
            format!(".{target}/skills/givn-review/SKILL.md"),
            ".opencode/commands/givn-review.md".to_string(),
        ];
        for relative in paths {
            let path = root.join(&relative);
            if !path.exists() {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap_or_else(|error| {
                panic!("read generated review guidance {relative}: {error}")
            });
            assert!(
                content.contains(&needle),
                "generated review guidance {relative} missing {needle:?}"
            );
        }
    }
}
