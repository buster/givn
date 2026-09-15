# Persona: Technical role (authoring agent / automation caller)

## Role

The non-human consumer of givn. Two merged segments: the authoring agent that
runs the lifecycle and commits once per scenario, and the automation caller
(kiekste preview host, CI) that drives givn through its machine contract. Both
consume givn's commands and outputs without human interpretation; that shared
property is why they are one persona.

## Interests

- T1: Deterministic commands and outputs. No guessing, no prompts, no
  interactive fallbacks.
- T2: Generated skills that state the truth for the active SCM. A Fossil
  checkout must never be told to run `git commit`.
- T3: Revision identifiers in a stable, opaque format for `tasks.md` and the
  verification receipt.
- T4: A machine contract (exit codes, ready line, JSON, receipt) that does not
  change shape with the SCM.

## Goals

- Run RED/GREEN/REFACTOR with exactly one revision per scenario and record it.
- Follow the generated guidance without discovering SCM-specific surprises
  mid-flow.
- Drive archive, check, and run from automation and reconcile the result
  deterministically.

## Pain points

- Skills that name Git commands in a Fossil checkout.
- Fossil requiring `fossil add` before a commit of a new file; a commit that
  fails on an unknown path.
- Attribution instructions that cannot be satisfied on Fossil.
- Parsing SCM-specific status output in automation.
- Any capability that silently no-ops on one SCM while reporting success.

## Success criterion

Every documented command works verbatim in both SCMs, and automation reads one
stable contract regardless of the active SCM.

## Vocabulary

- "commit", "hash", "revision", "clean tree", "hook", "gate", "exit code",
  "receipt".
- Attribution context: LLM, reasoning mode, harness, Givn version.
- Automation: "machine contract", "ready line", "fail-closed".

## Decision, need, and demand log

| Date | Entry | Source |
|---|---|---|
| 2026-09-14 | Need: one revision per scenario remains the rule in both SCMs. | Frame |
| 2026-09-14 | Need: generated skills branch on the active SCM instead of naming Git unconditionally. | Frame |
| 2026-09-14 | Need: the receipt and integrity formats accept both SCM hash shapes. | Frame, gap G8 |
| 2026-09-14 | Open demand: a commit path that stamps attribution on Fossil (Q2). | Attribution probe |
