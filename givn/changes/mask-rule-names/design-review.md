# Design Review: mask-rule-names

## Grilling

A fresh-context grilling subagent reviewed the proposal, design, arc42
marker, delta, and the implemented tree. Findings and resolutions:

1. **Should-fix — the pattern list is hand-maintained and detached from the
   generated-path list.** Added unit tests proving every rule shape is
   covered and user content (`AGENTS.md`, `.agents/skills/gh-stack/SKILL.md`)
   is not; removed the dead `.claude/settings.json` filter.
2. **Should-fix — the leak assertion scanned the whole `.gitignore` and only
   one name.** Added a block-scoped step that asserts none of a name sample
   (`givn-propose`, `givn-spec`, `givn-archive`, `grill-me`) appears inside
   the fenced block; the negative step is now substring-based.
3. **Low — the integration files still publish fixed Givn file names.**
   Reworded the proposal outcome to "no individual skill or command name";
   the fixed integration paths remain (masking them would be dishonest about
   paths the tool must know exactly).
4. **Low — the Fossil block is untested.** The reviewer confirmed empirically
   that Fossil's `*` crosses `/` and the stripped form is cosmetic; the
   stripped form is kept for readability and the empirical check is recorded
   here.
5. **Low — `arc42.md` recorded `alternatives: FAIL` despite a real choice.**
   Corrected to `PASS` with the two options recorded.
6. **Info — `write_ignores` returned entries not written on Fossil.**
   It now returns the written list per backend.

All other branches were clean: scope matches; the transition and cleanliness
predicate are unchanged and still discover tracked rules; the modified
scenario fails RED against the old per-path block.

## Persona disposition

- `developer--primary-user` — a published repository no longer enumerates
  rule names.
- `external-contributor--open-source-consumer` — fewer Givn traces in the
  published tree.
- `givn-maintainer--owner` — the reserved namespace stays documented and
  tested.

## Deferrals

None.

## Hardening applied

- Unit tests for pattern coverage and user-content safety.
- Block-scoped, substring-based leak assertion.
- Proposal outcome reworded; arc42 gate value corrected; Fossil evidence
  recorded.
- `cargo clippy -D warnings` clean; lib tests 268 pass; delta 1/1.

DESIGN-REVIEW: PASS
