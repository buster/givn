# Design: fix-lint-exit-handling

## Problem

`lint::run()` calls `std::process::exit(1)` and `std::process::exit(2)`
directly, bypassing `main`'s anyhow error handler. The source comment is
misleading.

## Solution

Change `run()` to return `Result<i32>` where the `i32` is the exit code
(0, 1, or 2). The dispatch layer in `main.rs` maps the returned code to
`std::process::exit(code)`.

### `src/cli/lint.rs`

Change signature from `pub fn run(...) -> Result<()>` to
`pub fn run(...) -> Result<i32>`.

Replace the exit-code mapping at the end:

```rust
let exit_code = if has_parse_error {
    1
} else if has_wip {
    2
} else {
    0
};
Ok(exit_code)
```

Also, the early returns that currently `return Ok(())` need to return
`Ok(0)`.

Remove the misleading comment about process exit handling.

### `src/main.rs`

Change the dispatch line:

```rust
Commands::Lint { change, paths } => {
    let code = cli::lint::run(change.as_deref(), paths)?;
    if code != 0 {
        std::process::exit(code);
    }
}
```

## Files affected

- `src/cli/lint.rs` — return `Result<i32>` instead of calling
  `std::process::exit`.
- `src/main.rs` — map the returned code to `std::process::exit`.

## Step definitions

All steps reused.

## Verify command

```
cargo test --test features_runner
```
