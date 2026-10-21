# Proposal: fix-outline-report-fidelity

## Release note

fix(outline): make call graphs, module outlines, and metrics faithful

## Use-Case Context

- Use-case ID: assess-code-structure
- Source topic: none (no ideation seed)
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — the report's human text and JSON are a
  user-visible CLI surface, and the existing use case already confirms these
  three review lenses.
- Owning capability: `outline-report`

## Problem / Opportunity

A developer ran `givn outline report` on two real projects and found six
defects that make the report look more certain and more complete than it is.

1. **A name-only guess is presented as a certain call.** A method call whose
   receiver type is unknown is attributed to the unique project method with
   that name. `current.is_empty()` becomes an unrelated type's `is_empty`,
   `Line::from(...)` becomes `Error::from`, and `config.providers.get(...)`
   becomes the provider registry's `get`. On givn itself, 269 `.is_empty()`
   calls and 93 `.get()` calls collapse onto two unrelated methods, and the
   mistaken edges are nested as if they were real same-unit calls.
2. **A finding states a comparison that was never tested.** A god type found
   because its cohesion is low reports `type-lcom4 2 > 2`. The same defect
   appears for a public-method interface and for a complexity-driven long
   method. The evidence contradicts the metric that produced the finding.
3. **A module outline omits the types the module declares.** A
   `multi-type-module` finding says the file declares ten types, but its
   outline lists only free functions, so the reader cannot see what the
   module is made of.
4. **The call graph cannot be audited.** JSON call objects carry only the
   callee name and a resolution label, with no target; human text shows the
   target owner but not how it was resolved. A reader cannot tell a certain
   same-unit edge from a cross-component edge or a name-only guess.
5. **Production-only metrics disagree with each other.** A file's code-line
   count includes inline test modules while unit code lines exclude them, and
   a unit's physical line count is the whole containing file rather than the
   unit's own spans.
6. **Signatures lose the type information that makes them useful.**
   `SetupWizard::from_config` prints `-> Result` instead of
   `-> Result<Self, Error>`; module paths, generic arguments, tuple, slice,
   array, pointer, and reference shapes are dropped.

The report is advisory, but advisory evidence must still be true. A false
certainty or a false comparison makes the outline unusable as review
evidence and undermines trust in every other label it prints.

## Proposed Solution

The report tells the reader exactly what the analysis knows, and no more.

1. **Call resolution follows the call's shape.** A call carries whether it is
   a free function or a method and what the receiver is: the unit itself, a
   named owner, or unknown. `same_file` and `project_other_file` are printed
   only when the resolved owner agrees with that evidence. An unknown
   receiver yields `name_only` (or `ambiguous` when several methods share the
   name); a qualifier that contradicts the only candidate yields `external`.
   A `name_only` edge keeps the unique candidate as its target for
   auditability but is rendered as a leaf, never nested as a same-unit call,
   and never counted as a cohesion edge.
2. **Evidence names only what was measured.** Evidence lists exactly the
   predicates that fired: the line-count threshold, LCOM4, TCC, public
   method count, trait kind, method code lines, or method complexity. No
   evidence line names a metric that did not satisfy the finding.
3. **Module outlines list their member types.** Every type a module file
   declares appears as a type item in that module's outline, alongside its
   free functions, in source order, for Rust and Python alike.
4. **The call graph is auditable.** JSON call objects name the resolved
   target. Human call tags begin with the resolution label, for example
   `-> [same_file, public, Engine] build`,
   `-> [project_other_file, public, module] normalize`, and
   `-> [name_only, private, TierReasoning] is_empty`. Ambiguous edges stay
   `-> [ambiguous] callee`.
5. **Production-only means production-only.** With a production-only scope,
   a file's code lines exclude inline test spans, and a unit's physical
   lines cover the unit's own spans rather than its containing files.
6. **Signatures keep their types.** Generic arguments, path qualification,
   references, tuples, slices, arrays, and pointers appear in the printed
   signature; closures and syntax the analysis cannot render remain
   explicitly approximate rather than silently truncated.

The report remains advisory: no action, no recommendation, no baseline, and
no change to review or archive behaviour.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if deviated from route) |
|---|---|---|---|
| `outline-report` | `EXTEND givn-workflow-enforcement` (use case `verify-change`) | `EXTEND outline-report` | The signal came from generic `givn check` and `givn spec` wording in the capability inventory, not from the change's behaviour. Every scenario here is report behaviour owned by `assess-code-structure`'s `outline-report` capability; the gate only consumes the same report renderer and gains no new rule. |

## Out of Scope

- A JSON schema version bump. The call target is an added field and the
  label/value changes are semantic corrections; no field is removed or
  renamed.
- Whole-program type inference or a compiler-backed resolver. Unknown
  receivers stay `name_only`; the report never guesses beyond its evidence.
- New languages, new metrics, new thresholds, or a change to the
  classification rule set or thresholds; corrected evidence may change an
  individual unit's classification.
- New review-gate rules. The gate reuses the same call tree, so its printed
  tags change with the renderer, but no verdict or blocking rule changes.
- Recommendations, actions, baselines, caches, or historical comparisons.
- Test-source inclusion, `#[cfg]` evaluation, and macro expansion remain as
  documented limitations.

## Open Questions

None. The six defects have one conservative resolution each, recorded in the
design's decision ledger (D1–D7 and D9; the proposed D8 is withdrawn); no
decision needs operator input before specs.
