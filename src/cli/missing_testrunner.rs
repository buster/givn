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

During the first product change, choose a known-working Gherkin runner and
adapt it to the project. Common coverage-capable choices include:

  Rust / cucumber-rs: cargo llvm-cov --test features_runner
  JavaScript / Cucumber.js: npx nyc --reporter=text-summary npx cucumber-js
  Python / Behave: coverage run -m behave givn/specs && coverage report
  Java / Cucumber-JVM: mvn verify

Bare `cargo test` with a cucumber runner, Behave, pytest-bdd, or Cucumber.js can
establish the Gherkin loop before coverage is ready.

None of those command lines is sufficient by name alone. The project adapter
must execute the path in GIVN_FEATURES and write a fresh GIVN_RESULT_FILE with
actual `total`, `passed`, `failed`, and `skipped` scenario counts. For
GIVN_RESULT_SCOPE, `change` runs every delta scenario, including `@e2e`; regular
excludes `@e2e` from permanent specs; e2e selects only `@e2e` from permanent
specs. Givn cannot infer those facts from a command's name.

Coverage setup is separate: configure the coverage commands to instrument the
Gherkin runner and create the regular and E2E Cobertura reports required by the
coverage gate. Do not substitute a copied command line or a fabricated result
for this setup.

After editing givn/commands.yaml, re-run the command that invoked this sentinel.
See docs/reference.md#runner-contract and givn/commands.yaml."#
    );
    std::process::exit(1);
}
