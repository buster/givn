//! Step-definition skeleton for the instruction-integrity capability.
//!
//! RED scenarios intentionally remain undefined until each scenario enters its
//! own RED -> GREEN -> REFACTOR cycle.

use crate::steps::GivnWorld;
use cucumber::then;

fn cli_stdout(world: &GivnWorld) -> String {
    let output = world
        .last_output
        .as_ref()
        .expect("no CLI output was captured");
    assert!(
        output.status.success(),
        "CLI command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn env(world: &GivnWorld) -> &crate::harness::TestEnv {
    world.env.as_ref().expect("project fixture is missing")
}

#[then("no separately generated characterization command is installed")]
fn no_characterization_command(world: &mut GivnWorld) {
    let root = env(world).root();
    for target in AGENT_TARGET_DIRS {
        for command in ["givn-characterize", "givn-implement"] {
            let path = root
                .join(target)
                .join("commands")
                .join(format!("{command}.md"));
            assert!(
                !path.exists(),
                "a generated command was installed at {}",
                path.display()
            );
        }
    }
}

#[then("no separately authored agent command is generated")]
#[then("no separately authored agent commands are generated")]
fn no_separately_authored_agent_command(world: &mut GivnWorld) {
    no_characterization_command(world);
    let root = env(world).root();
    for target in AGENT_TARGET_DIRS {
        let dir = root.join(target).join("commands");
        let legacy = std::fs::read_dir(&dir)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        assert!(
            legacy.is_empty(),
            "generated command files remain: {legacy:?}"
        );
    }
}

#[then("the startup guidance says to load givn-start for new or unclassified work")]
fn startup_guidance_names_start(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("givn-start") && output.contains("new or unclassified work"),
        "startup guidance does not route new or unclassified work to givn-start: {output}"
    );
}

#[then("the startup guidance names givn new as the direct alternative")]
fn startup_guidance_names_direct_new(world: &mut GivnWorld) {
    let output = cli_stdout(world);
    assert!(
        output.contains("givn new") && output.contains("direct alternative"),
        "startup guidance does not name givn new as the direct alternative: {output}"
    );
}

/// The dot-directories the agent targets own.
const AGENT_TARGET_DIRS: [&str; 5] = [".opencode", ".agents", ".claude", ".omp", ".maki"];
