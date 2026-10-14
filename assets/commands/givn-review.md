---
description: Complete the coverage review for a givn change — classify every gap, sign off, confirm GREEN
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-review`.

Complete the coverage review for change `<change-id>`. Load the step's rules with `givn instructions review --change <change-id>` and follow them; this file only triggers the step.

When you delegate to a fresh context, pass this block with the change id filled in:

```
You are a fresh-context agent for this givn step.
For every user-facing question, follow the effective policy from `givn instructions dialog`: this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question.
The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.
Load the step's rules with `givn instructions review --change <change-id>` and follow them.
```
