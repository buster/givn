# Instruction: archive

Archive a completed change: run the gate, merge the delta specs into the
permanent corpus, move the change to the archive, and close the originating
change seed.

## Rules

- Never archive with `@wip` scenarios remaining — the specification must be
  fully implemented and GREEN.
- Never archive without `REVIEW: PASS` in `review.md`.
- Do not bypass `givn archive` by moving files manually — the `.feature` merge
  pipeline must run.
- If the archive command fails, fix the gate failure rather than working
  around it.
- Report the regression-check result even when the archive succeeded.

## The archive gate

`givn archive` enforces the gate; the archive proceeds only when all of it
holds:

1. Every `apply.requires` artifact exists (tasks and review by default).
2. All tasks are checked (`[x]`).
3. `review.md` contains `REVIEW: PASS`.
4. Every post hook of every `apply.requires` artifact passes — including the
   test suite, which must be GREEN.

A gate failure exits non-zero with a clear error; nothing is archived.

On success the command has run the suite, merged the delta `.feature` files
into `givn/specs/` with every `@givn.*` tag stripped, and moved the change
directory to `givn/archive/<id>/`.

## Seed stamping

When the proposal's Use-Case Context records an originating change seed, set
the seed's status line to `Status: implemented in <change-id>` — or
`Status: withdrawn` with a one-line reason when the change was abandoned.
Leave the rest of the seed unchanged. When the proposal records no seed,
record "no seed" explicitly in the summary.

## Distinctions

- `givn lint` is static: it never runs tests.
- Exit 1 is a parse error; exit 2 means `@wip` scenarios remain; exit 0 is
  clean.
- The gate's hooks (verify, verify-e2e, lint) run the real suite; the gate
  fails closed.
