# Review: maintain-project

## Fabrication audit

- **@e2e tag integrity**: the delta preserves `@e2e` on both e2e scenarios;
  no tag was removed.
- **Empty/no-op step bodies**: 0 found across the touched step files
  (`release_0_3_0_e2e_steps.rs`, `scm_backend_selection_steps.rs`,
  `common/mod.rs`); the removed binding's only scenario was retired with it.
- **Task commits**: the implementation commit touches production source
  (`src/scm.rs`, `src/fossil.rs`, `src/cli/migration.rs`) plus step
  definitions and the permanent fragment alignment; the plan commit carries
  the artifacts.
- **design.md components**: Fossil `stage_paths`/`has_staged_changes`/
  `commit_staged` exist on `FossilCheckout`; the refusal helpers are deleted;
  the Fossil fixtures and backend-aware assertions exist.
- **Strict-mode proof**: present in `tasks.md` (stash-restored refusal run,
  non-zero exit).
- **@e2e primary assertions**: both e2e scenarios assert the compiled CLI's
  success and the published revision shape through the real backend.
- **verify.e2e_command**: `./run-tests.sh --e2e` (tag-filtered), distinct from
  `verify.command`; the change scope runs 4 tests in the regular scope and 2
  in the e2e scope (strictly smaller).
- **Design conformance**: matches design.md as re-reviewed (the migration e2e
  became an outline during design-review hardening, which design.md records).

### Interaction coverage cross-reference

| Inventory entry | @e2e scenario title | Driving mechanism |
|---|---|---|
| upgrade a Fossil project | A clean Fossil project receives the same managed upgrade revision shape | compiled `givn` binary in a real Fossil checkout |
| plan a release migration | A clean 0.2.0 project receives a reviewable aggregate migration plan | compiled binary, Git and Fossil outline examples |
| run a managed upgrade | A clean project without migration requirements receives one managed upgrade commit | compiled binary (Git; the shape is shared with the Fossil row) |
| invoke a retired engine command | A consumer invoking a retired engine command is told it does not exist | compiled binary |
| archive a maintenance migration | A completed maintenance migration can be archived without a product specification | compiled binary |
| readme-enforcement rows | unchanged titles | compiled binary |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| TC-21 both backends for upgrade and migrations | 02 | Yes | seam ops | scenarios 1-3 | `FossilCheckout::stage_paths/commit_staged`, refusal deleted | Yes |
| Allowlist force staging (`fossil add [--force]`) | 02 TC-22, 05, 08 | Yes | stage_paths force rule | scenario 1 | the managed revision contains the ignored generated paths and nothing else | Yes |
| Fossil CLI entity covers the upgrade | 03 | Yes | fixtures | scenarios 1-3 | upgrade runs on Fossil | Yes |
| `fossil` building block | 05 | Yes | operation table | scenarios 1-3 | `src/fossil.rs` | Yes |
| 6.13 preflight and staging | 06 | Yes | migration neutralization | scenario 3 | `src/cli/migration.rs` | Yes |
| 8.18 shared consumption | 08 | Yes | — | — | `src/scm.rs` | Yes |
| ADR-071 amendment, no new record | 09, adr/README | AMEND_ADR-071 | A12 | — | chapter 09 prose + relationship row | Yes |
| D-20 repaid | 11 | row removed | — | — | no debt row remains | Yes |
| Glossary Fossil note | 12 | Yes | — | — | note updated | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The specs, design, and code use the glossary's exact terms (revision,
checkout, clean state, allowlist, plan revision). No unrecorded term appears.

## Coverage classification

No archive receipt exists yet for this change. Static inspection: the Fossil
staging/commit operations are exercised by the three scenarios; the deleted
refusal path leaves no dead code (clippy `-D warnings` clean); the parser-only
fossil paths keep their unit tests. The embedded migration guide's Git wording
is a documented deferral (A13), not a coverage gap in this change's code.

## Deterministic gate dispositions

### Overlap dispositions

| Scenario A | Scenario B | Decision |
|---|---|---|

### Removed + added pairs

| Removed | Added | Decision |
|---|---|---|
| `A Fossil checkout is refused by name until the Fossil backend is available` | the Fossil upgrade scenarios | SUPERSESSION: the refusal is retired because the behavior it pinned no longer exists; the upgrade scenarios pin the replacement behavior. |

## Persona dispositions

- developer--primary-user: the upgrade now runs on the maintainer's backend,
  proven by the Fossil e2e scenarios.
- givn-maintainer--owner: one seam for both backends, proven by the deleted
  refusal and the shared preflight.

## Visual review

The change declares no visual interface (CLI text only); nothing renders
HTML/CSS/JS. Exempt.

## Sign-off

- [x] Fabrication audit clean
- [x] Arc42 implementation conformance clean
- [x] Ubiquitous language conformance clean
- [x] Coverage gaps classified and resolved
- [x] Both runners green (`./run-tests.sh` 477/477, `./run-tests.sh --e2e`
      84/84; change scope 4/4 and 2/2)
- [x] Deterministic overlap findings dispositioned

README-IMPACT: none

REVIEW: PASS
