---
description: Execute the characterization loop for a reverse-engineering change — RED/GREEN/REFACTOR with step reuse, testability seams, reverse review
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-characterize`.

Characterize the existing capability with a RED/GREEN/REFACTOR loop. Load the step's rules with `givn instructions characterize` and follow them; this file only triggers the step. When the characterization is complete, request the reverse review with `givn instructions review-reverse`.

When you delegate to a fresh context, pass this block with the change id filled in:

```
You are a fresh-context agent for this givn step.
For every user-facing question, follow the effective policy from `givn instructions dialog`: this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question.
The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
Load the step's rules with `givn instructions characterize` and follow them.
```
