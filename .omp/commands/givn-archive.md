---
description: Archive a completed givn change — runs gate hooks, merges delta specs, moves to archive/
---

Archive a completed givn change.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: Optionally specify a change ID after `/givn-archive`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

**Rules**: the archive rules — the gate, the seed-stamping rule, and the
guardrails — are served by `givn instructions archive`. Load it and follow it;
this file owns the invocation orchestration only.

---

## Steps

### 1. Resolve the change and run pre-flight checks

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".

Apply the pre-flight checks from the served archive instruction: tasks done,
`REVIEW: PASS` present, `all_required_done`, and the spec lint:

```sh
givn lint
```

Stop and report when a check fails; fix parse errors and remaining `@wip`
scenarios before retrying.

### 2. Run the archive command

```sh
givn archive --change <id>
```

Report any gate error and stop.

### 3. Apply the archive instruction and verify the result

On success, `givn archive` has run the suite, merged the delta `.feature`
files, and moved the change to `givn/archive/<id>/`.

Then apply the seed-stamping rule from `givn instructions archive` (set the
recorded seed's status, or record "no seed" explicitly), and run the
regression check:

```sh
givn lint
```

Exit 0 = permanent specs are clean. Exit non-zero = report to the user (the
archive completed, but a problem in the permanent specs needs attention).

### 4. Show summary

```sh
givn status
```

---

## Output on success

```
## Archive complete: <id>

**Archived to:** givn/archive/<id>/

### Delta specs merged into givn/specs/
  + <capability>: N scenarios added
  ~ <capability>: N scenarios modified
  - <capability>: N scenarios removed

### Gate result
All hooks passed — test suite GREEN.

### Regression check
givn lint → clean (permanent specs well-formed).
```

## Output on gate failure

```
## Archive blocked: <id>

**Gate failed:** <error from givn archive>

**Action required:**
<specific guidance based on the failure>
```

---

## Guardrails

- Follow the rules served by `givn instructions archive`; fix gate failures
  rather than working around them, and never move files manually.
- Report the regression check result even if the archive succeeded.
