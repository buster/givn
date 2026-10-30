# Review: evaluate-tree-sitter-outline

Review kind: self-review
Author: Maki
Reviewer context: Maki

README-IMPACT: none

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Scope

This change records a differential Tree-sitter evaluation for Rust and Python,
adds a test-only evaluation module, and retains the current adapters because
required facts are missing. It does not switch the default engine, add a
language, or change the outline command.

## Implementation evidence check

| Check | Result |
|---|---|
| `@e2e` tag integrity | No delta E2E scenario was added or removed; the existing outline E2E remains permanent. |
| Empty or trivial step bodies | 0 empty bodies in the changed step file. |
| Checked tasks with commits | The scenario and closeout tasks record batch revisions. |
| Promised components exist | `src/outline/tree_sitter_eval/`, the dev dependencies, the decision record, and the regression scenario exist. |
| Strict-mode proof | `.fail_on_skipped()` remains the runner's strict mode. |
| E2E primary assertions | The changed scenario is regular; the existing outline E2E still drives the real CLI. |
| `verify.e2e_command` | `./run-tests.sh --e2e`; no scope change. |
| Design deviation | None: Tree-sitter is test-only and the default engine is unchanged. |

### Commit evidence

| Batch | Revision | Contents |
|---|---|---|
| tree-sitter-evaluation | `1477d19f`, `2759cd30` | Dev dependencies, evaluation module, decision record, regression scenario |
| tree-sitter-documentation-closeout | `9912d5ed` | arc42 chapters, glossary, proposal/design/design-review/arc42/tasks |
| tree-sitter-task-evidence | `3c077300`, `fcee6cc0` | Task evidence and verification receipt |

### Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `outline-report`: run the advisory report | Present | Existing outline E2E | Real `givn outline report` subprocess |

The new scenario is a regular variant of the existing action and does not add
an E2E.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Test-only evaluation in 8.26 | `docs/arc42/08-crosscutting-concepts.md` | Yes | Overview | Batch A | Evaluation record and module | Yes |
| Grammar/toolchain and parity risks | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Risks | Batch B | R-130/R-131 | Yes |
| Tree-sitter evaluation term | `docs/glossary.md` | Yes | Decisions | Batch B | Glossary entry | Yes |
| Default outline report unchanged | Regression scenario | Yes | Use-case traceability | Batch A | JSON schema/languages scenario | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`docs/glossary.md` records **Tree-sitter evaluation** as a test-only
differential comparison, not a production engine or parity claim. The
evaluation record, design, specs, and tests use the same term. The existing
outline vocabulary is unchanged.

## Coverage

The delta feature run covers the unchanged default outline report. The
test-only evaluation is covered by four unit tests:

- every required category is present;
- missing required facts produce `RETAIN`;
- the decision record matches the matrix;
- malformed input is recorded as an error;
- the default analyzer still supports Rust and Python.

The release binary does not link Tree-sitter, so production coverage is not
diluted by test-only code.

## Visual review

The design declares `Interface: terminal`. This change adds no new E2E
scenario and no new visible surface; the existing outline report and its E2E
remain. The regression scenario asserts the real CLI JSON output. No browser
surface exists.

Persona dispositions:

- `developer--primary-user`: the outline report is unchanged; no finding.
- `agent--technical-role`: the report contract and schema are unchanged; no finding.
- `givn-maintainer--owner`: the retain decision and dev-only scope are recorded;
  no finding.

VISUAL-REVIEW: PASS

## Overlap dispositions

No shape-match finding involving this change requires a disposition.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| | | |

## Split-or-keep

No delta scenario exceeds the long-scenario threshold.

| Scenario | Decision |
|---|---|
| | |

## Deferrals

None. Replacement and additional languages are explicitly out of scope, not
deferred checks.

## Findings

None remaining.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching implementation, tests, or guidance.
- [x] Every promised component exists.
- [x] Every consumer uses only its declared contract.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded for the change scope.
- [x] Coverage classified; the evaluation is test-only and does not dilute production coverage.
- [x] Dead code avoided; tests cover the evaluation and default engine.
- [x] No `@wip` tags remain; no implementation-layer detail in the spec.
- [x] Canonical E2E policy read; no new E2E action.
- [x] Existing E2E remains real-interface.
- [x] `verify.e2e_command` unchanged.
- [x] Implementation matches design.md: test-only dev dependencies.
- [x] Interaction coverage verified.
- [x] No finding excused with an invented bucket.

REVIEW: PASS
