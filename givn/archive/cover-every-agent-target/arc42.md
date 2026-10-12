# arc42 Documentation Update: cover-every-agent-target

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute change; the consumer property Q-97 already names it. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; the suite reads the existing production list. | |
| 3 | 03 context-and-scope | No | No external system or user-facing surface changes; generated guidance text is untouched. | |
| 4 | 04 solution-strategy | No | No strategy or technology direction change. | |
| 5 | 05 building-block-view | No | No production building block: the change is test support under `tests/`. | |
| 6 | 06 runtime-view | No | No runtime flow or sequence change. | |
| 7 | 07 deployment-view | No | No deployment or topology change. | |
| 8 | 08 crosscutting-concepts | No | Section 8.7's resolution rule text is unchanged; target derivation and per-target coverage are properties of the check, hosted by 10.3. | |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate; the verdict below routes to 10.3. | |
| 10 | 10 quality-requirements | Yes | Q-97 owns the guidance-integrity acceptance statement; the check now also fails a configured target with no synchronized guidance, and the 10.3 signal states the production-derived coverage. | Q-97's expected response gains the no-guidance clause; the 10.3 bullet states that the check covers every production agent target and requires synchronized guidance from each configured one. |
| 11 | 11 risks-and-technical-debt | Yes | The per-target coverage assertion is a new bounded check with a residual limit. | D-21 records that coverage is "at least one installed source per configured target", not completeness. |
| 12 | 12 glossary | No | `Agent target` already defines all four destinations; no new term. | |

## Notes

The suite consumes two production surfaces: collection follows the project's
configured targets (validated against `KNOWN_AGENT_TARGETS`), while
recognition and the coverage guard follow `AgentTarget::ALL`. Divergence is
caught loudly — a target the fixture writes but collection drops fails the
coverage step — and the recognition predicate is itself asserted for every
production target, so a stale literal cannot silently narrow extraction.

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. A change may affect chapter 09 without qualifying for an
ADR, and a qualifying ADR must still be recorded as its own file and linked
from the chapter-09 register. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/10-quality-requirements.md section 10.3

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: NOT_SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/10-quality-requirements.md
target_adr: none
replacement_adr: none
```

Alternatives were weighed (static copies versus production-derived
recognition; acceptance-suite check versus a `givn check` command), which
satisfies the alternatives dimension. No component, ownership, dependency, or
compatibility boundary moves — the change is test support — so
architectural_impact fails. Reversal is test edits with no migration, so
durable_consequence fails. The complete rationale belongs to the capability
spec and design, hosted durably by the verification signal in 10.3, so
lower_level_artifact fails. `existing_adr_check` searched the register:
ADR-013 owns the agent-integration destination boundary and is not amended
(no production change); ADR-015 is untouched; the predecessor
`validate-generated-references` routed a comparable check decision as
`CANONICAL_ARTIFACT`, not an ADR.

Falsification tests applied after the mandatory dimensions: single-change
reversibility holds — the change reverses within one change — which by itself
fails the gate; no-multi-change-constraint does not hold, because the fixtures
and the check constrain every future agent-target addition; product-behavior
exclusion does not hold, because this is verification tooling, not
use-case-ownable workflow behavior. The outcome is unchanged.

---

## Status

STATUS: DONE
