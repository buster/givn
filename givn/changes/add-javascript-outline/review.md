# Review: add-javascript-outline

Review kind: self-review
Author: Maki
Reviewer context: Maki

README-IMPACT: updated - Description

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Scope

This change adds production JavaScript to the advisory outline report. It adds
`Language::JavaScript`, the `.js`/`.mjs`/`.cjs` discovery rules, the
`JavaScriptAnalyzer` behind the existing `SourceAnalyzer` port, and JSON schema
version 5. JSX and Go remain unsupported. The report stays advisory and
deterministic.

## Implementation evidence check

| Check | Result |
|---|---|
| `@e2e` tag integrity | No delta `@e2e` tag was added or removed. The permanent outline E2E remains; the JavaScript scenarios are regular variants. |
| Empty or trivial step bodies | 0 empty bodies in `tests/steps/outline_report_steps.rs`, the changed step file. The 13 pre-existing no-op Given steps in unrelated step files are not introduced by this change. |
| Checked tasks with commits | All 25 task boxes are checked; each of the nine scenarios records `eecf9262 -- Batch: javascript-adapters`. |
| Promised components exist | `JavaScriptAnalyzer`, `Language::JavaScript`, `tree-sitter-javascript 0.25.0`, schema 5, and the JavaScript discovery/exclusion rules exist. |
| Strict-mode proof | `tests/features_runner.rs` uses `.fail_on_skipped()`. A temporary undefined-step feature exited 100 with `Step doesn't match any function`; the setup task records it. |
| E2E primary assertions | The permanent outline E2E drives the real `givn` binary and asserts CLI output. The new scenarios are regular JSON variants. |
| `verify.e2e_command` target | `./run-tests.sh --e2e` runs `tests/features_runner.rs` with `@e2e and not @givn.removed`; outline E2E steps live in `tests/steps/outline_report_e2e_steps.rs`. No parallel E2E implementation was found. |
| Scope selection | `run-tests.sh` selects `regular` for `--non-e2e` and `e2e` for `--e2e`; the archive receipt carries both scopes. |
| Design deviation | None. The implementation matches the design's adapter, extensions, exclusions, schema 5, step file, and commands. |

### Commit evidence

| Batch | Revision | Contents |
|---|---|---|
| javascript-adapters | `eecf9262` | Dependency, language model, discovery, adapter, analyzer, CLI, schema 5, delta spec, step bindings, adapter tests |
| javascript-documentation-closeout | `55d76b44` | README, reference, index, outline guide, arc42, glossary, change artifacts |
| javascript-task-evidence | `35861ea`, `3d37db66`, `e58f1f0`, `b80a694` | Tasks, verification receipts |
| javascript-review-cleanup | `2b775b4`, `32e1c73` | LCC precondition assertion and JavaScript function-expression test |

### Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `outline-report`: run the advisory report | Present | Existing permanent "A developer runs a outline report and sees the classified units" | Real `givn outline report` subprocess |

The JavaScript scenarios are regular variants of the existing report action.
No new user action was introduced.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| FR-46 supported languages | `docs/arc42/01-introduction-and-goals.md` | Yes | Overview | Batch B | JavaScript scenarios and docs | Yes |
| TC-42 parser set | `docs/arc42/02-architecture-constraints.md` | Yes | Technology decisions | Batch A | `Cargo.toml`, `tree_sitter_adapter` | Yes |
| Outline interface/schema | `docs/arc42/03-context-and-scope.md` | Yes | Data model changes | Batch A | Schema 5 scenario | Yes |
| Code-structure strategy | `docs/arc42/04-solution-strategy.md` | Yes | Overview | Batch B | Report behavior | Yes |
| Adapter building block | `docs/arc42/05-building-block-view.md` | Yes | Module changes | Batch A | `JavaScriptAnalyzer` | Yes |
| Runtime discovery and rendering | `docs/arc42/06-runtime-view.md` | Yes | Module changes | Batches A/B | Discovery and JSON scenarios | Yes |
| Cross-cutting analysis boundary | `docs/arc42/08-crosscutting-concepts.md` | Yes | Overview | Batch B | Adapter and docs | Yes |
| Quality examples Q-112/Q-114 | `docs/arc42/10-quality-requirements.md` | Yes | Risks/quality | Batch B | JSX-only boundary scenario | Yes |
| R-114/R-115 and coverage debt | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Risks | Batch B | Docs | Yes |
| Glossary terms | `docs/glossary.md` | Yes | Decisions | Batch B | Glossary | Yes |
| ADR register | `docs/arc42/09-architecture-decisions.md` | No | Decisions | n/a | No ADR needed; canonical artifacts own the rationale | Yes (unaffected) |
| Deployment view | `docs/arc42/07-deployment-view.md` | No | Local runnability | n/a | Grammar compiled into the existing binary | Yes (unaffected) |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`docs/glossary.md` defines the **Tree-sitter language adapter** for Java,
TypeScript/TSX, and JavaScript, updates **Outline report** to schema 5 and five
languages, and updates **Production source** with JavaScript exclusions. The
design, specs, docs, and code use the same `javascript` language value and
`JavaScriptAnalyzer` term. `docs/domain-model.md` explicitly excludes internal
Rust components and file formats, so this adapter change has no structural-model
impact.

## Coverage

Preflight evidence: `./run-tests.sh --non-e2e` ran 893 tests (893 passed, 104
skipped); `./run-tests.sh --e2e` ran 104 tests (104 passed, 893 skipped). The
delta scope ran 11 scenarios and 56 steps, all passed. The merged Cobertura
report shows line-rate 0.655; `src/outline/tree_sitter_adapter/mod.rs` has 949
lines with 647 covered. The configured project coverage command is line-based;
this change did not remove branch instrumentation. The archive coverage gate is
authoritative.

Classification:

- Dead code: none. The previous `source_language` inference was replaced by an
  explicit language parameter; no dead port or contract member remains.
- Missing test coverage: none. Adapter unit tests cover class, field, method,
  function, arrow binding, `.js`/`.mjs`/`.cjs`, class/function expressions, and
  malformed source. The Gherkin scenarios cover schema 5, mixed-language
  disclosure, malformed files, exclusions, and the JSX boundary.
- Legitimately hard to test: none.

## E2E coverage

The capability's real interface is the CLI. The permanent `@e2e` scenario
"A developer runs a outline report and sees the classified units" covers the
one inventory action: run the report. It drives the real `givn` binary and
asserts stdout. The JavaScript scenarios are variants of that action. A browser
classification does not hold: the report writes terminal text or JSON; it
renders no HTML, CSS, or JavaScript in a browser, so a browser driver is not
applicable.

## Visual review

- Interface: terminal.
- Evidence: the existing permanent E2E scenario runs the real CLI and captures a
  terminal transcript. The new scenarios assert the JSON text and exit behavior.
- Opposite classification: a browser-rendered page would be a Web UI and require
  a browser driver; this capability has no browser surface.
- Persona dispositions: `developer--primary-user` reads the report and gets the
  same classified units with JavaScript included; `agent--technical-role` gets
  the schema 5 language disclosure; `givn-maintainer--owner` gets the documented
  grammar and limitation boundary. No finding.

VISUAL-REVIEW: PASS

## Overlap dispositions

| Scenario A | Scenario B | Decision | Reason |
|---|---|---|---|
| JavaScript production modules are reported for every supported extension | A JavaScript arrow-function binding is reported | variant | Both are language-coverage variants of the report action; they assert different constructs (extension selection versus arrow binding), so merging would lose a distinct failure signal. |
| A malformed Java file does not hide a valid TypeScript file | A malformed JavaScript file does not hide a valid TypeScript file | boundary | Different parser and failure path; the JavaScript parser must be proven independently. |
| Java and TypeScript test and dependency sources are excluded | JavaScript test and dependency sources are excluded | boundary | Different language-specific exclusion rules and file names. |
| Unsupported-only Go production sources are not reported as absent | JSX-only production sources remain unsupported | boundary | Different unsupported extensions; JSX is the explicit boundary retained by this change. |

## Split-or-keep

No delta scenario exceeds the long-scenario threshold.

## Deferrals

None. JSX, Go, import/export graphs, type-aware resolution, and cross-language
resolution are out of scope by the proposal and use case.

## Findings

None remaining. No `@wip` tags remain in the delta feature; no implementation
detail appears in the scenario text beyond observable report fields.

## Sign-off

- [x] Fabrication audit clean; no empty bodies in the changed step file.
- [x] Every checked scenario records `eecf9262 -- Batch: javascript-adapters`; the
      commit contains production code and regression tests.
- [x] Every promised component exists and consumers use the `SourceAnalyzer`
      contract, not the adapter implementation.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded for the delta scope and the full
      regular and e2e scopes.
- [x] Coverage classified; no dead code or missing coverage gap remains.
- [x] Redundant unit tests removed; adapter tests protect distinct mapping cases.
- [x] No `@wip` tags remain; the spec names observable language, unit, field
      count, schema, and failure behavior.
- [x] Canonical E2E policy applied; every inventory action has one `@e2e`
      scenario and variants are regular scenarios.
- [x] The E2E scenario drives the real CLI and asserts its output.
- [x] `verify.e2e_command` and the invoked runner file were identified.
- [x] E2E scope is selected explicitly by `--e2e`/`--non-e2e`; both scopes
      report zero failures and zero skips.
- [x] Implementation matches `design.md`; no silent deviation.
- [x] Interaction coverage verified.
- [x] No finding excused with a classification outside the three buckets.

REVIEW: PASS
