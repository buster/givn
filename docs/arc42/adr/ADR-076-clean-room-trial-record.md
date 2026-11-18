---
status: accepted
date: 2026-11-14
decision-makers: buster (project owner)
consulted: none
informed: none

---

# Bind a clean-room trial to a versioned, per-path record outside the trial

Amended 2026-11-16 by `strengthen-ise-specification`: the commands are renamed,
the record becomes the `ise-manifest/v1` manifest, and its canonical checksum
bytes and additive-field compatibility rule are stated below. The original
decision's ownership and placement stand.

Amended 2026-11-18 by `ise-rebuild-decision-and-working-tree-ship`: the
clean-room trial record is scoped to a **copy-based Ise Test**. A working-tree
Ise Test builds in the project working tree, relies on an instruction not to
consult accessible history, makes no clean-room claim, and produces no trial
record; its evidence is the change verification receipt and the review record.
The command contract, `ise-manifest/v1` scope, and per-path checksums below are
unchanged. **Rejected alternative:** reserving `Ise Test` for the copy-based
path. The working-tree practice is also a rebuild execution with the
Specification and acceptance suite held fixed, so reserving the term would
leave it without an execution noun; the term covers both practices, and the
served instruction states the history-access limitation.

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

## Amendment 2026-11-16: the manifest and the command surface

The original decision fixed a versioned, per-path record. It left the byte form
of its digests to the implementation, and its scope described a trial. The
bounded self-rebuild produced different per-feature checksums for identical
inputs, could not reproduce the field set from the Specification alone, and the
operator settled three command names with the spec copy vocabulary. This
amendment replaces the `ise-trial/v1` record with the `ise-manifest/v1`
manifest and names the commands that write it.

### Amended Decision Outcome

The clean-room surface is `givn ise rebuild-all`, `givn ise rebuild-component
<path>...`, and `givn ise compare`; the earlier names `givn ise replace` and
`givn ise verify` are retired. The rebuild commands write a spec copy and, only
with `--manifest <file>`, one ephemeral manifest with the fields `scope`,
`revision`, `removed_paths`, `specification_files`, `source_kept_files`,
`end_to_end_files`, `counts`, `checksums`, `specification_checksum`,
`end_to_end_checksum`, `copy_directory`, and `givn_version`.

The checksums are computed by the canonical rule in `docs/reference.md` and
§8.29: one path checksum hashes the path, the entry kind (`f`, `x`, `l`, or
`g` for a gitlink whose content is the recorded commit id), the executable
marker, and the content bytes; a rollup hashes the sorted
`path, NUL, checksum, LF` pairs; the per-feature scenario checksum hashes the
feature path and each selected scenario's title and compact executable-content
JSON. `end_to_end_files` is the sorted union of the keys of
`checksums.end_to_end` and `checksums.scenarios`; `end_to_end_checksum` rolls up
that union, with the scenario checksum winning when a path is in both maps. This
scope has no optional field; a field added later is
optional only when the scope states its default, and a field with no derivable
default opens a new scope; an unknown or missing field and a mismatched scope
are refused naming the manifest and the expected value. The `ise-trial/v1` shape is refused
by name and is not migrated.

Rejected: keeping `replace` and `verify` (two mechanisms named as commands);
reading the retired record shape (two field sets and an ambiguous count
mapping); printing the manifest (the report has no path to name).

### Amendment Consequences

- Good, because the commands name the action: one rebuilds the whole product,
  one rebuilds a component, one compares.
- Good, because a second implementation can produce byte-identical checksums
  from the Specification alone.
- Bad, because a script or habit that calls the retired names fails with exit
  `2`, and a manifest written before this change must be re-prepared.

### Amendment Confirmation

`Developer rebuilds the whole product from its Specification` and `Developer
rebuilds one component and leaves its neighbours` drive the two rebuild
commands through the binary; `Preparation records the canonical checksums for
a pinned fixture` asserts every checksum, `Preparation records the canonical
checksums for a richer feature` pins the content-encoder branches, and
`Verifier compares the spec copy with the manifest` drives the comparison. The design computed the pinned values
twice, once with the production binary and once with an independent
implementation of the stated rule.

## More Information

- `givn/changes/add-ise-clean-room-commands/specs/prepare-an-ise-test/usecase.md`
  owns the observable behavior; archive merges it into the permanent corpus.
- `strengthen-ise-specification` amended this record with the command names,
  the `ise-manifest/v1` scope, and the canonical checksum form. The permanent
  sources are `givn/specs/prepare-an-ise-test/usecase.md`, `docs/reference.md`,
  and §8.29 of `docs/arc42/08-crosscutting-concepts.md`.
- `givn/changes/add-ise-clean-room-commands/design.md` owns the identity rule,
  the record fields, and the refusals.
- ADR-010 owns the declaration authority; ADR-070 owns the change verification
  receipt (`change-proof/v1`); ADR-071 owns the SCM seam. ADR-078 owns what the
  spec copy contains: the frozen `e2e` package is never copied, and the
  `ise-trial/v1` record's optional `excluded_roles` field named the omitted role
  before this record's 2026-11-16 amendment replaced that shape with
  `ise-manifest/v1`, which has no `excluded_roles` field.
- Arc42 chapters [2](../02-architecture-constraints.md),
  [6](../06-runtime-view.md), and [8](../08-crosscutting-concepts.md) carry the
  current facts.

## Amendment 2026-11-18: the verification contract and the trial gate

The 2026-11-16 amendment fixed three commands and one `ise-manifest/v1` field
set. It left the record able to prove frozen-input equality but unable to prove
that the kept component corpus executed: a trial could pass with the frozen
`@e2e` suite green and zero component scenarios run.

This amendment adds one generated `givn/ise-verification.yaml` contract and one
`givn ise verify-trial` command. The rebuild writes the contract into the spec
copy and records its path and checksum in the manifest's optional
`verification: {path, checksum}` field. The contract names the component and
frozen End-to-end commands and feature paths, each tier's declared
execution-unit count and scenario titles, the rebuilt `product.path` and
`product.env`, and the coverage command and paths when the coverage addon is
effective. An execution unit is one plain scenario or one Scenario Outline
example row, matching the runner's expanded count.

`givn ise compare` remains read-only. It verifies a recorded contract's
checksum, excludes the reserved path from its added-Specification-file refusal,
and prints `Trial result: unverified`; its exit 0 proves frozen-input equality
only. `givn ise verify-trial` is the only command that executes the contract:
it requires fresh results that name every declared scenario title, exact
declared counts, zero failed and zero skipped, the rebuilt product bound by
path and SHA-256, and declared coverage, re-checks the frozen inputs after the
scripts, then writes `ise-trial-result/v1`. A count-only result or a foreign
product refuses. A `--component-not-run` decision is refused when component
units are declared. Givn does not inspect a `givn/trials/**` or other external
report; external and manual reports are outside Givn enforcement. Only
`verify-trial` writes the clean `ise-trial-result/v1`.

The optional `verification` field's default is absence, so an older manifest
still compares; `verify-trial` refuses an older manifest by name because it
cannot prove the contract. The field belongs to `ise-manifest/v1` rather than a
new scope because absence is a derivable default and the frozen-input fields
are unchanged. **Rejected alternative:** executing the contract inside
`givn ise compare`. A comparison of an untrusted copy would run copied scripts,
and exit 0 would conflate input equality with trial success.

### Amendment Confirmation

`Preparation writes a runnable verification contract`, `Comparison labels
frozen-input equality as unverified`, `Verification refuses a missing component
runner`, `Verification records the exact declared counts`, `Verification
refuses an exact count mismatch`, and `A not-run decision cannot replace
declared component units` drive the real commands through the binary.
