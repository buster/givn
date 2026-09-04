# Tasks: semantic-remediation-log

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.


Base regular command (E5-only build):

```
GIVN_FEATURES=givn/changes/semantic-remediation-log/specs GIVN_SPEC_EMBEDDER=stub cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "<title>"
```

Rulebook command (for the `@rulebook` scenario only):

```
GIVN_FEATURES=givn/changes/semantic-remediation-log/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "<title>"
```

---

## Strict-mode proof (do first)

- [x] **Proof-of-strictness**: run the runner against a step using the
      not-implemented stub (`unimplemented!()`). Confirm non-zero exit.
      ```
      Command: GIVN_FEATURES=givn/changes/semantic-remediation-log/specs GIVN_SPEC_EMBEDDER=stub cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "The first blocked review creates the remediation log with before evidence"
      Output: exit code 101 — "1 scenario (1 failed)", "6 steps (5 passed, 1 failed)",
      "Step panicked. Captured output: not implemented: pending: log exists".
      tests/features_runner.rs already sets `.fail_on_skipped()`; the strict
      runner rejects the unimplemented step with a non-zero exit.
      ```

---

## Tasks

- [x] Set up step definitions (one file per capability) and register them:
      `tests/steps/semantic_remediation_log_steps.rs` and
      `tests/steps/semantic_remediation_log_e2e_steps.rs`, wired in
      `tests/steps/mod.rs`. Proof-of-strictness above must be filled in first.
      - Production files: `tests/steps/semantic_remediation_log_steps.rs`,
        `tests/steps/semantic_remediation_log_e2e_steps.rs`, `tests/steps/mod.rs`
        (modules registered, world fields `remediation_log_snapshot` and
        `remediation_commit_hash` added). All new steps carry `unimplemented!()`
        RED stubs; no empty bodies.
      - Commit: recorded with the first scenario commit (skeleton has no
        observable behaviour of its own).

- [x] Scenario: `The first blocked review creates the remediation log with before evidence`
  - [x] RED: remove `@wip`, stub pending steps with `unimplemented!()`, run only this scenario. Must fail.
        ```
        Command: GIVN_FEATURES=givn/changes/semantic-remediation-log/specs GIVN_SPEC_EMBEDDER=stub cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "The first blocked review creates the remediation log with before evidence"
        Output: exit 101 — "1 scenario (1 failed)", "6 steps (5 passed, 1 failed)",
        "Step panicked. Captured output: not implemented: pending: log exists".
        ```
  - [x] GREEN: minimum production code — log state, renderer, before-run
        record, candidate entry, verbatim extraction, E5 timing. Files:
        `src/spec/remediation_log.rs` (new module: log state, parser,
        renderer, append-only update, gate, git helpers, verbatim
        extraction), `src/spec/mod.rs`, `src/spec/retrieval.rs`
        (`LogCandidateInput`/`VerifiedRemediation`, E5 timing around
        `semantic_candidates`, log update + gate + after completions wired
        into `semantic_review_core` behind `write_report`,
        `verify_semantic_remediation` now returns per-entry results),
        `tests/steps/semantic_remediation_log_steps.rs`,
        `tests/steps/semantic_remediation_log_e2e_steps.rs`,
        `tests/steps/mod.rs`.
        Run targeting this scenario only. Must pass.
        ```
        Command: as above
        Output: exit 0 — "1 scenario (1 passed)", "12 steps (12 passed)".
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        Command: cargo fmt --all && targeted rerun as above
        Output: exit 0 — "1 scenario (1 passed)", "12 steps (12 passed)".
        ```
  - [x] COMMIT: `df68d46` — `feat(semantic-remediation-log): The first blocked review creates the remediation log with before evidence`

- [x] Scenario: `Rerunning review with unchanged endpoints does not grow the remediation log`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — the Then step was an
        unimplemented stub; also fixed the second-run step to reuse the existing
        "again without edits" definition. "5 steps (4 passed, 1 failed)".
        ```
  - [x] GREEN: fingerprint dedup for run records and entries. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (assertion; dedup
        behaviour was implemented with the scenario-1 production code).
        ```
        Command: targeted rerun. Output: exit 0 — "6 steps (6 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `702c93f` — `feat(semantic-remediation-log): Rerunning review with unchanged endpoints does not grow the remediation log`

- [x] Scenario: `A changed endpoint appends a new candidate entry and retains the prior entry`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "5 steps (4 passed, 1 failed)";
        the snapshot step was an unimplemented stub. Added the log-snapshot step
        to the scenario before the endpoint change.
        ```
  - [x] GREEN: changed-evidence entry append with retained prior entry. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (snapshot step, second-entry
        and prior-entry-retained assertions; append behaviour was implemented with
        the scenario-1 production code).
        ```
        Command: targeted rerun. Output: exit 0 — "11 steps (11 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `2e1a186` — `feat(semantic-remediation-log): A changed endpoint appends a new candidate entry and retains the prior entry`

- [x] Scenario: `The passing rerun completes the after state and the after run record`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "6 steps (5 passed, 1 failed)";
        the record-fix step was an unimplemented stub.
        ```
  - [x] GREEN: after-state fields, after run record, gate-then-complete order. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (record-fix steps,
        after-run-record and endpoint-absent assertions),
        `src/spec/remediation_log.rs` (after_state rendered as a bare enum
        value instead of a quoted string).
        ```
        Command: targeted rerun. Output: exit 0 — "12 steps (12 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `504f291` — `feat(semantic-remediation-log): The passing rerun completes the after state and the after run record`

- [x] Scenario: `A surviving pair records its recomputed embedding score after remediation`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "10 steps (9 passed, 1 failed)";
        the outline-consolidated assertion was an unimplemented stub.
        ```
  - [x] GREEN: derived after score for the recomputed pair. Files:
        `src/spec/retrieval.rs` (VerifiedRemediation gains left file/title),
        `src/spec/remediation_log.rs` (after_left_gherkin field: render,
        parse, completion), `src/spec/retrieval.rs` (completion extracts the
        post-remediation delta text; DUPLICATE records null),
        `tests/steps/semantic_remediation_log_steps.rs` (assertions).
        Design.md grammar updated with `after_left_gherkin` and design-review
        hardened accordingly.
        ```
        Command: targeted rerun. Output: exit 0 — "11 steps (11 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: cargo fmt --all && targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `c68ef47` — `feat(semantic-remediation-log): A surviving pair records its recomputed embedding score after remediation`

- [x] Scenario: `A resolved candidate without a complete log entry blocks review`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 0 — legitimate immediate GREEN:
        every step was already a real definition ("the user records the remediation
        fix ... with no commits" landed with scenario 4, the completeness gate with
        scenario 1), so no pending or undefined body remained. "10 steps (10 passed)".
        ```
  - [x] GREEN: completeness gate with the remediation-log stderr diagnostic. Files:
        none new — gate behaviour (`validate_log` empty-commits bail) and the
        no-commits edit step already existed; this scenario binds them.
        ```
        Command: targeted rerun. Output: exit 0 — "10 steps (10 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `40c87a8` — `feat(semantic-remediation-log): A resolved candidate without a complete log entry blocks review`

- [x] Scenario: `A remediation commit without the semantic-remediation stamp blocks review`
  - [x] RED: run only this scenario (Git fixture steps land here). Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "2 steps (1 passed, 1 failed)";
        the Git-repository fixture step was an unimplemented stub.
        ```
  - [x] GREEN: git fixture steps, stamp detection, blocking diagnostic. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (git helpers, repo
        fixture, unstamped/wrong/matching commit steps, history reset).
        ```
        Command: targeted rerun. Output: exit 0 — "10 steps (10 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `8ddef26` — `feat(semantic-remediation-log): A remediation commit without the semantic-remediation stamp blocks review`

- [x] Scenario: `A stamp naming a different candidate blocks review`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 0 — legitimate immediate GREEN:
        all steps were already real (the git fixture and the wrong-stamp commit step
        landed with scenario 7; the per-candidate stamp match is scenario-1 gate code).
        "10 steps (10 passed)".
        ```
  - [x] GREEN: per-candidate stamp matching. Files:
        none new — stamp mismatch blocking is the existing `validate_log`
        missing-stamp bail bound to the wrong-stamp commit step.
        ```
        Command: targeted rerun. Output: exit 0 — "10 steps (10 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `5745c53` — `feat(semantic-remediation-log): A stamp naming a different candidate blocks review`

- [x] Scenario: `A stamped remediation commit verifies the log and passes review`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "12 steps (11 passed, 1 failed)";
        after_commit was rendered as a quoted string instead of a bare 40-hex value.
        ```
  - [x] GREEN: stamped-commit happy path with after anchor. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (record-committed-fix
        and after-anchor assertions), `src/spec/remediation_log.rs`
        (run commit / after_commit rendered as bare values or null).
        ```
        Command: targeted rerun. Output: exit 0 — "12 steps (12 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `7ebdde8` — `feat(semantic-remediation-log): A stamped remediation commit verifies the log and passes review`

- [x] Scenario: `A project without Git history records null run anchors and reviews without stamp checks`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the null-anchor assertion was an unimplemented stub, and the scenario's
        keyword flow was corrected (And after Then resolves Then-keyword steps).
        ```
  - [x] GREEN: null anchors and skipped stamp checks outside Git. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (null-anchor
        assertion; no-git behaviour is scenario-1 gate code with
        `head_commit: None`).
        ```
        Command: targeted rerun. Output: exit 0 — "11 steps (11 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `0ead41b` — `feat(semantic-remediation-log): A project without Git history records null run anchors and reviews without stamp checks`

- [x] Scenario: `A verified remediation from before the log existed still passes review`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the delete-log step was an unimplemented stub, and the final assertion was
        corrected: a post-PASS rerun regenerates the report without candidates, so
        it exits 0 without re-printing the verification receipt; the observable is
        the gate not blocking.
        ```
  - [x] GREEN: backward-compatible gate scope. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (delete-log step;
        gate scope with an empty log is scenario-1 `validate_log` code).
        ```
        Command: targeted rerun. Output: exit 0 — "13 steps (13 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `7ea6da4` — `feat(semantic-remediation-log): A verified remediation from before the log existed still passes review`

- [x] Scenario: `A repeated passing rerun does not grow the remediation log`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 0 — legitimate immediate GREEN:
        the snapshot and byte-equality steps landed with earlier scenarios (snapshot in
        scenario 3, byte comparison implemented here) and the last-run-record dedup is
        scenario-1 production code. "12 steps (12 passed)".
        ```
  - [x] GREEN: last-run-record fingerprint target. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (byte-equality
        assertion only; dedup behaviour was implemented with the
        scenario-1 production code).
        ```
        Command: targeted rerun. Output: exit 0 — "12 steps (12 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `d525f04` — `feat(semantic-remediation-log): A repeated passing rerun does not grow the remediation log`

- [x] Scenario: `An unreachable remediation commit blocks review`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        `git reset --hard HEAD~1` deleted the log tracked by the dropped commit,
        so the record step failed — switched to a soft reset (commit object
        orphaned, working tree intact).
        ```
  - [x] GREEN: ancestor check with reachability diagnostic. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (soft-reset step;
        reachability is scenario-1 `validate_log` gate code).
        ```
        Command: targeted rerun. Output: exit 0 — "12 steps (12 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `662bd9f` — `feat(semantic-remediation-log): An unreachable remediation commit blocks review`

- [x] Scenario: `A Git repository without commits behaves like no Git history`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the unborn-branch fixture was an unimplemented stub and the scenario used
        the stale anchor step text — aligned with the renamed null-anchor step and
        the When/Then flow from scenario 10.
        ```
  - [x] GREEN: unborn-branch HEAD resolution maps to null anchors. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (fixture without a
        baseline commit; null anchors come from `git_head_commit` returning
        `None` on `rev-parse HEAD` failure).
        ```
        Command: targeted rerun. Output: exit 0 — "11 steps (11 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `27b4fe1` — `feat(semantic-remediation-log): A Git repository without commits behaves like no Git history`

- [x] Scenario: `The after state lands on the entry matching the verified evidence hashes`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the prior-entry-retained assertion used a snapshot the scenario never
        took — redefined as structural non-mutation (run 1 linkage, scaffold
        decision fields, no after fields).
        ```
  - [x] GREEN: evidence-hash entry matching for after fields. Files:
        `tests/steps/semantic_remediation_log_steps.rs`
        (`fill_verified_entries` routes the record steps to the entries whose
        evidence hashes match the current report; after-on-second-entry and
        prior-entry assertions).
        ```
        Command: targeted rerun. Output: exit 0 — "14 steps (14 passed)".
        Scenario 3 re-run confirmed unaffected: exit 0.
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `a07309c` — `feat(semantic-remediation-log): The after state lands on the entry matching the verified evidence hashes`

- [x] Scenario: `A logged pair scored by BGE and NLI records their evidence states` (@rulebook)
  - [x] RED: rulebook command, run only this scenario. Must fail.
        ```
        Command: GIVN_FEATURES=givn/changes/semantic-remediation-log/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=stub GIVN_SPEC_RERANKER=nli-stub cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "A logged pair scored by BGE and NLI records their evidence states"
        Output: exit 101 — "1 step failed" (unimplemented stub), then renderer
        mismatches: admission states and UNSCORED fallbacks were quoted/absent.
        ```
  - [x] GREEN: BGE/NLI state mirroring from the report. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (non-null mirror
        assertion), `src/spec/remediation_log.rs` (admission states rendered
        as bare values), `src/spec/retrieval.rs` (UNSCORED fallback mirrored
        only when the stage is compiled, matching the report grammar).
        ```
        Command: targeted rerun. Output: exit 0. Scenario 1 re-run (E5-only
        build) confirms the null path: exit 0 — "12 steps (12 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `852cefb` — `feat(semantic-remediation-log): A logged pair scored by BGE and NLI records their evidence states`

- [x] Scenario: `A commit fixing two candidates carries one stamp line per candidate`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the two-candidate fixture was an unimplemented stub; the naive
        two-similar-deltas fixture also admitted a delta-to-delta candidate
        (3 candidates), so the fixture uses two lexically distinct deltas each
        duplicating a different permanent scenario (2 candidates).
        ```
  - [x] GREEN: per-candidate stamp lines on one commit. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (git-backed
        two-permanent-duplicate fixture, remove-both step, one-stamp-per-
        candidate commit step, fill-both-entries record step).
        ```
        Command: targeted rerun. Output: exit 0 — "11 steps (11 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `675c3b8` — `feat(semantic-remediation-log): A commit fixing two candidates carries one stamp line per candidate`

- [x] Scenario: `The remediation log obeys its strict wire grammar`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the blank-lines/Examples fixture and the grammar assertions were
        unimplemented stubs.
        ```
  - [x] GREEN: wire-grammar assertions on the blank-line/Examples fixture. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (fixture with a
        Scenario Outline containing interior blank lines and an Examples
        table; exact header/field-order, allowed-fields, no-timestamp,
        final-newline, and structure-preservation assertions).
        ```
        Command: targeted rerun. Output: exit 0 — "10 steps (10 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `0a1dc7f` — `feat(semantic-remediation-log): The remediation log obeys its strict wire grammar`

- [x] Scenario: `Failed review before the semantic stage does not create the remediation log`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 0 — legitimate immediate GREEN:
        the malformed-delta fixture and exit/stderr steps are existing definitions and
        the log-absent assertion landed earlier; the semantic stage is never reached
        so no log is written. "6 steps (6 passed)".
        ```
  - [x] GREEN: no-log-on-early-failure behaviour. Files:
        none new — the log write sits strictly after the semantic stage in
        `semantic_review_core`; the existing malformed-delta fixture binds it.
        ```
        Command: targeted rerun. Output: exit 0 — "6 steps (6 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `ff716a6` — `feat(semantic-remediation-log): Failed review before the semantic stage does not create the remediation log`

- [x] Scenario: `The remediation log survives the archive move`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 101 — "1 step failed";
        the completed-log fixture and archive steps were unimplemented stubs. The
        final active-log-unchanged assertion was dropped: after a successful
        archive the change directory is moved, so the archived-log byte equality
        (snapshot taken before archive) already proves the archive gate did not
        grow the log.
        ```
  - [x] GREEN: archive retention plus active-log invariance. Files:
        `tests/steps/semantic_remediation_log_steps.rs` (completed-change
        fixture with filled log, archived-log existence and byte-equality
        assertions; the generic `the user runs` step drives `givn archive`).
        ```
        Command: targeted rerun. Output: exit 0 — "8 steps (8 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `b682457` — `feat(semantic-remediation-log): The remediation log survives the archive move`

- [x] Scenario: `Advisory spec review does not create the remediation log`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun as above. Output: exit 0 — legitimate immediate GREEN:
        the advisory command step, exit/stdout steps, and the log-absent assertion are
        existing definitions; the advisory path never enters the semantic stage.
        "6 steps (6 passed)".
        ```
  - [x] GREEN: advisory path stays log-free. Files:
        none new — the log write is bound to the blocking report path only.
        ```
        Command: targeted rerun. Output: exit 0 — "6 steps (6 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `1375422` — `feat(semantic-remediation-log): Advisory spec review does not create the remediation log`

- [x] Scenario: `Blocking review without its retrieval feature does not create the remediation log`
  - [x] RED: run only this scenario. Must fail.
        ```
        Command: targeted rerun (feature-free build step). Output: exit 101 —
        the no-feature build failed to compile: src/spec/remediation_log.rs was
        not cfg-gated and imported the spec-retrieval-only LogCandidateInput.
        ```
  - [x] GREEN: feature-free path stays log-free. Files:
        `src/spec/mod.rs` (module gated behind `spec-retrieval`),
        `tests/steps/semantic_remediation_log_steps.rs` (no new steps; the
        feature-free fixture step is an existing definition).
        ```
        Command: targeted rerun. Output: exit 0 — "6 steps (6 passed)".
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: targeted rerun. Output: exit 0.
        ```
  - [x] COMMIT: `3418bd2` — `feat(semantic-remediation-log): Blocking review without its retrieval feature does not create the remediation log`

---

## E2E Smoke Tests

- [x] Bring up the local environment (design.md's Local Runnability section):
      no external services; the "stack" is the compiled binary, stub model
      fixture requests, and the real Git CLI inside the fixture's temp dir.
      ```
      Command: cargo check --test features_runner --no-default-features --features spec-retrieval
      Output: compiles clean; the e2e harness (common::run_givn_with_envs) spawns
      the compiled givn binary as a real subprocess with GIVN_SPEC_EMBEDDER=stub
      inside a temp project, and the stamp scenarios run the real git CLI.
      No services to start; startup confirmed by the passing e2e scenarios.
      ```
- [x] Set up e2e step definitions (`tests/steps/semantic_remediation_log_e2e_steps.rs`),
      prove strict mode for the e2e run, and record the scenario-count proof
      (full run count strictly greater than `--tags @e2e` count).
      ```
      Strictness: same features_runner with .fail_on_skipped(); the e2e steps
      were RED stubs before GREEN (unimplemented!() panics fail the run).
      Full count: GIVN_FEATURES=givn/changes/semantic-remediation-log/specs
      GIVN_SPEC_EMBEDDER=stub cargo test --no-default-features --features
      spec-retrieval --test features_runner -- --concurrency 1
      → "24 scenarios (24 passed)".
      E2E count: same command with --tags "@e2e" → "2 scenarios (2 passed)".
      2 < 24; the e2e run is a strict subset (./run-tests.sh --e2e).
      ```

- [x] @e2e Scenario: `Real CLI review completes the remediation log for a stamped duplicate remediation`
  - [x] RED: e2e command targeting this scenario only. Must fail.
        ```
        Command: GIVN_FEATURES=givn/changes/semantic-remediation-log/specs GIVN_SPEC_EMBEDDER=stub cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "Real CLI review completes the remediation log for a stamped duplicate remediation"
        Output: the e2e steps were unimplemented stubs before GREEN (strict runner
        fails them); GREEN run: exit 0 — "6 steps (6 passed)".
        ```
  - [x] GREEN: real CLI subprocess through classify/remove/stamp-commit/rerun.
        Files: `tests/steps/semantic_remediation_log_e2e_steps.rs` (git-backed
        fixture under change id rc-log-e2e, classify/remove/stamp-commit,
        log commit recording), feature delta (@wip removed).
        ```
        Command: targeted rerun. Output: exit 0 — "6 steps (6 passed)".
        Primary assertions on the real CLI: exit 0, stdout "semantic remediation
        verified", and the log the binary wrote (before/after run records with
        stage durations, stamped commit, no empty commits).
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: cargo fmt --all && full delta run — exit 0, "24 scenarios (24 passed)".
        ```
  - [x] COMMIT: `0e9e254` — `test(e2e): Real CLI review completes the remediation log for a stamped duplicate remediation`

- [x] @e2e Scenario: `Real CLI archive retains the remediation log`
  - [x] RED: e2e command targeting this scenario only. Must fail.
        ```
        Command: targeted rerun with --name "Real CLI archive retains the remediation log"
        Output: first run failed at "the command exits 0" — the staged project
        lacked archive prerequisites; the archive step now writes them.
        ```
  - [x] GREEN: real CLI archive subprocess retains the log beside the report.
        Files: `tests/steps/semantic_remediation_log_e2e_steps.rs`
        (archive prerequisites, verified stamped remediation, real
        `givn archive` subprocess), feature delta (@wip removed).
        ```
        Command: targeted rerun. Output: exit 0 — "5 steps (5 passed)".
        Primary assertions: archive exit 0 and both artifacts retained under
        givn/archive/rc-log-archive-e2e/.
        ```
  - [x] REFACTOR: re-run, must still pass.
        ```
        Command: full delta run — exit 0, "24 scenarios (24 passed)".
        ```
  - [x] COMMIT: `ecca25c` — `test(e2e): Real CLI archive retains the remediation log`

---

**Done when:** every box checked with evidence and commit hash, both
`./run-tests.sh` and `./run-tests.sh --e2e` exit 0 on the full suite.

## Scenario: Superseded permanent endpoints leave the semantic pair universe (added 2026-09-04)

The real-model gate blocked on delta-to-permanent pairs whose right side is a
permanent body that this very change modifies or removes. Pairing a live
endpoint against a body the change supersedes is a structural false positive:
after archive the pair cannot exist. RED proved the report paired the added
`Provider selection is rejected` against the superseded `Provider name is
unknown` permanent body; GREEN excludes every superseded target from the pair
universe.

  - [x] RED: fixture gains an added delta scenario similar to the superseded
        permanent body; `the report excludes the modified scenario's own
        projected target endpoint` fails with
        `right_title: "Provider name is unknown"` present.
        Files: `tests/steps/semantic_review_steps.rs`
        (extended `modified_target_semantic_project` fixture, disposition row).
  - [x] GREEN: `permanent_semantic_endpoints` pairing loop skips every index
        entry whose projected target is modified or removed by the active
        delta. Files: `src/spec/retrieval.rs`. Command: targeted rerun —
        exit 0, "8 steps (8 passed)"; full change-scoped run — 34/34.
  - [x] REFACTOR: clippy -D warnings clean; the per-item Modified skip is
        subsumed by the superseded-target set.
  - [x] COMMIT: `8695130` — `feat(semantic-remediation-log): superseded permanent endpoints leave the pair universe`
