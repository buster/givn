# Persona: Givn maintainer (owner)

## Stable core

- Role: The person maintaining givn itself. Two merged segments — the
  seam-keeper who owns the portable boundary and the truth-keeper who owns
  the permanent specifications — share the authority boundary of owning the
  architecture record and the evaluation capability of checking every claim
  against the implementation.
- Authority boundary: Owns the portable seam, the permanent specifications,
  the constraint and ADR register, the domain vocabulary, and the test
  harness; may reject scope that widens the product boundary.
- Evaluation capability: Can judge whether a claim matches the implemented
  boundary, whether a decision is durable, and whether vocabulary is
  consistent across specs, design, and code.
- Own vocabulary: "seam", "backend", "port", "adapter", "candidate revision",
  "tested revision", "receipt", "constraint", "ADR", "fixture", "delta".
- Success criterion: One interface, two backends, no duplicated SCM logic;
  all existing Git scenarios still pass; Fossil scenarios pass where Fossil
  is available; no Git-only claim remains in specs, ADRs, or the glossary.

## Running record

- 2026-09-14 — Decision: Frame confirmed (stakeholders, scope boundary,
  detection rule). (User)
- 2026-09-14 — Decision: three personas; audience dropped. (User)
- 2026-09-14 — Need: every decision stays visible to the user; no silent
  capture. (User)
- 2026-09-14 — Need: domain vocabulary decision recorded before specs are
  written (Q10). (Frame, gaps G3/G8)

## Topic stances

- 2026-09-14 — multi-scm-support: no third SCM, no VCS library dependency, no
  kiekste hosting work; attribution parity must not widen the design.
