# Design: retire-lint-command-adr

## Overview

Move the ADR-027 rationale into the verify-change specification, delete the
record, and repair the references. The naming behavior does not change.

## Exact file changes

| Path | Change |
|---|---|
| `givn/changes/retire-lint-command-adr/specs/verify-change/usecase.md` | Delta: Rules gain the lint naming rule; base hash `beec7612309622a7c18d03602497d9da8eceac705769c50e700606c6fb53eaec` |
| `docs/arc42/adr/ADR-027-lint-command-name.md` | Deleted |
| `docs/arc42/adr/archive/ADR-019-lint-contract.md` | Supersession references point at the spec; the historical status `superseded by ADR-027` stays, and the body notes the retirement |
| `docs/arc42/adr/README.md` | Active count 25 → 24; ADR-027 row removed; relationship and archive rows updated |

## Procedure

1. Write the delta narrative with the added rule and the base hash.
2. Delete the record with `git rm`.
3. Update ADR-019's references and the README so every reference resolves; the historical status value stays because the register-history scenario asserts it.

## Verification

- `rg -n "status: accepted" docs/arc42/adr/ADR-*.md | wc -l` reports 24.
- No reference to `ADR-027-lint-command-name.md` remains.
- The archive merge accepts the narrative (base hash matches).
- `./run-tests.sh` and `./run-tests.sh --e2e` pass.

## Step Definitions

N/A — the delta carries one narrative document; no new steps.

## Strict Mode (mandatory)

Inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`); no step is added.

## Local Runnability & Digital Twins (mandatory)

N/A — documentation-only change.

## Internal Primitives

None.

## Use-case traceability

- Use-case ID: `verify-change`
- Affected capability: the lint capability's naming contract moves to the
  use-case Rules
- Confirmed Personas: `developer--primary-user`

## Open Technical Questions

None.
