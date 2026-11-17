# Design review: align-ise-rules-with-passuff-evidence

Review kind: independent
Author: align-ise-rules-with-passuff-evidence author
Reviewer context: independent-design-review-2026-11-17

Reviewed: working tree at `95b83225` with the uncommitted
`givn/changes/align-ise-rules-with-passuff-evidence/` directory.

| Artifact | sha256 (first 16) |
|---|---|
| `design.md` | `ae9469e7bf47a4b4` |
| `proposal.md` | `7c753168bc44a60a` |
| `arc42.md` | `db2e40abf130aa55` |
| `specs/fragments/instruction-integrity.feature` | `fbcc7dab17479e1c` |
| `specs/create-change/prose-consolidation.feature` | `449fbd82c9e3256c` |

The two remaining findings are fixed: G1 adds the schema/migration class to both
`Then` steps and both `New Then` rows, and A1 records the carried decision's
default, trigger, switch cost, and owner. No blocking finding remains.

## G1 verification

The feature's first scenario now has:

```gherkin
And the served instruction names the build manifest, the lockfile, the toolchain pin, a structural template, a schema or migration asset, and served instruction text as build inputs
And the served instruction places the build manifest and the lockfile under contracts, the toolchain pin under limits, a structural template under structure, a schema or migration asset under contracts, and served instruction text under behavior
```

The `New Then` rows carry the same text. Row 1 asserts `a schema or migration
asset`; row 2 asserts the four role keys beside the named classes, including
the schema/migration class under `contracts`. The design's exact instruction
text names `a schema or migration asset under contracts` in the same bullet.

A reader that omits the schema/migration sentence fails both: the first
`Then`'s binding asserts `a schema or migration asset`, and the second `Then`'s
binding asserts the named classes beside `contracts`; the omitted sentence
removes the class from both. The scenario declares six steps; the first new
`Then` is the earlier failure, so the observed single-scenario summary stays
`4 steps (3 passed, 1 failed)` and exit `101`.

## A1 verification

`Decides D2` now records the carried decision in full:

- **Default:** the five roles carry the build inputs, with no `build:` list.
- **Revisit trigger:** an input whose content fits no content role, for example
  a binary-only tool.
- **Switch cost:** a loader key, the unknown-key refusal text, a second copy
  path, `givn ise compare` and archive-guard cases, and new declaration
  scenarios.
- **Owner:** `givn-maintainer--owner`.

The default, trigger, switch cost, and owner satisfy the decision-timing
requirement; the current rule remains the five-role content-class mapping.

## Previous blockers re-checked

F1: the rule reads "A build input the declaration leaves outside the
Specification roles is a `specification defect`", consistent with the spec-copy
rule, the `Specification files` inventory, and `givn ise compare`. F2: both
permanent bodies are written in the implementation batch; the full
`verify.command` is green before the review, and the archive merge replaces the
two bodies with identical text. F3: the generated-asset `Then` asserts `whose
bytes no frozen test fixes`; the dependency-cache `Then` asserts `blocks the
rebuild`, `the blocked rebuild is a setup failure, not a Spec gap`, and `not a
specification defect`. D1-D4: the content-class principle, the observed 4-step
summary, the setup-failure classification, and the `Build input` qualifier are
present. B1-B6 and A1-A10 from the earlier reviews remain fixed.

## Role mapping and rule-set consistency

| Build input | Role home | Consistency |
|---|---|---|
| Build manifest, lockfile, schema/migration asset | `contracts` | The role is a path list; a declared input is a Specification file; the copy carries it and the manifest/compare checksum it. "Contracts and schemas" owns the dependency, build-target, and schema content. |
| Toolchain pin | `limits` | "Technical limits" owns a pinned toolchain. |
| Structural template (`assets/templates/domain-model.md`, `glossary.md`) | `structure` | "Vocabulary and structural model" owns the generated structural model. |
| Served instruction/skill text (`assets/instructions/**`, `assets/skills/**`) | `behavior` | "Behavior and invariants" owns the served text the frozen tests assert. |
| E2E package's own manifest/lockfile | `e2e` for the test build | The served text excludes it from the product build. |

The four spec-copy sentences and the manifest/compare semantics hold: a declared
build input is carried and checksummed; a build input the declaration leaves
outside the roles is the `specification defect`; the copy omits it as `Outside
the spec copy` (or removes it inside a source root); the neighbor record covers
files outside the Specification or the replaced unit; a generated asset inside
the replaced unit whose bytes no frozen test fixes is an acceptable
undocumented change; the environment inputs are supplied and recorded, and a
blocked rebuild is a setup failure, not a `Spec gap`.

## Grilling branch coverage

| Branch | Outcome |
|---|---|
| Scope | Six lessons map to the six rules; the schema/migration class is now asserted. No issue. |
| Tech choices | Guidance-only; the single-scenario command, the count guard, and the observed 4-step summary are concrete. No issue. |
| Missing scenarios | The six build-input classes are asserted; the generated/environment boundaries are asserted. No issue. |
| Testability | Every new `Then` has a binding row; the boundary clauses are asserted; the observed failure is recorded. No issue. |
| E2E fidelity | No new action; existing `@e2e` scenarios; no tag removal. No issue. |
| Interaction Coverage | Four matrix rows match the use-case inventories; no row added. No issue. |
| Risk | The declaration-actor rule, the permanent-body route, and the classification boundaries are all addressed. No issue. |
| Use-case and Persona context | Use cases and the two contracts are complete; Persona evidence is present. No issue. |
| Decision ledger | D1-D8 resolve; D2 records the carried decision fully. No issue. |
| Contracts and components | Two contracts, two components. No issue. |
| Decision timing | D2 has default, trigger, switch cost, and owner. No issue. |
| change strategy | D1 records one `incremental modification`; no rebuild. No issue. |
| Visual design contract | `Interface: none`; three confirmed Personas. No issue. |
| Deferrals | No `.givn-skip`, no deferral. No issue. |
| Persona disposition | Evidence per Persona. No issue. |
| Interface classification | CLI output; the opposite browser classification does not hold. No issue. |
| ADR qualification | No `NEW_ADR`; canonical homes are the instruction, reference, and glossary. No issue. |
| Architecture documentation | `arc42.md` row 12 Yes; row 2 corrected; row 11 records the permanent-body route. No issue. |
| Ubiquitous language | `Build input`, `Neighbor record`, `Replaced unit`, `Change verification receipt`; American spelling recorded. No issue. |
| Structural model | Unchanged; the three terms are process records. No issue. |

No blocking findings remain. The design is ready for `tasks.md`.

DESIGN-REVIEW: PASS
