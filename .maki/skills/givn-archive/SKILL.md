---
name: givn-archive
description: Archive a completed givn change — runs gate hooks, merges delta specs into permanent specs, and moves the change to archive/.
---

# givn-archive

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

Archive completed change `<change-id>`. The rules for this step are served by `givn instructions archive` — load the instruction and follow it. This file triggers the step and carries no rules of its own.
