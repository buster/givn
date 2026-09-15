# Change seed: verify-change

- **Stable use-case ID:** `verify-change` (existing, modified)
- **Topic source:** `givn/ideation/multi-scm-support/`
  (`use-cases/verify-change.md`, eventstorming B8)
- **Confirmed Persona IDs:** `agent--technical-role`,
  `givn-maintainer--owner`
- **Accepted decisions:**
  - Q11: the recorded revision identifier is opaque — non-empty,
    non-placeholder, non-duplicate; no length or alphabet assumption; the
    integrity check needs no backend knowledge. Placeholder and duplicate
    rejection is unchanged.
- **Open questions:** none.
- **Handoff status:** ready for `/givn-propose`; may be folded into seed 1.
- **Scope note:** modifies the integrity scenarios in
  `givn/specs/verify-change/givn-workflow-enforcement.feature`.
