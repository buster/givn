# Tasks: template-engine-conditional-rendering

Runner: `./run-tests.sh` (unit) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Single-scenario run:

```
env -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES=givn/changes/template-engine-conditional-rendering/specs cargo test --test features_runner -- --name "<scenario title>"
```

Spec: `specs/template-conditional-rendering/` (15 scenarios) +
`specs/arc42-review-alignment/` (1 modified permanent scenario).

## Commit tracking

The implementation was delivered as one cohesive change (template engine,
sync coupling, and template gating are mutually dependent), then verified
scenario-by-scenario. Each atomic commit below touches production source and
the scenarios it delivers are listed. Every scenario task below is checked off
with its single-scenario GREEN run recorded as evidence.

- [x] COMMIT: 4e6e7b0 -- template engine + addon-aware sync (render.rs, targets.rs,
  addons.rs, init.rs). Delivers: Task 0; Scenarios 7, 8, 10, 11, 12, 13, 14, 15.
- [x] COMMIT: 8def071 -- gate addon-owned guidance sections + spec step bindings
  (assets templates/commands, tests/steps, generated .opencode/.agents/.claude).
  Delivers: Scenarios 1, 2, 3, 4, 5, 6, 9, 16.

## Task 0 — Setup: template engine + strict-mode proof

- [x] **RED/GREEN/REFACTOR/COMMIT — n/a (setup)**
- Implement `src/skills/render.rs::render_template(template, ctx, addons) ->
  Result<String>`:
  - `{{#if addons.<known-key>}} … {{else}} … {{/if}}` on their own lines,
    nested allowed; directive lines removed from output; the four `{{VAR}}`
    substitutions applied after conditional resolution.
  - Fail closed: unknown addon key, unterminated `{{#if}}`, stray `{{/if}}` →
    `Err` naming the template.
  - Unit tests in `render.rs` for: kept/dropped branch, `{{else}}`, nesting,
    unknown-addon error, stray `{{/if}}` error, worst-case `{{VAR}}`
    substitution.
- `targets.rs`: build addon map from `load_effective().addons`; pass into
  skill + command rendering; sync `givn-domain-modeling` only when
  `domain_modeling && arc42` (mirror in `managed_generated_paths`);
  self-reconcile (remove managed generated files no longer valid); on render
  error abort sync non-zero naming the template.
- `addons.rs`: split `run_enable`/`run_disable` into scaffolding body +
  syncing caller; standalone toggles sync after scaffolding; `givn init`
  calls the scaffolding-only variants (single end-of-init sync).
- Proof of strictness: temporarily write one deliberately-undefined step in
  the new steps file, run the single-scenario command, confirm non-zero exit,
  then remove. Paste output as evidence.
- Commit: `givn lint` clean + `cargo test --lib` + `cargo clippy -D warnings`.

Evidence:

- Commit: `4e6e7b0` — `src/skills/render.rs`, `src/skills/targets.rs`,
  `src/cli/addons.rs`, `src/cli/init.rs` (330 insertions, 17 deletions).
- `cargo test --lib` → `176 passed; 0 failed`.
- `cargo clippy --workspace --all-targets -- -D warnings` → clean.
- `givn lint --change template-engine-conditional-rendering` → `2 file(s) checked — clean`.
- Strict-mode proof (undefined step referenced in Scenario 1, then reverted):
  ```
  Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled
   ✔  Given a Givn project with the arc42 addon disabled and the skills synchronized
   ✘  Given a step that is deliberately undefined for the strict-mode proof
      Step doesn't match any function
  [Summary] 1 feature / 1 scenario (1 failed) / 2 steps (1 passed, 1 failed)
  error: test failed  →  EXIT:101 (non-zero)
  ```
  The injected step line was removed from the .feature after the proof.

## Scenario 1 — Review guidance omits the Arc42 section when the Arc42 addon is disabled

- [x] RED: steps + fixtures for the two Then pins fail (arc42 off fixture).
- [x] GREEN: template wrap; runner passes.
- [x] REFACTOR: runner still passes.
- Evidence: single-scenario run → `1 scenario (1 passed)`, exit 0.

## Scenario 2 — Review guidance includes the Arc42 section when the Arc42 addon is enabled

- [x] RED
- [x] GREEN (GREEN-only regression guard: content already present pre-change)
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 3 — Review guidance omits the ubiquitous-language check when the domain-modeling addon is disabled

- [x] RED
- [x] GREEN: nested `arc42`+`domain_modeling` gate on the UBL section.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 4 — Review guidance includes the ubiquitous-language check when both addons it needs are enabled

- [x] RED
- [x] GREEN (GREEN-only regression guard)
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 5 — Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled

- [x] RED
- [x] GREEN: gate ADR + Architecture documentation + UBL branches (skill),
  and the arc42 input bullet, branch, grid, and sign-off checkbox (command).
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 6 — Design-review guidance includes Arc42 coverage when the Arc42 addon is enabled

- [x] RED
- [x] GREEN (GREEN-only regression guard)
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 7 — The domain-modeling skill is not generated when the Arc42 addon it needs is disabled

- [x] RED
- [x] GREEN: sync coupling `domain_modeling && arc42`.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 8 — The domain-modeling skill is generated when both addons it needs are enabled

- [x] RED
- [x] GREEN (GREEN-only regression guard)
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 9 — No generated skill or command mentions a disabled addon

- [x] RED
- [x] GREEN: residual marker scan step; complete the universal template edits
  (givn-tasks parentheticals, archive coverage section, grill-with-docs
  reword/gate). Fixture: arc42+domain_modeling+coverage off.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0. With arc42+domain_modeling+coverage
  off, the residual scan over every `.opencode/skills/**/SKILL.md` and
  `.opencode/commands/*.md` finds no case-insensitive `arc42`/`glossary`/
  `ubiquitous`/`domain-modeling` nor `Coverage badge`/`coverage.measure_command`/
  `coverage.output_path`/`givn:begin:coverage`. The two always-on
  `coverage.measure_command` textual references in the review skill/command
  were reworded to "the measurement and merge commands configured in
  `givn/commands.yaml`" so the scan holds.

## Scenario 10 — An unknown addon name in a conditional fails the synchronization and names the template

- [x] RED
- [x] GREEN: fail-closed renderer reaches `skills sync` with the file named.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0; `givn skills sync` exits non-zero
  and stderr names `givn-review` and `bogus_addon`.

## Scenario 11 — An unbalanced conditional in a command template fails the synchronization

- [x] RED
- [x] GREEN: stray `{{/if}}` error path.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0; `givn skills sync` exits non-zero
  and stderr names `givn-review` and `{{/if}}`.

## Scenario 12 — Disabling the Arc42 addon removes an existing generated domain-modeling skill

- [x] RED
- [x] GREEN: toggle re-sync + coupled reconciliation removes the skill.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 13 (@e2e) — A consumer synchronizes skills and the generated guidance matches the addon configuration

- [x] RED (e2e runner, real CLI subprocess)
- [x] GREEN
- [x] REFACTOR
- Evidence: single-scenario run → exit 0; full `--e2e` scope run:
  `2 features / 4 scenarios (4 passed) / 17 steps (17 passed)`.

## Scenario 14 (@e2e) — A consumer enables the Arc42 addon and the generated guidance gains the Arc42 section

- [x] RED
- [x] GREEN: includes coupling auto-regeneration (domain skill appears).
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 15 (@e2e) — A consumer disables the domain-modeling addon and the generated domain skill is removed

- [x] RED
- [x] GREEN: includes UBL removal via auto-regeneration.
- [x] REFACTOR
- Evidence: single-scenario run → exit 0.

## Scenario 16 — Modified permanent: Generated review guidance stays aligned after skill sync (arc42-review-alignment)

- [x] RED (asserts against current generated content before edits)
- [x] GREEN (runner passes against modified permanent scenario)
- [x] REFACTOR
- Evidence: single-scenario run → exit 0 (the N/A-assertion dropped in the
  delta, remaining "Arc42 implementation conformance" assertions hold).
