# Design: fix-lint-missing-change-error

## Problem

`givn lint --change <id>` silently exits 0 with "no .feature files found"
when the change directory doesn't exist, giving no signal that the id is
wrong.

## Solution

In `src/cli/lint.rs`, in the `--change <id>` branch, check if
`givn/changes/<id>/` exists before looking for specs. If the change
directory itself doesn't exist, return an error via `anyhow::bail!`:

```rust
} else if let Some(id) = change {
    let change_dir = project_root.join("givn/changes").join(id);
    if !change_dir.exists() {
        anyhow::bail!("change '{id}' not found");
    }
    let specs_dir = change_dir.join("specs");
    let files = collect_feature_files(&specs_dir);
    if files.is_empty() {
        println!("givn lint: no .feature files found under givn/changes/{id}/specs/");
        return Ok(());
    }
    files
}
```

`main` maps `Err` to exit code 1 + stderr message, so the observable
behaviour is: exit 1, stderr contains "change 'does-not-exist' not found".

## Files affected

- `src/cli/lint.rs` — add change-dir existence check in the `--change` branch.

## Step definitions

All steps reused: `a project root with givn/ initialised`, `the user runs`,
`the command exits 1`, `stderr contains "..."`.

## Verify command

```
cargo test --test features_runner
```
