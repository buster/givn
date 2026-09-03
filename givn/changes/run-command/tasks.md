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
- [x] RED: SIGINT not yet in the handler set — givn subprocess died by default disposition, `the command exits 130` failed (`status.code()` None), exit 101; stray fixture left behind (expected in RED: no teardown path ran)
- [x] GREEN: files: src/cli/run.rs (SIGINT added to sigaction set), tests/steps/run_execution_e2e_steps.rs (stays-ready given, interrupt when with 30s watchdog, exits-130 then, trap-marker then with 2s poll), tests/steps/mod.rs (run_trap_marker field) — targeted run `1 scenario (1 passed)`, exit 0. Deviation recorded: first GREEN attempt passed `--trap-marker=<path>` (equals syntax); the fixture only parses `--trap-marker <path>` (space-separated), marker never written — fixed in the step, not the fixture.
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): forward SIGINT to the started product and exit 130` — hash `bf935fc`

## T8 — Caller-provided port + print mode @e2e

### @e2e Scenario: A caller-provided port is handed to the product
- [x] RED: ready-line step for the caller port went missing in the T6 step rewrite (change-scope run: `And the ready line names the caller-provided port — step doesn't match`, exit 101)
- [x] GREEN: files: tests/steps/run_execution_steps.rs (`the ready line names the caller-provided port` asserting `:{port}/` from `run_caller_port`) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `test(e2e): caller-provided port hand-off and print-mode contract steps` — hash `1fbf3c5`

### @e2e Scenario: Print mode emits the start plan without starting
- [x] RED: print-contract steps went missing in the T6 step rewrite (change-scope run: `stdout is one line that exports the port hand-off and names the start command — step doesn't match`, exit 101)
- [x] GREEN: files: tests/steps/run_execution_steps.rs (both print-mode steps restored from b8246bf) — targeted run `1 scenario (1 passed)`, exit 0; fixed-port/library print variants re-verified green
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `test(e2e): caller-provided port hand-off and print-mode contract steps` — hash `1fbf3c5`

## T9 — README run block

Production: `src/cli/readme.rs` (markers, `render_run_block`, `refresh_run_block`, damage check, `scaffold_readme`), `assets/addons/readme/README.md.tmpl` (`{{RUN_BLOCK}}`, shortened quick-start todo).

RED evidence shared by the five render scenarios: steps undefined until this task (change-scope run exit 101, `Step doesn't match any function`); the `a run declaration:` Given is reused from T3's `run_command_steps.rs` (a duplicate Given caused an ambiguous-match failure, removed in favor of the shared one), and the renderer gained a `render_run_block_for(&Manifest)` seam so the unit scenarios render from the overlay manifest without chdir.

### Scenario: An unconfigured declaration renders exactly one open placeholder
- [x] RED: undefined steps, exit 101 (change-scope run)
- [x] GREEN: files: src/cli/readme.rs (render_run_block/render_run_block_for/run_block_body), tests/steps/run_readme_steps.rs (new), tests/steps/mod.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### Scenario: The README renders a complete server declaration
- [x] RED: undefined steps, exit 101 (change-scope run)
- [x] GREEN: files: as above — targeted run `1 scenario (1 passed)`, exit 0 (start command, `/healthz`, `$PORT`, no placeholder)
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### Scenario: The README renders a CLI product
- [x] RED: undefined steps, exit 101 (change-scope run)
- [x] GREEN: files: as above — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### Scenario: The README renders a library product as nothing to start
- [x] RED: undefined steps, exit 101 (change-scope run)
- [x] GREEN: files: as above — targeted run `1 scenario (1 passed)`, exit 0 (library line, no placeholder, no start command)
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### Scenario: The README renders a library's declared start command
- [x] RED: undefined steps, exit 101 (change-scope run)
- [x] GREEN: files: as above — targeted run `1 scenario (1 passed)`, exit 0 (library line + real declared command fenced)
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### Scenario: A half-configured server declaration renders the placeholder
- [x] RED: undefined steps, exit 101 (change-scope run)
- [x] GREEN: files: as above — targeted run `1 scenario (1 passed)`, exit 0 (render predicate routes findings to the todo line, never a partial render)
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### @e2e Scenario: The check names a damaged run managed block
- [x] RED: damage check and Given missing until this task (change-scope run exit 101)
- [x] GREEN: files: src/cli/readme.rs (run-marker damage check naming the run managed block), tests/steps/run_execution_e2e_steps.rs (broken-run-block Given) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

### @e2e Scenario: Scaffolding ships an unconfigured run declaration
- [x] RED: template lacked `{{RUN_BLOCK}}` and the run-block/placeholder/sentinel Thens were missing (change-scope run exit 101)
- [x] GREEN: files: assets/addons/readme/README.md.tmpl ({{RUN_BLOCK}} under Quick start, todo shortened to "Exact commands to clone and build the project."), src/cli/readme.rs (scaffold_readme replaces {{RUN_BLOCK}}), tests/steps/run_execution_e2e_steps.rs (sentinel, run block, run placeholder Thens) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR + suite gate: cargo fmt + clippy clean; full `TMPDIR=$HOME/tmp ./run-tests.sh` exit 0 (template change affects init paths used by permanent fixtures; substring assertions hold)
- [x] COMMIT: `feat(run-command): README run managed block render, check, scaffold, refresh` — hash `45cd9c5`

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

Production: `src/cli/archive.rs` (gate before README gate, §6.4) + `refresh_run_block` call. The refresh call landed with T9 (same file, same rollback handling); the gate is the missing piece.

### Scenario: Archive is blocked while the run declaration is unconfigured
- [x] RED: archive proceeded past the missing gate — `stdout mentions "run declaration"` never appeared and the change directory moved away, exit 101
- [x] GREEN: files: src/cli/archive.rs (run-declaration gate after the similarity gate, before the README gate, labeled findings + bail), tests/steps/run_execution_e2e_steps.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): archive run-declaration gate before the README gate` — hash `a45c606`

### @e2e Scenario: Archive passes once the run declaration is complete
- [x] RED: `verify commands are no-ops with a complete server run declaration` Given and the named archived-directory Then missing, `Step doesn't match any function`, exit 101
- [x] GREEN: files: tests/steps/run_execution_e2e_steps.rs (composed Given: no-op verify/coverage yaml + complete server declaration via declare_run; `the archived change directory "<id>" exists`), src/cli/archive.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR + suite gate: cargo fmt + clippy clean; the new gate initially blocked the two coverage-workflow archiving fixtures (no run declaration) — both writers converted to the idempotent `common::write_commands_yaml` (T11 pattern); full `TMPDIR=$HOME/tmp ./run-tests.sh` exit 0, 548 scenarios, 0 failures
- [x] COMMIT: `feat(run-command): archive run-declaration gate before the README gate` — hash `a45c606`

## T15 — Backfill + managed upgrade

Production: `src/cli/upgrade.rs` (`backfill_commands`, `RUN_SCAFFOLD`), `src/cli/migration.rs` (snapshot, apply, commit paths).

### Scenario: The backfill appends the run scaffold to an existing commands file
- [x] RED/GREEN: steps and production landed in one working pass — first targeted run passed, no separate RED possible; GREEN files: src/cli/upgrade.rs (backfill_commands + RUN_SCAFFOLD include_str), tests/steps/run_upgrade_steps.rs (new), tests/steps/mod.rs — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): run-scaffold backfill in the managed upgrade` — hash `0c035af`

### Scenario: The backfill is idempotent
- [x] RED/GREEN: as above (first targeted run passed); GREEN files: as above — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): run-scaffold backfill in the managed upgrade` — hash `0c035af`

### Scenario: The upgrade commit includes the backfilled run declaration
- [x] RED: `givn upgrade` failed with `requires a clean Git worktree` — the fixture committed the baseline WITH a run declaration, then the chain stripped it, leaving a dirty tree (fixture defect, exit 101)
- [x] GREEN: files: tests/steps/run_execution_e2e_steps.rs (git-managed baseline commits a run-less commands.yaml; working-tree-clean and latest-commit-contains Thens; file-contains Then), src/cli/migration.rs (snapshot/apply/commit wiring) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): run-scaffold backfill in the managed upgrade` — hash `0c035af`

### Scenario: A repeated upgrade creates no additional commit
- [x] RED: covered by the shared missing-steps defect above (exit 101)
- [x] GREEN: files: tests/steps/run_execution_e2e_steps.rs (`upgrade again` When captures the pre-hash, unchanged-commit Then), src/cli/migration.rs (no diff → no commit) — targeted run `1 scenario (1 passed)`, exit 0
- [x] REFACTOR: cargo fmt + clippy -D warnings clean, exit 0
- [x] COMMIT: `feat(run-command): run-scaffold backfill in the managed upgrade` — hash `0c035af`

### @e2e Scenario: Upgrade backfills the run declaration into an existing project
- [x] RED: covered by the shared fixture defect above (exit 101)
- [x] GREEN: files: tests/steps/run_execution_e2e_steps.rs (git-managed Given), src/cli/migration.rs (snapshot + backfill + fill-it-in print + commit path) — targeted run `1 scenario (1 passed)`, exit 0 (exit 0, commands.yaml contains run:, stdout mentions "fill it in")
- [x] REFACTOR + suite gate: cargo fmt + clippy clean; full `TMPDIR=$HOME/tmp ./run-tests.sh` exit 0, 548 scenarios, 0 failures (migration/upgrade paths exercised by permanent fixtures)
- [x] COMMIT: `feat(run-command): run-scaffold backfill in the managed upgrade` — hash `0c035af`

## T16 — ADR + arc42 finalization

- [x] ADR-067 status `accepted` after the confirmation evidence held: full suite 548 scenarios exit 0 (T15 suite gate) and `./run-tests.sh --e2e` 106 scenarios exit 0 (this task); register README (prose + table row), chapter 9, and the ADR's Confirmation section record the transition; `arc42.md` STATUS: DONE re-verified (chapter-9 row updated to describe the completed flip). Permanent adr-qualification-gate scenario bumped to "36 accepted decisions and three proposed decisions" per the established single-transition pattern; its step updated to match (targeted run passed, exit 0).
- [x] COMMIT: `feat(run-command): accept ADR-067 and finalize the arc42 register` — hash `51a6c42`

## Final gate

- [ ] `./run-tests.sh` (full suite incl. delta scope) → exit `_`
- [ ] `./run-tests.sh --e2e` → exit `_`
- [ ] `givn lint --change run-command` → exit `_`
- [ ] Coverage gate green (review hook)
