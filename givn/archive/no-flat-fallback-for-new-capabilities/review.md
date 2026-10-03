# Review: no-flat-fallback-for-new-capabilities

## Fabrication Audit

**@e2e tag integrity**: both added scenarios carry `@e2e @givn.added` in the
delta since creation; the `@wip` probe tag existed only during the RED phase
and was replaced by `@e2e` before GREEN (git history of the delta file).

**Step body scan**: `0` empty step bodies in the touched step files. One new
fixture Given (`mixed_routing_change` in `tests/steps/grouped_merge_steps.rs`)
builds a real two-delta change directory; all other steps reuse existing
bindings listed in design.md.

**Commit cross-check**:

| Task | Commit | Production code? |
|---|---|---|
| Planning layer | `719d92a` | Planning artifacts (pre-implementation by design) |
| Blocked archive (`@e2e`) | `f425541` | Yes — `src/cli/archive.rs` fail-closed arm + fixture routing decisions |
| Nothing merged on mixed routing (`@e2e`) | `89422e9` | Yes — prevalidation pass, `resolve_permanent_dir`/`delta_capability` extraction |

**Promised components**: the blocking error, the prevalidation pass, and the
fixture helper `fixture_routing_proposal` — all in the tree.

**Strict-mode proof**: `.fail_on_skipped()` (existing); both RED runs exited
101 (evidences recorded in tasks.md — archive succeeded via the flat
fallback in RED 1; partial merge persisted in RED 2).

**E2E downgrade check**: both @e2e scenarios drive the real `givn archive`
subprocess end-to-end (init → completed change with deltas → archive with
all gates) and assert CLI exit code, stderr wording, absence of merged
permanent files, and the surviving change directory. Real interface.

**verify.e2e_command isolation**: both scenarios pair the unconfigured-e2e
fixture with deltas containing zero @e2e scenarios, so the e2e gate is
never reached; the block fires strictly earlier. Change scope: 2 e2e
scenarios vs the permanent corpus's superset — no downgrade.

**Design conformance**: as-built matches design.md, including the
"error surfaces before any file is written" claim — that claim initially
held only for single-delta changes; the grilling pass found the partial-merge
gap, it was spec'd as a second scenario, and prevalidation now enforces the
claim universally.

**Interaction coverage**: no new inventory entry (variant of the existing
archival action), as declared in design.md.

**Arc42 conformance**: chapter-selection table independently re-derived —
only chapter 6 (Runtime View, stage 7 sentence) affected; claim verified
against `docs/arc42/06-runtime-view.md:171`. No other chapter touched;
matches arc42.md's self-report.

**ADR qualification**: the fail-closed rule fails the five-dimension gate
(complete rationale owned by design.md + the archival group's permanent
spec = lower_level_artifact; reversal is one branch = durable_consequence).
No MADR entry; canonical destinations: design.md + spec delta.

## Dispositions

| Finding | Disposition |
|---|---|
| (none — `givn lint` exits 0) | — |

Grilling findings dispositioned during hardening:

| Grilling finding | Disposition |
|---|---|
| Partial merge persisted when one delta file of a multi-file change was unrouted (minor) | Fixed: prevalidation pass merges nothing until every target resolves; spec'd as scenario `Nothing is merged when one delta file of a change has no routing decision` |
| Stale comments describing the removed flat fallback (minor) | Fixed in the same commit |
| Block message names 'NEW in <group>' even when an EXTEND row exists for a nonexistent capability (note) | Accepted: fail-closed outcome identical; message remains accurate (no NEW decision recorded) |
| `NEW in <group>` group identifier joined unvalidated (note, pre-existing) | Out of scope: unchanged by this diff, traversal surface not widened; candidate for a future hardening change |
| Superseded flat-layout permanent scenario | Supersession: removed via `@givn.removed`, replaced by `Archive is blocked when a new capability has no declared routing decision` |

## Coverage

Measured per `./measure-coverage.sh` + `./merge-coverages.sh`; the
non-zero production probe for the blocked arm is
`cli::archive::process_delta_file` / `resolve_permanent_dir`.

README-IMPACT: none

## Sign-off

REVIEW: PASS
