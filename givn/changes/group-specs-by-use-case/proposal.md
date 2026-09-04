# Proposal: group-specs-by-use-case

## Problem / Opportunity

The permanent spec corpus is flat: one directory per capability
(`givn/specs/<cap>/<cap>.feature`). With 38 capabilities this gives no
orientation: an author cannot see which capabilities belong to the same
user-facing workflow, and the e2e-per-interaction inventory (Phase 3 of
the retirement plan) has no durable home.

`givn spec regroup` produced a command-surface clustering proposal
(validated against the plan's expected signal). This change applies the
human-confirmed version of that proposal: rename clusters to use-case
labels and merge tiny clusters sensibly.

## Proposed Solution

Regroup `givn/specs/` into `givn/specs/<group>/<capability>.feature`, one
`group.md` per group, with these confirmed groups:

- **authoring** (4): new-command, init-agent-target-selection,
  prose-consolidation, install-extras
- **verification** (6): lint-command, spec-overlap-gates,
  deduplicate-change-dir-detection, givn-workflow-enforcement,
  review-disposition-gates, validate-artifact-id-input
- **status** (4): givn-status-command, fix-uppercase-checkbox-parsing,
  remove-dead-skip-list-code, validate-change-id-input
- **spec-discovery** (3): spec-duplicates, spec-progress-output,
  release-strict-model-overrides
- **archival** (3): enforce-post-merge-e2e-gate, extract-coverage-gate,
  preserve-docstrings-on-archive
- **maintenance** (4): release-0-3-0, run-command, run-execution,
  readme-enforcement
- **support** (14): the `«include»`-fragment capabilities with no dominant
  command surface — adr-qualification-gate, arc42-review-alignment,
  choose-project-license, config-overlay, coverage-readme-example,
  coverage-script-workflow, domain-skill-trigger, instruction-integrity,
  readme-coverage-badge, repository-ownership-fixtures, skill-rewrites,
  spec-explore-change, spec-explore-permanent,
  template-conditional-rendering

Human decisions beyond the tool output: `new`/`init`/`instructions`/
`addons` clusters merged into `authoring` (all four are change-authoring
commands); `check`+`lint` merged into `verification`; `upgrade`+`run`
merged into `maintenance` with `readme` added (product upkeep); fragments
housed in `support`. No single-capability groups remain, so the
`singleton_group` warning is clean.

File moves use `git mv` so history follows the files.

## Out of Scope

- The watn project's equivalent migration (done immediately after, same
  mechanism, separate commit in that repo).
- Rewriting scenario content (phrasing remediation is task 2.8, after
  Phase 5).
- Any delta .feature content — this change is pure reorganization, so the
  `specs` artifact is skipped.

## Open Questions

None — the grouping above is the confirmation this plan requires.
