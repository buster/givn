# Change seed: archive-change

Status: implemented in `archive-change`

- Use-case ID: `archive-change` (existing, modified)
- Topic: `multi-scm-support`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: after `scm-backend`
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Q3 — Mirrored strictness for the clean-state witness

- Context: Git's `-uall` counts untracked files as dirty; Fossil's default
  counts only what a commit would include.
- Options:
  1. **Mirror Git's strictness** — `EXTRA` files count as dirty.
     In practice: an untracked file blocks the archive on both backends.
     Gain: "an untracked file blocks the archive" stays true everywhere.
     Cost: Fossil projects record or ignore extras before archiving.
  2. **Fossil default** — only changes a commit would include count.
     In practice: an untracked file does not block on Fossil.
     Gain: less friction.
     Cost: the guarantee differs per backend.
- Chosen: **Option 1** because the archive guarantee must not vary by
  backend. Rejected: Option 2.

### Q5 — Who commits

- Context: the archive proof publishes a candidate revision and a
  finalization revision; per-scenario commits are the developer's.
- Options:
  1. **givn publishes the candidate (with the unbound pending receipt) and
     the finalization revision; per-scenario revisions stay with the
     developer/agent.**
     In practice: `givn archive` creates two revisions; the RED/GREEN
     commits remain the developer's.
     Gain: one owner per revision kind.
     Cost: none.
  2. **givn commits everything** — including per-scenario work.
     In practice: the developer's commits disappear.
     Gain: none.
     Cost: the developer loses their revision trail.
  3. **givn verifies only** — the developer publishes the archive.
     In practice: the proof cannot bind to a givn-published candidate.
     Gain: no write access needed.
     Cost: the proof contract weakens.
- Chosen: **Option 1**. Rejected: Options 2 and 3.

### Q13 — Hook-rejection repair parity

- Context: Git keeps a rejected candidate merge staged; Fossil has no index.
- Options:
  1. **Promise "unproven and repairable", not an identical internal
     state.**
     In practice: Git leaves the merge staged; Fossil leaves the merged
     state uncommitted in the checkout.
     Gain: one honest promise across backends.
     Cost: the exact intermediate state is backend-specific.
  2. **Require an identical repair state.**
     In practice: impossible without an index on Fossil.
     Gain: parity.
     Cost: not implementable.
- Chosen: **Option 1** because repairability is the real guarantee.
  Rejected: Option 2.

## Open questions

### Q10 — Vocabulary

Affects wording only; resolved 2026-09-14 with the neutral terms merged into
`docs/arc42/12-glossary.md`.

## Scope note

Added the Fossil write path for the archive proof: clean witness, candidate
publication, pending/proven receipt binding, evidence-only finalization,
refusals, forward repair. Modified
`givn/specs/archive-change/archive-proof.feature`.
