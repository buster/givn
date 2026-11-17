# Design Review: enable-ise-and-run-the-givn-self-rebuild

Review kind: independent
Author: enable-ise-and-run-the-givn-self-rebuild author (omp)
Reviewer context: paseo design-review context, 2026-11-17 (deepseek-flash, message 8d6e24ce); this one context also owns the final review

Reviewed: `HEAD` `ad1b8441a88ea1fe7c66cf35b918dafa169168fa`; the change directory is untracked. Working-tree artifacts reviewed: `proposal.md` sha256 `e100a165f54a2309388b552c9f6412623d62d9173ef2bae35f2ee8ea202221e8`; `design.md` sha256 `072e5c27ab606711f74008b5e41fd3329b5b2604e4833482455e45be4717801b` (re-signed after the `## Step Definitions` edit); `arc42.md` sha256 `82f04aa085e04b6a5575864b3e360480392ed2ef74a49e6e0407ea792c97c865`. Uncommitted chapter edits: `docs/arc42/02-architecture-constraints.md` sha256 `5056f63a93a2c44cca40d1826ab8ac061ca3566dca63729feeed8933d9ed9fb0`, `docs/arc42/04-solution-strategy.md` sha256 `c415c17f43d49ade60ea85721c44ce051bd3845b4e2d084faedc4ee5ce521f34`, `docs/arc42/11-risks-and-technical-debt.md` sha256 `178f8339738f0c8340577263509d8ffc51d487618f1c970b0ae9f42ed9a09f41`. `docs/glossary.md` and `docs/domain-model.md` are unmodified.

## Verdict

No blocking finding remains. The first round returned FAIL with B1 to B4; all four are closed in the revised text and nine advisories are applied. Four non-blocking advisories remain below. The change can proceed to `tasks.md`.

## Re-sign: Step Definitions integrity edit

The tasks author found that `givn check review` refused `design.md`'s `## Step Definitions` prose. The operator prepended the standalone token `none.` to the section's first paragraph. The current `design.md` sha256 is `072e5c27ab606711f74008b5e41fd3329b5b2604e4833482455e45be4717801b`. Removing exactly `none. ` from `design.md:412` reproduces the previously reviewed hash `bd2b21ef5cf7c87ca304ac2e231f70e40a2f31b34fcd49ede864f205da82c4b7`, and the diff is that one line. The section still states that no step definition is added, moved, or copied, that `specs` is skipped, and that the permanent step files stay unchanged.

`givn check review --change enable-ise-and-run-the-givn-self-rebuild` no longer reports the `## Step Definitions` integrity finding. It fails on `tasks.md`'s absent checked `COMMIT:` lines (`hash presence`) and on the missing `review.md` verdict and persona evidence. Neither failure is a design finding; the Step Definitions check passed. `givn check tasks` still exits 0 with the boundary transcript.

The other reviewed artifacts did not move: `proposal.md` sha256 `e100a165f54a2309388b552c9f6412623d62d9173ef2bae35f2ee8ea202221e8`, `arc42.md` sha256 `82f04aa085e04b6a5575864b3e360480392ed2ef74a49e6e0407ea792c97c865`, and the three chapter hashes `5056f63a93a2c44cca40d1826ab8ac061ca3566dca63729feeed8933d9ed9fb0`, `c415c17f43d49ade60ea85721c44ce051bd3845b4e2d084faedc4ee5ce521f34`, and `178f8339738f0c8340577263509d8ffc51d487618f1c970b0ae9f42ed9a09f41` are unchanged. `docs/glossary.md` and `docs/domain-model.md` remain unmodified. No new finding; the verdict remains PASS and the four advisories stand.

## First-round closure

### B1. The change's own tasks gate and the promised receipt

Closed. `proposal.md` now declares `Behavior change: none`, `givn/changes/enable-ise-and-run-the-givn-self-rebuild/.givn-skip` lists `specs`, and `givn check tasks --change enable-ise-and-run-the-givn-self-rebuild` exits 0 (transcript below). The design no longer claims a change verification receipt: `## Evidence Artifacts` states that no `verification/tasks-verification.json` exists, D6 states the not-applicable path returns before `write_change_receipt`, and Verification item 3 states the same. The boundary evidence is the two transcripts, `boundary-check.txt` before `review.md` exists and `boundary-review-check.txt` after. The source agrees: `src/cli/check.rs:452-470` returns on the not-applicable path, and `src/cli/check.rs:693-710` uses `preflight_e2e_boundary` for review. The design does not cite the `verify passed` line as suite evidence. The proposal's bullet now says the no-behavior-change path produces no receipt.

### B2. The non-Specification build inputs

Closed. `## Scope` names the build inputs, `## Clean-Room Builder Boundary` has the `### Non-Specification build inputs` subsection, and D7 resolves as a design decision. The five inputs are named with their consequences: `Cargo.toml`, `Cargo.lock`, `assets/**` (95 files; `du -sk` 764 KB; `rust_embed` in `src/config/embedded.rs` plus the five `include_str!` paths), the Rust toolchain, and the cargo registry cache. The design states that the trial does not test whether these inputs are derivable from the Specification and requires `builder-notes.md` to list every authored file and crate version and `spec-gaps.md` to hold one `specification defect` row against `givn/specification.yaml`. `givn/specification.yaml` declares none of them; verified.

### B3. The Interaction Coverage Matrix

Closed. `## Interaction Coverage Matrix` has five rows, one per inventory entry: `clean-room-preparation`, `trial-input-verification`, `specification-declaration`, `change-strategy`, and `spec-gap`. Each row names the consumer action, the `@e2e` scenario, the real interface, the driving mechanism, and the `unchanged` disposition. The five inventory rows exist in the three owning `usecase.md` files; the five scenario titles exist under `givn/specs/**`, and each carries `@e2e` on the line before `Scenario:`. The one sentence that `create-change`'s other inventory rows are not touched is present.

### B4. The review context

Closed. `proposal.md` records `Review kind: independent` with the operator reason and the reviewer context string. This artifact records the same reviewer context at the top. `src/review_context.rs:143-230` requires the proposal and review kinds to agree, and for an independent review it requires the design-review and final review to share one reviewer context; the recorded string satisfies both. This context did not write the design.

## Advisories from the first round

| # | Disposition |
|---|---|
| A1 | Applied. `## Scope` names `S3`, `S4`, `S5`, and `S12` from the phase table (`plan.md:165`), notes that section 0 prose names only `S12`, and states that `S3` to `S5` are recorded as delivered (`plan.md:33-38`), leaving `S12` outstanding. |
| A2 | Applied. `## Trial Inputs` now says `S14` compares the recorded `e2e-inventory.md` total at the trial revision, not the stale 108 (`plan.md:165,981`) or 119 (`plan.md:1140`), and that a correction which retires a scenario changes the total. |
| A3 | Applied. The `GIVN_READY_FIXTURE` row now says `run-tests.sh` exits 2 when it cannot resolve the built fixture, a step that needs the fixture fails by name, and the runner exits 1. |
| A4 | Applied. The registry check scans `registry/src`, `registry/cache`, and `registry/git`; the design states that the cache holds `.crate` archives and git checkouts and that a fresh `CARGO_HOME` is the fallback. |
| A5 | Applied. The dependency-tree command is `cargo tree --locked --offline --manifest-path e2e/Cargo.toml -p givn-e2e`. |
| A6 | Applied. `### Contracts and Components` has a fourth row for the served change guidance (`givn instructions design` and `givn instructions review`), consumer `create-change`, implementing component `crate::cli::instructions` over `assets/instructions/**`. |
| A7 | Applied. The `boundary-check.txt` row names the expected `task preflight: review.md is missing` warning and says "It is not a clean preflight." |
| A8 | Applied. `## Exact Commands` and D1 state the order: the pre-trial commit holds the boundary transcript, inventory, coverage export, dependency tree, and pre-trial task ticks; `trial-revision.txt` and the post-trial evidence are committed after the run; a restart commit holds the first attempt's evidence, the corrected test, and the regenerated inventory and coverage. |
| A9 | Applied. The sandbox uses `--ro-bind` for `registry` and `git`; the toolchain mounts are read-only as well. |

## Fresh design attack

### Offline build from the copy

The copy has no `Cargo.toml`, no `Cargo.lock`, and no `assets/**`; the builder authors them from the Specification and the disclosed inputs. I probed the exact mount set with a temporary crate that depends on `serde` and `serde_json`: `cargo build --release --offline` inside the sandbox compiled every dependency from the read-only registry and produced a binary that ran. The product rebuild was not run. The probe shows the mount set admits the offline build; it does not prove the builder's invented `assets/**` will satisfy the frozen corpus, which the design already records as the trial's central risk and as the disclosed `specification defect` row.

### Sandbox mount set

The only repository path in the sandbox is `<copy>` at `/work`. The mount set admits `/usr`, `/bin`, `/lib`, `/lib64`, and `/etc` read-only, the three rustup shims and `~/.rustup`, the registry and git caches read-only, `/dev`, `/proc`, a tmpfs `/home` and `/tmp`, and the copy read-write. The builder's own file tools run outside the sandbox; the design does not claim the mount enforces read isolation. It states that the builder's workspace is the copy and that `builder-notes.md` records every read. That is the residual process control, consistent with the evidence plan.

### Restart rule, manifest, and frozen e2e

A restart after an `E2E test defect` reads a new pre-trial commit. That commit records the corrected test and the regenerated inventory and coverage; the rebuild writes a new copy and manifest whose `revision`, `removed_paths`, `specification_files`, `end_to_end_files`, and checksums are recomputed at that revision; `trial-revision.txt` and the post-trial evidence are committed after the run. The manifest invariant table compares every recorded value against `git ls-tree` and the declaration. The restart rule is coherent with the manifest and with the frozen package: the corrected test is part of the new frozen input, and the old attempt's evidence is preserved in the restart commit. The design's `## Exact Commands` states the same order.

### No unrun rebuild claim

The proposal and design describe deliverables and checks, not completed results. The evidence rows name the producer and the content; `spec-gaps.md`'s disclosed-input row is a requirement, not a recorded result. The trial step is explicitly gated on the `S12` evidence, which is outstanding. The change does not claim a rebuild ran.

### Data-flow checks

- `givn ise compare` after the build: `target/` and the authored `src/**` are not Specification paths, and `src/ise/compare.rs` refuses only added Specification paths and missing kept paths; source additions are allowed. The after-build comparison can report both subjects as match.
- The report's `End-to-end tests: not copied (100 files)` uses `report.end_to_end_files`, which is the package path count, not the 156-entry manifest union; the design's sample is correct.
- The manifest's `end_to_end_files` is the union of the 100 package files and the 56 selected `@e2e` feature paths; `scenario_digests` keys by feature path, and `end_to_end_rollup` lets the scenario digest win for a feature path. The design's 156 and the rollup sentence are correct.

### Arc42, glossary, and structural model

Independent derivation against the 12 rows: chapters 02, 04, and 11 are affected; the other nine are not. The three chapter files exist, hold content, and match the design. No chapter contains box-drawing characters. Chapter 09 needs no new MADR; the qualification records no architectural commitment and names `docs/arc42/04` and `docs/arc42/11` as the cheaper homes. Chapter 11's `R-163` records the undeclared surfaces and the triage. `docs/glossary.md` holds `Specification`, `Spec copy`, `Spec gap`, `Ise Test`, `Dry-Run Ise`, and `Change strategy` with the meanings the design uses. `docs/domain-model.md` stays unmodified because no use case, capability, or relationship changes; the design records that assessment.

## Grilling branches

| Branch | Verdict |
|---|---|
| Scope | B1 and B2 are closed in the revised text; the change still matches the proposal: enablement recorded, boundary proved, one `Dry-Run Ise`, no promotion. No extra or missing deliverable. |
| Tech choices | The command surface matches `src/main.rs:264-305`; the report and manifest invariances match `src/cli/ise.rs` and `src/ise/manifest.rs`; the sandbox probe builds offline. B2's disclosure is present. |
| Missing scenarios | No finding. No product behavior is added; every observable is a report line, comparison subject, refusal, exit code, or gap outcome, and each has a named check. |
| Testability | No finding, not applicable to a delta: no production code and no step body. Strict mode is inherited (`tests/features_runner.rs` and `e2e/tests/e2e_runner.rs` call `.fail_on_skipped()`, and the E2E runner exits 1 on failed, skipped, or zero). No stub pattern or single-scenario run command is needed because no scenario is added. |
| E2E fidelity | No finding. No delta `@e2e` scenario; the permanent suite runs unchanged through `GIVN_BIN`; no action scope, interface, driver, or assertion rule is redefined. |
| Interaction Coverage | B3 is closed. The matrix has the five inventory rows, each with the real CLI interface, the driving mechanism, and the `unchanged` disposition. |
| Risk | B2 was the most likely failure: the copy cannot reproduce the embedded served corpus without invented inputs. The design discloses them, records the consequence, and requires the builder notes and the `specification defect` row. Residual: the builder's asset choices and the read-isolation process control, both recorded in the evidence. |
| Use-case and Persona context | No finding. The three use-case IDs and their capabilities match the owning documents; the three confirmed Personas appear in each owning `## Personas`; no biography acts in Gherkin; no new use-case document is created, so no scope rejection criterion is triggered. |
| Decision ledger | No finding. D1, D2, and D3 resolve in `## Decisions`; D5 records the change strategy and the `Ise Test` mode; D4, D6, and D7 are design decisions. The seed question is answered by the plan's phase table, not by a `D` id. |
| Contracts and components | No finding after A6. Four rows, one component each, consumer-shaped surfaces; the served change guidance row names `crate::cli::instructions`; no new contract or component. |
| Decision timing | No finding. No carried decision remains, no scenario depends on an open answer, and no speculative abstraction is built. D7 is a disclosure, not a carried question. |
| change strategy | No finding. Exactly one `Decides D5` entry records `whole-product rebuild` with `Dry-Run Ise`, the affected consumers, the persisted state (none), and the rebuild boundary. |
| Visual design contract | No finding. `Interface: terminal`, viewports not applicable, no new screen; the terminal surface has three confirmed Personas and the transcript baseline. |
| Deferrals | None recorded; none needed. |
| Persona disposition | No finding. Three rows, each naming the artifacts that carry its evidence. |
| Interface classification | No finding. The product is a CLI with no browser surface, server, database, or queue; the design states no screen is added, so the Web UI classification does not hold. |
| ADR qualification | No finding. Recorded once in `arc42.md`: no architectural commitment, no durable consequence, canonical homes `docs/arc42/04` and `docs/arc42/11`, and ADR-010, ADR-076, and ADR-078 already own the mechanisms. No `NEW_ADR`. |
| arc42 | No finding after independent re-derivation (02, 04, 11 are affected; the other nine are not). Chapter files exist and hold real content; no box-drawing characters; chapter 09 needs no new MADR; chapter 11's `R-163` matches the design. |
| Ubiquitous language | No finding. `docs/glossary.md` holds `Specification`, `Spec copy`, `Spec gap`, `Ise Test`, `Dry-Run Ise`, and `Change strategy` with the meanings the design uses. |
| Structural model | No finding. No use case, capability, or relationship changes; leaving `docs/domain-model.md` unmodified is correct, and the design records that assessment. |
| Skipped specs | No finding. `specs` is the established `.givn-skip` form; the proposal's `Behavior change: none` and the skip agree; `givn check tasks` exits 0. |
| Prerequisite gate | A1 is applied. The phase table gates `S13` on `S3`, `S4`, `S5`, and `S12`; section 0 records `S3` to `S5` delivered and `S10` to `S12` outstanding. The change can be designed and tasked before `S12`; the trial cannot start before it. |

## Gates run

`givn check tasks --change enable-ise-and-run-the-givn-self-rebuild` exits 0:

```text
Mandatory checks for 'tasks':
  ⚠ task preflight: review.md is missing
  changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)
  ✓ verify passed
```

The `review.md is missing` line is the expected pre-trial warning named in `boundary-check.txt`. The `verify passed` line is the not-applicable path's line; it is not `verify.command` evidence. `givn lint --change enable-ise-and-run-the-givn-self-rebuild` exits 0 with no feature files under the change and a clean result.

## Counts and inputs re-verified

| Claim | Verification |
|---|---|
| `ise` is effective | `givn/config.yaml` has `addons.ise: true`; `config-lock.yaml` has `ise: true`; the enablement is revision `a9b54140`. |
| The command surface matches the binary | `src/main.rs:264-305`; `src/cli/ise.rs` renders the eleven fixed lines plus `Manifest`. |
| Tracked `src/**` and `tests/**` files | 187 (`git ls-tree`). |
| Tracked `e2e` package files | 100 (`git ls-tree`). |
| Tracked `givn/specs/**` files | 85 (`git ls-tree`). |
| Specification files | 90 (85 behavior plus the three declared documents, the arc42 limits file, and the declaration). |
| `@e2e` scenarios | 115; 56 feature paths (`givn spec coverage --format json`). |
| `end_to_end_files` entries | 156 (100 package files plus 56 feature paths). |
| The `e2e` package reaches no product crate | `e2e/Cargo.toml` is its own workspace root with no product dependency; the runner refuses a product package in `e2e/Cargo.lock`. |
| The runner is strict | `.fail_on_skipped()` and exit 1 on failed, skipped, or zero; `GIVN_RESULT_FILE` records `total`, `passed`, `failed`, `skipped`. |
| `.givn-skip` `specs` is the established form | `src/state.rs`; `givn status` reports `specs` skipped. |

Not run: the product rebuild, the frozen suite, and the `bwrap` product build. The sandbox probe above used a temporary crate, not the product.

## Remaining advisories

1. **Post-archive test command.** Verification item 10 repeats the `AGENTS.md` sequence with `./run-tests.sh --e2e`, which `run-tests.sh:52-55` refuses with `--e2e names the tier, not the package`. The preceding `./run-tests.sh` already runs both scopes. Name `./run-tests.sh --e2e --package e2e` or rely on the first command, and correct the stale `AGENTS.md` line in the post-archive maintenance commit.
2. **The `verify passed` line in `boundary-check.txt`.** The artifact will contain the not-applicable path's `verify passed` line. The design correctly does not cite it as suite evidence; name the line in the evidence row and state that it is not `verify.command` evidence, so the transcript cannot be misread.
3. **Setup failures and the gap list.** `## Strict Mode` says a skipped or zero-scenario result is recorded in `spec-gaps.md`, while `## External Boundary Cases` classifies it as a setup error, not a `Spec gap`. Record it as a setup failure in the trial notes (or a separate setup section), so the gap list keeps one outcome per difference.
4. **Asset size measurement.** The `assets/**` figure "764 KB" is `du -sk` disk usage; the tracked blobs total 491,382 bytes. Name the measure (`du -sk`) or use the blob total.

DESIGN-REVIEW: PASS
