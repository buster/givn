---

status: accepted
date: 2026-10-20
decision-makers: buster (project owner)
consulted: none
informed: none

# Analyze source code behind a neutral SourceAnalyzer port with a syn adapter

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

Evidence: the readability design weighed four materially different options —
coupling metrics and classification directly to `syn`, a neutral port with a
`syn` adapter (chosen), starting with tree-sitter behind the same port, and
running an external analyzer process — so a real alternative exists that does
not use the chosen parser or process model (design.md D3 and the analyzer-port
section). The decision fixes a component boundary and a dependency direction:
`model` is the shared contract, `syn` may appear only inside `syn_adapter`, and
metrics, classification, and rendering depend on the neutral model rather than
on a parser. It is durable because the later `add-readability-review-gate`
change analyzes the same units at the change base and head, and a later
tree-sitter adapter must pass the same non-E2E scenarios without changing the
report, metrics, classification, or review markers; reversing the port means
rewriting all those consumers. The observable report behaviour, classifications,
and exit codes are owned by the `assess-code-structure` Gherkin specification;
`design.md` owns the change-scoped model fields, metric formulas, and rendering
mechanics, but neither it nor a chapter can own a cross-change dependency
boundary. The active and archived registers were searched in full: ADR-069 owns
the feature-free dependency surface but not source analysis; ADR-037 owns the
deterministic overlap fingerprint; ADR-028 owns the mandatory-check boundary;
ADR-011 owns feature-file specification; no record owns a source analyzer,
source-unit metrics, or an engine-independent analysis contract. The route is a
new record, not an amendment.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: the later review gate analyzes change base and head through the
      same model, a later tree-sitter adapter is accepted against the same
      contract scenarios, and abandoning the port would rewrite metrics,
      classification, both renderers, and the gate's analysis path.
- [x] **Shapes quality attributes** - It materially affects maintainability and
      testability because: the parser becomes replaceable and a fake adapter can
      prove the port carries enough information, while one shared metric and
      classification implementation is tested once rather than per parser.
- [x] **Structural** - It defines a boundary, contract, and dependency direction
      because: `cli -> readability -> model`; `metrics`, `classify`, and
      `report` depend only on `model`; `syn_adapter` depends on `model` and
      `syn`; nothing depends on `syn_adapter` except the analyzer factory.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: importing `syn` into a metric module or
      computing a metric inside an adapter would break the replacement contract
      and make a second language adapter a fork of the report.

## Context and Problem Statement

Givn has no source-analysis subsystem; the readability report must measure and
classify Rust source units while keeping its human and machine-readable formats
stable when the analysis engine changes. The later review gate also needs the
same unit analysis at a change's base and head. If metrics, classification, and
rendering depend directly on `syn`, the parser's types and limitations enter the
durable report contract, and a tree-sitter adapter becomes a rewrite instead of
a sibling implementation.

## Decision Drivers

- The report format, metrics, classifications, and later review markers must not
  depend on one parser.
- `syn` should be the first adapter, not a permanent part of the analysis
  contract.
- Metrics and classification are defined once and remain testable without a
  real parser.
- A later tree-sitter adapter must be a drop-in implementation behind the same
  contract.
- The analysis stays in-process, deterministic, and offline.

## Considered Options

- **Neutral `SourceAnalyzer` port with a `SynAnalyzer` adapter**: adapters return
  a parser-neutral `Analysis`; shared metrics, classification, and rendering run
  above the port.
- **Couple metrics and rendering directly to `syn`**: metric and report code
  consume `syn` AST types without a neutral model.
- **Start with tree-sitter behind a port**: the first adapter parses with
  tree-sitter; `syn` is not used.
- **Run an external analyzer process**: a separate tool parses the project and
  Givn consumes its output.

## Decision Outcome

Chosen option: "Neutral `SourceAnalyzer` port with a `SynAnalyzer` adapter",
because the report contract then survives a parser replacement and the later
review gate can consume the same neutral analysis without a format change.

### Consequences

- Good, because metrics, classification, and both renderers are implemented
  once above the port and can be exercised with a hand-written fake adapter.
- Good, because a later tree-sitter adapter adds one sibling module and one
  factory arm; it must pass the unchanged adapter contract and cannot edit the
  report, metric, classification, or marker code.
- Good, because the report carries no analyzer identity, so equivalent neutral
  analyses produce byte-identical output.
- Bad, because the neutral `Analysis` model is a shared compatibility surface:
  every later adapter must map every construct into it, and parser-specific
  information tree-sitter exposes beyond the model can only be used after a
  contract change affecting metrics, classification, the report, and the later
  review gate (R-99).
- Bad, because the port isolates `syn` types but not the parser's cost or
  limitations: `syn` remains a linked dependency whose binary-size effect must
  be measured, and item-position macros or non-UTF8 files can only be reported
  as diagnostics rather than analyzed (R-100, R-102).
- Bad, because the shared metrics are syntactic: LCOM4 and TCC/LCC use visible
  calls and field accesses that the neutral model can represent, so dynamic or
  ambiguous dispatch is under-counted and a threshold verdict can misdirect a
  plan as well as reveal debt (R-101).

### Confirmation

Non-E2E Gherkin scenarios confirm the boundary in process through the public
`givn::readability` API: "The same source produces the same report through
either analysis engine" runs metrics, classification, and both renderers from a
hand-written neutral `Analysis` supplied by a fake adapter and from
`SynAnalyzer`, and requires byte-identical output; the boundary and determinism
scenarios exercise the same public surface. A `#[cfg(test)]` source scan of
`src/readability/**` and `src/cli/readability.rs` keeps `use syn` and `syn::`
only in `syn_adapter.rs`. A future tree-sitter adapter is
accepted only when it is registered in the factory and the same non-E2E
scenarios pass unchanged.

## Pros and Cons of the Options

### Neutral `SourceAnalyzer` port with a `SynAnalyzer` adapter

- Good, because the parser is replaceable without changing the report contract.
- Good, because shared metrics and classification are parser-independent and
  testable with a fake adapter.
- Good, because the dependency direction is explicit and enforceable by a
  source scan.
- Neutral, because the adapter must translate the complete AST into the neutral
  model, which is more work than consuming `syn` directly.
- Bad, because the neutral model can lose parser-specific detail and becomes a
  cross-change contract that is expensive to widen.

### Couple metrics and rendering directly to `syn`

- Good, because it needs no translation layer and reaches every `syn` fact
  directly.
- Bad, because `syn` types become part of the report and later review-gate
  contract.
- Bad, because a tree-sitter adapter requires a parallel metric and rendering
  implementation, and the two formats can drift.

### Start with tree-sitter behind a port

- Good, because tree-sitter is designed for multiple languages and incremental
  parsing.
- Bad, because the first release is Rust-only, where `syn` already parses the
  language completely without a grammar bundle.
- Bad, because the grammar and node-mapping surface is larger than this
  experiment needs before the report contract has been validated.

### Run an external analyzer process

- Good, because the analyzer could be replaced or upgraded without rebuilding
  Givn.
- Bad, because it adds a process, an executable dependency, and a serialization
  contract to an offline single-binary tool.
- Bad, because the report would depend on an external tool's availability and
  version rather than a linked, testable crate.

## More Information

- Change design: `givn/changes/add-readability-report/design.md` (D2, D3, DD3 to
  DD8).
- Specification: `givn/specs/assess-code-structure/` and the
  `readability-report` delta.
- Related records: ADR-069 owns the feature-free dependency surface; ADR-037
  owns deterministic overlap evidence; this decision owns only the source
  analysis boundary and its replaceability contract.
- The later review gate (`add-readability-review-gate`) and the later
  tree-sitter adapter are the two consumers that make this a cross-change
  boundary rather than a local implementation detail.
