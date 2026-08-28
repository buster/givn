# Tasks: template-engine-conditional-rendering

Runner: `./run-tests.sh` (unit) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Single-scenario run:

```
env -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES=givn/changes/template-engine-conditional-rendering/specs cargo test --test features_runner -- --name "<scenario title>"
```

Spec: `specs/template-conditional-rendering/` (15 scenarios) +
`specs/arc42-review-alignment/` (1 modified permanent scenario).

## Task 0 — Setup: template engine + strict-mode proof

- [ ] **RED/GREEN/REFACTOR/COMMIT — n/a (setup)**
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

## Scenario 1 — Review guidance omits the Arc42 section when the Arc42 addon is disabled

- [ ] RED: steps + fixtures for the two Then pins fail (arc42 off fixture).
- [ ] GREEN: template wrap; runner passes.
- [ ] REFACTOR: runner still passes.
- [ ] COMMIT (hash):

## Scenario 2 — Review guidance includes the Arc42 section when the Arc42 addon is enabled

- [ ] RED
- [ ] GREEN (GREEN-only regression guard: content already present pre-change)
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 3 — Review guidance omits the ubiquitous-language check when the domain-modeling addon is disabled

- [ ] RED
- [ ] GREEN: nested `arc42`+`domain_modeling` gate on the UBL section.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 4 — Review guidance includes the ubiquitous-language check when both addons it needs are enabled

- [ ] RED
- [ ] GREEN (GREEN-only regression guard)
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 5 — Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled

- [ ] RED
- [ ] GREEN: gate ADR + Architecture documentation + UBL branches (skill),
  and the arc42 input bullet, branch, grid, and sign-off checkbox (command).
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 6 — Design-review guidance includes Arc42 coverage when the Arc42 addon is enabled

- [ ] RED
- [ ] GREEN (GREEN-only regression guard)
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 7 — The domain-modeling skill is not generated when the Arc42 addon it needs is disabled

- [ ] RED
- [ ] GREEN: sync coupling `domain_modeling && arc42`.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 8 — The domain-modeling skill is generated when both addons it needs are enabled

- [ ] RED
- [ ] GREEN (GREEN-only regression guard)
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 9 — No generated skill or command mentions a disabled addon

- [ ] RED
- [ ] GREEN: residual marker scan step; complete the universal template edits
  (givn-tasks parentheticals, archive coverage section, grill-with-docs
  reword/gate). Fixture: arc42+domain_modeling+coverage off.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 10 — An unknown addon name in a conditional fails the synchronization and names the template

- [ ] RED
- [ ] GREEN: fail-closed renderer reaches `skills sync` with the file named.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 11 — An unbalanced conditional in a command template fails the synchronization

- [ ] RED
- [ ] GREEN: stray `{{/if}}` error path.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 12 — Disabling the Arc42 addon removes an existing generated domain-modeling skill

- [ ] RED
- [ ] GREEN: toggle re-sync + coupled reconciliation removes the skill.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 13 (@e2e) — A consumer synchronizes skills and the generated guidance matches the addon configuration

- [ ] RED (e2e runner, real CLI subprocess)
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 14 (@e2e) — A consumer enables the Arc42 addon and the generated guidance gains the Arc42 section

- [ ] RED
- [ ] GREEN: includes coupling auto-regeneration (domain skill appears).
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 15 (@e2e) — A consumer disables the domain-modeling addon and the generated domain skill is removed

- [ ] RED
- [ ] GREEN: includes UBL removal via auto-regeneration.
- [ ] REFACTOR
- [ ] COMMIT (hash):

## Scenario 16 — Modified permanent: Generated review guidance stays aligned after skill sync (arc42-review-alignment)

- [ ] RED (asserts against current generated content before edits)
- [ ] GREEN (runner passes against modified permanent scenario)
- [ ] REFACTOR
- [ ] COMMIT (hash):
