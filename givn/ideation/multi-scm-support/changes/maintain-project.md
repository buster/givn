# Change seed: maintain-project

- **Stable use-case ID:** `maintain-project` (existing, modified)
- **Topic source:** `givn/ideation/multi-scm-support/`
  (`use-cases/maintain-project.md`, eventstorming D1-D7)
- **Confirmed Persona IDs:** `developer--primary-user`,
  `givn-maintainer--owner`
- **Accepted decisions:**
  - Q8: the managed upgrade and migration bundles run on both backends with
    the same guarantees — clean preflight, calculated allowlist, reviewable
    revisions. A migration-free upgrade publishes exactly one revision; a
    migration-bearing upgrade adds the plan revision recording the upgrade
    revision.
- **Open questions:** none blocking; Q10 affects wording only.
- **Handoff status:** ready for `/givn-propose` after seed 1.
- **Scope note:** adds the Fossil path for the managed upgrade and migration
  commits, including force-staging ignored managed paths only through the
  calculated allowlist. Modifies
  `givn/specs/maintain-project/release-0-3-0.feature`; the observable
  upgrade-revision content assertions in
  `givn/specs/operate-product/run-command.feature` are updated by seed 7.
