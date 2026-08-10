# Design Review: init-agent-target-selection

## Grilling

A grilling subagent with fresh context interrogated the full plan (proposal,
specs, design, arc42) against the implementation and test harness. Findings
and resolutions:

### Blockers resolved

**F1 — Exclusion scenario forced unintended cleanup semantics.** The original
scenario "Skills and commands are not installed to an agent excluded from the
selection" started from an init'd project (opencode files already on disk)
and asserted their absence after sync — GREEN would have forced sync to
delete files, contradicting design/ADR-036/R-12 (write-only sync, orphans
preserved).
Resolution: keep sync **write-only**; rewrote the scenario to start from a
bare root with a hand-scaffolded `givn/config.yaml` (`targets: [claude]`),
asserting that the first sync writes only the selected target. Design.md now
states the write-only semantic explicitly (stale files in unselected targets
are deliberately left in place — removal is a manual decision per R-12).

**F2 — Re-init scenario was blind to clobber-on-EOF.** The scenario started
from persisted `"opencode"` — identical to the EOF default, so an
implementation that re-prompts on every init and clobbers with the default
would have passed while violating the persistence promise.
Resolution: scenario now starts from persisted `"claude"`; clobber-on-EOF
fails RED, skip-prompt stays GREEN.

### Should-address resolved

**F3 — `--force` re-init untested.** Added scenario: init'd with
`opencode, claude` → `givn init --force` (null stdin) → targets reset to
`"opencode"` (force replaces config, so the prompt runs and EOF yields the
default). Design.md records the force-re-prompts rule.

**F4 — Invalid-answer semantics ambiguous.** Adopted drop-invalid: unknown
names are dropped with a warning listing valid agents; zero known agents /
empty / EOF → default. Scenario reworked to answer `"claude, nope"` → targets
`"claude"` + hint on stdout, plus a new scenario pinning the zero-known-agents
fallback to `"opencode"`.

**E2 — Single-scenario command would run against the permanent spec.**
`--name` with zero matches exits 0, so RED would report GREEN. Test runner
commands in design.md now set
`GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/`.

**F5 — arc42 chapter 4 contradicted by the strategy row.** Updated
`04-solution-strategy.md` (configured targets instead of unconditional
mirrors) and flipped arc42.md row 4 to Yes.

**F7 — Risk.** Primary failure modes were F1/F2 (both fixed). The permanent
spec audit came back clean: agent-directory references exist only in
`install-extras.feature` (covered by this change's delta) and the arc42 sync
step (narrowed to configured targets); noted in design.md.

### Verdict

Plan hard and consistent after hardening. All branches re-checked:

- Scope: spec now matches proposal exactly; persistence promise pinned.
- Tech: std-only prompt confirmed (F8 verified: `OverlayDoc` is lenient, no
  `deny_unknown_fields`, `skill:` block survives `givn upgrade`, line-based
  write matches the `set_addon` precedent).
- Testability: all 12 scenarios are RED-able via subprocess + piped stdin;
  `fail_on_skipped()` is active (`tests/features_runner.rs:20`).
- E2E fidelity: CLI interface, exactly one `@e2e` per inventory entry,
  assertions on real subprocess output.
- Interaction coverage: all 3 inventory entries mapped to matrix rows with
  valid driving mechanisms.
- Arc42: independent 12-row re-derivation matches the table (rows 3, 4, 5, 6,
  8, 9, 11, 12 Yes; others No, each with reasons); ADR-036 well-formed;
  R-12 present; chapter edits match design.md.

_Note (F6): the grilling flagged that the modified install-extras scenario
dropped the `givn-dev-principles` assertion. Verified false — the assertion
is present in the delta. No action._

---

## Status

DESIGN-REVIEW: PASS