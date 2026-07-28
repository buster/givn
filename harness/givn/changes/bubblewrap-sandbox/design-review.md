# Design Review: bubblewrap-sandbox

## Grilling findings resolved

| # | Severity | Finding | Resolution |
|---|---|---|---|
| 1 | BLOCKING | `--` separator missing in build_bwrap_cmd execution flow | Fixed: execution flow inserts `["--"]` between bwrap args and agent command |
| 2 | BLOCKING | `check_bubblewrap()` only checks PATH, not namespace support | Fixed: now runs `bwrap --unshare-user -- true` to verify user namespace availability |
| 3 | BLOCKING | Sandbox delta E2E scenario missing `@e2e` tag | Fixed: added `@e2e` tag to the E2E smoke scenario |
| 4 | BLOCKING | Sandbox delta modifies unarchived test-harness delta | Documented in design.md Dependencies section — test-harness must be archived first |
| 5 | MEDIUM | Resource limit scenarios cannot orchestrate Givens | User agreed: keep as spec documentation, step implementations verify `_wrap_with_limits` command construction |
| 6 | MEDIUM | test-harness.feature E2E uses `--profile` (removed by this change) | Will be addressed when test-harness is archived — the E2E step in test-harness must be updated to remove `--profile` |
| 7 | MEDIUM | test_harness_steps.py has sandlock imports (in test-harness change scope) | Same as #6 — resolved during test-harness archive |
| 8 | MEDIUM | `_profiles_dir` in runner.py imports sandlock (test-harness change scope) | Same as #6 |
| 9 | MEDIUM | `sandboxed` field type underspecified | Fixed: `sandbox` is string enum (`"bwrap"` / `"none"`), `resource_limits` is separate field (`"systemd"` / `"none"`). E2E scenario step updated to expect `"bwrap"` |
| 10 | MEDIUM | Modified scenario title keeps "sandlock" word | Acknowledged: `@givn.modified` identifies by title — changing the title would break the match. Cosmetic issue only. |
| 11 | LOW | `~/.config` fully writable exposed credentials | Fixed: removed `--bind-try ~/.config`, added `--ro-bind-try ~/.config/opencode` |
| 12 | LOW | `~/.local` writable exposes pip packages | Accepted: agent needs writable `~/.local` for pip/uv at runtime |
| 13 | LOW | `GIVN_HARNESS_NO_SANDLOCK` env var retained despite sandlock removal | Deferred: backward compatibility |
| 14 | LOW | `/lib` and `/lib64` may be symlinks | Fixed: changed to `--ro-bind-try` |
| 15 | MEDIUM | bwrap startup failure silently reported as "completed" | Fixed: new Termination Detection section distinguishes bwrap errors from agent errors via stderr inspection |

## Design changes applied

1. **Tightened HOME exposure**: `--ro-bind $HOME $HOME` replaced with specific `--ro-bind-try` mounts for `~/bin`, `~/.cargo`, `~/.opencode`, `~/.gitconfig`, `~/.config/opencode`. No `.ssh`, no other private dirs.
2. **Removed `~/.config` writable**: Only `~/.config/opencode` is exposed (read-only) for API key config. Agent cannot read/write other config subdirs.
3. **User namespace test**: `check_bubblewrap()` verifies bwrap runtime capability, not just PATH presence.
4. **`--` separator**: Execution flow explicitly inserts `["--"]` between bwrap args and agent args.
5. **Termination detection**: Documented how `_run_with_bwrap` distinguishes timeouts, bwrap failures, and agent failures.
6. **Report fields**: `sandbox` (string enum) + `resource_limits` (string enum) replace the boolean `sandboxed`.
7. **lib/lib64 symlink resilience**: `--ro-bind-try` for `/lib` and `/lib64`.
8. **Dependency documented**: test-harness must be archived before this change.

## Lint

```
givn lint --change bubblewrap-sandbox — clean
```

DESIGN-REVIEW: PASS