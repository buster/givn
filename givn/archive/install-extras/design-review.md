# Design Review: install-extras

## Phase 1 — Grilling

### Q1: Does spec cover all proposal behaviors?

Proposal promises:
- `givn init` defaults extras on ✓ (Scenario 1)
- `givn init --no-extras` ✓ (Scenario 2)
- `givn extras install` ✓ (Scenario 3)
- Idempotent install ✓ (Scenario 4)
- `givn extras uninstall` ✓ (Scenario 5)
- Idempotent uninstall ✓ (Scenario 6)
- `givn skills sync` includes extras ✓ (Scenario 7)
- `givn skills sync` excludes extras ✓ (Scenario 8)
- Error when uninitialized ✓ (Scenario 10, 11)
- Target directory coverage ✓ (Scenario 9)

Verdict: PASS — every proposal behavior has a matching scenario.

### Q2: Are there missing error scenarios?

- `givn extras install` when no config exists → error (Scenario 10).
- `givn extras uninstall` when no config exists → error (Scenario 11).
- `givn extras install` when extras template assets are missing → internal
  invariant (embedded assets). Tested by unit tests if needed, not a spec
  scenario since it can't occur in normal operation.

Verdict: PASS — both uninitialized paths covered.

### Q3: Tech choices — simpler alternatives?

**Line-based config editing vs YAML parse**: Addons already use line-based
editing to preserve comments. Extras follows the same pattern. A full
YAML round-trip (parse → modify → serialize) would strip comments, which
is user-hostile for a config file with 100+ lines of documentation.

**Hard-coded file removal list vs tracking manifest**: For 4 extras (2 skills,
2 commands), a hard-coded list is simpler and sufficient. A tracking manifest
would require maintaining an extra file and adds complexity without benefit.

**Separate `extras_skill_names()` vs extending `default_skill_names()`**:
Keeping them separate makes the conditional logic explicit in `sync()`. If
they were merged, callers would need to filter. Separation is clearer.

Verdict: All choices are the simplest thing that works.

### Q4: Testability — can every scenario fail in RED?

- Scenario 1: Before implementation, no extras skills exist → steps asserting
  file existence will fail → RED. ✓
- Scenario 2: Before impl, `givn init --no-extras` still generates extras
  (current behavior) → steps asserting file absence fail → RED. ✓
- Scenario 3: Before impl, `givn extras` command doesn't exist → "unexpected
  argument" error, exits non-zero → RED. ✓
- All scenarios: RED achievable.

Concern: "givn config key 'extras' is 'false'" — need to ensure the init
root fixture writes `extras: false` for the false-asserting tests. The step
will need to line-edit the config to set the value.

Verdict: All RED-able.

### Q5: Risk — most likely failure mode?

**Biggest risk**: The init flow timing. `givn init` writes config → sets extras →
syncs skills. If any step fails, the project could be left in a partial state
(extras enabled but skills not generated, or vice versa). Mitigation: the
sync warning pattern (already exists for init) handles this gracefully — init
prints a warning and tells the user to run `givn skills sync` later.

**Second risk**: Config template drift. The config template must document
extras (for `givn upgrade` to preserve). If the template and code disagree
on the default, users get confusing behavior. Mitigation: the existing
`config_template_keys_match_manifest_schema` test catches schema drift. We
add `extras` to the documented keys list in that test.

Verdict: Risks are acceptable with existing mitigations.

### Q6: Extras vs addons boundary — is it clear?

Addons (arc42): expand the artifact graph, appear in `givn status`, gate
archive, have per-change artifacts.

Extras: project-level skills/commands, never appear in `givn status`, don't
gate archive, don't create artifacts.

The `extras` field lives on `Manifest` directly (not inside `Addons`).
This is the clearest signal in code that they are different concepts.

Verdict: Boundary is clean.

### Q7: Frontmatter validation — extras skills pass?

Extras skill templates will have YAML frontmatter with `name:` matching the
directory name. Example: `assets/extras/skills/givn-domain-modeling/SKILL.md.tmpl`
must have `name: givn-domain-modeling`. The existing `validate_frontmatter()`
is called in `sync()` for every skill — extras included.

Verdict: No special handling needed.

### Q8: `givn init --force` interaction?

When `--force` is used, init overwrites `givn/config.yaml` with the template,
THEN sets extras, THEN syncs. The order matters: overwrite first, then set,
then sync.

Current code flow:
1. Create dirs.
2. Write config (or skip if exists and !force, or overwrite if force).
3. Sync skills.

New flow:
1. Create dirs.
2. Write config (or skip if exists and !force, or overwrite if force).
3. `set_extras(!no_extras)` ← NEW.
4. Sync skills ← reads extras flag from step 3.

If `--force` is not used and config exists, step 2 skips writing but step 3
still sets extras. This is correct — `givn init` is idempotent and should
ensure extras config matches the flag.

Verdict: Order is correct.

## Phase 2 — Hardening

### Changes made

1. **Added missing scenario**: `givn extras uninstall` errors when givn/ is not
   initialised (Scenario 11).

2. **Design clarifications**:
   - Explicitly documented init flow ordering (config → set_extras → sync).
   - Added `extras` to the documented keys for schema validation.
   - Confirmed frontmatter validation applies automatically.

3. **No spec changes for grill-me/grill-with-docs content** — the body of these
   commands is implementation detail. The spec only asserts they are generated
   as files. Their content quality is verified by code review.

## DESIGN-REVIEW: PASS