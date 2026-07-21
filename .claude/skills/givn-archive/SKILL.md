---
name: givn-archive
description: Archive a completed givn change — runs gate hooks, merges delta specs into permanent specs, and moves the change to archive/.
---

# givn-archive

Archive change `<change-id>`.

## Context

- Change: `givn/changes/<change-id>/`
- Archive target: `givn/archive/<change-id>/`
- Permanent specs: `givn/specs/`
- Test runner: `cargo test --test features_runner`

## Archive gate

`givn archive` automatically checks:

1. All `apply.requires` artifacts exist (tasks.md, review.md by default).
2. All tasks checked (`[x]`) — each with evidence and commit hash
   filled in. A mass check-in via `sed` or other text replacement is
   not completion; the review fabrication audit and component diff will
   catch it and block archive anyway.
3. `review.md` contains `REVIEW: PASS`.
4. Hooks of all `apply.requires` artifacts pass (runs `cargo test --test features_runner`).

If any gate fails, the archive is refused with a clear error. Do **not**
bypass a gate failure by mass-checking boxes — go back, genuinely
complete each task (RED→GREEN→REFACTOR→COMMIT with evidence), then
re-attempt archive.

## Steps

1. Confirm all tasks are done:
   ```
   givn status --change <change-id>
   ```

2. Confirm review is signed off:
   ```
   givn check review --change <change-id>
   ```

3. Archive:
   ```
   givn archive --change <change-id>
   ```

This will:
- Run the gate hooks (test suite must be GREEN).
- Merge delta `.feature` files into `givn/specs/` (strips `@givn.*` tags;
  preserves `@e2e` tags — they are not `@givn.*` tags).
- Move `givn/changes/<change-id>/` to `givn/archive/<change-id>/`.

## Post-archive: README coverage update

After archive succeeds, if the project has a README and
`cargo test --test features_runner` produces coverage data, run it one final time and
update the README with the latest coverage stats (line and branch
percentages from the summary output). Example update format:

```markdown
## Coverage

Current: 87% line, 82% branch (measured via cargo llvm-cov).
```

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
