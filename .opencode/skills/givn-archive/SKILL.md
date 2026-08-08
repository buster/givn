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

## Post-archive: README update

After archive succeeds, if the project has a README, update it to reflect
the current project state:

1. **Project name**: If the README still contains `{{PROJECT_NAME}}`,
   replace it with the actual project name (from `Cargo.toml`,
   `package.json`, or the directory name).
2. **Coverage section**: If coverage data is available (via
   `coverage.command` or a coverage-instrumented `cargo test --test features_runner`),
   update the coverage section with the latest line and branch
   percentages. If coverage measurement is not configured, state this
   explicitly. Use `<!-- givn:begin:coverage -->` /
   `<!-- givn:end:coverage -->` blocks to allow idempotent updates.

**Example** — project-specific values (replace ` watn ` with your binary and
adjust paths and measured numbers accordingly):

```markdown
## Coverage

[![Coverage: 77.8%](https://img.shields.io/badge/coverage-77.8%25-brightgreen)](coverage/non-e2e-cobertura.xml)

Coverage runs the acceptance scenarios against the instrumented ` watn `
executable. The latest non-E2E run covers `583/749` lines (`77.8%`); the
E2E subset covers `465/749` lines (`62.1%`).

```sh
mkdir -p coverage
cargo llvm-cov clean --workspace
cargo llvm-cov run --bin watn --no-report -- --version
cargo llvm-cov test --no-clean --test features_runner \
  --cobertura --output-path coverage/non-e2e-cobertura.xml \
  -- --tags 'not @wip'
```
```

The managed coverage marker block is generated automatically during archive
and should not be manually edited. The example above is a starting point;
adapt it to your project's binary name, test runner, and coverage tool.

## Verify command

Unit/integration:
```
cargo test --test features_runner
```

E2E smoke tests:
```
verify.e2e_command (configured in givn/config.yaml)
```
