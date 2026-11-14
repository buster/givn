# Persona: Technical role (authoring agent / automation caller) — topic record

Reuse: agent--technical-role, base 7bf38024; this file adds record entries only

## Running record

- 2026-11-13 — Need: instruction growth must not repeat the `v0.9.15`
  collapse (`25/25` hidden grade kept, scenarios `130` -> `76`), so per-language
  runner recipes load on demand. (Frame, gap G2)
- 2026-11-13 — Need: deterministic `givn check` failures when `@e2e` imports
  production code or the verifier is candidate-editable. (Frame, gaps G4/G5)
- 2026-11-13 — Need: a stated boundary between normal feature co-evolution and
  frozen clean-room reconstruction. (Frame, gap G6)

## Topic stances

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
