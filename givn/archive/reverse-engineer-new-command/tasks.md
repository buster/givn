# Tasks — reverse-engineer-new-command

> **Amendment (post-archive):** The original tasks below used a parallel
> `#[test]` file (`tests/new_command.rs`) run via `cargo test`. This was
> superseded by the executable-spec approach: the `.feature` file is now run
> by `cucumber-rs` via `cargo test --test features_runner`, with step
> definitions in `tests/steps/mod.rs`. The per-scenario characterization
> intent (GREEN-first, probe observed behaviour) is unchanged; only the
> execution mechanism changed. References to `tests/new_command.rs` and
> `cargo test` describe the original (now-replaced) mechanism.

Characterization task breakdown. Each scenario task is a **GREEN-first**
characterization cycle: write a test asserting the **observed** behaviour of
the real binary, then run `cargo test` and confirm it passes immediately.
If it fails, fix test plumbing or correct the spec — never the production
code. REFACTOR steps apply a testability seam only where design.md calls
for one (it calls for none here); otherwise REFACTOR is skipped.

Test runner: `cargo test` (configured in `givn/config.yaml`).
Spec: `givn/changes/reverse-engineer-new-command/specs/new-command/new-command.feature`
Test functions: `tests/new_command.rs` (one function per scenario, named after
the scenario, with a `// Scenario: <title>` line for traceability).
Shared helpers: `tests/common/mod.rs`.

---

## [x] Task 0 — Test infrastructure setup

- [x] Confirm `cargo test` runs and exits 0 on the current project (no
      existing tests yet — expect "0 passed" or an empty run, still exit 0).
- [x] Set `verify.command` in `givn/config.yaml` to `cargo test` (replacing
      the `givn missing-testrunner` sentinel). Done as part of this change.
- [x] Add `tempfile` as a dev-dependency in `Cargo.toml` (characterization
      is failure-heavy; `tempfile::TempDir` auto-cleans up on drop, including
      on panic, preventing leaked temp dirs).
- [x] Create `tests/common/mod.rs` with a reusable fixture (e.g. `TestEnv`
      or `temp_givn_root()`) that:
      - creates a fresh `tempfile::TempDir`;
      - runs `givn init` with that temp dir as CWD (MUST run `givn init`
        rather than hand-creating a minimal `givn/` subtree, so scaffolding
        logic is not duplicated in test setup);
      - returns a handle carrying the `TempDir` (so it lives as long as the
        test needs it).
      Expose helpers: `make_bare_root()` (a `TempDir` with no `givn/`),
      `make_init_root()` (the default fixture with `givn init` already run),
      `run_givn(env, args) -> Output`, and
      `write_override(env, artifact, file, content)` (which must
      `create_dir_all` the `givn/artifacts/proposal/` subtree itself, since
      `givn init` does not scaffold `givn/artifacts/` — see SMELL-3).
- [x] Create `tests/new_command.rs` as an empty module skeleton that imports
      `common` and compiles.
- [x] Verify `cargo test` still exits 0 (or only with `@wip`-unrelated
      empty-suite status) before any scenario is characterised.
- [x] Run `givn lint --change reverse-engineer-new-command` and confirm exit
      2 (all scenarios still `@wip`).

Design constraints: black-box subprocess seam; no production code changes;
binary located via `CARGO_BIN_EXE_givn`; temp dirs managed by
`tempfile::TempDir` for automatic cleanup on panic. Note: `givn init` writes
the sentinel `verify.command` into the temp dir's `givn/config.yaml`; this
is harmless because `src/cli/new.rs` does not read `givn/config.yaml`.

---

## [x] Task 1 — Valid multi-word kebab-case id creates the change scaffold

- [x] GREEN: Remove `@wip` from this scenario in the `.feature` file.
- [x] Write `// Scenario: Valid multi-word kebab-case id creates the change
      scaffold` test in `tests/new_command.rs`.
- [x] Assert: exit code 0; `givn/changes/add-user-auth/` exists;
      `givn/changes/add-user-auth/proposal.md` exists; stdout contains
      `Created change 'add-user-auth'`; stdout contains the absolute path to
      `givn/changes/add-user-auth/proposal.md`; stdout contains a `Next:`
      line mentioning `givn instructions proposal --change add-user-auth`.
- [x] Run `cargo test` → this test MUST PASS immediately.
- [x] REFACTOR: none (no seam needed).

Design constraints: init-root helper; assert stdout uses single-quoted id
and an **absolute** proposal path (the current working directory is the temp
root, so the path is the temp root plus the relative file path).

---

## [x] Task 2 — Single-word id is accepted

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: exit 0; `givn/changes/feature/` and `proposal.md`
      exist.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper.

---

## [x] Task 3 — Id containing digits is accepted

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: exit 0; `givn/changes/fix-123/` exists.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper.

---

## [x] Task 4 — Empty id is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] **Probing test first:** write the test invoking `givn new` with an
      empty-string id argument and asserting ONLY a non-zero exit (and no
      `givn/changes/` entry created for the empty id). Do not assert a
      specific exit code or stderr message yet.
- [x] Run `cargo test` and observe the actual exit code and stderr. If clap
      rejects the empty string before `validate_id` runs, exit code will be
      2 with a clap message; otherwise 1 with "change id must not be empty".
- [x] Tighten the `.feature` scenario (and the design.md seam note) to match
      the AS-IS observation — assert the observed exit code and stderr. Never
      change production code.
- [x] Re-run `cargo test` → the tightened test MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root or bare root both acceptable (empty-id check
runs before the givn/-not-found check). Confirm via probing which root
yields the empty-id error; if both do, prefer the init root for
consistency with neighbouring scenarios.

---

## [x] Task 5 — Id with uppercase letters is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new HasUppercase` → non-zero exit; stderr
      contains `kebab-case`; `givn/changes/HasUppercase/` not created.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper.

---

## [x] Task 6 — Id with underscores is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new has_underscores` → non-zero; stderr
      `kebab-case`; no dir.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper.

---

## [x] Task 7 — Id with spaces is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new "has spaces"` → non-zero; stderr `kebab-case`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper; pass the id as a single argv element
containing a space (the subprocess `Command` API handles this directly — no
shell quoting needed).

---

## [x] Task 8 — Id with non-ASCII letters is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new café` → non-zero; stderr `kebab-case`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper; pass the UTF-8 id as a single argv
element.

---

## [x] Task 9 — Id with a dot is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new a.b` → non-zero; stderr `kebab-case`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper.

---

## [x] Task 10 — Id ending with a hyphen is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new foo-` → non-zero; stderr
      `must not start or end with a hyphen`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper; `foo-` is a normal positional argv
element (no `--` needed since it does not start with a hyphen).

---

## [x] Task 11 — Id beginning with a hyphen is rejected by argument parsing

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new -foo` (NO `--` separator) → assert exit
      code **2** (the argument-parsing exit code, not 1); stderr mentions
      an unexpected argument; no change directory created.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper. This characterizes SMELL-2 / the
clap-intercepted AS-IS path. If the test observes a different exit code or
message, correct the spec to match AS-IS — do not change production code.

---

## [x] Task 12 — Id beginning with a hyphen passed after the argument separator is rejected by validation

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: `givn new -- -foo` → non-zero exit; stderr contains
      `must not start or end with a hyphen`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper; pass args `["new", "--", "-foo"]` so
the leading hyphen reaches the validator rather than clap.

---

## [x] Task 13 — Uninitialised project is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: bare root (no `givn/`); `givn new some-change` →
      non-zero; stderr contains `givn/ not found` and mentions running
      init first; no change directory created.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: `make_bare_root` helper — a temp dir with no `givn/`
subtree. Do NOT run `givn init` first.

---

## [x] Task 14 — Already-existing change is rejected

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: init root with `givn/changes/dup-change/`
      pre-created (via `std::fs::create_dir_all`); `givn new dup-change` →
      non-zero; stderr contains `already exists` and the absolute path to
      `givn/changes/dup-change`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper plus a pre-created change directory.
The absolute path in stderr is `<temp root>/givn/changes/dup-change`.

---

## [x] Task 15 — Only proposal.md is created in the change directory

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: init root; `givn new only-proposal` → exit 0; the
      entry list of `givn/changes/only-proposal/` is exactly `proposal.md`;
      no `specs/` subdirectory exists inside it.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper; use `std::fs::read_dir` to collect
entry names and assert the set equals `{"proposal.md"}`.

---

## [x] Task 16 — proposal.md content equals the built-in template verbatim

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: init root; `givn new template-check` → exit 0; the
      bytes of `givn/changes/template-check/proposal.md` equal
      `include_str!("../assets/templates/proposal.md")` exactly; the
      heading contains the literal text `{{CHANGE_ID}}`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper; assert byte-for-byte equality (not
just substring) to characterize BUG-1 (the unsubstituted placeholder) as
AS-IS. If the observed content differs from the embedded template, correct
the spec to match AS-IS.

---

## [x] Task 17 — Project override at givn/artifacts/proposal/proposal.md is used instead of the built-in template

- [x] GREEN: Remove `@wip` from this scenario.
- [x] Write the test: init root; write a custom override to
      `givn/artifacts/proposal/proposal.md` via `write_override`; `givn new
      with-override` → exit 0; `givn/changes/with-override/proposal.md`
      equals the override content byte for byte; the file does NOT contain
      `{{CHANGE_ID}}`.
- [x] Run `cargo test` → MUST PASS.
- [x] REFACTOR: none.

Design constraints: init-root helper plus `write_override`. The override
directory `givn/artifacts/proposal/` must be created by the helper (`givn
init` does not scaffold `givn/artifacts/` — see SMELL-3).

---

## Done when

- All checkboxes above are checked.
- `cargo test` exits 0.
- `givn lint --change reverse-engineer-new-command` exits 0 (all `@wip`
  removed, no parse errors).
- Manually confirm each `// Scenario:` comment in `tests/new_command.rs`
  corresponds to exactly one scenario in the `.feature` (and vice versa) —
  the convention-based mapping has no automated enforcement.
- `givn status --change reverse-engineer-new-command` reports stage
  `tasks-done` with `review` as the next required step (i.e. proposal,
  specs, design, design-review, and tasks artifacts all Done; review still
  Pending). This is the real state `givn status` derives from the manifest
  artifact graph (`proposal → specs → design → design-review → tasks →
  review`) once all characterization tasks are complete — `tasks` is the
  last Done artifact before `review`. (The string "characterization
  complete, awaiting review" is NOT a real `givn status` output and must
  not be used as the Done criterion.)
