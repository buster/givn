# Readability addon

**TL;DR:** The `readability` addon is an optional, default-off experiment for
production Rust. Its core idea is outline comprehension: read the outline of a
type or module — method declarations, signatures, and internal calls, without
bodies or data — and check that it explains the unit's responsibility and call
order. When the addon is enabled, every touched unit with methods needs an
outline verdict from a fresh-context reviewer. Metrics are supporting evidence,
not the gate. An unreadable outline has no justification path. The addon stores
no baseline, ignores non-Rust code, and is not a product guarantee: the schema,
thresholds, classifications, and gate rules may change.

## What it does

- **Report.** `givn readability report` prints each unit's outline first —
  method declarations, signatures, and internal calls — then the supporting
  metrics, as human text or JSON. It never blocks a change and stores no
  baseline.
- **Review gate.** With the addon enabled, `givn check review --change <id>`
  prints the outline of every touched unit that has methods. The review record
  must give each outline a `PASS` with a responsibility and call-order
  explanation, or a `FAIL`. A missing verdict and a `FAIL` both block. The
  reviewer must be a fresh-context reviewer, not the author.

## Outline comprehension — the core idea

The addon is not a size counter. Its central rule is R1: a reader given only a
unit's outline should be able to state (a) what the unit is responsible for and
(b) why its internal calls happen in that order.

The outline lists the unit's method declarations and signatures, their order,
and the internal calls each method makes. Data — fields and enum variants — is
not part of the outline. It contains no method bodies and no comments.
`givn readability report` prints the outline for every unit; `--json` emits the
full outline under `units[].outline`, including call-resolution labels.

An example, abridged from the report:

```text
OUTLINE type:src/engine.rs::Engine
  method [public] fn build(&self) -> Engine
    calls: validate -> compile
  method [private] fn validate(&self) -> Result<()>
  method [private] fn compile(&self) -> Engine
  metrics (supporting): code lines 120; methods 3; public methods 1
```

The names and call order state the unit's job: build an `Engine` by validating,
then compiling. The metric line is support, not the point.

The outline is the gate. Metrics find candidates for a reviewer's attention,
but they do not decide. Both directions are valid:

| Outline | Metrics | Result |
|---|---|---|
| Readable | clean | pass |
| Readable | flag | pass; the metric is supporting evidence |
| Unreadable | clean | fail; the outline is the finding |
| Unreadable | flag | fail; the metric supports the finding |

An unreadable outline is not justified. Fix the code and re-review.

## Workflow

![Readability addon workflow: implement produces an optional advisory report with the outline; review produces outline verdicts and a PASS and checks every touched outline; archive checks the review record](readability-workflow.svg)

Source: [`readability-workflow.d2`](readability-workflow.d2). Regenerate with
`d2 docs/readability-workflow.d2 docs/readability-workflow.svg`.

The report is advisory. The review gate is blocking. The report never stores a
baseline; the gate only looks at the units the change touched.

## What is produced and checked

| Phase | Produced by the addon | Checked by the addon |
|---|---|---|
| Proposal / design | Nothing automatic. You may run the report as evidence. | Nothing. |
| Implement | An optional report with each unit's outline first, then the supporting metrics, as human text or JSON. | Nothing. The report never blocks a change. |
| Review | Per-unit `READABILITY: OUTLINE PASS` or `FAIL`, `READABILITY: AUTHOR` and `REVIEWER`, and `READABILITY: PASS` when every outline passes. | Every printed outline has a verdict. A `PASS` has a responsibility and call-order explanation; a `FAIL` blocks. The reviewer differs from the author. |
| Archive | Nothing new. | The review record is complete. An outlined unit without a verdict blocks archive. |

The review record markers:

```text
READABILITY: AUTHOR <id>
READABILITY: REVIEWER <id>
READABILITY: OUTLINE PASS <unit> | <responsibility; why the calls happen in that order>
READABILITY: OUTLINE FAIL <unit> | <reason>
READABILITY: PASS
```

`REVIEWER` must differ from `AUTHOR`. An empty `PASS` explanation fails. Unknown
or conflicting markers fail. A standalone `FAIL <reason>` still blocks.

## What is checked in the source code exactly

The report and the gate share the same analysis:

- **Scope.** Only production `.rs` files under `src/` (or explicit `--path`
  roots). `tests/`, `benches/`, `examples/`, and `target/` are excluded;
  `#[cfg(test)]` items are excluded; `src/bin/**` is included.
- **Units.** A unit is a type (struct, enum, trait, union, or type alias) or a
  module. Its methods are checked with it. A unit with no methods has no
  outline and needs no outline verdict.
- **Outline.** Method declarations, signatures, order, and internal calls. Data
  is excluded. The gate prints the outline for every touched unit with methods.
- **Outline verdict.** Every printed outline needs one `OUTLINE PASS` with a
  non-empty responsibility/call-order explanation, or one `OUTLINE FAIL`. A
  missing verdict, an empty explanation, a `FAIL`, or conflicting verdicts for
  one unit fails. There is no justification path for a `FAIL`.
- **Supporting type metrics.** Over 500 code lines: a trait or a type with more
  than 50 public methods is an `interface`; a struct or enum with at most one
  method is `data-only`; LCOM4 over 2 or TCC below 50% is a `god-type`;
  otherwise it is a `cohesive-large-type`.
- **Supporting module metrics.** Over 1,000 code lines is a `module`; 500–1,000
  with at least two declared types is a `multi-type-module`; 500–1,000
  otherwise is a `module`.
- **Supporting method metrics.** A method over 100 code lines or over 50 flow
  complexity is a `method-length` finding.
- **Supporting findings are not decisions.** The gate prints them as
  `SUPPORTING` and never requires a `FIXED` or `JUSTIFIED` decision for them.
- **Reviewer identity.** `READABILITY: AUTHOR` and `READABILITY: REVIEWER` are
  required when any outline verdict is present, and must differ.
- **Fail-closed guards.** No reachable change commit, a shallow clone, dirty
  production paths, a non-UTF8 or line-break path, a symlink, a submodule, an
  unreadable blob, or a changed `.rs` file the analyzer cannot analyze fails
  the gate.
- **Removed units.** A removed unit has no head outline, so it needs no outline
  verdict.
- **Accounting is unit-level.** Every outline verdict names exactly one unit.

## Non-Rust code

The addon is Rust-only. The report discovers only `.rs` files; non-Rust files
under `src/` are ignored. The review gate applies the same scope and ignores
changed paths that do not end in `.rs`.

Non-Rust code is not “unsupported”: it is out of scope. A change that only
touches non-Rust files has nothing for the gate to check. A mixed-language
`src/` tree gets a Rust-only structural view.

| Input | Report | Review gate |
|---|---|---|
| Non-Rust file under `src/` | Ignored | Ignored |
| Changed non-Rust file | Not analyzed | No effect |
| Rust file with an unsupported construct | Named in `unsupported` | Reported; not a failure by itself |
| Rust file that cannot be parsed | Named as a parse failure | Fails closed if changed |
| Rust file the analyzer skips under a production root | Not analyzed | Fails closed if changed |
| Rust file under `tests/`, `benches/`, `examples/`, `target/` | Excluded | Not production scope |

A Rust file the analyzer cannot parse fails closed if changed. An unsupported
construct is named in the report's `unsupported` list; it is not automatically a
gate failure. Supporting another language would need a second adapter behind
the neutral `SourceAnalyzer` port (ADR-074); the Rust adapter does not pretend
to understand other syntax.

## Expected outcome

- Every touched unit with methods is read through its outline by an independent
  reviewer.
- An unreadable outline blocks even when every metric is clean.
- A readable outline passes even when a metric flags the code; the metric stays
  visible as supporting evidence.
- The reviewer explains responsibility and call order in the record, so the
  judgment is inspectable.
- The report remains useful for untouched code without blocking unrelated
  changes.

Success looks like outlines that a fresh reader can explain, verdicts that name
the unit's job, and code changes that improve the outline rather than only the
metrics.

## What may hint at a problem due to this addon

Feedback on these signals is wanted. This is an experiment; if you see them,
say so.

- **`OUTLINE PASS` becomes a rubber stamp.** Short, generic explanations that
  would fit any unit mean the reviewer is not reading the outline.
- **One explanation covers several units.** Unit-level accounting is the point;
  a single paragraph for a module and its types is not evidence.
- **The reviewer names a responsibility but not the call order.** R1 has two
  parts; a verdict that omits why the calls happen in that order is incomplete.
- **The same unit recurs across changes.** Repeated outline failures on one unit
  mean the shape should change, not the wording.
- **The metric discussion dominates the review.** If the verdict explains the
  line count and not the method names and calls, the secondary evidence has
  become primary again.
- **The addon stays disabled or the report is ignored.** An opt-in gate that
  nobody enables or acts on has no effect.
- **False positives or negatives dominate.** Macros, non-UTF8 files,
  conditional compilation, and braces inside string literals are known limits
  (risks R-105 and R-108); the report is a completeness-limited view, not a
  total-debt claim.
- **A verdict is complete and still wrong.** The gate checks presence and
  shape, not the judgment; independent review is a second opinion, not a proof.
- **Untouched code is assumed clean.** The gate sees only touched units. A green
  gate does not mean the repository has no unreadable outlines.

## Enable and disable

```sh
givn addons enable readability
givn readability report
givn readability report --json --out readability.json
givn check review --change <id>
givn addons disable readability
```

`givn init` never enables it. When disabled, the report refuses with the named
reason, the review gate does not run, and the readability section is omitted
from the served review guidance.

## References

- Report specification: [`givn/specs/assess-code-structure/readability-report.feature`](../givn/specs/assess-code-structure/readability-report.feature)
- Gate specification: [`givn/specs/verify-change/readability-review-gate.feature`](../givn/specs/verify-change/readability-review-gate.feature)
- Applied independent review (historical): [`givn/archive/apply-readability-independent-review/review.md`](../givn/archive/apply-readability-independent-review/review.md)
- Neutral analyzer port: [`docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md`](arc42/adr/ADR-074-neutral-source-analyzer-port.md)
- Known risks: [`docs/arc42/11-risks-and-technical-debt.md`](arc42/11-risks-and-technical-debt.md) (R-105 to R-109, D-24)
- Reviewer rules: [`assets/instructions/review.md`](../assets/instructions/review.md) (`Readability review gate`)
