---
name: givn-propose
description: Write the proposal for a givn change — describe WHAT and WHY in domain language, without implementation detail.
---

# givn-propose

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Write the proposal for change `<change-id>`. The rules for this step are served by `givn instructions proposal --change <change-id>` — load the instruction and follow it. This file triggers the step and carries no rules of its own.
