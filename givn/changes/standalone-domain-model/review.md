# Review: standalone-domain-model

Review kind: self-review
Author: Maki
Reviewer context: Maki

README-IMPACT: updated - Description

## Scope

This change moves durable vocabulary to `docs/glossary.md`, adds
`docs/domain-model.md`, scaffolds both from embedded templates, decouples the
domain-modeling skill from arc42, and points review and ideation guidance at the
correct owner. The review covers the 28 delta scenarios (27 regular, 1 e2e),
the scaffold functions, the routing change, the embedded guidance, and the
repository's own migration.

## Implementation evidence check

| Check | Result |
|---|---|
| `@e2e` tag integrity | No delta scenario lost `@e2e`. The modified sync scenario retains `@e2e`; no new E2E action was invented. |
| Empty or trivial step bodies | 0 empty bodies across the changed step files. |
| Checked tasks with commits | Every checked scenario records a batch revision touching implementation or regression tests. |
| Promised components exist | `assets/templates/glossary.md`, `assets/templates/domain-model.md`, `scaffold_glossary`, `scaffold_domain_model`, `scaffold_document_at`, and the routing changes exist. |
| Strict-mode proof | `tests/features_runner.rs` calls `.fail_on_skipped()`; an undefined-step probe exited non-zero. |
| E2E primary assertions | The sync scenario asserts generated CLI output and the served instruction through real subprocesses, not repository-only state. |
| `verify.e2e_command` | `./run-tests.sh --e2e`; the archive scope is selected with `GIVN_RESULT_SCOPE=e2e`. |
| Design deviation | None: the implementation matches the design's file layout, commands, and component boundaries. |

### Commit evidence

| Batch | Revision | Contents |
|---|---|---|
| standalone-document-scaffolds | `e14e7a76` | Embedded templates, chapter 12 link, scaffold functions, scaffold step bindings |
| domain-skill-routing | `39a87413` | `domain_modeling_present`, `domain_skill_to_load`, generated skill template |
| standalone-guidance | `666ce56a` | Domain-modeling instruction, ideation/review/design-review guidance, arc42 instruction, config comments |
| standalone-guidance | `f4bb1f2e` | Preserved ideation and skill-trigger phrases after the permanent-suite check |
| standalone-documentation-closeout | `40d1b63b` | Glossary move, repository domain model, arc42 chapters, docs index/reference/workflow, change artifacts |
| standalone-document-e2e | `39a87413` | E2E scenario implementation; evidence in `f49aa68d` |
| documentation-only follow-ups | `384d28b7`, `26ad0b19` | README addon bullet, verification evidence, glossary terms |

### Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `initialize-project`: initialize with explicit noninteractive selection | Present | Consumer runs init non-interactively and gets opencode skills | Real `givn init --agents opencode` subprocess |
| `enable-optional-addons`: enable an addon (arc42 and domain_modeling are variants) | Present | Consumer enables the outline addon and it is listed as enabled | Real `givn addons enable` subprocess; variants assert scaffolded files |
| `refresh-generated-guidance`: synchronize skills and read the domain guidance | Present | An agent synchronizes skills and the domain skill is routed | Real `givn skills sync` and `givn instructions` subprocesses |

No capability has more than one `@e2e` scenario for the same normalized action.
The scaffold assertions are file-level secondary evidence behind the CLI action,
not a replacement for the real interface.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `cli::addons` scaffolds both documents | `docs/arc42/05-building-block-view.md` 5.3 | Yes | Module changes | Batch A | `src/cli/addons.rs` `scaffold_document_at` | Yes |
| `cli::instructions` routes the domain skill without arc42 | `docs/arc42/06-runtime-view.md` 6.3 | Yes | Module changes | Batch B | `src/cli/instructions.rs` one-flag router | Yes |
| `givn addons enable` behavior | `docs/arc42/06-runtime-view.md` 6.12 | Yes | Module changes | Batch A | `src/cli/addons.rs` scaffold branches | Yes |
| 8.11 addon scaffolding and 8.27 ownership | `docs/arc42/08-crosscutting-concepts.md` | Yes | Overview and module changes | Batch D | Chapter sections | Yes |
| R-127 migration two-location interval | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Decisions and risk | Batch D | Risk row | Yes |
| Chapter 12 is a link to the standalone glossary | `docs/arc42/12-glossary.md` | Yes | Module changes | Batch D | Chapter file and `docs/glossary.md` | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`docs/glossary.md` is the sole vocabulary authority and now defines **Domain
model**, **Standalone glossary**, and **Documentation scaffold**. The changed
specs, design, guidance, and code use those terms consistently. The only
remaining `docs/arc42/12-glossary.md` references are the migration procedure and
the historical archive; both are deliberately non-authoritative. No duplicate
definition table remains.

## Coverage

The delta coverage run measured the changed scope through
`./measure-coverage.sh` and merged it with `./merge-coverages.sh`. The changed
paths are exercised: `scaffold_document_at` by the nine scaffold scenarios,
`domain_skill_to_load` and `domain_modeling_present` by the routing scenarios,
and the instruction renderer by the guidance scenarios.

The project's existing coverage command produces line coverage with a zero
branch rate because branch instrumentation is unstable in the pinned stable
toolchain. This is a pre-existing measurement limitation, not a gap introduced by
this change; no production branch was added that the line-only report hides.
Bucket 3 justification: branch measurement requires an unstable toolchain flag
outside this change's scope. The archive receipt will carry the full-suite
measurement.

No dead code was added. The new unit tests cover template embedding, missing-file
creation, existing-byte preservation, and the routing flag without duplicating a
black-box scenario.

## Visual review

The design declares `Interface: terminal`. The modified `@e2e` scenario committed
a transcript at
`givn/changes/standalone-domain-model/evidence/visual/an-agent-synchronizes-skills-and-the-domain-skill-is-routed/transcript.txt`.
It shows the real `givn skills sync` output and exit code. The opposite
classification (Web UI) does not hold: no HTML, CSS, or JavaScript is rendered.

Persona dispositions:

- `agent--technical-role`: can load the routed skill and read the standalone
  document guidance; no finding.
- `developer--primary-user`: receives both documents on init/enable and can
  retain existing content; no finding.
- `givn-maintainer--owner`: sees the vocabulary and structure split in the
  reference and architecture docs; no finding.

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

None.

## Findings

None remaining. The five permanent-suite failures are exactly the scenarios the
delta removes or modifies:

- `The skill requires the arc42 glossary for domain modeling` — removed.
- `The ubiquitous language is recorded in the project's arc42 glossary` — removed.
- `A project without the arc42 glossary is not pointed at the domain skill` — removed.
- `The design-review skill checks that the ubiquitous language is followed` — modified.
- `The review skill checks that the ubiquitous language is followed` — modified.

All other permanent non-e2e scenarios pass.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching implementation, tests, or guidance.
- [x] Every promised component exists.
- [x] Every consumer uses only its declared contract.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded for the change scope.
- [x] Coverage classified from the measured delta run; no new gap.
- [x] Dead code deleted; missing tests added; the branch-measurement limitation is justified.
- [x] No `@wip` tags remain; no implementation-layer detail in the spec.
- [x] Canonical E2E policy read; one normalized action per `@e2e` scenario.
- [x] Every E2E scenario asserts the real CLI interface.
- [x] `verify.e2e_command` identified and read; no parallel E2E implementation exists.
- [x] E2E scope selected explicitly via `GIVN_RESULT_SCOPE`.
- [x] Implementation matches design.md's commands, file layout, and framework choice.
- [x] Interaction coverage verified against the use-case inventory.
- [x] No finding excused with a classification outside the three buckets.

REVIEW: PASS
