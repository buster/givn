---
name: givn-spec
description: Write Gherkin .feature delta files for a givn change — observable behaviour in domain language, no implementation detail.
---

# givn-spec

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Write the change's Gherkin delta files under `<change-id>/specs/`. The rules for this step are served by `givn instructions specs --change <change-id>` — load the instruction and follow it. This file triggers the step and carries no rules of its own.
