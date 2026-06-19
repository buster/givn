/// Sentinel default `verify.command`.
///
/// Prints per-language setup guidance and exits non-zero so that nothing
/// ever falsely reports GREEN before a real test runner is configured.
pub fn run() -> anyhow::Result<()> {
    eprintln!(
        r#"givn: no test runner configured.

Set `verify.command` in givn/config.yaml to your project's test runner, e.g.:

  # Rust / Cargo
  verify:
    command: "cargo test"

  # Java / Maven
  verify:
    command: "mvn test"

  # JavaScript / Cucumber.js
  verify:
    command: 'npx cucumber-js "givn/specs/**/*.feature"'

  # Python / Behave
  verify:
    command: "behave"

  # Go
  verify:
    command: "go test ./..."

  # Python / pytest-bdd
  verify:
    command: "pytest"

After editing givn/config.yaml, re-run the command that invoked this sentinel.
See givn/config.yaml (key: verify.command) and arc42 ch.8.10 for details."#
    );
    std::process::exit(1);
}
