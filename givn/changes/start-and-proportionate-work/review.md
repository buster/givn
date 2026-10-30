# Review: start-and-proportionate-work

Review kind: self-review
Author: Maki
Reviewer context: Maki

README-IMPACT: updated - Quick start

PERSONAS: agent--technical-role
PERSONAS: developer--primary-user
PERSONAS: givn-maintainer--owner

## Scope

This change replaces the `givn-explore` entry point with `givn-start`, separates
session state from request routing, and removes recognizable generated explore
output from existing projects. The review covers 21 delta scenarios (20 regular,
1 e2e), the start instruction, the routing text/JSON, skill retirement, upgrade
staging, onboarding, and the documentation sweep.

## Implementation evidence check

| Check | Result |
|---|---|
| `@e2e` tag integrity | The modified startup scenario retains `@e2e`; no E2E action was removed or added beyond that existing action. |
| Empty or trivial step bodies | 0 empty bodies across the changed step files. |
| Checked tasks with commits | Every checked scenario records a batch revision touching implementation, tests, or guidance. |
| Promised components exist | `assets/instructions/start.md`, `assets/skills/givn-start/SKILL.md.tmpl`, the retired-id refusal, `reconcile_retired_skills`, `retired_managed_generated_paths`, and upgrade staging exist. |
| Strict-mode proof | `tests/features_runner.rs` calls `.fail_on_skipped()`; the undefined-step probe exited non-zero. |
| E2E primary assertions | The startup scenario runs real `givn instructions` subprocesses and asserts stdout and the transcript. |
| `verify.e2e_command` | `./run-tests.sh --e2e`; archive scope uses `GIVN_RESULT_SCOPE=e2e`. |
| Design deviation | None: the implementation matches the design's file layout, commands, and component boundaries. |

### Commit evidence

| Batch | Revision | Contents |
|---|---|---|
| start-entry-routing | `324ccc15` | Start instruction, explore removal, ideation/dialog/agent-hint updates, retired-id refusal, no-active text/JSON, startup steps |
| skill-retirement | `2df5429a` | Start skill template, default list, retirement reconciliation, upgrade staging, onboarding, generation/cleanup steps |
| start-documentation-closeout | `0fed58ed` | README, reference, workflow, diagram, arc42 chapters, glossary, handoff update, change artifacts |
| start-entry-e2e | `5ef3624b` | E2E startup assertion, delta feature, transcript |
| start-task-evidence | `33b36388`, `a84e1ad3`, `2d7836e1` | Task evidence, `@wip` removal, verification receipt |

### Interaction coverage

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `serve-extended-rules`: request effective instructions by id | Present | An agent receives the extended rules through the Givn CLI | Real `givn instructions` subprocess |
| `instruction-integrity`: request normal session-start instructions | Present | Author starts a session with the universal dialog obligation | Real `givn instructions` subprocess |
| `refresh-generated-guidance`: synchronize selected skills | Present | Synchronized guidance explains how to resolve the deterministic gates | Real `givn skills sync`; retirement cases are regular variants |
| `initialize-a-project`: initialize with explicit noninteractive selection | Present | Consumer runs init non-interactively and gets opencode skills | Real `givn init --no-addons --agents opencode` subprocess |

No capability has more than one `@e2e` scenario for the same normalized action.
The start comparison in the ideation feature has no inventory row in the
existing support fragment; this change does not invent one, and the comparison
is a regular content variant.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `givn instructions` no-active routing | `docs/arc42/03-context-and-scope.md` 3.2 | Yes | Decisions and module changes | Batch A | `src/cli/instructions.rs` | Yes |
| Proportionate entry strategy | `docs/arc42/04-solution-strategy.md` 4.1 | Yes | Overview | Batch A | Start instruction and routing | Yes |
| Init/instructions/skills responsibilities | `docs/arc42/05-building-block-view.md` 5.3 | Yes | Module changes | Batches A/C | Source and assets | Yes |
| Init onboarding, startup guidance, sync retirement | `docs/arc42/06-runtime-view.md` 6.1/6.3/6.12 | Yes | Module changes | Batches A/C | Source and scenarios | Yes |
| 8.7 retirement and 8.28 entry policy | `docs/arc42/08-crosscutting-concepts.md` | Yes | Overview | Batch D | Chapter sections | Yes |
| R-128/R-129 | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Risks | Batch D | Risk rows | Yes |
| Start, Direct work, Proportional routing | `docs/glossary.md` | Yes | Decisions | Batch D | Glossary terms | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`docs/glossary.md` is the sole vocabulary authority. **Start**, **Direct work**,
and **Proportional routing** are defined there and used consistently in the
instruction, design, specs, and documentation. `explore` remains only as a
retired identifier in migration, legacy cleanup, and history; it has no
authoritative definition and no callable entry point.

## Coverage

The delta coverage run measured the changed scope through
`./measure-coverage.sh` and merged it with `./merge-coverages.sh`. The changed
paths are exercised: the no-active branch and `next_skill` by the startup
scenarios, the retired-id refusal by the alias scenarios, and
`reconcile_retired_skills`, `is_generated_skill`, `retired_managed_generated_paths`,
and upgrade staging by the retirement scenarios.

The project's existing coverage command produces line coverage with a zero
branch rate because branch instrumentation is unstable in the pinned stable
toolchain. This is a pre-existing measurement limitation, not a gap introduced by
this change. Bucket 3 justification: branch measurement requires an unstable
toolchain flag outside this change's scope. The archive receipt will carry the
full-suite measurement.

No dead code was added. The new unit tests cover retirement recognition, the
retired managed list, and the default skill list.

## Visual review

The design declares `Interface: terminal`. The modified `@e2e` scenario committed
a transcript at
`givn/changes/start-and-proportionate-work/evidence/visual/author-starts-a-session-with-the-universal-dialog-obligation/transcript.txt`.
It shows the real `givn instructions` output and exit code. The opposite
classification (Web UI) does not hold: no HTML, CSS, or JavaScript is rendered.

Persona dispositions:

- `agent--technical-role`: receives start routing, the direct alternative, and
  the state/request separation; no finding.
- `developer--primary-user`: can choose direct work or a formal change without a
  mandatory change ceremony; no finding.
- `givn-maintainer--owner`: sees the retirement and upgrade behavior documented;
  no finding.

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

None remaining. The one permanent-suite non-e2e failure is the scenario the
delta removes:

- `The explore guidance points to ideation for structured domain work` — replaced
  by `The start guidance points to ideation for structured domain work`.

All other permanent non-e2e scenarios pass. The permanent E2E suite passes.

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
