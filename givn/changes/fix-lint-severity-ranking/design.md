# Design: fix-lint-severity-ranking

## Problem

The exit-code severity logic in `src/cli/lint.rs` assigns
`ParseError -> 1u8` and `Wip -> 2u8`, then takes the maximum. This inverts
the documented intent ("Parse errors outrank wip/PENDING") — a `@wip`
finding forces exit code 2 even when a parse error is also present.

## Solution

Invert the severity values so that `ParseError` has the higher severity:

```rust
let severity = match error.kind {
    LintKind::ParseError => 2u8,
    LintKind::Wip => 1u8,
};
```

The `max_severity` logic stays the same (take the max). The exit-code
mapping at the end of `run()` stays the same:

```rust
match max_severity {
    0 => Ok(()),
    1 => std::process::exit(1),   // now: @wip/PENDING only
    _ => std::process::exit(2),   // now: parse error present
}
```

Wait — that would swap the exit codes. The contract must NOT change:
- Exit 0 = clean
- Exit 1 = parse error
- Exit 2 = @wip/PENDING

The issue is that `max_severity` maps directly to the exit code, but
parse errors should "win" (produce exit 1) over @wip (exit 2). The current
code uses `max_severity` as the exit code directly, which means higher =
more severe. But the exit-code contract has 1 < 2, so "more severe" must
produce the LOWER number.

The fix: instead of mapping severity values to exit codes directly, use
explicit priority. Parse errors (exit 1) take priority over @wip (exit 2):

```rust
// Determine exit code: parse errors (1) outrank @wip (2).
let exit_code = if max_severity >= 2 {  // has parse error
    1
} else if max_severity >= 1 {  // has @wip only
    2
} else {
    0
};
```

Actually, the cleanest approach: keep two booleans.

```rust
let mut has_parse_error = false;
let mut has_wip = false;

for error in &errors {
    total_errors += 1;
    match error.kind {
        LintKind::ParseError => has_parse_error = true,
        LintKind::Wip => has_wip = true,
    }
    // ... print the finding ...
}

// Exit code: parse errors (1) outrank @wip (2).
let exit_code = if has_parse_error {
    1
} else if has_wip {
    2
} else {
    0
};

match exit_code {
    0 => Ok(()),
    1 => std::process::exit(1),
    _ => std::process::exit(2),
}
```

This replaces the `max_severity` counter with two booleans and an explicit
priority check. The exit-code contract is unchanged (0/1/2); only the
ranking logic changes so that parse errors always win.

## Files affected

- `src/cli/lint.rs` — replace `max_severity: u8` logic with `has_parse_error`
  / `has_wip` booleans and explicit priority.

## Step definitions

No new step definitions needed. The scenario "A parse error outranks a @wip
finding" reuses the same steps as the characterization scenario it replaces:
- `a project root with givn/ initialised` — exists
- `a .feature file at "..." with:` — exists
- `the user runs \`givn lint\`` — exists
- `the command exits 1` — exists
- `stderr contains "..."` — exists

## Verify command

```
cargo test --test features_runner
```

Already configured in `givn/config.yaml`.
