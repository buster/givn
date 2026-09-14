# Tasks: author-change

Domain constraints (from `givn/changes/author-change/specs/author-change/usecase.md`):
- The SCM-neutral commit path is `givn commit`; Fossil attribution exists only through it; Git keeps the chained hook as a fallback for raw commits.
- Generated guidance may name only commands that work on the active backend, and states backend limitations explicitly.
- The installation report names what was installed, what was skipped, and why.
- Initialization without an SCM does not refuse; it warns (exit 0).
- Confirmed Personas: developer--primary-user, agent--technical-role, givn-maintainer--owner (review lenses).

## Setup

- [ ] Runner contract and strict mode confirmed (fail_on_skipped; RED runs of
      the new scenarios must exit non-zero).
- [ ] E2E count proof for the change scope (full vs `@e2e`).

## Scenario: The SCM-neutral commit path attributes a Git revision

- [ ] RED: the wrapper When/Then bindings start as `unimplemented!()`.
- [ ] GREEN: `givn commit` stages and commits with the stamped message; the
      revision message carries the managed trailer.
- [ ] REFACTOR: clean up; run still green.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: The SCM-neutral commit path attributes a Fossil check-in

- [ ] RED: shared with the first wrapper RED.
- [ ] GREEN: the Fossil branch records the pending change and checks in with
      the stamped message.
- [ ] REFACTOR: clean up.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: The commit wrapper without attribution context leaves the message unchanged

- [ ] RED: shared.
- [ ] GREEN: without `GIVN_ATTRIBUTION_ACTIVE` the message is committed as
      given.
- [ ] REFACTOR: clean up.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook

- [ ] RED: the Fossil-checkout Given and the report/no-hook Thens start as
      `unimplemented!()`.
- [ ] GREEN: `install` prints the givn-commit path on Fossil and writes no
      hook; init exits 0.
- [ ] REFACTOR: clean up.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Consumer initializes a Fossil checkout and receives backend-aware guidance

- [ ] RED: the guidance Then starts as `unimplemented!()`.
- [ ] GREEN: the generated givn-implement skill contains the `givn commit`
      section.
- [ ] REFACTOR: clean up.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Consumer initializes a plain directory and receives a named warning

- [ ] RED: the warning Then and the archive-refusal Then start as
      `unimplemented!()`.
- [ ] GREEN: init exits 0 with the warning; a later archive refuses by name.
- [ ] REFACTOR: clean up.
- [ ] COMMIT: hash recorded: `<hash>`

## Scenario: Generated guidance names the commit path that works on every backend

- [ ] RED: the guidance-content Thens start as `unimplemented!()`.
- [ ] GREEN: the template note renders into the generated skill.
- [ ] REFACTOR: clean up.
- [ ] COMMIT: hash recorded: `<hash>`

## Finalize

- [ ] Full suite green; clippy and fmt clean; `givn lint --change
      author-change` exits 0.
- [ ] All tasks checked with evidence and commit hashes.
