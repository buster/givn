# Review: review-diagnostic-corrections

REVIEW: PASS
README-IMPACT: updated - Description
VISUAL-REVIEW: PASS
PERSONAS: developer--primary-user

## Fabrication audit

- **@e2e tag integrity:** the delta declares no `@e2e` scenario and no tag was added or removed. The owning review action keeps its existing permanent `@e2e` scenario.
- **Empty step bodies:** 0 empty/trivial step bodies found across the touched step-definition file `tests/steps/workflow_enforcement_steps.rs`. No `unimplemented!()` or `todo!()` remains in `tests/steps/` or `src/`.
- **Task commits:** all seven checked scenario commits touch `src/gherkin/usecase_md.rs` and/or `tests/steps/workflow_enforcement_steps.rs`; the guard commit adds the parser regression test. No commit only edits `tasks.md` or the feature text.
- **Promised components:** `DocumentScan`, `TitleRecovery`, `canonical_title_id`, `report_missing_headings`, the `required_text` empty-report flag, and the `interaction_section` undeclared-report flag exist in `src/gherkin/usecase_md.rs` (lines 395, 559, 573, 599, 641, 755). `src/spec/routing_table.rs` is unchanged and still consumes `Option<UsecaseDoc>`.
- **Strict mode:** the probe and baseline output are recorded in `tasks.md`; the probe exited 1 with `Step doesn't match any function`, and the runner uses `.fail_on_skipped()`.
- **E2E Then steps:** the delta has no `@e2e` scenario, so there is no downgraded e2e assertion to audit.
- **Browser driver:** not applicable. The interface is terminal; the opposite classification (Web UI) does not hold because the output is plain CLI text and no HTML/CSS/JS is rendered or driven.
- **Configured E2E command:** `givn/commands.yaml` binds `verify.command: ./run-tests.sh --non-e2e` and `verify.e2e_command: ./run-tests.sh --e2e`; the strings differ. `run-tests.sh --e2e` runs `@e2e and not @givn.removed`. The observed scopes are 713 regular passes with 111 skipped, and 111 e2e passes with 713 skipped: the e2e scope is strictly smaller. The archive receipt will carry the per-scope counts.
- **Parallel e2e implementation:** the only e2e step file for the capability is `tests/steps/workflow_enforcement_e2e_steps.rs`; this change adds no second implementation.
- **Design conformance:** the parser file, step file, `review_contract_with_level` helper, exact-suggestion step, `report_undeclared` gate, `report_empty` gate, and unchanged routing file match the reviewed `design.md`. No silent deviation.
- **Interaction coverage:** the owning use-case document has 13 inventory rows; the design matrix has 13 matching rows; the existing `@e2e` scenario for the review action is unchanged. No new interaction and no duplicate e2e coverage.

### Interaction coverage cross-reference

| Inventory entry (capability: consumer action) | Design matrix row | Existing @e2e scenario | Driving mechanism | Status |
|---|---|---|---|---|
| `validate-artifact-ids`: check an unknown artifact | present | Consumer invokes givn check with an unknown artifact and gets a helpful error | CLI subprocess | unchanged |
| `visual-review-gate`: run the review check on a visual change | present | A visual change with complete evidence passes review | CLI subprocess | unchanged |
| `visual-review-gate`: read the visual review guidance | present | Review guidance requires the visual verdict and persona lenses | CLI instruction output | unchanged |
| `spec-overlap-gates`: detect a duplicate title | present | A duplicate title across two permanent spec files is a hard error | CLI subprocess | unchanged |
| `spec-overlap-gates`: inspect duplicate JSON | present | The json output reports duplicate-title findings machine-readably | CLI subprocess | unchanged |
| `review-disposition-gates`: pass a disposed review | present | Review passes when every shape match is disposed and reports the net-delta receipt | CLI subprocess | unchanged |
| `resolve-change-directories`: run integrity in a change | present | Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected | CLI subprocess | unchanged |
| `resolve-change-directories`: reject placeholder commit evidence | present | Integrity check detects a placeholder COMMIT hash in tasks.md and fails | CLI subprocess | unchanged |
| `givn-workflow-enforcement`: run migration coverage check | present | Consumer runs migration coverage check and sees passed evidence | CLI subprocess | unchanged |
| `givn-workflow-enforcement`: run the change-scoped checks | present | Consumer runs check tasks and receives a change verification receipt | CLI subprocess | unchanged |
| `givn-workflow-enforcement`: write a runner result | present | The result helper writes four counts without a scope | CLI subprocess | unchanged |
| `givn-workflow-enforcement`: run the review check on a change with use-case schema errors | present | Consumer runs review and receives every use-case schema problem in one pass | real `givn check review` subprocess | unchanged; seven error variants are regular scenarios |
| `readability-review-gate`: run the readability review during check review | present | A developer runs the review with accounted readability findings and it passes | CLI subprocess | unchanged |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Root-first review diagnostics in the runtime view | `docs/arc42/06-runtime-view.md` §6.7 | affected | Parser flow change and behavior matrix | Scenarios 4-6; unit tests | `parse_usecase_md_diagnostics` gates missing headings on `root_valid` and reports roots before dependent findings | yes |
| Mandatory review gate reports actionable problems in one pass | `docs/arc42/08-crosscutting-concepts.md` §8.12 | affected | Decisions D2/D4 | Scenarios 2, 4, 5, 6 | `report_missing_headings`, `required_text(report_empty)`, `interaction_section(report_undeclared)` | yes |
| Residual advisory-slug and exact-prefix behavior-change risks | `docs/arc42/11-risks-and-technical-debt.md` R-119/R-120 | affected | Risks and D1 | Scenarios 1 and 3 | `canonical_title_id` suggestion only; exact-prefix path returns no document | yes |
| Root diagnostic, dependent diagnostic, title line, section schema | `docs/arc42/12-glossary.md` | affected | Domain vocabulary and Decisions | All scenarios | Parser names and diagnostics use the same vocabulary | yes |
| Chapters 1-5, 7, 9, 10 | no change | no | no new goal, constraint, context, strategy, block, deployment, ADR, or quality scenario | n/a | no new module, command, interface, dependency, or ADR | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary holds **root diagnostic**, **dependent diagnostic**,
**title line**, and **section schema**. The design, delta feature, README, and
owning contract use those terms; the parser names the corresponding mechanics
`root_valid`, `report_missing_headings`, `recover_title`, and
`interaction_section`. No term is invented in code or contradicted by the
specification.

## Coverage classification

No instrumented pass is run in review; the archive receipt will carry the
measured result. Static inspection finds:

- **Dead code:** none. Every new helper is reached by the parser; no dependency
  or public type was added.
- **Missing test coverage:** none. Seven delta scenarios cover the corrected
  report through the real CLI; 14 new parser tests cover document `None`, slug
  edge cases, fragment preservation, duplicate/outside findings, empty required
  sections, both invalid roots, and the valid-root non-suppression guard.
- **Hard to test:** none. The parser is deterministic and offline.
- **Redundant unit tests:** none removed. Each retained test asserts a parser
  branch that the black-box scenarios do not exhaust (document identity,
  suggestion edge cases, fragment schema, duplicate/outside diagnostics, empty
  required sections, both roots invalid).

## E2E coverage

The canonical specs instruction was read. The review action is one normalized
interaction with one existing `@e2e` scenario. The delta adds error variants of
that same action as regular scenarios, with the real `givn check review`
subprocess and stderr/exit-code assertions. No new `@e2e` scenario, no duplicate
action, and no downgraded repository-only assertion. The terminal interface is
not a Web UI: no browser renders the output, so a browser driver is not
applicable.

## Use-case and Persona conformance

`verify-change` remains the owner. Its guarantees, rules, capability list,
typed relationships, and interaction inventory are preserved; the rule now
names the root cause first and then all actionable problems. No capability,
interaction, or Persona is invented. The Persona images remain review lenses,
not Gherkin actors.

| Persona | Disposition |
|---|---|
| `developer--primary-user` | Receives the exact canonical title correction and no dependent missing-section/leaf noise. |
| `agent--technical-role` | Receives a stable grouped report, unchanged exit codes, and a machine-actionable exact suggestion. |
| `givn-maintainer--owner` | Owns the strict schema and the documented suppression boundary; no variant is accepted and no ADR is added. |

## Visual review

The contract declares `Interface: terminal`. The delta has no `@e2e` scenario,
so the per-scenario transcript set is empty; the archived transcript is
historical and depicts the pre-correction output. The corrected outputs below
were captured verbatim from the real CLI in a committed fixture:

```text
error: capability routing decisions do not match the built delta:
  usecase document 'givn/changes/review-diagnostics-demo/specs/review-diagnostics-demo/usecase.md':
    line 1: expected '# Use case: <kebab-case-id>', got '# Use Case: Lending Register'; headings are case-sensitive and IDs are kebab-case — did you mean '# Use case: lending-register'?
    line 39: unknown heading '## Guarantees'; expected one of '## Minimal guarantee', '## Success guarantee' — did you mean '## Minimal guarantee' or '## Success guarantee'?
```

```text
error: capability routing decisions do not match the built delta:
  usecase document 'givn/changes/review-diagnostics-demo/specs/review-diagnostics-demo/usecase.md':
    use-case Level must be '!' or '+', got '`+` — summary use case. Its five capabilities decompose the lending-desk activity into the goals below.'
```

Three fresh-context Persona reviews read both captures. Findings and
dispositions:

| Persona | Finding | Disposition |
|---|---|---|
| `developer--primary-user` | The unknown-heading message offers two alternatives without stating whether one or both sections are required. | boundary — pre-existing unknown-heading text; this change preserves it and adds no report-format change. |
| `developer--primary-user` | The malformed-Level value is the whole line remainder, without a marked token boundary. | boundary — pre-existing message rendering; the diagnosis and rule are correct. |
| `developer--primary-user` | The malformed-Level diagnostic has no line number. | boundary — pre-existing document-level diagnostic shape; the proposal excludes a report-format change. |
| `developer--primary-user` | The routing envelope labels document-shape errors as routing mismatches. | boundary — pre-existing routing-gate framing; unchanged by this change. |
| `agent--technical-role` | The Level finding has no `line N:` prefix, unlike the title finding. | boundary — pre-existing diagnostic shape; a caller can still act on the document path and the named rule. |
| `agent--technical-role` | The `got` value is not delimited from the commentary. | boundary — pre-existing rendering; the exact accepted values remain parseable. |
| `agent--technical-role` | The two-heading alternative requires domain intent. | boundary — pre-existing unknown-heading guidance; not part of the two corrected defects. |
| `agent--technical-role` | Root and dependent findings share one envelope with no severity marker. | boundary — the grouped one-pass report is the reviewed contract; no new format is introduced. |
| `givn-maintainer--owner` | The routing envelope claims a routing mismatch for document-shape errors and no added scenario covers the envelope. | boundary — the change deliberately leaves the envelope and report format unchanged; the scenarios cover the corrected document diagnostics inside it. |

All findings are pre-existing message/envelope characteristics, not regressions
from this change, and each names a contract rule or a failed Persona goal. No
pixel-level, coordinate, or taste finding was accepted. The corrected title
and Level roots are present, the strict schema rejects every variant, and no
dependent missing-section or leaf-capability claim appears.

VISUAL-REVIEW: PASS

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A valid kebab-case use-case title remains accepted` | `A dangling persona reference is reported as an advisory` | boundary |
| `The readability report is unavailable when the experiment is disabled` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `The readability report is unavailable when the experiment is disabled` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `The readability report is unavailable when the experiment is disabled` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `The readability report is unavailable when the experiment is disabled` | `Independent local findings are still reported with a malformed title` | boundary |
| `Id with spaces is rejected` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Id with non-ASCII letters is rejected` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Id with a dot is rejected` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Id ending with a hyphen is rejected` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Id with spaces is rejected` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Id with non-ASCII letters is rejected` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Id with a dot is rejected` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Id ending with a hyphen is rejected` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Id with spaces is rejected` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Id with non-ASCII letters is rejected` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Id with a dot is rejected` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Id ending with a hyphen is rejected` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Id with spaces is rejected` | `Independent local findings are still reported with a malformed title` | boundary |
| `Id with non-ASCII letters is rejected` | `Independent local findings are still reported with a malformed title` | boundary |
| `Id with a dot is rejected` | `Independent local findings are still reported with a malformed title` | boundary |
| `Id ending with a hyphen is rejected` | `Independent local findings are still reported with a malformed title` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Independent local findings are still reported with a malformed title` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `Independent local findings are still reported with a malformed title` | boundary |
| `Routing requires exactly one input` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Routing requires exactly one input` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Routing requires exactly one input` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Routing requires exactly one input` | `Independent local findings are still reported with a malformed title` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Independent local findings are still reported with a malformed title` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Independent local findings are still reported with a malformed title` | boundary |
| `givn addons disable when already disabled is idempotent` | `A valid kebab-case use-case title remains accepted` | boundary |
| `givn skills targets add errors when every given name is unknown` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `givn skills targets add errors when givn/ is not initialised` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `givn skills targets add errors when every given name is unknown` | `Review does not accept a non-kebab-case title as the use-case identifier` | boundary |
| `givn skills targets add errors when givn/ is not initialised` | `Review does not accept a non-kebab-case title as the use-case identifier` | boundary |
| `givn skills targets add errors when every given name is unknown` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `givn skills targets add errors when givn/ is not initialised` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `givn skills targets add errors when every given name is unknown` | `A malformed title does not fabricate missing-section or empty-section diagnostics` | boundary |
| `givn skills targets add errors when givn/ is not initialised` | `A malformed title does not fabricate missing-section or empty-section diagnostics` | boundary |
| `givn skills targets add errors when every given name is unknown` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `givn skills targets add errors when givn/ is not initialised` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `givn skills targets add errors when every given name is unknown` | `Independent local findings are still reported with a malformed title` | boundary |
| `givn skills targets add errors when givn/ is not initialised` | `Independent local findings are still reported with a malformed title` | boundary |
| `Detail mode — unknown change errors` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Uninitialized project errors` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Detail mode — unknown change errors` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Uninitialized project errors` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Detail mode — unknown change errors` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Uninitialized project errors` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Detail mode — unknown change errors` | `Independent local findings are still reported with a malformed title` | boundary |
| `Uninitialized project errors` | `Independent local findings are still reported with a malformed title` | boundary |
| `Path traversal id is rejected by status` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Path traversal id is rejected by archive` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Path traversal id is rejected by check` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Path traversal id is rejected by lint` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Path traversal id is rejected by graph` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Path traversal id is rejected by instructions` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Path traversal id is rejected by status` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Path traversal id is rejected by archive` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Path traversal id is rejected by check` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Path traversal id is rejected by lint` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Path traversal id is rejected by graph` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Path traversal id is rejected by instructions` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Path traversal id is rejected by status` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Path traversal id is rejected by archive` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Path traversal id is rejected by check` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Path traversal id is rejected by lint` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Path traversal id is rejected by graph` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Path traversal id is rejected by instructions` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Path traversal id is rejected by status` | `Independent local findings are still reported with a malformed title` | boundary |
| `Path traversal id is rejected by archive` | `Independent local findings are still reported with a malformed title` | boundary |
| `Path traversal id is rejected by check` | `Independent local findings are still reported with a malformed title` | boundary |
| `Path traversal id is rejected by lint` | `Independent local findings are still reported with a malformed title` | boundary |
| `Path traversal id is rejected by graph` | `Independent local findings are still reported with a malformed title` | boundary |
| `Path traversal id is rejected by instructions` | `Independent local findings are still reported with a malformed title` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A valid kebab-case use-case title remains accepted` | boundary |
| `Missing impact decision refuses the review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `An unknown impact value is refused` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `An unknown impact value is refused` | `Review does not accept a non-kebab-case title as the use-case identifier` | boundary |
| `Missing impact decision refuses the review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `An unknown impact value is refused` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `An unknown impact value is refused` | `A malformed title does not fabricate missing-section or empty-section diagnostics` | boundary |
| `Missing impact decision refuses the review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `An unknown impact value is refused` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Missing impact decision refuses the review` | `Independent local findings are still reported with a malformed title` | boundary |
| `An unknown impact value is refused` | `Independent local findings are still reported with a malformed title` | boundary |
| `The check passes on a complete README` | `A valid kebab-case use-case title remains accepted` | boundary |
| `Updated impact decision passes the review` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A disabled addon needs no impact decision` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A recorded impact decision passes the review` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A CLI product forwards the product's exit code` | `A valid kebab-case use-case title remains accepted` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Check tasks blocked when delta spec has @wip` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Lost E2E evidence blocks migration coverage` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Lower interaction coverage blocks migration coverage` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Lower line coverage blocks migration coverage` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Lower branch coverage blocks migration coverage` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Check tasks blocked when delta spec has @wip` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Lost E2E evidence blocks migration coverage` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Lower interaction coverage blocks migration coverage` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Lower line coverage blocks migration coverage` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Lower branch coverage blocks migration coverage` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Check tasks blocked when delta spec has @wip` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Lost E2E evidence blocks migration coverage` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Lower interaction coverage blocks migration coverage` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Lower line coverage blocks migration coverage` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Lower branch coverage blocks migration coverage` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `Independent local findings are still reported with a malformed title` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Independent local findings are still reported with a malformed title` | variant |
| `Check tasks blocked when delta spec has @wip` | `Independent local findings are still reported with a malformed title` | variant |
| `Lost E2E evidence blocks migration coverage` | `Independent local findings are still reported with a malformed title` | variant |
| `Lower interaction coverage blocks migration coverage` | `Independent local findings are still reported with a malformed title` | variant |
| `Lower line coverage blocks migration coverage` | `Independent local findings are still reported with a malformed title` | variant |
| `Lower branch coverage blocks migration coverage` | `Independent local findings are still reported with a malformed title` | variant |
| `Integrity hook passes when arc42 is disabled` | `A valid kebab-case use-case title remains accepted` | variant |
| `Integrity hook passes on edited arc42 chapter` | `A valid kebab-case use-case title remains accepted` | variant |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A valid kebab-case use-case title remains accepted` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Independent local findings are still reported with a malformed title` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `A valid kebab-case use-case title remains accepted` | boundary |
| `Review passes when a shape match exists only between permanent scenarios` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A modified delta matching its merge target in the same capability is allowed` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A removed delta referencing its target by title is allowed` | `A valid kebab-case use-case title remains accepted` | boundary |
| `The default scope excludes the archive tree` | `A valid kebab-case use-case title remains accepted` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A visual change without a visual verdict fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `An unverified visual verdict cannot pass review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A web contract without viewports fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `An empty evidence file fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A web contract with missing state evidence fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A visual change without a persona disposition fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A visual interface with an empty persona disposition fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `An empty state screenshot fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A deferral without an operator decision fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A visual change with an unknown persona disposition fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A visual change with a reasoned none fails review` | `Review corrects a non-kebab-case use-case title to its canonical identifier` | boundary |
| `A visual change without a visual verdict fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `An unverified visual verdict cannot pass review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A web contract without viewports fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `An empty evidence file fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A web contract with missing state evidence fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A visual change without a persona disposition fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A visual interface with an empty persona disposition fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `An empty state screenshot fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A deferral without an operator decision fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A visual change with an unknown persona disposition fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A visual change with a reasoned none fails review` | `Review corrects an exact-prefix non-kebab-case title with the canonical identifier` | boundary |
| `A visual change without a visual verdict fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `An unverified visual verdict cannot pass review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A web contract without viewports fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `An empty evidence file fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A web contract with missing state evidence fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A visual change without a persona disposition fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A visual interface with an empty persona disposition fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `An empty state screenshot fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A deferral without an operator decision fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A visual change with an unknown persona disposition fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A visual change with a reasoned none fails review` | `A malformed Level does not fabricate missing-section or leaf-capability diagnostics` | boundary |
| `A visual change without a visual verdict fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `An unverified visual verdict cannot pass review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A web contract without viewports fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `An empty evidence file fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A web contract with missing state evidence fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A visual change without a persona disposition fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A visual interface with an empty persona disposition fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `An empty state screenshot fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A deferral without an operator decision fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A visual change with an unknown persona disposition fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A visual change with a reasoned none fails review` | `Independent local findings are still reported with a malformed title` | boundary |
| `A visual change with complete evidence passes review` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A change without a visual contract is exempt from the visual gate` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A terminal change passes review with a transcript` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A change that declares no visual interface is exempt` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A web contract with complete state evidence passes review` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A visual change with a confirmed persona disposition passes review` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A deferral with an operator decision passes review` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A change without a visual interface passes review with a reasoned none` | `A valid kebab-case use-case title remains accepted` | boundary |
| `A machine-only CLI records not applicable and passes review` | `A valid kebab-case use-case title remains accepted` | boundary |

## Split-or-keep

No delta scenario exceeds the long-scenario threshold; no split-or-keep row is
required.
