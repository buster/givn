# Design Review: migrate-0-4-0-to-0-5-0

## Managed upgrade verification (Prompt 2)

- Commit A `e9938d66772adc8722fb1e5cfb0d6d19cbdf8f89` contains exactly one
  managed path: `givn/config.yaml` (marker `0.4.0` → `0.5.0`). Verified via
  `git show --stat`.
- Commit B `817e9481c63377137507b59c2dcb10d1136c2443` contains only this
  migration change (proposal, design, migration.yaml, `.givn-skip: specs`).
  `migration.yaml` records Commit A under `upgrade_commit`.
- No product feature file was invented; no `DESIGN-REVIEW: PASS`,
  implementation commit, or `REVIEW: PASS` existed in the generated change.

## Prompt 3 evidence: guidance reconciliation

- `givn skills sync` + `givn instructions` ran on the 0.5.0 binary
  (commit `3b55389`); all generated skills/commands carry the 0.5.0
  contract.
- The single project-owned override (`givn/artifacts/arc42-docs/impact-assessment-template.md`)
  is an impact-assessment template with no retired semantic sections and
  no missing 0.5.0 section — project intent, kept as-is.
- Resolved review instruction carries the deterministic disposition
  contract (`duplicate`/`variant`/`boundary`, `## Overlap dispositions`,
  `## Split-or-keep`); resolved specs instruction points the interaction
  inventory at `group.md`. No semantic classification contract remains in
  any generated guidance.

## Prompt 4 evidence: project-specific decisions (all pre-applied by this repo)

| Decision | State | Evidence |
|---|---|---|
| Semantic features | feature-free default accepted; opt-in documented | `Cargo.toml` `default = []`; README `--no-default-features` path asserted by `release-strict-model-overrides` |
| Corpus layout | grouped (7 use-case groups, one `group.md` each) | commit `3c2512b`; `givn spec tree` lists all groups with Actor/Goal |
| Capability retirement | 15 semantic capabilities retired via `@givn.retired` | `givn/specs/.retired.yaml`, change `retire-semantic-capabilities` |
| Interaction inventories | live in `group.md`, reconciled to `@e2e` counts | lint reports zero interaction over/under-coverage |
| Disposition vocabulary | deterministic decisions shipped and tested | `overlap::disposition` tests; template ships exact headings |
| Protected review artifact | no `remove: true` overlay present; `review` in effective manifest | `givn/config.yaml` has no active artifact-removal patch |
| Capability Routing | no active changes; template + gate shipped | `assets/templates/proposal.md`, `spec::routing_table` |
| Dead code | semantic duplicate-audit cluster and 37 orphaned step files deleted | commit `aef4959` |

## Stress-test

- The repo is the 0.5.0 reference implementation: every contract the bundle
  describes is exercised by its own executable specs, so verification is the
  suite itself (`./run-tests.sh`: 354/354; `cargo test --no-default-features`
  green; clippy clean).
- `.givn-skip` contains `specs` — the verify/e2e gates run against the
  permanent corpus, which is the strongest available verification for an
  operational migration record.

DESIGN-REVIEW: PASS
