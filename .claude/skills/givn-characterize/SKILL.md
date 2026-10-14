---
name: givn-characterize
description: Execute the characterization loop for a reverse-engineering change — RED/GREEN/REFACTOR with step reuse, testability seams, reverse review.
---

# givn-characterize

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Characterize the existing capability with a RED/GREEN/REFACTOR loop. The rules for this step are served by `givn instructions characterize` — load the instruction and follow it. This file triggers the step and carries no rules of its own.

When the characterization is complete, request the reverse review with `givn instructions review-reverse`.

When you hand work to a fresh context, pass the obligation explicitly: for every user-facing question, follow the effective policy from `givn instructions dialog` — this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
