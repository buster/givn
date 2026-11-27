# Design: make-ise-guidance-self-contained

## Scope

A served-guidance and regression-test change. It edits `assets/instructions/ise.md` and adds two delta scenarios with their permanent bindings. It changes no command, flag, exit code, persisted format, report label, `ise-manifest/v1` field, or gate.

## Architecture Impact

| Surface | Change | Owner |
|---|---|---|
| `assets/instructions/ise.md` | Defines `spec copy`, `spec-copy manifest`, `checksum`, `Rebuild impact record`, and `Replaced unit` inline; removes the `docs/reference.md` delegation. | served `ise` instruction |
| `givn/changes/make-ise-guidance-self-contained/specs/fragments/instruction-integrity.feature` | Two added scenarios: the ise-enabled file-reference check and the ise-off served-guidance sweep. | delta spec |
| `tests/steps/instruction_integrity_steps.rs` | Binds the two `Then` steps and includes the startup guidance in the printed-instruction corpus. | permanent step file |

No Component, contract, data model, or runtime branch changes.

## Data Model

None. The change reads no persisted state and writes none.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `instruction-integrity` | `tests/steps/instruction_integrity_steps.rs` | the same file |

## Test Runner, Strict Mode, and Stubs

- Regular scope: `./run-tests.sh --non-e2e`.
- Change scope: `GIVN_FEATURES=givn/changes/make-ise-guidance-self-contained/specs GIVN_CUCUMBER_TAGS='not @e2e and not @givn.removed' cargo nextest run --locked --test features_runner`.
- One scenario: `GIVN_FEATURES=<feature> GIVN_CUCUMBER_TAGS='not @e2e and not @givn.removed' cargo nextest run --locked --test features_runner -E 'test("<feature>::<scenario title>")'`.
- E2E scope (unchanged): `./run-tests.sh --e2e --package e2e`.
- Strict mode: `.fail_on_skipped()` on the cucumber builder; the e2e runner fails a skipped step.
- Not-implemented stub: `unimplemented!("<step text>: no assertion yet")` or an explicit panic, never an empty body, `@wip` until GREEN.

## Decisions

- **Decides D1: define the terms inline.** The served instruction is the consumer surface; no scaffolded `docs/reference.md` is added. Realized in `ise.md` "Vocabulary".
- **Decides D2: one off-path sweep.** A single scenario catches a moved `{{#if addons.ise}}` boundary across every printed instruction and the startup guidance. Realized in the new `Printed guidance omits the opt-in ise rules` scenario.
- **Decides D3: the ise-enabled reference check is a served-text scenario.** The checker runs over the exact `givn instructions ise` output in an ise-enabled consumer project. Realized in the new `The ise instruction's file references resolve in a consumer project` scenario.
- **Decides D4: no behavior change.** The default-off posture, commands, and gates stay unchanged; the change corrects served vocabulary and pins it.
- **Decides D5: this change is `ise rebuild decision No rebuild`.** It edits served guidance and tests and replaces no Source code; no `Rebuilt code` line is recorded.

## Use-Case Traceability

| Guarantee or rule | Technical decision | Evidence |
|---|---|---|
| A served instruction defines the terms it uses | D1 | `The ise instruction's file references resolve in a consumer project` |
| The ise instruction's file references resolve in a consumer project | D3 | the same scenario |
| The default-off guidance omits ise-only rules | D2 | `Printed guidance omits the opt-in ise rules` |
| No command, flag, field, or gate changes | D4 | the full suite and the unchanged permanent scenarios |

## Interaction Coverage Matrix

| Capability | Inventory entry (owning use case) | Real driver | One `@e2e` scenario | Change treatment |
|---|---|---|---|---|
| `instruction-integrity` | resolve an instruction and expose the machine contract | `givn instructions ise` and the reference checker | An agent requests machine-readable artifact guidance | the ise-enabled reference scenario is a variant |
| `instruction-integrity` | request normal session-start instructions | `givn instructions` | Author starts a session with the universal dialog obligation | the ise-off sweep adds the startup guidance to the corpus |

The remaining inventory entries are unchanged from the archived `ise-rebuild-decision-and-working-tree-ship` design (44 rows). No new inventory row is invented; the two scenarios are variants of the two entries above.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable

Terminal text only; no screen is rendered.

### Persona dispositions

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Reads the served instruction in a consumer project. | the two new scenarios |
| `agent--technical-role` | Consumes the served instruction; needs the terms inline. | the ise-enabled reference scenario |
| `givn-maintainer--owner` | Keeps the instruction self-contained and the off path gated. | the ise-off sweep |

## E2E Infrastructure

No new `@e2e` scenario. The delta has two regular scenarios; the existing `instruction-integrity` smoke remains. The e2e package and `verify.e2e_command` are unchanged.

## Black-Box-First

The scenarios drive the built binary and read its stdout; the reference check reads the served text, not the asset file.

## External Boundary Cases

None.

## Structural Model

`docs/domain-model.md` is unchanged: the terms already exist there and the correction is below the structural model.

## Risks and Tradeoffs

- **Duplicate definitions.** The terms also appear in `docs/reference.md` and the glossary; the inline definitions are the consumer-facing source. Keep them short so they cannot drift into a second rulebook.
- **Reference checker scope.** The ise-enabled scenario checks the `ise` instruction; the existing printed-instruction sweep still runs with ise off. Together they cover both modes.
