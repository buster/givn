# Event Storming: multi-scm-support

Domain events in the order they happen, with actor, command, reacting policy,
and read model. `[diverges]` marks a policy that differs by backend (table
below). `[guard]` marks an internal check or refusal rather than a value
event. IDs are stable for cross-references from `use-case-definitions.md`.

## Chain

### A. Project start

- **A1 — SCM backend was selected** — Aggregate: Project. Actor: givn (on any lifecycle
  command). Command: `givn init`, `givn status`, `givn archive`, ... Policy:
  a Git worktree wins; otherwise a Fossil checkout; otherwise a named
  refusal (including a selected backend whose binary is absent — Q14). Read
  model: `.git` / `git rev-parse --is-inside-work-tree`;
  `.fslckout` / `_FOSSIL_`; `fossil status`.
- **A2 — Project was initialized** — Aggregate: Project. Actor: primary user. Command:
  `givn init`. Policy: scaffold `givn/`, generate skills for the selected
  targets, install attribution plumbing only where the backend supports it.
  Read model: project root, selected backend, skill targets.
- **A3 — Guidance was generated for the backend** — Aggregate: Project. Actor: givn. Command:
  `givn skills sync`. Policy: generated guidance may name only commands that
  work on the active backend. Read model: backend, skill templates, targets.
- **A4 — Attribution path was prepared** — Aggregate: Project. Actor: givn. Command: `givn init`
  / `givn upgrade`. Policy `[diverges]`: Git installs the chained
  `commit-msg` hook; Fossil has no `commit-msg` hook, so the path is a
  givn-controlled commit (open question Q2). Read model: hook capability of
  the backend.

### B. Change authoring

- **B1 — Change was proposed** — Aggregate: Change. Actor: primary user. Command:
  `/givn-propose`. Policy: lifecycle gates, artifact graph. Read model:
  `givn/` state, effective manifest.
- **B2 — Change was specified** — Aggregate: Change. Actor: primary user. Command:
  `/givn-spec`.
- **B3 — Design was written and reviewed** — Aggregate: Change. Actor: primary user. Commands:
  `/givn-design`, `/givn-design-review`.
- **B4 — Tasks were broken down** — Aggregate: Change. Actor: primary user. Command:
  `/givn-tasks`.
- **B5 — Scenario was tagged as delta** — Aggregate: Change. Actor: primary user. Command:
  `/givn-spec` (delta authoring). Policy: `@givn.delta` tags describe Added,
  Modified, or Removed behavior and are consumed by the archive merge. Read
  model: delta feature files.
- **B6 — Scenario was implemented** — Aggregate: Change. Actor: authoring agent. Command: RED /
  GREEN / REFACTOR loop.
- **B7 — Revision was created for the scenario** — Aggregate: Change. Actor: authoring agent.
  Command `[diverges]`: `git commit` (with the attribution hook) or
  `fossil commit` (new files need `fossil add` first). Policy: exactly one
  atomic revision per scenario; attribution appended where supported. Read
  model: `git status` / `fossil status`.
- **B8 — Revision identifier was recorded** — Aggregate: Change. Actor: authoring agent.
  Command: edit `tasks.md`. Policy: the integrity check accepts the active
  backend's identifier shape and rejects placeholders and duplicates. Read
  model: `git rev-parse HEAD` / `fossil status` checkout hash.
- **B9 — Review was signed off** — Aggregate: Change. Actor: primary user. Command:
  `/givn-review`. Policy: mandatory checks and gate dispositions.

### C. Archive publication

- **C1 — Archive was requested** — Aggregate: Change. Actor: primary user. Command:
  `givn archive --change <id>`. Policy: `apply.requires` gates run first.
- **C2 — Project state was witnessed** — Aggregate: Project. Actor: givn. Policy `[diverges]`:
  no changes beyond declared evidence paths; Git counts untracked files,
  Fossil reports them only with `changes --differ` (open question Q3). Read
  model: `status --porcelain -z -uall` / `fossil changes --differ`.
- **C3 — Accepted behavior was merged** — Aggregate: Change. Actor: givn. Policy: delta merge
  with base-revision match; retirement and grouping rules.
- **C4 — Evidence scope was resolved** — Aggregate: Change. Actor: givn. Policy: the
  finalization may change only the resolved evidence paths — the archive
  directory, the README when the readme addon is on, and the configured
  coverage outputs. Read model: effective manifest addons, coverage paths,
  archive id.
- **C5 — Archive candidate was published** — Aggregate: Change. Actor: givn (publisher per open
  question Q5). Command `[diverges]`: a Git commit of the merged paths, or a
  Fossil check-in of the merged paths (`fossil add` for new files). Policy:
  the candidate revision contains exactly the merged corpus paths. Read
  model: revision identifier and its file list.
- **C6 — Pending receipt was recorded** — Aggregate: Change. Actor: givn. Policy: the pending
  receipt carries the candidate revision and marks the archive unproven, so
  a rerun resumes without repeating the merge. Read model:
  `givn/archive/<id>/verification.json`.
- **C7 — Verification scopes were executed against the candidate** — Aggregate: Change. Actor:
  givn and the test runner. Command: `verify.command`,
  `verify.e2e_command`. Policy: one run per scope, fail-closed runner
  result.
- **C8 — Coverage was measured and published** — Aggregate: Change. Actor: givn. Command:
  coverage addon commands. Policy: coverage gate.
- **C9 — Archive was finalized** — Aggregate: Change. Actor: givn. Policy: the revision
  binding is checked `[guard]` (tested revision equals candidate revision),
  the proven receipt is recorded, README managed blocks are refreshed, and
  an evidence-only revision is published. Read model: receipt, revision
  identifier, evidence scope.
- **C10 — Archive was proven** — Aggregate: Change. Actor: givn. Policy: only a proven archive
  is complete; an unproven candidate is never reported as proven.

### D. Managed upgrade

- **D1 — Upgrade was requested** — Aggregate: Project. Actor: primary user. Command:
  `givn upgrade`. Policy: version gap resolution through the migration
  catalog.
- **D2 — Clean state was witnessed** — Aggregate: Project. Actor: givn. Policy `[diverges]`:
  same witness semantics as archive.
- **D3 — Managed scaffold was synchronized** — Aggregate: Project. Actor: givn. Policy:
  Givn-owned allowlist only; ignored managed paths may be force-staged only
  through the calculated allowlist.
- **D4 — Run declaration was backfilled** — Aggregate: Project. Actor: givn. Policy: append the
  `run:` scaffold to `givn/commands.yaml` when the key is absent;
  idempotent. Read model: effective manifest, `givn/commands.yaml`.
- **D5 — Upgrade revision was published** — Aggregate: Project. Actor: givn (publisher per open
  question Q5). Command `[diverges]`: the allowlist-only revision (the
  current glossary's "Upgrade Commit A").
- **D6 — Migration plan was recorded and published** — Aggregate: Project. Actor: givn. Policy:
  the plan revision records the upgrade revision (the current glossary's
  "Migration Commit B"); the maintenance change is created.
- **D7 — Maintenance change was created** — Aggregate: Project. Actor: givn. Policy: visible
  change that may skip a product specification.

### E. Failure, refusal, and repair

- **E1 — Candidate verification failed** — Aggregate: Change. Actor: givn. Policy: the
  candidate stays published and unproven; no rollback.
- **E2 — Unproven candidate was surfaced** — Aggregate: Change. Actor: givn. Command:
  `givn status` / `givn instructions`. Policy: the unproven candidate is
  reported and routed to its resume command. Read model: receipt status,
  change state.
- **E3 — Candidate was repaired forward** — Aggregate: Change. Actor: primary user / authoring
  agent. Command: fix, new revision, `givn archive` resume. Policy: the
  failed candidate is never silently discarded.
- **E4 — Candidate revision was refused by a hook** — Aggregate: Change. Actor: backend hook.
  Policy `[diverges]`: a Git `commit-msg` rejection leaves the merge staged
  and unproven; a Fossil `before-commit` rejection aborts the check-in with
  the working tree unchanged (open question Q13). Read model: hook output.
- **E5 — README check refused the archive** — Aggregate: Change. Actor: givn `[guard]`.
  Policy: open placeholders or damaged managed blocks block the archive when
  the readme addon is enabled.
- **E6 — Archive run was refused** — Aggregate: Change. Actor: givn `[guard]`. Cases: dirty
  state, nested repository root, already proven, legacy archive without a
  receipt, active and archived copy both present. The no-SCM case belongs to
  A1's selection policy.
- **E7 — Revision binding was checked** — Aggregate: Change. Actor: givn `[guard]`. Policy:
  finalization refuses a tree change outside the resolved evidence; the
  tested revision must be the published candidate revision.

## Canvas

Every event of the chain with its actor, command, and aggregate. Kinds are
marked on every node (`Actor`, `C`, `A`, `E`, `P`, `RM`, `X`, `Record`): a
command sits between the actor that issues it and the aggregate it changes,
and the aggregate emits its events. The only aggregates are `Project` (the
version-controlled Givn project) and `Change` (a unit of work under
`givn/changes/`); the SCM backend, receipts, manifests, and revision
identifiers are external systems, records, configurations, or specifications
and are never drawn in the aggregate shape.

### A. Project start

```mermaid
%% kinds: Actor/C/A/E/P/RM/X/Record
flowchart LR
  GV([Actor: Givn]) --> C1[C: givn init]
  GV --> C2[C: givn status]
  GV --> C3[C: givn archive]
  PU([Actor: Primary user]) --> C1
  C1 --> PRJ[(A: Project)]
  C2 --> PRJ
  C3 --> PRJ
  PRJ --> EA1(E: SCM backend was selected)
  PRJ --> EA2(E: Project was initialized)
  GV --> C4[C: givn skills sync]
  C4 --> PRJ
  PRJ --> EA3(E: Guidance was generated for the backend)
  GV --> C5[C: givn upgrade]
  C5 --> PRJ
  PRJ --> EA4(E: Attribution path was prepared)
```

### B. Change authoring

```mermaid
%% kinds: Actor/C/A/E/P/RM/X/Record
flowchart LR
  PU([Actor: Primary user]) --> C1[C: givn-propose]
  C1 --> CHG[(A: Change)]
  CHG --> EB1(E: Change was proposed)
  PU --> C2[C: givn-spec]
  C2 --> CHG
  CHG --> EB2(E: Change was specified)
  PU --> C3[C: givn-design and givn-design-review]
  C3 --> CHG
  CHG --> EB3(E: Design was written and reviewed)
  PU --> C4[C: givn-tasks]
  C4 --> CHG
  CHG --> EB4(E: Tasks were broken down)
  C2 --> CHG
  CHG --> EB5(E: Scenario was tagged as delta)
  AA([Actor: Authoring agent]) --> C5[C: RED GREEN REFACTOR loop]
  C5 --> CHG
  CHG --> EB6(E: Scenario was implemented)
  AA --> C6[C: git commit or fossil commit]
  C6 --> CHG
  CHG --> EB7(E: Revision was created for the scenario)
  AA --> C7[C: edit tasks.md]
  C7 --> CHG
  CHG --> EB8(E: Revision identifier was recorded)
  PU --> C8[C: givn-review]
  C8 --> CHG
  CHG --> EB9(E: Review was signed off)
```

### C. Archive publication

```mermaid
%% kinds: Actor/C/A/E/P/RM/X/Record
flowchart LR
  PU([Actor: Primary user]) --> C1[C: givn archive]
  C1 --> CHG[(A: Change)]
  C1 --> PRJ[(A: Project)]
  CHG --> EC1(E: Archive was requested)
  PRJ --> EC2(E: Project state was witnessed)
  CHG --> EC3(E: Accepted behavior was merged)
  CHG --> EC4(E: Evidence scope was resolved)
  GV([Actor: Givn]) --> C2[C: git commit or fossil commit]
  C2 --> CHG
  CHG --> EC5(E: Archive candidate was published)
  CHG --> EC6(E: Pending receipt was recorded)
  GV --> C3[C: verify.command and verify.e2e_command]
  TR([Actor: Test runner]) --> C3
  C3 --> CHG
  CHG --> EC7(E: Verification scopes were executed against the candidate)
  GV --> C4[C: coverage addon commands]
  C4 --> CHG
  CHG --> EC8(E: Coverage was measured and published)
  CHG --> EC9(E: Archive was finalized)
  CHG --> EC10(E: Archive was proven)
```

### D. Managed upgrade

```mermaid
%% kinds: Actor/C/A/E/P/RM/X/Record
flowchart LR
  PU([Actor: Primary user]) --> C1[C: givn upgrade]
  C1 --> PRJ[(A: Project)]
  PRJ --> ED1(E: Upgrade was requested)
  PRJ --> ED2(E: Clean state was witnessed)
  PRJ --> ED3(E: Managed scaffold was synchronized)
  PRJ --> ED4(E: Run declaration was backfilled)
  PRJ --> ED5(E: Upgrade revision was published)
  PRJ --> ED6(E: Migration plan was recorded and published)
  PRJ --> ED7(E: Maintenance change was created)
```

### E. Failure, refusal, and repair

```mermaid
%% kinds: Actor/C/A/E/P/RM/X/Record
flowchart LR
  GV([Actor: Givn]) --> C1[C: givn status and givn instructions]
  C1 --> CHG[(A: Change)]
  CHG --> EE1(E: Candidate verification failed)
  CHG --> EE2(E: Unproven candidate was surfaced)
  PU([Actor: Primary user]) --> C2[C: fix and resume givn archive]
  C2 --> CHG
  CHG --> EE3(E: Candidate was repaired forward)
  BH([Actor: Backend hook]) --> CHG
  CHG --> EE4(E: Candidate revision was refused by a hook)
  CHG --> EE5(E: README check refused the archive)
  CHG --> EE6(E: Archive run was refused)
  CHG --> EE7(E: Revision binding was checked)
```

## Reverse narrative

Walking backwards from the terminal value "archive was proven" to the
domain's beginning:

```mermaid
%% kinds: Actor/C/A/E/P/RM/X/Record
flowchart RL
  PU([Actor: Primary user]) --> C1[C: givn archive]
  C1 --> CHG[(A: Change)]
  CHG --> E1(E: Archive was proven)
  E1 --> E2(E: Proven receipt was recorded)
  E2 --> E3(E: Archive was finalized)
  E3 --> E4(E: Scopes passed against the candidate)
  E4 --> E5(E: Archive candidate was published)
  E5 --> E6(E: Accepted behavior was merged)
  E6 --> E7(E: Project state was witnessed clean)
  E7 --> E8(E: Archive was requested)
  E8 --> E9(E: Review was signed off)
  E9 --> E10(E: Scenario revisions were created)
  E10 --> E11(E: Scenarios were implemented)
  E11 --> E12(E: Tasks were broken down)
  E12 --> E13(E: Design was written and reviewed)
  E13 --> E14(E: Change was specified)
  E14 --> E15(E: Change was proposed)
  E15 --> E16(E: Project was initialized)
  E16 --> E17(E: SCM backend was selected)
  PRJ[(A: Project)] --> E17
  PRJ --> E16
```

The chain starts at the domain's beginning: no revision can exist before a
backend was selected, and no archive can be proven before the project state
was witnessed clean in that backend. The diagram is the happy path; the
refusal and repair events in section E hang off these nodes.

## Backend-divergent policies

| Policy | Git | Fossil |
|---|---|---|
| Backend detection | `.git` / `rev-parse --is-inside-work-tree` | `.fslckout` / `_FOSSIL_` |
| Clean-state witness | `status --porcelain -z -uall` (untracked counts) | `fossil status -b` for the clean/dirty verdict, `changes --differ` for entries (EXTRA included) — Q3 |
| Publishing a subset | `add -- <paths>` then `commit` (index) | `add --force <paths>` then `commit -m <msg> <paths>` (no index) |
| New file in a published subset | `add` | `add` required first; unknown path fails |
| Ignored paths | `check-ignore`, skipped | `ignore-glob`, invisible; `add --force` would include |
| Revision identifier | `rev-parse HEAD` (40 hex) | `status` checkout hash (40 hex shown, 64 hex internal) |
| Attribution | `commit-msg` hook, chained launcher | no `commit-msg` hook; givn-controlled commit path only — Q2 |
| Hook rejection repair | merge stays staged | working tree unchanged; no index |
| Root rule | project root must equal worktree top level (TC-32) | repository file lives outside the checkout; one repository may have several checkouts — Q4 |

## Term collisions

- "commit" (Git) vs. "check-in" (Fossil) vs. the domain's neutral term
  "revision" — Q10.
- "worktree" (Git) vs. "checkout" (Fossil).
- "repository root": Git worktree top level vs. the Fossil checkout root,
  while the Fossil repository file lives elsewhere — Q4.
- "hook": a Git `commit-msg` hook, a Fossil `before-commit` hook, and givn's
  own artifact `post` hooks are three different things.
- "clean": Git counts untracked files; Fossil's default does not — Q3.
- "check-in" (Fossil) vs. givn's `check` commands and mandatory checks — the
  words are unrelated.
- "Upgrade Commit A" / "Migration Commit B" (glossary) vs. the neutral
  "upgrade revision" / "plan revision" used here; the receipt fields
  `candidate_commit` / `tested_commit` have the same collision — Q10.

## Open questions raised here

- Q13: Hook-rejection repair parity — does the archive promise the same
  repair behavior when a backend hook rejects the candidate, given Git keeps
  the merge staged and Fossil has no index?
