---
description: Shared step-writing mechanics for Gherkin scenarios — RED/GREEN/REFACTOR with step reuse. Reference skill for givn-implement and givn-characterize.
---

Shared step-writing mechanics for Gherkin `.feature` scenarios. Reference
skill, not a standalone workflow — read by `/givn-implement` and
`/givn-characterize` subagents before writing step definitions.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

This command is a pointer: the reference is served by
`givn instructions steps`. To write steps as part of a change:
`/givn-implement` (normal changes) or `/givn-characterize` (reverse changes).

Run `givn instructions steps` and follow it — it carries the canonical
step-writing rules: definition locations, step reuse, the no-empty-body
rule, the canonical E2E policy, the capture mechanics, and the
RED→GREEN→REFACTOR loop.
