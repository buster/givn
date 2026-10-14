---
name: givn-design-review
description: Stress-test the plan before tasks are written — grill the design question by question, harden the artifacts, then sign off.
---

# givn-design-review

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Stress-test the complete plan for change `<change-id>` before tasks are written. The rules for this step are served by `givn instructions design-review --change <change-id>` — load the instruction and follow it. This file triggers the step and carries no rules of its own.

When you hand work to a fresh context, pass the obligation explicitly: for every user-facing question, follow the effective policy from `givn instructions dialog` — this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
