---
status: accepted
date: 2026-09-10
decision-makers:
  - givn maintainers
consulted:
  - archive-proof proposal, executable specs, and design
informed:
  - consumer projects that publish completed changes with `givn archive`
  - reviewers relying on review evidence and verification receipts
---

# Publish archives forward-only and bind proof to a committed candidate

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

### MADR checklist evidence

- [x] **Hard to reverse** - the permanent corpus becomes append-only and its
      provenance moves into a receipt file; reverting to transactional
      rollback later would strand existing pending/proven archives and force
      a migration of the publication boundary.
- [x] **Shapes quality attributes** - fail-closed verification and a durable
      receipt raise recoverability and auditability of the publication
      boundary without weakening the green-corpus guarantee.
- [x] **Structural** - it defines who owns verification (the archive, once,
      against a committed candidate) and changes the permanent-corpus
      writer boundary from "mutate then restore" to "commit then prove".
- [x] **Must be shared** - without this record an implementer would
      re-derive rollback semantics or treat the pending receipt as
      optional; supporting only, never a gate.

- **alternatives** — three materially different options recorded below
  (transactional rollback with backups, a dual-mode rollback/forward flag,
  forward-only candidate plus verification receipt).
- **architectural_impact** — a Git-backed publication boundary, a new Git
  wrapper and proof module, a receipt data model, and a changed archive
  runtime contract; permanent specs gain a single responsible writer.
- **durable_consequence** — `givn/archive/<id>/verification.json` becomes
  durable project data; consumers and tooling may read it, and changing the
  semantics later requires migration rather than a code edit.
- **lower_level_artifact** — the candidate/proven state machine and the
  fail-closed result contract are executable behaviour owned by the
  `archive-proof` capability specs and the design; the ADR records only the
  irreversible boundary (forward-only publication) and the ownership of
  verification.
- **existing_adr_check** — searched the active and archived register:
  ADR-014 (filesystem-derived state), ADR-015 (declared runner), and
  ADR-028 (hooks versus mandatory checks) constrain adjacent boundaries but
  no record owns archive transactionality or proof. Route: `NEW_ADR`.

## Context and Problem Statement

Archiving mutated the permanent specification corpus and then executed the
full suite, E2E scope, and coverage against the mutation. On failure the
archive restored the pre-archive corpus and deleted the exact state that
produced the failure, so the developer had to recreate the merge and
rediscover the failure. Publication left no durable evidence of which commit
was tested, and the suite ran at review as well as at archive. The problem is
to make publication auditable and repairable without letting an unverified
tree masquerade as complete.

## Decision Drivers

- The permanent corpus must never be reported as green without executed,
  reconciling verification evidence.
- A failed publication must be recoverable in place; the tested state must
  survive the failure.
- One stable command must own publication; verification must not be
  duplicated across review and archive.
- The state must survive process restarts and be observable by developers and
  automation without a database (ADR-014).

## Considered Options

- Transactional rollback: keep the backup/restore pipeline and add receipts.
- Dual-mode publication: a flag selects rollback or forward-only behavior.
- Forward-only candidate: commit the merged tree, verify it, and finalize
  with a receipt.

## Decision Outcome

Chosen option: "Forward-only candidate", because keeping the tested tree
committed makes the failure state itself the repair input, and a receipt
bound to that commit makes the green-corpus claim falsifiable.

### Consequences

- Good, because a failing archive leaves one committed candidate that the
  developer repairs with ordinary commits and reruns; no work is recreated or
  silently discarded.
- Good, because the receipt binds a proven archive to a tested commit, so
  `git show` and the receipt together answer what was verified.
- Good, because review no longer executes the suite; the archive is the single
  full-suite authority.
- Bad, because review-time coverage classification loses its fresh
  instrumented run until an archive receipt exists (chapter 11, R-54).
- Bad, because archive now requires a Git worktree and writes commits, adding
  a hard prerequisite and hook interaction to the publication path
  (chapter 11, R-55).
- Neutral, because legacy archives without a receipt keep the existing
  already-archived behavior and are never re-published.

### Confirmation

The executable `archive-proof` scenarios confirm the contract: a dirty
worktree blocks and names files; a suite failure leaves a committed candidate
with a pending receipt; a repair rerun proves the candidate without repeating
the merge; a missing, failed, skipped, zero, malformed, or non-reconciling
scope result blocks proof; finalization refuses a tested-tree change. The
archive gate verifies the receipt status and the finalization guards; the
change review independently checks the step-definition table, fabrication
audit, and arc42 conformance.

## Pros and Cons of the Options

### Transactional rollback with receipts

- Good, because it preserves the existing failure semantics and fixtures.
- Bad, because rollback destroys the failing state, forcing re-creation of the
  exact merge and failure before repair.
- Bad, because the backup pipeline adds a third state (restored) that no
  consumer can observe and no receipt can describe.

### Dual-mode rollback/forward flag

- Good, because it allows an incremental migration.
- Bad, because two publication semantics must be specified, tested, and
  documented forever; the flag becomes a compatibility surface for a
  transient concern.
- Bad, because the corpus can be green under two different procedural
  meanings, which weakens the audit claim.

### Forward-only candidate plus receipt

- Good, because publication is a linear, resumable state machine: candidate,
  verification, receipt, finalization.
- Good, because Git already provides history and recovery, so Givn does not
  reimplement rollback.
- Bad, because Git becomes a hard prerequisite and archives create commits.
- Bad, because unproven candidates are visible project state and must be
  surfaced by status and instructions.

## More Information

- Executable specs: `givn/changes/archive-proof/specs/archive-change/`
  (`archive-proof`, `extract-coverage-gate`, `enforce-post-merge-e2e-gate`)
  and `specs/verify-change/givn-workflow-enforcement.feature`.
- Design: `givn/changes/archive-proof/design.md`.
- Related decisions: ADR-014 (filesystem-derived state), ADR-015 (declared
  runner), ADR-028 (hooks versus mandatory checks), ADR-069 (feature-free
  binary; the receipt and result formats use the existing JSON dependency).
- Risk records: chapter 11 R-5, R-53, R-54, R-55.
