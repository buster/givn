# Tasks: ise-rebuild-decision-and-working-tree-ship

The direct vocabulary and guidance work is committed before this plan (`bbd7de5`,
`4b30dfb`, `92ad0a5`, `8a7efce`). This change adds the regression scenarios that assert the
working-tree practice, its limitation, the copy distinction, the restart rule,
the No rebuild line, and the principle citation. Every scenario is regular and
asserts served CLI text; no `src/**` file changes.

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both set in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` on both cucumber
builders (`tests/features_runner.rs`, `e2e/tests/e2e_runner.rs`). No runner,
framework, or fixture infrastructure is added by this change.

Release note (subject of the behaviour batch commit, from `proposal.md`):

```text
The ise guidance records one Ise rebuild decision, states whether rebuilt code ships or is discarded, and lets a Single-Component Rebuild ship from the current project working tree without a spec copy.
```

**Commit rule.** One atomic `givn commit` with explicit paths per batch. The
behaviour batch uses the release note as its subject and `Batch:
working-tree-ship-guidance` in the body. The commit contains the delta
scenarios and their permanent step bindings; a comment-only, task-only, or
`@wip`-only change is not evidence. Every task of the batch records the same
revision as `COMMIT: <hash> -- Batch: working-tree-ship-guidance`.

**Placeholders.** Every `COMMIT:` line starts with `___` until the batch commit
exists. A checked line with a placeholder, a revision that does not exist, or a
revision not reachable from `HEAD` fails the review and archive gates.

**Evidence fields.** This file records no test-first order and no RED/GREEN
checkbox. A scenario whose requirement is already implemented is recorded as
existing behaviour protected and manufactures no production diff.

## Batch map

| Batch | Tasks | Scenarios | Commit | Purpose |
|---|---|---|---|---|
| `setup` | 3 | 0 | C1 | Plan commit, runner confirmation, strictness confirmation. |
| `working-tree-ship-guidance` | 7 | 7 | C2 | Add the seven regression scenarios and their step bindings. |
| `verification` | 4 | 0 | C3 | Delta run, full regular suite, e2e scope, `givn check tasks` receipt. |
| `review-and-archive` | 4 | 0 | C4, C5 | Review, archive, post-archive suites, self-dogfood. |
| **Total** | **18** | **7** | | |

## Setup

- [x] **C1 — Commit the plan.** `givn commit` the change directory (this file,
  `proposal.md`, `design.md`, `arc42.md`, `design-review.md`, `evidence/`).

  Design constraints: none.

  - [x] COMMIT: 320c9a1 -- Batch: setup

- [x] **C2 — Confirm the runner and strict mode.** `verify.command` is
  `./run-tests.sh --non-e2e`; `verify.e2e_command` is
  `./run-tests.sh --e2e --package e2e`. `tests/features_runner.rs` calls
  `.fail_on_skipped()`; a skipped or undefined regular step fails the run. The
  e2e package runner fails a skipped step. The full regular suite passes before
  the batch.

  Evidence: `cargo nextest run --test features_runner --no-fail-fast` with
  `GIVN_FEATURES=givn/specs GIVN_CUCUMBER_TAGS='not @e2e and not @givn.removed'`:
  `1117 tests run: 1117 passed, 115 skipped`.

  - [x] COMMIT: 320c9a1 -- Batch: setup

- [x] **C3 — Confirm the single-scenario command.** Run one delta scenario with
  `GIVN_FEATURES=givn/changes/ise-rebuild-decision-and-working-tree-ship/specs
  GIVN_CUCUMBER_TAGS='not @e2e and not @givn.removed' cargo nextest run --locked
  --test features_runner -E 'test("...")'`.

  - [x] COMMIT: 320c9a1 -- Batch: setup

## Batch `working-tree-ship-guidance`

Planned proof: the delta's seven scenarios fail before their step bindings exist
and pass after the bindings are added; the served text is already present from
the direct work, so no production diff is manufactured. The commit contains the
two delta feature files and the two permanent step files.

### Scenario: The ise instruction teaches the working-tree ship sequence

- [x] **Given** an ise-enabled project, **When** the consumer requests the `ise`
  instruction, **Then** the served instruction teaches the five steps in order
  and states the non-requirements.

  Design constraints: `ise.md` "Single-Component Rebuild with Rebuilt code:
  ship" names Write the active Specification, Keep the previous implementation
  in history, Remove the old implementation in the working tree, Rebuild in
  place, Verify, commit, archive. It states that no spec copy, candidate
  revision, early archive merge, generated spec-copy manifest, history-hiding
  repository view, or sandbox is required. D5.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

### Scenario: The ise instruction states the working-tree ship limitation

- [x] **Then** the served instruction states that a passing verification proves
  tested behaviour and does not prove the Specification alone was sufficient.

  Design constraints: `ise.md` "Limitation" paragraph. D6. No clean-room claim
  for the working-tree path.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

### Scenario: The ise instruction distinguishes the working-tree ship from copy preparation

- [x] **Then** the served instruction states that the working-tree ship rebuilds
  in the current project working tree and that the spec-copy commands prepare an
  external copy and never edit the working tree.

  Design constraints: `ise.md` "Two rebuild practices" and "Commands"; D5 keeps
  `givn ise rebuild-component` as an external-copy preparer.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

### Scenario: The ise instruction restarts verification when the Specification changes after a rebuild starts

- [x] **Then** the served instruction states that a changed Specification or E2E
  suite records a new frozen revision and reruns the relevant verification.

  Design constraints: `ise.md` limitation section; D5. The acceptance target is
  never moved to make the rebuilt code pass.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

### Scenario: The ise instruction cites the principle source

- [x] **Then** the served instruction cites `https://iseprinciple.org`.

  Design constraints: `ise.md` first paragraph; the site is framework-agnostic
  and names no Givn command, flag, or record.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

### Scenario: The design guidance names the working-tree ship practice

- [x] **Then** the served design guidance states that a Single-Component Rebuild
  with `Rebuilt code: ship` follows the working-tree sequence and names the `ise`
  instruction as the owner.

  Design constraints: `design.md` decision block; D5. The design instruction
  points at the `ise` instruction and does not restate the sequence.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

### Scenario: The design guidance omits the Rebuilt code line for No rebuild

- [x] **Then** the served design guidance states that a `No rebuild` change
  records no `Rebuilt code` line.

  Design constraints: `design.md` decision block; D1 and D8. A rebuilding change
  records `discard` or `ship`.

  - [x] COMMIT: 0e5c905 -- Batch: working-tree-ship-guidance

## Verification

- [x] **C3 — Delta run.** Run the two delta features through the regular scope;
  all seven scenarios pass. Record the summary.

  - [x] COMMIT: 8a7efce -- Batch: verification

- [x] **C4 — Full regular suite.** `./run-tests.sh --non-e2e` exits 0 with
  `1117 passed`. Record the summary.

  - [x] COMMIT: 8a7efce -- Batch: verification

- [x] **C5 — E2E scope.** `./run-tests.sh --e2e --package e2e` exits 0; the
  direct `@e2e` decision scenario passes. Record the summary.

  - [x] COMMIT: 8a7efce -- Batch: verification

- [x] **C6 — Change gate.** `givn check tasks --change
  ise-rebuild-decision-and-working-tree-ship` writes its receipt and exits 0.

  - [x] COMMIT: 8a7efce -- Batch: verification

## Review and archive

- [x] **C4 — Independent review.** The reviewer context `agent://DesignReviewIseVocab`
  performs the final review and writes `review.md` with `REVIEW: PASS`.

  - [x] COMMIT: 1c95772 -- Batch: review-and-archive

- [x] **C5 — Archive.** `givn archive --change
  ise-rebuild-decision-and-working-tree-ship` publishes the proven archive.

  - [x] COMMIT: ebede7 -- Batch: review-and-archive

- [x] **C5 — Post-archive suites.** Run the regular and e2e suites against the
  merged corpus; both exit 0.

  - [x] COMMIT: ebede7 -- Batch: review-and-archive

- [x] **C5 — Self-dogfood.** Install the checkout's binary, `givn skills sync`,
  `givn instructions`, and the required suites per `AGENTS.md`.

  - [x] COMMIT: ebede7 -- Batch: review-and-archive
