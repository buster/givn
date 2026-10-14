---
name: givn-design
description: Write the technical design for a givn change — the HOW layer with technology decisions, step definitions, and architecture impact.
---

# givn-design

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Write the technical design (`design.md`) for change `<change-id>`. The rules for this step are served by `givn instructions design --change <change-id>` — load the instruction and follow it. This file triggers the step and carries no rules of its own.
