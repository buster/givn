# Design Review: release-0-3-0

## Grilling Evidence

### Scope

**Question:** Does the change cover the requested release, complete stacked
delta, LLM migration guide, automatic migration change, clean Git boundary,
and two-commit evidence without silently changing project behavior?

**Answer:** Yes. The proposal, feature scenarios, and design cover package
versioning, changelog, the complete 0.2.0-to-0.3.0 guide, managed integration
refresh, allowlisted Commit A, the generated maintenance change with Commit A
evidence, and the refusal/no-duplication boundaries. Domain specs, source code,
ADRs, Arc42 facts, and ejected facets remain LLM-reviewed migration work.

### Technology Choices

**Question:** Is a Git subprocess plus existing rendering code simpler and safer
than adding a Git library or a generic migration framework?

**Answer:** Yes. The current project already treats Git diffs as the review
surface. Calling the installed Git CLI avoids a new dependency and preserves
the user's exact Git configuration. The route is version-specific and
allowlisted, so a generic migration registry would add complexity without
improving this release.

### Missing Scenarios

**Question:** What happens when the target is not a Git repository, when the
worktree is dirty, when a migration plan already exists, or when the generated
maintenance change is later archived without a product spec?

**Answer:** These are explicit scenarios. The command refuses before writing in
the first two cases. It does not overwrite or duplicate an existing migration
plan. A separate failure path for an unavailable Git identity is handled as a
Git commit error and leaves the user with an actionable message. The archive
scenario verifies that skipped specs use the permanent project feature path for
global gates and do not invoke a change-specific semantic comparison.

### Testability

**Question:** Can every new scenario fail for a concrete reason rather than
merely proving that the command ran?

**Answer:** Yes. Documentation scenarios assert required headings, terms, and
prompt order. The E2E scenario asserts exit status, version marker, exact
commit paths, commit count, migration files, recorded hash, and absent
product spec. Dirty and non-Git scenarios assert byte-level preservation and
no commit. Idempotency asserts the unchanged plan and commit history.

### Primary Risk

**Question:** What is the most likely implementation failure?

**Answer:** The allowlist could accidentally stage user content from a
generated-file directory, or a failure between Commit A and Commit B could
leave an unclear state. Mitigation is full clean-worktree preflight, explicit
known generated paths, no directory-wide staging, a pre-write snapshot, and
an output that names the successful first commit and any remaining staged plan
files. The command never resets or reverts user history.

### ADR Qualification

The candidate decision is the automatic two-commit upgrade protocol and its
maintenance-change boundary.

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "AMEND_ADR",
  "canonical_artifact": null,
  "target_adr": "ADR-023",
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "manual patch without an automatic commit",
      "one automatic commit without a recorded migration plan",
      "two commits separating machine-safe preparation from the LLM plan"
    ],
    "architectural_impact": [
      "defines the compatibility boundary between a 0.2.0 project and the 0.3.0 tool",
      "defines which generated paths Givn may stage and commit"
    ],
    "durable_consequence": [
      "changes project history and future upgrade behavior",
      "a later reversal requires coordinated migration and commit semantics"
    ],
    "lower_level_artifact": [
      "the existing ADR-023 owns versioned-config upgrade semantics but does not own the new automatic migration-plan protocol"
    ],
    "existing_adr_check": [
      "ADR-023 was found as the existing versioned-config upgrade decision; this change refines it rather than creating a duplicate ADR"
    ]
  }
}
```

`Must be shared` is supporting evidence only. The qualification is based on
all five mandatory dimensions. The decision is an amendment to ADR-023; no new
ADR is created.

### Arc42 Impact

The change affects chapters 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, and 12:

| Chapter | Affected? | Reason |
|---|---|---|
| 01 introduction-and-goals | Yes | 0.3.0 adds an upgradeability and migration goal for existing projects. |
| 02 architecture-constraints | Yes | Automatic commits require a Git worktree boundary and explicit ownership rules. |
| 03 context-and-scope | Yes | `givn upgrade` gains a visible migration-preparation interaction. |
| 04 solution-strategy | Yes | The release separates safe machine preparation from LLM-reviewed project migration. |
| 05 building-block-view | Yes | Upgrade orchestration and managed-path selection gain a migration responsibility. |
| 06 runtime-view | Yes | The two-commit upgrade sequence is a new runtime flow. |
| 07 deployment-view | Yes | The 0.3.0 binary and Git CLI interaction are part of deployment/use. |
| 08 crosscutting-concepts | Yes | Config, generated guidance, ownership, and migration contracts change. |
| 09 architecture-decisions | Yes | ADR-023 is amended with the automatic migration protocol. |
| 10 quality-requirements | Yes | New release and migration safety scenarios are added. |
| 11 risks-and-technical-debt | Yes | Commit failure, clean-worktree, and project-specific migration risks change. |
| 12 glossary | Yes | Release migration, maintenance change, and managed upgrade commit become domain terms. |

The durable chapter updates and the ADR-023 amendment are part of the
implementation tasks. They must be checked against the completed implementation
before `REVIEW: PASS`.

## Hardening Applied

- Added explicit non-Git and existing-plan scenarios to the feature.
- Required a clean worktree before any write or commit.
- Limited automatic staging to a calculated Givn-owned allowlist.
- Separated the machine-safe Commit A from the migration-plan Commit B because
  a commit cannot contain its own hash.
- Kept product specs, project source, ejected facets, Arc42 facts, and ADR
  decisions outside the automatic file operation.
- Required the generated maintenance change to skip `specs` and to contain no
  fabricated design-review, task, implementation, or review evidence.
- Added the archive contract for a no-product-delta maintenance change: global
  verification remains active, while delta merge and the change-specific
  semantic gate are skipped.
- Re-ran the interaction coverage review for the new archive action and added
  its executable E2E scenario and separate permanent-spec assertion.

## Review Result

The design is complete, the missing boundaries are represented by executable
scenarios, and the architectural decision is routed to the existing ADR-023
for amendment. The additional no-product-delta archive flow is covered by the
permanent-spec verification path and does not weaken product-change gates.

DESIGN-REVIEW: PASS
