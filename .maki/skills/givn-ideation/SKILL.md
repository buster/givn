---
name: givn-ideation
description: Work a vague idea into change-ready use cases through a structured, state-persisting domain exploration
---

# givn-ideation

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Work the vague idea at hand into change-ready use cases. The rules for this step are served by `givn instructions ideation` — load the instruction and follow it. This file triggers the step and carries no rules of its own.

When you hand work to a fresh context, pass the obligation explicitly: for every user-facing question, follow the effective policy from `givn instructions dialog` — this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
