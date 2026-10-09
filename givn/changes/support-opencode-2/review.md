# Review: support-opencode-2

## Fabrication audit

Mechanical checks, run before trusting any artifact.

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity | Clean. The delta carries `@givn.modified @e2e` (acceptance) and two `@givn.added` scenarios; no `@e2e` was removed and no `@wip` remains. |
| 1 | Empty/no-op step bodies | **0 empty step bodies found across 1 file** (`tests/steps/llm_attribution_e2e_steps.rs` scanned for `{}`, `() {}`, bare `return`, `unimplemented!`, `todo!`). |
| 2 | Checked tasks carry production-touching commits | Clean with a recorded reuse disposition: the production surface (ported asset, dogfood copy, rewritten V2 driver glue) lands in `beadf314d5585edc0bff04b22c87cf353e548f19` (scenario 1); `50b08405147a425741366f480a8ba4e7e6859932` and `8c78d19888ea0238dd93cae7e3b4d128eb26e862` verify that shared production code — pure reuse, with genuine REDs recorded (initial full-scope pre-change failure; V1-asset acceptance failure). |
| 3 | Promised components exist | `assets/agent-integrations/opencode/givn-attribution.js`, its byte-identical `.opencode/plugins/` copy, and the rewritten glue in `tests/steps/llm_attribution_e2e_steps.rs`. Every declared helper (`guarded`, `attempt`, `report`, `rememberModel`, `stamp`, `withTimeout`, `configuredDefault`, `claimShell`) is referenced. |
| 4 | Strict-mode proof | Present in `tasks.md`: the out-of-repo probe feature with an undefined step exits non-zero (`Step doesn't match any function`). |
| 5 | `@e2e` Then assertions | The acceptance scenario asserts the environment values the generated integration contributes to the shell environment (`GIVN_ATTRIBUTION_LLM`, `_REASONING`, `_HARNESS`, `GIT_COMMITTER_NAME`) read from the executed driver output — the change's real interface (the generated plugin module executed by Node), not a repository mirror. |
| 6 | Browser-UI interaction | N/A — no browser surface (Interface: none). |
| 7 | `verify.e2e_command` files and parallel implementations | `verify.e2e_command` = `./run-tests.sh --e2e`; the OpenCode `@e2e` steps exist only in `tests/steps/llm_attribution_e2e_steps.rs`; working state and tree searched, no parallel implementation. |
| 8 | E2E action scope | One normalized action — an OpenCode session supplies a model and prepares a shell command — carries exactly one `@e2e` scenario; the two added scenarios are regular (no-model and second-session variants). |
| 9 | `verify.e2e_command` scoped and distinct | `./run-tests.sh --e2e` vs `./run-tests.sh` are different strings; counts prove the strict subset: 90 vs 589 (repository) and 1 vs 3 (change scope). No archive receipt exists yet; the archive step measures the committed candidate. |
| 10 | Design conformance | Commands, file layout, and driver match `design.md`: glue stays in `tests/steps/llm_attribution_e2e_steps.rs`, the given step asserts the generated temporary-project file against the embedded asset (omp pattern), the single-scenario command uses the nextest unit form, the driver is the inline `.mjs` V2 replay, no new fixture or target. |
| 11 | Interaction coverage | The fragment feature declares no `# User Interaction Inventory:` block (pre-existing gap recorded in `design.md`, not extended here); the design's matrix row maps the OpenCode session/shell action to the acceptance scenario; the step file contains the promised inline Node driver. |
| 12 | Coverage measurement validity | No instrumented pass in this review (the review instruction assigns the committed-candidate measurement to the archive); no Rust production source changed; the production surface is the generated JS asset, exercised end-to-end by all three scenarios. |

### Interaction coverage cross-reference

| Inventory entry | Matrix row | `@e2e` scenario | Driving mechanism | Match |
|---|---|---|---|---|
| none declared in the fragment feature | `attribute-commits`: an OpenCode session supplies a model and prepares a shell (change-level mapping) | `OpenCode prepares attribution using its selected model` | Inline Node driver replaying the OpenCode 2 session, tool, and shell hooks against the generated integration | Yes |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | arc42.md claim | design.md | tasks.md | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| OpenCode 2 attribution integration constraint (TC-29) | `docs/arc42/02-architecture-constraints.md` | Yes — rewritten | session/tool/shell hooks, session-scoped model | arc42-docs task | the asset registers `session` lifecycle hooks, tool `execute.before`/`execute.after`, and `shell.create.before` | Yes |
| OpenCode 2 ordered runtime path (6.1a) | `docs/arc42/06-runtime-view.md` | Yes — rewritten | replay shapes documented | arc42-docs task | the driver replays `session:generate` → `tool:execute.before` → `shell:create.before` | Yes |
| Attribution precondition (Q-78) | `docs/arc42/10-quality-requirements.md` | Yes — reworded | scenario asserts the environment values | acceptance scenario task | acceptance scenario passes against the committed asset | Yes |
| Plugin-contract drift risk (R-50) | `docs/arc42/11-risks-and-technical-debt.md` | Yes — reworded | guarded hooks, non-blocking fallbacks | acceptance scenario task | guarded hooks; unavailable defaults written before the first await; no shell blocked | Yes |
| Issuing-session term (ch 12) | `docs/arc42/12-glossary.md` | Yes — new term | session-link decision (D2) | issuing-session scenario task | the driver prepares a shell for `session-1` after a later `session-2` model and the environment identifies `session-1`'s model | Yes |
| Documentation status | `docs/arc42/README.md` | Yes — paragraph | — | — | the paragraph matches chapters 2, 6.1a, 10, 11, 12 | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The change reuses `Attribution context`, `Model identity`, `Harness`, `Reasoning mode`, and `Commit attribution` unchanged, and adds `Issuing session` to `docs/arc42/12-glossary.md`. Specs, design, and code use the same terms; no unrecorded term is introduced.

## Coverage classification

No archive verification receipt exists yet for this change; the archive step measures the committed candidate. Classifying from inspection:

- **Production surface**: the generated OpenCode attribution integration (JavaScript) — exercised end-to-end by all three scenarios (with a selected model, with no model, and with a second session observed later), including the configured-default fallback path that ends in `unavailable`.
- **Dead code**: none — every helper in the asset is referenced.
- **Missing coverage**: none beyond the classification below.
- **Legitimately hard to test** (bucket 3): the defensive internals — last-observed-model fallback, configured-default timeout and cancellation, malformed tool-event and rejected-registration guards, and warn-once behaviour. Concrete justification: these are non-blocking compatibility paths with no stable observable trigger through the real interface; their automated coverage was deliberately cut by the operator's scope decision recorded in `design-review.md` (rejected alternative: "record the cut") and in the planning decision of 2026-10-09.
- **Redundant unit tests**: none added (the `.feature` files are the executable authority).

## Use-case and Persona conformance

Owning contract: the reusable `support` fragment (`givn/specs/fragments/fragment.md`). Its guarantees are preserved: attribution is best effort, missing metadata never blocks a commit, the attribution context stays process-scoped per shell command, and regeneration overwrites the generated integration. Persona `developer--primary-user` is dispositioned: the acceptance evidence is the committer identity `LLM: openai/gpt-5.6-luna via opencode` and the environment values a developer reads from an OpenCode 2 session. No Persona is invented and no user-visible surface exists beyond the generated file.

Deferrals: every mandatory check is satisfied; the only pending item is the archive-time coverage receipt, which is the review instruction's own sequence rather than a deferred operator decision.

## Visual review

Interface: none. The opposite classification was tested and does not hold: the change renders no screen, produces no terminal transcript, and changes no CLI text output; its only output is environment variables contributed to a shell command and a generated JavaScript module.

VISUAL-REVIEW: N/A (Interface: none).

## Overlap dispositions

The overlap gate reports no shape-match findings involving this change's
delta, so no disposition row applies. The three `[SUBST]` advisories reported
by `givn lint` for the delta — the issuing-session scenario as a subset of the
acceptance scenario in both its permanent and delta form, and as a subset of
the unavailable scenario — are variant relationships by design: the
issuing-session scenario asserts the contested precedence between two
observed session models, the acceptance scenario asserts the single-session
happy path including reasoning mode and committer identity, and the
unavailable scenario asserts the no-model stamp. None is redundant with
another.

## Split-or-keep

No scenario in the delta is classified as long by the overlap gate, so no
split-or-keep decision applies. Both multi-step scenarios stay whole by
design: `OpenCode prepares attribution using its selected model` is one
cohesive behaviour (the OpenCode 2 lifecycle end to end), and `OpenCode
attributes a command to the session that issued it` is one cohesive
behaviour (session precedence).

## README impact

README-IMPACT: none

Rationale: no consumer-observable command, flag, configuration, or output
changed, and the existing attribution paragraph stays accurate.

## Sign-off checklist

- [x] Fabrication audit clean (table above).
- [x] Every checked task has a production-touching commit or a recorded reuse disposition.
- [x] Every promised component exists.
- [x] Strict-mode proof present and non-zero.
- [x] Scenario execution evidence recorded: change scope 3/3, repository suite 589/589, repository E2E 90/90; the archive receipt will record the committed candidate.
- [x] Every gap classified in the three buckets, with no fourth wording.
- [x] Dead code deleted, missing tests added, hard-to-test justified; redundant unit tests removed.
- [x] No `@wip` remains; no implementation detail in the spec.
- [x] Canonical E2E policy read; one `@e2e` per normalized action; variants are regular scenarios.
- [x] Every E2E scenario asserts through the real interface with the reviewed driver.
- [x] Browser UI never replaced by HTTP or `fetch()` (no browser surface).
- [x] Local run command starts the full stack cleanly (node only, no services or twins; verified by the passing suites).
- [x] The files `verify.e2e_command` invokes were identified and read; the tree was searched for parallel implementations (none).
- [x] `verify.e2e_command` differs from `verify.command`; counts prove a strictly smaller E2E scope (589 vs 90; 3 vs 1).
- [x] Implementation matches `design.md`; no deviation.
- [x] Interaction coverage verified (table above).
- [x] No classification outside the three buckets anywhere in this report.

REVIEW: PASS
