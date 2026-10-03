# Review: run-command

README-IMPACT: updated - Usage (the consumer-observable surface — `givn run`, the
`run:` declaration, the upgrade backfill, the archive run gate — is documented in
`docs/reference.md`, the README's referenced full CLI reference; its CLI-commands
and configuration-keys tables were updated this change, commit `3e23f6a`).

## Fabrication audit

- **@e2e tag integrity**: git history of both delta `.feature` files shows the
  only removed tag lines are `@givn.added @e2e @wip → @givn.added @e2e`
  (`@wip` dropped after GREEN); no scenario lost `@e2e`. Delta net:
  48 added, 0 modified, 0 removed. 10 `@e2e` scenarios preserved.
- **Empty step bodies**: 0 found across the 8 files written or extended by this
  change (`tests/steps/run_execution_steps.rs`, `run_execution_e2e_steps.rs`,
  `run_readme_steps.rs`, `run_upgrade_steps.rs`, `run_command_steps.rs`,
  `run_execution_support.rs`, `run_declaration_fixtures_support.rs`,
  `tests/ready_fixture.rs`) — every body references assertions, the fixture
  binary, or domain objects; every file carries `assert`/`expect`/`panic`.
- **Task commits**: every checked `[x]` task records a commit hash; all hashes
  verified in `git log` and their diffs touch production source
  (`src/cli/run.rs`, `run_block.rs`, `missing_run_command.rs`, `readme.rs`,
  `archive.rs`, `upgrade.rs`, `migration.rs`, `src/main.rs`, `Cargo.toml`,
  `assets/run.yaml.tmpl`), not only specs/steps. Combined units are recorded
  once per hash with the covered scenarios named (repo convention, e.g.
  `unified-semantic-rulebook` S7+S8 style); the integrity hook enforces
  hash uniqueness and passed.
- **Design components exist**: `src/cli/run.rs` (executor), `run_block.rs`
  (R1–R10), `missing_run_command.rs` (sentinel), `readme.rs` (render/refresh/
  damage/scaffold), `archive.rs` (gate + refresh), `upgrade.rs` +
  `migration.rs` (backfill), `tests/ready_fixture.rs` (harness-free fixture),
  step files as tabled in design.md §5. All present and non-empty.
- **Strict-mode proof**: recorded in tasks.md T0 — undefined step exits 101 via
  `fail_on_skipped` (non-zero exit observed).
- **@e2e Then steps**: all ten @e2e scenarios assert CLI-level outcomes (exit
  codes, stdout lines, stderr guidance, fixture trap-marker files, git state) —
  none is repository-only. No browser-UI capability exists; no HTTP-client
  driver substitution found.
- **`verify.e2e_command` isolation**: `verify.command` = `./run-tests.sh`,
  `verify.e2e_command` = `./run-tests.sh --e2e` — distinct strings; the e2e run
  adds the `@e2e` tag filter. Counts: 548 scenarios (full) vs 106 (e2e);
  106 < 548, filter real.
- **As-built vs design**: three deviations, all recorded in design.md before
  review: (1) process-group spawn + group kill/forward (§7.4, added during T6
  when the direct-child draft was observed losing the product), (2)
  `render_run_block_for(&Manifest)` render seam (§6.5), (3) as-built step-file
  layout — `run_readme_steps.rs`, `run_upgrade_steps.rs` (new), git-state steps
  in `run_execution_e2e_steps.rs` (§5 table + §16 updated). No deviation
  overrode an architecture decision; the arc42 chapter table is unaffected.
- **E2E scope (canonical normalization)**: 6 inventory entries (run-command) +
  4 (run-execution), each one @e2e scenario; variants and error cases are
  regular scenarios. No excess @e2e.

### Interaction coverage cross-reference

| Inventory entry | Matrix row (design §12) | @e2e scenario | Driving mechanism in steps | Match |
|---|---|---|---|---|
| initialise a new project with `givn init` | yes | Scaffolding ships an unconfigured run declaration | `common::run_givn(env, ["init"])` subprocess | ✓ |
| run the review check with `givn check review --change <id>` | yes | The review check reports an unconfigured run declaration | `run_givn(env, ["check", "review", ...])` | ✓ |
| archive a change with `givn archive --change <id>` | yes | Archive passes once the run declaration is complete | `run_givn(env, ["archive", ...])` staged project | ✓ |
| refresh managed files with `givn upgrade` | yes | Upgrade backfills the run declaration into an existing project | git-managed fixture, `run_givn(env, ["upgrade"])` | ✓ |
| check the README for run managed-block damage with `givn readme check` | yes | The check names a damaged run managed block | `run_givn(env, ["readme", "check"])` | ✓ |
| start the product with `givn run` | yes | The started server reports readiness and forwards a clean exit | `run_givn(env, ["run"])` + ready fixture | ✓ |
| start the product with `givn run --port <n>` | yes | A caller-provided port is handed to the product | `run_givn(env, ["run", "--port", N])` | ✓ |
| print the start plan with `givn run --print` | yes | Print mode emits the start plan without starting | `run_givn(env, ["run", "--print"])` | ✓ |
| interrupt the started product with SIGINT | yes | An interrupt stops the started product | spawn `givn run`, read ready line from stdout, `libc::kill(pid, SIGINT)` | ✓ |
| execute the sentinel with `givn missing-run-command` | yes | The sentinel start command prints setup guidance | `run_givn(env, ["missing-run-command"])` | ✓ |

All 10 inventory entries mapped, all @e2e scenarios present, all driving
mechanisms as promised (real subprocess + real HTTP fixture + real signals).

### Coverage measurement validity

`COVERAGE MEASUREMENT: VALID` — `./measure-coverage.sh` (delta scope,
`GIVN_FEATURES=givn/changes/run-command/specs`) replays the runner's group
partition under `cargo llvm-cov --no-report` (per-process profraws, flushed),
produces both source reports; `./merge-coverages.sh` merges them into a fresh
merged Cobertura report. Production processes are instrumented; a known
exercised production path has non-zero coverage: `src/cli/run.rs` line-rate
0.904 (187 valid lines), `src/cli/run_block.rs` 0.977, `src/cli/missing_run_command.rs`
1.000, `src/cli/readme.rs` 0.738. Runner-only/zero-production coverage: rejected
and not present. The archive gate runs the same scripts on its own features
scope, so the merged report is freshly produced per run; stale outputs are not
relied on.

## Arc42 implementation conformance

ARC42 CONFORMANCE: CLEAN — derivation cross-check between the durable
chapters, the change-level `arc42.md`, design.md, tasks.md, and the tree:

| Chapter/fact | Durable source | Design/task mapping | Implementation evidence |
|---|---|---|---|
| ch. 03 context: givn executes the product start | ADR-067, ch. 09 paragraph | design §2/§7; T5/T6/T7 | `src/cli/run.rs` executor + @e2e ready-line scenarios |
| ch. 05 building blocks: executor module | ch. 05 run block description | design §7; T5–T7 | `src/cli/run.rs` (execute/resolve/probe/start), `src/main.rs` dispatch |
| ch. 06 runtime: check review / archive gates order | ch. 06 gate pipeline order | design §6.3–6.4; T3/T14 | `check.rs` review arm; `archive.rs` gate before README gate |
| ch. 08 § coverage/reporting: unchanged | not affected | — | no drift |
| ch. 09 + adr/: ADR-067 accepted, register count 36+3 | 09 updated, README register table+prose, qualification-gate scenario bumped | T16 | commits `2ad0415`; permanent scenario green |
| ch. 11 R-41/R-42/R-44/R-45, TC-25 | updated during T0 | design §7; tasks T6/T7 | risks carry the contract/process-group/compat facts |
| ch. 12 glossary: run terms | recorded (12-glossary.md) | domain-modeling during change | terms below |

No stale chapter, no contradiction found.

## Ubiquitous language

Run declaration, product type, port hand-off, readiness path, readiness probe,
ready line, run managed block, run finding, run gate, start plan, backfill
scaffold, sentinel — recorded in `docs/arc42/12-glossary.md` and reused
verbatim across the two delta features, design.md, the implementation
(`Run`, `ProductType`, `StartPlan`, `probe_until_ready`, `render_run_block`,
`backfill_commands`), and the ADR.

## Overlap dispositions

Deterministic scan reports 39 shape matches, all within the delta's
assertion skeleton (one finding in, labeled findings out) or between delta
and permanent scenarios sharing a Given/When shape. Every pair disposed
individually; shape similarity alone never merges two different domain
outcomes.

A distinct rule or distinct failure mode per pair; the shared skeleton is the checker's fixed finding shape, not duplicated intent.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A server without a readiness path is incomplete | A server needs exactly one port source | boundary: distinct rule (R3 MISS vs R4 FORB) and distinct finding message; retain both |
| A server without a readiness path is incomplete | A server rejects two port sources | boundary: absence of a required field vs contradiction of two sources (R3 vs R4); retain both |
| A server without a readiness path is incomplete | Server-only fields are rejected for a CLI product | boundary: incomplete server vs forbidden-field misuse (R3 vs R6/R7); retain both |
| A server without a readiness path is incomplete | A zero readiness timeout is rejected | boundary: missing ready_path vs non-positive timeout (R3 vs R7); retain both |
| A server without a readiness path is incomplete | A relative readiness path is rejected | boundary: absent path vs malformed path (R3 vs R8); retain both |
| A server without a readiness path is incomplete | A no-op start command is rejected | boundary: readiness field vs start-command validity (R3 vs R9); retain both |
| A server needs exactly one port source | A server rejects two port sources | boundary: the two halves of R4's exclusivity (missing vs both present); retain both |
| A server needs exactly one port source | Server-only fields are rejected for a CLI product | boundary: server field-completeness vs CLI product-type confinement (R4 vs R6); retain both |
| A server needs exactly one port source | A zero readiness timeout is rejected | boundary: port source vs timeout validity (R4 vs R7); retain both |
| A server needs exactly one port source | A relative readiness path is rejected | boundary: port source vs path shape (R4 vs R8); retain both |
| A server needs exactly one port source | A no-op start command is rejected | boundary: port source vs sentinel command on a server (R4 vs R9); retain both |
| A server rejects two port sources | Server-only fields are rejected for a CLI product | boundary: server self-contradiction vs cross-product-type forbiddance (R4 vs R6); retain both |
| A server rejects two port sources | A zero readiness timeout is rejected | boundary: R4 exclusivity vs R7 range; retain both |
| A server rejects two port sources | A relative readiness path is rejected | boundary: R4 exclusivity vs R8 shape; retain both |
| A server rejects two port sources | A no-op start command is rejected | boundary: R4 exclusivity vs R9 no-op; retain both |
| Server-only fields are rejected for a CLI product | A zero readiness timeout is rejected | boundary: product-type confinement vs timeout range (R6 vs R7); retain both |
| Server-only fields are rejected for a CLI product | A relative readiness path is rejected | boundary: product-type confinement vs path shape (R6 vs R8); retain both |
| Server-only fields are rejected for a CLI product | A no-op start command is rejected | boundary: product-type confinement vs no-op command (R6 vs R9); retain both |
| A zero readiness timeout is rejected | A relative readiness path is rejected | boundary: timeout range vs path shape (R7 vs R8); retain both |
| A zero readiness timeout is rejected | A no-op start command is rejected | boundary: timeout range vs no-op command (R7 vs R9); retain both |
| A relative readiness path is rejected | A no-op start command is rejected | boundary: path shape vs no-op command (R8 vs R9); retain both |

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A library may keep the placeholder command | A complete server declaration validates cleanly | boundary: R10 library exception vs complete-server render of the same checker; distinct outcomes (empty findings via exemption vs via full configuration); retain both |
| A library may keep the placeholder command | A complete CLI declaration validates cleanly | boundary: R10 exception vs full cli declaration; retain both |
| A complete server declaration validates cleanly | A complete CLI declaration validates cleanly | boundary: per-product-type clean renders assert different field sets; retain both |
| An unconfigured declaration renders exactly one open placeholder | A half-configured server declaration renders the placeholder | boundary: the render predicate must collapse both incompleteness kinds (unset sentinel vs partial server) to the same single todo line; both sides pin one input class; retain both |
| An unconfigured declaration refuses to start | A caller port without a port hand-off is refused | boundary: distinct preflight refusal (unset declaration vs `--port` without `port_env`); different exit-path evidence; retain both |
| A port that is not listening yet is retried until ready | A failing readiness response is retried until ready | boundary: connection-refused retry vs non-2xx retry; distinct probe failure modes; retain both |
| Print mode omits the port hand-off for a fixed-port declaration | Print mode for a library resolves the declared start command | boundary: the two `print_line`/library print branches (no hand-off vs library resolution); retain both |

Permanent scenarios sharing only a Given/When shape with a delta scenario; capabilities and outcomes differ.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Lowercase [x] checkboxes are counted as done in text mode | A library has nothing to start | unrelated work: status-render fixture vs the executor's library no-op; shape similarity only; retain both |
| Uppercase [X] checkboxes are counted as done in text mode | A library has nothing to start | unrelated work: same status fixture family; retain both |
| No changes at all — text mode | A library has nothing to start | unrelated work: status empty-state vs executor no-op; retain both |
| Detail mode — archived change (text) | A library has nothing to start | unrelated work: status detail rendering vs executor; retain both |
| Detail mode — task progress shown for tasks artifact | A library has nothing to start | unrelated work: status task progress vs executor; retain both |
| arc42-docs appears in the artifact checklist when the addon is enabled | A library has nothing to start | unrelated work: artifact checklist vs executor; retain both |
| givn status still reports skipped artifacts with the skip marker after dead code removal | A library has nothing to start | unrelated work: status skip marker vs executor; retain both |
| A legacy README is reported for migration | The check names a damaged run managed block | boundary: legacy-text placeholder finding vs run managed-block damage finding (different ReadmeFindingKind + message); retain both |
| The check names a damaged managed block | The check names a damaged run managed block | boundary: development-block damage vs run-block damage — this change's delta adds the second checker arm; retain both |
| The check reports a missing README | The check names a damaged run managed block | boundary: MissingReadme vs DamagedBlock; retain both |
| A marker-less README is unmanaged | The check names a damaged run managed block | boundary: unmanaged-README verdict vs damaged run block; retain both |

## Coverage gate (completeness)

Gaps in the change's new production code (delta-scope measurement):

| Gap | Classification | Resolution/Reason |
|---|---|---|
| `resolve_start_plan` re-check of `check_run` | dead code (bucket 1) | deleted — `execute` always preflights immediately before resolving, so the branch is unreachable (commit `cc3c4d1`) |
| `backfill_commands` empty-base branch | dead code (bucket 1) | deleted — `givn/commands.yaml` always carries the init template; the branch cannot occur (commit `cc3c4d1`) |
| `pre_exec`/`setpgid` lines (run.rs) | legitimately hard to test (bucket 3) | the closure runs in the forked child between fork and exec; coverage counters cannot survive exec. The behavior is proven by the process-group scenarios (timeout kill and group forward both depend on the group existing) |
| probe_once mid-request TCP failure branches (write err / read err / unparsable status) | legitimately hard to test (bucket 3) | loopback mid-request failure injection is racy by nature; all three branches fail-safe to `false`, which feeds the retry loop — and the retry behavior itself is fully covered (404/204/no-listener/timeout scenarios) |
| `givn run: failed to start` spawn-error branch | legitimately hard to test (bucket 3) | `sh -c` spawns fine on any host that runs givn at all; `sh` failing to spawn is an environment catastrophe outside the machine contract. The realistic failure (nonexistent command → sh exits 127) is forwarded through the same child-exit path covered by the exit-3/exit-7 scenarios |
| SIGKILL-after-grace in the forward path (product ignores the signal) | legitimately hard to test (bucket 3) | requires a product that deliberately violates the signal contract (blocks SIGTERM >2s); the kill primitive (`kill(-pgid, SIGKILL)`) is identical to the readiness-timeout kill which is covered; givn's observable behavior (bounded wait, exit 128+sig) is asserted either way |
| `#[cfg(not(unix))]` fallbacks (run.rs) | legitimately hard to test (bucket 3) | compiled out on this platform by construction; the unix paths carry the spec'd contract |

Pre-existing uncovered lines in touched files (legacy texts, dev-block
refresh, project-name detection, migration machinery) belong to capabilities
outside this change and are exercised by the permanent-scope groups the
coverage gate replays; they are not gaps of this change.

## Evidence

- `./run-tests.sh`: exit 0, 548 scenarios, 0 failures (rerun after the
  review's dead-code deletions).
- `./run-tests.sh --e2e`: exit 0, 106 scenarios, 0 failures.
- Delta suite (`GIVN_FEATURES=givn/changes/run-command/specs`): 48/48
  scenarios green, exit 0 (rerun after the deletions).
- `givn lint --change run-command`: exit 0 ("2 file(s) checked — clean")
  after `@wip` removal.
- `givn check review --change run-command`: exit 0 — verify passed,
  verify-e2e passed (10 scenarios), integrity passed, overlap dispositions
  passed (39 rows), semantic report gate passed with the real backend
  (pair cache: 0 reused, 20 computed).
- `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- No local run command beyond `./run-tests.sh` / `./run-tests.sh --e2e`
  exists; no service stack or digital twin is required (design §13).

## Ubiquitous language conformance

No spec or design term outside the glossary was found; the run-capability
terms above resolve to glossary entries (see Arc42 table and the glossary
chapter).

REVIEW: PASS
