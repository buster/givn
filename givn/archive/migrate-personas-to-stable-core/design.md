# Design: migrate-personas-to-stable-core

## Overview

Rewrite the three permanent persona files to the stable-core schema. The
content is preserved where the schema has a field; the old `Interests`,
`Goals`, and `Pain points` sections fold into authority boundary, evaluation
capability, own vocabulary, and success criterion.

## Exact file changes

| Path | Change |
|---|---|
| `givn/personas/agent--technical-role.md` | Stable core + running record + topic stances; shared criterion stated in the Role line |
| `givn/personas/developer--primary-user.md` | Same |
| `givn/personas/givn-maintainer--owner.md` | Same |

## Identity criteria

| Persona | Merged segments | Shared criterion |
|---|---|---|
| agent--technical-role | authoring agent, automation caller | authority boundary (drive givn non-interactively) and evaluation capability (read the machine contract) |
| developer--primary-user | developer, project maintainer | authority boundary (own the project and its repository) and own vocabulary (day-to-day version control) |
| givn-maintainer--owner | seam-keeper, truth-keeper | authority boundary (own the architecture record) and evaluation capability (check claims against the implementation) |

## Content mapping

| Old section | New home |
|---|---|
| Role | Stable core Role (with the shared criterion) |
| Interests | Authority boundary / evaluation capability / own vocabulary |
| Goals | Success criterion / evaluation capability |
| Pain points | Own vocabulary refusals / evaluation capability limits |
| Success criterion | Stable core Success criterion |
| Vocabulary | Stable core Own vocabulary |
| Decision, need, and demand log | Running record (dated entries preserved) |
| Topic-specific demands | Topic stances (dated) |

## Verification

- Each file has the three sections with the five stable-core fields.
- The running record preserves every dated entry from the old log.
- `givn lint` reports no `[PERS ]` finding; `givn check review` resolves the
  Persona IDs.
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
- Affected capability: none; the change converts persona files
- Confirmed Personas: the three subjects

## Open Technical Questions

None.
