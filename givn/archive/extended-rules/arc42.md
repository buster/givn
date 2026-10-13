# arc42 Documentation Update: extended-rules

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or requirement changes: instructions are still emitted in text and JSON through the unchanged cascade (FR-7, FR-11); the change alters what the delivered content carries. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; the existing constraints (visible `givn/`, no configurable workflow, project-owned overrides) already bound the solution. | |
| 3 | 03 context-and-scope | No | No new external entity or interface: the extension is project-owned configuration read by the existing CLI; generated guidance and `AGENTS.md` keep their existing contracts. | |
| 4 | 04 solution-strategy | No | No strategy or technology direction change: one binary, the embedded-assets-plus-cascade delivery, and the project-overrides-first rule all stay. | |
| 5 | 05 building-block-view | Yes | One small production module is added and two responsibilities are amended. | 5.3 gains `config::extensions` (load, validate/warn, render the marked section); `cli::instructions` renders addon conditionals and appends the extension; `cli::skills` refreshes the project-start block with resident rules rendered from the canonical tasks instruction. |
| 6 | 06 runtime-view | Yes | The `givn instructions` flow gains two steps. | 6.3 records the request-time conditional rendering and the extension append, and names the step and reverse-mode ids that resolve through the non-artifact fallback. |
| 7 | 07 deployment-view | No | No deployment or topology change; nothing new is distributed beyond the same single binary and its embedded assets. | |
| 8 | 08 crosscutting-concepts | Yes | Sections 8.3a (normative authoring guidance) and 8.7 (skill and rule generation) own the guidance-delivery contract this change realizes; 8.3b owns the JSON contract that now carries the rendered-and-extended text. | 8.3a: generated files carry a direction instead of rule content; the extended rule is the built-in rules plus the project's extension; rules are extended, never overridden. 8.7: the direction-only skill and command form, the instruction-id resolution rule, the reconciliation behavior, the `extensions:` source with warn-and-drop, the replacement interplay, and the resident rules rendered into the project-start block. 8.3b: the `instruction` field carries the rendered-and-extended text. |
| 9 | 09 architecture-decisions | No | No qualified ADR candidate; the verdict below routes the decision to its canonical artifact (8.3a/8.7). | |
| 10 | 10 quality-requirements | Yes | Q-97 (guidance integrity) gains a second graded property, and its verification signal gains the companion check. | Q-97 requires every instruction id a direction names to resolve through the CLI (an artifact id may refuse only for a missing change context); 10.3 names the companion deterministic check beside the file-reference check. |
| 11 | 11 risks-and-technical-debt | Yes | Two durable boundaries of the new mechanism are recorded. | R-79: an extension can textually contradict a built-in rule; both appear and no semantic conflict detection exists. R-80: a committed direction-only file carries no rules where Givn is not installed; under `local-only` the case cannot arise, under `versioned` it is accepted and diagnosable. |
| 12 | 12 glossary | No | The confirmed term the change relies on — `Extended rule` — is already recorded (added with the ideation handoff); the change coins no new confirmed term. | |

## Notes

The serving contract, the extension source, and the reserved always-on surface
are architecture documentation owned by 8.3a and 8.7; the change's design
records the realization, and the delta scenarios prove the observable behavior
on the real CLI. Chapter 6.3 is the runtime entry point an agent reads before
pulling guidance, so it names the two added serve steps.

Reconciliation note: `docs/arc42/12-glossary.md` carries working-tree edits
(the `Extended rule` row and the `Givn rule` disambiguation) that predate this
change — they are ideation-handoff material, not this change's impact. Chapter
12's `No` is therefore correct, and the table's file-change accounting matches
the actual diff: chapters 05, 06, 08, 10, and 11 are this change's
documentation updates, and the chapter-12 diff is handoff material this change
relies on rather than produces.

This impact marker does not replace the ADR Qualification Checklist or the
individual ADR file. A change may affect chapter 09 without qualifying for an
ADR, and a qualifying ADR must still be recorded as its own file and linked
from the chapter-09 register. Record the result once in the change assessment:

```text
ADR action: CANONICAL_ARTIFACT
ADR target: docs/arc42/08-crosscutting-concepts.md sections 8.3a and 8.7

qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/08-crosscutting-concepts.md
target_adr: none
replacement_adr: none
```

D2 offered materially different options (a config map, an extension file per
instruction, or both), and D1/D4 weighed resident-rule generation and the
replacement interplay against alternatives, which satisfies the alternatives
dimension. No component, authority, dependency, or deployment boundary moves:
the instruction-policy authority already sits with the resolved instruction
(8.3a doctrine), the generated-file destination boundary belongs to ADR-013,
the facet cascade to ADR-005, and the JSON contract to ADR-048 — none of them
changes here. The extension is a new feature behavior of the existing CLI,
ownable by use cases plus Gherkin, so architectural_impact fails and the
complete rationale belongs to the canonical artifact. Reversal is a set of
asset rewrites plus one config parse and one render call, so
durable_consequence fails. `existing_adr_check` searched the active and
archived registers: ADR-013 owns generated-guidance destinations and
`AGENTS.md` ownership (unchanged), ADR-005 the project-overrides-first cascade
(unchanged), ADR-048 the machine-readable instruction contract (shape
unchanged); the "what the guidance carries" rule is owned by 8.3a/8.7, so no
amendment or supersession applies.

Falsification tests applied after the mandatory dimensions: single-change
reversibility holds — the delivery rewrite, the extension, and the resident
rules reverse within one change and touch one binary — which by itself fails
the gate; no-multi-change-constraint does not hold, because the direction form
constrains every future guidance asset; product-behavior exclusion does not
hold, because the extension is observable workflow-supporting behavior. The
outcome is unchanged: two mandatory dimensions already FAIL.

---

## Status

STATUS: DONE
