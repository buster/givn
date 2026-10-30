# Proposal: strengthen-outline-guidance

## Release note

Outline guidance now appears only in projects that enable its addon, routes implementation to the shared step mechanics, and treats changed code that cannot explain itself from its outline as a review finding.

## Use-Case Context

- Use-case ID: obtain-effective-guidance — supporting owners: assess-code-structure, verify-change.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: agent--technical-role, developer--primary-user — their existing authority and goals apply; no Persona is invented or reconfirmed.
- Source: the operator's completed audit of the current outline guidance, recorded in the change plan; the portable decisions are repeated here and in `design.md`. No execution depends on an agent-local scratch plan.

## Review Context

- Review kind: self-review
- Reason: served-guidance wording and its regression scenarios; no public command, configuration, or data contract changes.

## Problem / Opportunity

Three defects make the outline guidance weaker than its own contract:

1. The forward implement path does not load the shared step mechanics. The REFACTOR outline obligation lives in `givn instructions steps`, but `givn instructions tasks` never names that instruction, so an agent that follows the implement pass can complete REFACTOR without ever requesting the steps guidance. The generated `givn-implement` skill routes only to the tasks instruction.
2. Outline instructions render for every project. The REFACTOR outline paragraph in the steps guidance and the review-context paragraph in the review guidance mention the outline unconditionally, and the review template shipped to every project carries a retired-gate phrase. A project that disabled the addon still receives outline obligations.
3. The guidance treats the outline as a metric aid rather than a readability contract. It asks architecture questions but never requires a reviewer or implementer to reconstruct a changed unit's responsibility and ordered call flow from the outline alone. It never states that names and signatures carry the domain operation, and it never states that an unreadable changed unit is a review finding that rejects PASS. As a result, a clean metric line can stand in for understanding, and the serviced text hardcodes a language set that will drift as analyzers are added.

User impact: an agent can skip the REFACTOR mechanics entirely, a project that opted out still receives outline obligations, and a reviewer can sign off on code that no reader can explain from its declarations.

## Proposed Solution

- When the outline addon is enabled, the steps guidance carries the full REFACTOR outline obligation; when it is disabled, no served instruction and no generated skill carries outline guidance. The review template never names an outline verdict.
- The REFACTOR outline block becomes a readability contract: state the unit's responsibility in one sentence and its ordered call flow from declarations, signatures, and call order alone; names, parameter names and types, return types, and called type names must state the domain operation in the project's vocabulary; generic machinery names are findings; metrics never excuse an unreadable outline; an unreadable changed unit is fixed in REFACTOR or recorded as a substantive review finding.
- The review advisory block carries the same comprehension test and naming test. The report itself stays advisory: no `OUTLINE:` marker, no metric waiver, no second reviewer context, and no mechanical gate. A substantive readability finding about the changed code is an ordinary review finding and can reject PASS.
- The tasks instruction names `givn instructions steps` as the owner of the step mechanics, and the generated `givn-implement` skill names both instructions. The tasks instruction keeps owning batch sequencing, verification, and commit evidence.
- Served guidance says `supported` and `unsupported`, never a concrete language list. The report's own capability disclosure and the factual analyzer reference may state the current language set as data.
- The outline guide, arc42 chapter 08, and the README addon description describe the readability contract, the serving rule, and the data limit (fields and enum variants are absent).

## Capability Routing

`givn spec route --change strengthen-outline-guidance` reports `EXTEND givn-workflow-enforcement` (signal). That is the verification owner, not an instruction to concentrate unrelated guidance behavior there.

| Proposed capability | Route's recommendation | Decision | Rationale (if deviated from route) |
|---|---|---|---|
| serve-extended-rules | EXTEND givn-workflow-enforcement | EXTEND serve-extended-rules | The REFACTOR and review rules are served guidance owned by obtain-effective-guidance. |
| match-guidance-to-enabled-addons | EXTEND givn-workflow-enforcement | EXTEND match-guidance-to-enabled-addons | Addon-conditional serving is owned by the existing support fragment. |
| review-architecture-documentation | EXTEND givn-workflow-enforcement | EXTEND review-architecture-documentation | Review readability findings and the review template are owned by the existing review fragment. |
| refresh-generated-guidance | EXTEND givn-workflow-enforcement | EXTEND refresh-generated-guidance | The generated `givn-implement` routing is owned by the existing synchronization fragment. |

## Out of Scope

- Report schema, metric definitions, classifications, and analyzer coverage. No `src/` change is expected.
- A new `OUTLINE:` marker, a metric waiver, a second reviewer context, a mechanical gate, or restoration of the retired blocking outline gate.
- Rewriting the once-per-batch cadence or the existing advisory-report semantics.
- New language adapters or a change to which languages the analyzer currently supports.
- Project instruction overrides: a project's own replacement text remains the project's responsibility.

## Open Questions

No unresolved decisions. The operator's plan fixes the conditional boundaries, the preserved sentence set, the new readability meaning, the routing, and the documentation scope. Exact wording is a design concern and is recorded in `design.md`.
