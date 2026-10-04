//! Step bindings for the `describe-the-release` capability: changelog
//! generation with the workspace `cliff.toml` and the release guards
//! (change `release-truth`).
//!
//! The fixture is a real temporary git repository; the generator is the real
//! pinned `git-cliff` binary resolved through `PATH`, the same interface the
//! release pipeline uses.

use std::process::{Command, ExitStatus, Output};

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
    /// Repository the release guards run against.
    guard_repo: Option<common::TestEnv>,
    /// The version the maintainer declares current.
    expected_version: Option<String>,
    /// Captured output of the last release-guard invocation.
    guard_output: Option<Output>,
    /// Directory holding the generated changelog file under test.
    changelog_dir: Option<common::TestEnv>,
    /// The changelog file the section guard reads.
    changelog_file: Option<std::path::PathBuf>,
    /// The release version whose section the guard checks.
    section_version: Option<String>,
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

/// Run the checked-in release guard script from the fixture root.
pub(crate) fn run_guard(args: &[&str], cwd: &std::path::Path) -> Output {
    Command::new("sh")
        .arg(common::repo_path("scripts/release-guard.sh"))
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run scripts/release-guard.sh through sh")
}

/// A temporary repository the guards run against.
pub(crate) fn make_guard_repo() -> common::TestEnv {
    let env = common::make_bare_root();
    common::git_init(&env);
    env
}

/// Install the changelog fixture for the current scenario.
pub(crate) fn install_fixture(world: &mut GivnWorld, fixture: ChangelogFixture) {
    world.release.fixture = Some(fixture);
}

/// Record one revision on the current changelog fixture.
pub(crate) fn fixture_commit(world: &GivnWorld, subject: &str) {
    world
        .release
        .fixture
        .as_ref()
        .expect("a release range is required first")
        .commit(subject);
}

/// Generate the changelog for the current fixture and keep the output.
pub(crate) fn generate_changelog(world: &mut GivnWorld) {
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

/// The stdout of the last generated changelog.
pub(crate) fn changelog_stdout(world: &GivnWorld) -> String {
    generated(world).stdout.clone()
}

fn guard_output(world: &GivnWorld) -> &Output {
    world
        .release
        .guard_output
        .as_ref()
        .expect("the release guard has not been run yet")
}

fn record_annotated_tag(world: &GivnWorld, tag: &str) {
    let env = world
        .release
        .guard_repo
        .as_ref()
        .expect("a release repository is required first");
    let out = common::git(
        env,
        &[
            "tag",
            "--annotate",
            tag,
            "--message",
            &format!("Release {tag}"),
        ],
    );
    assert!(
        out.status.success(),
        "git tag failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn state_guard_repo(world: &GivnWorld) -> (&common::TestEnv, &str) {
    let env = world
        .release
        .guard_repo
        .as_ref()
        .expect("a release repository is required first");
    let version = world
        .release
        .expected_version
        .as_deref()
        .expect("a current version is required first");
    (env, version)
}

#[given(expr = "a repository whose current version is {string}")]
fn given_guard_repository(world: &mut GivnWorld, version: String) {
    world.release.guard_repo = Some(make_guard_repo());
    world.release.expected_version = Some(version);
}

#[given(expr = "whose newest version tag is {string}")]
fn given_newest_version_tag(world: &mut GivnWorld, tag: String) {
    record_annotated_tag(world, &tag);
}

#[when("the maintainer verifies the release base")]
fn when_verify_release_base(world: &mut GivnWorld) {
    let (env, version) = state_guard_repo(world);
    let version = version.to_string();
    let root = env.root();
    world.release.guard_output = Some(run_guard(&["base", &version], &root));
}

#[then(expr = "the check fails naming the missing tag {string}")]
fn then_check_fails_naming_tag(world: &mut GivnWorld, tag: String) {
    let out = guard_output(world);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.status.success(),
        "the release base check passed unexpectedly:\n{text}"
    );
    assert!(
        text.contains(&tag),
        "the failure does not name the missing tag {tag:?}:\n{text}"
    );
}

#[then(expr = "the check accepts the repository once the newest version tag is {string}")]
fn then_check_accepts_with_tag(world: &mut GivnWorld, tag: String) {
    record_annotated_tag(world, &tag);
    let (env, version) = state_guard_repo(world);
    let version = version.to_string();
    let root = env.root();
    let out = run_guard(&["base", &version], &root);
    assert!(
        out.status.success(),
        "the release base check failed after the tag was recorded:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
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

#[given(
    "a release range with only documentation, refactoring, test, chore, and uncategorised revisions"
)]
fn given_skipped_range(world: &mut GivnWorld) {
    let fixture = make_fixture();
    fixture.commit("docs(alpha): A documentation revision");
    fixture.commit("refactor(beta): A refactoring revision");
    fixture.commit("test(gamma): A test revision");
    fixture.commit("chore(delta): A chore revision");
    fixture.commit("misc: An uncategorised revision");
    world.release.expected_groups.clear();
    world.release.fixture = Some(fixture);
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

#[then("the release section lists no change group")]
fn then_no_change_group(world: &mut GivnWorld) {
    let changelog = generated(world).stdout.clone();
    let groups: Vec<&str> = changelog
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("### "))
        .collect();
    assert!(
        groups.is_empty(),
        "the release section lists change groups {groups:?}:\n{changelog}"
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

#[given(expr = "a generated changelog whose release section for {string} holds no change group")]
fn given_sectionless_changelog(world: &mut GivnWorld, version: String) {
    let env = common::make_bare_root();
    let path = env.path("CHANGELOG.md");
    std::fs::write(
        &path,
        format!("# Changelog\n\n## [Unreleased]\n\n## [{version}] - 2026-10-04\n"),
    )
    .expect("write the generated changelog fixture");
    world.release.changelog_dir = Some(env);
    world.release.changelog_file = Some(path);
    world.release.section_version = Some(version);
}

#[when(expr = "the maintainer verifies the release section for {string}")]
fn when_verify_release_section(world: &mut GivnWorld, version: String) {
    let file = world
        .release
        .changelog_file
        .clone()
        .expect("a generated changelog is required first");
    let cwd = world
        .release
        .changelog_dir
        .as_ref()
        .expect("a generated changelog is required first")
        .root();
    world.release.guard_output = Some(run_guard(
        &["section", &version, &file.to_string_lossy()],
        &cwd,
    ));
}

#[then(expr = "the check fails naming the release version {string}")]
fn then_section_check_fails(world: &mut GivnWorld, version: String) {
    let out = guard_output(world);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.status.success(),
        "the release section check passed unexpectedly:\n{text}"
    );
    assert!(
        text.contains(&version),
        "the failure does not name the release version {version:?}:\n{text}"
    );
}

#[then("the check accepts a release section holding a change group")]
fn then_section_check_accepts(world: &mut GivnWorld) {
    let file = world
        .release
        .changelog_file
        .clone()
        .expect("a generated changelog is required first");
    let version = world
        .release
        .section_version
        .clone()
        .expect("a release version is required first");
    let mut content = std::fs::read_to_string(&file).expect("read the changelog fixture");
    content.push_str("\n### Features\n\n- **scope:** A user-visible change\n");
    std::fs::write(&file, content).expect("write the changelog fixture with a change group");
    let cwd = world
        .release
        .changelog_dir
        .as_ref()
        .expect("a generated changelog is required first")
        .root();
    let out = run_guard(&["section", &version, &file.to_string_lossy()], &cwd);
    assert!(
        out.status.success(),
        "the release section check failed for a section holding a change group:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
