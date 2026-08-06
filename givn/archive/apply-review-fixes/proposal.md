# Proposal: apply-review-fixes

## Problem / Opportunity

A Linus Torvalds-style review of the entire givn project identified correctness
bugs, security risks, and code-quality issues that degrade the tool's reliability.
Four are blocking — they affect correctness or safety. Four are
recommended — they affect maintainability and polish.

**Blocking defects:**

1. **Uppercase `[X]` checkboxes are ignored by `givn status`.** The status
   command's task-progress parser counts `[x]` (lowercase) as done but ignores
   `[X]` (uppercase). Many editors default to uppercase `[X]` when toggling
   checkboxes. The user sees tasks as perpetually incomplete even though the
   integrity hook correctly parses those uppercase lines. The feature's commit
   hash pass-rate is correct; the status display is wrong.

2. **Post-merge verify-e2e gate is weaker than the per-change gate.** When
   `verify.e2e_command` is empty, the archive command skips the post-merge e2e
   gate entirely — even when the merged permanent specs now contain `@e2e`
   scenarios contributed by the delta. This means a delta that adds `@e2e`
   scenarios, once merged into the permanent spec, runs without any e2e
   verification. The per-change hook behaviour (hard-fail in the same condition)
   is the correct policy.

3. **Path traversal in `--change` parameter.** Only `givn new` validates that
   change ids are kebab-case. All other commands that accept `--change`
   (`archive`, `check`, `graph`, `instructions`, `lint`, `status`) construct
   `givn/changes/<id>` paths without validating `<id>`. A value like `../` can
   escape the `givn/changes/` directory and read or write outside the expected
   tree.

4. **Duplicate `find_change_dir` implementations.** Two implementations exist:
   one string-based (fails for absolute paths), one component-based (correct).
   Behavioural divergence is a latent bug — a test written against one version
   could pass while the other fails silently.

**Recommended improvements:**

5. The coverage gate in archive.rs is ~160 lines of self-contained logic inside a
   467-line file. Extracting it improves navigability and testability.

6. `givn check <artifact>` passes unrecognized artifact ids straight to the hook
   engine without validating them against the manifest. The error message is
   functional ("artifact not found in manifest") but comes from deep in the hook
   engine, not from the validation layer. `givn instructions` validates first and
   gives a helpful error. Consistency is expected.

7. `read_skip_list()` in state.rs is dead code — annotated `#[allow(dead_code)]`
   and called nowhere. The skip-list logic is handled by `is_skipped()`.

8. The project has no license. `Cargo.toml` says `"TBD"`; the README says "not
   yet chosen." This blocks `cargo publish` and signals ambiguity to potential
   contributors.

## Proposed Solution

**For blocking defects:**

1. `givn status` and `givn status --json` report checkbox progress identically
   for `[x]` and `[X]`. Both count toward "done" totals.

2. `givn archive` mirrors the per-change e2e hook behaviour: when
   `verify.e2e_command` is empty and the merged permanent specs contain `@e2e`
   scenarios, the archive is blocked with a clear error.

3. All commands accepting `--change` validate the id as kebab-case before
   constructing any filesystem path. Invalid ids produce a clear error.

4. Change-directory detection produces identical results regardless of the code
   path that calls it. A single implementation serves all call sites.

**For recommended improvements:**

5. Coverage-gate extraction to a separate file — no observable behaviour change.

6. `givn check <artifact>` and `givn eject <artifact>:instruction` validate
   `<artifact>` against the manifest before any further processing, producing
   the same helpful error as `givn instructions` does.

7. Unreachable code removed — no observable behaviour change.

8. The project is dual-licensed MIT OR Apache-2.0, matching Rust ecosystem
   convention. `Cargo.toml` and `README.md` reflect this.

## Out of Scope

- Adding a `--change` parameter to commands that currently lack it.
- Changing the format of the `[x]` / `[X]` markers in tasks.md (the marker is
  settled; this fix only changes parsing).
- Refactoring the archive command beyond extracting the coverage gate.
- Changing the license once chosen (MIT OR Apache-2.0 is the final decision).
- Path validation via `resolve_change_dir` applies uniformly to all commands that
  accept `--change` (including `givn lint`), but `givn lint`'s core rules
  (syntax checking, tag enforcement, `@wip`/PENDING detection) are unchanged.
  The only CLI-visible difference in lint is the improved error message when
  `--change` receives an invalid id — identical to every other command that
  takes `--change`.
- Pipeline changes to the archive rollback mechanism (only the e2e gate logic
  changes).

## Open Questions

None — all decisions are settled. The review identified specific, actionable
changes. The fix for each is clear from the affected source lines.
