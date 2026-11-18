# Tasks: make-ise-guidance-self-contained

The change edits the served `ise` instruction and adds two regression scenarios. It changes no command, flag, exit code, persisted format, report label, metric, or gate.

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both set in `givn/commands.yaml`. Strict mode is `.fail_on_skipped()` on both cucumber builders. No runner or fixture infrastructure is added.

Release note (subject of the behaviour batch commit, from `proposal.md`):

```text
The served ise instruction defines its rebuild vocabulary inline, so an ise-enabled consumer needs no Givn-repository reference, and two scenarios pin the ise-enabled reference check and the ise-off guidance sweep.
```

**Commit rule.** One atomic `givn commit` with explicit paths per batch. The behaviour batch uses the release note as its subject and `Batch: self-contained-guidance` in the body. The commit contains the served instruction edit, the delta scenarios, and their permanent step bindings. Every scenario records the same revision as `COMMIT: <hash> -- Batch: self-contained-guidance`.

**Placeholders.** Every `COMMIT:` line starts with `___` until the batch commit exists. A checked line with a placeholder, a revision that does not exist, or a revision not reachable from `HEAD` fails the review and archive gates.

## Batch map

| Batch | Tasks | Scenarios | Commit | Purpose |
|---|---|---|---|---|
| `setup` | 3 | 0 | C1 | Plan commit, runner confirmation, strictness confirmation. |
| `self-contained-guidance` | 2 | 2 | C2 | Inline the terms and add the reference-check and off-path sweep scenarios. |
| `verification` | 4 | 0 | C3 | Delta run, full regular suite, e2e scope, `givn check tasks` receipt. |
| `review-and-archive` | 4 | 0 | C4, C5 | Review, archive, post-archive suites, self-dogfood. |
| **Total** | **13** | **2** | | |

## Setup

- [ ] **C1 — Commit the plan.** `givn commit` the change directory (this file, `proposal.md`, `design.md`, `arc42.md`, `design-review.md`).

  Design constraints: none.

  - [ ] COMMIT: ___ -- Batch: setup

- [ ] **C2 — Confirm the runner and strict mode.** `verify.command` is `./run-tests.sh --non-e2e`; `verify.e2e_command` is `./run-tests.sh --e2e --package e2e`. `.fail_on_skipped()` is present on both runners.

  - [ ] COMMIT: ___ -- Batch: setup

- [ ] **C3 — Confirm the single-scenario command.** Run one delta scenario with the design's `GIVN_FEATURES=... cargo nextest run --locked --test features_runner -E 'test(...)'`.

  - [ ] COMMIT: ___ -- Batch: setup

## Batch `self-contained-guidance`

Planned proof: with the old `docs/reference.md` delegation restored, `The ise instruction's file references resolve in a consumer project` fails at `givn instructions ise:16 — docs/reference.md`; with the inline definitions it passes. The off-path sweep passes because the conditional blocks are intact. The batch commit contains the served instruction edit, the delta feature, and the permanent step file.

### Scenario: The ise instruction's file references resolve in a consumer project

- [ ] **Given** an ise-enabled project with the reference check available, **When** the consumer requests the `ise` instruction, **Then** every file reference in the served text resolves in the consumer project.

  Design constraints: `ise.md` "Vocabulary" defines `spec copy`, `spec-copy manifest`, `checksum`, `Rebuild impact record`, and `Replaced unit` inline. The checker is `tests/common/reference_check.rs`; the served text is the label. D1, D3.

  - [ ] COMMIT: ___ -- Batch: self-contained-guidance

### Scenario: Printed guidance omits the opt-in ise rules

- [ ] **Given** an ise-off project with an active change and the reference check available, **When** the author requests every printed instruction, **Then** `Ise rebuild decision`, `Rebuilt code`, `Spec gap`, `Rebuild impact record`, `Specification Manifest`, and `Ise Test` are absent.

  Design constraints: the conditional `{{#if addons.ise}}` blocks must stay off; the corpus includes the startup guidance and the graph artifacts. D2.

  - [ ] COMMIT: ___ -- Batch: self-contained-guidance

## Verification

- [ ] **C3 — Delta run.** Run the two delta scenarios; both pass.

  - [ ] COMMIT: ___ -- Batch: verification

- [ ] **C4 — Full regular suite.** `./run-tests.sh --non-e2e` exits 0; record the summary.

  - [ ] COMMIT: ___ -- Batch: verification

- [ ] **C5 — E2E scope.** `./run-tests.sh --e2e --package e2e` exits 0; record the summary.

  - [ ] COMMIT: ___ -- Batch: verification

- [ ] **C6 — Change gate.** `givn check tasks --change make-ise-guidance-self-contained` writes its receipt and exits 0.

  - [ ] COMMIT: ___ -- Batch: verification

## Review and archive

- [ ] **C4 — Self review.** Perform the final review in this context and write `review.md` with `REVIEW: PASS` and the persona disposition.

  - [ ] COMMIT: ___ -- Batch: review-and-archive

- [ ] **C5 — Archive.** `givn archive --change make-ise-guidance-self-contained` publishes the proven archive.

  - [ ] COMMIT: ___ -- Batch: review-and-archive

- [ ] **C5 — Post-archive suites.** Run the regular and e2e suites against the merged corpus; both exit 0.

  - [ ] COMMIT: ___ -- Batch: review-and-archive

- [ ] **C5 — Self-dogfood.** Install the checkout's binary, `givn skills sync`, `givn instructions`, and the required suites per `AGENTS.md`.

  - [ ] COMMIT: ___ -- Batch: review-and-archive
