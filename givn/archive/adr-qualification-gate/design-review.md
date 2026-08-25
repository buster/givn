# Design Review: adr-qualification-gate

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---:|---|---|---|---|
| 1 | Scope | Does the specification cover strict qualification, routing, register migration, source ownership, and both changed CLI consumers without changing runtime behavior? | Keep the change at the authoring/documentation boundary and make every required outcome observable. | Resolved. Proposal, feature scenarios, design, and Arc42 marker now state the same scope; no production runtime behavior is added. |
| 2 | Scope | Does the migration preserve one ADR per independent durable decision rather than merging by subsystem or record count? | Retain independent records and absorb only explicit refinements. | Resolved by decision and the ID-by-ID manifest. Only documented refinements are absorbed; independent boundaries remain separate. |
| 3 | Scope | What happens to non-ADR history and to the incomplete ADR-063? | Relocate useful non-ADR information once or delete redundant records; retain genuinely architectural superseded history; exclude ADR-063. | Resolved. The manifest gives every ID one disposition, reserves 050/051, leaves ADR-063 proposed under `semantic-reranker-budget`, absorbs only named refinements, and forbids non-ADR archive copies. |
| 4 | Tech choices | Is guidance plus structured output sufficient, or should the runtime infer and enforce architectural significance? | Use a structured LLM/author verdict and human review; keep runtime and integrity hooks non-semantic. | Resolved. The design explicitly rejects runtime qualification enforcement and records the current integrity-hook limitation. |
| 5 | Missing scenarios | Does the guidance contract cover each mandatory dimension and route without requiring an imaginary evaluator? | Assert the canonical guidance clauses, strict AND semantics, and support-only behavior directly. | Resolved. The feature tests the five dimensions and routing as guidance/source contracts; it does not simulate candidate evaluation. |
| 6 | Missing scenarios | Are structured fields, allowed values, evidence provenance, amendment, supersession, Arc42-only impact, register ownership, and guidance parity observable? | Define exact fields and enum values and assert file/status/output consequences. | Resolved. The feature asserts all verdict fields and values, no invented evidence, amendment versus supersession, Arc42 routing, register/source separation, and effective/generated parity. |
| 7 | Testability | Can every scenario fail in RED with concrete assertions? | Keep step text at the domain boundary, assert exact statuses/values/paths, use `fail_on_skipped`, and retain `unimplemented!()` until GREEN. | Resolved. The design names the Cucumber strictness mechanism, anchored single-scenario command, and concrete Then contracts; all new scenarios remain explicitly `@wip`. |
| 8 | E2E fidelity | Does each E2E scenario use the real consumer interface and the configured runner rather than a lower-level substitute? | Invoke the compiled CLI subprocess through `./run-tests.sh --e2e`, assert stdout and exit status first, and use no external service. | Resolved. The matrix names CLI/terminal and a real subprocess for both inventory entries; the E2E command and primary assertion rule are explicit. |
| 9 | Interaction Coverage | Does every normalized inventory entry have exactly one distinct E2E scenario and a valid driving mechanism? | Treat Arc42 guidance and design-review guidance requests as two distinct CLI actions. | Resolved. The feature has two inventory entries and two matching `@e2e` scenarios; the matrix has two corresponding rows. |
| 10 | Risk | What is the most likely implementation failure? | Drift between the canonical instruction, generated mirrors, effective overrides, and register migration is the primary risk. | Resolved. Source ownership, parity scenarios, the deterministic manifest, and an explicit no-semantic-integrity-enforcement boundary are recorded; the current source/register updates are included in the working tree. |
| 11 | Architecture documentation | Which Arc42 rows are affected when no runtime component changes? | Mark documentation-quality goal, repository authoring constraint, changed existing CLI guidance, cross-cutting policy, register, quality, risk, and glossary rows Yes; mark strategy, building blocks, runtime, and deployment No. | Resolved. The marker and design mapping agree on Yes for 1, 2, 3, 8, 9, 10, 11, 12 and No for 4, 5, 6, 7, with a concrete update for every Yes row. |
| 12 | Architecture documentation | Do permanent chapter and register files now contain the policy described by the design? | Verify the actual current files and do not claim completion from the marker alone. | Resolved. The affected durable chapters, register, retained ADR verdicts, and source guidance are updated in the working tree and are verified below. |
| 13 | Architecture documentation | Are all twelve chapter files present, substantive, and free of prohibited ASCII diagrams? | Verify the complete fixed chapter set independently. | Resolved by codebase exploration. All twelve files exist and contain substantive content; the scan found no box-drawing characters or non-table ASCII diagrams. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Workflow state | `givn status --change adr-qualification-gate --json` reports proposal, specs, design, arc42-docs, and design-review done, with no `tasks.md` yet. |
| Arc42 enablement | `givn/config.yaml` has `addons.arc42: true`; the Arc42 branch is applicable and the fixed `docs/arc42/` path is the durable output boundary. |
| Runner and strictness | `givn/commands.yaml` supplies `./run-tests.sh` and `./run-tests.sh --e2e`; `tests/features_runner.rs` calls `fail_on_skipped()`. |
| Existing E2E fixture | `tests/steps/instruction_integrity_e2e_steps.rs` owns the generic phrase and hardcoded `instruction-integrity` change. The design now requires a capability-specific `adr_qualification_gate_e2e_steps.rs` fixture with `adr-qualification-gate`. |
| E2E primary boundary | The shared fixture helper invokes the compiled binary and captures `Output`; the new matrix therefore uses child stdout and exit status as primary assertions, with filesystem setup only as secondary evidence. |
| ADR-063 status | `docs/arc42/adr/ADR-063-reranker-pair-budget.md` is `proposed`, and the active `semantic-reranker-budget` change is incomplete. The migration explicitly excludes it. |
| Permanent documentation state | All twelve `docs/arc42/` chapters are substantive; affected chapters, the ADR register, retained ADR verdicts, and canonical guidance are updated in the working tree. |
| Integrity behavior | Existing Arc42 integrity behavior checks resolvable files and hashes; it does not semantically determine qualification or validate the register. The design records this as a boundary and risk rather than claiming enforcement. |

## Open questions

| # | Question |
|---:|---|
| - | None. Every grilling finding is resolved by the supplied decisions or by an explicit artifact contract; durable source, chapter, index, and ADR edits are present in the working tree. |

## Architecture documentation (arc42) check

The Arc42 addon is enabled. The reviewer's Yes/No assessment was independently
derived from `proposal.md`, `specs/**/*.feature`, `design.md`, and the chapter
selection table before comparing it with `arc42.md`.

| # | Chapter | Grilling subagent's own Yes/No | arc42.md's Yes/No | Match? | Content matches design.md? |
|---:|---|:---:|:---:|:---:|---|
| 1 | 01 introduction-and-goals | Yes | Yes | Yes | The documentation-quality goal and acceptance ownership are present in the updated chapter. |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | The chapter delegates the strict gate to the canonical instruction and states one-decision, single-source, and non-runtime constraints. |
| 3 | 03 context-and-scope | Yes | Yes | Yes | The changed `givn instructions` guidance surface and source cascade are represented without adding a runtime interface. |
| 4 | 04 solution-strategy | No | No | Yes | Current strategy remains consistent; no chapter update is required. |
| 5 | 05 building-block-view | No | No | Yes | No production building block changes; current substantive chapter remains consistent. |
| 6 | 06 runtime-view | No | No | Yes | No runtime sequence changes; current substantive chapter remains consistent. |
| 7 | 07 deployment-view | No | No | Yes | No deployment or dependency changes; current substantive chapter remains consistent. |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Section 8.15 identifies the canonical qualification source, derived guidance, override behavior, and non-runtime boundary. |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | The pointer/register and ADR index now reflect the migration; retained ADRs carry the qualification verdict and deleted records are absent. |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Q-41 through Q-47 cover the strict gate, routing, source ownership, migration, and parity. |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | R-23 through R-26 and D-13 cover inflation, migration loss, source drift, and non-semantic enforcement. |
| 12 | 12 glossary | Yes | Yes | Yes | The glossary defines candidate, verdict, mandatory dimension, canonical artifact, routing, amendment, supersession, and single-source terms. |

- [x] All 12 rows were assessed independently against the proposal/spec/design before comparing `arc42.md`.
- [x] No reviewer-Yes / marker-No or omitted row exists.
- [x] Every planned non-trivial architecture decision has an explicit migration disposition; no new ADR is proposed for this policy.
- [x] Superseded architectural records have explicit replacement targets; non-ADR records are not retained as ADR archive history.
- [x] All twelve chapter files exist, contain substantive content, and contain no prohibited ASCII art diagrams; Yes-row permanent content changes are present and match the design.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/adr-qualification-gate/proposal.md` | Replaced permissive/open-question language with strict AND qualification, support-only `Must be shared`, canonical single source, independent-ADR granularity, default-policy override boundary, and resolved decisions including ADR-063 ownership. |
| `givn/changes/adr-qualification-gate/specs/adr-qualification-gate/adr-qualification-gate.feature` | Added concrete failure/routing, strictness, verdict enum, evidence provenance, amendment/supersession, Arc42-only impact, register/source ownership, parity, and two distinct CLI E2E scenarios with two inventory entries. |
| `givn/changes/adr-qualification-gate/design.md` | Added source ownership, strict structured verdict contract, no-runtime-enforcement boundary, complete ADR-001 through ADR-065 manifest including reserved IDs and ADR-063 exclusion, one-decision-per-ADR treatment with explicit refinement-only absorption, corrected runner command, unique E2E fixture plan, two-row interaction matrix, Arc42 chapter mappings, and the implemented migration scope. |
| `givn/changes/adr-qualification-gate/arc42.md` | Corrected all twelve impact rows, supplied concrete row-1-to-3 reasons, separated chapter impact from `ADR action: NONE`, and recorded the implemented durable updates. |
| `givn/changes/adr-qualification-gate/design-review.md` | Recorded the fresh grilling outcomes, codebase resolutions, independent Arc42 audit, migration/source hardening, and sign-off. |

The live guidance sources and generated mirrors are aligned in the working tree.
`assets/addons/arc42-docs/instructions.md` owns the complete MADR template and
qualification procedure, `assets/addons/arc42-docs/template.md` remains
change-level impact-only, and design-review/specs/Arc42 assets are derived or
artifact-specific guidance without competing with the canonical procedure.

## Sign-off

- [x] All required branches were walked.
- [x] All grilling findings and open questions are resolved.
- [x] `design.md` reflects the decisions reached.
- [x] `specs/**/*.feature` contains concrete scenarios for the identified gaps.
- [x] `givn lint --change adr-qualification-gate` completed with the expected WIP exit (`2`) and no parse error.
- [x] Arc42 assessment is complete, internally matches `arc42.md`, and all
      affected permanent chapter, source, register, and ADR updates are present
      and checked.

DESIGN-REVIEW: PASS
