# Use Case Definitions: local-only-integrations

Cut from `eventstorming.md` (stable event IDs). Existing permanent use cases
keep their IDs, levels, and contracts; this topic defines their local-only
delta plus the reusable fragments it needs. Levels: `!` user goal
(change-sized), `+` summary, `-` subfunction (never becomes a use case),
fragment = reusable behavior with no independent user goal. Every event has
exactly one owner; "Consumes" lists inclusion or observation, not ownership.
`[gated]` marks a definition whose behavior depends on an open question.

## Summary table

| ID | Level | Status | Actors / personas | Goal (delta in italics) | Owns events | Consumes |
|---|---|---|---|---|---|---|
| `initialize-project` | ! | existing, modified | Developer, Supporting: Authoring agent (developer--primary-user, givn-maintainer--owner) | Set up a project and its agent integrations. *Givn rules resolve the policy, install locally by default, are ignored file-precisely, the `AGENTS.md` fence is written without being staged, tracked rules are reconciled, and the installation report names the policy.* | A2-A5, B5, E4 | A1, C4-C5 |
| `upgrade-a-project` | ! | existing, modified | Project maintainer, Supporting: Authoring agent (developer--primary-user, agent--technical-role, givn-maintainer--owner) | Cross a release gap with reviewable revisions. *Tracked strict rules become local-only, ignore entries join the revision, the revision excludes `AGENTS.md`, history is checked, and a policy change reconciles idempotently `[gated]`.* | C0-C3, C6-C9 | A1, A3, A4, C4-C5 |
| `author-change` | + | existing, unchanged | Developer, Authoring agent (developer--primary-user, agent--technical-role) | Set up a Givn project and create changes with guidance. *No direct delta; the delta lives in its children `initialize-project` (modified) and `create-change` (unchanged).* | — | — |
| `create-change` | ! | existing, unchanged | Developer, Supporting: Authoring agent (developer--primary-user, agent--technical-role) | Create a change ready to carry from intent to executable specification. *No delta; the commit path it drives changes through the `attribute-commits` fragment.* | — | A1, A3 |
| `archive-change` | ! | existing, unchanged | Developer (developer--primary-user, agent--technical-role) | Archive a completed change with proven publication. *No delta: ignored strict rules were already invisible to the clean-state witness.* | B4 | A3 |
| `maintain-project` | + | existing, unchanged | Project maintainer (developer--primary-user, givn-maintainer--owner) | Keep documentation and managed files current across releases. *No direct delta; the delta lives in `upgrade-a-project`.* | — | — |
| `verify-change` | ! | existing, possibly modified `[gated]` | Developer, Supporting: Reviewer (developer--primary-user; the Reviewer is a supporting role without a persona) | Check a change for structural validity, completeness, and overlap. *May consume the shared cleanliness report under local-only (Q7); no new gate until adopted.* | — | A1, D2 |
| `rules-policy` | fragment | new | included by every writer of generated rules | Resolve and expose whether generated rules are versioned or local-only. | A1 | — |
| `local-only-transition` | fragment | new | included by `initialize-project` and `upgrade-a-project` | Make tracked strict rules local-only: keep every file, remove exactly those paths from the current revision, leave the result staged. | C4, C5 | A1, C2 |
| `publication-cleanliness` | fragment | new | included by `initialize-project`, `upgrade-a-project`, and `verify-change` when adopted | Report strict rules that are tracked or present in history; exempt `AGENTS.md`; point at the opt-in rewrite; verify a fresh clone needs no Givn. | D2 | C7, E2, E3, G4 |
| `rewrite-history` | ! | new | Developer (developer--primary-user, givn-maintainer--owner) | Remove strict rules from reachable history before publication. *Explicit, never automatic; every rule file stays on disk; the reachable boundary and the new revision identifiers are reported.* | G1-G3 | D2, A3 |
| `support` — capability `attribute-commits` | fragment | existing, modified | included by the authoring and archive flows | Reusable commit plumbing. *Under local-only, `givn commit` stages project content only: `AGENTS.md` and strict rules are never staged by it.* | B1, B3 | A1, A3 |

Events without a single Givn owner: D1 (the developer publishes) and E1 (a
clone happens) are developer/audience facts the system observes. E2 and E3
are external-contributor actions observed as evidence by
`publication-cleanliness`; Givn cannot enforce them. F1-F6 are per-flow
refusal variants, each realized by the use case whose flow it interrupts, not
standalone owned events. B2 is the unchanged authoring loop and stays with
the existing use cases.

D3 (non-Givn files are never ignored, untracked, or deleted) is a
cross-cutting preservation rule carried by every writer of ignore rules and
transitions: `initialize-project` and `upgrade-a-project`.

## `initialize-project` (existing, modified)

- Goal unchanged; setup gains the policy and the local-only install path.
- Capability status: `initialize-a-project` modified (policy resolution,
  local install by default, ignore rules, `AGENTS.md` fence never staged);
  `enable-optional-addons` possibly modified (the installation report names
  the policy and what was kept local).
- Changed observable behavior:
  - `givn init` and `givn skills sync` write generated rules for the
    selected targets and never stage them under local-only (A2-A5).
  - Ignore entries are file-precise: one entry per strict rule path,
    never a directory, never `AGENTS.md`, never a user-owned file, written to
    both the tracked fenced block and the local exclude (A3; Q10).
  - A tracked strict rule is made local-only through the
    `local-only-transition` fragment; init leaves the removals staged and
    commits nothing.
  - The `AGENTS.md` fence is written or refreshed, and Givn never stages the
    file, tracked or not (A4, C9).
  - A fresh clone or second machine restores every local rule
    `[gated]` with one command (E4; Q14). No drift detection exists; sync
    overwrites.
  - Under local-only, the installation report names the local-only state and
    every path kept out of the repository or removed from it.
- Boundary: setup and local installation. Policy resolution is the
  `rules-policy` fragment; reconciliation is the `local-only-transition`
  fragment; the upgrade's managed revision stays with `upgrade-a-project`.
- Stakeholders: developer--primary-user (clean repository from day one),
  givn-maintainer--owner (default and report truthful).

## `upgrade-a-project` (existing, modified)

- Goal unchanged; the managed upgrade becomes policy-aware.
- Capability status: `apply-a-managed-upgrade` modified (reconciliation,
  revision content, policy change, history check); migration preparation
  unchanged.
- Changed observable behavior:
  - A tracked strict rule becomes local-only through the
    `local-only-transition` fragment while its file stays on disk, and the
    removals join the managed revision (C4-C5; Q7).
  - Ignore-block changes join the managed revision so the working tree stays
    clean for archive and later commits cannot re-add the paths (C6, C8).
  - Under local-only the managed revision contains the scaffold, ignore-entry
    changes, and removals only — no strict rule content and no
    `AGENTS.md`; under versioned it stages the generated rules per today's
    allowlist, with `AGENTS.md` still excluded (C8; Q7, Q9).
  - A policy change in `givn/config.yaml` triggers the same reconciliation
    `[gated]`, idempotently (C0; Q12, Q14).
  - A history occurrence of a strict rule is reported at the decided
    level; the level is open (C7; Q2).
- Boundary: reconciliation of a project past its setup. New-project setup is
  `initialize-project`.
- Stakeholders: developer--primary-user (existing projects become clean and
  stay reviewable), agent--technical-role (deterministic revisions),
  givn-maintainer--owner (constraint 2.7 and Q-57 amended truthfully).

## `create-change` (existing, unchanged)

- No observable delta; the commit path it drives changes through the
  `attribute-commits` fragment. Listed to confirm the interaction is
  examined, not to change it.

## `archive-change` (existing, unchanged)

- No observable delta: ignored strict rules were already invisible to
  the clean-state witness; untracked-not-ignored files still block as today
  (B4). Ignore rules must exist for local-only to coexist with archive (Q13).

## `verify-change` (existing, possibly modified `[gated]`)

- Goal unchanged.
- Candidate delta: consume the shared cleanliness report for a change's
  repository state under local-only. Adopt only if a per-change gate is
  wanted; the repository-wide answer stays with `publication-cleanliness`.
  Until Q7 and Q2 are decided, this stays a candidate, not an assertion.

## `author-change` and `maintain-project` (summaries, unchanged)

- Both are `+` summaries whose children carry this topic's delta;
  `author-change` covers `initialize-project` (modified),
  `maintain-project` covers `upgrade-a-project` (modified). Their own
  contracts and coverage rules are untouched.

## Fragment `rules-policy` (new)

- No independent user goal; every writer of generated rules consumes
  it.
- Capability: `policy-resolution`.
- Behavior to elaborate:
  - The default for new and existing projects without the key is local-only
    (Q1 confirmed).
  - An unrecognized value warns and continues as local-only, naming the valid
    values (Q1 confirmed).
  - The resolved value is readable by init, sync, upgrade, commit, archive,
    and any check.
  - Changing the value is a configuration edit; the next applicable command
    applies it `[gated]` (C0; Q12, Q14).
- Boundary: resolving and exposing the policy. It writes no Givn rules and
  stages nothing.
- Stakeholders: developer--primary-user, agent--technical-role,
  givn-maintainer--owner.

## Fragment `publication-cleanliness` (new)

- No independent user goal: the developer's goal is served inside setup and
  upgrade reporting and through the dedicated on-demand check (name open,
  Q21). The dedicated check exits non-zero on a violation; init and upgrade
  report and exit 0 (Q22 confirmed). History rewriting is its own use case,
  `rewrite-history`.
- Behavior to elaborate:
  - Report strict rules tracked at the current revision (Q7).
  - Report strict rules in reachable history and name that boundary (Q2);
    unreachable objects, remotes, and forks stay outside the guarantee (G4).
  - Exempt `AGENTS.md`; never touch or report developer-owned files.
  - Point at `rewrite-history` for remediation; never rewrite itself.
  - Verify, when Q15 is adopted, that a fresh clone builds and tests with
    `givn` absent; evidence E2/E3.
- Boundary: observation and reporting. Reconciliation is
  `upgrade-a-project`; rewriting is `rewrite-history`; deletion is nobody's
  automatic behavior.
- Stakeholders: developer--primary-user (publish with confidence),
  external-contributor--open-source-consumer (no Givn dependency),
  givn-maintainer--owner (an honest guarantee).

## `rewrite-history` (new)

- Goal: remove every strict rule from reachable history so a published
  repository carries none, while every rule file stays on disk.
- Level: `!`; Actors: Developer.
- Behavior to elaborate:
  - Explicit command only (working name `givn rewrite-history`); never
    automatic, never implied by upgrade or check (G1).
  - Requires the clean-state witness as a preflight; refuses before any
    write when the project is dirty (G1).
  - Names the strict rules found, the reachable boundary, and the revision
    identifiers before the rewrite (G1).
  - Rewrites reachable history so no revision contains a strict rule; the
    working files stay on disk (G2). Git mechanism open (Q20); Fossil
    support and mechanics open (Q20), including a possible named refusal.
  - Verifies afterwards and reports the new revision identifiers; refuses
    by name when the backend cannot rewrite (G3).
  - Never claims a remote, fork, or teammate's clone is clean (G4).
- Boundary: history rewriting only. Reporting is `publication-cleanliness`;
  reconciliation of the current revision is `local-only-transition`.
- Stakeholders: developer--primary-user (publish without Givn traces),
  givn-maintainer--owner (bounded, honest, never automatic).

## Fragment `support` — capability `attribute-commits` (existing, modified)

- No independent user goal; included by the authoring and archive flows.
- Behavior to elaborate: under local-only, the stage-all set excludes
  `AGENTS.md` and every strict rule (B3; user decision 2026-09-23);
  named paths are staged as given; attribution stays best-effort.
- Boundary: one revision per invocation with the correct stage set. It does
  not decide the policy; it consumes the fragment's resolved value.
- Stakeholders: developer--primary-user (no accidental fence commits),
  agent--technical-role (deterministic stage set).

## Rejected candidates

| Candidate | Level proposed | Why rejected |
|---|---|---|
| `publish-a-project` as a user-goal use case | ! | Givn performs no remote or publication operation (multi-scm-support boundary); it can only verify cleanliness, and publication is the developer's act (D1). |
| `verify-publication` as a standalone `!` use case | ! | The cleanliness check has no independent goal while it is included in setup and upgrade reporting. Promote it if a standalone command is ever wanted (see the fragment). |
| `restore-local-workflow` as a use case | - | A step inside init and sync (E4); no independent goal. |
| `become-local-only` as a use case | - | A step inside the `local-only-transition` fragment, included by setup and upgrade. |
| `configure-rules-policy` as a user-goal use case | - | Config resolution is a fragment consumed by every writer; the developer's goal is setup, not configuring an internal knob. |
