# Review: local-only-rules

## Fabrication Audit (do this first)

**Step definition scan** — the new step file
`tests/steps/local_only_rules_steps.rs` carries real assertions (git state,
file contents, ignore blocks, exit codes). No empty or no-op bodies were
added; existing step files were only updated for the new hook home and the
policy-aware revision shape.

**Commit cross-check** — every checked-off task has a commit touching
production source:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Plan | `55b3dcb1` | Planning artifacts |
| rule policy | `73b5d226` | `src/config/overlay.rs`, `src/cli/init.rs`, assets, config, steps |
| transition + setup | `357c97b6` | `src/skills/rules.rs`, targets, SCM seam, arc42 chapters, specs |
| commit stage set | `f94c26b8` | `src/cli/commit.rs`, delta spec |
| upgrade | `0644838b` | `src/cli/migration.rs`, e2e step updates, delta spec |
| dedicated check | `ff7fb5cd` | `src/cli/check.rs`, delta spec |

**Promised-architecture cross-check** — every component design.md named
exists: `src/skills/rules.rs`, `RulesPolicy` in `src/config/overlay.rs`,
backend `remove_from_index` / `history_occurrences`, `givn check rules`,
`assets/config.yaml.tmpl` policy section, updated chapters 5/8/2/10/11/12.

**Strict-mode proof** — `.fail_on_skipped()` in `tests/features_runner.rs`;
the first delta run before the fixes exited non-zero (6 failed), and each
capability landed with a targeted green (recorded in tasks.md).

**Downgraded e2e scenario scan** — no `@e2e` tag was added or removed by this
change. The capability's existing `@e2e` scenarios drive real `givn init` /
`givn upgrade` subprocesses and assert files plus revision contents.

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Consumer initializes a project and receives shared attribution plumbing | Yes — files on disk | N/A (CLI) | No |
| A clean 0.2.0 project receives a reviewable aggregate migration plan (Git/Fossil) | Yes — published revision contents | N/A (CLI) | No |

**E2e scope check** — one `@e2e` per distinct action; the delta adds none.

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| initialize-project / upgrade-a-project | existing | Yes | No |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`; the tree (including untracked files) was searched; a
single implementation per capability exists.

**Local environment check** — no stack and no digital twins; local run is the
runner and `cargo run`.

**E2e isolation check** — commands differ; counts from the change
verification runs: regular 543 (one expected pre-merge modified-scenario
failure; the archive verifies the merged candidate), e2e 88. Strictly
smaller; the archive receipt carries the measured result.

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2E scope scenario count | E2E count strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 544 | 88 | Yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `initialize-project` and `upgrade-a-project` keep their goals; the policy adds behavior through their contracts; new fragments documented in `fragment.md` | Yes |
| Capability ownership and typed relationships preserved | New capabilities on the `support` fragment; existing capabilities extended, none moved or retired | Yes |
| Confirmed Persona-relevant outcomes preserved | Clean repository, deterministic commands, Givn-free clone, truthful records | Yes |
| Actors remain distinct from Persona biographies | Gherkin names actions only | Yes |
| Migration ledger and coverage evidence complete, if applicable | N/A — no migration route | Yes |

PERSONAS: developer--primary-user, agent--technical-role, external-contributor--open-source-consumer, givn-maintainer--owner

## Deferred mandatory checks

None.

**Design.md conformance check**:

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| D1 repository policy | `rules: versioned` in `givn/config.yaml` | Present | Yes |
| D2 trigger/idempotence | sync reconciles; repeats change nothing | `reconcile_tracked_rules`, idempotence scenario | Yes |
| D3 report-only check | no fresh-clone run | `cleanliness` + `givn check rules` | Yes |
| D4 Claude hook home | `.claude/settings.local.json` | `targets.rs` writes it; fixtures updated | Yes |
| D5 check name | `givn check rules` | Implemented | Yes |

**Interaction coverage verification**:

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised driving mechanism | Actual | Match? |
|---|---|---|---|---|---|
| initialize and receive plumbing | Yes | Yes | Real subprocess | Real subprocess | Yes |
| upgrade a legacy project | Yes | Yes | Real subprocess | Real subprocess | Yes |

## Visual Review

N/A — no visual interface in the design.

VISUAL-REVIEW: PASS

## Retirements

N/A.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable source | `arc42.md` claim | design.md | tasks.md | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Managed allowlist and force-staging | 2.7 | Yes | yes | transition task | `commit_managed_upgrade` forces only under versioned | Yes |
| Building blocks | 8.7 / 5.3 | Yes | yes | policy + transition tasks | `skills::rules`, updated rows | Yes |
| Release-safety expectation | Q-57 | Yes | yes | policy task | policy-aware revision | Yes |
| Force-staging risk | R-33 | Yes | yes | transition task | force scoped to versioned; `AGENTS.md` excluded | Yes |
| Terms | 12 | No new term | yes | policy task | row updated for user-owned settings | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt yet; the delta suite is 15/15 and the e2e scope 88/88.
The merged report is produced by the archive gate.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| New policy paths | 0 uncovered | The delta suite exercises resolution, both policies, both destinations, transition, stage set, revision shape, and the check | None required |

## Scenario execution evidence

Unproven candidate — no receipt yet; the archive runs both scopes against the
merged candidate. Pre-merge: delta 15/15, e2e 88/88, permanent regular has the
one expected modified-scenario failure that the merge replaces.

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The stage-all path excludes AGENTS.md and generated rules under local-only | The versioned stage-all path still excludes AGENTS.md | `variant` |
| The transition is idempotent after the removal is committed | The dedicated check passes on a clean project | `variant` |
| The transition is idempotent after the removal is committed | The upgrade reports rules that survive in reachable history | `variant` |
| The dedicated check names a tracked rule and exits non-zero | The check names rules that survive in reachable history | `variant` |
| The dedicated check passes on a clean project | The upgrade reports rules that survive in reachable history | `variant` |
| Exact duplicates across features are reported together | The transition is idempotent after the removal is committed | `variant` |
| Exact duplicates across features are reported together | The dedicated check passes on a clean project | `variant` |
| Exact duplicates across features are reported together | The upgrade reports rules that survive in reachable history | `variant` |
| Same assertion shape without exact behavior is reported as a structural match | The transition is idempotent after the removal is committed | `variant` |
| Same assertion shape without exact behavior is reported as a structural match | The dedicated check passes on a clean project | `variant` |
| Same assertion shape without exact behavior is reported as a structural match | The upgrade reports rules that survive in reachable history | `variant` |
| A structural subset is reported as an overlap | The transition is idempotent after the removal is committed | `variant` |
| A structural subset is reported as an overlap | The dedicated check passes on a clean project | `variant` |
| A structural subset is reported as an overlap | The upgrade reports rules that survive in reachable history | `variant` |
| A missing permanent specifications directory is an empty corpus | The transition is idempotent after the removal is committed | `variant` |
| A missing permanent specifications directory is an empty corpus | The dedicated check passes on a clean project | `variant` |
| A missing permanent specifications directory is an empty corpus | The upgrade reports rules that survive in reachable history | `variant` |
| Archived scenarios are excluded from the audit | The transition is idempotent after the removal is committed | `variant` |
| Archived scenarios are excluded from the audit | The dedicated check passes on a clean project | `variant` |
| Archived scenarios are excluded from the audit | The upgrade reports rules that survive in reachable history | `variant` |
| Lexical search reports an empty result for unknown terms | The transition is idempotent after the removal is committed | `variant` |
| Lexical search reports an empty result for unknown terms | The dedicated check passes on a clean project | `variant` |
| Lexical search reports an empty result for unknown terms | The upgrade reports rules that survive in reachable history | `variant` |
| A proposal matching an existing capability surface recommends EXTEND | The transition is idempotent after the removal is committed | `variant` |
| A proposal matching an existing capability surface recommends EXTEND | The dedicated check passes on a clean project | `variant` |
| A proposal matching an existing capability surface recommends EXTEND | The upgrade reports rules that survive in reachable history | `variant` |
| A proposal with no matching surface reports no signal | The transition is idempotent after the removal is committed | `variant` |
| A proposal with no matching surface reports no signal | The dedicated check passes on a clean project | `variant` |
| A proposal with no matching surface reports no signal | The upgrade reports rules that survive in reachable history | `variant` |
| Integrity hook passes when tasks.md commit hashes are present | The transition is idempotent after the removal is committed | `variant` |
| Integrity hook passes when tasks.md commit hashes are present | The dedicated check passes on a clean project | `variant` |
| Integrity hook passes when tasks.md commit hashes are present | The upgrade reports rules that survive in reachable history | `variant` |
| Integrity hook skips arc42 check when slug cannot be resolved | The transition is idempotent after the removal is committed | `variant` |
| Integrity hook skips arc42 check when slug cannot be resolved | The dedicated check passes on a clean project | `variant` |
| Integrity hook skips arc42 check when slug cannot be resolved | The upgrade reports rules that survive in reachable history | `variant` |
| Explicit retirement documents a changed behavior | The transition is idempotent after the removal is committed | `variant` |
| Explicit retirement documents a changed behavior | The dedicated check passes on a clean project | `variant` |
| Explicit retirement documents a changed behavior | The upgrade reports rules that survive in reachable history | `variant` |
| Consumer runs migration coverage check and sees passed evidence | The transition is idempotent after the removal is committed | `variant` |
| Consumer runs migration coverage check and sees passed evidence | The dedicated check passes on a clean project | `variant` |
| Consumer runs migration coverage check and sees passed evidence | The upgrade reports rules that survive in reachable history | `variant` |
| An opaque revision identifier passes the integrity check | The transition is idempotent after the removal is committed | `variant` |
| An opaque revision identifier passes the integrity check | The dedicated check passes on a clean project | `variant` |
| An opaque revision identifier passes the integrity check | The upgrade reports rules that survive in reachable history | `variant` |

## Split-or-keep

None — no long-scenario finding for this delta.

## Sign-off

- [x] Fabrication audit: CLEAN.
- [x] Scenario execution evidence recorded per scope.
- [x] Every coverage gap is classified and resolved from measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta.
- [x] The delta contains no implementation-layer detail.
- [x] Local run command starts cleanly; no digital twins.
- [x] `verify.e2e_command` identified and searched for parallel implementations.
- [x] `verify.e2e_command` differs from `verify.command`; counts prove isolation.
- [x] Implementation matches design.md as reviewed.
- [x] Interaction coverage verified.
- [x] `## Internal Primitives` lists `RulesPolicy` and `Cleanliness`.
- [x] arc42 chapter slugs map to real files.
- [x] No finding excused outside the three buckets.

README-IMPACT: none

**REVIEW: PASS**
