---
description: (Re-)write the scenario-by-scenario RED/GREEN/REFACTOR task list for a givn change
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-tasks`.

Write the implementation task list for change `<change-id>`. Load the step's rules with `givn instructions tasks --change <change-id>` and follow them; this file only triggers the step.
