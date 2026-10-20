# Handoff: archive-transaction-safety

Planning input for the givn agent. This is not a givn artifact. Use it to write
the specs, design and tasks; do not treat it as an approved design.

## Evidence

Real two-stage trial, bundle
`/tmp/opencode/real-runs4/20261019T160626Z-real-initial-long-r1`:

- 12 initial archive attempts, 10 failures;
- 2 follow-up attempts, 1 failure;
- archive state corruption at event `0:1667` (16:51:28):
  ```text
  archive blocked: project state is corrupt —
  'givn/changes/lending-register' and 'givn/archive/lending-register' both exist
  ```
- tested-tree rejection for change evidence at events `0:1685`, `0:1692`;
- manual `rm -rf givn/archive/<id>` was required;
- later recovery resets recreated the both-directory state.

## Source references

- `src/cli/archive.rs:45-58` — `read_status` classification and corrupt bail.
- `src/cli/archive.rs:227-232` — create archive, copy, write pending, remove
  change, publish candidate. This is the non-atomic transition.
- `src/proof.rs:43-59` — both active and archive directories are always
  `Corrupt`, even with a pending receipt.
- `src/proof.rs:279-306` — evidence path exemption omits
  `givn/changes/<id>/evidence/` and the new verification directory.
- `src/cli/archive.rs:332-339` — tested revision and dirty-path finalization.

## Likely implementation shape

The design must choose one of:

1. atomic rename of the change directory into the archive location after all
   pre-publication gates pass;
2. a persistent transaction marker that records copy, pending, commit and
   cleanup phases.

Preferred: atomic rename on the same filesystem. It removes the both-directory
window. Keep the pending receipt write and candidate publication ordered so a
failed publish is resumable.

`read_status` should distinguish:

```text
both dirs + pending receipt + identical candidate -> recoverable pending
both dirs + conflicting content                 -> corrupt
```

Evidence paths to exempt from the tested-tree check:

```text
givn/archive/<id>/**
givn/changes/<id>/evidence/**
givn/changes/<id>/verification/**
README.md                    (when readme addon enabled)
coverage outputs             (when coverage addon enabled)
```

## Acceptance criteria

- A failed copy or interrupted transition never leaves an unrecoverable
  both-directory state.
- A pending candidate can be retried without deleting either directory.
- A stale or conflicting state is reported as specific corruption, not a
  generic failure.
- A verification failure leaves a pending candidate; repair and rerun archive
  work without restoring the active change.
- Change evidence files never cause a tested-tree rejection.
- No rollback is introduced.
- Existing proven archives and legacy archives keep their current behavior.

## Risks

- Atomic rename may not work across filesystems; the design must state the
  fallback.
- SCM publication may fail after the local move; the pending receipt must remain
  the source of truth.
- Treating both directories as recoverable must not hide genuine conflicting
  edits. Compare contents and receipt state.
- The fix must not weaken the clean-tree or tested-revision guards.

## Suggested scenario areas

- interrupted archive move leaves a resumable pending candidate;
- failed archive publication can be retried;
- pending candidate plus active change resolves when identical, rejects when
  conflicting;
- change evidence changes do not reject finalization;
- verification failure rolls forward, not back;
- proven archive remains refused on a second archive.
