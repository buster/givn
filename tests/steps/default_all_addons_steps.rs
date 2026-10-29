//! Steps for the all-addons-enabled default.

use crate::steps::GivnWorld;
use cucumber::when;

#[when(regex = r#"^the user writes the addons block "(.+)"$"#)]
fn user_writes_addons_block(world: &mut GivnWorld, block: String) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&path).expect("read givn/config.yaml");
    let block = block.replace("\\n", "\n");

    // Remove the existing top-level addons block, then append the caller's
    // block. This keeps the rest of the project-owned config intact.
    let mut kept = Vec::new();
    let mut in_addons = false;
    for line in config.lines() {
        let trimmed = line.trim();
        if trimmed == "addons:" {
            in_addons = true;
            continue;
        }
        if in_addons {
            if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('#') {
                in_addons = false;
                kept.push(line.to_string());
            }
            continue;
        }
        kept.push(line.to_string());
    }
    while kept.last().is_some_and(|line| line.trim().is_empty()) {
        kept.pop();
    }
    kept.push(block);
    std::fs::write(path, kept.join("\n") + "\n").expect("write givn/config.yaml");
}
