# Design Review: retire-semantic-capabilities

## Coverage

Every required concern is covered by design.md:

| Question | Answer | Where |
|---|---|---|
| New components? | None — deletion only | design.md §Technology decisions |
| New step definitions? | None — deltas have zero scenarios | design.md §Step Definitions |
| Mechanism | Pre-shipped `@givn.retired` archive op + tombstone | design.md §Overview |
| Feature-flag interaction? | Cargo features dropped in a later change; suite stays green now | design.md §Architecture Impact |
| Rollback safety? | Tombstone and deletions covered by archive backup/rollback | design.md §Architecture Impact |

## Stress-test

- **What if a retirement delta carries a wrong `@<cap>` tag?** Archive
  validates the tag against the existing `givn/specs/<cap>/` directory and
  refuses when it doesn't exist (Phase 0 behavior). Each delta was authored
  from a directory listing taken immediately before authoring.
- **What if archive fails midway?** The whole `givn/specs` tree is backed
  up before the merge loop; rollback restores deleted directories and
  clears the tombstone. Covered by the Phase 0 rollback test.
- **Can the retirement list creep?** The list is closed: 15 capabilities,
  enumerated in proposal.md, one delta each. The four `spec-*` subcommand
  capabilities are explicitly excluded.

## Open Questions

None.

DESIGN-REVIEW: PASS
