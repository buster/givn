# Design review: ise-rebuild-decision-and-working-tree-ship

Review kind: independent
Author: Main (omp session)
Reviewer context: `agent://DesignReviewIseVocab` (fresh context, did not author the change; reused for the final review)

## Operator decisions recorded during grilling

- **Direct-work boundary.** The operator decided that the new working-tree guidance text is direct work, like the 1:1 vocabulary renames. The change's delta carries only the new regression scenarios; the guidance text lives in the direct commits `bbd7de5`, `4b30dfb`, and `92ad0a5`.
- **Vocabulary scope.** The operator selected the full sweep, then directed that plain 1:1 renamings land as direct work. The unchanged `prepare-an-ise-test` narrative and its two feature files keep their step text as the recorded exception; their report labels remain `Specification files:`.
- **Reference direction.** The site stays framework-agnostic and names no Givn command, flag, or record; Givn cites https://iseprinciple.org. The site's earlier CLI `--dry-run` comparison, the preview distinction, and the `Rebuilt code` record heading were removed.

## Grilling questions and answers

| Branch | Question | Answer |
|---|---|---|
| Scope / decision ledger | Does the change record its own Ise rebuild decision? | Yes: `Decides D8: this change is ise rebuild decision No rebuild`, with no `Rebuilt code` line; the traceability row is present. |
| Scope | Is the delta the active Specification the design claims? | The operator decided the guidance text is direct work; the delta carries the seven added regression scenarios (four ship/limitation/copy/restart, one citation, two decision variants) and no byte-identical modified copies. |
| Scope | Is the `specification-manifest` capability consistent with the routing? | It is a direct 1:1 rename committed before the delta; the design lists the file, step file, and matrix row; no NEW routing remains. |
| Scope | Is the copy-path exception an operator decision? | Yes, recorded in the proposal's D4 disposition, the design's D4, and this record. |
| Testability | What is the RED stub pattern? | Each new step body starts as `unimplemented!("<step text>: no assertion yet")` or an explicit panic, never empty; `@wip` until GREEN; no `@wip` at archive. |
| Testability | Single-scenario command and strict mode? | `GIVN_FEATURES=<feature> GIVN_CUCUMBER_TAGS='not @e2e and not @givn.removed' cargo nextest run --locked --test features_runner -E 'test("<feature>::<scenario>")'`; `.fail_on_skipped()` on the builder and the e2e runner fails a skipped step. |
| Missing scenarios | Does the delta cover the rebuild impact record and the review limitation? | The served guidance and its permanent scenarios are direct work; the delta adds the ship sequence, limitation, copy distinction, restart, and citation. |
| E2E fidelity / interaction coverage | Where is the Interaction Coverage Matrix? | One continuous 44-row table covers every inventory entry of `create-change`, `declare-the-specification`, `prepare-an-ise-test`, and the 32 support-fragment entries, with exact `@e2e` titles and drivers. |
| Contracts and components | Is the served guidance a Component? | No; the `### Contracts and Components` table records no new Component or contract, and the existing CLI boundary is the consumer surface. |
| Persona context | Are all confirmed Personas dispositioned? | Yes: developer (authoring lens), agent (served-CLI consumer), maintainer (vocabulary/Arc42 owner). |
| Risk | What is the most likely failure? | Implemented guidance drifting from the delta; mitigated by the delta scenarios, the full suite, and the permanent phrase bindings. |
| ADR qualification | Does the `Ise Test` redefinition amend ADR-076? | Yes: ADR-076 is amended to scope the clean-room trial record to the copy-based Ise Test; the chapter 09 register and ADR index record it; the rejected alternative is recorded. |
| Arc42 | Are the affected chapters actually current? | Chapters 02, 03, 04, 05, 06, 08, 09, 11, and 12 are updated; the assessment rows match the files. |
| Ubiquitous language | Does the glossary hold the terms? | Yes; the Specification Manifest row records the reversed `specification manifest` anti-term, and the copy-path exception is recorded. |
| Structural model | Did the change assess `docs/domain-model.md`? | Yes; the Change aggregate names the Ise rebuild decision and the project record names the Specification Manifest. |

## Findings and dispositions

| ID | Finding | Disposition |
|---|---|---|
| B1 | The change did not record its own Ise rebuild decision. | Fixed: `Decides D8` and the traceability row. |
| B2 | The delta was not the Specification the design claimed. | Fixed: the guidance is direct work by operator decision; the delta has seven added scenarios and no identical modified copies. |
| B3 | `specification-manifest` was routed NEW but absent. | Fixed: direct 1:1 rename; the proposal says so and the design lists it. |
| B4 | The D4 exception narrowed the operator's full sweep. | Fixed: the proposal records the later direct-rename decision; the design and this record state the exact exception. |
| B5 | No not-implemented stub pattern. | Fixed: the stub rule is in the design. |
| F1 | Reference/Arc42 still carried retired nouns. | Fixed at the cited lines; literal report/error labels are the recorded exception. |
| F2 | Chapter 11 had no working-tree ship risk. | Fixed: R-165, placed after R-164; assessment row updated. |
| F3 | The `Ise Test` redefinition did not reconcile with ADR-076. | Fixed: ADR-076 amendment, register, and index. |
| F4 | No Interaction Coverage Matrix. | Fixed: 44-row continuous table. |
| F5 | No Contracts and Components subsection. | Fixed: the no-new-Component table. |
| F6 | Persona dispositions incomplete. | Fixed: per-persona table. |
| F7 | Structural model not assessed. | Fixed: `docs/domain-model.md` updated; design section added. |
| F8 | Undefined non-requirement phrases. | Fixed: "generated spec-copy manifest" and "repository view that hides history". |
| F9 | Restart rule untraced. | Fixed: proposal bullet and design traceability row. |
| F10 | Modified existing `@e2e` scenario outside the delta. | Fixed: the direct commit modifies it; the design names the step file and the E2E infrastructure states the direct ownership. |
| F11 | Surface and step tables incomplete. | Fixed: expanded Architecture Impact and delta-only Step Definitions. |
| F12 | Arc42 claims contradicted the files. | Fixed: chapters 06, 08, 11, and row 09 re-derived. |
| A1 | Duplicate `rebuild-component` table rows differed. | Fixed: both rows use Component/Component Interfaces. |
| A2 | Review Context said one retired capability. | Fixed: direct renames named. |
| A3 | Reversed anti-term not recorded. | Fixed: the Specification Manifest row records it. |

## Site check

The site at `/home/buster/projects/iseprinciples/index.html` was served and inspected at 1920x1080 and 360x780, including the pattern drawer and diagram labels. It contains no Givn command, flag, or record; the Dry-Run pattern, the decision cards, and the outcome wording are framework-agnostic.

DESIGN-REVIEW: PASS
