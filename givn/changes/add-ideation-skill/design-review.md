# Design Review: add-ideation-skill

## Phase 1: Grilling (fresh-context subagent)

All ten branches walked by an independent grilling subagent with full
codebase tracing and a live probe of the no-addons init fixture.

**Findings:**

1. **Testability (critical):** the always-on scenario could never go GREEN.
   `target_skill_paths` (tests/steps/prose_consolidation_steps.rs) discovered
   targets by substring-matching the entire `givn/config.yaml` including
   scaffold comments that name "agents" and "claude"; the no-addons fixture
   writes `targets: [opencode]` only, so the guidance step demanded files sync
   never writes there and panicked even after a correct implementation. No
   existing scenario had ever paired that Given with a guidance step.
2. **Scope (four unpinned proposal promises):** session resume, the
   `docs/ideation/` handoff publish target, quality reviews, and the changed
   `assets/commands/givn-explore.md` asset had no asserting scenario or
   contract phrase.
3. **Overlap:** the persona-binding and state scenarios had shape-identical
   masked fingerprints; the review gate would report deterministic findings
   and possible delta-to-delta semantic candidates (unsupported remediation
   path, R-19).
4. Clean branches: scope otherwise exact; tech choices verified against the
   `givn-explore` precedent (default-skill registration is unconditional in
   `sync()`; `reconcile_addon_artifacts` never touches default skills);
   E2E fidelity and interaction coverage clean (one inventory bullet ↔ one
   matrix row ↔ one `@e2e`); ADR verdict confirmed (all three candidates fail
   the five-dimension gate; `CANONICAL_ARTIFACT` routing is correct; no
   existing ADR covers the boundary); arc42 independently re-derived with
   full 12-row agreement and content verified for every Yes row (no ASCII
   art in any chapter); ubiquitous language consistent across proposal,
   spec, design, and glossary; all contract phrases embeddable.

## User questions and decisions

1. Always-on scenario fix — **root-fix `target_skill_paths`** (parse the real
   `skills: targets` list via the production config loader) instead of
   rewriting the scenario's Then. Decided: root fix.
2. Four unpinned promises — **pin all four**: `quality review`,
   `session.md`, `docs/ideation/`, and `the generated givn-explore command
   contains "givn-ideation"`. Decided: pin all four (explore command edit
   kept and covered).
3. Shape-identical scenarios — **keep separate** with pre-planned dispositions
   (one behavior per scenario; classify at review time as VALID-BOUNDARY with
   the differing assertion sets as the observable distinction). Decided: keep
   separate.

## Phase 2: Hardening (fresh-context subagent)

Applied:

- `tests/steps/prose_consolidation_steps.rs`: `target_skill_paths` now uses
  the production parser `givn::config::overlay::load_skills_targets`; the
  no-addons fixture yields `[opencode]`, the all-targets fixture still yields
  three targets; panic-on-missing downstream unchanged.
- Delta spec: four new pins added; the `@e2e` delivery scenario additionally
  asserts the verbatim per-skill sync line `synced skill: givn-ideation`
  (verified against the `println!` in `src/skills/targets.rs`).
- `design.md`: declares the shared-helper fix; rewritten interface-obstacle
  paragraph (fixture now selects `[opencode]`); corrected contract-phrase
  table (always-on is a content assertion; five new phrases recorded); the
  givn-explore command asset row marked covered; RED anchors extended; an
  overlap paragraph records the verified findings (SHAPE between phase-ladder
  and state scenarios, persona-binding a SUBSET of both) and the planned
  VALID-BOUNDARY disposition per decision 3.
- One evidence-based deviation: the overlap paragraph names the lint-observed
  finding pair rather than the pre-verification hypothesis; the decided
  disposition is unchanged.

Verification:

- `cargo check` and `cargo check --tests`: exit 0.
- `givn lint --change add-ideation-skill`: exit 2 (six expected `@wip`
  findings plus informational SHAPE/SUBST findings; within the 0-or-2
  allowance).
- Regression through the fixed helper: `GIVN_FEATURES=givn/specs/support
  ./run-tests.sh` — pass, 95/95 scenarios, 426/426 steps (includes the
  skill-rewrites `@e2e` and template-conditional scenarios that exercise the
  changed helper).

All questions resolved; all hardening edits applied and verified.

## Post-sign-off sequencing edit

Before writing tasks.md, the delta scenarios were reordered (no contract
change): the always-on scenario is first among the non-`@e2e` scenarios and
the delivery scenario stays last. Rationale: every scenario must have a
genuine failing RED in the scenario-by-scenario loop. The always-on scenario
is foundational (its GREEN creates the template plus `default_skill_names()`
registration); every later scenario then REDs on missing contract phrases.
The delivery `@e2e` scenario keeps a genuine RED because the command-side
registration (`default_command_names()` plus `assets/commands/givn-ideation.md`)
is deferred to its GREEN — its RED fails on the missing generated command
file. Scenario content, tags, and phrases are unchanged; `givn lint --change`
findings are unchanged.

## Amendment re-pass (final-verification finding)

The full suite failed one permanent scenario
(`template-conditional-rendering.feature`: "the generated guidance omits
disabled-addon sections") — its step scans every generated skill and command
for disabled-addon markers (`arc42`, `glossary`, `ubiquitous`,
`domain-modeling` case-insensitive, plus coverage marker strings). The
always-on ideation template named `docs/arc42/12-glossary.md` and the
glossary. Addon-gated skills are exempt (removed when disabled); always-on
skills never contain the markers — confirmed against every other always-on
template.

Amendment applied to `design.md` (state file renamed `glossary-candidates.md`
→ `domain-terms.md`; Frame/Event-Storming/Handoff wording routes to "the
durable term list" without naming the addon-owned artifact; behavior
unchanged — candidates are captured per topic and merges are user-decided at
handoff). Fresh-context re-review verdict: PASS — spec delta needs no change
(no scenario asserts a banned marker or the old file name), proposal not
invalidated (HOW-level wording only), arc42 records consistent, no test or
permanent fixture touched. TEMPLATE-REVIEW: PASS (amendment).

DESIGN-REVIEW: PASS