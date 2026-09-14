# Review: archive-change

## Fabrication audit

- **@e2e tag integrity**: the delta preserves `@e2e` on the archive e2e
  outline; no tag was removed.
- **Empty/no-op step bodies**: 0 found across the touched step files
  (`archive_proof_steps.rs`, `archive_proof_e2e_steps.rs`,
  `archive_proof_support.rs`, `adr_qualification_gate_steps.rs`,
  `scm_backend_selection_steps.rs`, `mod.rs`).
- **Task commits**: every scenario commit touches production source
  (`src/fossil.rs`, `src/scm.rs`, `src/git.rs`, `src/cli/archive.rs`,
  `src/cli/migration.rs`, `src/proof.rs`) plus step definitions; none is
  spec-only.
- **design.md components**: `src/fossil.rs` (`FossilCheckout`),
  `Backend::Fossil`, `Backend::publish`, `Backend::revision_contains`,
  the archive resume republication, the managed-upgrade refusal, and the
  receipt rename all exist in the tree.
- **Strict-mode proof**: present in `tasks.md` with a non-zero exit.
- **@e2e primary assertion**: the archive e2e outline asserts the real CLI
  invocation's success (compiled binary, `givn archive`), then the proven
  receipt as secondary evidence.
- **verify.e2e_command**: `./run-tests.sh --e2e` (tag-filtered), distinct
  from `verify.command` `./run-tests.sh`; the change scope runs 17 tests in
  the regular scope and 1 in the e2e scope (strictly smaller).
- **Design conformance**: the implementation matches design.md's named
  files, commands, and step layout; no silent deviation.

### Interaction coverage cross-reference

| Inventory entry | @e2e scenario title | Design matrix row | Driving mechanism |
|---|---|---|---|
| archive a completed change | Consumer archives a completed change and receives a proven archive | present | compiled `givn` binary subprocess in a real Git project and a real Fossil checkout |
| repair a blocked archive | Consumer repairs a blocked archive and the rerun proves it | present | compiled binary, archive twice around a committed repair |
| inspect an unproven archive candidate | Consumer inspects an unproven archive candidate | present | compiled binary, `givn status` |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| TC-21 managed upgrade Git-only, archive both backends | 02-architecture-constraints | Yes | upgrade refusal | scenario 11 | `src/cli/migration.rs` `require_managed_upgrade_support` | Yes |
| TC-31 witness-bounded publication | 02 | Yes | publication contract | scenarios 1, 4 | `Backend::publish`, `FossilCheckout::publish` validation | Yes |
| TC-32 checkout root | 02 | Yes | seam boundary | scenario 10 | `scm::select` boundary check | Yes |
| TC-33 selection and refusals | 02 | Yes | Fossil executable probe | scenarios 9, 11 | `scm::select`, `fossil_executable` | Yes |
| Fossil CLI external entity | 03 | Yes | Fossil operations | all Fossil scenarios | `src/fossil.rs` | Yes |
| `fossil` building block | 05 | Yes | module registration | scenario 1 | `src/lib.rs`, `src/main.rs`, `src/fossil.rs` | Yes |
| Archive runtime flow (6.7) | 06 | Yes | resume republication | scenarios 2, 4 | `src/cli/archive.rs` | Yes |
| Upgrade Fossil refusal (6.13) | 06 | Yes | temporary refusal | scenario 11 | migration preflight | Yes |
| Selection flow (6.16) | 06 | Yes | Fossil backend | scenarios 9-11 | `src/scm.rs` | Yes |
| Backend executable prerequisite (7.3) | 07 | Yes | test environment | setup | CI install step, `AGENTS.md` | Yes |
| 8.18 selection and publication | 08 | Yes | witness-bounded | scenarios 1, 4 | `Backend` methods | Yes |
| ADR-072 registered (28 accepted) | 09, adr/README | Yes | ADR-072 | scenario 12 | ADR file, register, count step | Yes |
| Q-92 portability | 10 | Yes | Fossil archive path | scenarios 1-8 | Fossil scenarios | Yes |
| R-65, R-66, D-20 | 11 | Yes | risks | — | documented | Yes |
| Witness-bounded publication, unpublished candidate | 12-glossary | Yes | terms | — | terms used consistently | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The specs, design, and code use the glossary's exact terms: revision,
checkout, clean state, candidate revision, tested revision, revision binding,
witness-bounded publication, unpublished candidate, named refusal. No
unrecorded term appears.

## Coverage classification

No archive receipt exists yet for this change (the archive measures and
publishes coverage once against the candidate). Static inspection: every new
production path is exercised by a scenario — `FossilCheckout` operations by
scenarios 1-8, the resume republication by scenario 4, the receipt rename by
scenario 3, the upgrade refusal by the permanent `scm-backend-selection`
scenario, and the parser-only paths by unit tests in `src/fossil.rs`
(`parse_change_line` rename/EXTRA cases, missing `checkout:` line). No dead
code: `Worktree::open` was deleted once unused.

## Deterministic gate dispositions

### Overlap dispositions

| Scenario A | Scenario B | Decision |
|---|---|---|
| `A Fossil EXTRA file blocks the archive` | `A Fossil edited file blocks the archive` | boundary |

The pair shares its step shape deliberately: both pin the clean-state witness
for exactly one Fossil dirty state (`EXTRA` versus `EDITED`). They assert
different repository states; merging them would lose the per-state evidence.

### Removed + added pairs

| Removed | Added | Decision |
|---|---|---|
| `A rejected candidate commit leaves the merge staged and unproven` | `A rejected candidate revision leaves the archive unproven and repairable` + the Git/Fossil intermediate-state scenarios | SUPERSESSION: the neutral promise replaces the Git-specific wording; the Git staged-state assertion survives in its own scenario, and the Fossil state is added. |
| `Archive requires Git version control` | `Archive requires a version-controlled project` | SUPERSESSION: the refusal is now backend-neutral. |
| `Archive requires the project root to be the repository root` | `Archive requires the project root to be the checkout root` | SUPERSESSION: the boundary is the checkout root. |
| `A proven archive records a verification receipt binding the tested commit` | `A proven archive records a verification receipt binding the tested revision` | SUPERSESSION: revision vocabulary. |

## Persona dispositions

- developer--primary-user: repairable failures and named refusals, proven by
  the repairable and refusal scenarios.
- agent--technical-role: deterministic commands and machine-readable
  receipts, proven by the receipt scenarios.
- givn-maintainer--owner: one publication seam and truthful architecture
  docs, proven by ADR-072 and the arc42 conformance table.

## Visual review

The change declares no visual interface (CLI text only); the opposite
classification does not hold because nothing renders HTML/CSS/JS. Exempt.

## Sign-off

- [x] Fabrication audit clean
- [x] Arc42 implementation conformance clean
- [x] Ubiquitous language conformance clean
- [x] Coverage gaps classified and resolved
- [x] Both runners green (`./run-tests.sh` 473/473, `./run-tests.sh --e2e`
      84/84; change scope 17/17 and 1/1)
- [x] Deterministic overlap findings dispositioned

README-IMPACT: none

REVIEW: PASS
