# Proposal: enable-ise-and-run-the-givn-self-rebuild

## Release note

Givn enables `ise` on itself and completes a whole-product `Dry-Run Ise`, with
the frozen End-to-end (E2E) package green or every difference recorded as one
`Spec gap` outcome.

## Behavior Change

Behavior change: none

The change enables a declared addon and records one trial's evidence; it adds
no product behavior.

## Use-Case Context

- Use-case IDs: `prepare-an-ise-test` (`clean-room-preparation`,
  `trial-input-verification`), `declare-the-specification`
  (`specification-declaration`), and `create-change` (`change-strategy`,
  `spec-gap`)
- Ideation topic: `ise-rebuildability`
- Seed path:
  `givn/ideation/ise-rebuildability/changes/enable-ise-on-givn-and-run-a-dry-run.md`
- Confirmed Personas: `agent--technical-role`, `developer--primary-user`,
  `givn-maintainer--owner`

## Review Context

- Review kind: independent
- Reason: the operator requested an independent review of the trial evidence,
  and `S14` reads it. This reviewer context did not write the design.
- Reviewer context: paseo design-review context, 2026-11-17 (deepseek-flash, message 8d6e24ce); this one context also owns the final review

## Problem / Opportunity

Givn enforces the declared E2E package boundary and provides the clean-room
rebuild commands, but it has never applied them to itself. The boundary change
validated the gate on a fixture project; the clean-room commands were proven on
fixture projects and on other products. The `ise` addon is opt-in, and `S14`
requires the Givn self-rebuild evidence before it can decide promotion.

Without this change the maintainer cannot tell whether Givn's declared
Specification is sufficient to rebuild Givn, whether the frozen E2E package
survives a replacement of the whole Source code, or which behavior the
Specification leaves unstated.

## Proposed Solution

Observable behavior, from the maintainer's side:

- **Effective declaration.** The project's effective add-on set records
  `ise: true`; the change lock records it. `givn check tasks` on the change
  runs the declared E2E package boundary preflight and reports
  changed-scenario verification as not applicable; a change that declares
  `Behavior change: none` produces no change verification receipt.
- **Whole-product spec copy.** A `Dry-Run Ise` writes one spec copy from a
  recorded revision. The copy carries the declared `behavior`, `structure`,
  `contracts`, and `limits` paths and the declaration. It excludes every
  End-to-end (E2E) file and removes every tracked file under `src/**` and
  `tests/**`.
- **Clean-room rebuild.** A fresh agent builds the binary in the spec copy from
  the Specification plus the non-Specification build inputs the design
  discloses (`Cargo.toml`, `Cargo.lock`, `assets/**`, the Rust toolchain, and
  the cargo registry cache). The agent reads neither the replaced Source code
  nor the checkout nor repository history. The build command and the rebuilt
  binary's version are recorded.
- **Frozen acceptance.** The frozen `e2e` package runs against the rebuilt
  binary. The result is green, or every difference is recorded with exactly one
  `Spec gap` outcome (`E2E test defect`, `implementation defect`,
  `specification defect`, or `acceptable undocumented change`) and the artifact
  that is wrong.
- **Recorded evidence.** The change records the package independence, the
  boundary preflight transcripts, the scenario inventory, the rebuild report
  and manifest, the rebuilt binary's version and E2E result, and the gap list.
- **No promotion.** The rebuilt binary and the spec copy stay outside the
  checkout. `ise` stays opt-in, and `S14` owns the promotion decision.

## Capability Routing

Routing reports its top candidate: `EXTEND givn-workflow-enforcement` (17.00,
signal). No capability behavior changes. The enablement is project
configuration, and the trial produces evidence for capabilities that
`prepare-an-ise-test`, `declare-the-specification`, and `create-change` already
own. No scenario is added, modified, or retired; `specs` is skipped, and the
proposal declares `Behavior change: none` so the skip and the declaration
agree. The change's own `givn check tasks` boundary preflight and the trial
evidence are the deliverable.

## Out of Scope

- Promoting `ise` to a default-on addon. `S14` decides that from the evidence.
- Product corrections for a difference classified as `specification defect`.
  The trial records the outcome and the wrong artifact; the correction belongs
  to a later change.
- Changes to the boundary gate, the clean-room command surface, the manifest
  shape, or the E2E runner.
- Moving or retagging `@e2e` scenarios. `enforce-ise-e2e-boundary` archived the
  inventory; this change re-checks it against the frozen package.
- Catopia and Passuff campaign evidence (`S7` to `S12`).
- State migration. Givn keeps no persisted product state in the trial.

## Open Questions

### D1: Which revision the dry run reads

- Question: The rebuild command reads one recorded revision and refuses a dirty
  checkout. Which revision does the trial read when the change's own task and
  evidence records sit in later commits, and how is a restart after an E2E test
  defect handled?
- Options:
  1. **The change's last implementation commit, passed explicitly**: the
     trial runs after the change's final task and evidence commit, reads that
     commit with `--at`, and records the 40-character revision; a restart after
     a corrected E2E test reads the commit that records the correction; gain:
     reproducible inputs, and the report and manifest name one revision; cost:
     one extra commit before the trial, and a restart needs one more commit; in
     practice: the report prints `Revision: <40 hex>` and the manifest repeats
     it.
  2. **`HEAD` at trial time**: read the branch tip without `--at`; gain: no
     revision bookkeeping; cost: a concurrent commit or branch move changes the
     input silently, and the transcript's revision cannot be reproduced; in
     practice: two trials the same day read different revisions without a
     transcript difference.
  3. **The change's base revision**: read the revision the change started
     from; gain: the Source code is exactly the pre-change product; cost: the
     trial ignores every trial input committed with the change, including a
     corrected E2E test and the evidence paths, so the report names a revision
     the evidence does not match; in practice: the E2E checksum in the manifest
     predates the corrected test.
- Disposition: carried. The design's Decisions ledger decides it, and the
  evidence records the exact revision.

### D2: How a difference is handled before it is recorded

- Question: A difference appears between the frozen E2E result and the rebuilt
  binary. Does the trial classify it first, fix it first, or stop?
- Options:
  1. **Classify first, then route**: record the observed result, the expected
     result, exactly one `Spec gap` outcome, and the wrong artifact before any
     edit; an `implementation defect` returns to the builder inside the same
     trial, an `E2E test defect` is corrected and the trial restarts at the
     correcting revision, and a `specification defect` or `acceptable
     undocumented change` is recorded without a corpus edit in this change;
     gain: the outcome is evidence, not a description of the fix; cost: the
     trial may run twice and a Specification gap waits for a later change; in
     practice: a failing assertion is recorded in `spec-gaps.md` before the
     builder touches the spec copy.
  2. **Fix as found**: repair the first difference and record the outcome
     after; gain: fewer trial restarts; cost: the outcome is chosen to fit the
     fix, and the Specification's silence becomes unobservable; in practice:
     the same adjustment is recorded as `implementation defect` whatever the
     Specification said.
  3. **Stop the trial on the first difference**: record one difference and
     end; gain: one small evidence set; cost: `S13` requires every difference
     classified, and the next difference stays unknown; in practice: a trial
     with three differences records one and reports success on the rest.
- Disposition: carried. The design's Decisions ledger decides it, and the gap
  list records each outcome.

### D3: What the trial may read

- Question: The spec copy excludes the `e2e` package and the replaced Source
  code, but the frozen E2E runner needs a `ready_fixture` binary and a product
  root for a few repository-file assertions. Which of those inputs come from
  the checkout, and does that weaken the clean-room claim?
- Options:
  1. **Test infrastructure from the checkout, product behavior from the
     rebuilt binary**: `GIVN_BIN` names the rebuilt binary; `GIVN_READY_FIXTURE`
     and `GIVN_ROOT` name the checkout because they are test infrastructure and
     read-only repository assertions; gain: the frozen suite runs without
     moving files into the spec copy; cost: the evidence must state that the
     fixture and a few assertions read the checkout, so the rebuild does not
     claim to reproduce those bytes; in practice: `GIVN_BIN` points at the
     rebuilt binary and `GIVN_ROOT` at the checkout.
  2. **Rebuild the fixture inside the spec copy**: the builder recreates
     `ready_fixture` from the Specification; gain: no checkout input except the
     frozen suite; cost: `ready_fixture` is a test harness binary, not product
     behavior, its source sits in the replaced `tests/` set, and the
     Specification does not declare it, so the builder has no authoring input;
     in practice: the builder invents a fixture the frozen suite may not
     accept.
  3. **Exclude the run-execution scenarios from the trial**: run only the
     `@e2e` scenarios that do not need the fixture; gain: no checkout input;
     cost: the frozen suite is no longer whole, and the excluded scenarios are
     exactly the product-start behavior the rebuilt binary must satisfy; in
     practice: the trial reports a smaller total than the scenario inventory.
- Disposition: carried. The design's Decisions ledger decides it, and the
  builder notes record every input.

## Seed Question Disposition

### Which recorded evidence satisfies the external-evidence prerequisite?

- Question: Which recorded evidence must exist before the self-rebuild trial
  starts?
- Disposition: answered by `givn/ideation/ise-rebuildability/plan.md`. The
  plan's phase table gates `S13` on `S3`, `S4`, `S5`, and `S12`, and its
  dependency graph routes `S12` to `S13`. That is the seed's third option: the
  last full-product trial, `S12`, is the prerequisite, not every run from `S7`
  to `S12`. The plan's section 0 records `S10` to `S12` as outstanding as of
  2026-11-16. The change was designed and tasked before that evidence existed.

## Operator Override: S12 Prerequisite

The plan's phase table gates `S13` on `S3`, `S4`, `S5`, and `S12`
(`plan.md:165`), and `plan.md` section 0 records `S10` to `S12`, the Passuff
arm, as outstanding as of 2026-11-16. The missing evidence row is the `S12`
phase row (Passuff full-product `Dry-Run Ise`); no `S10` to `S12` evidence is
archived.

The operator's S13 execution instruction of 2026-11-17 states that the plan's
blocker premise is stale: the Passuff product is committed on `ise/trial`,
but the evidence rows are still absent. The instruction starts the S13 trial
with the `S12` row outstanding. This trial proceeds under that explicit
override.

`S12` is not satisfied. This trial is not `S12` evidence, and `S14` stays
gated on `S12`; `S14` must not count this trial as the Passuff arm.
`evidence/ise/spec-gaps.md` records the absent row under `## Setup failures`
and names this section.
