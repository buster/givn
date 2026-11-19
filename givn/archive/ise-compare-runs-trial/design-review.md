# Design review: ise-compare-runs-trial

Review kind: independent
Author: givn-maintainer--owner
Reviewer context: independent-reviewer

Reviewer note: the independent reviewer context re-read this design review and
the implementation and confirmed the grilling and the `10b8074a` step-table
correction. The self-review questions and answers below are retained as the
grilling record.

## Grilling

| Branch | Question and answer | Finding |
|---|---|---|
| Scope | Does the delta contain exactly the proposal? The proposal asks for default `compare` execution, an explicit `--inputs-only` path, a result outside the copy, the existing product binding, the preserved scenario gate, and component-workflow guidance. The delta covers each; no numeric coverage rule is added. | None. |
| Tech choices | Is reusing `ise::verify_trial` from `cli::ise::compare` the simplest fit? Yes. It reuses the existing gate, result writer, product binding, and freshness helpers; the library `ise::compare` stays the internal input-only helper, so there is no recursion. A second verification implementation would drift. | None. |
| Missing scenarios | Are error paths present? Yes: input difference, missing component runner, result inside the copy, inputs-only no result, and the Passuff-shaped missing-runner integration test. A stale result is removed before the trial; the Passuff and component-tier tests assert the overwritten result. The copy-execution security boundary has a documented risk and `--inputs-only`; it needs no separate Gherkin scenario. | None. |
| Testability | Can each scenario fail in RED? Yes. The missing-runner scenario exits 1 with `component_executed=0/10`; the inputs-only scenario writes no result; the result-inside-copy scenario refuses by name; the E2E scenario prints `Trial result: passed` only after the real contract runs. The strict mode is cucumber-rs `.fail_on_skipped()` plus a non-zero exit on failed or skipped stats in both runners. Single-scenario command is in `design.md`. | None. |
| E2E fidelity | The E2E scenario drives the real binary through the `e2e` package and the real CLI; the fixture writes a component runner in the copy and a frozen End-to-end runner in the checkout, and the assertion reads the real stdout and exit code. No HTTP or internal call replaces the CLI. | None. |
| Interaction Coverage | The owning use case has one `trial-input-verification` row; the delta E2E title is `Verifier compares the spec copy with the manifest and runs the trial`. The inputs-only, missing-runner, and result-path scenarios are variants of the same action, not additional inventory entries. | None. |
| Risk | Most likely failure: a future caller treats `--inputs-only` as a trial and cites exit 0. Mitigation: the output line starts with `Trial result: unverified`, the help and three docs name it, and `--inputs-only` writes no result. The second risk is executing an untrusted copy's scripts; R-172 records it and the docs name `--inputs-only`. | None. |
| Use-case and Persona context | Use case `prepare-an-ise-test`, personas `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. The delta use case preserves the goal, guarantees, rules, typed relationships, and inventory, and updates only the compare/verify flow. No invented Persona. | None. |
| Decision ledger | D1-D7 resolve in `design.md`; the Ise rebuild decision is `No rebuild`, with no `Rebuilt code` line. | None. |
| Contracts and components | `IseTrial` is consumer-shaped (manifest, copy, optional result, inputs-only) and implemented by `cli::ise` plus `ise::verify`; `ServedInstruction` is implemented by `src/cli/instructions` plus the asset. Each has one responsibility and an external consumer. No consumer imports a component implementation. | None. |
| Decision timing | No carried question and no speculative abstraction. The research result closes the coverage question: no numeric non-decline; the scenario gate and non-zero component line measurement stay. | None. |
| Ise rebuild decision | Exactly one `Decides D8` entry in `design.md` records `No rebuild`; the change edits existing Source code and records no `Rebuilt code` line. | None. |
| Visual design contract | The change is CLI text only, not a screen. No visual contract is declared; the CLI output is covered by terminal assertions and the E2E transcript. | None. |
| Deferrals | None. | None. |
| Persona disposition | The CLI output is user-visible, so `developer--primary-user` and `agent--technical-role` are named; their evidence is the command output and the guidance scenario. | None. |
| Interface classification | CLI, not Web UI. The real interface is the compiled binary invoked by the test harness; no browser driver is required. | None. |
| ADR qualification | `arc42.md` records `AMEND_ADR` for ADR-076. The 2026-11-18 amendment's rejected alternative is the decision being reversed; the security consequence is recorded in R-172. A new ADR would split one decision across two files. | None. |
| Architecture documentation | Independently walked the 12 rows: 2, 3, 4, 5, 6, 8, 9, 11, 12 are Yes; 1, 7, 10 are No. `arc42.md` matches. The chapter files carry the updated text and no ASCII diagrams. Chapter 9 links the amended ADR; chapter 11 carries R-172. | None. |
| Ubiquitous language | The design uses existing glossary terms **Trial result**, **Verification contract**, and **Execution unit**; the glossary entry is updated, no new term is invented. | None. |
| Structural model | No new domain entity or boundary; `docs/domain-model.md` needs no structural change. | None. |

## Hardening

- `design.md` already carries the strict-mode mechanism, the single-scenario
  command, the result-path default, freshness, and the component workflow rule.
- No new `@wip` scenario was needed; the delta scenarios bind to real steps.
- No `@e2e` tag was removed. The E2E fixture now supplies the component runner,
  the frozen End-to-end runner, and the product binding.

## Re-review after independent findings

| Finding | Fix | Evidence |
|---|---|---|
| `--result <manifest>` deleted the manifest before it was read. | `prepare_trial_result` refuses a result equal to the manifest and validates before any read or comparison. | `a_result_path_equal_to_the_manifest_refuses_without_deleting_it` |
| The pre-run revocation deleted an arbitrary existing file. | Revocation removes only a regular `ise-trial-result/v1` file and regular Givn runner-result siblings; a directory, symlink, malformed file, or unrelated file refuses. | `an_existing_non_givn_result_is_never_deleted`, `a_non_givn_raw_sibling_is_never_deleted`, `a_symlink_result_path_refuses` |
| A default compare on an input difference left a stale clean result. | The CLI validates and revokes the result before the comparison; a prior Givn-owned result is removed even when the inputs differ. | `a_previous_givn_result_is_revoked_on_an_input_difference` |
| A default result inside the checkout was accepted. | `prepare_trial_result` refuses a result inside the checkout; a manifest inside the checkout is covered. | `a_default_result_inside_the_checkout_refuses`, `a_result_path_inside_the_checkout_refuses` |
| `--result` was not validated on mismatched inputs. | The CLI preflight runs before the comparison for every non-inputs-only call. | `a_result_path_inside_the_checkout_refuses`, `a_result_path_equal_to_the_manifest...` |
| The E2E transcript argv omitted `--inputs-only`; the new executed title had no transcript. | The transcript selector covers both titles and appends `--inputs-only` only for the read-only scenario. | `verifier-compares-the-spec-copy-with-the-manifest-and-runs-the-trial/transcript.txt` |
| The component success fixture printed titles with `awk` without executing a step. | The fixtures now use a minimal step-binding runner that parses the frozen feature, executes `Given nothing`, and refuses an unbound step; the missing-runner regression remains. | `compare_runs_the_trial_and_records_a_clean_result`, `a_passuff_shaped_copy_runs_the_trial_through_compare_by_default`, `compare_refuses_a_missing_component_runner` |
| The `## Step Definitions` table put the file in column 1; the integrity hook reads column 2. | Reordered the table to `| Capability | Step definition file | Permanent merge target |`; no decision changed. | `givn check review` promised-file check |

DESIGN-REVIEW: PASS
