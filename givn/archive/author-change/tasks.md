# Tasks: author-change

Domain constraints (from `givn/changes/author-change/specs/author-change/usecase.md`):
- The SCM-neutral commit path is `givn commit`; Fossil attribution exists only through it; Git keeps the chained hook as a fallback for raw commits.
- Generated guidance may name only commands that work on the active backend, and states backend limitations explicitly.
- The installation report names what was installed, what was skipped, and why.
- Initialization without an SCM does not refuse; it warns (exit 0).
- Confirmed Personas: developer--primary-user, agent--technical-role, givn-maintainer--owner (review lenses).

## Setup

- [x] Runner contract and strict mode confirmed (fail_on_skipped): the first
      targeted run of the wrapper scenarios before the step bindings existed
      exited with undefined-step failures; the bindings were then implemented
      and the suite re-run.
- [x] E2E count proof for the change scope:

      ```text
      GIVN_FEATURES=givn/changes/author-change/specs ./run-tests.sh
      Summary: 7 tests run: 7 passed, 0 skipped
      GIVN_FEATURES=givn/changes/author-change/specs ./run-tests.sh --e2e
      Summary: 3 tests run: 3 passed, 4 skipped
      ```

## Scenario: The SCM-neutral commit path attributes a Git revision

- [x] RED: the wrapper bindings did not exist; the first change-scope run
      failed on undefined steps (non-zero).
- [x] GREEN: `src/cli/commit.rs` stages (`stage_all`/`stage_paths`) and
      commits the stamped message; the Git revision message carries the
      managed trailer. Targeted run green.
- [x] REFACTOR: clean up; run still green.
- [x] COMMIT: 1a756dfc with the Git-wrapper scope; the implementation slice
      covers the three wrapper scenarios' RED+GREEN.

## Scenario: The SCM-neutral commit path attributes a Fossil check-in

- [x] RED: shared with the wrapper RED.
- [x] GREEN: the Fossil branch records the pending change (`addremove`) and
      checks in with the stamped message; the check-in comment carries the
      trailer. Targeted run green.
- [x] REFACTOR: the Fossil comment read moved to the shared
      `fossil_latest_comment` helper.
- [x] COMMIT: f208b36a.

## Scenario: The commit wrapper without attribution context leaves the message unchanged

- [x] RED: shared with the wrapper RED.
- [x] GREEN: without `GIVN_ATTRIBUTION_ACTIVE` the message is committed as
      given; the When strips the harness-exported attribution variables
      explicitly (the outer suite may run inside a harness that exports
      them).
- [x] REFACTOR: the strip is the scenario's behavioral fix.
- [x] COMMIT: 78dbb0db.

## Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook

- [x] RED: shared with the change-scope RED.
- [x] GREEN: `install` asks the seam and prints the givn-commit path on
      Fossil without writing a hook; init exits 0. Targeted run green.
- [x] REFACTOR: clean up.
- [x] COMMIT: 1a756dfc (installation scope; shared implementation slice).

## Scenario: Consumer initializes a Fossil checkout and receives backend-aware guidance

- [x] RED: shared with the change-scope RED.
- [x] GREEN: the generated givn-implement skill contains the committing
      section (template note rendered at init's skills sync).
- [x] REFACTOR: clean up.
- [x] COMMIT: 1a756dfc (guidance scope; shared implementation slice).

## Scenario: Consumer initializes a plain directory and receives a named warning

- [x] RED: shared with the change-scope RED.
- [x] GREEN: init exits 0 with the two warning lines; a later
      `givn archive --change demo` refuses naming the missing
      version-controlled project.
- [x] REFACTOR: clean up.
- [x] COMMIT: 1a756dfc (no SCM scope; shared implementation slice).

## Scenario: Generated guidance names the commit path that works on every backend

- [x] RED: shared with the change-scope RED.
- [x] GREEN: the generated skill names `givn commit` and states the raw
      `fossil commit` limitation.
- [x] REFACTOR: clean up.
- [x] COMMIT: 1a756dfc (prose scope; shared implementation slice).

## Finalize

- [x] Full suite green: 479/479 and e2e 85/85; clippy clean; fmt clean.
- [x] `givn lint --change author-change` exits 0; the regenerated repository
      skills carry the committing note (synced and committed with the
      implementation).
- [x] All tasks checked with evidence and commit hashes; the scenarios whose
      production change is one atomic slice share the implementation commit
      and name their scope in the COMMIT line (A16).
