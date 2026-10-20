# Design Review: add-readability-review-gate

Status: all questions resolved; hardening applied. No blocking question remains.

Reviewed 2026-10-20 against `proposal.md`, `design.md`, both delta specs,
`arc42.md`, and the live Arc42 chapters, use cases, ADR records, and README.
This review changes no production code and no archived artifact.

## Decision ledger

| Q | Blocking question | Answer | Resolution |
|---|---|---|---|
| Q1 | How are the new gate scenarios mapped to interaction/E2E coverage? | **A** | The disabled-addon scenario is a configuration variant and becomes a regular scenario; `@e2e` moves to the existing result-helper scenario that already drives the real `givn test-result` subprocess; the matrix row and driver are updated. |
| Q2 | How strong is the mechanical rationale floor? | **B** | The gate deterministically rejects empty/`none`/`n/a` alternatives, a missing rejection clause, evidence without a metric or finding path, a why-section that names no responsibility, and an unconfirmed owner; the owner still judges truth. |
| Q5 | Does this repository dogfood the gate in its own review? | **A** | The change enables `readability`, records `READABILITY: JUSTIFIED module:src/cli/check.rs` and `READABILITY: PASS`, and writes the first rationale; its evidence is refreshed at review time. |
| Q3 | Is the E2E fixture exact enough to build? | Finding | The design now carries the ordered init, addon, base, change-directory, precondition, receipt, review-record, and `check review` sequence. |
| Q4 | Is the output example internally consistent? | Finding | Cohesive is `JUSTIFIED`, Engine is `FIXED`; output sorts by `(path, unit_id)`; singular/plural and `touched` counts are defined; a complete MISSING block shows the marker and rationale path. |
| Q6 | Do the use-case documents parse? | Finding | Every `## Rules`/`## Extensions` bullet is one physical line. |
| Q7 | Do the ADR verdicts match their routing? | Finding | Three verdicts: gate behavior `NOT_QUALIFIED`/`CANONICAL_ARTIFACT`; mandatory review member `QUALIFIED`/`AMEND_ADR` to ADR-028; SCM refinement `QUALIFIED`/`AMEND_ADR` to ADR-071. The "no dependency direction" claim is corrected to name the internal `cli::check -> readability::gate` edge. |
| Q8 | Does the README describe the shipped contract? | Finding | The report/gate bullets describe findings-only schema 2 and the opt-in blocking gate; the review plan records `README-IMPACT: updated - What it is`. |
| Q9 | Does the post-analysis guard apply to deletions? | Finding | It guards the head tree only; a deletion is proven by the base unit's `Removed` finding; a file-deletion scenario covers it. |
| Q10 | Are the core fail-closed cases specified? | Finding | Scenarios and failure variants cover an under-threshold `JUSTIFIED`, an unknown marker, a positive temporary rationale, an unanalyzed changed `.rs` file, the exactly-500 boundary, aggregation, and a slug collision. |
| Q11 | Is the SCM contract exact? | Finding | Fossil uses `--reverse` and the oldest line; `changed_paths` reuses `dirty_entries` plus the revision diff; line-break paths are refused and Git reads with `-z`; Fossil's `parent:` (never `merged-from:`) is the merge base; failure variants and a Fossil merge scenario are added. |

## Ranked resolution log

1. **Q1 (coverage correctness).** `@e2e` removed from "The readability review
   does not run when the experiment is disabled"; added to the permanent
   "The result helper writes four counts without a scope"; matrix row 11 now
   names the CLI driver, row 13 keeps the gate E2E, the disabled variant is
   recorded as a configuration variant, and every shape match involving the new
   E2E scenario gets a `boundary` disposition.
2. **Q2 (rationale authority).** `design.md` DD10 and the rationale section,
   `proposal.md`, the review instruction, Arc42 6.20 and 8.26a now specify the
   deterministic floor: a named alternative with `Rejected:`, concrete evidence,
   the unit's responsibility, a confirmed owner (`givn-maintainer--owner`), the
   expanded whole-word banned list, and the `Follow-up:`/`Expiry:` pair. Two
   scenarios cover a passing concrete rationale and a rejected boilerplate one.
3. **Q5 (dogfood).** `design.md` gains the dogfood section; the rationale lives
   at `readability/rationales/module-src-cli-check-rs.md`; the review plan
   records the marker, `READABILITY: PASS`, and `README-IMPACT: updated - What
   it is`; a review-time task refreshes the evidence.
4. **Q3 (E2E fixture).** The ordered fixture sequence in `design.md` names the
   init, addon-enable commit, base type commit, change-directory commit,
   step/design/tasks/receipt/README/coverage/result-runner preconditions, the
   real `COMMIT:` revision, `givn check tasks`, the review record and rationale,
   and the exact `givn check review` assertion.
5. **Q4 (output contract).** The PASS example has Cohesive `JUSTIFIED` and
   Engine `FIXED`; both blocks sort by `(path, unit_id)`; `1 finding`/`N
   findings` and `touched` are defined; the MISSING block names the marker and
   the rationale path.
6. **Q7 (ADR routing).** `arc42.md` and `design.md` carry the three verdicts;
   `docs/arc42/09-architecture-decisions.md` records the `NOT_QUALIFIED` route
   and the two amendments; the dependency claim is corrected.
7. **Q9 (deletion guard).** DD7, the runtime view, the revision/finding rules,
   and the obstacles table now say the guard inspects head only and a deletion
   is a `Removed` finding; a file-deletion scenario is added.
8. **Q10 (fail-closed coverage).** The gate feature adds the six missing core
   scenarios; `GateFailure` gains `UnknownMarker`, `SlugCollision`,
   `NonUtf8Path`, `NewlinePath`, `UnsupportedUnit`, and
   `SkippedEntryDependency`; `FindingReason` gains `Fixed`.
9. **Q11 (SCM contract).** Fossil base resolution uses `--reverse` and the
   oldest line; `changed_paths` reuses `dirty_entries` plus the revision diff;
   newline paths are refused and Git uses NUL-delimited `cat-file --batch -z`;
   the Fossil merge parent is `parent:`, never `merged-from:`; a Fossil merge
   scenario is added.
10. **Q6 (use-case parsing).** Every Rules/Extensions bullet in both use cases
    is one physical line; `givn spec coverage` parses both documents.
11. **Q8 (README).** The two report/gate bullets now describe findings-only
    schema 2 and the opt-in blocking review gate; the review step records the
    README impact.

## Hardening applied

Gate behavior and design:

- `givn/changes/add-readability-review-gate/design.md`
- `givn/changes/add-readability-review-gate/proposal.md`
- `givn/changes/add-readability-review-gate/arc42.md`
- `givn/changes/add-readability-review-gate/readability/rationales/module-src-cli-check-rs.md`

Specs:

- `givn/changes/add-readability-review-gate/specs/verify-change/readability-review-gate.feature`
- `givn/changes/add-readability-review-gate/specs/assess-code-structure/readability-report.feature`
- `givn/specs/verify-change/givn-workflow-enforcement.feature`
- `givn/specs/verify-change/usecase.md`
- `givn/specs/assess-code-structure/usecase.md`

Durable docs:

- `README.md`
- `docs/arc42/01-introduction-and-goals.md`
- `docs/arc42/02-architecture-constraints.md`
- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/04-solution-strategy.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/09-architecture-decisions.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`
- `docs/arc42/adr/ADR-028-hooks-and-mandatory-checks.md`
- `docs/arc42/adr/README.md`

Mechanical fixes applied in the same files:

- `docs/arc42/06-runtime-view.md` 6.19 renders `classification, evidence`.
- `docs/arc42/09-architecture-decisions.md` cites R-104 through R-108.
- `arc42.md` uses Q-109 and Q-112–Q-117, R-108–R-115, and D-24–D-26, and says
  R-108 is the conditional-compilation limitation (formerly R-97).
- Report support and step tables include `tests/steps/readability_report_support.rs`,
  `assessment_counts` changes with schema 2, and the unused `_action_label`
  helper and `Action` import are deleted in the report work.
- The addon flag changes cover `src/skills/targets.rs` and
  `src/cli/instructions.rs`, with one shared helper as the alternative.
- Gate findings and decisions sort by `(path, unit_id)`; the temp-path sentence
  says "the gate".
- `FAIL` has whole-token boundaries; fenced examples are ignored; duplicate
  `FIXED`/`JUSTIFIED` markers are harmless and conflicting ones fail.
- The test strategy adds the `--e2e` strict RED proof.
- `verify-change` lists all three Personas; the text says "gate finding".
- The report delta keeps the removed/modified titles identical to the permanent
  spec and its added titles use terms the glossary now defines.

## Verification

- `givn lint --change add-readability-review-gate`: exit 2 is expected from
  `@wip`; the run contains only WIP findings and SHAPE advisories. There is no
  syntax error, duplicate title, tag error, or `UCDOC blocking` finding.
- `givn spec coverage`: exit 0; both use-case documents parse.
- The unscoped `givn lint` still reports the expected pre-archive
  `dangling_capability_ref` for the new capability (its feature is merged into
  `givn/specs/verify-change/` at archive) plus the `@wip` findings; neither is a
  design-review blocker and neither is a parser or bullet error.
- `givn check review --change add-readability-review-gate` reaches the Step
  Definitions integrity check and stops on the three gate step files that the
  implementation task creates; the report step files resolve. `givn spec
  coverage` independently proves that both use-case documents parse.
- `git diff --check`: clean.
- Internal consistency: findings-only report schema 2; fix-or-justify gate;
  unit-level accounting; committed `HEAD`; dirty-production guard; base edges;
  ADR-028 and ADR-071 amendments; no new ADR, register count unchanged.

## Remaining implementation-time work (tracked tasks, not review questions)

1. Enable the addon (`givn addons enable readability`) and commit
   `givn/config.yaml`; record the marker and rationale in the change's review.
2. Implement the report delta: remove `Action`, `ClassifiedUnit.action`,
   `ClassifiedUnit.recommendation`, `_action_label`, and the `Action` import;
   bump `SCHEMA_VERSION` to 2; update `assessment_counts` and delete the unused
   recommendation step functions.
3. Implement the gate modules, the five SCM methods, the CLI rendering, and the
   two addon-flag maps.
4. Refresh the dogfood rationale evidence at review time. `module:src/cli/check.rs`
   measures 477 code lines at the design base; if the head stays at or under
   500, record no marker for it (a `JUSTIFIED` marker on a non-finding fails)
   and move the first rationale to the first touched gate finding.
5. Run the self-dogfood sequence from `AGENTS.md` after archiving:
   `cargo install --path . --locked --force`, `givn skills sync`,
   `givn instructions`, `./run-tests.sh`, `./run-tests.sh --e2e`.

DESIGN-REVIEW: PASS
