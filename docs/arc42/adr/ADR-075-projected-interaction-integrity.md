---
status: accepted
date: 2026-11-05
decision-makers: buster (project owner)
consulted: none
informed: none

---

# Enforce the projected interaction inventory before accepting a change

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

The projection algorithm, in-memory corpus mechanics, shared merge-text entry
point, and exact finding text are owned by `design.md`, `spec::corpus`,
`spec::merge_plan`, `spec::projection`, and the delta features. The durable
boundary between acceptance and publication is not owned by a lower-level
artifact, so the candidate is not routed to one. The active register and `docs/arc42/adr/archive/` were searched for
interaction, mandatory-check, and archive-publication records. The archived
semantic-engine records are disjoint from this boundary; ADR-019 is a
historical lint-contract record that delegates the current contract to
`givn/specs/verify-change/`, so it does not own the projection boundary.
ADR-011 owns the feature-is-spec contract, ADR-028 the mandatory checks, and
ADR-070/ADR-072 the archive proof and publication boundaries. None owns a
projected pre-acceptance consistency gate.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would remove a component boundary
      from review and archive and break the executable scenarios that pin the
      pre-mutation refusal and the merged-candidate recheck.
- [x] **Shapes quality attributes** - It materially affects specification
      integrity and reliable evidence because a proven archive can no longer
      leave an E2E interaction declared but unproven.
- [x] **Structural** - It adds `spec::projection`, moves merge planning from
      `cli::archive` to `spec::merge_plan`, and makes the dependency direction
      point inward from both gates.
- [x] **Must be shared** - Developers unfamiliar with the boundary would
      otherwise re-enable the same false green by linting only the permanent
      corpus or only after the merge.

## Context and Problem Statement

The permanent specification corpus is the executable record, but review could
not see an interaction mismatch that a change would introduce. Interaction
coverage read only the permanent corpus, while change-scoped lint kept only
findings attributed to the change's delta; archive ran its static gates before
the merge and never checked the merged inventory. A change could therefore add
an `@e2e` scenario without an interaction row, archive a green proof, and leave
the corpus contradicting its own E2E inventory.

## Decision Drivers

- Gate correctness: a documented blocking finding must actually block at the
  review and archive boundaries.
- Pre-mutation safety: a change must fail while it is still active whenever the
  failure is detectable before the archive move.
- One merge implementation: the projected corpus and the published corpus must
  not drift.
- Recovery: a failure after the candidate publication must leave the pending
  candidate repairable in place.
- No semantic model or external service: the gate stays deterministic and local.

## Considered Options

- Project the merged corpus before mutation, in review, change-scoped lint, and
  the archive static gates.
- Check only after the merge, before candidate verification.
- Run the check only in CI after the change lands.

## Decision Outcome

Chosen option: "Project the merged corpus before mutation", because it gives the
developer the failure while the change is still active and preserves the
archive's block-before-mutation rule.

### Consequences

- Good, because review, `givn lint --change`, and archive share one projected
  inventory predicate and one merge-plan implementation.
- Good, because a fresh archive refuses before the pending receipt or the
  rename, and a merged candidate is rechecked on the actual corpus before its
  verification scopes.
- Bad, because a project with pre-existing interaction-inventory drift is
  blocked at its next review or archive until it repairs the inventory; the
  projection deliberately does not subtract pre-existing findings.
- Bad, because review and archive read and parse one in-memory copy of the
  specification corpus, and a new component and module move are introduced.

### Confirmation

The delta scenarios `Change-scoped lint reports an E2E scenario the projected
inventory does not declare`, `Review refuses a projected interaction-inventory
mismatch before archive`, `Archive refuses a projected interaction-inventory
mismatch before moving the change`, and `Archive refuses a merged candidate
whose interaction inventory is inconsistent` drive the real CLI. The existing
archive proof suite and `givn lint` on the permanent corpus remain green.

## Pros and Cons of the Options

### Project before mutation

- Good, because the developer sees the failure before the archive move and no
  red candidate is created.
- Good, because review and archive share one projection and one merge plan.
- Bad, because it adds `spec::projection`, moves merge planning out of
  `cli::archive`, and reads the specification corpus into memory per review and
  archive.

### Check only after the merge

- Good, because it validates the actual merged bytes with no projection.
- Bad, because the change has already moved and the developer repairs an archive
  candidate instead of an active change.
- Bad, because review remains blind and the failure is late.

### CI only

- Good, because it adds no gate or recovery path.
- Bad, because a red corpus can still be archived and a project without CI gets
  no protection.
- Bad, because it does not satisfy the documented hard-gate contract.

## More Information

The change record is `givn/archive/` after proof; before archive it lives under
`givn/changes/enforce-spec-interaction-integrity/`. The design owns the
in-memory corpus and shared merge-text algorithm and the step locations; the
scenarios own the refusal and the merged-candidate recheck. Related decisions: ADR-011 (feature is the
specification), ADR-028 (mandatory checks), ADR-070 (forward-only archive
proof), and ADR-072 (witness-bounded publication).
