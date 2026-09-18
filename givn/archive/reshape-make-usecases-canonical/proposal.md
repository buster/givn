# Proposal: reshape-make-usecases-canonical

## Use-Case Context

- Use-case ID: `make-usecases-canonical`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `givn-maintainer--owner`

## Problem / Opportunity

The 0.8.0 use-case model falsifies the current document's **R5 extension
discipline**: the extension lacks its main-flow step and the protected
guarantee.

The 0.7.0 to 0.8.0 migration inventory
(`givn/archive/migrate-0-7-0-to-0-8-0/design.md`, Phase 1) records this
deviation and names this follow-up.

## Proposed Solution

Anchor the extension to step 1: a `1: <condition> → <handling>; protects
<guarantee>` bullet. No other change.

## Out of Scope

- The one-capability advisory (`[WARN ]`), which is acceptable for this
  maintainer tooling.
- The other use-case reshapes and the support-fragment renames.

## Open Questions

None.
