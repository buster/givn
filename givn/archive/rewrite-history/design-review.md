# Design Review: rewrite-history

## Grilling

A fresh-context grilling subagent reviewed the proposal, use-case document,
delta, design, arc42 marker, and the implemented tree, and reproduced the
command against live fixtures.

Findings and resolutions:

1. **Should-fix — the happy-path scenario proved the rewrite only on
   non-`@e2e` steps.** The scenario runs the real subprocess against a real
   `git-filter-repo`; the interaction matrix now names all three real CLI
   interactions. No separate `@e2e` tag is added because the permanent
   corpus's e2e scope must stay action-scoped; the command's effect is
   verified through the real binary in the delta suite and re-verified by
   the archive against the merged candidate.
2. **Note — `git-filter-repo` normalizes the remote and recommends a fresh
   clone.** Givn reports the force-push need and names the boundary; the
   backup policy stays with the tool, matching seed Q20.
3. **Note — no new ADR.** Both the reviewer and `arc42.md` route the
   decision to the generated guidance plus 8.7.

All other branches were clean: scope matches the proposal (five scenarios,
one command, no extras); the tech split (command module over the existing
policy, backend, and cleanliness primitives) is the simplest fit; error
paths are covered (versioned, Fossil, dirty, absent confirmation, missing
tool); every scenario fails RED without the command (confirmed by running
the delta before the command existed as part of the TDD loop); no invented
Personas; the CLI-text classification holds; chapter 5/8 edits match the
tree; the glossary already carries "History rewrite".

## Persona disposition

- `developer--primary-user` — the only way to publish a clean history is
  explicit and reviewable.
- `givn-maintainer--owner` — bounded behavior: no push, no automatic
  rewrite, named refusals.

## Deferrals

None.

## Hardening applied

- Interaction matrix completed for all three CLI interactions.
- Delta, design, use-case, and arc42 records written before the
  implementation commit; `givn lint --change rewrite-history` clean.
- Delta suite 5/5; `cargo clippy -- -D warnings` clean.

DESIGN-REVIEW: PASS
