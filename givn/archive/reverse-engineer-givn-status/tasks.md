# Tasks: reverse-engineer-givn-status

## Task 0 — Test Infrastructure Setup

- [x] Verify `verify.command` in `givn/config.yaml` is set to a working
      Gherkin runner (e.g. `cargo test --test features_runner`).
- [x] Confirm `cargo test --test features_runner` exits 0 on the existing
      spec suite before any new step definitions are added.
- [x] Confirm `givn lint --change reverse-engineer-givn-status` exits 2
      (all scenarios @wip — expected).

---

## Scenario 1: No changes at all — text mode

- [x] Remove `@wip` from Scenario 1 only.
  - **GREEN**: Spec corrected to add `Given a project root with givn/ initialised`
    (bare dir returns "givn/ not found" not "No changes found"). Step
    definitions already existed (`stdout mentions`, `the command exits 0`).
  - **REFACTOR**: Seam: none.

## Scenario 2: No changes at all — JSON mode

- [x] Remove `@wip` from Scenario 2 only.
  - **GREEN**: Assertion changed to `stdout mentions "active"` and
    `stdout mentions "archived"` — JSON is pretty-printed (multiline) and
    full-line match with escaped quotes doesn't survive cucumber's regex.
  - **REFACTOR**: Seam: none.

## Scenario 3: Single active change listing — text mode

- [x] Remove `@wip` from Scenario 3 only.
  - **GREEN**: Wrote step def `a change "<id>" exists with artifact "<file>"`.
    `givn new` call made idempotent (checks dir exists first). Assertions
    use `stdout mentions` for robustness.
  - **REFACTOR**: Seam: none.

## Scenario 4: Multiple active changes listed alphabetically — text mode

- [x] Remove `@wip` from Scenario 4 only.
  - **GREEN**: Wrote step def `stdout line N matches "<id>"`. Fixed line
    numbers: blank line between header and first change means a-feature is
    on line 3, z-feature on line 4.
  - **REFACTOR**: Seam: none.

## Scenario 5: Active and archived changes — text mode

- [x] Remove `@wip` from Scenario 5 only.
  - **GREEN**: Wrote step def `a change "<id>" exists in archive`. Creates
    `givn/archive/<id>/` with a marker file.
  - **REFACTOR**: Seam: none.

## Scenario 6: Archived only — text mode (leading newline)

- [x] Remove `@wip` from Scenario 6 only.
  - **GREEN**: Reuses archive step. `stdout does not contain "Active changes:"`
    confirms no active section. Leading newline is AS-IS behaviour (suspected bug).
  - **REFACTOR**: Seam: none.

## Scenario 7: Active and archived changes — JSON mode

- [x] Remove `@wip` from Scenario 7 only.
  - **GREEN**: Assertions use `stdout mentions "active"` — robust against
    pretty-printed JSON formatting.
  - **REFACTOR**: Seam: none.

## Scenario 8: Detail mode — unknown change errors

- [x] Remove `@wip` from Scenario 8 only.
  - **GREEN**: Existing step defs. Assert non-zero exit + stderr contains
    "change 'nosuch' not found".
  - **REFACTOR**: Seam: none.

## Scenario 9: Detail mode — archived change (text)

- [x] Remove `@wip` from Scenario 9 only.
  - **GREEN**: Assert `stdout mentions "archived"`. Output is "Change 'old':
    archived" — substring match covers it.
  - **REFACTOR**: Seam: none.

## Scenario 10: Detail mode — archived change (JSON)

- [x] Remove `@wip` from Scenario 10 only.
  - **GREEN**: Assert `stdout mentions "\"id\": \"old\""`. `stdout_mentions`
    now unescapes `\"` → `"` for Gherkin quote escaping.
  - **REFACTOR**: Seam: none.

## Scenario 11: Detail mode — active change with proposal only (text)

- [x] Remove `@wip` from Scenario 11 only.
  - **GREEN**: Assertions use `stdout mentions` for key fields. Unicode
    markers (✓, ·, ⊘) avoided in assertions to prevent encoding issues.
  - **REFACTOR**: Seam: none.

## Scenario 12: Detail mode — active change with all required done (text)

- [x] Remove `@wip` from Scenario 12 only.
  - **GREEN**: Wrote step def `a change "<id>" exists with all required
    artifacts complete`. Creates all artifact files + PASS markers.
    Assert `stdout mentions "ready-to-archive"`.
  - **REFACTOR**: Seam: none.

## Scenario 13: Detail mode — task progress shown for tasks artifact

- [x] Remove `@wip` from Scenario 13 only.
  - **GREEN**: Wrote step def `a change "<id>" exists with artifact
    "tasks.md" containing "N done, M total"`. Writes checkbox counts.
    Assert `stdout mentions "3/5 tasks"`.
  - **REFACTOR**: Seam: none.

## Scenario 14: Detail mode — skipped artifacts shown with skip marker

- [x] Remove `@wip` from Scenario 14 only.
  - **GREEN**: Wrote step def `the change "<id>" has ".givn-skip" listing
    "<ids>"`. Creates `.givn-skip` file. Assert `stdout mentions "design"`
    and `stdout mentions "design-review"`.
  - **REFACTOR**: Seam: none.

## Scenario 15: Detail mode — active change (JSON)

- [x] Remove `@wip` from Scenario 15 only.
  - **GREEN**: Assertions use `stdout mentions` for JSON keys. Quoting
    handled by `\"` → `"` unescape in step def.
  - **REFACTOR**: Seam: none.

## Scenario 16: Uninitialized project errors

- [x] Remove `@wip` from Scenario 16 only.
  - **GREEN**: Existing step defs. Assert non-zero exit + stderr contains
    "givn/ not found".
  - **REFACTOR**: Seam: none.

## Scenario 17: Summary line includes stage label and next required step

- [x] Remove `@wip` from Scenario 17 only.
  - **GREEN**: Assertions use `stdout mentions` rather than
    `stdout contains a line` (summary line has leading spaces, trimmed
    match fails).
  - **REFACTOR**: Seam: none.

## Scenario 18: Optional pending artifacts shown when present

- [x] Remove `@wip` from Scenario 18 only.
  - **GREEN**: Rewrote step def to use `givn addons enable arc42` instead
    of raw config editing (config template has addons commented out).
  - **REFACTOR**: Seam: none.

## Spec corrections during characterization

1. **Scenario 1**: Added `Given a project root with givn/ initialised` —
   the original spec omitted it; bare dir returns error not "No changes found".
2. **JSON scenarios (2,7,10,15)**: Changed from full-line assertions to
   substring `stdout mentions` — JSON is pretty-printed multiline.
3. **Scenario 4**: Fixed line numbers (3,4 instead of 2,3) — blank line
   between header and first change.
4. **Scenario 17**: Changed from `stdout contains a line` to
   `stdout mentions` — summary line has leading indent.
5. **Scenario 18**: Changed step to use `givn addons enable arc42` —
   config template has addons section commented out.

## Step definition changes

- Added 8 new step definitions in `tests/steps/mod.rs`.
- Fixed `stdout_mentions` to unescape `\"` → `"` (Gherkin step text escaping).
- Fixed `stdout_contains_line` to use `l.trim()` for indented output.
- All `givn new` calls in step defs made idempotent.

## Post-implementation

- [x] Run `givn lint --change reverse-engineer-givn-status` — exits 0 (clean).
- [x] Run `cargo test --test features_runner` — 60/60 scenarios GREEN.
- [x] Run `givn apply --change reverse-engineer-givn-status` — confirm task tracking.