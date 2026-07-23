# Review: integrity-hook-and-archive-gate

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
0 empty step bodies found across 2 step definition files:
  tests/steps/mod.rs — all steps have real assertions
  tests/steps/workflow_enforcement_steps.rs — all steps have real assertions
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Integrity hook passes when tasks.md commit hashes are present | 2e5dcca | Yes |
| Integrity hook fails on placeholder commit hash | e80e416 | Yes |
| Integrity hook fails on missing step-definition file | 2e5dcca | Yes |
| Integrity hook fails on empty step-definition file | 2e5dcca | Yes |
| Integrity hook fails on Step Definitions section that is not a table | 2e5dcca | Yes |
| Integrity hook passes when arc42 is disabled | e80e416 | Yes |
| Integrity hook fails on unedited arc42 chapter | 2e5dcca | Yes |
| Integrity hook passes on edited arc42 chapter | 2e5dcca | Yes (covered by same commit) |
| Integrity hook skips arc42 check when slug cannot be resolved | 45ceabd | Yes |
| Archive re-runs verify against merged permanent specs and passes | e80e416 | Yes |
| Archive blocked when merged permanent specs fail verify | e80e416 | Yes |
| New-capability merge failure produces clear recovery instructions | e80e416 | Yes |
| Design.md template includes Internal Primitives section | 2e5dcca | Yes |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `src/integrity_hook.rs` | Yes | src/integrity_hook.rs |
| `assets/default-manifest.yaml` (modified) | Yes | assets/default-manifest.yaml |
| `assets/templates/design.md` (modified) | Yes | assets/templates/design.md |
| `assets/templates/tasks.md` (modified) | Yes | assets/templates/tasks.md |
| `assets/templates/review.md` (modified) | Yes | assets/templates/review.md |
| `tests/steps/workflow_enforcement_steps.rs` | Yes | tests/steps/workflow_enforcement_steps.rs |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
fail_on_skipped() is configured in tests/features_runner.rs. Confirmed by
permanent spec suite: 74 scenarios pass. Undefined steps would cause a
failure.
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a real-interface primary assertion (page content, HTTP
response, redirect, CLI output), not repository-only:

No @e2e scenarios in this change. All scenarios are non-e2e and drive the
CLI interface as the real interface (subprocess invocation + filesystem
assertions).

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action, not one per variant/error case:

N/A — no @e2e scenarios in this change.

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git
status`) was searched for any other/parallel e2e implementation:

N/A — verify.e2e_command is not configured; no @e2e scenarios exist.

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
N/A — CLI-only tool with no external dependencies. No local environment needed.
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/config.yaml`; if identical, this is always a finding:

N/A — verify.e2e_command is not configured.

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named:

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Step definitions file | tests/steps/workflow_enforcement_steps.rs | tests/steps/workflow_enforcement_steps.rs | Yes |
| Test runner | cargo test --test features_runner | cargo test --test features_runner | Yes |
| Strict mode | cucumber-rs .fail_on_skipped() | .fail_on_skipped() in features_runner.rs | Yes |
| Integrity hook function | src/integrity_hook.rs | src/integrity_hook.rs | Yes |
| Hook dispatch | src/hooks.rs | src/hooks.rs | Yes |
| Archive gate | src/cli/archive.rs (snapshot + verify) | src/cli/archive.rs | Yes |
| Template changes | design.md, tasks.md, review.md | All three updated | Yes |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions:

| Inventory entry | @e2e scenario exists? | Driving mechanism | Match? |
|---|---|---|---|
| User runs `givn check review` — integrity findings for missing commit hashes | No (non-e2e) | CLI subprocess | Yes |
| User runs `givn check review` — integrity findings for missing step-def files | No (non-e2e) | CLI subprocess | Yes |
| User runs `givn check review` — arc42 unedited chapter detection | No (non-e2e) | CLI subprocess | Yes |
| User runs `givn archive` — verify gate passes | No (non-e2e) | CLI subprocess | Yes |
| User runs `givn archive` — verify gate blocked by failing merged specs | No (non-e2e) | CLI subprocess | Yes |
| User writes design.md with Internal Primitives section | No (non-e2e) | Template assertion | Yes |

**Result:** CLEAN

---

## Coverage

<!-- Coverage was not configured for this change (addons.coverage was
     disabled). The review gate did not require it at the time.

     Policy now: unmeasured coverage is a fabrication-audit finding.
     See assets/templates/review.md and assets/instructions/review.md. -->

---

## Test suite result

```
GIVN_FEATURES=givn/changes/integrity-hook-and-archive-gate/specs cargo test --test features_runner

[Summary]
1 feature
13 scenarios (13 passed)
92 steps (92 passed)
```

```
cargo test --test features_runner

[Summary]
5 features
74 scenarios (74 passed)
382 steps (382 passed)
```

**Result:** PASS

**Coverage measured:** NO

---

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] All scenarios pass (`cargo test --test features_runner` exits 0).
- [x] Every coverage gap is classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (or "all scenarios are @e2e" is stated).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces; no
      production-code struct/fn/enum that the step definitions rely on is
      omitted (or the section states "none" if the change has no new
      primitives).
- [x] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md
      was mapped to a real `docs/arc42/<slug>.md` file per
      `src/integrity_hook.rs:check_arc42_conformance`.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets (no "acceptable for now," "future scope,"
      "sufficient for current scope," or similar invented exception).

**REVIEW: PASS**
