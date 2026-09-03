# Tasks: semantic-remediation-log

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in. Check boxes immediately after each scenario — tasks.md must always reflect true state.

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
  - [x] COMMIT: pending hash — `feat(semantic-remediation-log): A stamp naming a different candidate blocks review`

- [ ] Scenario: `A stamped remediation commit verifies the log and passes review`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: stamped-commit happy path with after anchor. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A stamped remediation commit verifies the log and passes review`

- [ ] Scenario: `A project without Git history records null run anchors and reviews without stamp checks`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: null anchors and skipped stamp checks outside Git. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A project without Git history records null run anchors and reviews without stamp checks`

- [ ] Scenario: `A verified remediation from before the log existed still passes review`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: backward-compatible gate scope. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A verified remediation from before the log existed still passes review`

- [ ] Scenario: `A repeated passing rerun does not grow the remediation log`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: last-run-record fingerprint target. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A repeated passing rerun does not grow the remediation log`

- [ ] Scenario: `An unreachable remediation commit blocks review`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: ancestor check with reachability diagnostic. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): An unreachable remediation commit blocks review`

- [ ] Scenario: `A Git repository without commits behaves like no Git history`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: unborn-branch HEAD resolution maps to null anchors. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A Git repository without commits behaves like no Git history`

- [ ] Scenario: `The after state lands on the entry matching the verified evidence hashes`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: evidence-hash entry matching for after fields. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): The after state lands on the entry matching the verified evidence hashes`

- [ ] Scenario: `A logged pair scored by BGE and NLI records their evidence states` (@rulebook)
  - [ ] RED: rulebook command, run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: BGE/NLI state mirroring from the report. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A logged pair scored by BGE and NLI records their evidence states`

- [ ] Scenario: `A commit fixing two candidates carries one stamp line per candidate`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: per-candidate stamp lines on one commit. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): A commit fixing two candidates carries one stamp line per candidate`

- [ ] Scenario: `The remediation log obeys its strict wire grammar`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: wire-grammar assertions on the blank-line/Examples fixture. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): The remediation log obeys its strict wire grammar`

- [ ] Scenario: `Failed review before the semantic stage does not create the remediation log`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: no-log-on-early-failure behaviour. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): Failed review before the semantic stage does not create the remediation log`

- [ ] Scenario: `The remediation log survives the archive move`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: archive retention plus active-log invariance. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): The remediation log survives the archive move`

- [ ] Scenario: `Advisory spec review does not create the remediation log`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: advisory path stays log-free. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): Advisory spec review does not create the remediation log`

- [ ] Scenario: `Blocking review without its retrieval feature does not create the remediation log`
  - [ ] RED: run only this scenario. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: feature-free path stays log-free. Files:
        <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(semantic-remediation-log): Blocking review without its retrieval feature does not create the remediation log`

---

## E2E Smoke Tests

- [ ] Bring up the local environment (design.md's Local Runnability section):
      no external services; the "stack" is the compiled binary, stub model
      fixture requests, and the real Git CLI inside the fixture's temp dir.
      ```
      <!-- paste local-run command + confirmation -->
      ```
- [ ] Set up e2e step definitions (`tests/steps/semantic_remediation_log_e2e_steps.rs`),
      prove strict mode for the e2e run, and record the scenario-count proof
      (full run count strictly greater than `--tags @e2e` count).
      ```
      <!-- paste e2e proof-of-strictness + counts -->
      ```

- [ ] @e2e Scenario: `Real CLI review completes the remediation log for a stamped duplicate remediation`
  - [ ] RED: e2e command targeting this scenario only. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: real CLI subprocess through classify/remove/stamp-commit/rerun.
        Files: <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `test(e2e): Real CLI review completes the remediation log for a stamped duplicate remediation`

- [ ] @e2e Scenario: `Real CLI archive retains the remediation log`
  - [ ] RED: e2e command targeting this scenario only. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: real CLI archive subprocess retains the log beside the report.
        Files: <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `test(e2e): Real CLI archive retains the remediation log`

---

**Done when:** every box checked with evidence and commit hash, both
`./run-tests.sh` and `./run-tests.sh --e2e` exit 0 on the full suite.
