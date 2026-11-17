# Proposal: align-ise-rules-with-passuff-evidence

## Release note

The ise rules now require the Specification to cover every build input and every value a frozen-test contract fixes, keep the acceptance scenarios in the Specification, quarantine a published product copy before a rebuild, record every neighbor and consolidation, and leave a dry run unpromoted.

The change edits served guidance, reference rows, glossary terms, and the scenarios that assert them. It adds no command, output format, exit code, persisted state, declaration key, or verification behavior. The behavior under test is the served text, so the delta scenarios are the executable specification for this change.

## Use-Case Context

- Use-case ID: `obtain-effective-guidance` and `create-change`. The `instruction-integrity` capability is owned by the `support` fragment both use cases include; `prose-consolidation` belongs to `create-change`, and the `prepare-an-ise-test` contract owns the spec-copy reference section that gains the rule rows.
- Ideation topic: `ise-rebuildability`.
- Seed path: none. The change incorporates archived trial evidence, not an ideation change seed.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

## Review Context

- Review kind: independent
- Reason: the operator requested an independent design review. The change touches two capabilities (`instruction-integrity`, `prose-consolidation`) and asserts its own served text, so the independent-design-review criterion applies. The reviewer context is recorded in `design-review.md` and must be reused by the final `review.md`.

## Problem / Opportunity

The three Passuff trials and the Givn self-rebuild expose six rules that the technology-agnostic ise guidance does not state. The evidence:

| Change | Evidence | Observation |
|---|---|---|
| `qualify-the-passuff-e2e-package` | `proposal.md` (Corpus alignment), `evidence/s10-gate.md` | The 29 acceptance scenarios moved into `givn/specs`, and the declared `e2e` package selects them. The ise guidance never states where the scenario text lives. |
| `constant-ise-key-parser` | `evidence/ise/builder-notes.md` (Assumptions 1 and 3, registry scan), `evidence/ise/neighbour-record.md`, `review.md` R3 | The copy omitted the build root, so the builder chose dependency versions; the `--keys` contract listed 20 tokens while the corpus pressed `s`, `S`, `f`, `F`, `h`, `u`, and `c` through the same parser; the local registry held published `passuff` sources until they were quarantined. |
| `full-product-dry-run` | `evidence/ise/gap-table.md` G5 to G8 and G16, `evidence/ise/neighbour-record.md`, `review.md` | The rebuild reconstructed the build root and could not recreate 23 script paths the declaration did not specify; the gap table records `s` and `S` accepted while the contract omitted them; 17 revision files were consolidated into fewer paths; the copy was not promoted. |
| `enable-ise-and-run-the-givn-self-rebuild` | `evidence/ise/spec-gaps.md` (first row), `evidence/ise/builder-notes.md`, `evidence/ise/rebuild-report.txt`, `review.md` | The builder supplied `Cargo.toml`, `Cargo.lock`, all 95 embedded assets, the toolchain, and the registry cache; the declaration is the named wrong artifact; nothing from the trial was promoted. |

Without these rules the next rebuild repeats the same reconstruction: the builder invents versions and assets, the frozen tests exercise values no contract fixes, the trial review cannot distinguish a consolidation from a missing file, a published copy can sit within reach, and a throwaway trial can be mistaken for a promotion.

## Proposed Solution

- **Build inputs.** The Specification completeness obligation covers every build input. A build input is declared under the Specification role that owns its content class: a build manifest, a lockfile, and a schema or migration asset under `contracts`, a toolchain pin under `limits`, a structural template under `structure`, and served instruction or skill text under `behavior`. A build input the declaration leaves outside the Specification roles is a `specification defect`, and a neighbor when it lies outside the replaced unit, each with its consequence. An asset the rebuild generates inside the replaced unit whose bytes no frozen test fixes is an `acceptable undocumented change`, not a neighbor. The dependency cache and the installed toolchain are environment inputs, never declared files: the lockfile and the toolchain pin declare their content, the trial records how both were supplied, and a locked dependency the cache cannot resolve blocks the rebuild; the blocked rebuild is a setup failure, not a `Spec gap` and not a `specification defect`.
- **Fixed values.** A contract the frozen tests rely on enumerates every value it fixes: literal text, headings, labels, error prefixes, exit codes, file formats, and test seams.
- **Acceptance corpus.** The `@e2e` scenarios stay in the Specification under the `behavior` role with their exact titles. The declared `e2e` package holds the runner and every `@e2e` step definition.
- **Clean-room boundary.** The builder reads no product source outside the spec copy and the declared inputs. Before the rebuild, scan the local package registry's source, cache, and git stores for a published copy of the product, and quarantine every hit outside the builder's reach; an absent store has no hit; the neighbor record carries the scan result.
- **Neighbor and consolidation record.** The trial review records every file changed outside the declared Specification or the replaced unit, and every consolidation, with the reason for each. It is a review question, never a gate.
- **Dry run.** A `Dry-Run Ise` trial promotes nothing: the rebuilt work stays in the copy, and the trial review records that no rebuilt file was promoted.

### Lesson dispositions

| Lesson | Disposition | Wording decision |
|---|---|---|
| 1. Build inputs beyond the declared roles | accepted, adjusted | Keep the technology-agnostic requirement and name build-input classes, never a language manifest. State the role home the declared roles already provide, so the declaration change is applyable without a schema change. Separate a build input the declaration leaves outside the Specification roles from an asset the rebuild generates inside its own unit, and name the dependency cache as an environment input. The S11, S12, and S13 build roots and the 95 embedded assets are the evidence. |
| 2. Contract enumeration completeness | accepted, adjusted | Generalize the Passuff `--keys` case to a contract the frozen tests rely on; the reference names literal text, headings, labels, error prefixes, exit codes, file formats, and test seams. |
| 3. Where the acceptance corpus lives | accepted | State the S10 split: scenario text in the `behavior` role, runner and step definitions in the declared `e2e` package. |
| 4. Neighbor and consolidation record | accepted, adjusted | Extend the existing review question to consolidations instead of adding a gate; the S12 consolidation of 17 revision files is the evidence. |
| 5. Registry quarantine | accepted | State the S11 scan roots and the quarantine; an absent store has no hit; record the scan result in the neighbor record. |
| 6. Dry run promises nothing | accepted | State that a `Dry-Run Ise` trial promotes nothing; S11 and S12 both kept the rebuilt work in the copy. |

## Capability Routing

`givn spec route --change align-ise-rules-with-passuff-evidence` reported signal: `EXTEND givn-workflow-enforcement` (use case `verify-change`, score 17.00). That match is template vocabulary (`givn check`, `route`), not the served-guidance subject. The guidance capabilities were not ranked.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `instruction-integrity` | `EXTEND givn-workflow-enforcement` (17.00) | `EXTEND instruction-integrity` | The `ise` and `review` served-text scenarios are owned by `instruction-integrity` in `givn/specs/fragments/`; the delta sits in `specs/fragments/`. Route matched the proposal template, not this capability. |
| `prose-consolidation` | not ranked | `EXTEND prose-consolidation` | The two-layer sentence is asserted by the permanent `Served ise guidance keeps component scenarios in the Specification` scenario under `create-change`; the delta sits in `specs/create-change/`. |

## Out of Scope

- Givn's own declaration and build inputs: `givn/specification.yaml` is not edited, no declaration key is added, and no self-rebuild is rerun. This change states the rule; declaring `assets/**`, the build root, and the toolchain is a separate corrective change.
- Any Passuff product change, its `crates/passuff-e2e` package, its Specification, or its trial copies.
- Command behavior, exit codes, report lines, manifest fields, and checksum rules of `givn ise rebuild-all`, `givn ise rebuild-component`, and `givn ise compare`.
- Capability identifiers, interaction inventory, and `@e2e` actions. The change adds no user action.
- Promotion of any rebuilt artifact.
- Arc42 chapter files. Chapter 4 already states that the self-rebuild promoted nothing; the three new terms are assessed in `arc42.md` row 12, and chapter 12 links the glossary without a file edit.

## Open Questions

None.
