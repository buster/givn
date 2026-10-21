# Outline addon

**TL;DR:** The `outline` addon is an optional, default-off experiment for
production Rust and Python. Its core idea is outline comprehension: read the
outline of a type or module — method declarations, signatures, and calls, without bodies or data — and check that it explains the unit's
responsibility and call order. When the addon is enabled, every touched unit
with methods needs an outline verdict from a fresh-context reviewer. Metrics
are hints, not the gate. An unreadable outline has no justification path. The
addon stores no baseline, analyzes only production Rust and Python, and is not
a product guarantee: the schema, thresholds, classifications, and gate rules
may change.

## What it does

- **Report.** `givn outline report` prints each unit's outline first —
  method declarations, signatures, and calls — then the metric
  hints, as human text or JSON. It never blocks a change and stores no
  baseline.
- **Review gate.** With the addon enabled, `givn check review --change <id>`
  prints the outline of every touched unit that has methods. The review record
  must give each outline a `PASS` with a responsibility and call-order
  explanation, or a `FAIL`. A missing verdict and a `FAIL` both block. The
  reviewer must be a fresh-context reviewer, not the author.

## Outline comprehension — the core idea

The addon is not a size counter. Its central rule is R1: a reader given only a
unit's outline should be able to state (a) what the unit is responsible for and
(b) why its calls happen in that order.

The outline lists the unit's method declarations and signatures, their order,
and the calls each method makes. The unit is the root: every method is a
first-level branch, and same-unit calls nest under their caller. A call to
another component is a leaf tagged with the target component; its callee's
calls stay in that component's outline. The report never prints reverse
callers. Data — fields and enum variants — is not part of the outline. It
contains no method bodies and no comments. `givn outline report` prints the
outline for every unit; `--json` emits the full outline under
`units[].outline`, including call-resolution labels.

An example, abridged from the report:

```text
WITHIN-LIMITS type  src/engine.rs  type:src/engine.rs::Engine within-limits
  outline:
    method [public] fn build(&self) -> Engine
      -> [private, Engine] validate
      -> [private, Engine] compile
      -> [public, module] normalize
    method [private] fn validate(&self) -> Result<()>
    method [private] fn compile(&self) -> Engine
  metrics (hint): code lines 120; methods 3; public methods 1
```

The names and call order state the unit's job: build an `Engine` by validating,
then compiling, with normalization delegated to the module component
`module:src/engine.rs`. The metric line is support, not the point.

The outline is the gate. Metrics find candidates for a reviewer's attention,
but they do not decide. Both directions are valid:

| Outline | Metrics | Result |
|---|---|---|
| Readable | clean | pass |
| Readable | flag | pass; the metric is a hint |
| Unreadable | clean | fail; the outline is the finding |
| Unreadable | flag | fail; the metric is a hint |

An unreadable outline is not justified. Fix the code and re-review.

## Workflow

![Outline addon workflow: implement produces an optional advisory report with the outline; review produces outline verdicts and a PASS and checks every touched outline; archive checks the review record](outline-workflow.svg)

Source: [`outline-workflow.d2`](outline-workflow.d2). Regenerate with
`d2 docs/outline-workflow.d2 docs/outline-workflow.svg`.

The report is advisory. The review gate is blocking. The report never stores a
baseline; the gate only looks at the units the change touched.

## What is produced and checked

| Phase | Produced by the addon | Checked by the addon |
|---|---|---|
| Proposal / design | Nothing automatic. You may run the report as evidence. | Nothing. |
| Implement | An optional report with each unit's outline first, then the metric hints, as human text or JSON. | Nothing. The report never blocks a change. |
| Review | Per-unit `OUTLINE: PASS` or `FAIL`, `OUTLINE: AUTHOR` and `REVIEWER`, and `OUTLINE: PASS` when every outline passes. | Every printed outline has a verdict. A `PASS` has a responsibility and call-order explanation; a `FAIL` blocks. The reviewer differs from the author. |
| Archive | Nothing new. | The review record is complete. An outlined unit without a verdict blocks archive. |

The review record markers:

```text
OUTLINE: AUTHOR <id>
OUTLINE: REVIEWER <id>
OUTLINE: PASS <unit> | <responsibility; why the calls happen in that order>
OUTLINE: FAIL <unit> | <reason>
OUTLINE: PASS
```

`REVIEWER` must differ from `AUTHOR`. An empty `PASS` explanation fails. Unknown
or conflicting markers fail. A standalone `FAIL <reason>` still blocks.

## What is checked in the source code exactly

The report and the gate share the same analysis:

- **Scope.** Only production `.rs` and `.py` files under `src/` (or explicit
  `--path` roots). `tests/`, `benches/`, `examples/`, and `target/` are
  excluded; `#[cfg(test)]` items are excluded; `src/bin/**` is included.
- **Units.** A unit is a type (struct, enum, trait, union, or type alias) or a
  module. Its methods are checked with it. A unit with no methods has no
  outline and needs no outline verdict.
- **Outline.** Method declarations, signatures, order, and calls. Data
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
  `HINT` and never requires a decision for them.
- **Reviewer identity.** `OUTLINE: AUTHOR` and `OUTLINE: REVIEWER` are
  required when any outline verdict is present, and must differ.
- **Fail-closed guards.** No reachable change commit, a shallow clone, dirty
  production paths, a non-UTF8 or line-break path, a symlink, a submodule, an
  unreadable blob, or a changed `.rs` file the analyzer cannot analyze fails
  the gate.
- **Removed units.** A removed unit has no head outline, so it needs no outline
  verdict.
- **Accounting is unit-level.** Every outline verdict names exactly one unit.

## Language scope

The addon analyzes production Rust and Python.

- **Rust** is parsed in-process by `syn`.
- **Python** is parsed by a `python3` subprocess that runs a bundled
  standard-library `ast` extractor. The subprocess starts only when the report
  scope contains at least one `.py` file, so a Rust-only project never invokes
  `python3` and does not need Python installed. The extractor parses source
  text; it never imports or executes project code and needs no third-party
  package. Python 3.10 or newer is required for Python projects.

Discovery covers `.rs` and `.py` files under the production roots. Python test
modules (`test_*.py`, `*_test.py`) are excluded, and cache, virtualenv, build,
and dependency directories (`__pycache__`, `.venv`, `venv`, `.tox`,
`.mypy_cache`, `.pytest_cache`, `build`, `dist`, `node_modules`) are skipped.
The review gate applies the same scope: a changed Python test module is not
production scope.

| Input | Report | Review gate |
|---|---|---|
| Rust or Python file with an unsupported construct | Named in `unsupported` | Reported; not a failure by itself |
| Rust or Python file that cannot be parsed | Named as a parse failure | Fails closed if changed |
| Production source file the analyzer skips | Not analyzed | Fails closed if changed |
| Python file when `python3` is unavailable | Named `analyzer-unavailable`; the report continues | Fails closed if changed |
| Non-source file under production roots | Ignored | Ignored |
| Python test module (`test_*.py`, `*_test.py`) | Ignored | Not production scope |
| Source file under `tests/`, `benches/`, `examples/`, `target/`, or a Python cache/virtualenv directory | Excluded | Not production scope |

Python's dynamic nature limits the outline: a call through a dynamic attribute
or an inferred type is not resolved precisely. Calls are resolved by name:
same unit first, then the same file, then a unique project method; otherwise
the call is `ambiguous` or `external`. The report names these limits as
analyzer limitations. An unsupported construct is named in the report's
`unsupported` list; it is not automatically a gate failure. Supporting another
language needs another adapter behind the neutral `SourceAnalyzer` port
(ADR-074); no adapter pretends to understand another language's syntax.

## Expected outcome

- Every touched unit with methods is read through its outline by an independent
  reviewer.
- An unreadable outline blocks even when every metric is clean.
- A readable outline passes even when a metric flags the code; the metric stays
  visible as a hint.
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
  conditional compilation, braces inside string literals, Python dynamic
  calls, and Python type inference are known limits (risks R-105 and R-108);
  the report is a completeness-limited view, not a total-debt claim.
- **A verdict is complete and still wrong.** The gate checks presence and
  shape, not the judgment; independent review is a second opinion, not a proof.
- **Untouched code is assumed clean.** The gate sees only touched units. A green
  gate does not mean the repository has no unreadable outlines.

## Enable and disable

```sh
givn addons enable outline
givn outline report
givn outline report --json --out outline.json
givn check review --change <id>
givn addons disable outline
```

`givn init` never enables it. When disabled, the report refuses with the named
reason, the review gate does not run, and the outline section is omitted
from the served review guidance.

## References

- Report specification: [`givn/specs/assess-code-structure/outline-report.feature`](../givn/specs/assess-code-structure/outline-report.feature)
- Gate specification: [`givn/specs/verify-change/outline-review-gate.feature`](../givn/specs/verify-change/outline-review-gate.feature)
- Applied independent review (historical): [`givn/archive/apply-readability-independent-review/review.md`](../givn/archive/apply-readability-independent-review/review.md)
- Neutral analyzer port: [`docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md`](arc42/adr/ADR-074-neutral-source-analyzer-port.md)
- Known risks: [`docs/arc42/11-risks-and-technical-debt.md`](arc42/11-risks-and-technical-debt.md) (R-105 to R-109, D-24)
- Reviewer rules: [`assets/instructions/review.md`](../assets/instructions/review.md) (`Outline review gate`)
