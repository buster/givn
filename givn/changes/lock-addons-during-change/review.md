# Review: lock-addons-during-change

Review kind: self-review
Author: Maki
Reviewer context: Maki

README-IMPACT: updated - Configuration

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Scope

This change records the effective add-on values when a change starts, refuses
add-on mutation while a locked change is active, detects direct configuration
drift in review and archive, and records an explicit TTY operator override. A
forced init that would change add-ons is refused before writing. Older changes
without a lock are unaffected.

## Implementation evidence check

| Check | Result |
|---|---|
| `@e2e` tag integrity | No delta E2E scenario was added or removed; existing add-on and review E2Es remain. |
| Empty or trivial step bodies | 0 empty bodies in the changed step files. |
| Checked tasks with commits | All eight scenarios and closeout tasks record batch revisions. |
| Promised components exist | `config::lock`, the add-on guard, the forced-init guard, review/archive checks, and the PTY helper exist. |
| Strict-mode proof | `.fail_on_skipped()` remains. |
| E2E primary assertions | The lock scenarios are regular variants; the PTY scenario drives the real binary. |
| `verify.e2e_command` | `./run-tests.sh --e2e`; no scope change. |
| Design deviation | None: lock format, locations, and override path match the design. |

### Commit evidence

| Batch | Revision | Contents |
|---|---|---|
| addon-lock | `aae0ac56`, `62d12d70` | Lock module, CLI guards, init guard, review/archive checks, PTY helper, scenarios |
| addon-lock-documentation-closeout | `492dff09` | README, reference, glossary, arc42 chapters, handoff outcome, change artifacts |
| addon-lock-task-evidence | `3b0d8bec`, receipt | Tasks and verification receipt |

### Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `enable-optional-addons`: enable/disable an add-on | Present | Existing add-on E2E | Real `givn addons` subprocess; lock cases are regular variants |
| `givn-workflow-enforcement`: request review checks | Present | Existing review E2E | Real `givn check review` subprocess |

The PTY override scenario is a regular variant of the add-on action and uses a real terminal.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Add-on command lock | `docs/arc42/03-context-and-scope.md` | Yes | Contracts | Batch A | CLI scenario | Yes |
| `config::lock` component | `docs/arc42/05-building-block-view.md` | Yes | Module changes | Batch A | Source | Yes |
| New/addon/init/review/archive flows | `docs/arc42/06-runtime-view.md` | Yes | Module changes | Batch A | Scenarios | Yes |
| 8.11a lock lifecycle | `docs/arc42/08-crosscutting-concepts.md` | Yes | Overview | Batch B | Chapter | Yes |
| R-132/R-133 | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Risks | Batch B | Chapter | Yes |
| Add-on lock/Operator override terms | `docs/glossary.md` | Yes | Decisions | Batch B | Glossary | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`docs/glossary.md` defines **Add-on lock** and **Operator override**. The lock
is change-local add-on policy, not a full configuration snapshot. The design,
docs, specs, and code use the same terms.

## Coverage

The delta features cover lock creation, refusal, non-interactive refusal, the
real-terminal override, forced-init refusal, preserving forced init,
no-active-change mutation, and review drift. Unit tests cover lock creation,
optional locks for old changes, and override recording. The review gate uses
the same `config::lock::check` that archive calls.

## Visual review

The design declares `Interface: terminal`. This change adds no new E2E action;
the existing add-on and review E2Es remain. The override scenario runs in a real
PTY and asserts the resulting configuration and recorded reason. No browser
surface exists.

Persona dispositions:

- `developer--primary-user`: add-on policy is protected and the operator path is available; no finding.
- `agent--technical-role`: the refusal names the policy and the missing-dependency path; no finding.
- `givn-maintainer--owner`: residual lock deletion and TTY override risks are documented; no finding.

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

None. Lock deletion hardening is explicitly out of scope for 0.9.16.

## Findings

None remaining.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching implementation, tests, or guidance.
- [x] Every promised component exists.
- [x] Every consumer uses only its declared contract.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded for the change scope.
- [x] Coverage classified; lock and review/archive paths are covered.
- [x] Dead code avoided; unit and PTY tests added.
- [x] No `@wip` tags remain; no implementation-layer detail in the spec.
- [x] Canonical E2E policy read; no new E2E action.
- [x] Existing E2Es remain real-interface.
- [x] `verify.e2e_command` unchanged.
- [x] Implementation matches design.md.
- [x] Interaction coverage verified.
- [x] No finding excused with an invented bucket.

REVIEW: PASS
