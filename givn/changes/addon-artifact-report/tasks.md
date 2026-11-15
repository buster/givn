# Tasks: addon-artifact-report

Release note (carried as the subject of every batch commit):

> `givn addons list` reports for each addon and each agent target whether its materialized files are present or missing, and prints the literal command that resolves the difference.

Use-case constraints carried into every task: the owning use case is
`initialize-project`; the capability is `EXTEND enable-optional-addons`; the
developer Actor runs `givn addons list` and expects the effective addon set and
its observed filesystem effect. Generated skill directories live at
`<target>/skills/<name>/SKILL.md` for `opencode`, `agents`, `claude`, `omp`,
`maki`. No task may add a write path, a `--fix` flag, or an operator override.

---

## Batch 1 — Listing reports materialization state

### Task 1.1: Contract and report core

- [x] Implement `AddonSpec::materialization` and the `src/cli/addon_report.rs` contract.
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization

**Design constraints**
- Registry table is authoritative; declare exactly these values:
  `arc42` -> `Files(["docs/arc42", "docs/glossary.md"])`; `communication_policy` -> `ManagedBlock` in `AGENTS.md` with the `givn:begin:communication-policy` / `givn:end:communication-policy` markers; `readme` -> `Files(["README.md"])`; `coverage` -> `None`; `domain_modeling` -> `FilesAndSkill { files: ["docs/domain-model.md"], skill: "givn-domain-modeling" }`; `dev_principles` -> `FilesAndSkill { files: [], skill: "givn-dev-principles" }`; `challenge` -> `FilesAndSkill { files: [], skill: "givn-challenge" }`; `architecture_principles` -> `None` (instruction-gated in `src/cli/instructions.rs:15`, no project file); `outline` -> `None`.
- `scaffold_enabled` (`src/cli/addons.rs:142`) dispatches on `materialization`, not on a name match. Existing user-facing messages keep their current text.
- Output keeps the existing `  [enabled ]  <id>` / `  [disabled]  <id>` lines byte-for-byte and appends `              artifacts: <present|missing|partial|none>` plus `              suggestion: <literal command>`.
- States: `present` (all declared paths and managed block intact), `missing` (any absent), `partial` (skill present in some but not all selected targets, with the missing target paths named in parentheses), `none` (no declared path).
- `run_list` returns whether any enabled addon is `missing`/`partial`; `main.rs` maps it to exit 1, otherwise 0.
- Contract name: `AddonMaterializationReport`; single implementing component `src/cli/addon_report.rs`.
- Reuse `config::overlay::load_effective`, `config::overlay::load_skills_targets`, `config::manifest::Addons::entries`, and `Addons::replacement_for_retired`.
- A unit test asserts no declared path or skill appears under two registry entries.

Scenarios: none directly; enables Task 1.2.

---

### Task 1.2: Report an enabled addon's materialized files

- [x] Scenario: "Consumer lists addons and sees each addon's materialized files reported"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "An enabled addon with missing files is reported and the exit status is non-zero"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "An enabled addon whose effect is a managed block reports the block state"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "A damaged managed block for an enabled addon fails the command"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization

**Design constraints**
- Step file: `tests/steps/addon_artifact_report_steps.rs` (permanent home; the delta adds no separate copy).
- `artifacts:` line text is `<state>` alone for `present`/`none`, and `<state> <path>` for `missing`, where `<path>` is the first absent declared path (or the damaged block's file for a managed block).
- Managed block is intact only when both markers are present exactly once and the begin precedes the end.
- Every finding prints `suggestion:` on the next line; the literal commands are `givn addons enable <id>` for an enabled addon with `missing` artifacts.
- Exit 1 only for an enabled addon that is `missing` or `partial`.

---

### Task 1.3: Report differences in the other direction

- [x] Scenario: "Files present for a disabled addon are reported without failing the command"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "An enabled skill addon missing its skill from a selected target is reported as partial"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "A retired addon key in the configuration is refused with its replacement"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "Disabling domain_modeling keeps the shared glossary from a partial report"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization

**Design constraints**
- Disabled addon whose declared files exist: `artifacts: present`, `suggestion: givn addons disable <id>`, exit 0.
- `partial` prints `artifacts: partial (<missing target path> missing)` and `suggestion: givn skills sync`; exit 1.
- Retired keys are read from `givn/config.yaml` `addons:` and matched against `Addons::RETIRED`; print `  [retired ]  <key>  (replaced by <replacement>)` and `              suggestion: replace `addons.<key>` with `addons.<replacement>` in givn/config.yaml`; exit 0; no config write.
- `domain_modeling` declares only `docs/domain-model.md`, so a retained `docs/glossary.md` never makes it `partial`.

---

### Task 1.4: Report writes nothing, respects the lock, and handles a bare project

- [x] Scenario: "The listing writes nothing to the project"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "A locked change adds the lock to each suggestion"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization
- [x] Scenario: "An addon-free project listing reports every addon explicitly disabled"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization

**Design constraints**
- `addon_report.rs` contains no `std::fs::write`, `remove_*`, or `set_addon` call; a unit test greps the module source for those symbols.
- Lock suffix: when `config::lock::locked_active_changes` returns entries, append `  (refused while change <ids> holds the add-on lock)` to every suggestion line. The report never refuses and never writes the lock.
- Addon-free project: every registered addon prints `[disabled]` and `artifacts: none` where nothing was ever scaffolded; exit 0.

---

## E2E setup task

- [x] Prove strict mode and record the e2e scope selection.
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization

**Design constraints**
- Runner: existing `tests/features_runner.rs` with `.fail_on_skipped()`.
- `verify.e2e_command` per `givn/commands.yaml` (`./run-tests.sh --e2e`), scope `GIVN_RESULT_SCOPE=e2e`.
- Strict-mode proof: with the new steps undefined, the e2e run exits non-zero; paste the command and the non-zero exit into this task. No local server or browser is required; the interface is a subprocess.

**Evidence.** Binding one Then phrase under a different name made the suite
reject the scenario instead of skipping it:

```
$ GIVN_TEST_RUNNER=cargo GIVN_FEATURES=givn/changes/addon-artifact-report/specs ./run-tests.sh
   ✘  And the project files are unchanged
      Step doesn't match any function
11 scenarios (10 passed, 1 failed)   -> exit 1
```

Restored, the delta runs green in both scopes:

```
$ GIVN_TEST_RUNNER=cargo GIVN_FEATURES=givn/changes/addon-artifact-report/specs ./run-tests.sh
11 scenarios (11 passed)

$ GIVN_TEST_RUNNER=cargo GIVN_FEATURES=givn/changes/addon-artifact-report/specs ./run-tests.sh --e2e
1 scenario (1 passed)
```

E2E scenario count (1) is strictly less than the full count (11).

---

## Batch 2 — `@e2e` smoke

### Task 2.1: Consumer runs `givn addons list`

- [x] Scenario: "Consumer lists addons and sees each addon's materialized files reported"
- [x] COMMIT: de8706da -- Batch: listing-reports-materialization

**Design constraints**
- Real interface: `common::run_givn(env, &["addons", "list"])`, asserting stdout and the exit code.
- Runs only under `verify.e2e_command`; must not replace the regular coverage of the same command in Batch 1.

COMMIT: de8706da -- Batch: listing-reports-materialization
