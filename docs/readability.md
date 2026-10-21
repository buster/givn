# Readability addon

**Experimental, feature-gated, default off.** The `readability` addon is an
opt-in experiment: it reports structural debt in production Rust and, when
enabled, blocks a review until every touched finding is fixed or independently
justified. It is not a product guarantee; the schema, thresholds, classifications,
and gate rules may change.

## What it does

Two surfaces, both advisory to the rest of the workflow until the review gate
runs:

1. **Report** — `givn readability report` scans the project-root `src/` tree
   (or explicit `--path` roots), parses it through the neutral `SourceAnalyzer`
   port, and prints one classification with evidence per unit: god type,
   interface, data-only, cohesive-large-type, multi-type module, module,
   method-length, or within-limits. The JSON form is schema version 2 and
   carries findings only — no action, recommendation, or stored baseline.
2. **Review gate** — when the addon is enabled, `givn check review --change
   <id>` runs after the verification receipt. It resolves the change base as the
   parent of the first reachable commit that touched
   `givn/changes/<id>/`, analyzes the base and the committed `HEAD`, and
   considers only the units the change touched. Dirty production paths fail
   closed before analysis.

## Where it applies

| Phase | Effect when enabled |
|---|---|
| Proposal / design | None automatic. The report can supply evidence for a structural problem or an architecture decision. |
| Implementation | Run the report before and after a refactor; it is a read-only snapshot and does not gate the change. |
| Review | Every touched unit that is still a finding needs `READABILITY: FIXED <unit>` or `READABILITY: JUSTIFIED <unit>`. When any finding exists, the review record also needs `READABILITY: PASS`; `FAIL <reason>` blocks. |
| Archive | The review record is part of the archive gate, so an unresolved finding blocks archive. |

A `JUSTIFIED` decision needs a rationale at
`givn/changes/<id>/readability/rationales/<unit-slug>.md`. The rationale names
the unit and classification, cites evidence, states the unit's responsibility,
names a rejected alternative, records the owner, and records `Author:`,
`Reviewed-by:`, and `Review verdict: accept`. The reviewer must be a
fresh-context reviewer that did not write the rationale. The gate checks the
mechanical floor; it does not prove the judgment is right.

## Expected outcome

- Structural debt in touched units becomes visible and is either fixed or
  explicitly justified.
- An independent reviewer checks the justification instead of accepting
  self-attestation.
- Refactors stay behavior-preserving; the report's evidence, not a score,
  drives the decision.
- The report remains useful for untouched code without blocking unrelated
  changes.

Success looks like fewer findings over time, rationales that survive independent
review, and fixes that change structure rather than only the measurement.

## What may hint at a problem

- **Justify rate stays high.** A growing set of `JUSTIFIED` units means the
  gate is recording debt, not reducing it.
- **Rationales reject an alternative that a later change implements.** The
  archived rationales from the earlier gate change did exactly this: one said
  splitting the adapter would weaken ADR-074, and the follow-up split it inside
  the adapter directory. Old rationales are historical evidence, not active
  decisions; when they read as current, the process is confusing.
- **The reviewer accepts the claim without checking it.** A rationale that
  restates "the current shape is right" without citing the base/head
  measurement or naming every responsibility of the unit is not evidence.
- **The measurement improves while the design does not.** Hiding literals,
  suppressing a clippy lint, turning a long method into a long parameter list,
  or lowering a count without fixing the cause is gaming the gate.
- **The same unit recurs across changes.** Repeated findings on one unit mean
  the rationale is papering over a shape that should change.
- **Findings cluster in the analyzer or gate itself.** The experiment is
  self-referential; its own machinery is a large Rust surface. Treat that as
  evidence about the rules, not only about the code.
- **The addon stays disabled or the report is ignored.** An opt-in gate that
  nobody enables or acts on has no effect.
- **False positives or negatives dominate.** Item-position macros, non-UTF8
  files, `#[cfg]` branches, and braces inside string literals are known limits
  (risks R-105 and R-108); the report is a completeness-limited view, not a
  total-debt claim.
- **Cross-run comparisons are treated as trends.** No baseline is stored and the
  rules version can change; comparing two runs can compare two different rule
  sets.
- **Untouched debt is assumed absent.** The gate sees only touched units. A
  green gate does not mean the repository is clean.

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
- Applied independent review: [`givn/archive/apply-readability-independent-review/review.md`](../givn/archive/apply-readability-independent-review/review.md)
- Neutral analyzer port: [`docs/arc42/adr/ADR-074-neutral-source-analyzer-port.md`](arc42/adr/ADR-074-neutral-source-analyzer-port.md)
- Known risks: [`docs/arc42/11-risks-and-technical-debt.md`](arc42/11-risks-and-technical-debt.md) (R-105 to R-109, D-24)
- Reviewer rules: [`assets/instructions/review.md`](../assets/instructions/review.md) (`Readability review gate`)
