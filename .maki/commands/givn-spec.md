---
description: (Re-)write the Gherkin .feature spec for a givn change
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-spec`.

Write the change's Gherkin delta files. Load the step's rules with `givn instructions specs --change <change-id>` and follow them; this file only triggers the step.
