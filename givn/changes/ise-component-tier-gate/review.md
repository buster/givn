# Review: ise-component-tier-gate

Review kind: independent
Author: DeepSeek Flash execution agent
Reviewer context: DesignReviewGate (independent reviewer subagent)

PERSONAS: developer--primary-user

ARC42 CONFORMANCE: CLEAN

README-IMPACT: updated - Configuration

## Scope and review context

One independent review context performed both the design review and this final
review. It did not author the change and received no author conversation. It
reviewed the committed batch `ed411f57`, the tasks record `41271e57`, the
transcript commit `a9c846e6`, and the current worktree's documentation
corrections.

Artifacts read: `proposal.md`, `design.md`, `design-review.md`, `arc42.md`,
`tasks.md`, the three delta feature files, the delta use case, the permanent
`prepare-an-ise-test` use case, `src/ise/contract.rs`, `src/ise/verify.rs`,
`src/ise/compare.rs`, `src/ise/manifest.rs`, `src/ise/mod.rs`,
`src/config/manifest.rs`, `src/cli/ise.rs`, `src/main.rs`,
`tests/ise_component_tier_gate.rs`, the three modified step files, the served
`ise` instruction, `README.md`, `docs/reference.md`, the affected arc42
chapters, and `docs/glossary.md`.

The change is major: it changes the public `givn ise` command surface, adds a
persistent result record, and touches three capabilities.

## Implementation evidence check

- **`@e2e` tag integrity.** The delta files carry `@givn.added` on seven
  scenarios and no `@e2e` tag was added or removed. The existing `@e2e`
  comparison scenario remains the real-interface entry for the verifier
  action; the trial gate is a variant of that action.
- **Step bodies.** No empty or trivial bodies in
  `tests/steps/trial_input_verification_steps.rs`,
  `tests/steps/prose_consolidation_steps.rs`, or
  `tests/steps/ise_clean_room_support.rs`. Every new body drives the built
  binary or asserts on its stdout, JSON, or YAML.
- **Scenario commits.** All seven delta scenarios record
  `COMMIT: ed411f57664b96b5bd33e4355b62c97b001ca5ff -- Batch:
  component-tier-gate`. That commit contains the implementation, the step
  bindings, the acceptance test, and the spec/docs changes.
- **Strict mode.** `tests/features_runner.rs` keeps `.fail_on_skipped()`.
  The change adds no runner and no skip path.
- **Acceptance test.** `cargo test --test ise_component_tier_gate --
  --test-threads=1` — 8 passed, including the missing runner, exact counts,
  mismatch, not-run, read-only compare, reserved path, traversal, and symlink
  cases.
- **Terminal evidence.** The real `givn ise compare` and
  `givn ise verify-trial` transcript is committed at
  `givn/changes/ise-component-tier-gate/evidence/visual/verification-records-the-exact-declared-counts/transcript.txt`.
  It shows `Trial result: unverified` for input equality and the ordered
  component, End-to-end, `Trial result: passed` lines for the clean trial.
- **Receipt.** `givn check tasks --change ise-component-tier-gate` produced a
  receipt for 7 scenarios, 0 failed, 0 skipped; the review reuses it across the
  documentation-only revisions.

## Overlap dispositions

`givn lint --change ise-component-tier-gate` reported one `[SHAPE]`
shape-match advisory and `[SUBST]` subset advisories; no delta scenario exceeds
the split threshold.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Verification refuses an exact count mismatch | A not-run decision cannot replace declared component units | keep: a present result with the wrong count and an explicit not-run bypass are distinct refusals with distinct recorded reasons |

No other delta scenario shares a shape, and no delta scenario exceeds the
14-step split threshold; the longest has 9 steps.

## Verification gates

| Gate | Result |
|---|---|
| `givn lint --change ise-component-tier-gate` | exit 0; 3 files checked, clean; only `[SUBST]` advisories |
| `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` | exit 0; 7 tests, 7 passed, 42 steps |
| `givn check tasks --change ise-component-tier-gate` | exit 0; 7 scenario(s), 0 failed, 0 skipped; E2E `not_applicable`; `verify passed` |
| `./run-tests.sh --non-e2e` | exit 0; 1126 tests, 1126 passed, 115 nextest units excluded by the regular tag expression |
| `./run-tests.sh --e2e --package e2e` | exit 0; 56 features, 120 scenarios (120 passed), 785 steps |
| `GIVN_FEATURES=givn/specs/prepare-an-ise-test ./run-tests.sh --e2e --package e2e` | exit 0; 2 scenarios, 16 steps |
| `givn check review --change ise-component-tier-gate` | exit 0 after this record; overlap dispositions, scenario commits (7), and the receipt pass |

The E2E run regenerated six archived `development-timeline` screenshots; all
six were restored before this record was committed.

## Findings and dispositions

No blocking finding remains.

| Finding | Disposition |
|---|---|
| F1: reserved contract path was checked only against the copy set | closed: `src/ise/mod.rs` checks every revision entry; regression `a_revision_tracking_the_reserved_contract_path_refuses` |
| F2: `--result ../copy/...` escaped the containment check | closed: `super::resolved_target` resolves the deepest existing ancestor; regression `a_result_path_escaping_into_the_copy_refuses` |
| F3: docs named `component_executed/component_total` instead of the serialized fields | closed: docs now name `component.declared`/`component.executed` and `end_to_end.declared`/`end_to_end.executed` |
| F4: a broken symlink at `--result` escaped into the copy | closed: `symlink_metadata` refuses a final-component symlink; regression `a_symlink_result_path_refuses` |
| F5: success guarantee named three outcomes | closed: collapsed to one `Trial outcome` paragraph |
| F6: design lacked the Interaction Coverage Matrix | closed: three-row matrix added for every inventory entry |
| F7: new risks reused R-61/R-62 | closed: original rows restored; new risks are R-166/R-167 |
| F8: visual transcript was a Cucumber runner log | closed: replaced with real `givn ise compare` and `givn ise verify-trial` output |
| F9: test-runner section omitted the E2E command | closed: exact E2E and ISE E2E commands recorded |
| F10: full E2E preflight was red on missing browser dependencies | closed: `npm ci --prefix e2e/browser` and Chromium install; full E2E 120/120 |

## Review checks

- [x] No fabricated step body; every command is exercised through the real binary.
- [x] Every checked scenario names a shared batch revision that contains its implementation.
- [x] `@e2e` tags were not removed to evade verification.
- [x] The delta features are bound in the permanent test tree; no delta step copy exists.
- [x] The interaction inventory maps every entry to a matrix row and an existing `@e2e` scenario.
- [x] The design's contracts and components have one implementing component each.
- [x] The `Ise rebuild decision` is `No rebuild` and records no `Rebuilt code` line.
- [x] The visual contract is terminal, has a committed real transcript, and names `developer--primary-user`.
- [x] arc42 chapters 2, 3, 4, 5, 6, 8, 9, and 11 are updated; chapter 9 records the ADR-076 amendment.
- [x] README impact is `updated - Configuration`.
- [x] The receipt is fresh and reused across documentation-only revisions.
- [x] The full regular and E2E preflights are green.

VISUAL-REVIEW: PASS

REVIEW: PASS
