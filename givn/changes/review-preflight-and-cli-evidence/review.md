# Review: review-preflight-and-cli-evidence

## Fabrication audit

- 0 empty step bodies across the archive-proof, workflow-enforcement, and support step files.
- Six checked scenarios carry six unique real revisions. Each revision exists, is reachable from HEAD, and its message body names the scenario; the strict check verified all six.
- The SCM commit seam, scenario parser, blocking check, template rule, and instruction rule exist in the tree.
- Strict mode is `.fail_on_skipped()`; the proof is in the design and the suite.
- No `@e2e` tag was removed. The change adds no E2E action; the existing CLI driver is the real interface.
- `verify.e2e_command` is `./run-tests.sh --e2e`, distinct from `./run-tests.sh --non-e2e`.
- Interaction coverage: the new checks are variants of `givn check tasks`; the visual marker is review evidence; CLI transcripts are archive evidence. No new inventory action.

## Arc42 conformance

| Fact | Durable source | arc42.md | design.md | tasks.md | Implementation | Match |
|---|---|---|---|---|---|---|
| Unique, real, scenario-named revision rule | 02 TC-40 | Yes | D2 | Scenario tasks | `check_scenario_commits` | Yes |
| Blocking check tasks/review gate | 06 6.7.1 | Yes | D4 | Scenario tasks | `run_change_verification`, `check_change_receipt` | Yes |
| SCM commit inspection | 05 Backend row | Yes | Impact table | Scenario tasks | `commit_exists`, `commit_message`, `is_ancestor` | Yes |
| CLI transcript evidence | 08, 12 | Yes | D1 | Scenario 6 | `is_evidence_path` | Yes |
| Not-applicable visual review | 10 Q-106 | Yes | D3 | Scenario 5 | `check_visual_review` | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage classification

- Dead code: none.
- Missing coverage: none; six scenarios cover the commit gate and evidence paths.
- Hard to test: Fossil ancestry is existence-based because `fossil info` exposes existence but not a simple ancestor query; recorded as a boundary, not deferred.
- Redundant tests: none.

## E2E and visual

The change adds no `@e2e` scenario. The archive/check paths are exercised through the compiled CLI. `Interface: none`; no screenshot or transcript sign-off is required. The opposite classification does not hold: no HTML, CSS, or JavaScript surface.

## Use case and Persona

The verify-change contract gains the unique-revision rule; archive-change keeps the evidence rule. The existing personas remain the review lenses; no new Persona.

## Deterministic gate dispositions

### Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Change evidence does not block the archive | CLI interaction transcripts are declared evidence | boundary |
| givn addons disable when already disabled is idempotent | A machine-only CLI records not applicable and passes review | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | A machine-only CLI records not applicable and passes review | boundary |
| The check passes on a complete README | A machine-only CLI records not applicable and passes review | boundary |
| Updated impact decision passes the review | A machine-only CLI records not applicable and passes review | boundary |
| A disabled addon needs no impact decision | A machine-only CLI records not applicable and passes review | boundary |
| A recorded impact decision passes the review | A machine-only CLI records not applicable and passes review | boundary |
| Integrity hook passes when arc42 is disabled | A machine-only CLI records not applicable and passes review | boundary |
| Integrity hook passes on edited arc42 chapter | A machine-only CLI records not applicable and passes review | boundary |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A machine-only CLI records not applicable and passes review | boundary |
| Review passes when a long scenario has a split-or-keep entry | A machine-only CLI records not applicable and passes review | boundary |
| Review passes when a shape match exists only between permanent scenarios | A machine-only CLI records not applicable and passes review | boundary |
| A modified delta matching its merge target in the same capability is allowed | A machine-only CLI records not applicable and passes review | boundary |
| A removed delta referencing its target by title is allowed | A machine-only CLI records not applicable and passes review | boundary |
| The default scope excludes the archive tree | A machine-only CLI records not applicable and passes review | boundary |
| givn check with known artifact proceeds to run hooks | A machine-only CLI records not applicable and passes review | boundary |
| A visual change with complete evidence passes review | A machine-only CLI records not applicable and passes review | boundary |
| A change without a visual contract is exempt from the visual gate | A machine-only CLI records not applicable and passes review | boundary |
| A terminal change passes review with a transcript | A machine-only CLI records not applicable and passes review | boundary |
| A change that declares no visual interface is exempt | A machine-only CLI records not applicable and passes review | boundary |
| A web contract with complete state evidence passes review | A machine-only CLI records not applicable and passes review | boundary |
| A visual change with a confirmed persona disposition passes review | A machine-only CLI records not applicable and passes review | boundary |
| A deferral with an operator decision passes review | A machine-only CLI records not applicable and passes review | boundary |
| A change without a visual interface passes review with a reasoned none | A machine-only CLI records not applicable and passes review | boundary |

### Split-or-keep

| Scenario | Decision |
|---|---|

No long-scenario finding.

## README impact decision

README-IMPACT: none

## Sign-off

- [x] Fabrication audit clean.
- [x] Every scenario has a real, unique, scenario-named revision.
- [x] No one-pass or shared-revision mode exists.
- [x] Strict mode, E2E scope, interaction coverage, and arc42 conformance verified.
- [x] No unresolved coverage gap.

REVIEW: PASS
