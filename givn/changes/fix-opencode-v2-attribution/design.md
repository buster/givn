# Design: fix-opencode-v2-attribution

## Scope and binding inputs

Planning only. The production change is confined to the embedded OpenCode
attribution asset; supporting tests and their isolated runtime provisioning are
described here, not implemented. No changes to user-owned
`.opencode/plugins/givn-attribution.js`, `.opencode/plugins/givn-attribution_v1.js`,
or harness plugins are permitted. Both user files were read in full on
2026-10-04. The former is the immutable behavioral baseline; the latter is the
legacy installation fixture input, not a second generated entrypoint.

Read-baseline SHA-256 values, rechecked after authoring:
`e7e5fa591521ba67fd05f3d5757b68baea4b9bb15ad674fb2f3ca9178c886c9d`
for the canonical supplied prototype and
`e1eec8605d353fdced8ec2f3068dadb720614dabf786312fb26d07af62999d27`
for the legacy file. The immutable fixture copies must match these values;
if root content changes meanwhile, do not copy it blindly as the baseline.

The compatibility-window decision (D1) promises recent V1 and V2, not older V1.
The preserve-attribution-behavior decision (D2) prohibits changing the supplied
logic, including correcting its cross-session inference risk. Full questions,
options, user responses, and rejected alternatives remain in `proposal.md`.

## Decisions

- **Decides D1: support OpenCode V1 1.18.29+ within the V1 generation and
  OpenCode V2 through one canonical default object with `id`, `server`, and
  `setup`** — realized in the embedded asset and three-release real-loader
  matrix. `server()` delegates to the retained named `GivnAttributionPlugin`.
  The named export can remain for consumer-fixture compatibility. No runtime
  version detection and no generated V1 sibling. Reject older-V1 support and
  separate version-specific installation because the user selected the recent
  V1 plus V2 window.
- **Decides D2: preserve the supplied prototype byte-for-byte except for the
  added default `server()` delegation** — realized by an immutable prototype
  fixture, public-hook characterization, installed-file parity, and a narrowly
  clarified glossary/adapter risk. Reject current-session-only fallback,
  clearing retained variants, extra title hooks, persistent model state,
  fallback refactoring, and fatal default lookup failures. The exact user
  constraint was “No changes to the plugin logic itself.” This is not an
  endorsement of inferred labels as verified facts.

No operator question remains unresolved. Deployment provision hashes and the
latest release refresh are mechanical setup values, not new product choices.

## Architecture impact

### Existing boundary and minimal realization

Givn's workflow bounded context already has an agent-integration adapter. Model
extraction belongs to the embedded JavaScript asset, not the generic Rust
attribution stamper. No Aggregate, Domain Event, repository, or domain
dependency is introduced. The module-level session map is adapter runtime
state, not a domain Aggregate or a persisted model selection.

| Exact path | Planned responsibility/change |
|---|---|
| `assets/agent-integrations/opencode/givn-attribution.js` | Replace the V1-only asset with the supplied canonical prototype, adding only `async server() { return GivnAttributionPlugin() }` on its default object. Retain all existing prototype statements and hook registrations. |
| `src/config/embedded.rs` | Existing Rust embedding; no source change required. A rebuilt binary embeds the updated asset. |
| `src/skills/targets.rs` | Existing `sync_agent_integrations` writes `.opencode/plugins/givn-attribution.js` from that asset when OpenCode is selected; `write_embedded_file` overwrites it unconditionally. No seam refactor, preservation feature, or second writer. |
| `src/cli/init.rs`, `src/cli/skills.rs`, `src/cli/addons.rs`, `src/cli/migration.rs`, `src/cli/upgrade.rs` | Existing callers converge on `skills::targets::sync`. Keep call order, reports, error policy, clean-state witnesses, allowlist calculation, and pending-migration early returns. No production edits expected. |
| `tests/steps/llm_attribution_steps.rs` | Existing per-capability regular bindings for `attribute-commits`; add prototype characterization glue here. |
| `tests/steps/llm_attribution_e2e_steps.rs` | Existing per-capability E2E bindings for `attribute-commits`; replace Node-only compatibility proof with the actual OpenCode runtime driver. Preserve unrelated Git/Claude bindings. |
| `tests/steps/llm_attribution_installation_steps.rs` | Existing regular bindings for `enable-commit-attribution`; reinit/config preservation, no-addons and target-exclusion variants. |
| `tests/steps/llm_attribution_installation_e2e_steps.rs` | Installation/report/upgrade and four new regeneration E2E actions, separate from regular bindings. |
| `tests/common/opencode_runtime.rs`, `tests/common/mod.rs`, `tests/steps/mod.rs` | Test-only runtime admission, supervisor lease, fresh World state, and module registration; no production abstraction. |
| `tests/opencode_supervisor.rs`, `Cargo.toml` test-target declaration | Test-only `harness = false` supervisor target using the existing unix `libc` dependency; signal-aware process-group teardown and parent-lease monitoring. No new production dependency or executor change. |
| `tests/fixtures/opencode/runtime_driver.mjs` | Real-runtime orchestration and result normalization; does not emulate the plugin loader. |
| `tests/fixtures/opencode/provider_stub.mjs` | Deterministic loopback OpenAI-compatible provider, catalog/proxy twin, request recording, deadlines and rejection of unexpected requests. |
| `tests/fixtures/opencode/prototype/givn-attribution.js` | Immutable copy of the full supplied 107-line prototype, made during implementation from the read baseline; never import mutable root content. |
| `tests/fixtures/opencode/prototype/givn-attribution_v1.js` | Immutable copy of the supplied 38-line legacy input, used only to seed obsolete integrations in temporary projects. |
| `tests/fixtures/opencode/characterize.mjs` | Consumer of public V1/V2 hooks with controlled event payloads; compares generated behavior with the immutable prototype. Not E2E loader evidence. |
| `tests/fixtures/opencode/run-suite.sh`, `tests/fixtures/opencode/resolve-runtime.sh`, `tests/fixtures/opencode/network.sb`, `tests/fixtures/opencode/runtime-lock.json` | Test-only shell suite and validator for the runtime variables exported by the active development environment, Darwin network policy template, and exact runtime URL/integrity/version/help provenance. These scripts never invoke Nix or Fish. |
| `run-tests.sh` | Validate the active-shell runtime variables using `resolve-runtime.sh` before running the existing scope command; preserve feature-root selection, tag filters, exit propagation, and one result per scope. |
| `flake.nix` | The sole Nix expression: inline the test-only pinned OpenCode runtimes, latest-LTS Node, required modules, and existing development tools; export the `GIVN_OPENCODE_*` runtime paths from the dev-shell hook. This is the only repository file containing Nix expressions or invoking Nix. No production npm dependencies. |
| `givn/specs/fragments/fragment.md` | Durable capability-aware interaction inventory already maintained before delta authoring. No new `usecase.md` for the reusable fragment. |

The existing E2E fixture at `llm_attribution_e2e_steps.rs:23–26` compares the
embedded asset with the manually editable root plugin. Remove that dependency
from the fixture, not from the user file. Compare the generated temporary
project's installed canonical file against the embedded asset. Separately
compare baseline behavior against the immutable prototype fixture; do not
mistake byte parity with the asset for compatibility or behavior preservation.

```mermaid
flowchart LR
    Init[Initialize project] --> Sync[Existing selected-target generation]
    Skills[Synchronize skills or add target] --> Sync
    Addons[Enable or disable addon] --> Sync
    Upgrade[Witnessed managed upgrade] --> Sync
    Asset[One embedded attribution asset] --> Sync
    Sync --> File[Canonical generated plugin]
    File --> V1[Real V1 loader calls server]
    File --> V2[Real V2 loader calls setup]
    V1 --> Env[Shell attribution environment]
    V2 --> Env
    Env --> Stamp[Existing generic commit stamper]
```

### Regeneration coverage and unchanged preconditions

| Action | Existing production path | Fixture precondition |
|---|---|---|
| init, including reinit without `--force` | `cli::init::run` → `skills::targets::sync` | Writable fresh project; reinit has saved targets and `--no-addons` to avoid unrelated addon writes. |
| skills sync | `cli::skills::run_sync` → same sync | Existing config selects OpenCode; obsolete canonical file seeded in fixture. |
| skills targets add | `cli::skills::run_targets_add` → persisted target addition → same sync | Add OpenCode to Claude-only, or add Claude with OpenCode already selected. |
| addons enable/disable | `cli::addons::run_enable/run_disable` → same sync | Toggle `dev_principles`, which has no external dependency. |
| upgrade | `cli::upgrade::run` → managed preparation in `cli::migration` → same sync | Clean Git fixture with `rules: versioned`, both OpenCode and Claude selected, no pending migration, no migration-bearing release gap. Commit the missing/obsolete state before invoking upgrade. |

For the missing-plumbing variant remove and commit the original scenario's
shared hook and both selected attribution integrations; its managed revision
must restore all of them. For the V1-only variant record obsolete content only
at the canonical OpenCode file; require that file in the upgrade revision, not
unchanged hook/Claude files. Compare all refreshed bytes against embedded
assets and keep the existing allowlist-only assertion. Test-only OpenCode
configuration and probe scripts are created after upgrade, never left dirty
before its witness.

Initialization catches generation errors and warns; the other commands retain
their existing error propagation. This change does not repair that policy.
Tests require successful generation and reject its warning in the happy path;
an exit code alone is insufficient. Missing executable, tool timeout, malformed
provider output, omitted shell result, and loader failure are hard test failures,
not skips. A pending-migration upgrade may return before generation; it is not
claimed to refresh attribution.

### Retained runtime semantics

`rememberModel` accepts `id` or `modelID`; it retains provider, model and variant
from the current session when omitted. An explicit variant overrides a model
variant, which overrides the remembered variant. A record with a model identity
updates global `lastModel`, including non-session observations. An event with
neither model nor variant is ignored. Module state remains ephemeral and
unbounded for the plugin instance's lifetime.

V1 returns `chat.message`, `chat.params`, and `shell.env`. It uses the session
record or global last record. V2 registers exactly `context`, `compaction`,
`generate`, and shell `create.before`; no `title` or `prompt` hook is added. V2
tries a usable session record, then `lastModel`, then `ctx.model.default()` only
when neither has a usable model identity. Default lookup returns the model
directly, not a V1 HTTP response envelope; its errors remain swallowed.

Both generations initialize `GIVN_ATTRIBUTION_ACTIVE=1`, model and reasoning
to `unavailable`, and harness to `opencode`. With a usable identity they stamp
`GIVN_ATTRIBUTION_LLM` and `GIT_COMMITTER_NAME`; reasoning changes only for a
truthy remembered/default variant. Without a usable identity the prior Git
committer name stays unchanged. Do not invent an author-name/email change.

The verified V2 shell lifecycle invokes `create.before` before the shell-tool
callback adds `OPENCODE_SESSION_ID`. Normally this field is therefore absent
at the prototype's lookup point. A one-session real-runtime happy path can
succeed through `lastModel` without proving session-correlated accuracy. Keep
that ordering; fixture-supplied session environments characterize the explicit
session branch without pretending it is always available in production.

```mermaid
sequenceDiagram
    participant Developer
    participant Runtime as Real OpenCode V2
    participant Plugin as Generated attribution adapter
    participant Twin as Deterministic provider twin
    participant Shell
    Developer->>Runtime: Run fixture prompt with selected model and variant
    Runtime->>Plugin: context with session and model
    Plugin->>Plugin: Retain model and variant
    Runtime->>Twin: Model request with shell tool definition
    Twin-->>Runtime: Fixed safe shell tool call
    Runtime->>Plugin: create.before with environment
    Plugin->>Plugin: Session if available, otherwise lastModel/default
    Plugin-->>Runtime: Attribution environment
    Runtime->>Runtime: Shell-tool callback adds current session ID
    Runtime->>Shell: Execute fixed fixture command
    Shell-->>Runtime: Actual environment JSON
    Runtime-->>Developer: CLI tool result and deterministic final text
```

Map access/update is expected O(1) per observation and shell event; state uses
O(number of observed sessions) memory. Default lookup is one asynchronous
fallback, not a new retry policy. No performance optimization is warranted.

## Technology decisions and verified sources

All research below was checked on 2026-10-04. Context7 resolution was attempted
in this session and failed with an invalid API key; official V2 documentation
and exact released source were the fallback. V1 docs/source were used only as
migration inputs, never to infer V2 fields.

| Technology | Choice and freshness/provenance |
|---|---|
| Rust and embedding | Existing Rust 2024 edition from `Cargo.toml`; latest stable Rust resolved through the locked Nix development environment at setup. Existing `rust-embed`; no new production library. |
| Cucumber/Gherkin | `cucumber` 0.23.0 and `gherkin` 0.16.0 verified in `Cargo.lock`; preserve lockfile-resolved versions. |
| Runtime scripting | Plain JavaScript ES modules; Node latest LTS resolved from official Node release metadata at setup and pinned in the development-shell lock. Native OpenCode binaries remain the real loaders. |
| V1 floor | `opencode-ai` 1.18.29, floor verified in the official plugin migration guide and exact tag's `plugin/index.ts` and `plugin/shared.ts`. |
| Latest stable V1 | `opencode-ai` 1.18.34 from `https://registry.npmjs.org/opencode-ai/latest`; exact tarball and npm integrity recorded at setup. Refresh latest stable within V1 before implementation, then pin the resolved release. |
| Latest stable V2 | `@opencode/cli` 2.0.22 from `https://registry.npmjs.org/@opencode%2fcli/latest`; local `opencode --version` also returns `v2.0.22`. `@opencode/core` 2.0.22 released npm metadata and chunks verified. Refresh latest stable within V2 before implementation, then pin the resolved release. |
| Provisioning | `flake.nix` alone owns pinned fixed-output release downloads and module provisioning, using `flake.lock` and `runtime-lock.json` metadata for supported Darwin/Linux platforms. Its development-shell hook exports absolute runtime paths. Test scripts only validate those already-exported paths and versions; they never call Nix, Fish, a registry, or an installer. Enter the development environment before invoking test commands. |
| Process/terminal driver | Rust `std::process::Command` through existing common fixtures, real OpenCode CLI, separate E2E bindings. PTY scenarios use the platform-provided `script` command with Darwin BSD syntax or util-linux syntax on Linux, supplied by the active development shell. No browser, database service, container engine, or paid provider is needed. |

Primary official sources:

- `https://opencode.ai/v2/docs/build/plugins/migrate-v1` — shared `server/setup`
  shape and floor; register request kinds separately, APIs are not translated.
- `https://opencode.ai/v2/docs/build/plugins` — full Promise API, models,
  session-hook payloads, shell event, plugin listing and lifecycle.
- `https://opencode.ai/v2/docs/migrate-v1` — migration input and V2-native
  provider/model/variant shapes.
- `https://opencode.ai/v2/docs/cli`, `/cli/commands`, `/config`, `/providers`,
  `/models`, `/plugins`, `/network` — private service, run flags, native provider
  configuration, model variants, discovery, global update suppression, proxy.
- `https://raw.githubusercontent.com/anomalyco/opencode/v1.18.29/packages/opencode/src/plugin/index.ts`
  and `shared.ts` — default object read happens before legacy named-export
  enumeration and returns after adding its server hooks. Keeping the named
  function does not activate the canonical source twice at the supported floor.
- The same tag's `cli/cmd/run.ts` — V1 `--model` uses provider/model and has
  separate `--variant`, `--auto`, and `--format json`; no V2 standalone flag.
- `https://unpkg.com/@opencode/core@2.0.22/dist/chunks/instruction-discovery-wbx60fzd.js`
  — released V2 loader decodes only `module.default` with `id` and
  `setup/effect`; ignores `server` and the named V1 export.
- Released chunks `instruction-discovery-rz6t0wqc.js` and
  `instruction-discovery-0n82x6dt.js` — shell hook precedes session-ID injection.
- Released chunk `instruction-discovery-4g11qfmx.js:190–303` — models.dev uses
  a bundled snapshot and background catalog refresh from
  `https://models.opencode.ai/api.json`; do not guess a disable-fetch env flag.
- `https://raw.githubusercontent.com/anomalyco/opencode/v1.18.29/packages/core/src/models-dev.ts:160–176`
  — V1 floor also defaults to `https://models.opencode.ai/api.json` (with
  `OPENCODE_MODELS_URL` override). No override is needed: intercept the exact
  default catalog request in the fixture proxy. Inspect the latest pinned V1
  source at setup to detect drift, not to choose an unspecified architecture.
- `https://opencode.ai/v2/openapi.json` — `server.info` is `GET /api/info`,
  including `pid`, `version`, `urls` and `paths.tmp`; `plugin.list` is
  `GET /api/plugin` with deep-object `location[directory]` query encoding.

Loader-source inspection proves export selection, while the E2E matrix proves
the generated artifact works through the installed release. Environment writes
are idempotent: counting them is not an activation-count proof. Inspect the V2
active plugin list for exactly one canonical local source/ID where available;
V1 source and a fixture with only the canonical external plugin bound the
evidence. Do not add an unrequested exactly-once-hook product requirement.

## Local runnability and digital twins

### One command and complete lifecycle

Local setup commands: `tests/fixtures/opencode/run-suite.sh --regular` and `tests/fixtures/opencode/run-suite.sh --e2e`, from an already active project development shell whose `flake.nix` hook provides pinned test-runtime variables and tools. Setup modes validate and execute permanent specs only; delta scenarios remain `@wip` until their scenario-by-scenario TDD tasks.

After all delta scenarios are green, run `tests/fixtures/opencode/run-suite.sh --all`. That final mode executes four fail-fast root/scope combinations: permanent regular, delta regular, permanent E2E, delta E2E. `--name <anchored title>` targets matching permanent and delta scenarios separately; a zero match in either tree is an error. Every mode validates exported runtimes, prepares `ready_fixture` and the supervisor, and fails on any child error; skips are not treated as passing. No host OpenCode service or existing project database is used. The wrapper uses POSIX shell and never invokes Nix or Fish.

### Shared runtime admission for every verification entrypoint

`tests/fixtures/opencode/resolve-runtime.sh` is a POSIX-shell validator, not a package resolver. It validates the runtime paths exported by `flake.nix` against `runtime-lock.json`, including exact versions, required CLI flags, fixture modules, and platform isolation prerequisites. It never starts Nix or Fish, changes PATH to a host-selected OpenCode, installs packages, or downloads anything. Missing variables or mismatched runtimes fail with a named setup error before Cucumber starts.

`run-tests.sh` and `run-suite.sh` call this validator directly before fixture builds or Cucumber. Direct Rust test invocations also validate exported runtime variables in `tests/common/opencode_runtime.rs`; they do not start a resolver process. The caller's `GIVN_FEATURES`, result/tag/name/profile variables, `CARGO_BIN_EXE_givn`, and supervisor path remain unchanged. The test wrapper builds the supervisor before scenarios and exports its exact Cargo-reported path. `run-suite.sh --regular` and `--e2e` target permanent specs during setup. `run-suite.sh --all` adds the delta root only after all delta tasks are green. The flake shell hook is the only place that declares/provisions Nix derivations and exports runtime paths.

Each compatibility example owns a unique temporary root under the approved
OpenCode temporary directory on Darwin; explicitly set `TMPDIR`. Set fresh
`HOME`, `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_CACHE_HOME`,
`XDG_STATE_HOME`, and Git config locations; use `env_clear` and a whitelist of
Nix executables, test variables and locale. Supply only a fake fixture key,
never inherited provider keys, tokens, proxies, MCP servers or global plugins.
Remove inherited `OPENCODE_SESSION_ID`. Verify ancestors outside the temporary
root contain no OpenCode configuration/plugin discovery directories, or fail
setup by name; V2 discovers config to filesystem root.

Generate the canonical plugin by running the compiled `givn` binary in the
temporary project. Do not copy the repository's `.opencode` tree. In particular,
`givn-attribution_v1.js` would be autodiscovered as a second plugin and is never
present in the runtime fixture's plugin discovery directory. Seed obsolete
content only at its canonical path before the tested regeneration action.

Setup smoke starts the provider twin on an owned loopback port and verifies its nonce-bearing readiness without asserting attribution. The real-loader shell-result assertion is reserved for the selected-model E2E scenario after the adapter TDD task adds the V2 entrypoint. The per-example driver starts the provider/proxy twin on kernel-assigned loopback ports. The twin publishes readiness after binding, including its port
and fake catalog. Start a real V2 foreground server using
`opencode serve --hostname 127.0.0.1 --port <reserved-port>`, with a unique
temporary database in its XDG data directory. Reserve an ephemeral port for
the server, close the reservation immediately before spawning, and retry a
bounded number of times on bind collision. Probe its actual `/api/info` through
the CLI/API before running the prompt; require the reported PID to equal the
spawned server process, version to equal the pinned release, and `paths.tmp`
to lie inside the example's temporary root. Reject accidental connection to
an unrelated process. Twin readiness includes a driver-generated nonce;
OpenCode's server-info API is not assumed to expose a nonce. V1 `run` uses its real
in-process server, without shared host service discovery.

RAII alone cannot clean up after the runner receives SIGINT/SIGTERM or calls
`process::exit`. Each example therefore uses a separate test-only supervisor
executable, `tests/opencode_supervisor.rs`, declared as a `harness = false`
Cargo test target and built with `ready_fixture` before scenarios. It uses
existing `libc`, `std::os::unix::process::CommandExt::process_group`,
`std::process::Child::{try_wait,wait}`, and `std::time::Instant`; no production
executor logic or new production dependency is added.

`tests/common/opencode_runtime.rs` launches the supervisor in its own process
group with piped stdin as a parent lease. Only the runner keeps the write end;
it is closed on unwind, explicit teardown, or OS process exit and is never
inherited by fixture children. The supervisor monitors lease EOF using
`libc::poll`/`read` and registers SIGINT/SIGTERM via `libc::sigaction`; handlers
only set a signal-safe flag, with cleanup in the main loop. It owns one separate
process group whose leader is Node running `runtime_driver.mjs`; the V2 server,
CLI runs, shell descendants, and provider/proxy twins inherit that group.
Children must not detach or invoke shared-service discovery. Record leader PID,
PGID and role/PID diagnostics at spawn, never infer ownership from a port.

Ready timeout is 15 seconds, provider request timeout 5 seconds, and run deadline
60 seconds. Readiness retains nonce/owned-server PID/version/tmp checks above;
early exit, wrong server, malformed readiness, bind retry exhaustion, missing
completed tool result, and timeout are hard failures with captured logs. On
completion, failure, lease EOF, signal, or deadline, the supervisor stops new
requests and signals only its registered fixture PGID with
`libc::kill(-pgid, SIGTERM)`. The Node driver uses `process.on('SIGTERM', ...)`
and `process.on('SIGINT', ...)` to abort outstanding work, close twin listeners,
stop the owned server, and await child exit/output. After 3 seconds the
supervisor sends SIGKILL to surviving members, waits for the direct child, and
confirms owned PIDs/PGID have disappeared before deleting only its owned root.
On Linux it first enables `libc::prctl(PR_SET_CHILD_SUBREAPER, 1, ...)` and
drains adopted descendants with `libc::waitpid`; failure is a setup failure.
On Darwin orphan reaping belongs to the OS; PID/group disappearance polling
has a further 5-second bound after direct-child reaping. Teardown failure remains
non-zero even if the attribution assertion succeeded.

Mandatory controlled probes interrupt the runner with SIGINT and SIGTERM,
exercise runner process exit/lease EOF, and force the run deadline while an
owned server, runtime descendant, and twins are alive. Each must demonstrate
escalation/reaping and closed owned ports, while an unrelated user-service
sentinel PID/listener remains alive and responsive. Record observed PIDs,
signals, exit status and logs; these probes have not run. Simultaneous external
SIGKILL of the supervisor itself is outside this controlled-interruption proof,
not an RAII guarantee. Never kill a process found only by a port number.

### Concrete Darwin network isolation

The development environment provides dependencies, not a network sandbox. Use `/usr/bin/sandbox-exec`
with a generated Seatbelt profile for each OpenCode process and all of its
descendants. The checked profile shape is:

```scheme
(version 1)
(allow default)
(deny network*)
(allow network-bind (local ip "localhost:*"))
(allow network-inbound (local ip "localhost:*"))
(allow network-outbound (remote ip "localhost:PROVIDER_PORT"))
(allow network-outbound (remote ip "localhost:PROXY_PORT"))
(allow network-outbound (remote ip "localhost:SERVER_PORT"))
```

Replace only the three driver-owned outbound ports with decimal bound values;
omit the server allowance for in-process V1. A literal `127.0.0.1:*` profile
address was rejected by the local parser; use its supported `localhost` form.
Do NOT use `(allow network* (local ip "localhost:*"))`: a planning probe found
that this incorrectly permits non-loopback outbound sockets. The separated
bind/inbound/remote-outbound profile was exercised on this host: local TCP
returned `loopback:local`, and TCP to reserved non-loopback `192.0.2.1:443`
failed immediately with `EPERM`. Repeat that proof at setup, with the narrowed
actual-port allowances and an unrelated loopback listener as a negative probe.
Allowed ports are fixture-owned; connections to other host services are denied.

A second planning probe exercised a concrete owned-port allowance: its
fixture TCP connection returned `owned:owned`, a different live loopback
listener failed with `other:EPERM`, and the non-loopback connection failed
with `remote:EPERM`; the probe exited 0. This establishes the narrowed profile
behavior on this Darwin host, not merely successful profile parsing.

Wrap twins in their own profile with no network-outbound permission. They accept
only driver/OpenCode loopback clients and cannot forward to a live provider.
The runner additionally checks nonce-bearing readiness and distinct allocated
ports per example. Filesystem home/config isolation and port ownership provide
the equivalent per-project local stack; no shared database is assumed. For
Linux CI use an unshared network namespace containing only the loopback
interface and these owned processes; a capability/namespace failure fails
setup, never silently falls back to host networking. Darwin Seatbelt is a
deprecated system tool: absence or inability to apply it is a named setup
failure, not authorization to weaken network evidence.

### External systems and twins

| External dependency | Local deterministic twin/control |
|---|---|
| Paid/model-provider API | `provider_stub.mjs`: actual loopback HTTP server implementing OpenAI-compatible chat completion JSON and streaming SSE. Fake `givn-fixture/model-a`, `model-b`, and `title-model`; no real credentials or model inference. |
| models.dev/catalog refresh | The same fixture's HTTPS-intercepting proxy serves fixed `/api.json` catalog JSON (empty catalog is valid; the fixture's models are configured explicitly). Preprovision a fixture CA/key in the development closure with OpenSSL; trust only in the test process with `NODE_EXTRA_CA_CERTS`. `HTTP_PROXY` and `HTTPS_PROXY` point at the local proxy; `NO_PROXY=localhost,127.0.0.1,::1` leaves owned local services direct. Support CONNECT only to `models.opencode.ai:443`, verified for V1 floor and V2; latest-release drift requires updating an explicit pinned fixture mapping. Log and fail on unknown destinations, never forward traffic. The proxy is test infrastructure, not a plugin. |
| OpenCode update service | Disable V2 updates with global `update: "disable"` in the temporary XDG config, as documented; V1 with `autoupdate: false`. Any unexpected residual update request is rejected/recorded by the proxy and OS sandbox; no live update/registry request can occur. |
| npm/provider/plugin package acquisition | The test-only hash-pinned development closure supplies native runtimes and all modules before scenario execution. No local package plugin and no runtime install; unplanned acquisition is a failing fixture prerequisite. |
| Git/Fossil and OpenCode itself | Real provisioned executables, not API twins. OpenCode's real loader is indispensable compatibility evidence. The existing Fossil fixture uses a private repository. |

The fake provider recognizes request kind from actual messages/tool schemas,
not a plugin-only test switch. Primary requests first return one tool call to
the runtime's advertised `shell` (V2) or `bash` (V1), using the exact advertised
schema. Arguments select only the fixed safe command
`node attribution-fixture.mjs` in the temporary project; it prints a sentinel
and JSON containing the four Givn attribution variables and Git committer name.
The next request containing that tool result returns deterministic final text.
Non-streaming and SSE formats include role, tool-call ID, argument deltas,
finish reason and usage as required by the pinned runtime. Title requests
return a fixed title without tools; compaction and transient-generation
requests return fixed text. Requests beyond this finite state machine fail
with diagnostic fixture output. No arbitrary model-supplied command is run.

### Verified runtime command and configuration shapes

V2 `run --help` was checked locally at 2.0.22; `--server`, `--standalone`,
`--format json`, `--auto` and model `provider/model#variant` are supported.
Use the owned foreground server for continued diagnostics and active-plugin
inspection:

```sh
opencode run --server http://127.0.0.1:SERVER_PORT --format json --auto --model givn-fixture/model-a#high 'Run the attribution fixture command.'
```

An equivalent fresh private-server diagnostic run is
`opencode run --standalone --format json --auto --model givn-fixture/model-a#high 'Run the attribution fixture command.'`;
it must receive the same isolated environment and no host service. It is not a
second required E2E action. V1 uses:

```sh
opencode run --format json --auto --model givn-fixture/model-a --variant high 'Run the attribution fixture command.'
```

Driver argv is constructed directly, not shell-evaluated. Preserve raw CLI
output, then parse actual tool-result events and final text from NDJSON. Primary
compatibility evidence is the real command's successful acknowledged tool
result containing the actual spawned environment, not the provider's final
text (which could repeat invented values). Installed-file parity, repository
state and plugin diagnostics are secondary checks.

The shared regeneration assertion checks all four attribution variables and
the committer name from that completed tool result, not only the two model and
reasoning fields named by the step. Check installed bytes, only one canonical
external plugin path, and the absence of generation warnings as secondary
evidence. The full existing supplied prototype remains the behavior oracle;
remove the one added `server()` property for the structural baseline comparison
and demand exact remaining text equality, not a broad regex allowing edits.

Native V2 fixture project config uses the official V2 shape below; its global
temporary config additionally disables updates. Substitute the bound URL and
configured shell path, retaining the published schema URI for editor
metadata without fetching it. The test orchestration scripts use POSIX `sh`; the configured fish executable remains a pinned test dependency only where a fixture explicitly needs it. This is not proof of the built-in tool's selected shell: released V2 compatibility-priority shell
selection rejects fish and falls back to `/bin/zsh` on Darwin. Runtime admission
must record the actual selected executable and compatibility-selection reason
from the pinned runtime, require that executable to exist, and retain a fixed
`node attribution-fixture.mjs` command valid in that shell. Linux admission
records its own actual selection; do not assume Darwin's fallback everywhere.
This corrects test setup only; no production shell-selection logic changes.

```json
{
  "$schema": "https://opencode.ai/config.json",
  "model": "givn-fixture/model-a",
  "shell": "NIX_FISH_PATH",
  "snapshots": false,
  "providers": {
    "givn-fixture": {
      "name": "Attribution fixture",
      "env": ["GIVN_FIXTURE_API_KEY"],
      "package": "@opencode/ai/providers/openai-compatible",
      "settings": {"baseURL": "http://127.0.0.1:PROVIDER_PORT/v1"},
      "models": {
        "model-a": {
          "capabilities": {"tools": true, "input": ["text"], "output": ["text"]},
          "limit": {"context": 32768, "output": 4096},
          "variants": [{"id": "high", "settings": {}}]
        },
        "model-b": {"variants": [{"id": "low", "settings": {}}]},
        "title-model": {}
      }
    }
  },
  "agents": {"title": {"model": "givn-fixture/title-model"}},
  "plugins": ["-opencode.provider.ollama", "-opencode.provider.lmstudio", "-opencode.provider.vllm"],
  "permissions": [{"action": "shell", "resource": "node attribution-fixture.mjs", "effect": "allow"}]
}
```

Deny other model-issued tools in the fixture agent's ordered permissions; keep
the built-in shell available, not a fake custom tool. V1 config uses singular
`provider`, provider `npm: "@ai-sdk/openai-compatible"`,
`options.baseURL`/fake `options.apiKey`, model `tool_call: true`,
`variants: {"high": {}}`, singular `agent`/`permission`, `snapshot: false`, and
`autoupdate: false`. Provide its preprovisioned AI SDK module through the pinned
runtime's normal resolution closure, never an online install. The legacy
fixture configuration is generated separately; do not mix nested V1 and V2
provider shapes. Recheck exact runtime help/config acceptance at setup against
both pinned V1 releases. Unknown flags or configuration warnings fail the
driver's admission check and require correcting fixture config, not weakening
the scenario or changing attribution logic.

For V2 diagnostic listing on the same owned server/location use a real HTTP
request to `GET /api/plugin?location[directory]=<URL-encoded-project-root>`.
Build the query with `URLSearchParams`, verify `plugin.list` against that
server's `/openapi.json`, and record the returned canonical local path,
`givn-attribution` ID and active state. The official published OpenAPI was
checked at design time. `opencode plugin list --help` has only `--builtin`,
not `--server`, so do not invent a CLI flag for this secondary diagnostic.
This is secondary inspection on the owned loopback server, not a weaker
replacement for CLI shell-result assertions. No extra SDK dependency or
attribution helper plugin is required.

## Executable specification, commands and strictness

`givn/commands.yaml` currently declares `verify.command: "./run-tests.sh"` and
`verify.e2e_command: "./run-tests.sh --e2e"`. Keep those configured values and
the existing runner's scope/result contract. `run-tests.sh` defaults to
nextest; `GIVN_TEST_RUNNER=cargo` selects plain cargo. Default runs all tags,
`--non-e2e` excludes E2E, and `--e2e` includes only E2E. It accepts exactly one
feature root from `GIVN_FEATURES` and does NOT combine permanent and delta
trees. For setup, `run-suite.sh --regular` and `--e2e` validate the permanent root only; this allows environment verification while all delta scenarios remain `@wip`. The final `--all` wrapper explicitly executes both roots after every delta scenario is green, using the same configured command per root and failing if any invocation fails:

```sh
env GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e
env GIVN_FEATURES=givn/changes/fix-opencode-v2-attribution/specs ./run-tests.sh --non-e2e
env GIVN_FEATURES=givn/specs ./run-tests.sh --e2e
env GIVN_FEATURES=givn/changes/fix-opencode-v2-attribution/specs ./run-tests.sh --e2e
```

These are ordered fail-fast shell invocations inside `run-suite.sh --all`, not four independent manual setup requirements. Every call uses the same shell-only runtime validation, including direct configured-gate invocation without the wrapper. Do not modify
runner behavior to select deltas by
filesystem sniffing. Givn gates already scope `GIVN_FEATURES` to a change; the
wrapper's explicit full validation adds permanent coverage rather than
claiming the single scoped gate discovered both. When `GIVN_RESULT_FILE` is
present, retain the runner's one-process-per-scope cargo path and do not merge
two root results into a fabricated receipt.

Exact single-scenario invocation; the common fixture validates already-exported runtime paths with the shell validator when wrapper-private admission JSON is absent:

Prerequisite: setup has already built the matching `opencode_supervisor`
artifact and supplied its executable path as `GIVN_OPENCODE_SUPERVISOR`, as
specified under shell-only runtime validation. The validator checks exported runtimes; it does not build the supervisor during scenario execution. Missing prepared artifact
is a named setup failure, not an on-demand build or skip.

```sh
env GIVN_FEATURES=givn/changes/fix-opencode-v2-attribution/specs cargo test --locked --test features_runner -- --name '^OpenCode prepares attribution using its selected model$'
```

The whole outline, including its release Examples, is one named scenario.
`--name` and `--tags` are mutually exclusive in this runner. Do not combine
them. The wrapper's `--name` runs the exact filter against both feature roots separately and fails if either has no match. A permanent run of a modified title and its delta run are two distinct
feature inputs before archive, not an instruction to duplicate scenario bodies.

Keep old permanent step text bindable until archive merges the delta. In
particular, the existing OpenCode selected-provider/model step records the
requested public selection; its shell-preparation step now drives a real
V1-floor CLI run against the same generated file and local provider twin,
configuring the fixture's explicit requested provider/model alias and high
variant. The arbitrary legacy session label is test selection state, not an
attempt to manufacture OpenCode's runtime session ID. New delta step text uses
the explicit release matrix. Both paths read the actual completed tool-result
environment; neither uses the old Node-only hook invocation as E2E evidence.
This compatibility glue lets permanent and delta verification both run before
the modified scenario replaces its permanent title. After archive, remove
unused legacy test glue only when no retained scenario uses it; never delete
user-owned plugin content to make the tests pass.

Strictness already uses `.fail_on_skipped()` at
`tests/features_runner.rs:54–64` for cargo and `:125–133` for nextest. Plain
runner explicitly exits non-zero on failed OR skipped scenarios. New Rust
bindings start with `unimplemented!("<step meaning>")`; no empty/no-op body,
`assert!(true)`, or catch-and-return-success. All added/modified scenarios stay
`@wip` until their bindings and scenario execution are genuinely green.

Before the first scenario, setup proves strictness and validates the environment using permanent-root gates only. The final `--all` run is after all delta scenarios are green. Setup verification independently runs two temporary Gherkin probes:
one undefined step, and one registered step that explicitly panics with
`unimplemented!()`. Repeat each under the cargo path, nextest custom harness,
and `--e2e` path with an `@e2e` probe. Each must exit non-zero and identify the
undefined/pending-or-unimplemented failure, not a setup/network failure.
Remove only temporary proof inputs afterward. This proves exact config, not
an assumed Cucumber default. Planning inspected the configured mechanism but
did not create bindings or execute these probes.

The delta contains 18 scenario identities: 15 added, 3 modified, none removed.
Seven E2E outlines cover distinct changed actions; 11 regular scenarios cover
separate edge/variant invariants. Expanded Examples yield 25 E2E executions
and 24 regular executions, 49 total. Feature tags are `@givn.delta` and exact
capability; every delta scenario is `@wip`. The permanent fragment retains
its other E2E actions without copying their scenarios into this change.

### Why regular characterization is retained

These public-hook tests are deliberately not advertised as real-loader
compatibility. They execute both immutable prototype and generated module in
fresh processes, constructing identical consumer payloads and comparing
observable environments. Use named V1 hook consumer and default V2 setup
consumer; fresh module loading avoids global/session state leakage. Inspect
registration names against the immutable prototype to catch adding a title
hook or omitting an existing hook. Do not fake current OpenCode payloads for
the E2E matrix.

| Regular invariant | Concrete coverage not provided by one-session real-loader happy path |
|---|---|
| Identifier spelling | Controlled `id` versus `modelID` payloads; real releases choose their own one spelling. |
| Session precedence | Two observations in different sessions with an explicitly supplied session shell environment. |
| Omitted variant retention | Model changes while variant is absent; successful high-variant E2E cannot detect clearing. |
| Global fallback | Unknown/absent session after a different session's observation; intentionally retains risk. |
| Remembered versus default priority | Contradictory remembered/default models, not one model. |
| Configured default only | No observed model before shell creation; E2E has a primary context event. |
| Unavailable/failing fallback | Missing/identityless/throwing default with preexisting committer; no live provider failure needed. |
| Non-title request kinds | Explicit primary/compaction/transient-generation inputs; E2E primary happy path is not compaction proof. |
| Reinit without force | Snapshot of saved config remains equal while canonical generated behavior refreshes. |
| No addons | Generation remains independent of addon enablement. |
| No OpenCode target | Negative installation/report behavior for Claude-only configuration. |

Keep generic stamper's existing human/partial-context/model-change tests; they
prove no fabrication by that consumer of supplied variables, not adapter
session accuracy. Keep existing Git/Fossil/Claude E2E drivers as regression
evidence. No separate hand-maintained test file duplicates feature assertions.

Existing coverage scripts remain authoritative. When running instrumented
coverage, the Cucumber runner binds the same feature files and each spawned
production Givn process must use its instrumented `CARGO_BIN_EXE_givn`, with
collision-safe `LLVM_PROFILE_FILE` paths inherited through the process
whitelist and flushed on orderly exit. Merge both runner and Givn profiles
before export; a non-zero `skills::targets` generation path and attribution
stamper path are the expected production probes. External OpenCode/Node
runtimes are not Givn production coverage targets. Do not substitute an
un-instrumented installed global Givn during measured E2E runs.

## Scenario revision evidence exception

Operator authorization: **2026-10-04, exact answer “1”**, to the sole
characterization/reuse evidence question recorded in `design-review.md`.
Limited to `fix-opencode-v2-attribution`: retain one atomic `givn commit` per
scenario. A genuine characterization/reuse revision may be test-only rather
than touching production source, but must contain real executable assertions,
targeted RED/GREEN/REFACTOR execution evidence, and an explicit reference to the
production-changing adapter revision it verifies. For example, the omitted
reasoning-mode scenario can commit its assertion that `high` is retained and
reference the adapter repair rather than rewrite that asset again.

RED must be a genuine targeted failure: record an undefined/unimplemented
binding failure before adding assertions, or an assertion-bearing negative
baseline/control that exposes the relevant invariant. If assertions already
pass on the repaired adapter, record that characterization honestly; never
pretend the retained behavior was defective or fabricate a product RED.
The scenario's evidence must show which failure was corrected and what the
subsequent assertion-bearing GREEN verifies. Reject empty steps, artificial
production edits/comment churn, repeated plugin rewrites, fabricated RED,
spec-only commits, batching scenario commits, or removal of `@e2e`. All other
completion-review, strictness, coverage, terminal-evidence and archive checks
remain mandatory. This is an explicit change-local evidence exception, not a
global workflow-policy edit and not executed GREEN.

## Step Definitions

| Capability | Exact binding path | Scope / module |
|---|---|---|
| attribute-commits | `tests/steps/llm_attribution_steps.rs` | Regular; `steps::llm_attribution_steps` |
| attribute-commits | `tests/steps/llm_attribution_e2e_steps.rs` | E2E; `steps::llm_attribution_e2e_steps` |
| enable-commit-attribution | `tests/steps/llm_attribution_installation_steps.rs` | Regular; `steps::llm_attribution_installation_steps` |
| enable-commit-attribution | `tests/steps/llm_attribution_installation_e2e_steps.rs` | E2E; `steps::llm_attribution_installation_e2e_steps` |

The canonical heading/table enables the existing promised-file integrity check.
Each row has exactly one project-relative binding path in column two; shared
runtime fixtures stay in the responsibility table, not a parallel capability
binding implementation. Presence checks do not prove assertion quality or GREEN.

## Use-case traceability

The initialization use case (`initialize-project`) guarantees selected agent
integrations and truthful backend-aware guidance. The upgrade use case
(`upgrade-a-project`) guarantees protected, history-visible managed refresh.
The support fragment owns reusable attribution, not either independent goal.
The existing contracts declare no new attribution Domain Event here; observable
installation/refreshed output and revision publication remain the existing
flow outcomes, not a newly minted event vocabulary.

| Contract concern | Technical realization | Capability/evidence | Confirmed Persona impact |
|---|---|---|---|
| Initialization success: chosen integrations work | Same selected-target writer, dual entrypoint asset | `attribute-commits`: “Consumer initializes a project and receives shared attribution plumbing” | Developer uses either supported harness generation without repair; agent receives executable context; maintainer checks matrix. |
| Target-selection rule and rejected-request protection | Keep persisted targets and validation; negative no-target regular invariant | `enable-commit-attribution`: “Consumer adds an agent target and receives compatible OpenCode attribution”; existing target-selection scenarios retained | Developer's choice remains effective; no new Persona. |
| Addons only by explicit decision | Attribution is independent of addon state; existing enable/disable policy preserved | New enable/disable E2E actions and no-addons regular invariant | Developer/agent keep working after explicit configuration changes. |
| Report names installed/skipped behavior | Preserve existing CLI text; require returned report and real shell acknowledgement | Existing “Consumer sees the commit attribution installation report”; sync/target/addon E2E reports | Developer can verify the reported installation; maintainer sees no new text contract. |
| Upgrade minimal guarantee and clean-state witness | Commit fixture's missing/V1-only state first; existing refusal/witness unchanged | “Consumer upgrades an existing project with attribution plumbing missing”; existing dirty/no-SCM upgrade tests retained | Developer's work is protected; maintainer can review managed history. |
| Upgrade success and allowlist-only revision | Existing managed-generated-path allowlist includes canonical plugin only for selected OpenCode | Same upgrade E2E, missing/obsolete Examples; revision check secondary | Maintainer gets explicit retrofit history, no extra integration path. |
| Backend attribution limitations | Generic Git hook and Fossil `givn commit` remain unchanged | Existing attributed-Git/Claude E2E and Fossil installation E2E | Developer's backend is not misrepresented. |
| Ownership/reuse | Two EXTEND decisions; fragment inventory instead of including-use-case duplication | Both delta files under `specs/fragments/`; no new use case | Agent and maintainer retain one support contract. |
| Runtime context optional/no fabrication in generic stamper | Preserve unavailable defaults and swallowed fallback; narrow adapter exception | Unknown/default/failed fallback regular invariants, existing human/partial commit cases | All three Personas must read inferred model labels as best effort, not verified current-session identity. |
| Events/flow outcomes | No new Domain Event, Aggregate or persisted model state; reports and managed revision remain existing outcomes | Runtime sequence above and CLI transcripts | Agent/maintainer avoid fabricated modeling entities. |

## Interaction Coverage Matrix

Rows below cover all 38 normalized rows now recorded in the durable support
fragment. Non-attribution rows are retained inventory evidence, not extra
implementation scope. Their existing driver files are explicitly named;
limitations in historical file-only assertions remain preexisting and are not
claimed repaired by this compatibility change.

| Inventory entry (capability/action) | Exact @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| attribute-commits / initialize shared attribution | Consumer initializes a project and receives shared attribution plumbing | CLI | Compiled givn init subprocess in Git fixture, stdout report, then actual provisioned OpenCode run acknowledges shell environment; `llm_attribution_e2e_steps.rs`. |
| attribute-commits / attributed Git commit | An attributed commit exposes the current LLM and harness | CLI | Existing real Git commit subprocess and Git show readback; same E2E file, unchanged. |
| attribute-commits / OpenCode attributed shell | OpenCode prepares attribution using its selected model | CLI | Actual V1/V2 run with loopback provider tool call, parse completed shell result and captured actual environment; same E2E file, new shared runtime driver. |
| attribute-commits / Claude attributed commit | Claude attributes a commit using its current model | CLI | Existing generated Claude hook invoked with fixture transcript/PreToolUse JSON, real wrapped Git command and Git show; same E2E file, unchanged. |
| enable-commit-attribution / read init report | Consumer sees the commit attribution installation report | CLI | Existing compiled givn init output assertions; `llm_attribution_installation_e2e_steps.rs`. |
| enable-commit-attribution / upgrade retrofit | Consumer upgrades an existing project with attribution plumbing missing | CLI | Real givn upgrade on clean committed missing/obsolete state, stdout refresh and OpenCode completed shell result; same installation E2E file. |
| enable-commit-attribution / Fossil init | Consumer initializes a Fossil project and receives the givn commit path without a hook | CLI | Existing real Fossil checkout and givn init report; installed path limitation asserted; installation E2E file. |
| enable-commit-attribution / skill sync | Consumer synchronizes skills and retains compatible OpenCode attribution | CLI | Real givn skills sync followed by actual OpenCode run against regenerated file; installation E2E file plus runtime driver. |
| enable-commit-attribution / target addition | Consumer adds an agent target and receives compatible OpenCode attribution | CLI | Real givn skills targets add with target variants, report plus OpenCode shell result; same files. |
| enable-commit-attribution / addon enable | Consumer enables an addon and retains compatible OpenCode attribution | CLI | Real givn addons enable dev_principles, report plus OpenCode shell result; same files. |
| enable-commit-attribution / addon disable | Consumer disables an addon and retains compatible OpenCode attribution | CLI | Real givn addons disable dev_principles, report plus OpenCode shell result; same files. |
| find-existing-scenarios / search | Lexical search locates a permanent scenario by a step text token | CLI | Existing `spec_discovery_deterministic_steps.rs`: real givn spec find subprocess and matching title/owner stdout. |
| find-existing-scenarios / tree | The spec tree reports per-use-case counts and the use-case narrative | CLI | Same existing driver invokes givn spec tree and reads names/counts/actor stdout. |
| coverage-readme-example / sync coverage guidance | Coverage-enabled project receives a complete README coverage example | CLI | `coverage_readme_example_steps.rs`: real skills sync; existing generated archive guidance readback. |
| show-coverage-in-the-readme / sync badge guidance | Coverage-enabled project is guided to place the coverage badge directly after the README title | CLI | `readme_coverage_badge_steps.rs`: real skills sync and existing archive guidance readback. |
| detect-the-project-repository / root Git upgrade | A Git worktree at the project root selects the Git backend | CLI | Existing SCM/backend fixture steps plus common command driver invoke real upgrade and revision readback. |
| refresh-generated-guidance / sync gate guidance | Synchronized guidance explains how to resolve the deterministic gates | CLI | `tests/steps/mod.rs::user_runs` invokes real skills sync; `prose_consolidation_steps.rs` asserts generated guidance/command content, with common stdout assertions. `skill_rewrites_steps.rs` reads instructions and is not this action's driver. |
| instruction-integrity / design instructions | An author requests the technical E2E design guidance | CLI | `instruction_integrity_e2e_steps.rs`: real givn instructions design and complete returned guidance. |
| instruction-integrity / JSON instructions | An agent requests machine-readable artifact guidance | CLI | Same driver invokes instructions specs --json and parses actual stdout. |
| instruction-integrity / review instructions | A reviewer requests disposition-aware review guidance | CLI | Same driver invokes instructions review and reads disposition-aware output. |
| route-a-change / proposal route | Routing a proposal prints ranked candidates with evidence | CLI | `spec_discovery_deterministic_steps.rs`: real givn spec route against fixture proposal and stdout ranking/evidence. |
| measure-coverage-per-project / initialize scripts | An initialized project documents reusable test and coverage scripts | CLI | `coverage_script_workflow_e2e_steps.rs`: real givn init, existing commands/README public-document readback. |
| measure-coverage-per-project / archive coverage | Archiving publishes line and branch badges from the merged report | CLI | Same driver starts real archive plus fixture coverage scripts and validates acknowledgement/report/README. |
| generate-ideation-guidance / sync facilitation | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | CLI | Existing command bindings in `tests/steps/mod.rs` run skills sync; `prose_consolidation_steps.rs` reads generated skill/command content, and stdout acknowledges synchronization. |
| match-guidance-to-enabled-addons / sync filtered guidance | A consumer synchronizes skills and the generated guidance omits disabled-addon sections | CLI | `template_conditional_rendering_e2e_steps.rs`: real skills sync in addon fixture and existing guidance readback. |
| match-guidance-to-enabled-addons / enable required addons | A consumer enables the addons a generated section needs and the guidance gains it | CLI | Same driver executes real addon enable commands and reads generated guidance. |
| match-guidance-to-enabled-addons / disable required addon | A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed | CLI | Same driver executes real addon disable; Examples vary addon, readback verifies removed guidance. |
| verify-repository-ownership / review overlaps | Repository-wide review accepts explicit overlap dispositions | CLI | `repository_ownership_fixtures_steps.rs`: isolated real fixture review CLI, stdout disposition receipt. |
| verify-repository-ownership / archive consolidation | Archive publishes consolidated permanent specifications | CLI | Same driver invokes real fixture archive and reads acknowledgement/permanent tree. |
| adr-qualification-gate / strict ADR instructions | An author requests the strict ADR guidance through the Givn CLI | CLI | `adr_qualification_gate_e2e_steps.rs`: real instructions arc42-docs and complete output. |
| adr-qualification-gate / ADR-aware design review guidance | A reviewer requests design-review guidance through the Givn CLI | CLI | Same driver invokes instructions design-review and reads qualification output; not a review invocation by this planning session. |
| guide-domain-modeling / sync domain guidance | An agent synchronizes skills and the domain skill is routed | CLI | `fix_domain_modeling_skill_trigger_e2e_steps.rs`: real skills sync and existing generated skill readback. |
| guide-domain-modeling / next instructions | An agent requests next-step guidance and is routed to the domain skill | CLI | Same driver invokes real instructions and checks returned route. |
| guide-domain-modeling / specs instructions | An agent requests the specs guidance and is routed to the domain skill | CLI | Same driver invokes real instructions specs and checks returned route. |
| guide-domain-modeling / design instructions | An agent requests the design guidance and is routed to the domain skill | CLI | Same driver invokes real instructions design and checks returned route. |
| review-architecture-documentation / conformance instructions | Arc42-enabled review instructions require conformance evidence | CLI | `arc42_review_alignment_steps.rs`: existing real new/instructions review subprocess and output. |
| review-architecture-documentation / eject review template | The review template includes the Arc42 conformance block | CLI | Same driver invokes real eject review:template and reads ejected public document. |
| review-architecture-documentation / sync review guidance | Generated review guidance stays aligned after skill sync | CLI | Same driver invokes real skills sync and existing guidance readback. |

## Visual Design Contract

- Interface: terminal
- Screen scope: no new or visibly redesigned screen, browser, controls, design
  tokens, breakpoints, or motion. Browser viewports are not applicable. Although
  OpenCode serve can also host a Web UI, this repair introduces or changes no
  browser screen; the interaction is a real CLI executing the built-in shell
  tool. HTTP is used only for readiness and secondary diagnostics. The opposite
  Web UI classification therefore does not apply; no rendered-option approval
  or fictional browser evidence is required.
- Language: preserve current English CLI text and existing labels. The change
  does not add a terminal screen or style contract under `docs/design/`.
- Evidence kind: captured real terminal/subprocess transcript. For each of the
  seven affected E2E titles, combine its release/flag Examples into labeled
  sections under
  `givn/changes/fix-opencode-v2-attribution/evidence/visual/<scenario-slug>/transcript.txt`.
  Include executed argv, CLI stdout/stderr, actual shell tool result, exit
  status and release version. Do not fabricate screenshots or transcript text.
- Capture after final assertions; keep intermediate diagnostics in
  `reports/visual/`. Redact only random temp-path prefixes and credentials;
  fake model IDs, reported installed canonical paths and actual result fields
  must stay visible. JSON identifiers belong to requested machine output and
  fixture evidence, not accidental labels in human-facing installation text.
- With concurrent nextest execution, each example writes to a uniquely owned
  transcript shard in `reports/visual/`; the one-command wrapper assembles
  ordered, labeled shards for each title after successful final assertions.
  Never let parallel examples append nondeterministically to the committed
  transcript. Failure diagnostics remain separate from final pass evidence.
- Baseline checks at every terminal assertion: visible command/report labels,
  correct configured language, no unexpected internal-state text or duplicated
  content introduced by this change, and an actionable existing error report
  if the driver fails. Page headings, hover states and reduced-motion animation
  are inapplicable to these noninteractive commands, not deferred browser checks.
- Rendered reference: existing CLI installation report is the retained text
  baseline; committed reference/evidence is captured by implementation, not
  asserted as already present. V1/V2 transcripts normalize formatting only for
  parsed assertions, retaining raw consumer output in evidence.

## Retained risks and limitations

1. A command with unknown/late-injected session ID can be attributed to another
   session's last model; V2's verified hook order makes this realistic, not a
   hypothetical accuracy guarantee. No correction is authorized.
2. Configured default is a fallback model, not proof of the command's model.
   Swallowed lookup failures leave unavailable fields and preserve shell use.
3. Omitted variants retain an earlier session variant even after model changes;
   compaction/transient-generation can update the remembered model. Title
   requests are intentionally not observed.
4. Module-global state lasts until plugin unload; no persistence, eviction,
   cross-session locking, or lifetime redesign is added.
5. Older V1 is outside the support window. Future payload drift requires the
   supported real-runtime matrix to be refreshed, not runtime detection.
6. Regeneration still overwrites the managed canonical file. Arbitrary local
   edits are not preserved; this fix prevents replacement with obsolete V1-only
   content, not overwriting as a general policy.
7. Active-shell runtime validation, signal-aware teardown, and Seatbelt availability
   are test setup prerequisites. The setup smoke proves owned provider readiness and permanent runner connectivity; the real V2 attribution tool-result assertion belongs to the selected-model scenario after the adapter RED/GREEN cycle. Failure is explicit; no test can substitute Node imports,
   synthetic loader status, live credentials, or host networking. The shell-only
   validator must work through canonical gates and coverage, not only the
   wrapper. RAII is insufficient for runner process exit or SIGINT/SIGTERM;
   supervisor lease and owned-process-group probes remain mandatory and unrun.
8. Active-plugin diagnostics and idempotent outputs cannot independently prove
   every hook was called exactly once. That is not a new product requirement;
   the documented entrypoint and real-loader evidence bound compatibility.

## Durable arc42 documentation handoff

The orchestrator delegates these durable edits to its documentation writer;
this session does not write arc42 chapters or its completion marker. The
writer must first read proposal, both complete delta files and this design,
then the relevant current chapters, and record the full assessment in
`givn/changes/fix-opencode-v2-attribution/arc42.md`, ending with the required
Status section only after its edits are complete. Do not mark future runtime
tests as executed. Keep current facts distinct from planned implementation.

| Assessment row | Affected? | Exact handoff |
|---|---|---|
| 1 Goals, stakeholders, quality attributes | Yes | This row maps chapters 01 and 10: supported-generation compatibility and truthfulness refine quality attributes in chapter 10. Existing chapter-01 goals/stakeholders/Personas remain unchanged; no chapter-01 churn is required. |
| 2 Constraints | Yes | `02-architecture-constraints.md`: replace V1-only TC-29 with the public recent-V1/V2 support window and unchanged optional best-effort behavior. Avoid hook/function detail in chapters 1–3. |
| 3 External systems/interfaces/context | Yes | `03-context-and-scope.md`: OpenCode row covers both supported generations and explicitly distinguishes inferred fallback values from verified session facts; no new external production service. |
| 4 Major strategy | No | Embedded adapter, one binary and generic stamper strategy remain unchanged; no new system-wide approach or dependency direction. |
| 5 Building blocks | Yes | `05-building-block-view.md`: embedded OpenCode adapter has dual protocol entrypoints, still one source and one generation writer; no new production module. |
| 6 Runtime flows | Yes | `06-runtime-view.md`: add V2 request observation/shell environment flow, preserve V1 sequence and document late session-ID injection plus retained fallback order. Use Mermaid if diagrammed. |
| 7 Deployment | Yes | `07-deployment-view.md`: one canonical selected-target file works with the support window; no generated V1 sibling or production npm install. Test-only runtimes/twins are not new production services. |
| 8 Cross-cutting concepts | Yes | `08-crosscutting-concepts.md` attribution environment and generation sections: describe session/last/default adapter behavior and preserve generic stamper no-fabrication/unknown handling. |
| 9 Architecture decisions | No | No qualified new/amended/superseding ADR. Rationale is local/reversible and completely owned by this design plus feature behavior; existing agent ownership and upgrade ADR boundaries remain unchanged. |
| 10 Quality scenarios | Yes | `10-quality-requirements.md`: refine Q-78 from V1-only payload wording to real supported-loader/command output evidence; add regeneration compatibility across distinct actions with edge cases linked to delta titles. Do not claim session accuracy. |
| 11 Risks/debt | Yes | `11-risks-and-technical-debt.md`: refine R-50 payload-drift mitigation to real-loader matrix and append retained cross-session/late-ID, configured-default and variant-retention risks from above; distinguish accepted behavior from future fixes. Assign free IDs only after reading current register. |
| 12 Glossary | Yes | `12-glossary.md`: apply narrow attribution-context/model-identity/reasoning definitions below; no new Aggregate or events. |

Update `docs/arc42/README.md` index/context note with a link to this change's
design and the compatible OpenCode adapter scope. Preserve unrelated chapters
and existing ADR indexes. The support-fragment inventory is already complete
in `givn/specs/fragments/fragment.md` (38 rows; 34 existing E2E mappings plus
four new regeneration actions). Do not duplicate it into including use cases.

Exact glossary replacement text:

```markdown
| **Attribution context** | The process-scoped values supplied while one commit is created: LLM, reasoning mode, harness, and Givn version. The generic stamper never fabricates missing values. The OpenCode adapter retains best-effort session, last-observed-model, and (in V2) configured-default fallbacks; inferred fallback values are not verified current-session facts. Without a usable supplied or fallback value, the field remains unavailable. |
| **Model identity** | The provider and model pair supplied for commit attribution. In the OpenCode adapter it may be the remembered session model, the last observed model, or (in V2) the configured-default fallback; a fallback does not verify which model issued the command. Anti-terms: model name, model label, verified session identity for inferred values. |
| **Reasoning mode** | The effort or thinking variant supplied for commit attribution. The OpenCode adapter retains a remembered variant when later metadata omits it; that retained or default variant is best effort, not proof of the current command's reasoning. |
```

Exact cross-cutting clarification to insert adjacent to the environment
contract, not into the generic stamping algorithm:

> OpenCode attribution is an adapter-specific best-effort exception to treating
> every supplied value as a verified current-session fact. The adapter retains
> the supplied prototype's session memory, global last-observed model, and V2
> configured-default fallback. V2 can inject the shell's current session ID
> after the attribution hook, so the last-observed fallback may label a command
> from another session. The generic stamper only consumes supplied values and
> marks missing values unavailable; it does not infer a model itself. This
> compatibility change preserves the adapter behavior at the user's explicit
> direction rather than correcting its accuracy limitations.

### ADR classification supplied to the documentation writer

Active/archived registers and source files were searched for OpenCode, plugin,
attribution and agent integration on 2026-10-04. The existing generated-agent
ownership decision (ADR-013) and preservation-oriented upgrade decision
(ADR-023) remain applicable and unchanged. No independent attribution ADR was
found. One canonical target owns the local dual-entrypoint rationale:

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/fix-opencode-v2-attribution/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": ["User chose recent V1 plus V2 over older V1 support", "User rejected attribution-logic changes"],
    "architectural_impact": ["Documented adapter compatibility with the supported external plugin protocols"],
    "durable_consequence": ["One embedded adapter asset changes; entrypoint update can be reversed in one change without data migration"],
    "lower_level_artifact": ["Compatibility behavior belongs in the two existing features; complete local plumbing rationale belongs in design.md"],
    "existing_adr_check": ["Read active and archived registers and ADR-013; searched all ADRs for OpenCode/plugin/attribution; existing ownership and upgrade boundaries are not changed"]
  }
}
```

Supporting MADR checklist treatment: hard to reverse **unchecked** (local
asset/test change); lasting quality-shaping constraint **unchecked** (no new
system-wide floor beyond existing adapter compatibility); structural
**checked** as external-protocol compatibility evidence, not a moved ownership
boundary; must be shared **checked** as maintainer support context only.
Single-change reversibility falsifies ADR candidacy. Do not create a parallel
ADR, amendment, archive copy, or record merely because chapters need updating.

The retained-best-effort-fallback candidate has a separate verdict:
`NOT_QUALIFIED`; alternatives `PASS`, architectural impact `FAIL`, durable
consequence `FAIL`, lower-level artifact `FAIL`, existing-ADR check `PASS`, and
must-be-shared `SUPPORTING`. Route: `CANONICAL_ARTIFACT`; its sole destination
and cheaper home is
`givn/changes/fix-opencode-v2-attribution/specs/fragments/attribute-commits.feature`.
The preserved-versus-current-session-only choice is observable behavior, not a
new component, authority, deployment or persisted-data boundary. Single-change
reversibility, no multi-change constraint, and product-behavior exclusion all
falsify a new ADR. Chapter 11 and the glossary record limits, not parallel
decision rationale; generated-agent ownership (ADR-013) and preservation-oriented
upgrade (ADR-023) remain unchanged. No new/amended/superseding MADR is required.

## Planning validation and handoff boundary

- Wrote only proposal, delta specs, this design and the permitted durable
  support-fragment inventory. No implementation, bindings, commits, regeneration,
  status commands, reviews, task authoring or arc42 marker writing.
- Routing the real proposal exited 0 with signal recommending
  `apply-a-managed-upgrade`, score 17.00; the proposal records both mandatory
  EXTEND decisions and explains the consumer-versus-fragment deviation.
- `givn lint --change fix-opencode-v2-attribution` exited 2: all 18 scenarios
  parse; 18 WIP findings, plus advisory masked overlaps. JSON inspection found
  13 shape matches and 267 subsets, no syntax errors or contradictions. Many
  subsets share only generic successful CLI/output assertions; same-shaped
  regeneration outcomes correspond to genuinely different action boundaries.
  Formal overlap dispositions belong to later review, not this session.
- An initial explicit lint invocation passed directories and exited 1 because
  this binary's explicit-path mode expects files. The corrected shell invocation
  `givn lint givn/specs/fragments/*.feature givn/changes/fix-opencode-v2-attribution/specs/fragments/*.feature --json`
  exited 2 with exactly the 18 WIP findings and no parse errors. It is the
  file-scoped parsing check, not a claim that all repository lint is clean.
- Source/help/registry and Darwin network proofs above were actually checked.
  Delta Cucumber runtime compatibility, strictness probes, runtime provisioning and
  terminal evidence are mandatory; distinguish setup-scope permanent runs from final `--all` execution after deltas are green. They require implementation
  of the test infrastructure before any scenario can be marked complete.

Stop after this technical design. The orchestrator's documentation writer owns
the durable arc42 step and required marker before a fresh-context design-review
handoff. No next-stage command sequence is hardcoded here; the effective
manifest remains the workflow authority.
