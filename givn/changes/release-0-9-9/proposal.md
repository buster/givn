# Proposal: release-0-9-9

## Release note

Givn 0.9.9 releases the bounded review context and the all-add-ons-enabled opt-out default.

## Use-Case Context

- Use-case ID: none (release-owned maintenance change; `specs` is skipped)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — release-owned version bump, no product surface change

## Review Context

- Review kind: self-review
- Reason: release-owned version bump; the default single-context review applies.

## Problem / Opportunity

Two archived changes are not yet released:

- `bound-review-context` makes the default review a self-review, permits one
  independent review context, records the review kind, and makes the outline
  gate use the recorded context.
- `default-all-addons` enables every add-on by default and writes an explicit
  opt-out for `givn init --no-addons`.

The package version is still 0.9.8 and the changelog has no 0.9.9 section. A
consumer installing the next release would not see either behavior named in the
release history.

## Proposed Solution

Release 0.9.9 with a version-only maintenance change:

- Bump `Cargo.toml` and the workspace package entry in `Cargo.lock` to 0.9.9.
- Bump the project config marker in `givn/config.yaml` to 0.9.9.
- Add the 0.9.9 changelog section naming both archived changes.
- Keep the existing migration catalog unchanged: both changes are guidance and
  default behavior, and an explicit project add-on false value is preserved.

No production Rust behavior changes in this release change. The two package
changes are already tested and archived.

## Capability Routing

No delta capability is authored: `specs` is skipped for this release-owned
maintenance change. The released behavior is already covered by the archived
changes and the permanent corpus.

## Out of Scope

- No new product behavior.
- No migration bundle.
- No add-on behavior change beyond the archived `default-all-addons` change.
- No harness, driver, model, or provider change.

## Open Questions

None.
