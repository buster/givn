---
status: accepted
date: 2026-11-16
decision-makers: buster (project owner)
consulted: none
informed: none

---

# Keep the frozen End-to-end (E2E) test package outside the clean-room trial

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

Evidence: the `ise-rebuildability` topic settled that the frozen End-to-end (E2E)
test package runs outside the builder's authority during an Ise Test, and a
`passuff` dry run showed a trial that copied it, a builder that read it, and a
frozen suite that passed 8 of 8 while five of its assertions stated strings the
Specification never states. ADR-076 owns the trial record, ADR-010 the
declaration authority, ADR-070 the change verification receipt, and ADR-071 the
SCM seam; none owns what the trial directory contains, so the route is a new
record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: every open Ise Test would have to be re-run, and the record rule
      that states the exclusion is the compatibility boundary for records
      already on disk.
- [x] **Shapes quality attributes** - It materially affects auditability and
      trust because: a frozen suite that the builder could read attests the
      builder's reading, not the Specification, and no `Spec gap` review can
      restore the difference after the fact.
- [x] **Structural** - It defines a boundary and a dependency direction
      because: the trial's content is fixed by the declaration's `e2e` role, the
      verifier reads the package from the checkout and never from the trial, and
      the trial cannot hold a path the frozen package covers.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: the obvious extension of the retained
      roles is to copy the sixth one too, and a trial that carries the suite
      looks complete while it defeats the acceptance claim.

## Context and Problem Statement

`givn ise replace` copies every declared retained role into the trial, and the
`e2e` role is one of the five, so the trial that a builder reads holds the frozen
acceptance suite: its tests, its mocks, and the exact strings they assert. The
record already separates the package's identity from the retained set, but the
copy set does not follow that separation. What the trial contains is therefore
the one part of the frozen boundary no record owns.

## Decision Drivers

- A builder reconstructs from the Specification, never from its acceptance
  evidence.
- The frozen identity stays verifiable after the package leaves the trial.
- A record written before this rule stays readable and re-checkable.
- The trial stays exactly one revision's files minus declared sets, so the
  write-time comparison needs no exception.

## Considered Options

- Exclude the resolved `e2e` package from the trial, record the excluded role,
  and refuse a trial that holds a frozen path.
- Copy the package as today and rely on the `Spec gap` review to catch a
  builder that read it.
- Replace the package with a note or a path listing so the builder knows what it
  cannot see.

### Cheaper home rejected

The rule could live only in `prepare-an-ise-test` rules plus chapter 02; that is
where the observable behaviour and the constraint are stated. Neither artifact
owns the rejected alternatives, the compatibility consequence of a record field,
or the reason a later reader must not read a smaller `Retained` as a shrinking
declaration. The contract therefore carries the rules and links this record, and
the ADR owns the rationale.

## Decision Outcome

Chosen option: "Exclude the resolved `e2e` package from the trial, record the
excluded role, and refuse a trial that holds a frozen path", because the frozen
suite is only acceptance evidence when the builder cannot read it, and an
exclusion that the record does not state leaves the reader unable to tell a
deliberate omission from a shrinking declaration.

The refusal is bounded by the record it reads: `givn/specification.yaml` and
every path the record's `retained_files` lists are exempt, because an `e2e` role
may resolve over `givn/**` and every trial retains the declaration. The report's
`Frozen package` line counts the frozen set, not the recorded `e2e_files`, so the
number states exactly what the trial omits even when the resolved package
includes the declaration.

### Consequences

- Good, because `E2E: clean` now describes a suite the builder never saw.
- Good, because the record's `excluded_roles` keeps `Retained` unambiguous for
  records written under either rule, and `ise-trial/v1` stays the scope.
- Good, because a trial assembled by any other path (a copied directory, a
  retained archive, a hand-edited record) is refused by name.
- Bad, because a builder can no longer run the acceptance suite during
  reconstruction, so a missing mock or a structural mismatch surfaces at the
  verifier instead of during the build (R-154).
- Bad, because a record whose `excluded_roles` was removed keeps the old
  behaviour, so the refusal is a record property rather than an unconditional
  trial property.

### Confirmation

The delta scenarios in
`givn/changes/keep-frozen-verifier-out-of-the-trial/specs/prepare-an-ise-test/`
drive the real `givn ise replace` and `givn ise verify` commands under the E2E
scope (`./run-tests.sh --e2e`, `GIVN_RESULT_SCOPE=e2e`): the preparation
scenario asserts `Retained: 2` and that neither package file is in the trial, and
the verification scenarios assert the refusal, its twenty-path bound, a clean
re-check of a trial prepared before the rule, and a clean re-check of a trial
whose `e2e` role covers the declaration.

## Pros and Cons of the Options

### Exclude the package, record the role, refuse a trial that holds one

- Good, because it makes the frozen boundary a property of the trial directory
  rather than of one command.
- Good, because the exclusion and the recorded identity come from the same
  resolver, so they cannot disagree.
- Neutral, because the record gains one field and the report one line.
- Bad, because the builder loses its local acceptance signal.

### Copy the package and rely on the `Spec gap` review

- Good, because a builder can run the suite while it reconstructs.
- Bad, because it gives the builder read authority over its own acceptance
  evidence; the live dry run passed 8 of 8 while five assertions stated strings
  the Specification never states.
- Bad, because no review can separate a reconstruction that followed the
  Specification from one that followed the tests.

### Replace the package with a note or a path listing

- Good, because the builder learns the exclusion without reading anything else.
- Bad, because the trial then holds a file that is not at the revision, so the
  copy set, the write-time comparison, and the record each need an exception.
- Bad, because a listing exposes the test file names, which describe the covered
  areas the Specification already covers.

## More Information

- `givn/changes/keep-frozen-verifier-out-of-the-trial/specs/prepare-an-ise-test/usecase.md`
  owns the observable behaviour; archive merges it into the permanent corpus.
- `givn/changes/keep-frozen-verifier-out-of-the-trial/design.md` owns the frozen
  set, the four partition filters, the record field, and the refusal.
- ADR-076 owns the trial record; this record adds its optional `excluded_roles`
  field without renaming anything, and ADR-076 keeps the record's shape,
  placement, and scope rule.
- Arc42 chapters [2](../02-architecture-constraints.md),
  [8](../08-crosscutting-concepts.md), and
  [11](../11-risks-and-technical-debt.md) carry the current facts.
