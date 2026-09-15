# Design: maintain-project

## Technical Decisions

### Fossil managed-upgrade operations

`src/scm.rs` replaces the temporary managed-upgrade refusal with real Fossil
operations on `FossilCheckout`:

| Operation | Fossil mechanism | Notes |
|---|---|---|
| `stage_paths(paths, force)` | `fossil add [--force] -- <path>` per path; `--force` is passed exactly when the caller passes `force` | `fossil add` on an ignored path exits 0 after an interactive "Add it?" prompt that a null stdin answers N, so a refusal is undetectable via exit status; `force` is only ever true for the calculated allowlist (TC-22), and `fossil add` prints `SKIP` (exit 0) for an unchanged managed file, which is not a refusal |
| `has_staged_changes()` | `!dirty_entries()?.is_empty()` | Fossil has no index; the preflight guarantees a clean checkout, so the pending set is exactly the staged writes (A6) |
| `commit_staged(message)` | `fossil commit -m <message>` | commits the witnessed pending set; the caller checks `has_staged_changes` first |

`require_managed_upgrade_support` and the `upgrade_unavailable` refusal are
deleted: every seam consumer now works on both backends.

### Migration neutralization

`src/cli/migration.rs` drops both `require_managed_upgrade_support()` calls
(the preflight refuses only on a dirty checkout or no backend) and neutralizes
every remaining "Git could not ..." error context (staging the allowlist,
reading the staged state, committing, and staging the aggregate plan) to
backend-neutral wording. The plan design keeps recording `backend.revision()`
— the upgrade revision identifier, the displayed 40-hex check-in on Fossil
(A9). The embedded migration guide and bundle assets keep their Git wording;
the catalog content is explicitly unchanged by this seed (proposal Out of
Scope), so a Fossil plan quotes the same guide until a later change rewords
the assets (A13).

### Backend outline for the migration e2e

The permanent e2e "A clean 0.2.0 project receives a reviewable aggregate
migration plan" becomes a `<backend>` outline (Git, Fossil), following the
archive-change precedent: the backend is an input variation of one action, so
the inventory keeps exactly one row. Its Thens ("one commit contains only the
managed scaffold and generated agent integration changes", "the migration
change records the managed upgrade commit") gain backend-aware bindings; the
step texts stay until a later seed neutralizes the wording (A13). The
standalone Fossil migration regular scenario is therefore unnecessary and is
not added.

### Fragment retirement

The permanent `scm-backend-selection` scenario "A Fossil checkout is refused
by name until the Fossil backend is available" becomes false the moment the
implementation lands, so the permanent feature is aligned in the same commit
that removes the refusal: the scenario is deleted and its
`stderr names the Fossil checkout and the unavailable Fossil backend` binding
is removed with it.

### Fossil fixtures for the release scenarios

The release e2e fixtures mirror the Git ones (`legacy_git_project`,
`current_git_project_with_stale_integration`) with the Fossil backend
(A4): `givn init`, downgrade the config marker to 0.2.0 where needed, mark the
generated skill stale, and set `fossil settings ignore-glob
".opencode/*,AGENTS.md"` **before** the initial `addremove` so the generated
agent files stay unmanaged and ignored — that is what makes the upgrade's
force-add observable. The managed-revision assertions count check-ins with
`fossil sql "select count(*) from event where type='ci'"`, read the managed
revision's paths with `fossil diff --brief --checkin current`, and read the
upgrade revision from `fossil info`.

The Fossil uncommitted-changes fixture reuses the legacy fixture plus one
extra uncommitted user file; the byte-for-byte assertion is backend-neutral.

## Production Impact

- `src/scm.rs`: Fossil `stage_paths`/`has_staged_changes`/`commit_staged`;
  delete `require_managed_upgrade_support` and `upgrade_unavailable`.
- `src/cli/migration.rs`: remove the two upgrade-support refusals; neutralize
  the aggregate-plan staging message.
- `givn/specs/fragments/scm-backend-selection.feature`: delete the retired
  scenario (permanent alignment).
- `tests/steps/release_0_3_0_e2e_steps.rs`: Fossil fixture builders, the
  backend outline Given, backend-aware revision assertions, and a
  backend-aware `the user changes remain byte-for-byte unchanged` (the shared
  step shells out to `git status` and must branch on the fixture backend).
- arc42 chapters: 02 (TC-21), 03 (Fossil marker entity and in-scope list),
  05 (migration building-block row), 06 (6.13), 08 (8.18), 09 prose plus the
  `adr/README.md` relationship row (ADR-071 amended, no new record), 11
  (D-20 repaid), 12 (glossary Fossil note). Remove the dead Fossil-refusal
  binding from `tests/steps/scm_backend_selection_steps.rs`.
- `tests/common/mod.rs`: expose `fossil_commit_all` for the fixture builders
  (already public).

User-visible surface: CLI text only; no `## Visual Design Contract` section
(exempt, same precedent as the previous seeds).

## Test Design

Executable scenarios in the change delta:

```text
givn/changes/maintain-project/specs/maintain-project/release-0-3-0.feature
givn/changes/maintain-project/specs/maintain-project/usecase.md
```

The delta carries three scenarios: the migration e2e becomes a Git/Fossil
outline, the migration-free Fossil upgrade is the new inventory row's `@e2e`,
and the Fossil uncommitted-changes protection is a regular scenario. The
fragment tombstone was dropped during implementation: the permanent scenario
was already deleted (commit a224e40f) to keep the suite green, and the merge
validates removed titles against the permanent corpus.

| File | Binds |
|---|---|
| `tests/steps/release_0_3_0_e2e_steps.rs` | The Fossil fixture Givens, the migration-free and migration-bearing revision Thens, the revision-only-managed-paths Then, the no-revision-created Then. |
| `tests/steps/scm_backend_selection_steps.rs` | Deletion of the retired refusal Then binding. |

Runner and strict mode: `verify.command` is `./run-tests.sh`; the cucumber
runner sets `.fail_on_skipped()`. New step bodies start as
`unimplemented!()` during RED.

Single-scenario command:

```sh
GIVN_FEATURES=givn/changes/maintain-project/specs cargo test --test features_runner -- --name "A clean Fossil project receives the same managed upgrade revision shape"
```

E2E scope command:

```sh
GIVN_FEATURES=givn/changes/maintain-project/specs ./run-tests.sh --e2e
```

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| upgrade a Fossil project | A clean Fossil project receives the same managed upgrade revision shape | givn CLI | The compiled `givn` binary runs `givn upgrade` in a real Fossil checkout; the test asserts exit 0, the refreshed config, one managed check-in, and no migration change |
| plan a release migration | A clean 0.2.0 project receives a reviewable aggregate migration plan | givn CLI | The compiled binary runs `givn upgrade` in a real 0.2.0 project on Git and on Fossil (outline examples); the test asserts the managed revision, the plan change, and the recorded upgrade revision |
| run a managed upgrade | A clean project without migration requirements receives one managed upgrade commit | givn CLI | unchanged (Git) |
| invoke a retired engine command | A consumer invoking a retired engine command is told it does not exist | givn CLI | unchanged |
| archive a maintenance migration | A completed maintenance migration can be archived without a product specification | givn CLI | unchanged |
| readme-enforcement rows | unchanged | givn CLI | unchanged |

## Use-Case Traceability

| Use-case guarantee or rule | Design decision | Capability | E2E evidence |
|---|---|---|---|
| The upgrade runs on both backends with the same guarantees (Q8) | Real Fossil staging/commit operations on the seam; refusal deleted | release-0-3-0 | A clean Fossil project receives the same managed upgrade revision shape |
| A migration-free upgrade publishes exactly one revision and no plan revision | `commit_staged` once after staging the allowlist | release-0-3-0 | The Fossil migration-free scenario |
| The plan revision records the upgrade revision identifier | The aggregate-plan design embeds `backend.revision()` | release-0-3-0 | The migration outline's Fossil example |
| Ignored managed paths are force-staged only through the allowlist | `fossil add --force` exactly when the caller passes `force`, which only the calculated allowlist triggers | release-0-3-0 | The migration outline's Fossil example asserts the managed revision contains the ignored generated paths and nothing else |
| A dirty checkout blocks before writing | Shared preflight `dirty_entries` on the selected backend | release-0-3-0 | The Fossil uncommitted-changes scenario |
| No backend is refused by name before writing | `scm::select` unchanged | release-0-3-0 | The permanent outside-Git scenario |
| Confirmed Personas | developer--primary-user (both backends), givn-maintainer--owner (one seam) | — | Review lens |

## Local Runnability

`./run-tests.sh` runs the full suite; external executables are `git`,
`fossil`, and the compiled `givn` binary. No digital twin. Obstacles and
fixes: the ignored generated paths must be unmanaged before the initial
check-in (ignore-glob set before `addremove`), and check-in counting uses the
`event` table rather than output scraping.
