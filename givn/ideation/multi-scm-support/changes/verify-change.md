# Change seed: verify-change

Status: implemented in `verify-change`

- Use-case ID: `verify-change` (existing, modified)
- Topic: `multi-scm-support`
- Personas: `agent--technical-role`, `givn-maintainer--owner`
- Suggested start: after `scm-backend`; could be folded into it
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Q11 — Opaque revision identifiers

- Context: Git hashes are 40 hex characters; Fossil shows 40 hex and stores
  64. Length or alphabet assumptions would reject valid Fossil revisions.
- Options:
  1. **Treat the identifier as opaque** — non-empty, non-placeholder,
     non-duplicate; no length or alphabet assumption; the integrity check
     needs no backend knowledge.
     In practice: a Fossil check-in identifier passes the same integrity
     gate as a Git commit hash.
     Gain: backend-independent verification.
     Cost: weaker format validation.
  2. **Validate the hash shape per backend** — 40/64 hex.
     In practice: the check learns backend-specific formats.
     Gain: stricter validation.
     Cost: backend knowledge in a neutral check.
- Chosen: **Option 1**. Rejected: Option 2.

## Open questions

None.

## Scope note

Modified the integrity scenarios in
`givn/specs/verify-change/givn-workflow-enforcement.feature`.
