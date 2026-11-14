# Proposal: add-ise-clean-room-commands

## Release note

`givn ise replace` materializes a clean-room trial from a recorded revision on Git or Fossil, and `givn ise verify` re-checks its frozen retained and `e2e` inputs per path.

## Use-Case Context

- Use-case ID: `prepare-an-ise-test` (`clean-room-preparation`,
  `trial-input-verification`)
- Ideation topic: `ise-rebuildability`
- Seed path: `givn/ideation/ise-rebuildability/changes/add-ise-clean-room-commands.md`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

The seed records no open question. Its accepted decisions bind this change:
`Q3` (the two commands), `Q4` (set arithmetic on named paths), `Q5` (only the
`End-to-end (E2E) test` package is frozen), `Q13` (a plain-directory `Ise
Test`), `Q5 and Q13` (qualify, then freeze), and `S4` (`--into` is required
and outside the checkout). This proposal does not reopen them.

## Review Context

- Review kind: independent
- Reason: major change. It adds a user-facing command pair, a durable trial
  record with its own content scope, a revision-level file reader that must
  agree on Git and Fossil, and a write path that materializes project content
  outside the checkout. Reversal changes the harness contract and every
  prepared trial record.

## Problem / Opportunity

An `Ise Test` has no materialization command. A developer must create a
worktree or a Fossil checkout and delete the replaced paths by hand. A
worktree carries history, and a Fossil checkout carries the repository
database, so either hands the builder the old `Source code`. A manual script
cannot prove that it withheld exactly the named paths, that it copied only the
declared `Specification`, or that the frozen `e2e` package on the verifier's
side is the one the builder received. The boundary check proves that a
declared package is isolated, but it produces no trial and no record of the
trial's inputs. The clean-room guarantee therefore rests on an untested
script, and an independent verifier has no per-path evidence that the trial
was not edited.

## Proposed Solution

Observable behavior, from the developer's and verifier's side:

- **Replace a trial.** `givn ise replace [--at <revision>] (--full |
  --path <path>...) --into <dir> [--receipt <path>] [--dry-run]` reads one
  selected revision (`HEAD` when omitted), reads the declaration, partitions
  the tracked paths into the retained roles, the kept source outside the named
  replacements, the named replacements, and the paths outside the declaration,
  and writes a plain directory outside the checkout.
- **Exactly the named paths.** The trial contains every retained path and
  every kept-source path. It omits every path named with `--path` (a tracked
  file, or every tracked file beneath a named directory) and every tracked
  file under every declared `source:` root under `--full`, and every tracked
  file outside the declaration. Deletion is set arithmetic: no unit test is
  inferred from `#[cfg(test)]`, a file-name convention, or a stack marker, and
  a glob is not expanded. Exactly one of `--full` or `--path` is required;
  neither, both, and an omitted `--into` are usage errors.
- **A plain directory.** The trial carries no version-control metadata and no
  ignored or untracked file, because preparation reads the revision, not the
  working tree. Relative paths, the executable bit, and symlink targets are
  preserved.
- **Nine refusals before any write.** Preparation refuses by name on a missing
  or malformed declaration, `ise` or `architecture_principles` off, a dirty
  checkout, a missing, mixed, or nested backend, a retained path missing at
  the revision, a `--path` value outside every `source:` root, a `--path`
  value or directory expansion matched by a retained role, a write target
  inside the checkout or already existing (including a receipt inside the
  trial or an ancestor of it), and a repository link among the copied files.
  Each refusal names its condition and subject, exits nonzero, and leaves the
  checkout and the target unchanged.
- **Report and record.** The report prints the ten fixed lines `Backend`,
  `Revision`, `Retained`, `Kept source`, `Replaced`, `Outside the
  declaration`, `Not replaced`, `Retained digest`, `E2E digest`, and `Written
  path`, plus `Receipt path` when a receipt path was given. The trial record
  carries the revision, the named paths, the counts, the retained, `e2e`, and
  replaced identities, the backend, and the running Givn version.
- **Re-read before recording.** Preparation re-reads every written file,
  compares it to the selected revision, and writes the record only after every
  file matches. A mismatch names the drifting paths and writes no record.
- **Verify on request.** `givn ise verify --trial <receipt> --at <dir>`
  confirms the record belongs to this trial and this backend, then re-checks
  the retained and `e2e` identities per path against the written trial and the
  current `e2e` package. A changed component step definition outside the
  frozen inputs produces no drift. A changed retained or `e2e` file fails and
  names the path.
- **Dry run.** `--dry-run` performs every read, the partition, every refusal,
  the counts, and the identities, writes no directory and no record, and
  prints the ten fixed lines, the optional receipt path, and the `Would write:`
  block.
- **No receipt means stdout.** An omitted `--receipt` prints the trial record
  to stdout, prints no `Receipt path` line, and writes nothing, so verification
  requires an explicit record path.
- **Backend parity.** Git and Fossil produce the same partition and report
  shape. A backend-specific read failure refuses before any write.

## Capability Routing

`givn spec route --change add-ise-clean-room-commands` reports no signal; its
top candidates tie. The table keeps the gate's four columns and records the
no-signal result in the recommendation column, then the decision and its
rationale.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `clean-room-preparation` | `no signal` | `NEW in prepare-an-ise-test` | No permanent capability materializes a trial. The boundary check refuses a change; it writes no directory and no record. The ideation handoff assigns this capability to the new leaf. |
| `trial-input-verification` | `no signal` | `NEW in prepare-an-ise-test` | No permanent capability re-checks a prepared trial's frozen inputs. Change-receipt freshness proves a change, not a trial directory, and cannot name a path that changed inside the trial. The handoff assigns this capability to the same leaf. |

## Out of Scope

- The stack instruction split, `steps-rust`, and the instruction line budget
  (`split-stack-instructions`, plan `S5`).
- Enabling `ise` on this repository, declaring Givn's own `Specification`, and
  the self-rebuild (`move-givn-e2e-to-black-box-package`,
  `enable-ise-on-givn-and-run-a-dry-run`, plan `S13`).
- Harness task variants, benchmark runs, and paid campaigns (plan `S6b` to
  `S14`).
- Traffic replay, builder containment, and proving what the builder read.
- Unit-test inference, a per-component file list, a `components:` key, phrase
  uniqueness, and a blast-radius gate.
- A branch or worktree mode for the trial. The plain directory is the settled
  model.
- Running the frozen `e2e` suite. The independent verifier runs it outside
  Givn.

## Open Questions

### D1: What `--path` names and what the replacement counts mean

- Question: Does `--path` name a tracked file, a directory, or a glob pattern,
  how is a whole-source replacement expressed, and how are `Replaced` and
  `Not replaced` counted when a named path is absent from the selected
  revision? The nine settled refusals do not include an absent replacement
  path, so the command needs a rule before tasks can assert the report.
- Options:
  1. **A tracked file or a directory, plus a separate `--full`** — a named
     file is removed; a named directory removes every tracked file beneath it
     at the revision; `--full` removes every tracked file under every declared
     `source:` root; exactly one form is required; a glob is never expanded; a
     named path absent from the revision removes nothing; `Replaced` counts
     the tracked files actually removed, and `Not replaced` counts the direct
     tracked files in those files' parent directories that the developer did
     not name; gain: one flag for a component and one for a whole product
     without matching semantics, and a directory cannot silently leave its
     unnamed files in place; cost: a directory that contains a retained path
     refuses, so the developer names the narrower directory; in practice:
     `--path crates/x/src/keys` removes `parser.rs` and `mod.rs`, while
     `--full` removes the whole product.
  2. **Exact tracked file paths only** — a named file is removed; a directory
     or a glob removes nothing; `--full` removes every tracked file under the
     source roots; gain: the narrowest deletion rule; cost: a component with
     several files needs one flag per file; in practice: `--path
     crates/x/src/keys` prints `Replaced: 0` and leaves the component files in
     the trial.
  3. **Glob patterns** — a named pattern expands against the revision; gain:
     compact invocations; cost: a glob matches files the developer did not
     inspect and introduces matching semantics into deletion; in practice:
     `--path 'crates/x/src/**'` matches every file under the component.
- Disposition: carried — the `design.md` Decisions ledger decides it under the
  owner named there; the deciding artifact is `design.md`.

### D2: Where the trial record may be written

- Question: May the receipt path sit inside the trial, may it lie inside the
  checkout, and may it overwrite an existing record? The record is the
  verifier's trust anchor, and the trial is the builder's write authority, so
  the answer fixes whether verification can trust the record it reads.
- Options:
  1. **Outside the trial and the checkout, and absent** — both write targets
     are checked before any write; the trial must not exist, the receipt must
     not exist, and the receipt must not lie inside the trial; gain: the record
     cannot be edited with the trial and the checkout stays unwritten; cost:
     one more path rule at the target check; in practice:
     `--receipt /tmp/ise-trial.yaml` is accepted, and `--receipt
     /tmp/ise-trial/record.yaml` is refused.
  2. **The record may sit inside the trial** — gain: one directory holds the
     trial and its record; cost: the builder can edit the record with the
     trial, so a verifier reading that record proves nothing; in practice: the
     record is `--into`/record.yaml.
  3. **An existing record may be overwritten** — gain: repeated runs need no
     cleanup; cost: evidence from an earlier trial is destroyed without a
     trace; in practice: a second prepare overwrites the first record.
- Disposition: carried — the `design.md` Decisions ledger decides it under the
  owner named there; the deciding artifact is `design.md`.

### D3: Which configuration decides `ise` effectiveness for these commands

- Question: Do `givn ise replace` and `givn ise verify` read `addons.ise` from
  the effective project manifest, from a change's `config-lock.yaml`, or from
  both? The commands take no `--change` argument, and the boundary gate uses
  the change lock, so the two authorities can disagree.
- Options:
  1. **The effective project manifest** — gain: the commands are
     project-scoped and usable without an active change; cost: they ignore a
     change lock that a change-scoped gate would honor; in practice:
     `givn/config.yaml` has `addons.ise: true`, so prepare runs while an
     active change's lock says `ise: false`.
  2. **The change lock** — gain: one authority with the boundary gate; cost:
     the command would need a current change and would refuse outside one; in
     practice: prepare fails in a project with no active change.
  3. **Both must say true** — gain: the strictest posture; cost: a
     project-level materialization is blocked by an unrelated change lock; in
     practice: a change created before `ise` was enabled blocks every trial.
- Disposition: carried — the `design.md` Decisions ledger decides it under the
  owner named there; the deciding artifact is `design.md`.

### D4: How the identities are recorded and how the report abbreviates them

- Question: Does the trial record store one rollup digest per identity, a
  per-path digest map plus a rollup, or the frozen contents themselves? The
  verifier must name drift per path, and the report prints an abbreviated
  `Retained digest` and `E2E digest`.
- Options:
  1. **A per-path digest map plus a rollup** — the record carries one digest
     per retained and `e2e` path and a rollup for the report; the report
     prints the first 12 hex characters of the rollup with `…`; gain:
     verification names each drifting path and the record stays compact; cost:
     a larger record than one digest; in practice: a changed
     `givn/specs/x/usecase.md` is named from its own digest.
  2. **One rollup digest per identity** — gain: the smallest record; cost:
     verification cannot name a path, which contradicts the per-path
     guarantee; in practice: verification reports that the retained set
     differs but cannot say which file.
  3. **The frozen file contents** — gain: the strongest self-contained record;
     cost: the record duplicates the `Specification` and becomes a second
     corpus; in practice: a small project produces a multi-megabyte record.
- Disposition: carried — the `design.md` Decisions ledger decides it under the
  owner named there; the deciding artifact is `design.md`.
