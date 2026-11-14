---
status: accepted
date: 2026-11-14
decision-makers: buster (project owner)
consulted: none
informed: none

---

# Bind a clean-room trial to a versioned, per-path record outside the trial

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

Evidence: the `ise-rebuildability` topic settled the two clean-room commands
(`Q3`) and the frozen `e2e` package (`Q5`); the `prepare-an-ise-test` use case
specifies the refusals and the per-path re-check but not the record's durable
scope, its placement, or its identity shape. The design's decisions `D2` and
`D4`, and the local decision `D5`, choose those. ADR-010 owns the declaration
authority, ADR-070 owns the change verification receipt, and ADR-071 owns the
SCM seam; none owns a trial record, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: every prepared trial, the harness that reads it, and the
      verifier's re-check depend on the `ise-trial/v1` shape and its per-path
      identities.
- [x] **Shapes quality attributes** - It materially affects auditability and
      trust because: a verifier can name the exact file that changed instead of
      trusting a set-level digest or a report summary.
- [x] **Structural** - It defines a boundary, contract, and dependency
      direction because: `givn ise replace` owns the writer, `givn ise verify`
      and the external verifier read the record, and the record cannot live in
      the builder's write authority.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: a record placed inside the trial, or a
      single rollup with no per-path evidence, would silently defeat the
      re-check.

## Context and Problem Statement

An `Ise Test` hands a builder a trial directory that contains the declared
`Specification` and kept source, with the named component paths removed. A
verifier must confirm which revision was read, which paths were removed, and
which retained and `e2e` files the builder received, before running the frozen
suite. Without a record, the handover is an untested script and a verifier has
no evidence that the trial was not edited. The change adds the two commands that
produce and re-check that trial; the record is the durable boundary between
them.

## Decision Drivers

- A verifier can name the exact drifting file, not only that a set changed.
- The record is outside the builder's write authority and the checkout is never
  written.
- Git and Fossil produce one record shape.
- A later format change is a new scope, not a silent reinterpretation.
- The frozen `e2e` package, not the record, remains the acceptance authority.

## Considered Options

- A versioned record (`ise-trial/v1`) with per-path digests, a rollup per
  identity, and per-feature scenario digests, written outside the trial.
- One rollup digest per identity, printed to stdout only.
- A self-contained record inside the trial, including the frozen file contents.

### Cheaper home rejected

The current record facts could live in
`docs/arc42/08-crosscutting-concepts.md` §8.29 plus the data model in the
change's `design.md`; that is where they are stated. Neither artifact can own
the rejected alternatives, the compatibility consequence, and the reason a
later format change needs a new scope without duplicating decision records in
chapter 08. §8.29 therefore carries the current facts and links this ADR, and
the ADR owns the rationale.

## Decision Outcome

Chosen option: "A versioned record (`ise-trial/v1`) with per-path digests, a
rollup per identity, and per-feature scenario digests, written outside the
trial", because the per-path evidence is what makes the verifier's guarantee
executable: a changed retained or `e2e` file is named, and the record cannot be
edited with the trial.

### Consequences

- Good, because `givn ise verify` names every drifting path against the trial
  and the current checkout.
- Good, because the record's `content_scope` makes a later format change a new
  scope, and an old record is refused by name rather than reinterpreted.
- Good, because the record carries the revision, the named and removed paths,
  the counts, the three identities, the backend, and the running Givn version
  in one auditable file.
- Bad, because the record is larger than one digest per identity.
- Bad, because the identity covers the recorded set; a file the builder adds
  after preparation is not drift (R-143).
- Bad, because the target containment check is not a lock and a concurrent
  local writer can race the check (R-144).
- Bad, because a caller that loses the record cannot re-check the trial; the
  record is the evidence.

### Confirmation

The change's delta `@e2e` scenarios in
`givn/changes/add-ise-clean-room-commands/specs/prepare-an-ise-test/` drive the
real `givn ise replace` and `givn ise verify` commands under the E2E scope
(`./run-tests.sh --e2e`, `GIVN_RESULT_SCOPE=e2e`); the drift scenarios name a
changed retained file, a changed `e2e` file in the trial and in the checkout, a
changed `@e2e` scenario, and a missing path.

## Pros and Cons of the Options

### A versioned per-path record outside the trial

- Good, because it names drift per path.
- Good, because the record and the trial have separate authorities.
- Neutral, because it stores one digest per file instead of one per set.
- Bad, because the record must be kept with the trial for a later re-check.

### One rollup digest per identity, printed to stdout only

- Good, because it is the smallest possible evidence.
- Bad, because the verifier cannot name a path.
- Bad, because an omitted receipt leaves no durable evidence.

### A self-contained record inside the trial with the frozen contents

- Good, because one directory holds the trial and all evidence.
- Bad, because the builder can edit the record with the trial.
- Bad, because the record duplicates the `Specification` and becomes a second
  corpus.

## More Information

- `givn/changes/add-ise-clean-room-commands/specs/prepare-an-ise-test/usecase.md`
  owns the observable behavior; archive merges it into the permanent corpus.
- `givn/changes/add-ise-clean-room-commands/design.md` owns the identity rule,
  the record fields, and the refusals.
- ADR-010 owns the declaration authority; ADR-070 owns the change verification
  receipt (`change-proof/v1`); ADR-071 owns the SCM seam.
- Arc42 chapters [2](../02-architecture-constraints.md),
  [6](../06-runtime-view.md), and [8](../08-crosscutting-concepts.md) carry the
  current facts.
