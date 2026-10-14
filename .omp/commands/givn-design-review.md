---
description: Stress-test the plan before tasks are written — grill the design, harden the artifacts, sign off
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-design-review`.

Stress-test the plan for change `<change-id>` before tasks are written. Load the step's rules with `givn instructions design-review --change <change-id>` and follow them; this file only triggers the step.

When you delegate to a fresh context, pass this block with the change id filled in:

```
You are a fresh-context agent for this givn step.
For every user-facing question, follow the effective policy from `givn instructions dialog`: this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question.
The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
Load the step's rules with `givn instructions design-review --change <change-id>` and follow them.
```
