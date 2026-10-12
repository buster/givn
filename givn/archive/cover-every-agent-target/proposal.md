# Proposal: cover-every-agent-target

## Release note

Givn's guidance reference check now follows the production agent-target list,
so a harness added to Givn is covered automatically and a configured target
without synchronized guidance fails the check instead of passing silently.

## Use-Case Context

- Use-case ID: fragment `support` — capability `instruction-integrity`.
- Ideation topic: none — originates from the operator's follow-up review of
  the archived `validate-generated-references` change (2026-10-12).
- Seed path: none.
- Confirmed Personas: `givn-maintainer--owner` (owns the acceptance suite and
  the production target list), `agent--technical-role` (its harnesses are the
  graded surface). Existing confirmed project records are reused; no persona
  is created or promoted.

## Problem / Opportunity

The consumer-view reference check recognizes agent targets through static
copies of production knowledge, and its fixtures hardcode a subset of the
supported targets:

- `tests/common/reference_check.rs` keeps its own target list
  (`CANDIDATE_ROOTS`, `AGENT_TARGETS`) beside the production
  `KNOWN_AGENT_TARGETS` / `AgentTarget::ALL`;
- the shared fixture behind "a givn project with all agent skill targets
  enabled" answers the init prompt with `opencode, .agents, .claude` — `.omp`
  is never exercised by any reference-check scenario, although it is a
  supported target and owns commands;
- the characterization command step hardcodes `["opencode", "omp"]`, a second
  copy of `AgentTarget::owns_commands`.

Consequences, concretely:

- Adding a fifth agent target to production leaves the check silently blind:
  a guidance reference naming the new target is neither extracted nor
  resolvable by name until the test copies are updated, and nothing fails if
  that update is forgotten. The automatic tripwires are the
  `givn skills targets add all` and `givn init --agents all` scenarios, which
  assert the exact target list — nothing else in the suite notices.
- The check's positive scenarios require only that *some* guidance exists. A
  configured target whose files were never synchronized passes silently,
  because the corpus simply contains fewer sources.
- A scenario named "all agent skill targets enabled" exercises three of four
  supported targets; the name and the coverage disagree.

## Proposed Solution

- The reference check's recognized agent-target names come from the
  production target list (`AgentTarget::ALL`), so adding a target
  automatically makes references naming it visible and resolvable by skill
  name — no test edit is required for recognition.
- Every "all targets" fixture and every "every target" loop derives its
  selection from the production list, so every scenario using them grades,
  and honestly exercises, each supported target.
- The installed-guidance positive requires synchronized guidance from every
  configured target: "no findings" can no longer be proven when a target
  contributed nothing. The printed-instructions positive is exempt — its
  corpus is text, which has no target dimension.
- A configured target without synchronized guidance fails the check, naming
  the target.
- The characterization command assertion derives "command-owning target" from
  production instead of a literal list.

## Capability Routing

Route reported no signal (the top candidates tied). The decision:

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `instruction-integrity` | none — route reported no signal | `NEW in fragments` | The capability lives in the reusable `support` fragment (`givn/specs/fragments/fragment.md`); fragment capabilities carry no use-case directory, so the delta lives under `specs/fragments/`. The coverage guarantee extends the same actor goal as the capability's existing scenarios: generated guidance remains complete and trustworthy for every supported harness. |

## Out of Scope

- No production agent target is added, removed, or changed; the production
  target list stays the single source this change follows.
- The `enable-optional-addons` stub fixture keeps its explicit three-target
  assertions; those scenarios name targets deliberately, and this change does
  not rewrite them.
- No generated guidance text changes; the content fixes from
  `validate-generated-references` stay as they are.
- No runtime command is added; the check remains part of the acceptance
  suite.

## Open Questions

None. Both test-design forks — deriving recognition from production, and
requiring per-target guidance in the positives — are consequences of the
approved problem statement; the design fixes their exact mechanics.
