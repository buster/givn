---

status: accepted
date: 2026-09-14
decision-makers: buster (project owner)
consulted: none
informed: none

# Publish revisions through the SCM seam with witness-bounded commits

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

Evidence: the `archive-change` design weighed naming paths in `fossil commit`
(impossible for a removed directory: Fossil fails with `no such file` for a
named directory containing missing files) against recording all changes and
committing the witnessed dirty set; the archive-proof Gherkin specifies the
observable candidate publication, its exact contents, and the receipt binding
but not the per-backend publication mechanism; ADR-071 owns selection and
ADR-070 owns forward-only proof, neither owns the per-backend publication
mechanism, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: every publishing command (archive today, the later Fossil
      upgrade and commit paths) consumes `Backend::publish`; changing the
      contract means touching each consumer and its evidence.
- [x] **Shapes quality attributes** - It materially affects correctness and
      maintainability because: candidate exactness is an invariant of the
      archive proof, and one publication contract replaces per-backend
      ad-hoc staging.
- [x] **Structural** - It defines a boundary, contract, and dependency
      direction because: commands depend on the seam's `publish` operation
      and never stage or record changes themselves.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: adding a command that records paths
      without the witness would publish unrelated changes on Fossil.

## Context and Problem Statement

The archive proof publishes a candidate revision and an evidence-only
finalization revision. On Git that is "stage exactly these paths, commit the
index". Fossil has no index: `fossil commit <paths>` cannot name a directory
that contains removed files, and new files must be recorded before they can be
committed. The publication contract therefore cannot be a literal copy of the
Git mechanism, yet the candidate must still contain exactly the merged corpus
paths on both backends.

## Decision Drivers

- Candidate exactness: the published revision contains exactly the resolved
  paths and nothing else.
- One contract for every publishing command, not per-backend staging logic in
  each command.
- Repairability: a rejected publication stays repairable in place.
- No new dependency or server-side state.

## Considered Options

- Keep the archive Git-only and refuse Fossil projects (status quo).
- Name paths in `fossil commit` and rely on Fossil's path expansion.
- Resolve the publication paths, validate the exact dirty set against them,
  and record only that set: Git stages the paths, Fossil records all changes
  and commits the witnessed dirty set.

## Decision Outcome

Chosen option: "Resolve the publication paths, validate the exact dirty set
against them, and record only that set", because the clean-state witness
already bounds what may be dirty, the validation makes the bound explicit, and
Fossil's record-all mechanism then commits exactly the resolved paths.

### Consequences

- Good, because the candidate-exactness invariant is enforced in one place
  and holds on both backends without Fossil-specific staging commands.
- Good, because a rejected publication is detected on resume by checking
  whether the current revision contains the pending receipt, so the archive
  stays repairable without a rollback.
- Bad, because the Fossil path commits the whole witnessed dirty set; a
  future caller that publishes without the witness would include unrelated
  changes (recorded as R-65).
- Bad, because the Fossil path needs an extra `fossil addremove` pass before
  each publication, so publication is two process invocations instead of one.

### Confirmation

The archive-proof scenarios assert candidate publication, finalization
contents, and receipt binding on both backends; the Fossil archive E2E
scenario drives a real `fossil` checkout through the compiled CLI.

## Pros and Cons of the Options

### Keep the archive Git-only

- Good, because no new mechanism is introduced.
- Bad, because the `multi-scm-support` topic commits givn to archiving on
  Fossil, so the status quo contradicts the goal.

### Name paths in `fossil commit`

- Good, because it mirrors the Git pathspec model.
- Bad, because Fossil fails on a named directory that contains removed files,
  so the moved change directory cannot be published this way.
- Bad, because deletions still need an explicit `fossil rm`/`addremove` pass,
  leaving two mechanisms with different failure modes.

### Witness-bounded record-all

- Good, because one validation point enforces candidate exactness.
- Good, because it works for additions, edits, and deletions uniformly.
- Bad, because it relies on the witness; the explicit dirty-set validation is
  the guard that keeps the reliance safe.

## More Information

- Ideation topic: `givn/ideation/multi-scm-support/` (use case
  `archive-change`, Q3, Q5, Q13).
- Design: `givn/changes/archive-change/design.md` (Fossil checkout handle,
  resume republication).
- Related records: ADR-070 (forward-only archive proof), ADR-071 (SCM backend
  selection).
