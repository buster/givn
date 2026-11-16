//! Compiled-CLI E2E steps for the 0.2.0-to-0.4.0 migration boundary.

use crate::harness as common;
use crate::steps::GivnWorld;
use cucumber::{then, when};

pub(crate) fn git_project_at_0_7_0() -> crate::harness::TestEnv {
    git_project_with_version("0.7.0")
}

fn git_project_with_version(version: &str) -> crate::harness::TestEnv {
    let env = crate::harness::make_bare_root();
    let init = crate::harness::run_givn_with_stdin(
        &env,
        &["init", "--no-addons", "--agents", "opencode"],
        "",
    );
    assert!(
        init.status.success(),
        "fixture init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let config_path = env.path("givn/config.yaml");
    let config = std::fs::read_to_string(&config_path).expect("read fixture config");
    std::fs::write(
        &config_path,
        config.replace(
            &format!(
                "givn_config_version: \"{}\"",
                crate::harness::binary::product_version()
            ),
            &format!("givn_config_version: \"{version}\""),
        ),
    )
    .expect("write legacy fixture config");

    let generated_skill = env.path(".opencode/skills/givn-spec/SKILL.md");
    let mut skill = std::fs::read_to_string(&generated_skill).expect("read generated skill");
    skill.push_str("\nlegacy generated marker\n");
    std::fs::write(generated_skill, skill).expect("mark generated skill as legacy");

    common::write_commands_yaml(
        &env.path("givn/commands.yaml"),
        "verify:\n  command: \"./emit-result.sh regular\"\n  e2e_command: \"\"\n",
    );
    std::fs::write(
        env.path(".gitignore"),
        ".opencode/\nAGENTS.md\ngivn/history/\n",
    )
    .expect("ignore generated agent files and the development record");

    git(&env, &["init"]);
    git(&env, &["config", "user.name", "Givn Migration Test"]);
    git(
        &env,
        &["config", "user.email", "givn-migration@example.test"],
    );
    git(&env, &["add", "--all"]);
    let commit = git(
        &env,
        &["commit", "-m", &format!("fixture: Givn {version} project")],
    );
    assert!(
        commit.status.success(),
        "fixture commit failed: {}",
        String::from_utf8_lossy(&commit.stderr)
    );

    env
}

#[then("the project config identifies the current Givn version")]
fn config_identifies_current_version(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("legacy project fixture");
    let config = std::fs::read_to_string(env.path("givn/config.yaml")).expect("read config");
    assert!(
        config.contains(&format!(
            "givn_config_version: \"{}\"",
            crate::harness::binary::product_version()
        )),
        "{config}"
    );
}

pub(crate) fn find_migration_change_id(env: &crate::harness::TestEnv) -> String {
    let mut entries: Vec<String> = std::fs::read_dir(env.path("givn/changes"))
        .expect("read active changes")
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("migrate-"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "expected exactly one migration change, got {entries:?}"
    );
    entries.pop().expect("exactly one entry")
}

fn git(env: &crate::harness::TestEnv, args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .args(args)
        .current_dir(env.root())
        .output()
        .expect("run git fixture command")
}

#[when("the consumer archives the maintenance migration")]
fn archive_maintenance_migration(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("completed migration fixture");
    crate::steps::archive_proof_support::write_result_runner(env, "pass");
    common::commit_fixture(env);
    let id = find_migration_change_id(env);
    crate::steps::archive_proof_support::refresh_change_receipt(world, &id);
    world.last_output = Some(crate::harness::run_givn(env, &["archive", "--change", &id]));
}

#[then("the permanent specification tree contains no migration feature")]
fn permanent_tree_has_no_migration_feature(world: &mut GivnWorld) {
    let env = world.env.as_ref().expect("completed migration fixture");
    let specs = env.path("givn/specs");
    let features = walk_feature_files(&specs);
    assert!(
        features
            .iter()
            .all(|path| !path.to_string_lossy().contains("migrate-0-2-0-to-0-4-0")),
        "migration feature appeared in permanent specs: {features:?}"
    );
}

fn walk_feature_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walk_feature_files(&path));
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("feature") {
                files.push(path);
            }
        }
    }
    files
}
