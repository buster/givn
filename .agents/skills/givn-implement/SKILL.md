---
name: givn-implement
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time.
---

# givn-implement

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Implement change `<change-id>` one scenario at a time. The rules for this step are served by `givn instructions tasks --change <change-id>` — load the instruction and follow it. This file triggers the step and carries no rules of its own.

Commit each finished scenario with `givn commit`; a raw fossil check-in carries no attribution. The exact commit contract is served by `givn instructions tasks --change <change-id>`.

When you hand work to a fresh context, pass the obligation explicitly: for every user-facing question, follow the effective policy from `givn instructions dialog` — this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
