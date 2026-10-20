# arc42 Documentation Update: add-readability-review-gate

## Impact assessment

Every row was checked against `proposal.md`,
`specs/verify-change/readability-review-gate.feature`,
`specs/assess-code-structure/readability-report.feature`, and `design.md`.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The report becomes findings-only and the gate's success condition changes from accounting to fix-or-justify. | FR-46 drops "one recommended action"; FR-47 requires a `FIXED` or `JUSTIFIED` decision per finding at the committed head. The goal paragraph states the report is findings-only and the gate is opt-in. |
| 2 | 02 architecture-constraints | Yes | The gate's head, base edges, decision model, rationale schema, and SCM operations changed. | TC-42 fixes schema 2 and no action/recommendation. TC-43 fixes committed `HEAD`, the dirty-production guard, first-parent/`ancestors current` base, root/shallow/rebase rules, unit-level `FIXED`/`JUSTIFIED`, rationale validation, and no stored state. |
| 3 | 03 context-and-scope | Yes | The report output changes and the change-scoped artifact directory changes. | The report row is schema 2 findings-only; the review row names unit ids and missing decisions; `readability/rationales/` replaces plan/exception artifacts. |
| 4 | 04 solution-strategy | Yes | The report strategy loses recommendations and the gate strategy becomes fix-or-justify. | The report row says one classification and evidence only. The gate row says `FIXED` under threshold or `JUSTIFIED` with a change-scoped rationale. |
| 5 | 05 building-block-view | Yes | New gate modules and the removed `Action` type. | Adds `gate::head`, `gate::revision`, and `gate::rationale`; updates `readability` to produce no action; updates the marker and finding responsibilities. |
| 6 | 06 runtime-view | Yes | The review runtime changes head, base edges, and decisions. | Section 6.20 now documents the committed head, dirty guard, two revision trees, unit-level findings, rationale validation, and the exact PASS/FAIL line. |
| 7 | 07 deployment-view | No | Still one native binary; no new process, service, port, container, or distribution artifact. | — |
| 8 | 08 crosscutting-concepts | Yes | The marker vocabulary and the report contract changed. | 8.26 states findings-only schema 2. 8.26a is rewritten around `FIXED`, `JUSTIFIED`, `PASS`, and `FAIL`, the rationale schema, banned reasons, and code ownership. |
| 9 | 09 architecture-decisions | Yes | The gate amends existing records instead of adding one. | Records the 2026-10-20 ADR-028 fifth amendment (conditional mandatory review member) and ADR-071 second amendment (five SCM history/revision operations). The active register stays 26 accepted, 0 proposed. |
| 10 | 10 quality-requirements | Yes | The report output and the gate's failure conditions changed. | Q-109 becomes findings-only; Q-112–Q-117 cover determinism, fix-or-justify, change relativity, fail-closed diagnosis, addon isolation, and schema 2. |
| 11 | 11 risks-and-technical-debt | Yes | The old watch/plan/exception escape is gone, the head is committed, and the report schema breaks. | R-108–R-115 are restated or added; R-108 remains the conditional-compilation limitation (formerly R-97); R-115 records the schema-2 compatibility surface; D-24–D-26 describe the experiment record, decisions, and rationale validation. |
| 12 | 12 glossary | Yes | The domain vocabulary changed. | Removes action, watch, plan, exception, closure, and finding accounting; adds report finding, gate finding, finding decision, fixed decision, justified decision, rationale artifact, and code ownership. |

## Affected-chapter evidence

- **01 introduction-and-goals.** `FR-46` now says one classification and its
  evidence, with no action or recommendation. `FR-47` says the gate analyzes the
  committed head and requires `FIXED` or `JUSTIFIED`. The goal paragraph keeps
  the report advisory and the gate opt-in.
- **02 architecture-constraints.** `TC-42` fixes schema 2. `TC-43` carries the
  design's D1, D4, D7, DD4, DD5, DD6, DD7, DD15, and DD16: first-parent or
  `ancestors current` base, committed head, dirty guard, shallow/root rules,
  unit-level decisions, rationale validation, no stored state, and no new
  command.
- **03 context-and-scope.** The report row names schema 2 and no action; the
  review row names unit ids and missing decisions; the artifact row names
  `readability/rationales/<unit-slug>.md`. Out of scope names rationale grading
  and the removed compatibility markers.
- **04 solution-strategy.** The report row says findings only. The gate row says
  `FIXED` under threshold or `JUSTIFIED` with a change-scoped, owner-approved
  rationale; no watch/plan/exception ladder.
- **05 building-block-view.** The responsibility table adds `gate::head`,
  `gate::revision`, and `gate::rationale`; `readability` no longer selects an
  action; `gate::findings` applies the finding predicate and post-analysis
  guard; `gate::markers` parses only the four fixed markers.
- **06 runtime-view.** Section 6.20 documents the committed head, the dirty
  guard, entry kinds and non-UTF-8 refusal, unit-level decisions, the rationale
  fields, the `FAIL` winner, and the exact `readability review: PASS` /
  `readability review: FAIL (<reason>)` lines.
- **08 crosscutting-concepts.** 8.26 names schema 2 and the findings-only
  contract. 8.26a owns the marker table, unit-level rationale contract, banned
  reasons, the "artifacts are not a baseline" rule, the procedural attestation
  limit, and the ownership rule.
- **09 architecture-decisions.** ADR-074 is consumed unchanged. ADR-028 is
  amended a fifth time for the conditional mandatory review member. ADR-071 is
  amended a second time for the five SCM operations. No new ADR file exists.
- **10 quality-requirements.** Q-109 and Q-112–Q-117 carry the new contract.
- **11 risks-and-technical-debt.** R-108 states the measured-upper-bound rule
  and the no-exemption decision. R-109 covers two revision trees. R-110 adds
  Fossil `ancestors current` and non-UTF-8 refusal. R-111 adds dirty and shallow
  failures and the moved-unit result. R-112 states fix-or-justify. R-113 keeps
  the attestation limit. R-114 keeps the opt-in scope gap. R-115 records schema
  2. D-24–D-26 describe the experiment record, the decision evidence, and the
  mechanical rationale check.
- **12 glossary.** The terms use the current vocabulary and each carries an
  anti-term. The old action/watch/plan/exception/closure terms are gone.

## ADR qualification

Three candidates are qualified separately because they route differently.

### Candidate 1: gate behavior and marker vocabulary

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/verify-change/readability-review-gate.feature
target_adr: null
replacement_adr: null
```

- **Alternatives (PASS).** The proposal answers real choices with materially
  different outcomes: D1 offers merge-base, first-touch parent, and `HEAD`; D2
  offers a ledger, marker-derived aggregation, and the manual record; D3 offers
  a unit id, a file path, and inline text; D4 offers strict all-debt, silent
  non-growth, fix-or-justify, and advisory-only; D5 offers a verdict marker, an
  outline hash, and mechanical-only; D7 offers the working tree, committed
  `HEAD` with a guard, and committed `HEAD` ignoring dirt; D8 offers leaving the
  report, revising it here, and a separate change.
- **Architectural impact (FAIL).** The gate adds no component ownership,
  deployment boundary, or compatibility surface. It does add one internal call
  edge: `cli::check -> readability::gate`, which then calls the existing public
  `readability::analyze`/`assess` and the existing `scm::Backend` seam. That is
  the same `cli -> readability` direction the report already uses; it is a
  module-level dependency inside one component, not a new cross-component
  dependency direction. `syn` stays confined to the adapter, and ADR-074's
  boundary is neither moved nor widened.
- **Durable consequence (FAIL).** Reversal is a single-change edit: remove the
  review call, the gate module, the instruction block, and the marker
  scenarios. No data is migrated, no interface outside review consumes the
  markers, and the rationale artifacts are per-change and archived with the
  change. The report schema bump is reversible by restoring version 1 while the
  archived report remains the recorded history.
- **Lower-level artifact (FAIL).** The complete rationale is already owned.
  `readability-review-gate.feature` owns the observable rules and messages,
  `readability-report.feature` owns the findings-only report, the two use cases
  own the guarantees and interaction rows, and `design.md` owns the base
  algorithm, the marker parser, the rationale fields, and the finding table.
- **Must be shared (supporting only).** A contributor writing a review record in
  an opted-in project must know the marker vocabulary and the rationale
  location, and a contributor adding a gate rule must not re-derive a metric.
  Both are delivered by the served review instruction and the specification.
- **Falsification tests.** Single-change reversibility, the repeated review
  convention, and the product-behavior exclusion all fail to qualify: the rule
  is observable workflow behavior owned by the capability.

Route: `CANONICAL_ARTIFACT` to `readability-review-gate.feature` and the
`readability-report` delta.

### Candidate 2: mandatory review-check member

```text
qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-028
replacement_adr: null
```

- **Alternatives (PASS).** A separate review command, an advisory-only member,
  and a conditional member of the existing check were the real options; the
  last is chosen because a separate command can be skipped and advisory-only
  cannot block.
- **Architectural impact (PASS).** It changes which mandatory checks run and
  when review may pass. That is the invariant ADR-028 owns, and it binds every
  opted-in project after this change.
- **Durable consequence (PASS).** The member outlives the experimental addon
  switch: once enabled it cannot be removed or downgraded by a `post` entry,
  and a future change that removes it must amend the record.
- **Lower-level artifact (FAIL).** A capability or design cannot own the
  authority of the mandatory-review boundary; only the existing decision record
  can be amended.
- **Existing-ADR check (PASS).** ADR-028 already carries conditional members.
- **Must be shared (supporting only).** The review instruction is the delivery
  surface; the authority stays in the record.

Route: `AMEND_ADR` to ADR-028 with a dated paragraph.

### Candidate 3: SCM seam refinement

```text
qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-071
replacement_adr: null
```

- **Alternatives (PASS).** Running `git`/`fossil` in the gate, extending the
  seam, and precomputing trees were the real options; extending the seam keeps
  the backend choice in one place.
- **Architectural impact (PASS).** The seam gains five history/revision
  operations, which is a change to a shared component contract that both
  backends implement and the gate consumes.
- **Durable consequence (PASS).** Every future backend must implement the five
  operations; the contract cannot be reverted without touching both backends
  and every caller.
- **Lower-level artifact (FAIL).** The design can describe the operations, but
  the seam's authority and backend-selection rules live in ADR-071.
- **Existing-ADR check (PASS).** ADR-071 owns the seam; this is a refinement,
  not a new boundary.
- **Must be shared (supporting only).** The gate's diagnostics name the backend
  seam; the contract itself stays in the record.

Route: `AMEND_ADR` to ADR-071 with a dated paragraph.

No new ADR file is created, and the active register count stays 26 accepted
records with 0 proposed records.

## Findings

- **Report schema supersession.** The report changes in this PR: `Action` and
  `recommendation` are removed, `schema_version` becomes 2, and the permanent
  report spec/use case are modified by a delta. The archived report change
  remains schema 1 and is not edited.
- **Marker vocabulary replaced.** `WATCH`, `PLAN`, and `EXCEPTION` are gone.
  The four markers are `READABILITY: FIXED`, `READABILITY: JUSTIFIED`,
  `READABILITY: PASS`, and `FAIL`.
- **Unit-level accounting.** A marker names the exact unit id. A type rationale
  does not cover the module finding, and two types in one file need two
  rationales. The gate's tests cover both.
- **Head and base edges.** The design fixes committed `HEAD`, the dirty guard,
  Git first-parent, Fossil `ancestors current`, root commit, merge, rebase,
  shallow clone, source-before-change-directory ordering, and the no-commit
  failure. Each has an executable scenario.
- **ADR counts.** ADR-028 is amended a fifth time and ADR-071 a second time.
  The active register remains 26 accepted records with no proposed record; no
  ADR file is added or removed.
- **Risk IDs.** R-108 is the conditional-compilation limitation (formerly
  R-97), now with the measured-value rule and no cfg exemption. R-115 is added
  for schema 2. D-24, D-25, and D-26 are current.
- **Diagram policy.** All diagrams added by this change are Mermaid fenced
  blocks. No ASCII diagram was introduced.

## Documents updated

- `givn/changes/add-readability-review-gate/proposal.md`
- `givn/changes/add-readability-review-gate/design.md`
- `givn/changes/add-readability-review-gate/specs/verify-change/readability-review-gate.feature`
- `givn/changes/add-readability-review-gate/specs/assess-code-structure/readability-report.feature`
- `givn/specs/verify-change/usecase.md`
- `givn/specs/assess-code-structure/usecase.md`
- `givn/specs/verify-change/givn-workflow-enforcement.feature`
- `README.md`
- `assets/instructions/dev-principles.md`
- `docs/arc42/01-introduction-and-goals.md`
- `docs/arc42/02-architecture-constraints.md`
- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/04-solution-strategy.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/09-architecture-decisions.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`
- `docs/arc42/README.md`
- `docs/arc42/adr/ADR-028-hooks-and-mandatory-checks.md`
- `docs/arc42/adr/ADR-071-scm-backend-selection.md`
- `docs/arc42/adr/README.md`

Not updated: `docs/arc42/07-deployment-view.md` (no deployment change).

## Status

STATUS: DONE
