//! E2E step definitions for the README enforcement capability.
//!
//! The real interface is the compiled `givn` binary invoked as a subprocess
//! against a temp project root. Shared fixtures live in the unit-level
//! `readme_enforcement_steps` module; this file holds the e2e-exclusive
//! fixture and the scaffold/archive assertions.

use cucumber::{given, then};

use super::GivnWorld;
use super::readme_enforcement_steps;

#[given("a givn project providing its own workflow scripts")]
fn workflow_scripts_project(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let scripts = [
        (
            "run-tests.sh",
            "#!/bin/sh\nexit 0\n",
        ),
        (
            "measure-coverage.sh",
            "#!/bin/sh\nmkdir -p coverage\nprintf '<?xml version=\"1.0\"?>\n<coverage line-rate=\"0.80\" branch-rate=\"0\" lines-covered=\"80\" lines-valid=\"100\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\">\n  <packages />\n</coverage>\n' > coverage/non-e2e-cobertura.xml\nprintf '<?xml version=\"1.0\"?>\n<coverage line-rate=\"0.90\" branch-rate=\"0\" lines-covered=\"90\" lines-valid=\"100\" branches-covered=\"0\" branches-valid=\"0\" timestamp=\"0\" version=\"1\">\n  <packages />\n</coverage>\n' > coverage/e2e-cobertura.xml\nexit 0\n",
        ),
        (
            "merge-coverages.sh",
            "#!/bin/sh\ncat coverage/non-e2e-cobertura.xml coverage/e2e-cobertura.xml > coverage/cobertura-coverage.xml\nexit 0\n",
        ),
    ];
    for (name, body) in scripts {
        let path = env.path(name);
        std::fs::write(&path, body).expect("write workflow script");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                .expect("chmod workflow script");
        }
    }
}

#[then("the README contains an open placeholder for the description")]
fn readme_has_description_placeholder(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme =
        std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        readme.contains("<!-- givn:todo: One-paragraph description"),
        "README must carry the description open placeholder:\n{readme}"
    );
}

#[then("the README contains an open placeholder for the license")]
fn readme_has_license_placeholder(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme =
        std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        readme.contains("<!-- givn:todo: The project's license"),
        "README must carry the license open placeholder:\n{readme}"
    );
}

#[then("the README contains the development managed block naming the configured workflow commands")]
fn readme_has_dev_block(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme =
        std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    assert!(
        readme.contains("<!-- givn:begin:development -->")
            && readme.contains("<!-- givn:end:development -->"),
        "README must contain the development managed block:\n{readme}"
    );
    assert!(
        readme.contains("./run-tests.sh"),
        "development managed block must name the configured workflow commands:\n{readme}"
    );
}

#[then("the development managed block lists the current configured workflow commands")]
fn dev_block_lists_commands(world: &mut GivnWorld) {
    let env = world
        .env
        .as_ref()
        .expect("a project root is required first");
    let readme =
        std::fs::read_to_string(env.path("README.md")).expect("read README.md");
    let start = readme
        .find("<!-- givn:begin:development -->")
        .expect("development managed block begin marker");
    let end = readme
        .find("<!-- givn:end:development -->")
        .expect("development managed block end marker");
    let block = &readme[start..end];
    assert!(
        block.contains("./run-tests.sh") && block.contains("./measure-coverage.sh"),
        "refreshed block must list the configured workflow commands:\n{block}"
    );
    assert!(
        !block.contains("stale-command-listing"),
        "refreshed block must not keep the stale listing:\n{block}"
    );
}

#[then(regex = r#"^the instructions mention the unfinished README and \`givn readme check\`$"#)]
fn instructions_mention_readme(world: &mut GivnWorld) {
    let out = super::output(world);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("givn readme check"),
        "instructions must name `givn readme check`: {stdout}"
    );
    assert!(
        stdout.contains("README"),
        "instructions must surface the unfinished README: {stdout}"
    );
}

// Silence the unused-import warning until the fixture references land.
#[allow(unused_imports)]
use readme_enforcement_steps as _readme_steps;
