# Review — reverse-engineer-new-command

Reverse-engineering coverage review for the `givn new <id>` capability.
Confirms the characterization is complete, that no behaviour was changed
during refactoring, and that all discovered bugs and code smells are
documented as follow-up candidates.

## 1. Coverage

Measured with `cargo-tarpaulin --skip-clean --follow-exec` (the `--follow-exec`
flag is required because the characterization exercises the `givn` binary as a
subprocess; without it the tool reports 0% on the binary's source).

**Overall:** 41.17% line coverage, 478/1161 lines covered.

Per-file coverage of the modules on the `new` command's critical path:

| File | Covered / Total | % |
| --- | --- | --- |
| `src/cli/new.rs` | 25 / 25 | 100% |
| `src/cli/init.rs` | 25 / 29 | 86% |
| `src/config/cascade.rs` | 26 / 35 | 74% |
| `src/config/embedded.rs` | 2 / 2 | 100% |
| `src/config/render.rs` | 4 / 4 | 100% |
| `src/skills/targets.rs` | 49 / 50 | 98% |

The unit under characterization — `src/cli/new.rs` — is **100% line-covered**.
All other uncovered lines are in shared utilities (`cascade.rs`,
`init.rs`) that lie outside the `new` command's own behaviour and are exercised
only by other subcommands. Coverage of the codebase at large (41.17%) is a
side effect of running the full test suite; it is not the target of this
review. The target is the `new` command's own behaviour, which is fully
characterized.

### Raw tool output (abridged to the relevant files)

```
|| src/cli/init.rs: 41, 52-54
|| src/cli/new.rs: (none)
|| src/config/cascade.rs: 39, 43, 64, 70, 76, 80, 93, 97, 99
|| Tested/Total Lines:
|| src/cli/new.rs: 25/25 +0.00%
|| src/cli/init.rs: 25/29 +0.00%
|| src/config/cascade.rs: 26/35 +0.00%
|| src/config/embedded.rs: 2/2 +0.00%
|| src/config/render.rs: 4/4 +0.00%
|| src/skills/targets.rs: 49/50 +0.00%
|| 41.17% coverage, 478/1161 lines covered, +0.00% change in coverage
```

## 2. Coverage gap table

All gaps are in code that is **not** part of the `new` command's own behaviour;
they are shared utilities exercised by other subcommands. None are dead code.
None are introduced by this change.

| Bucket | Region | Action |
| --- | --- | --- |
| Missing test coverage | `src/cli/init.rs:41` — the "config.yaml already exists, use `--force`" branch of `givn init` | Follow-up `@givn.added` candidate in a future `givn init` characterization change. Not in scope for `new`. |
| Missing test coverage | `src/cli/init.rs:52-54` — `skills::targets::sync` failure / warning path inside `givn init` | Follow-up `@givn.added` candidate in a future `givn init` characterization change. Not in scope for `new`. |
| Missing test coverage | `src/config/cascade.rs:39,43` — error fallback in `resolve_template` when no template is found at any cascade layer | Unreachable through the `new` boundary (`new` hard-codes `"proposal"`, which always has an embedded template). Legitimately hard to test through `new`'s black-box seam. Follow-up `@givn.added` candidate for a future `config::cascade` characterization. |
| Missing test coverage | `src/config/cascade.rs:64,70,76,80` — `resolve_instruction` function (project override / embedded core / addon fallback / error path) | Not called by `new` (exercised by `givn instructions`). Follow-up `@givn.added` candidate for a future `givn instructions` characterization. |
| Missing test coverage | `src/config/cascade.rs:93,97,99` — `resolve_asset` function (generic embedded-asset resolver with project override) | Not called by `new`. Follow-up `@givn.added` candidate for a future characterization of whichever command consumes this helper. |

No dead code was found in the modules under review.
No region was classified as "legitimately hard to test" within the `new`
command's own code (`src/cli/new.rs` is 100% covered). The two `cascade.rs`
gaps that are unreachable through the `new` seam are classified as missing
test coverage (with the concrete justification that `new` hard-codes
`"proposal"`) rather than as hard-to-test, because they are testable through
other commands' boundaries.

## 3. Spec corrections

Scenarios updated during characterization because the initial inference (from
reading code/docs) was wrong.

### 3.1 Empty id is rejected

- **Scenario title:** `Empty id is rejected`
- **What was originally inferred:** The empty string would be intercepted by
  clap as a missing positional argument, yielding exit code 2 (the argument-
  parsing exit code). The original spec asserted only "exits non-zero".
- **What the actual behaviour is:** clap does **not** intercept the empty
  string — it is a valid positional value, so `validate_id` runs and bails
  with `"change id must not be empty"`. The process exits **1** (via the main
  error mapper), not 2. The spec was tightened from "exits non-zero" to
  "exits 1" with the specific stderr message
  `"change id must not be empty"`.
- **Root cause:** Code ambiguous / non-obvious — clap's treatment of `""` as a
  valid positional value is not self-evident from reading the arg parser
  definition; one expects clap to reject an empty positional. The probing
  test (`givn new ""`) revealed the truth.

No other spec corrections were made. The remaining 16 scenarios matched their
initial inference on the first probing run.

## 4. Documentation corrections

Documentation updated during characterization because it was wrong.

### 4.1 `design.md` — § Scenario → seam mapping (Empty id is rejected)

- **What it said before:** The empty-id scenario was described as asserting
  only a generic non-zero exit, with a parenthetical suggesting clap might
  intercept the empty string (exit 2).
- **What it says now:** The seam note records the observed AS-IS behaviour:
  `validate_id` runs (clap does not intercept an empty positional argument),
  rejects it before the `givn/`-not-found check, and the process exits **1**
  with stderr `"change id must not be empty"`. The note explicitly calls out
  that exit is 1, not 2.

### 4.2 `docs/arc42/06-runtime-view.md` — § 6.2 step 1 (validate id)

- **What it said before:** The validation list noted only that a leading-
  hyphen id passed without `--` is intercepted by clap (exit 2) before the
  validator runs. It did not address the empty-string case.
- **What it says now:** An additional note clarifies that the empty string
  `""` is NOT intercepted by clap — it is a valid positional value, so
  `validate_id` runs and bails with `"change id must not be empty"` → exit 1,
  not exit 2. This captures the spec correction at the architecture-doc
  layer so future readers are not misled about clap's empty-string handling.

### 4.3 `docs/arc42/11-risks-and-technical-debt.md` — § 11.2 deliberate debt

- **What it said before:** No entries for the bugs/smells found during `new`
  characterization (D-5 through D-8 absent).
- **What it says now:** Four new deliberate-debt entries (D-5 BUG-1
  `{{CHANGE_ID}}` placeholder not substituted; D-6 SMELL-1 check-ordering;
  D-7 SMELL-2 clap-intercepted leading hyphen; D-8 SMELL-3 `givn init` does
  not scaffold `givn/artifacts/`) record each discovered issue, its AS-IS
  characterization, and the future `@givn.modified` repayment delta.

No other documentation (README, code comments, tasks.md) required correction.
The `proposal.md` Out-of-Scope section already listed BUG-1, SMELL-1,
SMELL-2, SMELL-3 from discovery and required no further edits during
characterization.

## 5. Code smell markers

All `REVERSE-ENGINEERING NOTE:` comments added during characterization.

**None.** No spec/implementation mismatches of type (b) — "code is
ambiguous/misleading but does Z while reading like Y" — were found. Every
observed behaviour matched what a careful reading of the source suggested;
the single mismatch (the empty-id exit code, §3.1) was a documentation/
inference gap, not misleading code, so no in-source marker was warranted.

## 6. Suspected bugs

All bugs discovered during characterization. None were fixed — each is
characterized as AS-IS and flagged as a follow-up `@givn.modified`
candidate.

### 6.1 BUG-1 — `{{CHANGE_ID}}` placeholder not substituted

- **Observed behaviour:** `givn new <id>` writes the resolved proposal
  template verbatim to `givn/changes/<id>/proposal.md`. The embedded default
  template's heading reads `# Proposal: {{CHANGE_ID}}`, and no substitution
  is performed, so the created file contains the literal text
  `{{CHANGE_ID}}` instead of the user-supplied change id.
- **Correct behaviour:** The placeholder should be substituted with `<id>`
  (or the placeholder should be removed from the embedded template).
- **Scenario that captures the AS-IS behaviour:** `proposal.md content
  equals the built-in template verbatim` — asserts the created file equals
  `include_str!("../assets/templates/proposal.md")` byte for byte, and that
  the heading contains the literal text `{{CHANGE_ID}}`.
- **Follow-up candidate:** `@givn.modified` delta against the above scenario
  to substitute the id (or drop the placeholder). Tracked as deliberate
  debt D-5 in `docs/arc42/11-risks-and-technical-debt.md`.

### 6.2 SMELL-2 — Leading-hyphen id rejected by argument parsing, not validation

- **Observed behaviour:** `givn new -foo` (the natural invocation) is
  intercepted by clap as an unexpected flag before the id validator runs;
  the process exits 2 with an "unexpected argument" message. The dedicated
  "must not start or end with a hyphen" message is reachable only via
  `givn new -- -foo`. The two paths produce different exit codes and
  different error messages for what is semantically the same invalid input.
- **Correct behaviour:** A consistent rejection path (same exit code and
  message) regardless of whether `--` is used.
- **Scenarios that capture the AS-IS behaviour:**
  - `Id beginning with a hyphen is rejected by argument parsing` — exit 2,
    stderr mentions an unexpected argument.
  - `Id beginning with a hyphen passed after the argument separator is
    rejected by validation` — exit non-zero, stderr contains "must not start
    or end with a hyphen".
- **Follow-up candidate:** `@givn.modified` delta against both scenarios to
  unify the two paths. Tracked as deliberate debt D-7.

### 6.3 SMELL-1 — "already exists" check precedes "givn/ not found" check

- **Observed behaviour:** In `src/cli/new.rs`, the `change_dir.exists()`
  check (line 9) runs before the `givn/.exists()` check (line 14). When
  `givn/` is absent, `change_dir` (which lives under `givn/changes/`) cannot
  exist either, so the "already exists" branch is unreachable in that state
  and no wrong error is produced today. The ordering is, however, brittle:
  a future change to the directory layout could make the unreachable branch
  reachable and surface a misleading "already exists" error before the more
  fundamental "givn/ not found" error.
- **Correct behaviour:** Check `givn/` presence before checking for a
  duplicate change, so the more fundamental precondition is reported first.
- **Scenario that captures the related AS-IS behaviour:** `Uninitialised
  project is rejected` and `Already-existing change is rejected` (each
  characterizes its own path; the ordering smell itself is not directly
  observable because the unreachable branch never fires).
- **Follow-up candidate:** `@givn.modified` delta to reorder the two checks.
  Tracked as deliberate debt D-6. (Not a runtime bug today — flagged as a
  code smell / latent ordering hazard.)

### 6.4 SMELL-3 — `givn init` does not scaffold `givn/artifacts/`

- **Observed behaviour:** `givn init` creates `givn/specs`, `givn/changes`,
  `givn/archive`, and `givn/config.yaml`, but does not create
  `givn/artifacts/`. A user who wants a custom proposal template must
  create `givn/artifacts/proposal/` by hand before placing their override
  `proposal.md`. The characterization harness worked around this with a
  `write_override` helper that `create_dir_all`s the subtree.
- **Correct behaviour:** Either `givn init` scaffolds `givn/artifacts/`
  (possibly empty, or with a `.gitkeep`), or `givn eject`/documentation
  guides the user to create it on demand.
- **Scenario that captures the related AS-IS behaviour:** `Project override
  at givn/artifacts/proposal/proposal.md is used instead of the built-in
  template` — the scenario's Given step pre-creates the override file; the
  harness note in `design.md` records that `givn init` does not scaffold the
  directory and the helper must `create_dir_all` it itself.
- **Follow-up candidate:** Tracked as deliberate debt D-8 (no `@givn.modified`
  scenario delta required — this is an enhancement to `givn init`, not a
  modification of an existing `new`-command scenario). Flagged in
  `proposal.md` Out of Scope.

## 7. Boundary failures

Scenarios commented out because the black-box boundary proved untestable.

**None.** Every scenario in the `.feature` file is backed by a passing Rust
integration test. The subprocess + temp-dir-CWD seam was sufficient to
characterize all 17 scenarios, including the clap-intercepted leading-hyphen
case (which a unit-level call could not reproduce). No scenario was
commented out, deferred, or marked `@wip`.

## 8. Test suite output and result

Final GREEN confirmation via `cargo test` (the configured `verify.command`,
also run by `givn check review --change reverse-engineer-new-command`):

```
running 17 tests
test already_existing_change_is_rejected ... ok
test empty_id_is_rejected ... ok
test id_beginning_with_a_hyphen_is_rejected_by_argument_parsing ... ok
test id_beginning_with_a_hyphen_passed_after_the_argument_separator_is_rejected_by_validation ... ok
test id_containing_digits_is_accepted ... ok
test id_ending_with_a_hyphen_is_rejected ... ok
test id_with_a_dot_is_rejected ... ok
test id_with_non_ascii_letters_is_rejected ... ok
test id_with_spaces_is_rejected ... ok
test id_with_underscores_is_rejected ... ok
test id_with_uppercase_letters_is_rejected ... ok
test only_proposal_md_is_created_in_the_change_directory ... ok
test project_override_at_givn_artifacts_proposal_proposal_md_is_used_instead_of_the_built_in_template ... ok
test proposal_md_content_equals_the_built_in_template_verbatim ... ok
test single_word_id_is_accepted ... ok
test uninitialised_project_is_rejected ... ok
test valid_multi_word_kebab_case_id_creates_the_change_scaffold ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The unit-test crate (`lib`) reports 49 passed; 0 failed. Total: 66 tests
passing, 0 failing. `givn check review` confirms GREEN and the review gate's
`verify` hook passes (`✓ verify`).

### Behaviour-preservation invariant

No production code was changed during this characterization — `src/cli/new.rs`
and `src/cli/init.rs` are byte-identical to their pre-characterization state,
and no testability seams were introduced (the subprocess + temp-dir-CWD seam
was sufficient). The REFACTOR phase was not invoked for any scenario.
Accordingly, the behaviour-preservation invariant (all characterization
tests GREEN after refactoring) holds trivially: there was no refactoring, and
all 17 characterization tests pass on the unmodified binary. No scenario
went RED during the change; no refactor was reverted.

## 9. Sign-off checklist

- [x] All scenarios pass (test runner exits 0). `cargo test` → 17/17
      characterization tests GREEN, 49/49 lib tests GREEN.
- [x] Every coverage gap classified and resolved (or justified). Five gaps
      found, all in shared utilities outside the `new` command's own code;
      each classified as missing test coverage and flagged as a follow-up
      `@givn.added` candidate for a future characterization change. No dead
      code; no untestable-within-`new` regions left unjustified.
- [x] No `@wip` tags remain in the delta spec. `grep "@wip"` on
      `specs/new-command/new-command.feature` returns no matches.
- [x] The delta spec contains no implementation-layer detail. The `.feature`
      file speaks only of exit codes, stderr/stdout content, and filesystem
      effects; no function names, module paths, or library references. The
      single implementation-flavoured word ("argument parsing") appears in
      a scenario title and exit-code assertion (exit 2) because the
      clap-interception split is itself the observable AS-IS behaviour
      (SMELL-2), not an implementation choice.
- [x] All spec corrections documented. One correction (§3.1, empty-id exit
      code 1 + specific message).
- [x] All documentation corrections documented. Three documents updated
      (§4.1 design.md, §4.2 arc42 ch.6.2, §4.3 arc42 ch.11.2).
- [x] All code smell markers listed. None were added (§5) — no type-(b)
      spec/implementation mismatches were found.
- [x] All suspected bugs flagged as follow-up candidates (NOT fixed). Four
      issues (§6.1 BUG-1, §6.2 SMELL-2, §6.3 SMELL-1, §6.4 SMELL-3); each
      characterized as AS-IS and tracked as deliberate debt D-5..D-8 in
      arc42 ch.11.2.
- [x] Behaviour-preservation invariant confirmed. All characterization tests
      GREEN after refactoring (§8) — no refactoring was performed, so the
      invariant holds trivially.

REVIEW: PASS
