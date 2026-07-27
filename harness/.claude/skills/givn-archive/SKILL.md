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
- Test runner: `givn missing-testrunner`

## Archive gate

`givn archive` automatically checks:

1. All `apply.requires` artifacts exist (tasks.md, review.md by default).
2. All tasks checked (`[x]`) — each with evidence and commit hash
   filled in. A mass check-in via `sed` or other text replacement is
   not completion; the review fabrication audit and component diff will
   catch it and block archive anyway.
3. `review.md` contains `REVIEW: PASS`.
4. Hooks of all `apply.requires` artifacts pass (runs `givn missing-testrunner`).

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

## Post-archive: README update

After archive succeeds, if the project has a README, update it to reflect
the current project state:

1. **Project name**: If the README still contains `{{PROJECT_NAME}}`,
   replace it with the actual project name (from `Cargo.toml`,
   `package.json`, or the directory name).
2. **Coverage section**: If coverage data is available (via
   `coverage.command` or a coverage-instrumented `givn missing-testrunner`),
   update the coverage section with the latest line and branch
   percentages. If coverage measurement is not configured, state this
   explicitly. Use `<!-- givn:begin:coverage -->` /
   `<!-- givn:end:coverage -->` blocks to allow idempotent updates.

## Verify command

Unit/integration:
```
givn missing-testrunner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
