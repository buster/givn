# Tasks: run-command

Runner: `./run-tests.sh` (verify.command) / `./run-tests.sh --e2e` (verify.e2e_command).
Single-scenario: `GIVN_FEATURES=givn/changes/run-command/specs cargo test --test features_runner -- --name "<title>"` (unit); e2e scenarios run under the e2e script scoped the same way.
Strict mode: `fail_on_skipped()` on the cucumber builder (`tests/features_runner.rs`) — proven in T0.
Stub for unimplemented steps: `unimplemented!()` (Rust/cucumber-rs).

## T0 — Setup: step skeletons + proof of strictness

- [x] Create step-definition skeletons, one file per capability, and register them:
  `tests/steps/run_command_steps.rs`, `tests/steps/run_command_e2e_steps.rs`,
  `tests/steps/run_execution_steps.rs`, `tests/steps/run_execution_e2e_steps.rs`,
  `tests/steps/run_execution_support.rs`, `tests/steps/run_declaration_fixtures_support.rs`;
  `pub mod` registration in `tests/steps/mod.rs`.
- [x] Proof of strictness (undefined step must fail the run):
  - Command: `GIVN_FEATURES=givn/changes/run-command/specs cargo test --test features_runner -- --name "An unknown product type is rejected when the commands file loads"`
  - Output:
    ```
    1 step (1 failed)
    thread 'main' panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
    1 step failed
    error: test failed, to rerun pass `--test features_runner`
    ```
    exit code **101** (non-zero) — `.fail_on_skipped()` rejects undefined steps.
- [x] Commit: `chore(run-command): step skeletons and strict-mode proof` — hash `43ef246`

## T1 — Data model + load rejection

Production: `src/config/manifest.rs` (`Run`, `ProductType`, `Default`, `Manifest.run`), `src/config/overlay.rs` (`OverlayDoc.run`, replace-whole merge in `apply`).

### Scenario: An unknown product type is rejected when the commands file loads
- [x] RED: `GIVN_FEATURES=... cargo test --test features_runner -- --name "An unknown product type is rejected when the commands file loads"` — `1 step (1 failed)`, `Step panicked ... unimplemented` — exit **101**
- [x] GREEN: files touched: `src/config/manifest.rs` (`Run`, `ProductType`, `Default`, `Manifest.run`), `src/config/overlay.rs` (`OverlayDoc.run`, replace-whole merge), `tests/steps/run_command_steps.rs` (3 steps). — `1 scenario (1 passed)`, `3 steps (3 passed)` — exit **0**
- [x] REFACTOR: none needed (minimal implementation, existing patterns) — re-run exit **0**
- [x] COMMIT: `feat(run-command): An unknown product type is rejected when the commands file loads` — hash `edb92f2`; lint follow-up `fix(run-command): wire Run into manifest fixtures; defer is_unconfigured to first use` — hash `9c28932`

## T2 — Shared scaffold asset

Production: `assets/run.yaml.tmpl` (new), embed verbatim in `assets/commands.yaml.tmpl`, unit test `assets/run_scaffold_is_embedded_verbatim`.

- [x] Asset + containment test written; `cargo test --lib run_scaffold_is_embedded_verbatim` → `test result: ok. 1 passed` — exit **0**
- [x] COMMIT: `feat(run-command): shared run scaffold asset embedded verbatim in the commands template` — hash `_t2`

## T3 — Validation rules R1–R10 (`src/cli/run_block.rs`)

### Scenario: An unconfigured declaration reports only unset findings
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A server without a readiness path is incomplete
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A server needs exactly one port source
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A server rejects two port sources
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: Server-only fields are rejected for a CLI product
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A zero readiness timeout is rejected
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A relative readiness path is rejected
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A no-op start command is rejected
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A library may keep the placeholder command
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A complete server declaration validates cleanly
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A complete CLI declaration validates cleanly
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T4 — Sentinel subcommand + e2e bring-up

Production: `src/cli/missing_run_command.rs` (pattern `missing_testrunner.rs`), `src/main.rs` hidden subcommand.

### E2E setup (before first @e2e scenario)
- [ ] Local environment confirmation: `_`
- [ ] Filter proof (e2e strictly smaller than full): full count `_`, e2e count `_`

### @e2e Scenario: The sentinel start command prints setup guidance
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T5 — Executor plan resolution + preflight

Production: `src/cli/run.rs` (`resolve_start_plan`, preflight, `--print`), `src/main.rs` `Commands::Run`.

### Scenario: A library has nothing to start
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: An unconfigured declaration refuses to start
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A caller port without a port hand-off is refused
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: Print mode omits the port hand-off for a fixed-port declaration
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: Print mode for a library resolves the declared start command
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: Print mode on an unconfigured declaration reports preflight findings
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T6 — Spawn/probe loop + exit-code forwarding

Production: `probe_until_ready` (std::net HTTP/1.0), spawn loop, exit-code table.

### Scenario: A fixed declared port is used without a port hand-off
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A readiness timeout abandons the start
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: An early product exit is forwarded before readiness
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A CLI product forwards the product's exit code
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A port that is not listening yet is retried until ready
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A failing readiness response is retried until ready
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: Any success response satisfies the readiness probe
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The readiness probe gives up after the timeout
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: The started server reports readiness and forwards a clean exit
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T7 — Signal forwarding

Production: libc sigaction handlers, `AtomicI32` flag, forward+wait, library seam returns 128+sig.

### Scenario: A terminate request is forwarded and the start waits
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: An interrupt stops the started product
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T8 — Caller-provided port + print mode @e2e

### @e2e Scenario: A caller-provided port is handed to the product
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: Print mode emits the start plan without starting
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T9 — README run block

Production: `src/cli/readme.rs` (markers, `render_run_block`, `refresh_run_block`, damage check, `scaffold_readme`), `assets/addons/readme/README.md.tmpl` (`{{RUN_BLOCK}}`, shortened quick-start todo).

### Scenario: An unconfigured declaration renders exactly one open placeholder
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The README renders a complete server declaration
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The README renders a CLI product
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The README renders a library product as nothing to start
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The README renders a library's declared start command
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A half-configured server declaration renders the placeholder
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: The check names a damaged run managed block
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: Scaffolding ships an unconfigured run declaration
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T10 — Shared run-declaration fixture step

Production/test infra: `tests/steps/run_declaration_fixtures_support.rs` (§4.2): `declare_run`, `strip_run`, shared Givens in `tests/steps/mod.rs`.

- [ ] Implementation + compile proof: `_`
- [ ] COMMIT: `_` — hash `_`

## T11 — Permanent-suite migration

- [ ] All permanent-spec fixture scenarios reaching `givn check review`/`givn archive` carry the shared complete-run-declaration Given; exact-output assertions audited.
- [ ] Evidence: full `./run-tests.sh` exit `_`
- [ ] COMMIT: `_` — hash `_`

## T12 — Dogfood: givn's own run declaration

- [ ] `givn/commands.yaml` gains `run: {command: "cargo run -- instructions", product_type: cli}` (design §7.5) — before the gates sharpen.
- [ ] COMMIT: `_` — hash `_`

## T13 — `givn check review` wiring

Production: `src/cli/check.rs` review arm (§6.3, findings on stdout).

### @e2e Scenario: The review check reports an unconfigured run declaration
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T14 — Archive run gate + refresh

Production: `src/cli/archive.rs` (gate before README gate, §6.4) + `refresh_run_block` call.

### Scenario: Archive is blocked while the run declaration is unconfigured
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: Archive passes once the run declaration is complete
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T15 — Backfill + managed upgrade

Production: `src/cli/upgrade.rs` (`backfill_commands`, `RUN_SCAFFOLD`), `src/cli/migration.rs` (snapshot, apply, commit paths).

### Scenario: The backfill appends the run scaffold to an existing commands file
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The backfill is idempotent
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: The upgrade commit includes the backfilled run declaration
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### Scenario: A repeated upgrade creates no additional commit
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

### @e2e Scenario: Upgrade backfills the run declaration into an existing project
- [ ] RED: `_` — GREEN: files `_` — REFACTOR: `_` — COMMIT: hash `_`

## T16 — ADR + arc42 finalization

- [ ] ADR-067 status `accepted`; arc42 chapters finalized; `arc42.md` STATUS: DONE re-verified.
- [ ] COMMIT: `_` — hash `_`

## Final gate

- [ ] `./run-tests.sh` (full suite incl. delta scope) → exit `_`
- [ ] `./run-tests.sh --e2e` → exit `_`
- [ ] `givn lint --change run-command` → exit `_`
- [ ] Coverage gate green (review hook)
