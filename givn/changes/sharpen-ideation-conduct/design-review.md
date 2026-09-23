# Design Review: sharpen-ideation-conduct

## Grilling

A fresh-context grilling subagent (spawned via the Task tool, 2026-09-23) read
the proposal, design, delta spec, arc42 marker, the target assets, the
permanent spec, the shared content-assertion steps, and the durable records.
Every required branch was covered:

- Scope — the delta covers the proposal; the two omitted proposal clauses are
  now asserted (finding 2).
- Tech choices — guidance-text change through the skill template and command
  guardrails is the right mechanism; no simpler alternative survives the
  requirement that both artifacts stay in sync.
- Missing scenarios — no permanent scenario asserts the old per-use-case rule
  or any phase confirmation, so no `@givn.modified` or `@givn.removed` delta
  is needed; the permanent assertions all survive unchanged.
- Testability — every asserted phrase is absent from the current assets, so
  all five scenarios fail RED; the exact-phrase mismatches are resolved below.
- Risk — the most likely failure is RED never turning GREEN because the
  rendered text differs in case or line wrapping from the assertion; the
  design now fixes the literal wording and the task includes a per-scenario
  GREEN proof.
- Use-case and Persona context — fragment `support` owns
  `generate-ideation-guidance`; all three named Personas exist and none was
  invented.
- Decision ledger — D1 and D2 resolve in the design's `## Decisions`; no
  carried item lacks an owner.
- Interface classification — CLI text output, no browser; the opposite
  classification does not hold because nothing renders a screen.
- Interaction coverage — the single matrix row matches the capability's
  existing `@e2e` scenario; the fragment carries no interaction inventory.
- ADR qualification — independently judged: neither decision qualifies
  (single-change reversible workflow policy, no boundary moves); the canonical
  home is the generated ideation guidance, not an ADR.
- Architecture documentation — the reviewer's independent 12-row derivation
  matched `arc42.md` except for the canonical-artifact naming, corrected
  below.
- Ubiquitous language — "Change seed", "Ideation", and "Quality review" are
  consistent after the glossary wording fix below.

## Findings and resolutions

1. Blocking: three delta assertions failed against the design's literal
   wording (case of "a bare option list", line wrapping of "phase report" and
   "dependencies and a suggested start order"). Resolved: the delta asserts
   the capitalized sentence and the design's wording was reflowed so each
   asserted phrase is contiguous on one line.
2. Should-fix: the proposal's live-question context, shared-direction
   question rule, and "one use case may yield several seeds" were unasserted.
   Resolved: assertions added for "context that forces it", "share one
   direction are one question", "one use case may yield several seeds", and
   the renamed seed path "changes/<change-id>.md".
3. Should-fix: the command guardrail text was unspecified while the spec
   asserted literal phrases. Resolved: the exact guardrail lines are recorded
   in the design.
4. Should-fix: `arc42.md` named two canonical artifacts and the generated
   output. Resolved: the single canonical home is now
   `assets/skills/givn-ideation/SKILL.md.tmpl`; the command repeats the rules
   in one line each.
5. Should-fix: the question section appeared to restate the canonical dialog
   policy, which arc42 8.3a forbids. Resolved: the template keeps the policy
   pointer and states only ideation-specific enforcement (a bare option list
   is a defect; never ask a decided direction again; phase reports; one
   question per direction). No 8.3a amendment is needed.
6. Note: the interaction matrix row did not read as an inventory entry.
   Resolved: the row now names the capability and records "fragment; no
   inventory", matching precedent.
7. Note: glossary wording drifted between "per-deliverable" and "per-change".
   Resolved: both records now say "per-change, cut by deliverable guarantee"
   and mirror both directions (several use cases per seed; several seeds per
   use case).
8. Note: the working tree holds the prior `local-only-integrations` ideation
   handoff as uncommitted files. Resolved operationally in tasks setup: the
   handoff is committed as its own documentation commit before any scenario
   commit, so this change's commits stay scoped and archive sees a clean
   state.

## Persona disposition

- `developer--primary-user` — high relevance; answers the questions and bears
  the re-asking cost. Evidence: the `local-only-integrations` session.
- `agent--technical-role` — high relevance; asks the questions and cuts the
  seeds, and consumes the written conduct rules.
- `givn-maintainer--owner` — relevance; owns the guidance corpus and the
  durable records amended here.

No Persona was invented or auto-promoted; the capability has CLI text output,
so confirmed Personas are named.

## Deferrals

None. No mandatory check is deferred.

## Hardening applied

- `design.md`: literal wording reflowed, exact command guardrails recorded,
  matrix row renamed.
- `specs/support/generate-ideation-guidance.feature`: assertions added and
  corrected.
- `arc42.md`: canonical artifact naming corrected.
- `docs/arc42/12-glossary.md`: Change seed row standardized.
- `givn lint --change sharpen-ideation-conduct` exits 0 (advisory overlap
  findings only).

DESIGN-REVIEW: PASS
