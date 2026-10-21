# arc42 Documentation Update: add-readability-report

> This is the change-level arc42 **impact assessment** and decision record. It
> is not an ADR template. The neutral source-analyzer boundary is recorded in
> exactly one MADR file, `docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md`;
> chapter 09 remains a register.

## Impact assessment

| # | Chapter | Affected? | Reason | Files updated |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The experiment adds a maintainer goal and a product functional requirement, and the existing "all addons by default" statement becomes false once an addon is default-off. | FR-46 added; the addon-default statement corrected; a short structural-visibility goal paragraph added. `docs/arc42/01-introduction-and-goals.md` |
| 2 | 02 architecture-constraints | Yes | The experiment adds durable constraints: Rust-only first, offline and deterministic, advisory only, no stored baseline, `syn` as the first parser behind a neutral port, and default-off opt-in. | TC-42 added; the 2.3 addon-default statement corrected for default-off entries. `docs/arc42/02-architecture-constraints.md` |
| 3 | 03 context-and-scope | Yes | A new user-facing command and a default-off addon flag join the external surface; the scope gains the advisory report and excludes the later review gate, baselines, other languages, and refactoring. | Interface rows for `givn readability report [--json] [--out]` and `addons.readability`; in-scope and out-of-scope bullets. `docs/arc42/03-context-and-scope.md` |
| 4 | 04 solution-strategy | Yes | The solution strategy gains the neutral-port approach (shared metrics and classification above a replaceable analyzer) and must state the default-on addon rule correctly. | "Code-structure report" strategy row added; Arc42 strategy row corrected. `docs/arc42/04-solution-strategy.md` |
| 5 | 05 building-block-view | Yes | New building blocks are introduced: `cli::readability`, the `readability` orchestration, the `SourceAnalyzer` port, and the `syn` adapter; the addon registry and init behavior change; `syn` becomes a dependency. | Crate diagram nodes for the port and adapter; `cli::init`, `cli::addons`, `cli::readability`, `readability`, and analyzer/adapter responsibility rows; `syn` dependency row. `docs/arc42/05-building-block-view.md` |
| 6 | 06 runtime-view | Yes | Two runtime paths are new: the enabled report flow (gate, scan, analyze, metrics, classify, render) and the disabled path that exits 1 with the named reason. The init and addon-list flows also change for default-off entries. | Init step 6 and its follow-up paragraph corrected; 6.12 addon list/enable/disable/upgrade behavior updated; new 6.19 with a Mermaid sequence. `docs/arc42/06-runtime-view.md` |
| 7 | 07 deployment-view | No | The product remains one statically linked binary with no new process, container, port, database, or deployment artifact; the `syn` binary-size effect is a chapter 11 risk, not a deployment-topology change. | None |
| 8 | 08 crosscutting-concepts | Yes | The addon-default rule changes across initialization and the directory layout, and the report adds cross-cutting rules for source scope, determinism, per-file diagnostics, advisory exits, no stored state, and parser isolation. | 8.4 and 8.11 default-on statements corrected; new 8.26 records the report and source-analysis boundary. `docs/arc42/08-crosscutting-concepts.md` |
| 9 | 09 architecture-decisions | Yes | The neutral `SourceAnalyzer` port qualifies as one durable decision; it fixes the dependency direction and the engine-independent analysis contract for the later review gate and the later tree-sitter adapter. | ADR-074 created and marked `accepted`; chapter 09 register paragraph added; `adr/README.md` active row and count updated. `docs/arc42/09-architecture-decisions.md`, `docs/arc42/adr/README.md`, `docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md` |
| 10 | 10 quality-requirements | Yes | The report adds four testable quality scenarios: byte-stable determinism, advisory non-blocking behavior, named unanalysed files, and engine-independent replacement. | Quality-tree branch added; Q-108 through Q-111 appended in order. `docs/arc42/10-quality-requirements.md` |
| 11 | 11 risks-and-technical-debt | Yes | The ADR's "Bad, because..." consequences and the design's experiment risks need durable entries: neutral-model compatibility, `syn` blind spots, syntactic cohesion and directional thresholds, binary growth, repeated re-classification without a baseline, and the unresolved feedback/exit rule. | R-104 through R-107 added; D-24 and D-25 added. `docs/arc42/11-risks-and-technical-debt.md` |
| 12 | 12 glossary | Yes | New domain vocabulary is introduced and the existing Addon definition becomes false for default-off entries. | Readability experiment, readability report, source unit, production source, readability classification, readability action, outline, LCOM4, and TCC/LCC entries added; Addon definition corrected. `docs/arc42/12-glossary.md` |

Index updated: `docs/arc42/README.md` records the proposed ADR and the affected
chapters. Chapter 7 was checked and deliberately not edited.

## ADR qualification verdict

Candidate: the neutral `SourceAnalyzer` port, with `SynAnalyzer` as the first
adapter, tree-sitter as a later adapter, and metrics/classification/renderers
above the port.

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "NEW_ADR",
  "canonical_artifact": null,
  "target_adr": null,
  "replacement_adr": null
}
```

Evidence:

- **Alternatives.** Four materially different options are recorded in the
  design: direct `syn` coupling, the chosen neutral port with a `syn` adapter,
  tree-sitter first behind the same port, and an external analyzer process. The
  alternatives differ in parser, process model, and contract ownership, not in
  wording.
- **Architectural impact.** The decision fixes a component boundary and a
  dependency direction: `cli -> readability -> model`; `metrics`, `classify`,
  and `report` depend only on `model`; `syn` appears only in `syn_adapter`; the
  report carries no analyzer identity. It also fixes the durable report contract
  (`schema_version` and `rules_version`) that later consumers read.
- **Durable consequence.** The later `add-readability-review-gate` change
  analyzes the same units at change base and head, and a later tree-sitter
  adapter must pass the same non-E2E scenarios. Reversing the port would rewrite
  metrics, classification, both renderers, and the later gate's analysis path;
  this is more than one-change implementation effort.
- **Lower-level artifact.** The Gherkin specification owns observable report
  behaviour, classifications, and exit behavior. `design.md` owns this change's
  model fields, metric formulas, and rendering mechanics. Chapter 5 records the
  current component fact. None of them can own a cross-change dependency
  boundary that forbids `syn` outside the adapter and admits a drop-in later
  parser; the ADR owns only that boundary and its consequences.
- **Existing-ADR check.** The active register (26 accepted records through
  ADR-074; ADR-050 and ADR-051 reserved) and the archived register were searched for
  source analysis, analyzers, ports, parsers, metrics, readability, and
  tree-sitter. ADR-069 owns the feature-free dependency surface but not source
  analysis; ADR-037 owns the deterministic overlap fingerprint; ADR-028 owns the
  mandatory-check boundary; ADR-011 owns feature-file specification. No record
  owns the analyzer boundary, so the route is `NEW_ADR`, not an amendment or
  supersession.
- **Must be shared (supporting only).** A contributor adding a language adapter
  or a metric must not import `syn` outside the adapter or compute metrics inside
  it; doing so silently breaks the replacement contract.

Falsification tests:

1. **Single-change reversibility:** does not fail. The decision is implemented
   within this change, but it constrains the later review gate and a later
   adapter, so it is not fully reversible within one component.
2. **No multi-change constraint:** does not fail. It constrains at least two
   future changes (`add-readability-review-gate`, the tree-sitter adapter) and
   two components (analyzer adapters versus shared metrics/classification).
3. **Product-behavior exclusion:** does not fail. The report's observable
   behavior is owned by Gherkin, but the decision moves a component and
   dependency boundary; the report format and later markers are not a workflow
   behavior decision.

Number check: ADR-073 is the highest assigned active record; ADR-050 and
ADR-051 are reserved and unassigned; therefore ADR-074 is the next free number.
No parallel ADR was created for the addon gate, the no-baseline rule, the report
schema, the classification table, or the `syn` version: those are observable
behavior, change-scoped design mechanics, or current facts owned by the spec,
`design.md`, and the chapters above.

ADR consequence routing:

- "Bad" neutral-model compatibility surface -> R-104.
- "Bad" `syn` linked dependency, parser blind spots, and binary cost -> R-105
  and R-107.
- "Bad" syntactic cohesion and threshold misdirection -> R-106.
- The no-baseline and unresolved-feedback costs are design/experiment debts
  recorded as D-24 and D-25; they are not additional ADRs.

## Documents updated

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
- `docs/arc42/adr/README.md`
- `docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md` (new)

Chapter 7 was assessed and left unchanged.

## Open documentation question

ADR-026's accepted body still says `givn init` enables all known addons. That
statement was true when the Arc42-addon decision was accepted but is now false
for default-off entries. The record was left unchanged because correcting an
accepted ADR is an amendment, not part of this change's assessment. If the
register owner wants ADR-026 to describe the current default-on rule, it needs an
explicit ADR-026 amendment in a later documentation pass.

## Status

STATUS: DONE
