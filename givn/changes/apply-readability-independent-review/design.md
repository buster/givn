# Design: apply-readability-independent-review

Status: implementation design complete. This change extracts five long methods,
touches two structural modules, and records the gate decisions. It adds no
command, flag, threshold, or gate rule.

## Context

The readability review gate judges every touched unit at committed `HEAD`
against the base derived from the change directory's first commit. A unit that
was a finding at the base and is under its threshold at head is `FIXED`; a unit
that is still a finding at head needs a rationale that records an independent
reviewer and an accept verdict.

The branch carries seven findings from the earlier gate work:

| Unit | Classification | Finding | Current measurement |
|---|---|---|---|
| `module:src/readability/classify.rs` | method-length | long method `decide` | 135 code lines, complexity 23 |
| `module:src/readability/gate.rs` | method-length | long method `review` | 110 code lines, complexity 26 |
| `module:src/readability/gate/findings.rs` | method-length | long method `decide` | 108 code lines, complexity 17 |
| `module:src/readability/report.rs` | method-length | long method `render_json` | 134 code lines |
| `module:src/skills/render.rs` | method-length | long method `find_block_end` | 116 measured code lines |
| `module:src/readability/syn_adapter.rs` | multi-type-module | existing grown | 954 code lines |
| `module:src/skills/targets.rs` | multi-type-module | existing grown | 673 code lines |

The five method-length units become `FIXED` when their longest method falls to
100 code lines or fewer and complexity 50 or lower. The two structural units
stay `JUSTIFIED`.

## Extraction plan

Every extraction moves a coherent block behind a named private function. The
call order, evaluation order, early returns, and error text stay identical.
Existing unit tests are not edited; they continue to exercise the public
functions through the extracted helpers.

### `src/readability/classify.rs` — `decide`

Split the decision ladder by unit kind:

- `classify_type(type_kind, metrics, code_lines, over_type_limit)` owns the
  interface, data-only, god-type, and cohesive-large-type branches and returns
  `Option<(Classification, Vec<Evidence>)>`.
- `classify_module(metrics, code_lines)` owns the module, multi-type-module,
  and module-size branches.
- `over_method_limit(metrics)` owns the method-length predicate.
- `decide` keeps the same guard order: type first, then module, then method
  length, then within limits.

The two identical cohesive-large-type branches are preserved as one branch;
their evidence is unchanged.

### `src/readability/gate.rs` — `review`

Split preparation from decision accounting:

- `prepare(request)` resolves the base, checks dirty production paths,
  materializes both trees, analyzes both sides, computes the changed paths,
  and runs the post-analysis guard. It returns `Preparation::Early` with the
  exact early outcome for every fail-closed case, or `Preparation::Ready` with
  the base, trees, analyses, assessments, changed paths, and accumulated guard
  failures.
- `review(request)` keeps the touched-unit selection, finding collection,
  marker parsing, decision accounting, unknown/fail marker handling, and the
  missing-verdict rule.
- `has_decision_failure(failures)` owns the failure-family predicate that
  decides whether a missing verdict is actionable.
- `missing_verdict(...)` owns the verdict condition.

Tree-materialization failures still return early before analysis. Guard
failures are still accumulated and reported together with decision failures.

### `src/readability/gate/findings.rs` — `decide`

Split the per-unit decision loop:

- `conflicting_units(markers, failures)` returns the units with both a fixed
  and a justified marker and records each conflict once.
- `slug_collisions(findings, failures)` returns the units whose rationale slugs
  collide and records the adjacent pairs in sorted order.
- `decision_for(...)` owns one finding's decision: conflict, fixed, justified
  (with validation), or missing (with the unaccounted failure).
- `decide` owns the loop and returns the accumulated failures.

The collision pair order, the validation calls, and the suppression of
per-unit unaccounted failures when the record is missing stay the same.

### `src/readability/report.rs` — `render_json`

Split the JSON document by section:

- `files_json(assessment)` and `file_json(file)`
- `units_json(assessment)`, `unit_json(unit)`, `methods_json(unit)`, and
  `outline_json(unit)`
- `failures_json(assessment)`, `unsupported_json(assessment)`, and
  `summary_json(assessment)`

`render_json` keeps the top-level keys in their existing order and serializes
the same values with the same key names. The schema version stays 2.

### `src/skills/render.rs` — `find_block_end`

Move the directive text into module constants (`OPEN_CONDITION`,
`CLOSE_CONDITION`, `ELSE_CONDITION`, and `CLOSE_BRACES`) and add
`is_open_condition`, `is_close_condition`, and `is_else_condition`.
`render_lines`, `parse_open_condition`, and `find_block_end` call the
predicates instead of comparing literal directive text inside their bodies.
`find_block_end` then stops at its own closing brace instead of walking into
the test module, so its measured length drops to its real size. The accepted
and rejected template behavior is unchanged.

## Justified units

### `src/readability/syn_adapter.rs`

The module remains the single `syn` boundary fixed by ADR-074. It stays a
multi-type module because cross-file impl attachment and the trait/implementor
relation pass aggregate facts for types declared in other files. The existing
rationale keeps that boundary and rejects splitting `syn` handling across
sibling modules. The touch extracts the pending-impl attachment and the
trait-relation resolution into two private helpers so `attach_impls` names the
two phases instead of carrying both.

### `src/skills/targets.rs`

The module remains the single registry of agent targets and their managed
paths. It stays a multi-type module because the target enum and the
integration records share one path and ownership contract. The existing
rationale rejects splitting the registry by target. The touch extracts the
per-target integration writes from the loop that walks all targets.

## Behavior preservation

- No public signature changes.
- No new or removed CLI output.
- No changes to analyzer thresholds, classification order, decision
  vocabulary, rationale floor, or gate exit codes.
- The existing unit tests and the existing gate feature scenarios are the
  regression net. This change adds one narrow gate scenario for the invalid
  rationale message; it does not replace or weaken an existing scenario.

## Verification

- `cargo fmt --all -- --check`
- `cargo test --locked --offline --lib`
- `cargo test --locked --offline --test readability_gate_smoke`
- `cargo clippy --workspace --all-targets --offline -- -D warnings`
- `./run-tests.sh --non-e2e` (the configured `verify.command`)
- One scenario:
  `GIVN_FEATURES=givn/changes/apply-readability-independent-review/specs cargo test --locked --test features_runner -- --name "An incomplete rationale names the rationale path"`
- `cargo run --offline --quiet -- check review --change apply-readability-independent-review`

## Decisions

Decides D1: move the directive literals to module constants and predicate
helpers instead of changing the span walker's brace semantics — realized in
`src/skills/render.rs`.

## Architecture impact

No new modules, types, ports, or dependencies. The five extracted helpers are
private functions in their existing modules. The two structural modules keep
their existing boundaries. arc42 is not updated for this demonstration change.
