# Design Review: cover-every-agent-target

## Grilling

A fresh-context grilling subagent (read-only, 2026-10-12) read the proposal,
the delta feature, `design.md`, the check module, the three step files, the
production `AgentTarget` API, and all seven feature files that use an
"all agent skill targets" fixture. It ran the four-target fixture experiment
in a full repository copy, both fixture variants, and an independent
`installed_guidance`/`unresolved_references` run against a real four-target
project under `/tmp`. It returned five questions and ten findings.

The operator delegated the lifecycle ("Do that", 2026-10-12). Every question
resolved from repository evidence; none required operator arbitration.

- Q1 — the fourth fixture. `prose_consolidation_e2e_steps.rs:71,94` is a
  fourth "all agent skill targets" Given that still selects three targets.
  Resolved: extend it; it now derives from `AgentTarget::ALL` like the other
  three. The subagent verified the scenario passes with four targets.
- Q2 — two "every target" loops iterate a literal three-target list
  (`core_entries_require_dialog_policy`, `skill_consumers_receive_replacement`).
  Resolved: both derive from the configured targets, closing the silent
  three-of-four grading.
- Q3 — the recognition refactor has no RED-able scenario and could regress to
  a stale literal silently. Resolved: `recognizes_target_dir` is public and
  the coverage step asserts it for every `AgentTarget::ALL` entry — an
  executable coupling guard.
- Q4 — the Q-97 row describes only the unresolved-reference failure. Resolved:
  the row gains the no-guidance clause; the 10.3 bullet names the
  production-derived coverage.
- Q5 — chapter 11. The new coverage check means "at least one source per
  configured target", not completeness; a partially synchronized target still
  passes. Resolved: D-21 records the boundary.

Findings and resolutions:

1. F1 (High) — fixture edits and the derived expectation must land in one
   atomic step; with the literal assertion left in place the permanent
   addon-free scenario fails (`targets: [opencode, agents, claude, omp]`
   versus the literal three). Resolved: the design groups them into one
   decision and the task list will carry them in a single scenario task.
2. F2 (High) — the overview overstated ("last static copies", "three
   fixtures"). Resolved: the overview now counts two modified plus one added
   scenario and names four fixtures plus two loops; the scope in the design
   and proposal matches what is built.
3. F3 (Medium) — the two literal loops (see Q2). Resolved.
4. F4 (Medium) — recognition refactor unguarded (see Q3). Resolved.
5. F5 (Medium) — the design's new-step table omitted the fourth step
   ("every command-owning agent target receives the givn-characterize
   command"). Resolved: the table lists all four.
6. F6 (Medium) — the proposal said the positives require per-target guidance
   while the design exempts the printed-instructions positive. Resolved: both
   artifacts now state the installed/printed split and why.
7. F7 (Low) — `enable-optional-addons` naming ambiguity. Resolved: the
   design names the exact helper and scenario and keeps them explicitly out
   of scope.
8. F8 (Low) — "the production target list" is two surfaces (config
   validation versus `AgentTarget::ALL`). Resolved: the design states the
   split and that the coverage step catches divergence loudly.
9. F9 (Low) — there are two exact-list tripwires, not one. Resolved: the
   proposal names both.
10. F10 (Low) — remaining hardcoded target knowledge judged deliberate.
    Resolved: recorded in the design's out-of-scope list with the exact
    files.

The subagent's independent arc42 read marked chapter 11 Yes (the coverage
boundary is new debt) where the design originally said No; the design now
records D-21. Its ADR verdict matches the design's routing
(`NOT_QUALIFIED` → `CANONICAL_ARTIFACT` to 10.3).

Required branches covered:

- **Scope** — the proposal's outcomes map to the delta's two modified and one
  added scenario; the fourth fixture and two loops were the only uncovered
  copies.
- **Tech choices** — test support reads the public production API
  (`AgentTarget::ALL`, `name`, `display`, `owns_commands`, `from_name`);
  verified to compile in a repository copy. No production change.
- **Missing scenarios** — the fail-closed scenario is the added delta; the
  coupling guard rides the coverage step.
- **Testability** — the subagent observed the delta RED: 3/3 fail; the
  add-on step fails with "Step doesn't match any function"; the modified
  scenario titles and step text match the permanent text byte-for-byte.
- **E2E fidelity** — no new consumer action; existing `@e2e` unchanged.
- **Visual contract / interface classification** — terminal; the opposite
  classification does not hold (no HTML/CSS/JS).
- **Persona disposition** — recorded below.
- **Deferrals** — none.
- **Interaction coverage** — no new inventory entry; fragment rows and matrix
  unchanged.
- **Risk** — the most likely failure is missing one of the remaining literal
  copies; the subagent's grep sweep is the mitigation and the loud
  addon-free failure catches the assertion pairing.
- **Domain context** — fragment `support`; actors stay distinct from
  Personas; no Persona invented.
- **ADR qualification** — independently re-derived: `NOT_QUALIFIED`,
  routing `CANONICAL_ARTIFACT` to `docs/arc42/10-quality-requirements.md`
  section 10.3; ADR-013 and ADR-015 were checked and are untouched.
- **Architecture documentation** — rows 10 and 11 Yes; the other rows No
  with reasons; chapter 8 explicitly unaffected because 8.7's rule text does
  not change.
- **Ubiquitous language** — no new term; "agent target" already defines the
  four destinations.

## Persona disposition

- `givn-maintainer--owner` — relevance: owns the acceptance suite and the
  production target list this change makes the suite follow. Evidence: the
  derived fixtures, the coverage guard, and D-21.
- `agent--technical-role` — relevance: its harnesses are the graded surface;
  a new harness now enters the check without test edits. Evidence: the
  production-derived recognition and the four-target fixture runs.

No Persona was invented or auto-promoted.
PERSONAS: givn-maintainer--owner, agent--technical-role

## Deferrals

None. No mandatory check is deferred.

## Hardening applied

- `proposal.md`: both tripwires named; the fourth fixture and the two loops
  included; the installed/printed split stated.
- `design.md`: overview corrected; the fourth fixture, the two loops, the
  recognition coupling guard, the Q-97 clause, and D-21 added; new-step table
  completed to four steps; the config-versus-`AgentTarget::ALL` split stated;
  the deliberate-literals list recorded.
- `givn lint --change cover-every-agent-target` exits 2 for the three expected
  `@wip` findings; the `[SUBST]` advisory (the no-addons scenario is a subset
  of the default-configuration scenario) is recorded for review-stage
  disposition.

DESIGN-REVIEW: PASS
