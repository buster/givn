# Persona: Givn maintainer (owner) — topic record

Reuse: givn-maintainer--owner, base 7bf38024; this file adds record entries only

## Running record

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
