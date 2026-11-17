# Persona: Technical role (authoring agent / automation caller)

## Stable core

- Role: The non-human consumer of givn. Two merged segments — the authoring
  agent that runs the lifecycle and commits once per scenario, and the
  automation caller (preview host, CI) that drives givn through its machine
  contract — share the authority boundary of driving givn non-interactively
  and the evaluation capability of reading its machine contract.
- Authority boundary: May run every lifecycle command, commit once per
  scenario, and drive archive, check, and run from automation; may not supply
  human judgment or resolve ambiguous output.
- Evaluation capability: Can parse exit codes, the ready line, JSON, and the
  verification receipt, and can verify one revision per scenario; cannot
  judge prose, domain intent, or visual quality.
- Own vocabulary: "machine contract", "ready line", "fail-closed", "receipt",
  "exit code", "revision". Refuses guidance that names Git unconditionally in
  a Fossil checkout.
- Success criterion: Every documented command works verbatim in both SCMs,
  and automation reads one stable contract regardless of the active SCM.

## Running record

- 2026-09-14 — Need: one revision per scenario remains the rule in both SCMs.
  (Frame)
- 2026-09-14 — Need: generated skills branch on the active SCM instead of
  naming Git unconditionally. (Frame)
- 2026-09-14 — Need: the receipt and integrity formats accept both SCM hash
  shapes. (Frame, gap G8)
- 2026-09-14 — Demand: a commit path that stamps attribution on Fossil (Q2).
  (Attribution probe; answered by `givn commit`.)
- 2026-11-13 — Need: instruction growth must not repeat the `v0.9.15`
  collapse (`25/25` hidden grade kept, scenarios `130` -> `76`), so per-language
  runner recipes load on demand. (Frame, gap G2)
- 2026-11-13 — Need: deterministic `givn check` failures when `@e2e` imports
  production code or the verifier is candidate-editable. (Frame, gaps G4/G5)
- 2026-11-13 — Need: a stated boundary between normal feature co-evolution and
  frozen clean-room reconstruction. (Frame, gap G6)

## Topic stances

- 2026-09-14 — multi-scm-support: the machine contract must not change shape
  with the SCM; SCM-specific mechanics stay inside givn.
- 2026-09-23 — local-only-integrations: on the developer's machine it reads
  the committed `AGENTS.md` fence when present and discovers the local
  skills; on a published clone it receives no Givn workflow and must work
  without one.
- 2026-09-23 — local-only-integrations: a managed revision never carries a
  Givn rule or `AGENTS.md`; the stage-all path excludes both.
- 2026-10-13 — rules-through-instructions: needs each rule discoverable at
  the moment of the step, so the generated file's direction to
  `givn instructions` must be sufficient to load the content
  deterministically.
- 2026-10-13 — rules-through-instructions: needs the machine-readable
  contract (`--json`) to carry the same effective content as the
  human-readable output, extensions included.
- 2026-10-13 — rules-through-instructions: a committed generated file that
  only directs the agent to `givn instructions` carries no content where
  Givn is not installed — a deliberate difference from today's
  self-contained file.
- 2026-11-13 — ise-rebuildability: reads a runner or driver recipe when the
  stack needs it instead of carrying every stack in the step instructions.
- 2026-11-13 — ise-rebuildability: the replacement package must be
  enumerable; an undocumented schema, CLI flag, exit code, or viewport budget
  is a blocked rebuild, not a judgment call.
- 2026-11-13 — ise-rebuildability: every behavioral difference found during a
  reconstruction trial is recorded as verifier defect, implementation defect,
  specification defect, or acceptable undocumented change; never resolved as
  silent drift.
- 2026-11-13 — ise-rebuildability: `Q10` renamed the artifact above
  `Specification`; the same demand reads "the Specification must be
  enumerable", and an undocumented schema, CLI flag, exit code, or viewport
  budget stays a blocked rebuild.
- 2026-11-13 — ise-rebuildability: `Q11` renamed the frozen external-interface
  suite `End-to-end (E2E) test`, with `black-box` as the no-shared-code
  qualifier; the same demands read "deterministic `givn check` failures when
  `@e2e` imports production code or the black-box E2E tests are
  candidate-editable" and "every behavioral difference is recorded as an E2E
  test defect, implementation defect, specification defect, or acceptable
  undocumented change".
- 2026-11-14 — ise-rebuildability: `Q13` renamed the bounded clean-room
  execution `Ise Test`; the same demand reads "every behavioral difference
  found in an Ise Test is recorded as an E2E test defect, implementation
  defect, specification defect, or acceptable undocumented change; never
  resolved as silent drift." (open: Q14)
- 2026-11-14 — ise-rebuildability: `Q15` added no term, so the earlier demand
  reads "the stack instruction loads on demand when the stack needs it
  instead of every stack sitting in the step instruction"; the id convention
  (`steps` -> `steps-rust`, `dialog` -> `dialog-examples`) is what the core
  instruction names.
- 2026-11-14 — ise-rebuildability: `Q14` closed with `Spec gap`, the
  four-outcome classification record; the earlier demand reads "every
  behavioral difference found in an Ise Test is recorded as a `Spec gap` with
  one of `E2E test defect`, `implementation defect`, `specification defect`,
  or `acceptable undocumented change`; never resolved as silent drift."
