# Design: install-extras

## Summary

Add an `extras` system to givn: a curated set of optional, project-level
skills and commands that install into givn projects independent of the
workflow artifact graph. The change touches the Manifest data model, the
config overlay pipeline, the skills sync engine, the CLI, and the embedded
asset templates.

## Technology decisions

### Language, framework, libraries
- Rust, same as the rest of givn. No new dependencies.
- `serde` for config deserialization (`extras: bool` on Manifest).
- Line-based config editing for `extras` (follows the `set_addon` pattern
  in `src/cli/addons.rs`) — avoids clobbering user comments.
- Embedded assets via `rust_embed` (already used in `src/config/embedded.rs`).

### Test runner
- `cargo test --test features_runner` — cucumber-rs executes `.feature` files
  under `givn/specs/`. Already configured in `givn/config.yaml`.
- Step definitions live in `tests/steps/mod.rs`. New step definitions for
  extras scenarios follow the existing Given/When/Then pattern.
- Black-box: invoke the compiled `givn` binary via `common::run_givn()`,
  assert on stdout/stderr/exit code and filesystem side-effects.

### Architecture impact

| Component | Change |
|-----------|--------|
| `src/config/manifest.rs` | Add `extras: bool` field to `Manifest`. |
| `src/config/overlay.rs` | Add `extras: Option<bool>` to `OverlayDoc`. |
| `src/skills/targets.rs` | Sync reads `extras` from manifest; conditionally includes extras skills+commands. Add `extras_skill_names()`, `extras_command_names()`, `resolve_extras_*_template()`. |
| `src/skills/render.rs` | No changes — extras skills use the same `SkillContext` and `render_skill()`. |
| `src/cli/extras.rs` | **New file.** `givn extras install`, `givn extras uninstall`, and `set_extras()` internal. |
| `src/cli/init.rs` | Add `--no-extras` flag. After writing config, call `set_extras(true)` unless `--no-extras`. |
| `src/cli/skills.rs` | No change — entry point unchanged; `targets::sync()` handles the logic. |
| `src/main.rs` | Add `Extras` variant to `Commands` enum with `ExtrasCommands` sub-enum. |
| `assets/extras/skills/` | **New directory.** `givn-domain-modeling/SKILL.md.tmpl`, `givn-dev-principles/SKILL.md.tmpl`. |
| `assets/extras/commands/` | **New directory.** `grill-me.md`, `grill-with-docs.md`. |
| `assets/config.yaml.tmpl` | Add commented `extras:` section following the addons pattern. |

### Data model changes

**Manifest** (`src/config/manifest.rs`):
```rust
pub struct Manifest {
    // ... existing fields ...
    /// Enable extras skills and commands (default: false).
    #[serde(default)]
    pub extras: bool,
}
```

**OverlayDoc** (`src/config/overlay.rs`):
```rust
struct OverlayDoc {
    // ... existing fields ...
    extras: Option<bool>,
}
```

**Default manifest** (`assets/default-manifest.yaml`): add `extras: false`.

The `extras` field is **not** part of the `Addons` struct — extras are a
separate concept. Addons expand the artifact graph; extras are project-level
skills independent of the lifecycle.

### Step definition locations

All new step definitions go in `tests/steps/mod.rs`:

| Step text | Implementation note |
|-----------|-------------------|
| `givn config key "extras" is "true"` | Parse `givn/config.yaml`, assert `extras:` block has `true`. Use line-based scan (not full YAML parse) following `parse_enabled_addons`. |
| `givn config key "extras" is "false"` | Same as above, assert `false`. |
| `extras skills and commands are already generated` | Write files manually via `common::write_override`. |
| `stdout does not contain "givn-domain-modeling"` | New step: assert `stdout()` does not contain the needle. |
| `the directory "..." does not exist` | New parameterized step for directory non-existence. |
| Existing `the file "..." exists` | Already implemented — works for extras files. |
| Existing `the command exits ...` | Already implemented. |

### Skill and command naming

Extras skill names (returned by `extras_skill_names()`):
- `givn-domain-modeling`
- `givn-dev-principles`

Extras command names (returned by `extras_command_names()`):
- `grill-me`
- `grill-with-docs`

### Template resolution

Extras templates live at:
- **Embedded**: `assets/extras/skills/<name>/SKILL.md.tmpl`
- **Project override**: `givn/extras/skills/<name>/SKILL.md.tmpl`

Extras commands live at:
- **Embedded**: `assets/extras/commands/<name>.md`
- **Project override**: `givn/extras/commands/<name>.md`

Resolution follows the existing cascade: check project override first, fall
back to embedded. Implemented via new `resolve_extras_skill_template()` and
`resolve_extras_command_template()` functions in `src/skills/targets.rs`.

### Sync engine changes

`targets::sync()` currently:
1. Loads effective manifest (for `verify.command`, `features_path`).
2. Gets `default_skill_names()` → iterates, renders, writes.
3. Gets `default_command_names()` → iterates, writes.

After this change:
1. Load effective manifest.
2. Get `default_skill_names()` + conditionally `extras_skill_names()`.
3. Render and write all skills (core + extras if enabled).
4. Get `default_command_names()` + conditionally `extras_command_names()`.
5. Write all commands.

The `SkillContext` for extras skills has `change_id: "<change-id>"` — extras
templates that don't use `{{CHANGE_ID}}` simply ignore it.

### Config template (`assets/config.yaml.tmpl`)

Add a new section after addons:

```yaml
# --- Extras ------------------------------------------------------------------
# Opt-in project-level skills and commands independent of the change lifecycle.
# When enabled, `givn skills sync` includes them alongside core workflow skills.
# `givn extras install` enables + syncs; `givn extras uninstall` disables + removes.
#
# extras: true   # uncomment and set to false to disable
```

### Config editing (`set_extras`)

New function `set_extras(enabled: bool)` in `src/cli/extras.rs`, following
the exact line-based pattern of `set_addon()` in `src/cli/addons.rs`:
1. If `extras:` key already exists (uncommented) → replace value.
2. If `extras:` block exists → insert/replace key inside.
3. If no `extras:` block → append new block at end.

### Uninstall file removal

`givn extras uninstall` removes generated files by iterating over the fixed
set of extras skill and command names and deleting from each target location:

| Target | Pattern to remove |
|--------|------------------|
| `.agents/skills/<name>/` | Entire directory per extras skill |
| `.claude/skills/<name>/` | Entire directory per extras skill |
| `.opencode/skills/<name>/` | Entire directory per extras skill |
| `.opencode/commands/<name>.md` | File per extras command |

Removal is best-effort — if a file/directory is already absent, it's a no-op
(not an error). This makes uninstall idempotent.

### CLI structure

```
givn extras
  install    → set_extras(true) + skills::targets::sync()
  uninstall  → set_extras(false) + remove_extras_files()
```

`givn extras` with no subcommand prints help (clap default).

### `givn init` changes

Add `#[arg(long)] no_extras: bool` to `Init` struct.

After writing `givn/config.yaml`:
- If `no_extras` is false (default): call `set_extras(true)`.
- If `no_extras` is true: call `set_extras(false)`.

Then sync skills (existing behavior). The sync engine reads `extras` from
the just-written config and includes/excludes accordingly.

### Pitfalls

1. **Init flow ordering**: config write → set_extras → sync. If `--force`
   overwrites config, set_extras runs after so the flag isn't wiped.
2. **Overwrite on re-sync is by design.** Git tracks diffs. Matches core
   skill behavior.
3. **Extras skill frontmatter must match the directory name.** The existing
   `validate_frontmatter()` check applies to extras skills automatically.
4. **`givn init --force`** overwrites config.yaml, which would wipe the
   `extras: true` set by init. The init flow must set extras AFTER the
   force-overwrite.

## Verification

Run `givn lint --change install-extras` after writing specs.
Run `givn apply --change install-extras` after writing tasks.
All scenarios @wip until step definitions are written during implementation.