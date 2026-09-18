# Change seed: scm-backend

Status: implemented in `scm-backend`

- Use-case ID: fragment `scm-backend` (new)
- Topic: `multi-scm-support`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: first; the prerequisite for the other six seeds
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Q1 — Detection only, no declaration

- Context: every repository-state command needs one answer to "which
  version-control system owns this project root?".
- Options:
  1. **Detect from project markers** — the seam resolves the backend from
     `.git`/`.fslckout`.
     In practice: a project needs no configuration; the markers are the
     source of truth.
     Gain: nothing can drift from reality.
     Cost: no way to force a backend when markers are unusual.
  2. **Declare the backend in `givn/config.yaml`** — explicit configuration.
     In practice: a project states `scm: git`.
     Gain: explicit control.
     Cost: a config migration and a second source of truth.
- Chosen: **Detect from project markers** because markers cannot drift.
  Rejected: declaration.

### Q4 — Project boundary

- Context: Fossil's repository file lives outside the checkout, so the Git
  worktree-top-level rule does not translate literally.
- Options:
  1. **The checkout root** — where `.fslckout`/`_FOSSIL_` lives; the
     repository file location is irrelevant.
     In practice: givn operates on the checkout directory.
     Gain: one rule that fits both backends.
     Cost: none.
  2. **The repository file's directory** — the Fossil repository location.
     In practice: givn would need the repository path, not the checkout.
     Gain: none.
     Cost: wrong working directory.
- Chosen: **The checkout root** because commands operate on the working tree.
  Rejected: repository-file location.

### Q6 — Mixed and nested setups

- Context: a Git repository can sit inside a Fossil checkout or vice versa.
- Options:
  1. **Probe the project root; both markers there is an ambiguity refusal;
     ancestor Fossil markers are ignored.**
     In practice: a nested Git worktree resolves to Git at its root.
     Gain: deterministic selection.
     Cost: nested mixed layouts are refused by name.
  2. **Define precedence by marker order.**
     In practice: the first marker wins silently.
     Gain: no refusals.
     Cost: silent wrong choices.
- Chosen: **Probe the project root and refuse ambiguity** because a wrong
  silent choice is worse than a named refusal. Rejected: precedence order.

### Q14 — Missing SCM executable

- Context: a marker can select a backend whose binary is not installed.
- Options:
  1. **Refuse at selection time** — name the missing executable before any
     mutation.
     In practice: `givn archive` fails with "fossil not found".
     Gain: no partial work.
     Cost: none.
  2. **Fail at first use** — let the backend error later.
     In practice: the failure surfaces mid-command.
     Gain: none.
     Cost: partial mutations.
- Chosen: **Refuse at selection time**. Rejected: fail at first use.

### Q9 — Hosting boundary

- Context: Fossil also offers a server, wiki, tickets, and forum.
- Options:
  1. **Keep kiekste hosting out of scope** — the topic covers the SCM seam
     only.
     In practice: givn talks to the local checkout; hosting is a separate
     topic.
     Gain: a bounded topic.
     Cost: hosting needs its own ideation if pursued.
  2. **Include hosting** — cover the Fossil server and its features.
     In practice: the topic grows beyond givn's boundary.
     Gain: one topic.
     Cost: scope creep.
- Chosen: **Out of scope** because hosting is a separate product boundary.
  Rejected: including hosting.

### Frame detection rule

- Context: the detection order had to be stated once for every command.
- Options:
  1. **Git worktree, then Fossil checkout, then a named refusal.**
     In practice: `init` and `status` continue without a backend; archive
     and managed upgrade refuse.
     Gain: one rule for every consumer.
     Cost: none.
  2. **Per-command detection** — each command decides.
     In practice: duplicated logic and divergent refusals.
     Gain: none.
     Cost: drift.
- Chosen: **One fixed order with named refusals**. Rejected: per-command
  detection.

## Open questions

### Q10 — Domain vocabulary

Resolved 2026-09-14: the neutral terms (`revision`, `checkout`, `clean
state`, `candidate revision`, `tested revision`, `revision binding`,
`upgrade revision`, `plan revision`) were merged into
`docs/arc42/12-glossary.md`; per-SCM terms stay in the backend seam.

### Q12 — Test strategy

Moved 2026-09-14: a design-level decision for the resulting changes, not a
domain decision.

## Scope note

Introduces the SCM port and the Git backend, and moves the duplicated Git
logic in `src/cli/migration.rs` onto the same seam without behavior change.
Adds `givn/specs/fragments/scm-backend.feature` and its fragment document.
This seed is the prerequisite for seeds 2-4.
