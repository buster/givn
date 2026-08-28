# Design Review: domain-modeling-glossary-enforcement

## Grilling questions and resolutions

1. **"Practically every step" — does the router name the domain skill for
   every artifact, including `archive`/`install`?**
   Resolved: with the predicate `domain_modeling && arc42`, `run_artifact`
   names the skill for any artifact. Naming it at archive/install is harmless
   and consistent with lifecycle-wide loading. Paths without a change
   (`report_no_active_changes`, `report_multiple_changes`) never call
   `report_change` and emit no pointer — no noise there.

2. **Does the JSON contract regress?** No. The existing scenario "…as JSON and
   the skill route is machine-readable" asserts `skill_to_load` is present at
   the specs stage; the widened predicate keeps it true and additionally
   populates it for later phases. `skill_to_load` remains absent when the
   addon set is off — existing consumers unaffected.

3. **Is the "awaiting review" fixture reproducible?** Verified against
   `artifact_status` in `src/state.rs`: `tasks` is Done only with a fully
   checked box list, `design-review` only with the `DESIGN-REVIEW: PASS`
   marker. The design now pins the exact fixture steps (marker-bearing
   design-review.md, complete tasks.md, absent review.md) — implementable and
   deterministic.

4. **Will the new "later change phases" scenario collide with the existing e2e
   next-step scenario in the semantic review?** Likely — both assert "the
   guidance says to load the domain-modeling skill now". Accepted: it is a
   distinct invariant (lifecycle-breadth routing at a post-specs stage, which
   only exists after this change); review will dispose it as a VARIANT with
   that rationale. Similarly the coupling-guard scenario and the permanent
   scenario "A project without the addon is not routed…" share a shape but
   differ in precondition (arc42 off vs addon off) — disposed as VARIANT. The
   removed "per-change ubiquitous-language table" / added "glossary" pair is a
   supersession, documented in review.

5. **Template-phrase coupling:** the generated skill assertions grep exact
   phrases from the template ("during every workflow step of a change",
   "maintaining the project's arc42 glossary"). The step definitions must use
   the literal template wording; the design pins the phrases so tasks can
   implement them in the template first, then bind steps.

6. **No scanner** — the scope decision is honored: only the deterministic
   config fact (arc42 present) is checked for the pointer; no term matching.

## Hardening

Design updated after grilling: the fixture paragraph now states the exact
marker/checkbox requirements for the review-stage fixture. No scenario
gaps found. No edits to proposal needed. No `tasks.md` written yet.

## Verdict

All questions resolved. Proceed to tasks.

DESIGN-REVIEW: PASS
