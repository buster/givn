# Use Case Definitions: multi-scm-support

Cut from `eventstorming.md` (stable event IDs). Existing permanent use cases
keep their IDs and contracts; this topic defines their SCM-neutral delta plus
one new reusable fragment. Levels: `!` user goal (change-sized), `+` summary,
`-` subfunction (never becomes a use case), fragment = reusable behavior with
no independent user goal. Every event has exactly one owner; "Consumes" lists
inclusion or observation, not ownership.

## Summary table

| ID | Level | Status | Actors / personas | Goal (delta in italics) | Owns events | Consumes |
|---|---|---|---|---|---|---|
| `archive-change` | ! | existing, modified | Developer (primary user, technical role, givn maintainer) | Publish a completed change and prove the published state. *Witness, candidate publication, receipt binding, and repair work on the active backend.* | C1-C10, E1-E7 | A1 |
| `maintain-project` | ! | existing, modified | Project maintainer (primary user, givn maintainer) | Keep a project current and able to cross a release migration. *Clean preflight, allowlist revisions, and the two-revision upgrade shape work on the active backend.* | D1-D7 | A1 |
| `author-change` | ! | existing, modified | Developer, Authoring agent (primary user, technical role, givn maintainer) | Create a valid change and its guidance. *Initialization and generated guidance respect the active backend.* | A2, A3, B7 | A1, A4 |
| `inspect-change-state` | ! | existing, possibly modified | Developer (primary user, technical role, givn maintainer) | Understand the current change and artifact state. *Status names the active backend and reports revision identity in backend-neutral wording.* | none (display delta only) | A1, E2 |
| `verify-change` | ! | existing, modified | Developer, Reviewer (technical role, givn maintainer) | Check that a change is structurally valid and complete. *The recorded revision identifier accepts both backends' shapes.* | B8 | A1 |
| `operate-product` | ! | existing, possibly modified | Product operator (primary user, technical role) | Start the declared product and observe readiness. *Upgrade-revision contents asserted through the run flow hold on both backends.* | none (consumer) | D4, D5 |
| `scm-backend` | fragment | new | included by the use cases above | Detect the project's SCM backend, expose its capability facts, or refuse by name. | A1 | — |
| `support` — capability `llm-attribution` | fragment | existing, modified | included by every use case | Reusable support behavior. *Attribution is stamped through a path the active backend supports.* | A4 | B7 |

Untouched events stay with their existing use cases and are not part of this
change program: B1-B6 and B9 (the authoring flow) remain `author-change`
behavior; A2/A3 are modified only for backend-aware initialization and
guidance.

## `archive-change` (existing, modified)

- Goal unchanged; the proof contract is restated per backend.
- Capability status: `archive-proof` modified (the change's core);
  `grouped-merge-for-new-capabilities` unchanged;
  `preserve-docstrings-on-archive` unchanged; `extract-coverage-gate`
  unchanged; `enforce-post-merge-e2e-gate` unchanged.
- Changed observable behavior:
  - The clean-state witness uses the active backend's change report (Q3).
  - The archive candidate is published as a revision of the active backend
    containing exactly the merged corpus paths (publisher per Q5).
  - Pending and proven receipts bind the backend's revision identifier (Q11).
  - A backend hook that rejects the candidate leaves a repairable state; the
    exact shape is per backend (Q13).
  - Refusal cases (dirty state, nested root, already proven, legacy archive,
    active and archived copy both present) name the backend state truthfully.
- Boundary: archive publication and proof. Detection and capability facts are
  the `scm-backend` fragment; the no-SCM refusal is A1's policy.
- Stakeholders: primary user (proven archive in either SCM), technical role
  (deterministic commands), givn maintainer (truthful constraints).

## `maintain-project` (existing, modified)

- Goal unchanged; the managed upgrade runs on the active backend.
- Capability status: `release-0-3-0` modified (clean preflight, allowlist-only
  upgrade revision, plan revision, no-SCM refusal); `readme-enforcement`
  unchanged.
- Changed observable behavior:
  - Clean-state preflight and allowlist staging use the active backend.
  - Upgrade and plan revisions are published on the active backend; the plan
    revision records the upgrade revision (publisher per Q5).
  - Ignored managed paths are force-staged only through the calculated
    allowlist, on both backends.
- Boundary: versioned maintenance and migration. Routine authoring is
  `author-change`; product start is `operate-product`.
- Stakeholders: primary user (reviewable upgrades in either SCM), givn
  maintainer (no duplicated SCM logic).

## `author-change` (existing, modified)

- Goal unchanged; initialization and generated guidance become backend-aware.
- Capability status: `init-agent-target-selection` modified (attribution
  plumbing installed only where supported; the installation report names what
  was installed and skipped); `prose-consolidation` modified (generated
  guidance names only commands valid for the active backend);
  `install-extras` unchanged; `new-command` unchanged;
  `visual-design-contract` unchanged.
- Changed observable behavior:
  - `givn init` in a Fossil checkout scaffolds the same `givn/` corpus and
    reports the selected backend.
  - Generated skills branch on the backend for commit guidance (Git:
    `git commit`; Fossil: the givn-controlled commit path — Q2).
  - Initialization without any SCM behaves per the open question Q7; this
    definition does not assert a warning or a refusal.
- Boundary: authoring a change and generating its guidance. Detection is the
  `scm-backend` fragment.
- Stakeholders: primary user (no dead guidance), technical role (commands
  that work verbatim), givn maintainer (guidance and specs stay truthful).

## `inspect-change-state` (existing, possibly modified)

- Goal unchanged; status becomes backend-aware.
- Capability status: `givn-status-command` possibly modified. Elaboration
  decides explicitly whether a new scenario is needed or the existing
  scenarios already cover the behavior once worded backend-neutrally.
- Delta scope: name the active backend and report revision identity. The
  unproven-candidate surfacing (E2) stays owned by `archive-change`; this use
  case only consumes it.
- Stakeholders: primary user (knows which SCM is active), technical role
  (machine-readable state), givn maintainer (spec truthfulness).

## `verify-change` (existing, modified)

- Goal unchanged.
- Capability status: `givn-workflow-enforcement` modified — the recorded
  revision identifier must accept both backends' hash shapes while keeping
  placeholder and duplicate rejection (Q11).
- Boundary: structural validity and integrity. It does not publish anything.
- Stakeholders: technical role (recording a valid identifier), givn
  maintainer (integrity contract).

## `operate-product` (existing, possibly modified)

- Goal unchanged.
- Capability status: `run-command` possibly modified — scenarios asserting the
  latest revision's contents after `givn upgrade` need backend-neutral
  wording. Elaboration decides whether the wording change is observable
  enough to need a scenario edit.
- Boundary: product start and its machine contract.
- Stakeholders: technical role (automation reads the contract), primary user.

## Fragment `scm-backend` (new)

- No independent user goal; included by every use case above.
- Capabilities: `scm-backend-selection`.
- Behavior to elaborate:
  - Selection order: Git worktree, then Fossil checkout, then named refusal.
  - A selected backend whose binary is absent is refused by name (Q14).
  - Mixed or nested setups are either refused explicitly or resolved by the
    confirmed order (Q6).
  - Capability facts exposed per backend: clean/dirty witness, revision
    identifier, subset publication, hook support, attribution support, and
    the project-boundary rule (Git: worktree top level, TC-32; Fossil:
    checkout root with the repository file outside — Q4).
- Boundary: detection and capability description. It publishes, merges, and
  verifies nothing.
- Stakeholders: all three.

## Fragment `support` — capability `llm-attribution` (existing, modified)

- No independent user goal; the attribution capability is reused by the
  authoring and archive flows.
- Behavior to elaborate (gated on Q2 feasibility):
  - Git keeps the chained `commit-msg` hook.
  - Fossil stamps attribution through the givn-controlled commit path, or the
    capability records an explicit limitation for the backend.
  - Missing attribution context never blocks a revision, on either backend.
- Boundary: attribution only; it must not drive a larger SCM design
  (maintainer interest O4).
- Stakeholders: technical role (guidance it can satisfy), primary user
  (attribution where feasible).

## Rejected candidates

| Candidate | Level proposed | Why rejected |
|---|---|---|
| `select-scm-backend` as a user-goal use case | ! | No independent user goal: detection is a precondition of every lifecycle action. Modeled as the `scm-backend` fragment. |
| `multi-scm-support` as a summary use case | + | Would group existing use cases that already have independent goals; the topic is a change program, not a domain goal. |
| `commit-attribution` as a use case | ! | Reusable behavior without an independent goal; already a fragment capability. |
| `publish-subset` as a use case | - | A step inside archive publication. |
| `detect-backend` as a use case | - | A subfunction of the `scm-backend` fragment. |
