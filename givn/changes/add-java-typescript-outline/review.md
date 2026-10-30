# Review: add-java-typescript-outline

Review kind: self-review
Author: Maki
Reviewer context: Maki

README-IMPACT: updated - Description

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Scope

This change keeps the Rust and Python analyzers and adds production Tree-sitter
adapters for Java and TypeScript/TSX. It adds the `interface` type kind, advances
the JSON schema to 4, and updates discovery, exclusions, classification, docs,
and scenarios.

## Implementation evidence check

| Check | Result |
|---|---|
| `@e2e` tag integrity | No delta E2E scenario was added or removed; the existing outline E2E remains permanent. |
| Empty or trivial step bodies | 0 empty bodies in the changed step file. |
| Checked tasks with commits | All eight scenarios and closeout tasks record batch revisions. |
| Promised components exist | `tree_sitter_adapter` Java/TypeScript analyzers, `Language::Java`, `Language::TypeScript`, `TypeKind::Interface`, and schema 4 exist. |
| Strict-mode proof | `.fail_on_skipped()` remains. |
| E2E primary assertions | The new scenarios are regular variants; the existing outline E2E still drives the real CLI. |
| `verify.e2e_command` | `./run-tests.sh --e2e`; no scope change. |
| Design deviation | None: Rust/Python adapters are unchanged; Java/TS are composed behind the same port. |

### Commit evidence

| Batch | Revision | Contents |
|---|---|---|
| java-typescript-adapters | `114afae1` | Dependencies, model/scan/report changes, adapters, CLI, scenarios, adapter tests |
| java-typescript-documentation-closeout | `4b3fdf28` | README, outline/reference, arc42, glossary, evaluation outcome, change artifacts |
| java-typescript-task-evidence | `a0adebcd`, `9c2a7d0d`, `545cb626` | Tasks, 0.9.16 note move, verification receipt |

### Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `outline-report`: run the advisory report | Present | Existing outline E2E | Real `givn outline report` subprocess |

The new scenarios are regular variants of the existing report action.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| FR-46 languages | `docs/arc42/01-introduction-and-goals.md` | Yes | Overview | Batch B | Feature scenarios | Yes |
| TC-42 parser set | `docs/arc42/02-architecture-constraints.md` | Yes | Technology decisions | Batch A | Cargo and adapter | Yes |
| Outline interface/schema | `docs/arc42/03-context-and-scope.md` | Yes | Data model changes | Batch A | Schema 4 scenario | Yes |
| Strategy | `docs/arc42/04-solution-strategy.md` | Yes | Overview | Batch B | Report behavior | Yes |
| Adapter building block | `docs/arc42/05-building-block-view.md` | Yes | Module changes | Batch A | `tree_sitter_adapter` | Yes |
| Runtime discovery/schema | `docs/arc42/06-runtime-view.md` | Yes | Module changes | Batches A/B | Scenarios | Yes |
| Cross-cutting analysis boundary | `docs/arc42/08-crosscutting-concepts.md` | Yes | Overview | Batch B | Adapter and evaluation docs | Yes |
| Q-112/Q-114 | `docs/arc42/10-quality-requirements.md` | Yes | Risks/quality | Batch B | Scenarios | Yes |
| R-114 and coverage debt | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Risks | Batch B | Docs | Yes |
| Language adapter/interface terms | `docs/glossary.md` | Yes | Decisions | Batch B | Glossary | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`docs/glossary.md` defines the **Tree-sitter language adapter**, updates
**Outline report** to schema 4 and four languages, and updates **Production
source** exclusions. The design, specs, docs, and code use the same terms.
`interface` is a type kind, not a relabeled class or trait.

## Coverage

The delta feature run covers eight scenarios: Java class, TypeScript class and
function, TypeScript interface, mixed languages, malformed Java, exclusions,
Go-only refusal, and schema 4. Adapter unit tests cover Java methods/fields/
visibility/calls, TypeScript interface/module function, TSX grammar selection,
and malformed Java. The production pipeline remains covered by the existing
outline suite.

## Visual review

The design declares `Interface: terminal`. This change adds no new E2E action;
the existing outline report and its E2E remain. The new scenarios assert the real
CLI JSON output. No browser surface exists.

Persona dispositions:

- `developer--primary-user`: Java/TypeScript units now appear in the same report; no finding.
- `agent--technical-role`: schema 4 and language values are documented; no finding.
- `givn-maintainer--owner`: grammar supply, limitations, and retain-Rust/Python decision are documented; no finding.

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

None. Go, JavaScript, JSX, and Rust/Python replacement are out of scope.

## Findings

None remaining. The permanent-suite failures are covered by delta changes:

- `The JSON report carries the schema and rules versions` — modified to schema 4.
- `Unsupported-only production sources are not reported as absent` — removed; Java is now supported.
- `The Tree-sitter evaluation leaves the current outline engine in place` — removed; the 0.9.15 engine includes Java/TypeScript.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching implementation, tests, or guidance.
- [x] Every promised component exists.
- [x] Every consumer uses only its declared contract.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded for the change scope.
- [x] Coverage classified; new languages and schema are covered.
- [x] Dead code avoided; adapter tests added.
- [x] No `@wip` tags remain; no implementation-layer detail in the spec.
- [x] Canonical E2E policy read; no new E2E action.
- [x] Existing E2E remains real-interface.
- [x] `verify.e2e_command` unchanged.
- [x] Implementation matches design.md: Rust/Python retained, Java/TS Tree-sitter.
- [x] Interaction coverage verified.
- [x] No finding excused with an invented bucket.

REVIEW: PASS
