# Proposal: ADR Qualification Gate

## Problem / Opportunity

The architecture decision register has accumulated many records that describe
ordinary design details, authoring rules, workflow conventions, test strategy,
coverage policy, or product planning rather than durable architecture
decisions. The current qualification guidance allows a record when a single
broad criterion can be argued, so an affected Arc42 chapter or a non-trivial
implementation choice is easily mistaken for an ADR requirement.

This makes the active architecture contract difficult to understand. It also
encourages one ADR for every implementation slice or later refinement instead
of preserving a small set of decisions whose rationale is needed to protect
system boundaries and long-lived contracts.

## Proposed Solution

The ADR guidance will distinguish an architecture decision from a normal design
decision and from an Arc42 documentation impact.

Qualification is a strict Boolean contract. A candidate qualifies only when all
mandatory dimensions pass: it records one real choice between materially
different alternatives, has architectural impact, has a durable consequence,
is correctly classified as needing an ADR rather than a lower-level artifact,
and includes a completed existing-ADR or duplicate check. The MADR checklist is
evidence for those dimensions, not an at-least-one escape hatch. `Must be
shared` is supporting evidence only and can never qualify a candidate by
itself.

A candidate that refines an existing ADR will amend that ADR; a candidate that
replaces its decision will supersede it. Neither creates a parallel ADR for the
same independent boundary. A failed candidate is routed once to the appropriate
Arc42 chapter, Gherkin specification, project documentation, or code.

The guidance will require the candidate to state its affected boundary or
contract, durable consequence, materially different alternatives, supplied
evidence, lower-level classification, and duplicate check. The structured
verdict will expose the five mandatory gate results, the supporting-only
`must_be_shared` result, and one routing value. It will repeat only evidence
provided by the author; it will not invent alternatives, evidence, impact, or
participants.

The existing register will be reviewed against the stricter gate with one ADR
per independent durable architecture boundary or trade-off. Refinements may be
amended into their governing ADR, but independent decisions remain separate.
Non-ADR records leave the ADR register and archive. Useful information is moved
once to its canonical Arc42 chapter, Gherkin specification, project
documentation, or code; redundant information is deleted.

The qualification and canonical-source policy is a default authoring policy,
not runtime enforcement. Project overrides may replace the effective guidance
through the normal asset cascade. LLM guidance and design review will use the
policy's explicit verdict and adversarial duplicate/classification check while
preserving the distinction between an affected Arc42 chapter and a required
new ADR.

## Out of Scope

- Changing the runtime behavior of the Givn CLI.
- Changing the meaning of existing Gherkin specifications.
- Retaining a non-ADR copy in the ADR archive after its useful information has
  been moved to a canonical source.
- Automatically deciding architectural significance from file names, scores,
  or keyword matches alone.
- Replacing the MADR format for records that remain valid ADRs.

## Resolved Decisions

- The gate is strict AND semantics across materially different alternatives,
  architectural impact, durable consequence, correct lower-level
  classification, and the existing-ADR/duplicate check. `Must be shared` is
  supporting evidence only.
- Canonical single source applies. Non-ADR material leaves both active and
  archived ADR indexes; it is relocated once or deleted as redundant.
- Consolidation is aggressive DRY only within one independent decision. An ADR
  is not merged with another independent boundary merely to reduce its count.
- Qualification and canonical-source policy are authoring guidance, not a new
  ADR and not a runtime enforcement mechanism. Project overrides may replace
  the default guidance.
- `ADR-063` is excluded from this migration. It remains proposed and is owned
  by the incomplete `semantic-reranker-budget` change.
