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
