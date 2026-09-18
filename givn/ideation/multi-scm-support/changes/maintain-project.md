# Change seed: maintain-project

Status: implemented in `maintain-project`

- Use-case ID: `maintain-project` (existing, modified)
- Topic: `multi-scm-support`
- Personas: `developer--primary-user`, `givn-maintainer--owner`
- Suggested start: after `scm-backend`
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Q8 — Managed upgrade and migration on both backends

- Context: the managed upgrade published allowlist-only revisions on Git
  only; migration bundles followed.
- Options:
  1. **Run on both backends with the same guarantees** — clean preflight,
     calculated allowlist, reviewable revisions; a migration-free upgrade
     publishes exactly one revision; a migration-bearing upgrade adds the
     plan revision recording the upgrade revision.
     In practice: a Fossil project receives the allowlist-only check-in and
     the plan check-in.
     Gain: one upgrade contract.
     Cost: none.
  2. **Keep the upgrade Git-only** — refuse Fossil projects.
     In practice: Fossil projects maintain by hand.
     Gain: less work.
     Cost: a reduced lifecycle.
- Chosen: **Option 1**. Rejected: Option 2.

## Open questions

### Q10 — Vocabulary

Affects wording only; resolved 2026-09-14 with the neutral terms merged into
`docs/arc42/12-glossary.md`.

## Scope note

Added the Fossil path for the managed upgrade and migration commits,
including force-staging ignored managed paths only through the calculated
allowlist. Modified
`givn/specs/maintain-project/release-0-3-0.feature`; the observable
upgrade-revision content assertions in
`givn/specs/operate-product/run-command.feature` were updated by the
`operate-product` seed.
