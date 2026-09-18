# Design: convert-topic-persona-copies

## Overview

Replace three byte-identical biography copies with reuse records. The
permanent persona files are untouched.

## Exact file changes

| Path | Change |
|---|---|
| `givn/ideation/multi-scm-support/personas/agent--technical-role.md` | Reuse record, base `3e48068e` |
| `givn/ideation/multi-scm-support/personas/developer--primary-user.md` | Reuse record, base `93a37080` |
| `givn/ideation/multi-scm-support/personas/givn-maintainer--owner.md` | Reuse record, base `a3b1fa11` |

## Shape

```text
# Persona: <name> — topic record

Reuse: <persona-id>, base <short-hash>; this file adds record entries only

No topic-specific entries: the topic reused the project persona unchanged,
and its biography lives in givn/personas/<persona-id>.md.
```

## Verification

- Each topic file contains exactly one `Reuse:` line and no biography
  sections (`## Interests`, `## Goals`, `## Pain points`).
- `givn lint` reports no dangling persona reference.
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.

## Step Definitions

N/A — `specs` is skipped; no scenario delta and no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — documentation-only change.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: none (project-maintenance repair; `specs` skipped)
- Affected capability: none; the change converts topic persona records
- Confirmed Personas: the three subjects; no biography is copied

## Open Technical Questions

None.
