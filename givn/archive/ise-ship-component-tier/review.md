# Review: ise-ship-component-tier

Review kind: self-review
Author: givn-maintainer--owner
Reviewer context: givn-maintainer--owner

README-IMPACT: updated - Configuration
PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Implementation evidence check

### `@wip` and step bodies

`givn lint --change ise-ship-component-tier` reports no `@wip` and no finding.
All ten delta scenarios run; every new step body asserts an observable value
(exit status, stdout/stderr, or receipt JSON). No empty or no-op step was added.

### Checked-task commit verification

| Batch | Revision | Contents |
|---|---|---|
| ship-gate | `d44ac91b0b3a5a38fbe58a799790e67ef3f88db5` | Gate implementation, receipt fields, fixture, five enforcement scenarios |
| ship-guidance | `e784c7fc43f0371def88eead4d94563b5f3bca64` | Five served-guidance corrections and five guidance scenarios |
| docs | `deb9905338f4851ffe6f0a081ca065ff72e77dae` | Reference, glossary, arc42, ADR-070 amendment, R-174 |
| hardening | `e427e87f9f5bfeed72c8d6d26ea2655235a4e90c` | Strict plain-line `Rebuilt code` reader and fixture |
| receipt | `672e0345e9d4405edd970339a71eaab3b871813f` | Change verification receipt and raw result |

Every checked scenario records its batch revision in `tasks.md`. The hardening
commit is a correction to the ship-gate batch, recorded beside it.

### Promised files and contracts

`design.md` names `src/ise/ship.rs`, `src/ise/contract.rs`, `src/proof.rs`,
`src/cli/check.rs`, `src/cli/archive.rs`, `src/state.rs`, `src/hooks.rs`, the
five instruction assets, and the two step-definition files. All exist and are
non-empty. `src/ise/ship.rs` is new; `ise::ship` and `ise::contract` implement
the ship-decision contract, and `proof` plus the command adapters consume it.

### Strict mode

`tests/features_runner.rs` calls `.fail_on_skipped()` and exits non-zero on any
failed or skipped step. The RED run before the implementation failed with
`Step doesn't match any function`; no scenario passed on an undefined step.

### Design match

The implementation matches `design.md` D1-D9, with one recorded deviation:
D6 first accepted a wrapped inline `Rebuilt code: ship` mention. That reader
matched prose in this change's own design and classified a `No rebuild` change
as a ship (`component_executed=1317/1317`). It was replaced by the plain-line
reader D6 now records, and the fixture records the plain line. The design and
design-review carry the correction.

### E2E boundary and receipt

`preflight_e2e_boundary` runs for this `ise`-effective change and accepts the
declared `e2e` package. The delta carries no `@e2e` scenario, so the change E2E
scope is `not_applicable`. The receipt is current at `e427e87f9f5bfeed72c8d6d26ea2655235a4e90c`
and is reused across the documentation-only `672e0345e9` revision: 10
scenarios, zero failed, zero skipped. The change records `No rebuild`, so no
component tier is required.

### Independent probes

- `K=1` on the `Rebuilt code:` prose case: a design quoting the rule returns no
  ship; the plain-line case returns `ship`.
- The exit-0 runner without a result is refused with
  `component_executed=0/2`; the one-scenario result with `1/2`.
- The delta-plus-corpus fixture records one delta scenario and a `component`
  record for `givn/specs` with two scenarios in the same receipt.

## Use-Case and Persona conformance

The change extends `verify-change` (capability `givn-workflow-enforcement`) and
`obtain-effective-guidance` (capability `serve-extended-rules`); routing
confirmed both are owned by their use cases. The delta adds no capability and
changes no use-case goal, guarantee, or interaction inventory. Personas
`developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner`
read the changed command output and guidance; the change declares no visual
design contract, so no visual persona disposition is recorded.

## Arc42 implementation conformance

Walked the twelve rows: 2, 4, 5, 6, 8, 9, 11, and the glossary changed; 1, 3, 7,
and 10 are unaffected. The chapter edits match the implementation:

- `02`: TC-54 records the ship tier, the exact count, and the retained subject.
- `04`: the change-verification strategy row names the ship tier.
- `05`: `ise::ship` is a new block beside `ise`.
- `06`: the change-verification flow and 6.7.1 name the subject selection and
  the component result.
- `08`: the `verify` hook sets and validates the result.
- `09`: ADR-070's register row records the 2026-11-19 amendment.
- `11`: R-174 records the count-only limit.
- `docs/glossary.md`: **Component evidence** is defined.

The ADR amendment is in `ADR-070-forward-only-archive-proof.md`, uses MADR, and
keeps the frontmatter valid. No new ADR file is created because ADR-070 already
owns the receipt.

## Ubiquitous language conformance

The change uses **Execution unit**, **Verification contract**, and **Trial
result** as defined, and adds **Component evidence**. No invented role noun or
bare `decision` is introduced. The refusal text reuses
`component_executed=<executed>/<declared>`.

## Coverage

The coverage addon is effective. The new branches are the ship predicate
(ship, non-ship, zero units), the subject selection (delta, corpus, both), the
three component stale reasons, and the hook result failure. The delta
scenarios execute each refusal and both acceptance paths; the unit tests cover
the reader and the predicate. The archive coverage gate measures the merged
tree and carries the result in the archive receipt.

## Scenario execution evidence

```text
GIVN_FEATURES=givn/changes/ise-ship-component-tier/specs \
  cargo test --locked --test features_runner -- --tags "@givn-workflow-enforcement"
5 scenarios passed

GIVN_FEATURES=givn/changes/ise-ship-component-tier/specs \
  cargo test --locked --test features_runner -- --tags "@serve-extended-rules"
5 scenarios passed

givn check tasks --change ise-ship-component-tier
10 scenarios passed; receipt written

./run-tests.sh --non-e2e
1138 passed, 115 skipped

cargo test --lib proof::tests       24 passed
cargo test --lib ise::ship           6 passed
cargo clippy --workspace --all-targets -- -D warnings   clean
```

## Overlap dispositions

The shape scan reports advisory pairs; none is a duplicate. `givn lint
--change` and the overlap gate both pass.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A whole-system Ise ship without a runner result is refused` | `A whole-system Ise ship that executes fewer component scenarios is refused` | boundary |
| `A whole-system Ise ship without a runner result is refused` | `Review refuses an Ise ship without a component receipt` | boundary |
| `A whole-system Ise ship that executes fewer component scenarios is refused` | `Review refuses an Ise ship without a component receipt` | boundary |
| `A Specification Manifest that is not valid YAML is refused by name` | `A whole-system Ise ship without a runner result is refused` | variant |
| `A Specification Manifest that is not valid YAML is refused by name` | `A whole-system Ise ship that executes fewer component scenarios is refused` | variant |
| `A Specification Manifest that is not valid YAML is refused by name` | `Review refuses an Ise ship without a component receipt` | variant |
| `A category that is not a list is refused by name` | `A whole-system Ise ship without a runner result is refused` | variant |
| `A category that is not a list is refused by name` | `A whole-system Ise ship that executes fewer component scenarios is refused` | variant |
| `A category that is not a list is refused by name` | `Review refuses an Ise ship without a component receipt` | variant |

The first three pairs share the refusal shape but assert distinct causes: no
result file, a short count, and a missing receipt read by a different command.
The remaining pairs are unrelated rules whose fixtures share generic refusal
steps; the manifest-schema pairs are step-text similarity only.

The `[SUBST]` advisories are step-text subsets produced by the shared fixture
vocabulary (`Given a fixture precondition`, `Given a Givn project with ise
effective`); they add no disposition.

## Split-or-keep

Ten scenarios stay separate: the five enforcement scenarios distinguish the
acceptance path, the two-subject path, the missing-result refusal, the short
result refusal, and the review refusal; the five guidance scenarios assert
different instruction text. Merging any pair would lose a distinct failure.

## Retirements

None. No capability or scenario is retired.

## Visual review

The change declares no visual design contract; `design.md` has no
`## Visual Design Contract` section and records `Interface: none` in its
traceability. The CLI output is covered by the scenario assertions, so the
visual gate is exempt.

## Sign-off checklist

- [x] Every requested behaviour is implemented and executed.
- [x] The change's own gate run is green and its receipt is current.
- [x] The full non-e2e suite passes; the e2e suite result is recorded in the
  archive evidence.
- [x] No known limit is recorded in place of a declared verification tier.
- [x] Documentation, arc42, and the glossary match the implementation.

REVIEW: PASS
