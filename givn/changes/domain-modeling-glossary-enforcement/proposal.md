# Proposal: domain-modeling-glossary-enforcement

## Problem / Opportunity

The domain-modeling skill currently maintains a throwaway `domain.md` scoped to
each change. That artifact is never merged or gated, so the project accumulates
no durable vocabulary: every change silently re-defines terms like "list",
"item", or "trip" independently, and nothing in the workflow checks that specs,
design, and code stay consistent with a single shared domain language.

The skill is also only loaded at the specs and design steps. By the time the
design is reviewed and the change is reviewed, nobody verifies that the agreed
terms were actually used, used consistently everywhere, and recorded somewhere
durable. The ubiquitous language exists only for the duration of a change and
vanishes at archive.

Finally, the project already keeps a durable glossary — the arc42 glossary
chapter (`docs/arc42/12-glossary.md`) — that is maintained project-wide and
gated by the existing arc42 conformance check. The domain vocabulary belongs
there, not in a per-change file that dies on archive.

## Proposed Solution

- The domain-modeling skill maintains the project's arc42 glossary: every
  domain term of a change is recorded there as `term | definition | spec
  wording | code symbol | anti-terms`, and the project's canonical vocabulary
  therefore grows durably across changes.
- The domain-modeling skill is active in practically every step of a change.
  Its trigger covers the whole change lifecycle — propose, spec, design,
  design-review, implement, review — and the next-step guidance names it
  across the workflow, not only at the specs and design steps.
- The design-review skill checks, before signing off, that the ubiquitous
  language is followed: terms are used, used consistently everywhere in the
  design, and described in the arc42 glossary.
- The review skill performs the same check on the finished change, in addition
  to its existing audience/fabrication audit: the specs, design, and code use
  the agreed terms consistently, and the glossary was updated.
- Domain modeling requires the arc42 glossary. The domain-modeling skill is
  only pointed to when the arc42 docs exist; a project that enables domain
  modeling without the glossary is told to enable arc42 first. Enforcement is
  human — carried by the skill discipline and the design-review/review
  checklists, never by term-matching scanners.

## Out of Scope

- No programmatic scanning of spec or design text for term membership.
  Vocabulary is validated by people. The automated surface changes only where
  a config fact is checkable (whether the arc42 glossary exists when domain
  modeling is on).
- No new artifact types, no new merge paths. The glossary remains an arc42
  chapter with its existing conformance gate.
- The domain-modeling skill's per-change modelling discipline (challenging
  generic terms, naming aggregates, probing edge cases) is kept as-is; only
  its output target and load breadth change.

## Open Questions

- None. The coupling decision (domain modeling requires arc42) is settled.
