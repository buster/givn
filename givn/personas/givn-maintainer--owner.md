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
- 2026-11-13 — Decision: confirmed the Frame for goal, scope boundary,
  exclusions, and stakeholders. (User, Q2)
- 2026-11-13 — Need: Catopia (`benchmarks/catopia-v2`,
  `benchmarks/catopia-spring-v2`) plus Passuff evidence before `ise` becomes a
  default. (Frame, vision)
- 2026-11-13 — Need: Givn's own machinery rebuildable while authored
  `assets/` stays product content. (Frame, gap G7)
- 2026-11-13 — Need: the opt-in default leaves existing projects unchanged on
  upgrade, so a missing `ise:` key never enables a gate. (Frame, gap G1;
  open: Q6)
- 2026-11-13 — Decision: the topic keeps three personas; the bare-clone
  consumer lens moves here and `external-contributor--open-source-consumer`
  is dropped. (User, Q7)

## Topic stances

- 2026-09-14 — multi-scm-support: no third SCM, no VCS library dependency, no
  kiekste hosting work; attribution parity must not widen the design.
- 2026-09-23 — local-only-integrations: the default flips to local-only
  rules; constraint 2.7, quality requirement Q-57, risk R-33, the
  managed-upgrade scenarios, and the migration docs must be amended to match.
- 2026-09-23 — local-only-integrations: this repository currently versions
  its generated rules for self-hosting; keeping that under the new default is
  a pending decision, not a settled exception (Q9).
- 2026-10-13 — rules-through-instructions: demands one canonical asset per
  rule instead of up to three copies (skill template, command asset,
  instruction asset).
- 2026-10-13 — rules-through-instructions: demands that the specs, the
  guidance-integrity requirement (Q-97), and the managed-upgrade path stay
  truthful when generated guidance stops carrying the rules.
- 2026-10-13 — rules-through-instructions: needs the experiment to prove the
  directing form before it becomes a default.
- 2026-11-13 — ise-rebuildability: a published repository or a frozen
  replacement package must build, run, and verify through real external
  interfaces with `givn` absent from PATH, with no dependency on deleted
  internals or repository history.
- 2026-11-13 — ise-rebuildability: treats the frozen verifier as the
  contract; a suite that needs production code to start is not black-box.
- 2026-11-13 — ise-rebuildability: no second binary, network service, or
  language runtime in givn; `TC-2` and `TC-3` stand.
- 2026-11-13 — ise-rebuildability: instruction size is a measured risk; a rule
  addition that lowers scenario volume and coverage is a regression even when
  the hidden grade holds.
- 2026-11-13 — ise-rebuildability: no historical `catopia-v2` task prompt
  mutates in place; a rebuild-permitting follow-up is a versioned variant.
- 2026-11-13 — ise-rebuildability: a `givn ise` or `givn rebuild` subcommand,
  if one is ever adopted, must clear Event Storming and use cases before its
  CLI and filesystem contract is fixed. (open: Q3)
- 2026-11-13 — ise-rebuildability: `Q10` renamed the frozen input above
  `Specification` (contrasted with `Source code`); the same stance reads "a
  published repository or a frozen Specification must build, run, and verify
  with `givn` absent from PATH".
- 2026-11-13 — ise-rebuildability: `Q11` renamed the frozen external-interface
  suite `End-to-end (E2E) test`; the same stance reads "treats the frozen
  black-box E2E test suite as the contract; a suite that needs production code
  to start is not black-box".
- 2026-11-14 — ise-rebuildability: `Q6` closed with the `ise` ->
  `architecture_principles` prerequisite; the opt-in default still leaves
  existing projects unchanged, and enabling `ise` requires the prerequisite
  first.
- 2026-11-14 — ise-rebuildability: `Q3` closed with `givn ise prepare` and
  `givn ise verify`; the CLI and filesystem contract is fixed by those two
  commands.
