---
description: (Re-)write the technical design for a givn change — HOW layer, technology decisions, architecture impact
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-design`.

Write the technical design for change `<change-id>`. Load the step's rules with `givn instructions design --change <change-id>` and follow them; this file only triggers the step.
