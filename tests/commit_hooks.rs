#[allow(dead_code)]
mod common;

use common::{TestEnv, git};
use std::{fs, process::Command};

fn git_ok(env: &TestEnv, args: &[&str]) -> Vec<u8> {
    let output = git(env, args);
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn git_fixture() -> TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    for path in ["batch.txt", "other.txt", "obsolete.txt"] {
        fs::write(env.path(path), "initial\n").unwrap();
    }
    common::git_commit_all(&env, "initial");
    env
}

fn pending(env: &TestEnv) -> Vec<u8> {
    git_ok(
        env,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
}

fn index(env: &TestEnv) -> Vec<u8> {
    git_ok(env, &["ls-files", "--stage", "-z"])
}

fn changed(env: &TestEnv) -> Vec<u8> {
    git_ok(
        env,
        &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"],
    )
}

fn assert_commit(env: &TestEnv, args: &[&str]) {
    let out = common::run_givn_with_envs(env, args, &[("GIVN_ATTRIBUTION_ACTIVE", "")]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn git_requires_paths_without_mutation() {
    let env = git_fixture();
    fs::write(env.path("batch.txt"), "modified").unwrap();
    fs::write(env.path("new.txt"), "new").unwrap();
    git_ok(&env, &["add", "other.txt"]);
    let before = (
        pending(&env),
        index(&env),
        git_ok(&env, &["rev-parse", "HEAD"]),
    );
    let out = common::run_givn(&env, &["commit", "-m", "must refuse"]);
    assert!(
        !out.status.success(),
        "implicit commit unexpectedly succeeded"
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("explicit paths"));
    assert_eq!(
        before,
        (
            pending(&env),
            index(&env),
            git_ok(&env, &["rev-parse", "HEAD"])
        )
    );
}

#[test]
fn git_selected_working_content_preserves_unrelated_index_operations() {
    for operation in ["modification", "addition", "deletion"] {
        let env = git_fixture();
        fs::write(env.path("batch.txt"), "old staged version").unwrap();
        git_ok(&env, &["add", "batch.txt"]);
        fs::write(env.path("batch.txt"), "selected working version").unwrap();
        match operation {
            "deletion" => {
                git_ok(&env, &["rm", "other.txt"]);
            }
            "addition" => {
                git_ok(&env, &["rm", "--cached", "other.txt"]);
                git_ok(&env, &["commit", "-m", "remove other from history"]);
                git_ok(&env, &["add", "other.txt"]);
            }
            _ => {
                fs::write(env.path("other.txt"), "staged other").unwrap();
                git_ok(&env, &["add", "other.txt"]);
                fs::write(env.path("other.txt"), "unstaged other").unwrap();
            }
        }
        let other_index = git_ok(&env, &["ls-files", "--stage", "--", "other.txt"]);
        let other_working = fs::read(env.path("other.txt")).ok();
        assert_commit(&env, &["commit", "batch.txt", "-m", "selected"]);
        assert_eq!(changed(&env), b"batch.txt\n", "{operation}");
        assert_eq!(
            git_ok(&env, &["show", "HEAD:batch.txt"]),
            b"selected working version"
        );
        assert_eq!(
            other_index,
            git_ok(&env, &["ls-files", "--stage", "--", "other.txt"])
        );
        assert_eq!(other_working, fs::read(env.path("other.txt")).ok());
    }
}

#[test]
fn git_selected_tracked_deletion_is_valid() {
    let env = git_fixture();
    fs::remove_file(env.path("obsolete.txt")).unwrap();
    assert_commit(&env, &["commit", "obsolete.txt", "-m", "delete"]);
    assert_eq!(changed(&env), b"obsolete.txt\n");
    assert!(!env.path("obsolete.txt").exists());
    assert!(
        !git(&env, &["cat-file", "-e", "HEAD:obsolete.txt"])
            .status
            .success()
    );
}

#[test]
fn git_unknown_selection_refuses_before_staging_valid_paths() {
    let env = git_fixture();
    fs::write(env.path("new.txt"), "new").unwrap();
    let before = (pending(&env), index(&env));
    let out = common::run_givn(&env, &["commit", "new.txt", "missing.txt", "-m", "invalid"]);
    assert!(!out.status.success());
    assert_eq!(before, (pending(&env), index(&env)));
}

#[test]
fn git_unmerged_index_refuses_before_staging() {
    let env = git_fixture();
    let base = git_ok(&env, &["rev-parse", "HEAD"]);
    git_ok(&env, &["checkout", "-b", "side"]);
    fs::write(env.path("other.txt"), "side").unwrap();
    common::git_commit_all(&env, "side");
    git_ok(
        &env,
        &[
            "checkout",
            "-b",
            "mainline",
            std::str::from_utf8(&base).unwrap().trim(),
        ],
    );
    fs::write(env.path("other.txt"), "mainline").unwrap();
    common::git_commit_all(&env, "mainline");
    assert!(!git(&env, &["merge", "side"]).status.success());
    fs::write(env.path("batch.txt"), "selected").unwrap();
    let before = (index(&env), pending(&env));
    let out = common::run_givn(&env, &["commit", "batch.txt", "-m", "invalid"]);
    assert!(!out.status.success());
    assert_eq!(before, (index(&env), pending(&env)));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unmerged"));
}

#[cfg(unix)]
fn executable(path: &std::path::Path, content: &str) {
    use std::os::unix::fs::PermissionsExt;
    fs::write(path, content).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

#[test]
#[cfg(unix)]
fn git_rejected_hook_preserves_index_and_head() {
    let env = git_fixture();
    executable(&env.path(".git/hooks/commit-msg"), "#!/bin/sh\nexit 1\n");
    fs::write(env.path("batch.txt"), "selected").unwrap();
    fs::write(env.path("other.txt"), "staged").unwrap();
    git_ok(&env, &["add", "other.txt"]);
    let before = (
        index(&env),
        pending(&env),
        git_ok(&env, &["rev-parse", "HEAD"]),
    );
    let out = common::run_givn(&env, &["commit", "batch.txt", "-m", "rejected"]);
    assert!(!out.status.success());
    assert_eq!(
        before,
        (
            index(&env),
            pending(&env),
            git_ok(&env, &["rev-parse", "HEAD"])
        )
    );
}

#[test]
fn fossil_requires_paths_and_preserves_pending_additions() {
    let env = common::make_fossil_root();
    fs::write(env.path("new.txt"), "new").unwrap();
    fs::write(env.path("pending.txt"), "pending").unwrap();
    assert!(
        common::fossil(&env, &["add", "pending.txt"])
            .status
            .success()
    );
    let before = common::fossil(&env, &["changes", "--differ"]).stdout;
    let revision = givn::fossil::FossilCheckout::at(&env.root())
        .revision()
        .unwrap();
    let out = common::run_givn(&env, &["commit", "-m", "must refuse"]);
    assert!(
        !out.status.success(),
        "implicit commit unexpectedly succeeded"
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("explicit paths"));
    assert_eq!(
        before,
        common::fossil(&env, &["changes", "--differ"]).stdout
    );
    assert_eq!(
        revision,
        givn::fossil::FossilCheckout::at(&env.root())
            .revision()
            .unwrap()
    );
}

#[test]
fn fossil_selected_deletion_and_addition_preserve_other_work() {
    for deletion in [false, true] {
        let env = common::make_fossil_root();
        fs::write(env.path("batch.txt"), "initial").unwrap();
        common::fossil_commit_all(&env, "initial selected");
        fs::write(env.path("pending.txt"), "pending").unwrap();
        assert!(
            common::fossil(&env, &["add", "pending.txt"])
                .status
                .success()
        );
        if deletion {
            fs::remove_file(env.path("batch.txt")).unwrap();
        } else {
            fs::write(env.path("batch.txt"), "selected").unwrap();
        }
        assert_commit(&env, &["commit", "batch.txt", "-m", "selected"]);
        let checkout = givn::fossil::FossilCheckout::at(&env.root());
        assert_eq!(
            checkout.revision_file("batch.txt").unwrap(),
            if deletion {
                None
            } else {
                Some("selected".into())
            }
        );
        assert_eq!(checkout.revision_file("pending.txt").unwrap(), None);
        assert!(
            String::from_utf8_lossy(&common::fossil(&env, &["changes", "--differ"]).stdout)
                .contains("ADDED      pending.txt")
        );
        assert_eq!(fs::read(env.path("pending.txt")).unwrap(), b"pending");
    }
}

#[test]
#[cfg(unix)]
fn repeated_hooks_path_install_preserves_user_hook_without_recursion() {
    for hooks_path in [".githooks", "./.githooks", ".git/hooks"] {
        let env = git_fixture();
        git_ok(&env, &["config", "core.hooksPath", hooks_path]);
        fs::create_dir_all(env.path(hooks_path)).unwrap();
        let hook = env.path(hooks_path).join("commit-msg");
        executable(&hook, "#!/bin/sh\nprintf 'user\\n' >> hook-marker\n");
        givn::cli::git::install(&env.root()).unwrap();
        givn::cli::git::install(&env.root()).unwrap();
        fs::write(env.path("batch.txt"), "human").unwrap();
        git_ok(&env, &["add", "batch.txt"]);
        let out = Command::new("timeout")
            .args(["5", "git", "commit", "-m", "human"])
            .current_dir(env.root())
            .env_remove("GIVN_ATTRIBUTION_ACTIVE")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{hooks_path}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(
            fs::read_to_string(env.path("hook-marker")).unwrap(),
            "user\n"
        );
        assert_eq!(git_ok(&env, &["log", "-1", "--format=%B"]), b"human\n\n");
        assert!(
            fs::read_to_string(hook.with_file_name("commit-msg.givn-original"))
                .unwrap()
                .contains("hook-marker")
        );
    }
}

#[test]
#[cfg(unix)]
fn missing_shared_helper_does_not_block_human_commit() {
    let env = git_fixture();
    executable(
        &env.path(".git/hooks/commit-msg"),
        "#!/bin/sh\nprintf 'user\\n' >> hook-marker\n",
    );
    givn::cli::git::install(&env.root()).unwrap();
    fs::remove_file(env.path(".githooks/commit-msg")).unwrap();
    fs::write(env.path("batch.txt"), "human").unwrap();
    git_ok(&env, &["add", "batch.txt"]);
    let out = git(&env, &["commit", "-m", "human"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        fs::read_to_string(env.path("hook-marker")).unwrap(),
        "user\n"
    );
}

#[test]
fn fossil_recursive_directory_selection_records_tracked_deletions() {
    let env = common::make_fossil_root();
    fs::create_dir_all(env.path("batch")).unwrap();
    fs::write(env.path("batch/obsolete.txt"), "old").unwrap();
    fs::write(env.path("batch/keep.txt"), "old").unwrap();
    common::fossil_commit_all(&env, "initial directory");
    fs::remove_file(env.path("batch/obsolete.txt")).unwrap();
    fs::write(env.path("batch/keep.txt"), "new").unwrap();
    assert_commit(&env, &["commit", "batch", "-m", "directory"]);
    let checkout = givn::fossil::FossilCheckout::at(&env.root());
    assert_eq!(checkout.revision_file("batch/obsolete.txt").unwrap(), None);
    assert_eq!(
        checkout.revision_file("batch/keep.txt").unwrap(),
        Some("new".into())
    );
}

#[test]
fn git_explicit_staged_deletion_is_valid() {
    let env = git_fixture();
    git_ok(&env, &["rm", "obsolete.txt"]);
    assert_commit(&env, &["commit", "obsolete.txt", "-m", "delete"]);
    assert_eq!(changed(&env), b"obsolete.txt\n");
}
