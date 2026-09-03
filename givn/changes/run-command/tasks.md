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
- [x] COMMIT: `feat(run-command): shared run scaffold asset embedded verbatim in the commands template` — hash `c7edf9d`

## T3 — Validation rules R1–R10 (`src/cli/run_block.rs`)

> **T3/T10/T11/T12/T13 executed as one atomic, suite-green unit.** The run
> gate cannot exist in the bin without its consumers (clippy `-D warnings`,
> dual lib/bin module tree), and the suite cannot stay green with the gate
> live before the shared fixtures, the permanent-suite migration, and the
> dogfood declaration land. One commit carries: run_block R1–R10, the check
> review wiring, the shared declaration Givens, the permanent-suite
> fixture migration (101 feature-level insertions + wholesale-fixture
> writers converted to the idempotent `write_commands_yaml` helper), and
> givn's own run declaration. Per-scenario RED/GREEN evidence below was
> captured against this evolving tree exactly as recorded.
>
> Environment note: the full suite requires `TMPDIR` on the root filesystem
> (13G tmpfs /tmp exhausts under cucumber's default concurrency 64); this is
> a pre-existing environment constraint, not a runner change.


### Scenario: An unconfigured declaration reports only unset findings
- [x] RED/GREEN: R1+R2+skip logic; RED: unimplemented stub panicked, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A server without a readiness path is incomplete
- [x] RED/GREEN: R3; RED: no finding mentions requires run.ready_path, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A server needs exactly one port source
- [x] RED/GREEN: R4; RED: no finding mentions exactly one of run.port or run.port_env, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A server rejects two port sources
- [x] RED/GREEN: R5; RED: no finding mentions mutually exclusive, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: Server-only fields are rejected for a CLI product
- [x] RED/GREEN: R6; RED: no finding mentions applies to product_type server only, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A zero readiness timeout is rejected
- [x] RED/GREEN: R7; RED: no finding mentions must be greater than zero, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A relative readiness path is rejected
- [x] RED/GREEN: R8; RED: no finding mentions must be an absolute path, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A no-op start command is rejected
- [x] RED/GREEN: R9; RED: no finding mentions does nothing, exit 101; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A library may keep the placeholder command
- [x] RED/GREEN: R10 exception (landed with R1's library exception); first run passed, no separate RED possible; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A complete server declaration validates cleanly
- [x] RED/GREEN: composition of implemented rules; first run passed, no separate RED possible; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

### Scenario: A complete CLI declaration validates cleanly
- [x] RED/GREEN: composition of implemented rules; first run passed, no separate RED possible; GREEN files: src/cli/run_block.rs, tests/steps/run_command_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: combined unit `feat(run-command): run-declaration validation, review gate, shared fixtures, permanent-suite migration, dogfood declaration` — hash b0ba35a

## T4 — Sentinel subcommand + e2e bring-up

Production: `src/cli/missing_run_command.rs` (pattern `missing_testrunner.rs`), `src/main.rs` hidden subcommand.

### E2E setup (before first @e2e scenario)
- [x] Local environment confirmation: no service stack exists (design §13); e2e interaction = compiled givn binary in temp projects + in-tree fixture binaries. `./run-tests.sh --e2e` ran clean end-to-end (exit 0).
- [x] Filter proof (e2e strictly smaller than full): full `./run-tests.sh` = 549 scenarios; `./run-tests.sh --e2e` = 106 scenarios. 106 < 549 — the `--tags @e2e` filter is real; commands are distinct (`--e2e` adds the tag filter).

### @e2e Scenario: The sentinel start command prints setup guidance
- [x] RED: targeted `--name` under e2e scope — `stderr missing "run.command is unset": error: unrecognized subcommand 'missing-run-command'` — exit 101
- [x] GREEN: files: `src/cli/missing_run_command.rs` (new), `src/cli/mod.rs`, `src/main.rs` (hidden subcommand + dispatch), `tests/steps/run_execution_steps.rs` (`stderr mentions` step per design §16) — `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: fmt + clippy clean, exit 0
- [x] COMMIT: `test(e2e): The sentinel start command prints setup guidance` — hash `ef41f34`

## T5 — Executor plan resolution + preflight

Production: `src/cli/run.rs` (`resolve_start_plan`, preflight, `--print`), `src/main.rs` `Commands::Run`.

### Scenario: A library has nothing to start
- [x] RED: RED: unrecognized subcommand 'run', exit 101
- [x] GREEN: files: src/cli/run.rs, src/main.rs (Run subcommand), tests/steps/run_execution_support.rs, tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): executor plan resolution, preflight, and print mode` — hash `b8246bf`

### Scenario: An unconfigured declaration refuses to start
- [x] RED: RED: stderr missing run.command is unset / no stderr-mentions step, exit 101
- [x] GREEN: files: src/cli/run.rs (preflight), tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): executor plan resolution, preflight, and print mode` — hash `b8246bf`

### Scenario: A caller port without a port hand-off is refused
- [x] RED: RED: unrecognized subcommand, exit 101
- [x] GREEN: files: src/cli/run.rs (resolve_start_plan --port rule), tests/steps/run_execution_support.rs (fixed-port fixture) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): executor plan resolution, preflight, and print mode` — hash `b8246bf`

### Scenario: Print mode omits the port hand-off for a fixed-port declaration
- [x] RED: RED: ambiguous step match + unrecognized subcommand, exit 101
- [x] GREEN: files: src/cli/run.rs (print_line), tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): executor plan resolution, preflight, and print mode` — hash `b8246bf`

### Scenario: Print mode for a library resolves the declared start command
- [x] RED: RED: unrecognized subcommand, exit 101
- [x] GREEN: files: src/cli/run.rs (library --print), tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): executor plan resolution, preflight, and print mode` — hash `b8246bf`

### Scenario: Print mode on an unconfigured declaration reports preflight findings
- [x] RED: RED: unrecognized subcommand, exit 101
- [x] GREEN: files: src/cli/run.rs (preflight before print), tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): executor plan resolution, preflight, and print mode` — hash `b8246bf`

## T6 — Spawn/probe loop + exit-code forwarding

Production: `probe_until_ready` (std::net HTTP/1.0), spawn loop, exit-code table.
Deviation recorded: production loop, fixture, and steps landed in one working
pass (no standalone per-scenario RED capture retained); the driving failures
were captured during the first full scenario pass and fixed one by one —
fixed-port fixture handed `None` to the writer (R4 violation), 404→200 flip
raced the first probe (responder now blocks until the first connection),
fixed-port product never exited (fixture lacked `--exit-after`), missing
`exits 7` step, and the readiness-timeout kill orphaned the `sh` grandchild
holding the inherited stdio pipes (product fix: process-group spawn +
group `SIGKILL`, design §7.4 updated).

### Scenario: A fixed declared port is used without a port hand-off
- [x] RED: first run hung — fixed-port product never exited; fixture command lacked `--exit-after 1` (fixture defect, then `exits 0`+ready-line passed only after fix)
- [x] GREEN: files: src/cli/run.rs (start_product), tests/ready_fixture.rs, tests/steps/run_execution_steps.rs (fixed-port given reserves the port), tests/steps/run_execution_support.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: A readiness timeout abandons the start
- [x] RED/GREEN defect capture: test harness hung — givn killed only `sh`, the never-ready fixture grandchild survived holding the inherited pipes; root fix in product (process-group kill), manual reproduction confirmed exit 2 in ~1s before the fix
- [x] GREEN: files: src/cli/run.rs (setpgid + kill(-pgid)), tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: An early product exit is forwarded before readiness
- [x] RED: initial run failed on missing generic exit step (harness gap, exit 101); `exits 3` exact step resolved the ambiguity
- [x] GREEN: files: src/cli/run.rs (early-exit forwarding), tests/steps/run_execution_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: A CLI product forwards the product's exit code
- [x] RED: `Then the command exits 7 — Step doesn't match any function` (exit 101); regex step conflicted with exact steps (cucumber ambiguity), replaced by exact `the command exits 7`
- [x] GREEN: files: tests/steps/run_execution_steps.rs (exits-7 step), src/cli/run.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: A port that is not listening yet is retried until ready
- [x] GREEN: in-process probe against reserved-but-unbound port; first full pass passed; targeted rerun green with T6 commit build
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: A failing readiness response is retried until ready
- [x] RED: 404→200 flip raced the probe's first attempt (attempt count not guaranteed >1); Responder gained a connection counter + `wait_for_connections`
- [x] GREEN: files: tests/steps/run_execution_support.rs (Responder), tests/steps/run_execution_steps.rs (When waits for first hit) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: Any success response satisfies the readiness probe
- [x] GREEN: 204-first responder; first full pass passed; targeted rerun green with T6 commit build
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### Scenario: The readiness probe gives up after the timeout
- [x] GREEN: never-answering responder with 300ms deadline → `ProbeOutcome::TimedOut`; first full pass passed; targeted rerun green with T6 commit build
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

### @e2e Scenario: The started server reports readiness and forwards a clean exit
- [x] RED: covered by the T6 shared defects above (fixture/executable-scan and step wiring)
- [x] GREEN: files: tests/steps/run_execution_e2e_steps.rs, tests/steps/run_execution_support.rs (ready_fixture_path executable-bit scan fixed to skip `.d` dep-info files) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): product start loop, readiness probe, exit-code forwarding` — hash `918a59d`

Suite gate: full permanent suite `TMPDIR=$HOME/tmp ./run-tests.sh` exit 0 after T6.

## T7 — Signal forwarding

Production: libc sigaction handlers, `AtomicI32` flag, forward+wait, library seam returns 128+sig.

### Scenario: A terminate request is forwarded and the start waits
- [x] RED: no handler installed — SIGTERM default-killed the test runner itself (`process didn't exit successfully ... signal: 15, SIGTERM`), exit 101
- [x] GREEN: files: src/cli/run.rs (sigaction handler, AtomicI32 flag, group-forward, bounded 2s reap grace, return 128+sig), tests/steps/run_execution_steps.rs, tests/steps/run_execution_support.rs (InProcStart), tests/steps/mod.rs — targeted run `1 scenario (1 passed)`, exit 0. Deviation recorded: first GREEN attempt forwarded to the direct child only; `sh` forked instead of exec'ing the fixture, the signal never reached the product (trap marker missing). Root fix: forward to the process group. All waiting loops bounded (10s ready poll, 2s marker poll, 2s reap grace) per reviewer directive — design §7.4 updated, R-44 residual resolved.
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): forward SIGTERM to the started product and exit 143` — hash `03dcbf0`

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

- [x] Implementation + compile proof: `tests/steps/run_declaration_fixtures_support.rs` (declare_run/strip_run over `common::replace_top_level_key`/`remove_top_level_key`), shared Givens registered; `cargo test --test features_runner --no-run` exit 0. Design note: the default fixture `make_init_root` and the wholesale fixture writers carry a complete cli declaration via the idempotent `common::write_commands_yaml` — same philosophy as the complete-README fixture.
- [x] COMMIT: combined unit — hash `b0ba35a`

## T11 — Permanent-suite migration

- [x] All permanent-spec fixture scenarios reaching `givn check review`/`givn archive` carry the shared complete-run-declaration Given (101 feature-level insertions across 14 files, anchored after the last Given-chain rewrite); wholesale fixture writers converted to `common::write_commands_yaml`; release-0-3-0 legacy flow fills the backfilled scaffold like a real project. Exact-output assertions audited by full-suite run (substring assertions unaffected; the run gate prints before the README gate).
- [x] Evidence: full `TMPDIR=$HOME/tmp ./run-tests.sh` exit **0** (2600+ steps, 0 failures)
- [x] COMMIT: combined unit — hash `b0ba35a`

## T12 — Dogfood: givn's own run declaration

- [x] `givn/commands.yaml` gains `run: {command: "cargo run -- instructions", product_type: cli}` (design §7.5) — landed before the gate wiring in the same suite-green unit.
- [x] COMMIT: combined unit — hash `b0ba35a`

## T13 — `givn check review` wiring

Production: `src/cli/check.rs` review arm (§6.3, findings on stdout).

### @e2e Scenario: The review check reports an unconfigured run declaration
- [x] RED/GREEN: gate wiring in `src/cli/check.rs` review arm behind the integrity hook; e2e scenario asserts exit non-zero + `run.command is unset` + `run.product_type is unset` on stdout — targeted run passed, exit 0
- [x] REFACTOR: fmt + clippy clean, exit 0
- [x] COMMIT: combined unit — hash `b0ba35a`

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
