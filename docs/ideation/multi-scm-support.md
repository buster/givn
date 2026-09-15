# multi-scm-support — Ideation topic

Status: handed off 2026-09-14. Topic state lives in
`givn/ideation/multi-scm-support/`; this document is the readable record.

## Summary

givn gains one SCM seam with a Git backend and a Fossil backend, selected
automatically per project. A Git worktree uses Git; a Fossil checkout uses
Fossil; a project without either still initializes and reports status, while
archive and managed upgrade refuse by name. The archive proof keeps its
forward-only guarantee and binds the proven archive to a revision of the
active backend.

## Problem

givn was coupled to Git on three production surfaces: the archive proof
pipeline, the managed upgrade, and the commit-attribution hook installer.
Specs, architecture constraints, and the glossary all spoke Git. A Fossil
project could not use the archive or the managed upgrade at all, so projects
that want wiki, tickets, and documentation in the same artifact as the code
could not use givn's full lifecycle.

## Vision

The full lifecycle runs in a Git worktree or a Fossil checkout with the same
observable behavior, chosen automatically. Which SCM a project uses is visible
in its history, never in the workflow it must follow.

## Scope

In scope: the SCM seam with two backends; deterministic selection and named
refusals; archive publication on both backends; managed upgrade and migration
commits on both backends; attribution with an explicit Fossil path;
initialization and generated guidance per backend; domain vocabulary, specs,
Arc42 records, tests, and harness.

Out of scope: kiekste hosting (Fossil server, wiki/tickets/forum, auth) as its
own topic; Fossil features beyond givn's lifecycle; any third SCM; VCS-library
dependencies; remote and network operations.

## Personas (promoted to `givn/personas/`)

- `developer--primary-user` — uses givn in Git and Fossil projects; wants no
  regression in Git, full lifecycle in Fossil, and one mental model.
- `agent--technical-role` — the authoring agent and automation caller;
  commits per scenario, follows generated skills, needs deterministic
  commands and one machine contract.
- `givn-maintainer--owner` — owns the seam, the specs, and the architecture
  record; wants one seam, truthful documents, bounded CI cost.

## How the domain works

The chain runs from backend selection through authoring to archive
publication, managed upgrade, and failure repair. Key events: the backend is
selected; the project is initialized with backend-aware guidance; each
scenario is implemented and published as one revision; the archive publishes
the merged corpus as a candidate revision with an unbound pending receipt,
runs each verification scope once, then records the proven receipt and
publishes the evidence-only finalization revision. A failed scope leaves a
repairable unproven candidate; a backend hook rejection leaves the archive
unproven and repairable. The managed upgrade publishes an allowlist-only
upgrade revision and, when migration requirements exist, a plan revision
recording it.

## Use cases and deltas

| Use case | Delta |
|---|---|
| `archive-change` | Clean witness, candidate publication, receipt binding, evidence-only finalization, refusals, and forward repair work on the active backend |
| `maintain-project` | Clean preflight, calculated allowlist, and reviewable upgrade/plan revisions on both backends |
| `author-change` | Backend-aware initialization report and generated guidance; conditional attribution plumbing |
| `verify-change` | The recorded revision identifier is opaque (non-empty, non-placeholder, non-duplicate) |
| `inspect-change-state` | Status reports `SCM: git` / `fossil` / `none`; no-backend projects still exit 0 |
| `operate-product` | Upgrade-revision content wording becomes backend-neutral |
| fragment `scm-backend` (new) | Selection order, named refusals, and per-backend capability facts |
| fragment `support` / `llm-attribution` | `givn commit` as the SCM-neutral path; Git keeps the chained hook; Fossil is trailer-only |

## Decisions

| # | Decision |
|---|---|
| Q1 | Detection only; no backend declaration in `givn/config.yaml` in v1 |
| Q2 | `givn commit` is the SCM-neutral commit path; Git keeps the chained `commit-msg` hook as fallback; Fossil attribution only through `givn commit`; givn stamps its own revisions directly |
| Q3 | Mirrored strictness: Fossil `EXTRA` counts as dirty; "an untracked file blocks the archive" holds on both backends |
| Q4 | The project boundary is the Git worktree top level or the Fossil checkout root; the repository file location is irrelevant |
| Q5 | givn publishes the candidate revision (with the unbound pending receipt) and the finalization revision; per-scenario revisions stay with the developer/agent |
| Q6 | Root-local detection; both markers at the root is an ambiguity refusal; ancestor Fossil markers are ignored |
| Q7 | `givn init` without an SCM scaffolds and exits 0 with a named warning |
| Q8 | Managed upgrade and migration bundles run on both backends with the same guarantees; a migration-free upgrade publishes one revision |
| Q9 | Kiekste hosting stays out of scope |
| Q10 | Neutral domain terms (`revision`, `checkout`, `clean state`, `candidate revision`, `tested revision`, `revision binding`, `upgrade revision`, `plan revision`) merged into the glossary; per-SCM terms stay in the seam |
| Q11 | The recorded identifier is opaque: non-empty, non-placeholder, non-duplicate |
| Q12 | Test strategy is a design decision for the resulting changes |
| Q13 | Hook rejection promises "unproven and repairable", not an identical internal state |
| Q14 | A selected backend whose executable is missing is refused by name before any mutation |

## Change seeds (suggested order)

| # | Seed | Use case |
|---|---|---|
| 1 | SCM port + Git backend + detection (behavior-preserving; prerequisite) | fragment `scm-backend` |
| 2 | Archive on both backends | `archive-change` |
| 3 | Managed upgrade on both backends | `maintain-project` |
| 4 | Backend-aware init/guidance + `givn commit` | `author-change` + `llm-attribution` |
| 5 | Opaque revision identifier | `verify-change` |
| 6 | Status names the active backend | `inspect-change-state` |
| 7 | Upgrade-backfill wording | `operate-product` |

Seeds 5-7 may be folded into 1-3. Seed files with full decision records live
under `givn/ideation/multi-scm-support/changes/`. Handoff does not start any
change; run `/givn-propose <id>` per seed when ready.
