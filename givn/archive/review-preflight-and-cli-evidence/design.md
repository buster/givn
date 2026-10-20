# Design: review-preflight-and-cli-evidence

## Decisions

- **D1: keep the existing evidence paths.** CLI transcripts live under the
  change's `evidence/` tree; the existing predicate exempts it. No new category
  and no migration.
- **D2: one real revision per scenario, with no one-pass mode.** A checked
  scenario must name one unique commit. `givn check tasks` and `givn check
  review` verify that the commit exists, is reachable from HEAD, and its
  message body contains the scenario title. Missing, duplicate, placeholder,
  unreachable, or untitled revisions block. A shared revision is always an
  error; there is no disclosure that makes it legal.
- **D3: no human visual sign-off for line-oriented CLI output.** A change whose
  design declares no visual contract is exempt. `VISUAL-REVIEW: NOT
  APPLICABLE` is accepted for that change; a declared Web UI or rendered
  terminal still requires `VISUAL-REVIEW: PASS`.
- **D4: the preflight is blocking before review and advisory for the README
  decision.** `givn check tasks` runs the scenario-commit check and prints a
  missing README-impact finding. `givn check review` blocks on the same commit
  check and on the README-impact integrity rule.

## Architecture impact

| File | Change |
|---|---|
| `src/scm.rs`, `src/git.rs`, `src/fossil.rs` | `commit_exists`, `commit_message`, `is_ancestor`. |
| `src/cli/check.rs` | Scenario parser and strict commit check; called by check tasks and review. |
| `assets/templates/tasks.md` | Hard rule: one scenario, one real revision, message names the scenario. |
| `assets/instructions/tasks.md` | The same rule in the served task instruction. |

No new dependency.

## Test design

Delta specs: `specs/verify-change/givn-workflow-enforcement.feature`
(four commit-check scenarios), `visual-review-gate.feature`, and
`specs/archive-change/archive-proof.feature`. Steps live in
`tests/steps/workflow_enforcement_steps.rs` and `archive_proof_steps.rs`.
Runner: `./run-tests.sh --non-e2e`; single scenario uses `GIVN_FEATURES`.
Strict mode: `.fail_on_skipped()`.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable

## Coverage process boundaries

The existing regular/e2e coverage passes and ready fixture are unchanged; the
new scenarios exercise `check` and the SCM commit seam.
