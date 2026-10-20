# Spike procedure: readability-experiment

Status: active 2026-10-19. This is a manual spike, not a Givn change. If the
experiment succeeds, the capability becomes a normal change.

## Scope

- Projects and pinned revisions:
  - `/home/buster/projects/givn` at `89e5423deb94fbd97775b484cd2aec14d8ef257f`
  - `/home/buster/projects/watn` at `9b2fc8455d9a5e145d69d9a01764ad874c6eee95`
  - `/home/buster/projects/kiekste` at `1be22f70d2a51ec2d94e626d7c4de735866132ea`
- Production Rust sources only: every `.rs` file outside a `tests/` directory.
  Tests, test support, and tooling are excluded.
- Inventory: 208 files, 121,708 physical lines.

## Four rules

### R1 — Outline comprehension

A unit passes when a reader given only its outline can state (a) the unit's
responsibility and (b) why its internal calls happen in that order. The outline
contains the public items, their signatures, and the ordered list of internal
calls each public item makes; it contains no method bodies and no comments.

A finding is recorded when the outline leaves the responsibility ambiguous, the
call order is not explainable without reading bodies, or the unit has no
coherent responsibility.

### R2 — Expressive names

A method or function name states the intent and the object of the action.
Generic verbs without a domain object — `handle`, `process`, `do`, `run`,
`update`, `manage`, `check`, `get`, `set` — are violations when a more specific
name is possible. A name that describes a mechanism rather than the intent, or
that cannot be distinguished from its siblings, is a violation.

### R3 — Single responsibility

A method or function does one thing at one level of abstraction. A violation
mixes unrelated concerns (for example parsing, persistence, and formatting), or
calls internal helpers from different layers, or needs "and" to describe what it
does.

### R4 — Source-unit size

A production source unit is a Rust file/module. A unit over 500 physical lines
(`wc -l`) is a finding. Severity by excess: 501-750 low, 751-1,000 medium,
1,001-5,000 high, over 5,000 critical. Blank lines and comments count; this is
the same measure used for the inventory.

## Severity

Within each rule: **critical** (public API incomprehensible, central public
generic name, file over 5,000 lines), **high** (public API violation, method
mixing three or more concerns, file 1,001-5,000 lines), **medium** (internal
violation with multiple callers, file 751-1,000 lines), **low** (private helper,
file 501-750 lines). Rank by severity, then by public reach (public API over
internal), then by size or fan-in.

## Finding record

Every finding carries: ID (`R1-givn-01`), project, file:line, unit/method,
rule, severity, outline evidence, recommended action, and whether it overlaps
another rule. A candidate finding becomes a selected finding when it is in the
top three per rule per project.

## Worktrees and fixes

- One worktree per project: `/home/buster/projects/readability-spike/<project>`.
- Branch: `readability-spike-2026-10-19`, based on the pinned revision.
- One commit per distinct fix. A fix that resolves overlapping findings cites
  every rule. Dependencies between fixes are recorded in the commit body.
- Commit subject: `readability(spike): <finding-id> <short summary>`.
- Every commit must pass `cargo check`. Run the project's test command when it
  is available and reasonably fast; record the result.
- The pinned main checkouts never move. No Givn product behavior changes.

## Reports

- `reports/<project>.md` — candidate findings, selected findings, evidence, and
  recommended actions.
- `reports/<project>-fixes.md` — commit hashes, files changed, build result, and
  test result.
- `reports/diff-index.md` — how the owner inspects each worktree and commit.

## Owner judgment

The owner opens each worktree, reads the code and the per-fix commits, and
judges whether the code improved. The owner accepts or rejects each fix and
decides whether the readability check is promoted, kept opt-in, or retired. The
spike does not merge the fixes into the pinned main checkouts.
