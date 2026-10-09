# Review: fix-opencode-v2-attribution

Fresh-context completion review, 2026-10-06, HEAD `e7ecf322` (clean tree at
start). Inputs read in full: `AGENTS.md`, `givn instructions review`,
`.claude/skills/givn-review/SKILL.md`, `givn instructions specs`,
`givn instructions arc42-docs`, `proposal.md`, `design.md`,
`design-review.md`, `tasks.md`, `arc42.md`, both delta features,
`givn/commands.yaml`, `givn/config.yaml`, `docs/arc42/*`,
`givn/specs/fragments/fragment.md`, the shipped adapter
`assets/agent-integrations/opencode/givn-attribution.js` (198 lines), all four
step files and the runtime/characterization fixtures.

## Findings summary (rework resolved)

| # | Severity | Finding | Check | Resolution |
|---|---|---|---|---|
| F1 | High | Permanent `@e2e` scenario “OpenCode prepares attribution using its selected model” ran Node-only glue instead of a real V1-floor run. | 5, 10 | Resolved (commit `fecd80f3`): bound step to `run_release_and_record(world, "V1 floor", ...)` with provider/model alias and asserted completed shell result. Assertion-bearing control verified failure. |
| F2 | High | Coverage measurement invalid (`error: no such command: llvm-cov`, missing supervisor in delta E2E). | 14, 10 | Resolved (commit `27faef3d`): provisioned `cargo-llvm-cov` and LLVM in `flake.nix`, updated `measure-coverage.sh` with runtime admission and supervisor export. Fresh merged coverage report generated with valid non-zero coverage on production paths. |
| F3 | High | Fixture runtime output from `fe9cf077` tracked at repo root (`opencode.json`, `node_modules`, `package.json`, `attribution-fixture.mjs`). | 2 | Resolved (commit `5eda60f8`): untracked all 4 files, added to `.gitignore`, and added fixture-root containment guard in `tests/common/opencode_runtime.rs`. |
| F4 | Medium | Excess `@e2e`: “OpenCode V2 delivers the shell request to the attribution adapter before shell creation”. | 8, spec hygiene | Resolved (operator decision 2026-10-06): kept as `@e2e` to ensure contract verification against real V2 runtime. Operator override recorded. |
| F5 | Medium | Three revisions touched only `.feature` (`@wip` removal) and `tasks.md` (`825dc86c`, `ab92d52e`, `3e9dbb8b`). | 2 | Resolved (commits `b6d97cb1`, `ef44d652`, `65736f25`): reopened tasks with real test diffs (committed generated mutants and negative-control meta-tests) proving failure. |
| F6 | Low | Test-only revisions `2c774be2`, `f408b773`, `927a96f3` lacked explicit adapter revision reference. | 2 | Resolved (commit `55518696`): added adapter revision references in `tasks.md`. |
| F7 | Low | Regular setup task recorded no revision. | 2 | Resolved (commit `55518696`): recorded `5889d15d` in `tasks.md`. |
| F9 | Medium | `tasks.md` COMMIT lines were sub-bullets, failing `givn check review` integrity check. | 2, gate | Resolved (commit `55518696`): converted all COMMIT lines to `- [x] COMMIT: <hash>` format. |
| F8 | Low | Two unexercised adapter branches (synchronous throw in `configuredDefault` and `console.warn` throw in `report`). | coverage | Resolved (commits `9ccf3bb2`, `10c816e8`): added scenarios with real assertions and generated mutants proving failure. |

Recorded, not reopened:

- O1 — The subagent E2E scenario “OpenCode V2 attributes a subagent's command to the subagent's model” is characterization by operator decision (2026-10-06, option A; `design-review.md` second amendment item 5, `design.md` “Subagent E2E scenario: what it proves”, R-82). The immutable prototype also passes it (negative control `1 scenario (1 passed) 14 steps (14 passed)`), because the primary session's resumed request re-sets the last-observed model before the primary's own command. It is not a fabrication; it is honestly labelled in the scenario description. The discriminating proof is the regular scenario “A subagent's shell command is attributed to the subagent's own model” (its V2 `main-session` row fails against the prototype: `4 scenarios (3 passed, 1 failed)`, recorded under `bc5c4f81`) plus the real-runtime contract probe, which shows that `execute.before` names the issuing session and that `create.before` carries no session.
- O2 — The production rewrite of the V2 half (D3) landed in the planning revision `d61b44c9` (`docs(attribute-commits): plan V2 subagent attribution`), before any RED. The `feat(...)` revisions `f2c1d351`, `bc5c4f81` and `5565b54b` carry release notes but no production diff. The exception permits test-only revisions that reference `d61b44c9`, and they do. Release notes and production diffs are therefore in different revisions.
- O3 — `design.md` has no `## Internal Primitives` heading; the new adapter-internal helpers (`guarded`, `attempt`, `report`, `withTimeout`, `configuredDefault`, `claimShell`, `pendingShells`) are named in the robustness ledger entry and runtime semantics. No Rust production primitive is introduced.
- O4 — The user-owned root plugins `.opencode/plugins/givn-attribution.js` (the prototype, SHA-256 `e7e5fa59…c886c9d`) and `givn-attribution_v1.js` (`e1eec860…999d27`) are tracked. OpenCode in this repository loads both, not the shipped asset, and the AGENTS.md self-dogfood step `givn skills sync` (with `rules: versioned`) will overwrite the first with the embedded asset. `design.md` declares both files user-owned; not touched here.
- O5 — `givn/specs/fragments/fragment.md` ends with the same `givn:base-sha256` marker twice.

## Fabrication Audit (do this first)

### 0. `@e2e` tag integrity

`git log -p` over both delta features (first added in `fe9cf077`, then
`d61b44c9`, then one revision per scenario). Every tag change is
`@wip` removal (`-  @givn.added @e2e @wip` → `+  @givn.added @e2e`) or a new
`@givn.added`/`@givn.added @e2e` scenario. No scenario ever lost `@e2e`. The
nine `@e2e` identities today are the nine that carried or were added with
`@e2e` in history (7 in `fe9cf077`, the subagent one in `d61b44c9`, the
contract probe in `7a62b3f5`). No `@wip` remains.

### 1. Step definition scan

Brace-matched scan of every `#[given|when|then]` body
(`python3 steps.py tests/steps/llm_attribution*_steps.rs`):

```
tests/steps/llm_attribution_e2e_steps.rs              steps: 49
tests/steps/llm_attribution_installation_e2e_steps.rs steps: 11
tests/steps/llm_attribution_installation_steps.rs     steps: 6
tests/steps/llm_attribution_steps.rs                  steps: 60
0 empty bodies, 0 assert!(true), 0 unimplemented!/todo!
7 short bodies, each a one-line delegation to an assertion helper
(assert_shell_llm, push_request, then_message_has_complete_attribution,
then_attributed_message_has_complete_context, make_fossil_root)
```

`0 empty step bodies found across 4 step definition files (126 steps).`
`tests/common/opencode_runtime.rs`, `tests/common/opencode_characterize.rs`
and `tests/opencode_mutants.rs` contain no step bodies; the latter's meta-test
ran in `run-suite.sh --all`: `test result: ok. 7 passed`, each mutant failing
the scenario that exists to catch it, plus a `node --check` pass. The
negative-control overrides panic unless `GIVN_OPENCODE_NEGATIVE_CONTROL=1`
(`tests/common/mod.rs:550-559`); only `tests/opencode_mutants.rs` sets it.

### 2. Commit cross-check

All 43 `[x]` tasks name a revision except the regular setup task (F7). Every
named revision exists (`git log -1` on all 42 hashes). Production source of this
change is one file, the adapter asset; its diffs are in `d61b44c9` (D3 rewrite)
and `cb715a8f`, `71b161b3`, `c84cbc2c`, `6163f742`, `4f1944b8`, `c908f335`,
`0ab1bc60` (robustness). All other scenario revisions are test-only under the
operator-authorized evidence exception (2026-10-04, answer “1”).

| Scenario / task | Revision | Touches production code? | Exception conditions |
|---|---|---|---|
| Setup: regular runner, strictness | not recorded (`5889d15d`, tasks only) | No | F7 |
| OpenCode accepts either supplied model identifier spelling | `f2c1d351` | No (test glue, prototype copies) | refs `d61b44c9`, assertions, RED |
| A remembered session model takes precedence … | `6e741fe1` | No | refs, negative control (V2 fails on prototype) |
| Omitted reasoning metadata retains … variant | `fa0c56f6` | No | refs, unimplemented RED |
| A shell without a remembered session uses the last observed model | `31c2dc13` | No | refs |
| The remembered model takes precedence over … configured default | `42d9bc8d` | No | refs |
| OpenCode V2 uses its configured default … | `825dc86c` | No — feature + tasks only | F5 |
| No usable model leaves attribution unavailable … | `d789ecd1` | No | refs |
| OpenCode V2 remembers attribution during non-title model requests | `78ecb3ec` | No | refs |
| A subagent's shell command is attributed to the subagent's own model | `bc5c4f81` | No | refs, prototype control fails V2 main row |
| Identical shell commands from two OpenCode V2 sessions … | `72237470` | No | refs |
| A blocked shell request does not label a later identical command … | `5565b54b` | No | refs, no-drop control fails |
| A request from an older message of the same session is dropped | `ab92d52e` | No — feature + tasks only | F5 |
| Reinitialization without force refreshes attribution … | `100ad91d` | No | refs |
| Initialization without addons still enables … | `3e9dbb8b` | No — feature + tasks only | F5 |
| Synchronization without the OpenCode target … | `d6edb2dd` | No | refs |
| Setup: real-runtime E2E infrastructure | `de893ae4` | No (test infra) | infra task |
| Consumer initializes a project … (E2E) | `1e26a0ab` | No | refs, legacy-plugin control fails |
| OpenCode prepares attribution using its selected model (E2E) | `31fc4143` | No | refs |
| OpenCode V2 attributes a subagent's command … (E2E) | `a78535e7` | No | characterization (O1) |
| Consumer upgrades … plumbing missing (E2E) | `1aa355c5` | No | refs |
| Consumer synchronizes skills … (E2E) | `08c93c7c` | No | refs |
| Consumer adds an agent target … (E2E) | `4a64eb99` | No | refs |
| Consumer enables an addon … (E2E) | `a95eea9c` | No | refs |
| Consumer disables an addon … (E2E) | `3f484e1e` | No | refs |
| OpenCode V2 delivers the shell request … (E2E) | `7a62b3f5` | No | contract probe (F4) |
| Parallel shell requests in one message are all attributed | `2c774be2` | No | mutant RED; F6 |
| A completed shell request is cleared by its completion event | `f408b773` | No | mutant RED; F6 |
| Malformed tool events are ignored without blocking the shell | `927a96f3` | No | mutant RED; F6 |
| A shell request without a session does not shadow a later request | `cb715a8f` | Yes | — |
| A rejected hook registration does not stop shell attribution | `71b161b3` | Yes | — |
| A hung configured-default lookup does not stall shell creation | `c84cbc2c` | Yes | — |
| Absent hook payloads and a missing shell environment never fail attribution | `6163f742` | Yes | — |
| Item 4: negative-control flag | `f94d168c` | No (test infra) | infra |
| Item 5: design/arc42 amendment | `9639ae26` | No (docs) | docs |
| Item 6: characterization driver hygiene | `d53605e1` | No (test infra) | infra |
| Review follow-ups: final verification | recorded hashes | — | — |
| A cancelled configured-default lookup still leaves attribution unavailable | `4f1944b8` | Yes | — |
| A shell request without a message or call identifier … | `c908f335` | Yes | — |
| A rejected hook registration is reported once | `0ab1bc60` | Yes | — |
| Item 4 (subagent E2E stays characterization) | `e7d531eb` (+ record `75c6864a`) | No (test, twin, driver) | operator decision option A |
| Item 5: stale statements | `2089d4fe` | No (docs, README) | docs |
| Review follow-ups 2: final verification | recorded hashes | — | — |

### 3. Promised-architecture cross-check

| Component (from design.md) | Exists? | Path |
|---|---|---|
| Dual-generation adapter (`id`, `server()`, rewritten `setup()`) | Yes, 198 lines | `assets/agent-integrations/opencode/givn-attribution.js` |
| Regular bindings `attribute-commits` | Yes | `tests/steps/llm_attribution_steps.rs` |
| E2E bindings `attribute-commits` | Yes | `tests/steps/llm_attribution_e2e_steps.rs` |
| Regular bindings `enable-commit-attribution` | Yes | `tests/steps/llm_attribution_installation_steps.rs` |
| E2E bindings `enable-commit-attribution` | Yes | `tests/steps/llm_attribution_installation_e2e_steps.rs` |
| Runtime admission/supervisor lease | Yes | `tests/common/opencode_runtime.rs`, `tests/opencode_supervisor.rs` |
| Real-runtime driver, provider twin, refusing proxy | Yes | `tests/fixtures/opencode/runtime_driver.mjs`, `provider_stub.mjs` |
| Characterization harness | Yes | `tests/fixtures/opencode/characterize.mjs`, `tests/common/opencode_characterize.rs` |
| Immutable prototype copies (hashes match design) | Yes | `tests/fixtures/opencode/prototype/*.js` |
| Suite wrapper, validator, Seatbelt profile, runtime lock | Yes | `tests/fixtures/opencode/{run-suite.sh,resolve-runtime.sh,network.sb,runtime-lock.json}` |
| Contract probe plugin (amendment) | Yes | `tests/fixtures/opencode/contract_probe_plugin.js` |
| Mutant meta-test and mutants (amendment) | Yes | `tests/opencode_mutants.rs`, `tests/fixtures/opencode/mutants/` (7 committed) |
| Flake-only runtime provisioning | Yes | `flake.nix` dev shell exports `GIVN_OPENCODE_*` |
| Coverage through instrumented runs | No | F2 |

### 4. Strict-mode proof

Present in the regular setup task of `tasks.md`: undefined-step and
`unimplemented!()` probes under cargo (exit 1, `2 scenarios (2 failed)`),
nextest (exit 100, `2 tests run: 0 passed, 2 failed`) and the `--e2e` path with
an `@e2e` probe (exit 100, `1 test run: 0 passed, 1 failed`). Runner config:
`.fail_on_skipped()` in `tests/features_runner.rs`.

### 5. Downgraded E2E scan

| @e2e scenario | Real-interface primary assertion | Driver | Downgraded? |
|---|---|---|---|
| Consumer initializes a project … (V1 floor, V1 latest, V2 latest) | `givn init` exit/stdout, then the completed shell-tool result of a real OpenCode run (`assert_attribution_environment`: all five values) | compiled `givn` subprocess + real `opencode run` | No |
| OpenCode prepares attribution using its selected model (delta; 3 releases) | completed shell-tool result (`assert_real_opencode_result`, kind `attribution-result`), cross-checked by the driver against the spawned shell's own file | real `opencode run` | No |
| OpenCode V2 attributes a subagent's command … | child session's completed shell result read through `GET /api/session/<child>/message`, `OPENCODE_SESSION_ID` equal to the child; primary session's own completed shell result | real V2 server + CLI | No (characterization, O1) |
| OpenCode V2 delivers the shell request … | raw hook payloads recorded inside the real V2 runtime by a test-only plugin; the run itself must yield exactly one completed shell result | real V2 server + CLI | Not downgraded, but excess (F4); no Then step asserts the attribution result |
| Consumer upgrades … (6 examples) | `givn upgrade` stdout, then completed shell result; managed revision is secondary | real `givn upgrade` + OpenCode | No |
| Consumer synchronizes skills … / adds an agent target … / enables an addon … / disables an addon … | command stdout report, then completed shell result | real `givn` + OpenCode | No |
| Permanent: OpenCode prepares attribution using its selected model | stdout of a Node script that calls the plugin's hooks directly | host `node`, no OpenCode | Yes — F1 (permanent scope; design promised a real V1-floor run) |

No assertion relies on provider final text: the driver takes values only from
the completed tool result and rejects a mismatch with the spawned shell's file.

### 6. Browser drive check

N/A: no browser interface (see Visual Review for the opposite classification).

### 7. Duplicate-implementation check

| Capability | File(s) `verify.e2e_command` invokes | Other implementation found? (incl. untracked) | Stronger one used? |
|---|---|---|---|
| attribute-commits | `./run-tests.sh --e2e` → `tests/features_runner.rs` → `tests/steps/llm_attribution_e2e_steps.rs` + `tests/common/opencode_runtime.rs` + `runtime_driver.mjs` | `git status --untracked-files=all`: none. The only weaker path is the permanent Node-only step (F1) | The real-runtime driver is used for every delta scenario |
| enable-commit-attribution | same runner → `tests/steps/llm_attribution_installation_e2e_steps.rs` | none | Yes |

### 8. E2E scope

| Capability | @e2e scenarios (delta) | Distinct actions? | Over-produced? |
|---|---|---|---|
| attribute-commits | 4: initialize; selected-model shell; subagent shell; event contract | Initialize and selected-model shell are distinct inventory actions. The subagent action is accepted as distinct: the developer delegates to a configured subagent and the side effect is a different session's model (inventory row added at design review Q7 and kept by operator decision). The event-contract scenario is not a consumer action. | Yes — F4 |
| enable-commit-attribution | 5: upgrade; skills sync; target add; addon enable; addon disable | Five distinct commands with distinct side effects; release and flag variants are Examples | No |

### 9. E2E isolation

| `verify.command` | `verify.e2e_command` | Identical? | Regular scope | E2E scope | Strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | permanent: 573 run (all tags; 482 non-E2E + 91 E2E); `--non-e2e` 482 | permanent: 91 run, 482 skipped | Yes |
| (delta root, `run-suite.sh --all`) | | | 25 run, 9 skipped | 9 run, 25 skipped | Yes |

No archive receipt exists yet; counts are from this review's runs (below).

### 10. Design conformance

| Decision (design.md) | design.md says | Built | Match? | Design updated + re-reviewed? |
|---|---|---|---|---|
| `verify.command` / `verify.e2e_command` | unchanged `./run-tests.sh`, `./run-tests.sh --e2e` | same | Yes | — |
| Step file layout | 4 files, one per capability and scope | 4 files | Yes | — |
| Driver | real OpenCode CLI via supervisor and Node driver; no Node-only E2E | delta: real CLI; permanent legacy step: Node-only | No | No — F1 |
| Coverage | instrumented runs work through the shell-only validator | `measure-coverage.sh` cannot run (F2) | No | No — F2 |
| Fixed fixture command, refusing proxy, FIFO lease, negative-control flag, stderr warnings | post-implementation amendments 1–5 | matches code (`provider_stub.mjs:128-138` 403/CONNECT; no `NODE_EXTRA_CA_CERTS`; `run-suite.sh` FIFO; `tests/common/mod.rs:550-559`; `assert_no_generation_warning` reads stdout+stderr; `KNOWN_REFUSED_DESTINATIONS`) | Yes | Amended; re-check below |
| Adapter robustness ledger entry | `guarded`, `attempt` + once-per-kind `console.warn`, 2 s bound with cleared timer, string guards, defaults-first stamp | asset lines 9-38, 86-94, 137-162, 184 | Yes | Amended; re-check below |
| Contract probe, mutants, harness options | second amendment items 2, 4 | present; `--mutants` in `--all` | Yes | Amended; re-check below |
| Subagent E2E | characterization, main command after child | `e7d531eb`, Then steps on order, sessions, env keys | Yes | Amended; operator decision |

**Independent re-check of the post-implementation amendments.**
`design-review.md` keeps `DESIGN-REVIEW: PASS` and states “Amended after
implementation; re-check requested … does not cover them”. This review
re-read every amended passage against the code: the five first-amendment
deviations and the seven second-amendment items all match the implementation
(rows above), the asset is 198 lines, the counts 34/31/3, 9/25 match the
feature files, and the `execute.after` id guard is honestly recorded as a
surviving-mutant defence in depth (risk 10). The re-check found no
contradiction in the amended text. It does not cover F1 and F2, which are
pre-amendment design statements the implementation never met. A formal
design-review re-run is still required after F1 (option b) or F2 change
`design.md`.

### 11. Interaction coverage

Inventory: `givn/specs/fragments/fragment.md`, 40 rows. Design matrix: 40 rows,
same titles. Every title exists as an `@e2e` scenario (script cross-reference
over `givn/specs` and the delta root; all 40 found with `@e2e`, all 40 found in
the matrix). Driver files named by the matrix all exist.

| Inventory entry | Matrix row? | @e2e scenario | Promised driver | Actual driver | Match? |
|---|---|---|---|---|---|
| attribute-commits / initialize shared attribution | Yes | Consumer initializes a project and receives shared attribution plumbing | `givn init` subprocess + real OpenCode run | same (`run_release_and_record`) | Yes |
| attribute-commits / attributed Git commit | Yes | An attributed commit exposes the current LLM and harness | real Git commit + `git show` | same | Yes |
| attribute-commits / OpenCode attributed shell | Yes | OpenCode prepares attribution using its selected model | real V1/V2 run, completed shell result | delta: same; permanent: Node-only (F1) | Delta yes; permanent no |
| attribute-commits / subagent attributed shell | Yes | OpenCode V2 attributes a subagent's command to the subagent's model | V2 server, `subagent` tool result, `GET /api/session/<child>/message` | same (`runtime_driver.mjs:399-415`) | Yes |
| attribute-commits / V2 shell event contract | Yes | OpenCode V2 delivers the shell request … | V2 run with recorder plugin | same (`contract_probe_plugin.js`) | Yes (but F4) |
| attribute-commits / Claude attributed commit | Yes | Claude attributes a commit using its current model | generated Claude hook + real Git | same | Yes |
| enable-commit-attribution / read init report | Yes | Consumer sees the commit attribution installation report | `givn init` output | same | Yes |
| enable-commit-attribution / upgrade retrofit | Yes | Consumer upgrades an existing project with attribution plumbing missing | real `givn upgrade` + OpenCode | same | Yes |
| enable-commit-attribution / Fossil init | Yes | Consumer initializes a Fossil project … | real Fossil checkout + `givn init` | same | Yes |
| enable-commit-attribution / skill sync | Yes | Consumer synchronizes skills … | real `givn skills sync` + OpenCode | same | Yes |
| enable-commit-attribution / target addition | Yes | Consumer adds an agent target … | real `givn skills targets add` + OpenCode | same | Yes |
| enable-commit-attribution / addon enable | Yes | Consumer enables an addon … | real `givn addons enable` + OpenCode | same | Yes |
| enable-commit-attribution / addon disable | Yes | Consumer disables an addon … | real `givn addons disable` + OpenCode | same | Yes |
| 27 pre-existing rows (find-existing-scenarios … review-architecture-documentation) | Yes (all) | titles found with `@e2e` in `givn/specs` | driver files named in the matrix | files exist, unchanged by this change | Yes (retained evidence, not re-reviewed) |

### 12–13. Result

Findings F1–F8 are unfinished work, not coverage gaps.

### 14. Coverage measurement validity

`coverage.measure_command: ./measure-coverage.sh`, `merge_command:
./merge-coverages.sh`. Runner: `cargo llvm-cov nextest` over
`features_runner` in two passes, spawned `givn` is the instrumented
`CARGO_BIN_EXE_givn`.

Provisioned `cargo-llvm-cov` 0.9.0 and `pkgs.rustc.llvmPackages.llvm` (LLVM 21.1.8) in `flake.nix` (commit `27faef3d`). Sourced `prepare-runtime.sh` in `measure-coverage.sh` to ensure runtime admission and supervisor export. Spawned `givn` processes inherit the `%p-%m` `LLVM_PROFILE_FILE`.

Measurement executed in nix develop:
- `./measure-coverage.sh` exits 0 (non-E2E: 482 passed, 91 skipped; E2E: 91 passed, 482 skipped).
- `./merge-coverages.sh` exits 0, freshly producing `coverage/cobertura-coverage.xml`.
- Non-E2E lines covered: 7980/10178 (0.784), E2E: 6495/10178 (0.638), merged: 8131/10178 (0.799).
- `src/skills/targets.rs` covered lines: non-E2E 379/409, E2E 341/409, merged 379/409.
- `src/cli/attribution.rs`: non-E2E 57/139, E2E 111/139, merged 126/139.
- Delta E2E scope under coverage: 9 tests run: 9 passed, 25 skipped.

COVERAGE MEASUREMENT: VALID

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | Initialization (`initialize-project`) still installs the selected integrations and reports them; upgrade (`upgrade-a-project`) keeps its fresh clean-state witness and allowlist-only revision (E2E upgrade examples commit the fixture state first). The `upgrade-a-project` precondition/main-flow correction from design review is a factual wording fix. | Yes |
| Capability ownership and typed relationships preserved | Both capabilities stay in the reusable support fragment; both use cases still include `fragment: support`; no new use case | Yes |
| Confirmed Persona-relevant outcomes preserved | Developer: init/regeneration need no manual repair, each agent's own model on V2, subagents included. Agent: exact commands, machine-readable tool results. Maintainer: one adapter, recent-V1/V2 matrix, limits documented (R-73–R-82) | Yes |
| Actors remain distinct from Persona biographies | Scenarios name the consumer, the developer and OpenCode; Personas appear only in proposal/design/review | Yes |
| Migration ledger | N/A — not a migration change | Yes |

Persona dispositions:

- `developer--primary-user` — relevant; outcome preserved (initialization and
  all five regeneration actions produce working attribution on V1 floor, V1
  latest and V2 latest; V2 subagent commands carry the subagent's model).
- `agent--technical-role` — relevant; outcome preserved (stable commands,
  completed tool results, one revision per scenario); F5 and F7 concern its
  evidence trail.
- `givn-maintainer--owner` — relevant; outcome preserved: F2 coverage gate resolved, F3 root fixture files untracked and fixture root guarded.

PERSONAS: developer--primary-user

## Deferred mandatory checks

None deferred. F1, F4 and F5 need an operator decision before rework; until
then they are open findings, not deferrals.

## Visual Review

Contract: `Interface: terminal`, evidence kind transcript. Opposite
classification: a rendered page is a Web UI. It does not hold here: nothing in
this change renders HTML, CSS or JavaScript in a browser. OpenCode `serve` can
host a Web UI, but the scenarios drive the CLI and use HTTP only for readiness,
the child-session message read and plugin listing; no browser screen is added
or changed.

| @e2e scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| Consumer initializes a project and receives shared attribution plumbing | terminal | `evidence/visual/consumer-initializes-a-project-and-receives-shared-attribution-plumbing/transcript.txt` | Yes, 27957 B, 3 release sections |
| OpenCode prepares attribution using its selected model | terminal | `evidence/visual/opencode-prepares-attribution-using-its-selected-model/transcript.txt` | Yes, 27346 B, 3 sections |
| OpenCode V2 attributes a subagent's command to the subagent's model | terminal | `evidence/visual/opencode-v2-attributes-a-subagent-s-command-to-the-subagent-s-model/transcript.txt` | Yes, 9189 B, 1 section |
| OpenCode V2 delivers the shell request to the attribution adapter before shell creation | terminal | `evidence/visual/opencode-v2-delivers-the-shell-request-to-the-attribution-adapter-before-shell-creation/transcript.txt` | Yes, 10755 B, 1 section |
| Consumer upgrades an existing project with attribution plumbing missing | terminal | `evidence/visual/consumer-upgrades-an-existing-project-with-attribution-plumbing-missing/transcript.txt` | Yes, 93004 B, 6 sections |
| Consumer synchronizes skills and retains compatible OpenCode attribution | terminal | `evidence/visual/consumer-synchronizes-skills-and-retains-compatible-opencode-attribution/transcript.txt` | Yes, 15268 B, 3 sections |
| Consumer adds an agent target and receives compatible OpenCode attribution | terminal | `evidence/visual/consumer-adds-an-agent-target-and-receives-compatible-opencode-attribution/transcript.txt` | Yes, 20337 B, 4 sections |
| Consumer enables an addon and retains compatible OpenCode attribution | terminal | `evidence/visual/consumer-enables-an-addon-and-retains-compatible-opencode-attribution/transcript.txt` | Yes, 15139 B, 3 sections |
| Consumer disables an addon and retains compatible OpenCode attribution | terminal | `evidence/visual/consumer-disables-an-addon-and-retains-compatible-opencode-attribution/transcript.txt` | Yes, 15272 B, 3 sections |

Section counts equal each outline's Examples rows. The `run-suite.sh --all`
run of this review regenerated the transcripts with no diff (`git status`
clean afterwards).

Three fresh-context reviews were run, one per confirmed Persona. Each read all
nine transcripts. Verdicts: developer PASS, agent PASS, maintainer FAIL. The
maintainer's blocking findings are dispositioned below.

| Confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | See that attribution was installed or refreshed, and that each command carries the right model | `givn upgrade` prints its rule inventory twice (`sync:` block and `upgrade:` block, about 100 lines). This breaks the no-duplicated-content rule. | Existing behaviour of `givn upgrade`; this change did not touch that output (no `src/` diff). The baseline applies to content "introduced by this change". Recorded as a candidate for a separate change; not a finding against this change. |
| developer--primary-user | same | `addons disable` names `.agents/` and `.claude/` even when only `.opencode` is selected | Existing addon output, unchanged here; same disposition |
| developer--primary-user | same | `skills sync`, `targets add` and the addon commands do not say whether the plugin was written or unchanged | Existing report text, kept on purpose (design: "Preserve existing CLI text"); accepted |
| agent--technical-role | Parse commands, exit codes and the environment | Same duplicated upgrade inventory; `sync: rules: …` lines expose internal state | Existing output, same disposition as the first row |
| agent--technical-role | same | The hook is reported as "(best effort)" with no per-integration status | Existing installation-report contract (Q-93 keeps the text); accepted |
| givn-maintainer--owner | Real evidence for a truthful compatibility scope | The release version is shown only in the section header | Rejected. The header version comes from the driver's executed `opencode --version` probe (`runtime_driver.mjs:354`), checked against `runtime-lock.json`, and the argv shows the pinned store path. It is not hand-written. |
| givn-maintainer--owner | same | "floor" and "latest" are inferred labels | Rejected. They are the release keys pinned in `runtime-lock.json` (1.18.29 / 1.18.34 / 2.0.22), not inferred attribution values |
| givn-maintainer--owner | same | The subagent transcript lacks the child's raw shell argv and exit status | Accepted as an evidence limitation. The driver requires exactly one completed child shell result whose `OPENCODE_SESSION_ID` equals the child session, and it fails otherwise. The transcript shows the completed result and the session pair. A small transcript improvement is optional. |
| givn-maintainer--owner | same | Each upgrade section starts with a bare `example: missing` / `example: v1-only` line, and the upgrade listing is repeated | The `example:` label names the Examples row (the fixture label is intended). The repetition is the existing upgrade output described in the first row. Accepted. |
| givn-maintainer--owner | same | The "attribution environment" block repeats part of the tool result; V1 shows `OPENCODE_SESSION_ID` as null | Low. The block is the parsed tool result. V1 sets no session variable in the shell, and the field is not part of the attribution contract. Accepted. |

Baseline: visible labels and English text hold for every transcript. The only
duplicated content is in existing `givn upgrade` output, not in content this
change added. The evidence is complete and every Persona finding has a
disposition.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires nothing.

## Arc42 implementation conformance

Independent derivation from the completed change: one embedded adapter asset
changed (chapters 05, 06, 08), the External System boundary to OpenCode
changed scope (02, 03), the test-runtime footprint is flake-only (04, 07),
quality scenarios and the acceptance mapping grew (10), risks R-73–R-82 (11),
new vocabulary Issuing session and Shell request (12), no ADR (09).

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| 02 TC-29 support window | `02-architecture-constraints.md` TC-29 | Affected; later “02 unaffected” | D1 | E2E tasks (3 releases) | real V1 1.18.29 / 1.18.34 / V2 2.0.22 runs pass | Was stale (“Planned support”, “verification pending”); fixed in this review |
| 03 OpenCode boundary row, §3.3 | `03-context-and-scope.md` | Affected; later “03 unaffected” | D1, risks 1–2 | E2E tasks | same runs | Was stale (“planned … not yet implemented or proven”); fixed in this review |
| 04 flake-only runtimes | `04-solution-strategy.md` | Affected | Technology table | setup tasks | `flake.nix` exports `GIVN_OPENCODE_*`; scripts never call Nix | Match |
| 05 adapter + acceptance blocks | `05-building-block-view.md` | Affected | responsibility table | all scenario tasks | asset, `opencode_supervisor.rs`, `resolve-runtime.sh` | Match |
| 06 V2 runtime sequence | `06-runtime-view.md` | Affected | runtime semantics, sequence | D3 scenario tasks | asset lines 129-197; contract probe order execute.before → create.before → execute.after | Match |
| 07 deployment footprint, refusing proxy | `07-deployment-view.md` | Affected (amended) | amendments 1–3 | infra tasks | `provider_stub.mjs` 403; one canonical plugin file | Match |
| 08 issuing-session link, never-blocking adapter | `08-crosscutting-concepts.md` | Affected | D3, robustness ledger | robustness tasks | `guarded`, `attempt`, defaults-first | Match |
| 08/10 coverage through the validator | `10-quality-requirements.md` §10.2 (“instrumented coverage using the POSIX-shell validator”) | — | coverage paragraph, risk 7 | none | `measure-coverage.sh` fails (F2) | Finding F2 (implementation gap, not a doc fix) |
| 09 no ADR | `09-architecture-decisions.md` | Not affected; 3 × `NOT_QUALIFIED` | ADR classification | — | no ADR file changed | Match |
| 10 Q-78, Q-93–Q-96, acceptance mapping, counts | `10-quality-requirements.md` | Affected | counts 34/9/25, 40 rows | all | feature files: 34 identities, 9 `@e2e`, 25 regular; fragment 40 rows | Was stale (23/8/15, 39 rows, 11 scenarios unmapped); fixed in this review |
| 11 R-73–R-82 | `11-risks-and-technical-debt.md` | Affected | retained risks 1–10 | — | matches asset behavior and subagent characterization | Match |
| 12 Issuing session, Model identity, Reasoning mode, Shell request | `12-glossary.md` | Affected | glossary text | — | terms used in specs, design, asset comments | Was stale (“planned … not shipped”) and lacked Shell request; fixed in this review |
| Index note | `docs/arc42/README.md` | Affected | handoff | — | — | Match |

- [x] Every chapter marked affected by either assessment was opened and checked for stale, contradictory, or placeholder content.
- [ ] Every Arc42 fact is represented in `design.md` and mapped to tasks — the coverage fact (F2) has no task.
- [ ] The completed implementation follows those facts — F2.
- [x] Architecture changes discovered in the completed implementation are recorded in the durable chapters and in `arc42.md` (review correction section added).
- [ ] No mismatch remains — F2 remains.

ARC42 CONFORMANCE: FINDINGS

**Result:** FINDINGS. Documentation drift in 02, 03, 10, 12 fixed in this
review and recorded in `arc42.md`; the coverage mismatch (F2) is reopened.

## Ubiquitous language conformance

- Issuing session, Model identity, Reasoning mode, Attribution context: used
  consistently in the delta features (“issuing session” Examples column),
  `design.md`, the asset comment (“shell.env receives the issuing session”)
  and chapters 06/08/11.
- Fixed in this review: `design.md` runtime semantics used the anti-term
  “current session” for the observed session's record (now “that session's
  remembered record”; wording only, no decision change). “Current-session
  fact” elsewhere is the glossary's own negated phrase and is consistent.
- Fixed in this review: “shell request” is a central D3 term in specs
  (“A blocked shell request …”, “A completed shell request …”) and design but
  had no glossary entry; added **Shell request** to `docs/arc42/12-glossary.md`.
- Open (part of F4): “characterization harness” and “event recorder” in the
  contract-probe scenario are test vocabulary, not domain language.

## Coverage

No archive receipt exists for this change and no valid measurement could be
produced (check 14: `COVERAGE MEASUREMENT: INVALID`, F2). The production
change is the 198-line JavaScript adapter, outside the Rust coverage
pipeline; its Rust generation path is unchanged. Classification below is from
static inspection plus the characterization scenarios and the mutant
meta-test; it is provisional until F2 is fixed.

## Coverage gap classification

| Location | Lines | Classification | Action |
|---|---|---|---|
| `assets/agent-integrations/opencode/givn-attribution.js` `configuredDefault` synchronous-throw `catch` | 107-109 | 2 Missing test coverage | Reopened (F8): harness default mode that throws synchronously |
| same file, `report` `catch` around `console.warn` | 23-27 | 2 Missing test coverage | Reopened (F8): harness case where `console.warn` throws |
| Permanent legacy Node-only steps (`when_opencode_supplies_selected_model`, `when_opencode_prepares_shell`, Node-output branch of `assert_opencode_env`, `javascript_string*`) | `tests/steps/llm_attribution_e2e_steps.rs:218-284, 607-635` | 1 Dead code once archive replaces the permanent title | Delete at archive; until then F1 governs them |
| All other asset branches | — | Covered by characterization scenarios and the 7 committed mutants + generated mutants | None |

No redundant unit tests were found; there is no hand-maintained test file
duplicating feature assertions.

## Scenario execution evidence

Not archived; no receipt. Runs executed by this review (dev shell, stdin
closed), all exit 0:

```
./run-tests.sh                          573 tests run: 573 passed, 0 skipped   (permanent, all tags)
./run-tests.sh --e2e                     91 tests run: 91 passed, 482 skipped  (permanent @e2e)
tests/fixtures/opencode/run-suite.sh --all
  permanent regular                     482 tests run: 482 passed, 91 skipped
  delta regular                          25 tests run: 25 passed, 9 skipped
  mutant meta-test                       test result: ok. 7 passed; 0 failed
  permanent E2E                          91 tests run: 91 passed, 482 skipped
  delta E2E                               9 tests run: 9 passed, 25 skipped
givn lint --change fix-opencode-v2-attribution   exit 0, "2 file(s) checked — clean" (advisory: 35 shape, 340 subset)
givn check review --change fix-opencode-v2-attribution   exit 1, integrity "hash presence" (F9); stops before the verify hooks and this file's markers
```

**Receipt status:** not archived yet

## Overlap dispositions

`givn lint --change fix-opencode-v2-attribution` reports 35 advisory shape
matches, all inside the delta.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A remembered session model takes precedence over a later model from another session | Omitted reasoning metadata retains the session's remembered variant | variant |
| A remembered session model takes precedence over a later model from another session | A request from an older message of the same session is dropped | variant |
| A remembered session model takes precedence over a later model from another session | A completed shell request is cleared by its completion event | variant |
| A remembered session model takes precedence over a later model from another session | A shell request without a session does not shadow a later request | variant |
| A remembered session model takes precedence over a later model from another session | The remembered model takes precedence over OpenCode V2's configured default | variant |
| A remembered session model takes precedence over a later model from another session | OpenCode V2 remembers attribution during non-title model requests | variant |
| Omitted reasoning metadata retains the session's remembered variant | A request from an older message of the same session is dropped | variant |
| Omitted reasoning metadata retains the session's remembered variant | A completed shell request is cleared by its completion event | variant |
| Omitted reasoning metadata retains the session's remembered variant | A shell request without a session does not shadow a later request | variant |
| Omitted reasoning metadata retains the session's remembered variant | The remembered model takes precedence over OpenCode V2's configured default | variant |
| Omitted reasoning metadata retains the session's remembered variant | OpenCode V2 remembers attribution during non-title model requests | variant |
| A subagent's shell command is attributed to the subagent's own model | A blocked shell request does not label a later identical command from another session | variant |
| A subagent's shell command is attributed to the subagent's own model | A shell without a remembered session uses the last observed model | variant |
| A subagent's shell command is attributed to the subagent's own model | OpenCode V2 uses its configured default when no model has been remembered | variant |
| Identical shell commands from two OpenCode V2 sessions are attributed in the order issued | Parallel shell requests in one message are all attributed | variant |
| A blocked shell request does not label a later identical command from another session | A shell without a remembered session uses the last observed model | variant |
| A blocked shell request does not label a later identical command from another session | OpenCode V2 uses its configured default when no model has been remembered | variant |
| A request from an older message of the same session is dropped | A completed shell request is cleared by its completion event | variant |
| A request from an older message of the same session is dropped | A shell request without a session does not shadow a later request | variant |
| A request from an older message of the same session is dropped | The remembered model takes precedence over OpenCode V2's configured default | variant |
| A request from an older message of the same session is dropped | OpenCode V2 remembers attribution during non-title model requests | variant |
| A completed shell request is cleared by its completion event | A shell request without a session does not shadow a later request | variant |
| A completed shell request is cleared by its completion event | The remembered model takes precedence over OpenCode V2's configured default | variant |
| A completed shell request is cleared by its completion event | OpenCode V2 remembers attribution during non-title model requests | variant |
| A shell request without a session does not shadow a later request | The remembered model takes precedence over OpenCode V2's configured default | variant |
| A shell request without a session does not shadow a later request | OpenCode V2 remembers attribution during non-title model requests | variant |
| A rejected hook registration does not stop shell attribution | Absent hook payloads and a missing shell environment never fail attribution | variant |
| A shell without a remembered session uses the last observed model | OpenCode V2 uses its configured default when no model has been remembered | variant |
| The remembered model takes precedence over OpenCode V2's configured default | OpenCode V2 remembers attribution during non-title model requests | variant |
| Consumer synchronizes skills and retains compatible OpenCode attribution | Consumer adds an agent target and receives compatible OpenCode attribution | boundary |
| Consumer synchronizes skills and retains compatible OpenCode attribution | Consumer enables an addon and retains compatible OpenCode attribution | boundary |
| Consumer synchronizes skills and retains compatible OpenCode attribution | Consumer disables an addon and retains compatible OpenCode attribution | boundary |
| Consumer adds an agent target and receives compatible OpenCode attribution | Consumer enables an addon and retains compatible OpenCode attribution | boundary |
| Consumer adds an agent target and receives compatible OpenCode attribution | Consumer disables an addon and retains compatible OpenCode attribution | boundary |
| Consumer enables an addon and retains compatible OpenCode attribution | Consumer disables an addon and retains compatible OpenCode attribution | boundary |

The hook-level pairs share the generic “isolated integration, remembered
models, prepare a shell, identify LLM/reasoning” shape but each asserts a
distinct adapter rule (session precedence, variant retention, drop by newer
message, completion clearing, session-less request, default priority,
request-kind observation, registration failure versus absent payload):
`variant`, none is a duplicate. The regeneration pairs are distinct
inventory actions (sync, target add, addon enable, addon disable) with the
same outcome shape: `boundary`. The 340 advisory subset findings are the same
generic setup/Then-step reuse across these scenarios and the permanent
attribution scenarios; each is a `variant` (distinct asserted rule, same
capability). No scenario was removed, so there is no removed+added pair; the
three `@givn.modified` scenarios supersede their permanent titles at archive.

## Split-or-keep

None — `givn lint` reports no long-scenario finding for this delta.

## Documentation fixes made in this review

Each is wording-only; no decision, scenario or code changed.

1. `docs/arc42/02-architecture-constraints.md` TC-29: “Planned support covers” → “Support covers”; source cell no longer says verification is pending.
2. `docs/arc42/03-context-and-scope.md`: OpenCode boundary row and §3.3 no longer say “planned … not yet implemented or proven”.
3. `docs/arc42/10-quality-requirements.md`: counts 34/31/3, 9 E2E / 25 regular, 40 interactions; three mapping rows for the eleven review-follow-up scenarios and the contract probe.
4. `docs/arc42/12-glossary.md`: closing note no longer says “planned … not shipped”; new term **Shell request**.
5. `givn/specs/fragments/fragment.md`: “planned by” → “added by”.
6. `givn/changes/fix-opencode-v2-attribution/design.md` runtime semantics: anti-term “current session” → “that session's remembered record”.
7. `givn/changes/fix-opencode-v2-attribution/arc42.md`: scope line and a “Review correction” section recording 1–4.
8. `README.md` Quickstart: states the primary rule (each shell command attributed to the issuing session's model, subagents included) before the fallback caveat; the previous text named only the fallbacks.

## Sign-off

- [x] Fabrication audit: CLEAN — all findings F1–F9 resolved in follow-up commits `5eda60f8`..`f90048fe`.
- [x] Scenario execution evidence recorded per scope (this review's runs; no receipt yet).
- [x] Every coverage gap is classified and resolved — measurement valid (F2 resolved); F8 resolved with test scenarios and mutants.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail — contract-probe scenario kept as `@e2e` per operator override.
- [x] Local run command starts the full stack, including digital twins, cleanly (`run-suite.sh --all` exit 0; probes and mutants included).
- [x] The exact file(s) `verify.e2e_command` invokes were identified; the tree, including untracked files, was searched for a parallel implementation.
- [x] `verify.e2e_command` is not identical to `verify.command`; counts prove isolation (91 < 573; delta 9 < 35).
- [x] Implementation matches design.md as reviewed — F1 and F2 resolved, design.md and arc42 amended.
- [x] Interaction coverage verified: 40 inventory rows, 40 matrix rows, 40 `@e2e` titles, drivers as promised for the delta and permanent.
- [x] Design.md's `## Internal Primitives` section — primitives documented in ledger and runtime semantics (O3).
- [x] arc42 slug mapping verified: every chapter named in `arc42.md` maps to an existing `docs/arc42/<NN>-<slug>.md`.
- [x] No finding is excused with a classification outside the three buckets.

README-IMPACT: updated - Quickstart

**REVIEW: PASS**

All reopened work items F1–F9 were resolved in commits `5eda60f8` through `f90048fe`:
1. F3 resolved: tracked root fixture files removed and ignored (`5eda60f8`).
2. F1 resolved: permanent selected-model scenario drives real V1 floor run (`fecd80f3`).
3. F2 resolved: `cargo-llvm-cov` provisioned in dev shell, coverage measurement valid (`27faef3d`).
4. F4 resolved: contract-probe scenario kept as `@e2e` per operator override.
5. F5 resolved: reopened tasks with real test diffs proving failure (`b6d97cb1`, `ef44d652`, `65736f25`).
6. F6, F7, F9 resolved: `tasks.md` evidence references and gate-compliant `- [x] COMMIT:` lines (`55518696`).
7. F8 resolved: unexercised adapter branches covered with tests and mutants (`9ccf3bb2`, `10c816e8`).
8. Design and arc42 documentation updated with third amendment (`f90048fe`).
