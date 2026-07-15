/// Sentinel default `verify.command`.
///
/// Prints setup guidance and exits non-zero so that nothing ever falsely
/// reports GREEN before a real Gherkin/Cucumber runner is configured.
pub fn run() -> anyhow::Result<()> {
    eprintln!(
        r#"givn: no test runner configured.

The .feature files under givn/specs/ are the executable specification.
They are valid Gherkin/Cucumber files and MUST be executed by a
Gherkin/Cucumber runner. A bare unit-test runner (cargo test alone, pytest
alone, go test alone) does NOT execute .feature files and is NOT sufficient
as verify.command.

Existing unit tests and integration tests are fine — keep them. But they are
NOT in scope for givn. givn's scope is the .feature files: they are the spec
you document, the cucumber test you run, and (after archive) the permanent
record. verify.command MUST execute them.

Set `verify.command` in givn/config.yaml to a runner that executes the
.feature files and produces COVERAGE DATA, e.g.:

  # Rust / cucumber-rs (line/branch coverage via llvm-cov)
  verify:
    command: "cargo llvm-cov --test features_runner --test e2e_runner --summary-only"

  # Rust / cucumber-rs (test only — no coverage data, not recommended)
  # verify:
  #   command: "cargo test --test features_runner"

  # JavaScript / Cucumber.js (coverage via nyc)
  verify:
    command: 'npx nyc --reporter=text-summary npx cucumber-js "givn/specs/**/*.feature"'

  # Python / Behave (coverage via coverage.py)
  verify:
    command: "coverage run -m behave givn/specs && coverage report"

  # Java / Cucumber-JVM (coverage via JaCoCo in verify phase)
  verify:
    command: "mvn verify"

Coverage requirement: when measuring test coverage, configure the coverage
tool to instrument EVERY test binary — unit tests, integration tests, AND
the Gherkin/Cucumber runner that executes the .feature files. The coverage
number must reflect the combined execution of all test kinds. A coverage run
that omits the .feature execution is invalid.

After editing givn/config.yaml, re-run the command that invoked this sentinel.
See givn/config.yaml (key: verify.command) and arc42 ch.8.10 for details."#
    );
    std::process::exit(1);
}
