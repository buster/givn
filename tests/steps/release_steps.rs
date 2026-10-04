//! Step bindings for the `describe-the-release` capability: changelog
//! generation with the workspace `cliff.toml` and the release guards
//! (change `release-truth`).
//!
//! The fixture is a real temporary git repository; the generator is the real
//! pinned `git-cliff` binary resolved through `PATH`, the same interface the
//! release pipeline uses.

use std::process::{Command, ExitStatus};

use cucumber::{given, then, when};

use super::GivnWorld;
use crate::common;

/// Per-scenario state for the release-truth scenarios.
#[derive(Debug, Default)]
pub(crate) struct ReleaseState {
    fixture: Option<ChangelogFixture>,
    /// Group heading to the entry sentence the fixture recorded for it.
    expected_groups: Vec<(String, String)>,
    generated: Option<Generated>,
}

#[derive(Debug)]
pub(crate) struct ChangelogFixture {
    env: common::TestEnv,
    base: String,
}

#[derive(Debug)]
pub(crate) struct Generated {
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    pub(crate) status: ExitStatus,
}

/// Create a temporary git repository with a deterministic identity, no host
/// hooks, and one recorded base revision.
pub(crate) fn make_fixture() -> ChangelogFixture {
    let env = common::make_bare_root();
    common::git_init(&env);
    let hooks = env.path("empty-hooks");
    std::fs::create_dir_all(&hooks).expect("create the empty hooks directory");
    let configured = common::git(
        &env,
        &["config", "core.hooksPath", &hooks.to_string_lossy()],
    );
    assert!(
        configured.status.success(),
        "git config core.hooksPath failed: {}",
        String::from_utf8_lossy(&configured.stderr)
    );
    let head = common::git(&env, &["rev-parse", "HEAD"]);
    assert!(
        head.status.success(),
        "git rev-parse HEAD failed: {}",
        String::from_utf8_lossy(&head.stderr)
    );
    let base = String::from_utf8_lossy(&head.stdout).trim().to_string();
    ChangelogFixture { env, base }
}

impl ChangelogFixture {
    /// Record one empty revision carrying `subject`.
    pub(crate) fn commit(&self, subject: &str) {
        let out = common::git(&self.env, &["commit", "-q", "--allow-empty", "-m", subject]);
        assert!(
            out.status.success(),
            "git commit failed for {subject:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// The revision range the maintainer generates over.
    pub(crate) fn range(&self) -> String {
        format!("{}..HEAD", self.base)
    }

    /// Generate the changelog with the workspace configuration.
    pub(crate) fn generate(&self) -> Generated {
        let config = common::repo_path("cliff.toml")
            .to_string_lossy()
            .into_owned();
        let range = self.range();
        let out = Command::new("git-cliff")
            .args(["--config", &config, &range])
            .current_dir(self.env.root())
            .output()
            .expect(
                "spawn git-cliff (install it with: cargo install git-cliff --version 2.13.1 --locked)",
            );
        Generated {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            status: out.status,
        }
    }
}

/// The body of one `### <group>` block, or `None` when the group is absent.
pub(crate) fn section_body(changelog: &str, group: &str) -> Option<String> {
    let heading = format!("### {group}");
    let mut body = String::new();
    let mut in_section = false;
    for line in changelog.lines() {
        let trimmed = line.trim();
        if trimmed == heading {
            in_section = true;
            continue;
        }
        if in_section && (trimmed.starts_with("### ") || trimmed.starts_with("## ")) {
            break;
        }
        if in_section {
            body.push_str(line);
            body.push('\n');
        }
    }
    in_section.then_some(body)
}

fn generated(world: &GivnWorld) -> &Generated {
    world
        .release
        .generated
        .as_ref()
        .expect("the changelog has not been generated yet")
}

#[given(
    "a release range with a feature, a bug fix, a performance change, a revert, and a breaking change"
)]
fn given_visible_range(world: &mut GivnWorld) {
    let fixture = make_fixture();
    fixture.commit("feat(alpha): A feature sentence");
    fixture.commit("fix(beta): A bug fix sentence");
    fixture.commit("perf(gamma): A performance sentence");
    fixture.commit("revert(delta): A revert sentence");
    fixture.commit("feat(epsilon)!: A breaking feature sentence");
    world.release.expected_groups = vec![
        (
            "Breaking Changes".to_string(),
            "A breaking feature sentence".to_string(),
        ),
        ("Features".to_string(), "A feature sentence".to_string()),
        ("Bug Fixes".to_string(), "A bug fix sentence".to_string()),
        (
            "Performance".to_string(),
            "A performance sentence".to_string(),
        ),
        ("Reverts".to_string(), "A revert sentence".to_string()),
    ];
    world.release.fixture = Some(fixture);
}

#[given("a test-harness bug fix and an uncategorised revision in the same range")]
fn given_internal_and_uncategorised(world: &mut GivnWorld) {
    let fixture = world
        .release
        .fixture
        .as_ref()
        .expect("a release range is required first");
    fixture.commit("fix(harness): Retarget the fixture scaffold");
    fixture.commit("misc: An uncategorised revision");
}

#[when("the maintainer generates the changelog for the release range")]
fn when_generate(world: &mut GivnWorld) {
    let fixture = world
        .release
        .fixture
        .as_ref()
        .expect("a release range is required first");
    let generated = fixture.generate();
    assert!(
        generated.status.success(),
        "git-cliff failed: {}",
        generated.stderr
    );
    world.release.generated = Some(generated);
}

#[then(
    expr = "the changelog lists the {string}, {string}, {string}, {string}, and {string} groups"
)]
fn then_group_entries(
    world: &mut GivnWorld,
    first: String,
    second: String,
    third: String,
    fourth: String,
    fifth: String,
) {
    let changelog = generated(world).stdout.clone();
    let expected = world.release.expected_groups.clone();
    for name in [first, second, third, fourth, fifth] {
        let body = section_body(&changelog, &name)
            .unwrap_or_else(|| panic!("the changelog has no {name} group:\n{changelog}"));
        let sentence = expected
            .iter()
            .find(|(group, _)| group == &name)
            .map(|(_, sentence)| sentence.clone())
            .unwrap_or_else(|| panic!("the fixture records no sentence for the {name} group"));
        assert!(
            body.contains(&sentence),
            "the {name} group does not list {sentence:?}:\n{body}"
        );
    }
}

#[then(expr = "the changelog lists no {string} group")]
fn then_no_group(world: &mut GivnWorld, group: String) {
    let changelog = generated(world).stdout.clone();
    assert!(
        section_body(&changelog, &group).is_none(),
        "the changelog lists an unexpected {group} group:\n{changelog}"
    );
}

#[then("the release section lists no test-harness bug fix")]
fn then_no_test_harness_fix(world: &mut GivnWorld) {
    let changelog = generated(world).stdout.clone();
    assert!(
        !changelog.contains("Retarget the fixture scaffold"),
        "the release section lists the test-harness bug fix:\n{changelog}"
    );
}
