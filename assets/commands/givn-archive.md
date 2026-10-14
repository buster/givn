---
description: Archive a completed givn change — runs gate hooks, merges delta specs, moves to archive/
---

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: the change ID (kebab-case) after `/givn-archive`; if omitted, resolve the single active change.

Archive completed change `<change-id>`. Load the step's rules with `givn instructions archive` and follow them; this file only triggers the step.
