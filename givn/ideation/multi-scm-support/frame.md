# Frame: multi-scm-support

## Problem

givn is coupled to Git on three production surfaces: the archive proof
pipeline, the managed upgrade, and the commit-attribution hook installer. The
permanent specs, the architecture constraints, and the domain glossary all
speak Git. A project versioned with Fossil cannot use the archive or the
managed upgrade at all. Fossil's mechanics cover the proof model's read and
publish operations (status, partial check-in, revision identity); the
transparent attribution hook is the one capability it does not offer. Projects
that want wiki, tickets, and documentation in the same artifact as the code
therefore cannot use givn's full lifecycle.

## Vision

givn runs its full lifecycle in a Git worktree or a Fossil checkout with the
same observable behavior, chosen automatically: Git when the project is a Git
worktree, Fossil when it is a Fossil checkout, a named refusal when it is
neither. The archive proof keeps its forward-only guarantee and binds the
proven archive to a revision of the active SCM. Which SCM a project uses is
visible in its history, never in the workflow it must follow.

## Scope boundary

In scope:

- One SCM seam with a Git backend and a Fossil backend.
- Deterministic backend selection (see confirmed rule below) and a named
  refusal when neither SCM is present.
- Archive publication (candidate revision, clean-state witness, receipt
  binding, forward repair) on both backends.
- Managed upgrade and migration commits on both backends.
- Commit attribution on both backends, including an explicit decision for the
  Fossil parity gap (`commit-msg` hook is not implemented in Fossil).
- `givn init` / `givn skills sync` behavior in both SCMs.
- Domain vocabulary, specs, Arc42 constraints/ADRs/glossary, agent guidance,
  tests, and the Python harness for both backends.

Out of scope:

- Kiekste hosting (Fossil server, wiki/ticket/forum UI, auth, sync policy);
  that is kiekste's own domain and gets its own topic if needed.
- Fossil feature parity beyond what givn's lifecycle uses (chat, technotes,
  mirroring, RBAC, `fossil all`).
- Any third SCM (Mercurial, SVN, Jujutsu) and any VCS-library dependency
  (`git2`/`gix`); backends shell out to the SCM CLI, as Git does today.
- Remote/network operations (clone, push, SCM remote sync).

## Detection rule (confirmed 2026-09-14)

1. Project root is inside a Git worktree (`.git` / `git rev-parse
   --is-inside-work-tree`) -> Git backend.
2. Otherwise, project root is a Fossil checkout (`.fslckout` or `_FOSSIL_`)
   -> Fossil backend.
3. Otherwise -> no SCM: archive and managed upgrade refuse with a named
   error, as they do today for a non-Git project.

Selection and root equality are separate concerns: the Git backend keeps the
TC-32 rule that the project root must equal the worktree top level, so a
nested project selects Git and is then refused by the archive exactly as
today. A Git worktree inside a Fossil checkout selects Git by this rule;
whether that setup should instead be an explicit refusal is open (Q6). A
checkout whose SCM binary is absent is not covered by this rule and must
still produce a named refusal rather than a raw spawn failure (Q14, G11).

## Stakeholders

Consolidated by user decision on 2026-09-14: one persona per type, three
types. The developer and project maintainer roles merge into the primary
user; the authoring agent and the automation caller merge into the technical
role; the givn maintainer is one persona. The Fossil-hosted audience was
dropped as a non-actor.

| Stakeholder type | Interest |
|---|---|
| Primary user (developer / maintainer) | Uses givn in Git and Fossil projects: archives changes, runs managed upgrades, crosses release gaps. Wants one mental model, no regression in Git, full lifecycle in Fossil, reviewable automatic commits, and the same failure behavior in both. |
| Technical role (authoring agent / automation caller) | Commits once per scenario, records the revision, follows generated skills, and drives givn through its machine contract. Needs deterministic commands, honest SCM-specific guidance, and one stable machine interface. |
| Givn maintainer (this repository) | Owns the portable seam, the specs, the constraint/ADR register, and the test harness. Wants one seam without duplicated SCM logic, truthful architecture records, and bounded CI cost. |

## Research protocol

### What exists

- `src/git.rs` is the only SCM abstraction, but its API is Git-shaped:
  `Worktree::open` (`rev-parse --is-inside-work-tree`, `--show-toplevel`),
  `head` (`rev-parse HEAD`), `dirty_entries` (`status --porcelain -z -uall`),
  `commit` (`check-ignore`, `add --`, `commit -m`).
- `src/cli/migration.rs` re-implements staging, status, commit, and hash
  resolution for the managed upgrade, independently of `src/git.rs`.
- `src/cli/git.rs` installs the attribution hook (`rev-parse --git-path
  hooks`, launcher that chains an existing `commit-msg`).
- Archive moves the change directory with plain filesystem copy and delete
  (`src/cli/archive.rs`), never `git mv`; no branch, stash, or worktree-command
  logic exists anywhere.
- Git concepts leak into artifacts: `COMMIT:` lines in `tasks.md`
  (`src/integrity_hook.rs`), `candidate_commit`/`tested_commit` in the
  verification receipt (`src/proof.rs`), `upgrade_commit` in `migration.yaml`.
- `givn/config.yaml` has no SCM key; there is no detection logic and no
  upward root search (the current directory is the project root).
- Test harness: `make_init_root` Git-inits nearly every fixture
  (`tests/common/mod.rs`), used across roughly 24 step files; the Python
  harness runs `git init` and validates Conventional Commits via `git log`.

### What the permanent specs already own

- `givn/specs/archive-change/archive-proof.feature` (20 scenarios) owns the
  committed-state witness, the untracked-file block, the Git requirement, the
  project-root-is-repository-root rule, candidate publication, receipt
  binding, hook rejection, and forward repair.
- `givn/specs/maintain-project/release-0-3-0.feature` owns the clean-Git
  preflight, the two-commit managed upgrade, and the non-Git protection.
- `givn/specs/fragments/llm-attribution.feature` and
  `llm-attribution-installation.feature` own trailer stamping and hook
  installation.
- `givn/specs/operate-product/run-command.feature` owns upgrade-commit
  contents observed through the upgrade flow (`givn upgrade` assertions on
  the latest commit).
- Constraints TC-21, TC-27, TC-31, TC-32 name Git; ADR-070 governs the
  forward-only archive proof; ADR-023 governs the versioned upgrade.

### Fossil probe (2.26, local)

- `fossil status -b` prints exactly `clean` or `dirty`; `fossil changes
  --classify` prints line-oriented change types; the `status` header carries
  the checkout hash (40 hex shown, 64-hex artifact hash internally).
- `fossil commit -m <msg> <paths>` commits exactly the named paths; new files
  need `fossil add` first; ignored files are invisible unless `add --force`.
- Untracked files appear as `EXTRA` only with `changes --differ` and do not
  block a commit.
- Hook types accepted by Fossil 2.26 are `after-receive`, `before-commit`, and
  `disabled`; `commit-msg` is rejected as invalid. `before-commit` runs before
  the check-in message exists and cannot rewrite it. `amend`, `stash`, and
  `cherrypick` exist; `json` is absent in the Debian build.
- The repository is a single SQLite file outside the checkout; one repository
  can be opened into several checkouts.

### Gaps

| ID | Gap |
|---|---|
| G1 | No SCM port; two independent Git implementations (`src/git.rs`, `src/cli/migration.rs`). |
| G2 | No backend detection and no SCM configuration key. |
| G3 | Domain vocabulary (commit, worktree, HEAD, repository root) is Git-specific across specs, glossary, and skills. |
| G4 | Attribution has no Fossil equivalent: no `commit-msg` hook. |
| G5 | Clean-state semantics differ: Git counts untracked files (`-uall`), Fossil reports them as `EXTRA` only with `--differ`. |
| G6 | TC-32 (project root must equal repository root) has no direct Fossil analogue. |
| G7 | Tests and the Python harness are hardwired to Git fixtures. |
| G8 | Receipt and integrity formats encode "commit" in field names and hash-shape assumptions. |
| G9 | Documentation, constraints, and specs that assert Git must be amended to state the SCM boundary truthfully. Enumerated: TC-21 and TC-31 (Git-required archive/upgrade), TC-32 (Git-root rule), TC-27 (Git hook plumbing), `archive-proof.feature` scenarios "Archive requires Git version control" and "Archive requires the project root to be the repository root", `release-0-3-0.feature` "non-Git project is protected", and the glossary terms built on Git commit/worktree. |
| G10 | The kiekste hosting boundary is not yet drawn (parked as out of scope). |
| G11 | A checkout whose SCM binary is absent is not detected: filesystem markers select the backend, but every operation shells out, so a missing `git`/`fossil` binary surfaces as a raw spawn failure instead of a named refusal (Q14). |
