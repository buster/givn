---
name: givn-steps
description: Shared step-writing mechanics for Gherkin scenarios — the RED/GREEN/REFACTOR loop with step reuse. Referenced by givn-implement and givn-characterize.
---

# givn-steps

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Shared step-writing mechanics for Gherkin `.feature` scenarios. Reference
skill, not a standalone workflow — read by `/givn-implement` and
`/givn-characterize` subagents before writing step definitions.

## Context

- Test runner: `./run-tests.sh`
- Features path: `GIVN_FEATURES=givn/specs`
- Change: `givn/changes/<change-id>/`

## Before starting

1. Run `givn instructions steps` and follow it — the step-writing rules
   (definition locations, step reuse, the no-empty-body rule, the canonical
   E2E policy, the capture mechanics, and the RED→GREEN→REFACTOR loop) are
   served there and nowhere else.
2. Run `givn instructions --change <change-id>` — this tells you exactly what
   to do next, derived from the project's current filesystem state. Do not
   guess, search files, or explore to determine next steps.
3. Run `givn status --change <change-id>` — this shows the full artifact
   checklist, task progress, and any pending task descriptions. Check state
   before acting.
4. The task you are about to implement: read its full entry in `tasks.md`,
   including what step definitions already exist, the commit status of previous
   tasks, and the Design constraints block. An unchecked box means the scenario
   is not done — tick it off immediately after completing each scenario, with
   evidence and commit hash filled in. Never batch-check boxes later.
