# Design: retire-semantic-adr-residue

## Overview

Move three obsolete records out of the active register. Each record keeps its
content; its `status` becomes `superseded by ADR-069`, it moves under
`docs/arc42/adr/archive/`, and every index that names it is updated.

## Exact file changes

| Path | Change |
|---|---|
| `docs/arc42/adr/ADR-044-current-change-explore.md` | Move to `archive/`; status becomes `superseded by ADR-069` |
| `docs/arc42/adr/ADR-045-project-relative-identity.md` | Move to `archive/`; status becomes `superseded by ADR-069` |
| `docs/arc42/adr/ADR-054-versioned-evidence-output.md` | Move to `archive/`; status becomes `superseded by ADR-069` |
| `docs/arc42/adr/ADR-069-single-feature-free-binary.md` | Supersession list gains the three records |
| `docs/arc42/adr/README.md` | Active count 28 → 25; three rows move to the archived list; relationship line updated |
| `givn/changes/retire-semantic-adr-residue/specs/fragments/adr-qualification-gate.feature` | Delta: the register-history scenario expects 25 accepted decisions |
| `givn/specs/fragments/adr-qualification-gate.feature` | Aligned with the delta (28 → 25) so the permanent suite stays green during the change, per the `scm-backend` precedent |

## Procedure

1. `git mv` each record into `archive/` and set `status: superseded by
   ADR-069` in its frontmatter.
2. Append the three links to ADR-069's "More Information" supersession list.
3. Update the README: count, active-register rows removed, archived-register
   rows added, and the `ADR-039, 041, … -> ADR-069` relationship line extended
   with `044, 045, 054`.
4. Align the permanent register-history scenario with the delta (28 → 25).
5. Check the cross-references: the records' own links to `archive/ADR-…` stay
   valid because they already use relative archive paths where needed.

## Verification

- `rg -n "status: accepted" docs/arc42/adr/ADR-*.md | wc -l` reports 25.
- `rg -n "ADR-044|ADR-045|ADR-054" docs/arc42/adr/README.md` shows only the
  archive and relationship references.
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.
- `givn check review --change retire-semantic-adr-residue` passes.

## Step Definitions

No new steps. The modified scenario reuses the existing step definition
`the final register has 25 accepted decisions and no proposed decisions`
(`tests/steps/adr_qualification_gate_steps.rs`), which asserts the live
register's accepted count.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — documentation-only change.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: none (project-maintenance repair; `specs` skipped)
- Affected capability: none; the repair maintains the decision register
- Confirmed Personas: none (no user-visible surface)

## Open Technical Questions

None.
