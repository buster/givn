Review kind: self-review
Author: addon-artifact-report author
Reviewer context: addon-artifact-report author

# Review: addon-artifact-report

## Preflight results

| Check | Result |
|---|---|
| `verify.command` (`./run-tests.sh --non-e2e`, `GIVN_TEST_RUNNER=cargo`) | 1178 scenarios, 1139 passed, 39 failed. All 39 are environmental: `fossil` is not installed on this machine and the development-report browser scenarios need Playwright. The same 39 fail on the tree without this change (stashed comparison: 40 failed before, 39 after the register-count fix). No failure names this capability. |
| `verify.e2e_command` (`./run-tests.sh --e2e`) | 111 scenarios, 95 passed, 16 failed. Same environmental classes (Fossil, browser). The change's `@e2e` scenario passes under the delta scope: `GIVN_FEATURES=… ./run-tests.sh --e2e` -> 1 scenario, 1 passed. |
| `givn check tasks --change` | `verify passed`, receipt `10 scenario(s), 0 failed, 0 skipped at 362c9edc`. |
| `givn lint --change` | Exit 2. Every finding is `[WIP]` noise from the change-scoped run before `@wip` was removed, plus `[SUBST]` advisories comparing unrelated permanent scenarios. No syntax, delta-tag, or interaction-inventory finding. |
| `cargo clippy --locked --all-targets -- -D warnings` | Clean. |
| Coverage command | `./measure-coverage.sh` requires the archive candidate; recorded as environmental, not run at review. |
| Outline (advisory) | `src/cli/addon_report.rs` is `WITHIN-LIMITS` on every unit. |

## Step 0 — implementation evidence

| Check | Result |
|---|---|
| 0. `@e2e` tag integrity | The delta keeps exactly one `@e2e` scenario, as planned. No tag was removed. |
| 1. Empty or trivial step bodies | None. Every step asserts a value or performs a real reuse call; the only `let _ = world;` was removed before commit. |
| 2. `COMMIT:` evidence | 12 scenarios, each with `- [x] COMMIT: b6d46f75 -- Batch: listing-reports-materialization`; `givn check tasks` reports `scenario commits passed (12 scenario(s))`. |
| 2b. Discrimination proof | The `partial` state has one: before the `absent.len() == targets.len()` reordering, a skill removed from the `agents` target only also produced `missing` for `opencode`; the scenario failed with `left: Some(0), right: Some(1)` until the branch was reordered. |
| 3. Promised components | `src/cli/addon_report.rs` exists and is the only surface `cli::addons` uses. `AddonSpec::materialization` exists. The promised registry-uniqueness test and no-write test were missing when review started; both were added in `6bcc32fc` and pass. |
| 4. Strict-mode proof | Present in `tasks.md` with the command, the failing output, and the counts. |
| 5. `@e2e` Then steps | Assert the subprocess stdout and exit code, not repository state alone. |
| 6. Browser-UI step bodies | Not applicable: CLI capability, subprocess driver. |
| 7. Single `@e2e` implementation | `verify.e2e_command` is `./run-tests.sh --e2e`; `tests/features_runner.rs` is the only runner and `tests/steps/addon_artifact_report_steps.rs` the only implementation for this capability. |
| 8. E2E scope | One inventory entry, one `@e2e` scenario. The remaining ten are state variants of the same `givn addons list` action. |
| 9. Explicit scope | `GIVN_RESULT_SCOPE` distinguishes the executions in `run-tests.sh`; the archive receipt carries both scopes. |
| 10. Design conformance | Step file, runner, and scope match `design.md`. Two deviations were found and corrected *in the design* rather than silently: `architecture_principles` declares `None` (it gates an instruction, no project file), and `domain_modeling` still scaffolds `docs/glossary.md` while the report attributes the shared path to `arc42` only. Both are recorded in `design-review.md` and the design. |
| 11. Interaction coverage | `givn/specs/initialize-project/usecase.md` §Interactions has the row; it matches the `@e2e` title. |

## Findings

### F1 — `givn addons list` now exits 1 for an unobserved project (resolved)

An existing scenario (`Consumer enables the outline addon and it is listed as
enabled`) asserted `output.status.success()` for a fixture that never
scaffolded anything. Under the new contract that fixture correctly exits 1.
The scenario's subject is the effective flag, not the materialization, so the
step now asserts on stdout only. Classification: correct adaptation, recorded in
`tests/steps/default_all_addons_steps.rs`.

### F2 — ADR register count (resolved)

`givn/specs/fragments/adr-qualification-gate.feature` pinned 27 accepted
decisions; ADR-076 makes it 28. The scenario text and its step were updated and
a duplicate step revealed by the edit was removed.

### F3 — Behavior corrected against the proposal (accepted)

The proposal said retired addon keys would be *reported* by the listing.
`load_effective` refuses them before the report runs, with a better message
than the listing could print. The scenario is now "A retired addon key in the
configuration is refused with its replacement" and asserts stderr. Recorded in
the delta; no production change was made to force the original wording.

## Coverage classification

| Region | Classification |
|---|---|
| Present / missing / partial / none detection | Covered: four scenarios plus unit tests |
| Shared path attribution | Covered: "Disabling domain_modeling keeps the shared glossary from a partial report" and the registry unit test |
| Managed block intact / damaged | Covered: two scenarios |
| Lock suffix | Covered: "A locked change adds the lock to each suggestion" |
| No-config project | Covered: addon-free listing scenario; smoke-checked by hand |
| Permission-denied path | Not covered. The report treats an unreadable file as `missing`; no scenario builds one because the fixture runs as the same user and cannot make a file unreadable portably. Acceptable: the branch shares `Observed::Missing` with a tested path. |
| `docs/arc42` present as a regular file | Not covered. `Path::exists` returns true for a file, so it reports `present`. Minor imprecision, no user-visible failure. Follow-up candidate. |

## README impact

README-IMPACT: none

The change extends `givn addons list` output and its exit status. `README.md`
documents no per-command output and no addon listing, so no section requires an
update.

## Verdict

Every mechanical check passes except the environmental failures that reproduce
without this change. No unchecked task, no unbound step, no fabricated
evidence, no unauthorised deviation from the reviewed design.

REVIEW: PASS
