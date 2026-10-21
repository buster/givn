# Tasks: review-diagnostic-corrections

Release note: Review now corrects a non-kebab-case use-case title to its canonical identifier and withholds section-schema errors that depend on an invalid title or Level, while still reporting genuinely unknown headings.

Use-case constraints carried into every scenario:

- `verify-change`: structural failures outrank other findings; a review PASS requires a fresh green change-scoped receipt; a review report names the root cause of an invalid use-case document first and then all actionable use-case schema and capability-routing problems in one pass; the schema stays strict and semantic validation is not weakened.
- The terminal gate keeps `VISUAL-REVIEW: PASS` and a confirmed `PERSONAS:` line. No `NOT APPLICABLE` or `PERSONAS: none` path is added.
- Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. No Persona is invented or promoted.
- One atomic `givn commit` per scenario with the release note as subject and the scenario title in the body. The valid-title guard records its baseline-green evidence and no fabricated RED.
- Gate order stays integrity, run declaration, overlap, retirement, routing, receipt. No gate correctness or archive behavior changes.

## Setup — runner, strict mode, and scope counts

- [x] Setup task

Design constraints:

- Runner: `tests/features_runner.rs`, cucumber-rs, `.fail_on_skipped()` on both the normal and single-scenario builders (lines 55 and 151). Undefined and pending steps fail.
- `verify.command`: `./run-tests.sh --non-e2e`. `verify.e2e_command`: `./run-tests.sh --e2e`. They are not equal.
- Delta filter: `GIVN_FEATURES=givn/changes/review-diagnostic-corrections/specs`.
- Step file: `tests/steps/workflow_enforcement_steps.rs`. E2E step file unchanged; this delta adds no `@e2e` scenario.
- Step bodies are never empty. RED uses `unimplemented!()` or an undefined step.
- The parser is shared by `gherkin/interaction.rs`, `spec/coverage.rs`, `spec/tree.rs`, `overlap/analysis.rs`, and `cli/archive.rs`; public signatures stay unchanged.

Strict-mode proof:

```sh
scratch=$(mktemp -d)
printf '%s\n' 'Feature: Strictness probe' '  Scenario: Undefined step fails' '    Given no step definition exists for this sentence' > "$scratch/probe.feature"
GIVN_FEATURES="$scratch" cargo test --locked --test features_runner -- --name "Undefined step fails"
```

- [x] Probe exits non-zero; paste command and output:
  ```
  GIVN_FEATURES="$scratch" cargo test --locked --test features_runner -- --name "Undefined step fails"
  Feature: Strictness probe
    Scenario: Undefined step fails
     ✘  Given no step definition exists for this sentence
        Step doesn't match any function
  [Summary] 1 feature, 1 scenario (1 failed), 1 step (1 failed)
  exit=1
  ```
- [x] Remove the probe and re-run the delta suite so the only failures are the `@wip` scenarios:
  ```
  GIVN_FEATURES=givn/changes/review-diagnostic-corrections/specs cargo test --locked --test features_runner
  [Summary] 1 feature, 7 scenarios (1 passed, 6 failed), 11 steps (5 passed, 6 failed)
  exit=1
  ```
  The one pass is the intended valid-title regression guard; the six failures are the unimplemented `@wip` scenarios.
- [x] Paste `verify.command` scenario count and `verify.e2e_command` scenario count; e2e count is strictly less than the full count:
  ```
  GIVN_FEATURES=<delta> features_runner --list                  -> 7
  GIVN_FEATURES=<delta> GIVN_CUCUMBER_TAGS=@e2e --list          -> 0
  ```

## Scenario 1 — Review corrects a non-kebab-case use-case title to its canonical identifier

- [x] Scenario: `Review corrects a non-kebab-case use-case title to its canonical identifier`

- [x] RED
- [x] GREEN
- [x] REFACTOR
- [ ] COMMIT

Design constraints:

- Parser: `src/gherkin/usecase_md.rs`. The title must be exactly `# Use case: <kebab-case-id>`.
- `recover_title` returns `TitleRecovery { suggestion: Option<String>, recovered_id: Option<String> }`. A case-insensitive prefix match with an invalid human remainder yields `suggestion = Some("# Use case: lending-register")` and `recovered_id = None`. The identifier is never accepted.
- `canonical_title_id` lowercases ASCII alphanumerics, collapses every other run into one hyphen, trims trailing hyphens, and returns `Some` only when `validate_id` passes. `Lending Register` → `lending-register`; `Start Project` → `start-project`; `lending--register` → `lending-register`; `Lending.Register` → `lending-register`; `!!!` → `None`.
- Diagnostic: `line 1: expected '# Use case: <kebab-case-id>', got '# Use Case: Lending Register'; headings are case-sensitive and IDs are kebab-case — did you mean '# Use case: lending-register'?`
- Unknown headings stay independent of title validity. `## Guarantees` keeps `unknown heading '## Guarantees'; expected one of '## Minimal guarantee', '## Success guarantee'`.
- Fixture: `review_contract` with title `# Use Case: Lending Register`, an unknown `## Guarantees` section, id `review-diagnostics-demo`, valid Level and sections. Commit the fixture and run the real `givn check review`.
- Steps: new Given `a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and has an unknown "## Guarantees" heading`; new Then `stderr suggests exactly "<text>"` asserting the whole `did you mean '<text>'?` fragment.
- Files: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output:
  ```
  1 scenario (1 failed), 5 steps (4 passed, 1 failed)
  ✘ And stderr suggests exactly "# Use case: lending-register"
    Step panicked. Captured output: not implemented: canonical title suggestion
  ```
- GREEN output:
  ```
  1 scenario (1 passed), 6 steps (6 passed)
  ```
- REFACTOR output:
  ```
  cargo fmt --all; 1 scenario (1 passed), 6 steps (6 passed)
  cargo test --locked --lib usecase_md: 7 passed, 326 filtered out
  ```
- Production files: `src/gherkin/usecase_md.rs` (`TitleRecovery`, `recover_title`, `canonical_title_id`, title diagnostic), `tests/steps/workflow_enforcement_steps.rs` (fixture and exact-suggestion assertion). Planning artifacts: `design.md`, `design-review.md`, `tasks.md`, `arc42.md`, `README.md`, `givn/specs/verify-change/usecase.md`, arc42 chapters 6/8/11/12.
- [x] COMMIT: `251a4858e3f6875ba1f530041f9cd1bf73ff2cc9`

## Scenario 2 — Review does not accept a non-kebab-case title as the use-case identifier

- [x] Scenario: `Review does not accept a non-kebab-case title as the use-case identifier`

- [x] RED
- [x] GREEN
- [x] REFACTOR
- [ ] COMMIT

Design constraints:

- The case-mismatched invalid title makes `scan.title = None`, so the parse returns `None` and routing skips checks that need a document.
- Gate the leaf-capability requirement on `root_valid`, the `## Capabilities` heading being present, `Level == Leaf`, and an empty parsed list. The message is never derived from the invalid human title.
- Dependent routing findings are withheld: no `does not declare capability` and no `has no interaction for capability`.
- The title diagnostic must never suggest the raw invalid title; only `# Use case: lending-register` may appear.
- Fixture: same title as scenario 1 with the `## Capabilities` content replaced by a Markdown table. Routing remains `NEW in review-diagnostics-demo`.
- Steps: new Given `... and whose capability list is a table`; reuse `stderr does not contain`.
- Files: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output:
  ```
  1 scenario (1 failed), 1 step (1 failed)
  ✘ Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and whose capability list is a table
    Step panicked. Captured output: not implemented: non-kebab-case title fixture with a capability table
  ```
- GREEN output:
  ```
  1 scenario (1 passed), 7 steps (7 passed)
  ```
- REFACTOR output:
  ```
  cargo fmt --all; 1 scenario (1 passed), 7 steps (7 passed)
  cargo test --locked --lib usecase_md: 7 passed, 326 filtered out
  ```
- Production files: `src/gherkin/usecase_md.rs` (`root_valid` gate for the leaf requirement, `## Capabilities` heading guard, document construction gate), `tests/steps/workflow_enforcement_steps.rs` (capability-table fixture).
- [x] COMMIT: `369f258f223a7c1a08765201c1e09f3725f4d1ef`

## Scenario 3 — Review corrects an exact-prefix non-kebab-case title with the canonical identifier

- [x] Scenario: `Review corrects an exact-prefix non-kebab-case title with the canonical identifier`

- [x] RED
- [x] GREEN
- [x] REFACTOR
- [ ] COMMIT

Design constraints:

- Exact-prefix path: `# Use case: Lending Register` matches `expected_title`; `validate_id` fails with `Use case ID 'Lending Register' must be a stable kebab-case identifier`.
- The diagnostic carries the same `canonical_title_id` suggestion; the title is not set.
- No spurious `document is missing '# Use case: <id>'`; `title_seen` is true even though `title` is `None`.
- Fixture: title `# Use case: Lending Register`; valid Level and sections.
- Steps: new Given `a clean committed change whose use-case contract starts with "# Use case: Lending Register"`; `stderr contains` and `stderr suggests exactly`.
- Files: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output:
  ```
  1 scenario (1 failed), 1 step (1 failed)
  ✘ Given a clean committed change whose use-case contract starts with "# Use case: Lending Register"
    Step panicked. Captured output: not implemented: exact-prefix non-kebab-case title fixture
  ```
- GREEN output:
  ```
  1 scenario (1 passed), 6 steps (6 passed)
  ```
- REFACTOR output:
  ```
  cargo fmt --all; 1 scenario (1 passed), 6 steps (6 passed)
  cargo test --locked --lib usecase_md: 7 passed, 326 filtered out
  ```
- Production files: `src/gherkin/usecase_md.rs` (canonical suggestion on the exact-prefix `validate_id` path, `title_seen` suppresses the spurious missing-title diagnostic), `tests/steps/workflow_enforcement_steps.rs` (exact-prefix fixture).
- [ ] COMMIT: `COMMIT:`

## Scenario 4 — A malformed title does not fabricate missing-section or empty-section diagnostics

- [ ] Scenario: `A malformed title does not fabricate missing-section or empty-section diagnostics`

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- `parse_document` returns `DocumentScan { title, title_seen, sections, seen, diagnostics }` and no longer emits missing headings.
- `report_missing_headings` is called by the use-case path only when `root_valid`; the fragment path calls it unconditionally.
- `required_text` gains `report_empty: bool`; the use-case path passes `root_valid` for non-Level sections and `false` for `## Level`. Fragments pass `true`.
- Duplicate-title and outside-section checks key on `title_seen`, not `title.is_some()`.
- Fixture: title `# Use Case: Lending Register`; remove the whole `## Rules` section; leave `## Examples` present but empty.
- Then: the canonical suggestion is present; `document schema does not recognise '## Rules'` and `section '## Examples' must not be empty` are absent.
- Steps: new Given `... starts with "# Use Case: Lending Register", omits "## Rules", and leaves "## Examples" empty`.
- Files: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output:
  ```
  ```
- GREEN output:
  ```
  ```
- REFACTOR output:
  ```
  ```
- Production files:
- [ ] COMMIT: `COMMIT:`

## Scenario 5 — A malformed Level does not fabricate missing-section or leaf-capability diagnostics

- [ ] Scenario: `A malformed Level does not fabricate missing-section or leaf-capability diagnostics`

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- `GoalLevel::parse` is unchanged and trims its input. The caller stores `Option<GoalLevel>`: `Ok` → `Some`, `Err` → document diagnostic and `None`. No `Leaf` fallback.
- `root_valid = title.is_some() && level.is_some()`. Missing headings and empty non-Level sections are gated on `root_valid`.
- The leaf requirement runs only when `root_valid`, `level == Leaf`, `## Capabilities` is present, and the list is empty.
- Fixture: valid title, the C3 malformed Level `` `+` — summary use case. Its five capabilities decompose the lending-desk activity into the goals below.``, `## Capabilities` and `## Interactions` set to `- none`, remove `## Examples`.
- Then: `use-case Level must be '!' or '+'` present; `document schema does not recognise '## Examples'` and `must declare at least one capability` absent.
- Steps: new Given `a clean committed change whose use-case contract has a malformed Level and omits "## Examples"`; new helper `review_contract_with_level` with `review_contract` delegating at `!`.
- Files: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output:
  ```
  ```
- GREEN output:
  ```
  ```
- REFACTOR output:
  ```
  ```
- Production files:
- [ ] COMMIT: `COMMIT:`

## Scenario 6 — Independent local findings are still reported with a malformed title

- [ ] Scenario: `Independent local findings are still reported with a malformed title`

- [ ] RED
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- Every section value is computed into a local before the `UsecaseDoc` conditional; moving a reader inside the conditional is a defect. This scenario targets the Rule reader, which is currently inside the conditional.
- `list_section` reports `section '## Rules' must contain one rule per bullet: '<line>'` for a non-bullet line.
- `interaction_section` keeps the Markdown table header, separator, and row-shape checks for a present `## Interactions` section; a wrong separator row reports `has a malformed separator row`.
- `report_undeclared = root_valid && seen.contains(HEADING_CAPABILITIES)`; with an invalid root, an undeclared-capability row is not reported.
- Fixture: title `# Use Case: Lending Register`, a Rule line `This rule is prose, not a bullet.`, and an interactions table with a one-column separator row.
- Steps: new Given `... starts with "# Use Case: Lending Register", has a rule line that is not a bullet, and has a malformed interaction table`.
- Files: `src/gherkin/usecase_md.rs`, `tests/steps/workflow_enforcement_steps.rs`.

Evidence:

- RED output:
  ```
  ```
- GREEN output:
  ```
  ```
- REFACTOR output:
  ```
  ```
- Production files:
- [ ] COMMIT: `COMMIT:`

## Scenario 7 — A valid kebab-case use-case title remains accepted (regression guard)

- [ ] Scenario: `A valid kebab-case use-case title remains accepted`

- [ ] RED (green at baseline — guard, no fabricated RED)
- [ ] GREEN
- [ ] REFACTOR
- [ ] COMMIT

Design constraints:

- This is an explicit green-at-baseline regression guard. The scenario passes before any production change; the RED entry records that baseline-green fact and does not claim a failing run. Rejected alternatives: removing the scenario (loses the direct valid-input CLI proof) and rewriting a permanent scenario as `@givn.modified` (still green at baseline).
- Fixture reuses `review_proof_fixture`: valid title, valid Level, green change verification receipt.
- Assertions: exit 0; no `expected '# Use case:`; no `document schema does not recognise`.
- Steps: existing Given `a clean committed change with a green change verification receipt recorded at one revision`; reuse existing assertions.
- Files: `tests/steps/workflow_enforcement_steps.rs` only if a step is missing; the parser must not regress.

Evidence:

- RED output (baseline green, guard):
  ```
  ```
- GREEN output:
  ```
  ```
- REFACTOR output:
  ```
  ```
- Production files:
- [ ] COMMIT: `COMMIT:`

## Final verification

- [ ] `cargo fmt`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --locked --lib usecase_md`
- [ ] `./run-tests.sh --non-e2e`
- [ ] `./run-tests.sh --e2e`
- [ ] `givn lint --change review-diagnostic-corrections` exits 0 after `@wip` removal
