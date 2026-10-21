# Handoff: review-diagnostic-corrections

## Status

The proposal, delta specification, and design are written. This change fixes
the diagnostic output added by the archived `review-friction-hardening`
change. The gates are correct; do not change gate correctness, archive
behavior, Persona validation, or the terminal visual-review gate.

Work in `/home/buster/projects/givn` only. Do not edit the benchmark or
`/home/buster/projects/givn-harness`.

## Exact C3 evidence

Bundle:

```text
/tmp/opencode/compare-runs/candidate-c3/20261020T234012Z-candidate-c3-r1
```

Initial review failures:

```text
stages/initial/events.stage.jsonl
stages/initial/agent.stdout.log
```

Follow-up review failures:

```text
stages/followup/events.stage.jsonl
stages/followup/agent.stdout.log
```

The final valid use case for the same change:

```text
/tmp/givn-harness-workspaces/candidate-c3-lending-register-b8pa6hta/givn/specs/lending-register/usecase.md
```

Harness findings:

```text
/home/buster/projects/givn-harness/docs/findings/2026-10-21-b4-c3-comparison.md
```

The relevant C3 review output was:

```text
line 1: expected '# Use case: <kebab-case-id>', got '# Use Case: Lending Register'; headings are case-sensitive — did you mean '# Use case: Lending Register'?
document schema does not recognise '## Diagram'
document schema does not recognise '## Examples'
document schema does not recognise '## Minimal guarantee'
document schema does not recognise '## Out of scope'
document schema does not recognise '## Rules'
leaf use case 'Lending Register' must declare at least one capability
```

The malformed Level text was:

```text
`+` — summary use case. Its five capabilities decompose the lending-desk
activity into the goals below.
```

## Defect 1 — invalid title correction

The suggestion preserves an invalid human title. `# Use case: Lending
Register` still contains a space and uppercase letters, so the suggested
document cannot pass. The parser's recovery path assigns the remainder of the
title line as the document ID without validating it against `validate_id`. The
existing test `case_mismatched_title_recovers_and_suggests_the_exact_spelling`
only covers a valid ID (`start-project`).

Required behavior:

- `# Use Case: Lending Register` suggests exactly
  `# Use case: lending-register`.
- The invalid human title is never accepted as the document ID.
- A case-mismatched title whose remainder is already a valid kebab-case ID
  remains recoverable (`# Use Case: review-diagnostics-demo` still suggests
  `# Use case: review-diagnostics-demo` and still feeds the batched checks).
- The schema stays strict; no title variant is accepted.

## Defect 2 — dependent section diagnostics

The same run reported standard section headings as unrecognized and invented a
leaf requirement while the title and Level were invalid. The final accepted
narrative contains `## Rules`, `## Examples`, `## Minimal guarantee`, and
`## Out of scope`, and it archived successfully. The root causes were the
invalid title and the invalid Level. The parser accepted the invalid title as
the document identity, fell back to `Leaf` when the Level failed to parse, and
then emitted missing-section and leaf-capability findings derived from those
invented values.

Required behavior:

- When the title or Level is invalid, report that root diagnostic and suppress
  dependent schema-completeness findings: missing required headings, empty
  required sections, the leaf-use-case capability requirement, and routing
  complaints that need a document.
- Genuinely unknown headings such as `## Guarantees` are still reported.
- Independent local findings (malformed list bullets, malformed interaction
  tables) are still reported in the same pass.
- A valid-root document still reports every missing section and independent
  schema problem in one pass.
- The schema stays strict; the batched report is preserved, not weakened.

## Files to change

| File | Work |
|---|---|
| `src/gherkin/usecase_md.rs` | Implement the root-first parser flow, canonical title suggestion, suppression boundary, and unit tests described in `design.md`. |
| `src/spec/routing_table.rs` | No change expected. The existing `Option<UsecaseDoc>` contract must suppress dependent routing checks when the parser returns `None`. Audit it; change only if a path fabricates a document from a root-invalid parse. |
| `tests/steps/workflow_enforcement_steps.rs` | Add the six fixture Givens and reuse the existing CLI and stderr assertions. Extend `review_contract` with a Level parameter. |
| `givn/changes/review-diagnostic-corrections/specs/verify-change/givn-workflow-enforcement.feature` | Already written. Adjust only if the design-review or step-writing step finds a concrete mismatch. |

Do not add a new capability, a new interaction inventory entry, or a new
`@e2e` scenario. Do not edit the archived `review-friction-hardening` change.
Do not add a `PERSONAS: none` escape or a `VISUAL-REVIEW: NOT APPLICABLE` path
for terminal.

## Non-negotiable constraints

- Preserve the existing batched diagnostics and strict schema. The schema does
  not become lenient; only the correction and the dependent cascade change.
- Do not change gate correctness, gate order, archive behavior, Persona
  validation, terminal visual review, or the benchmark/harness.
- Do not accept the invalid recovered title. A canonicalized title is a
  suggestion only; the author must make the edit.
- Do not suppress genuinely unknown headings.
- Do not introduce a new dependency, CLI flag, JSON schema, or report format.
- Do not run a paid benchmark.
- Follow the repository's TDD workflow: one scenario at a time, RED/GREEN/
  REFACTOR, tick `tasks.md` immediately after each scenario, and one atomic
  `givn commit` per scenario with the release note as the message subject and
  the scenario title in the body.
- Keep the terminal visual gate intact. The delta adds no new `@e2e` scenario,
  so the per-scenario transcript requirement is empty; the existing transcript
  remains the reference for the same review action. `review.md` must still
  carry `VISUAL-REVIEW: PASS` and a confirmed `PERSONAS:` line. Do not add a
  `VISUAL-REVIEW: NOT APPLICABLE` path or `PERSONAS: none`.

## Acceptance criteria

1. `# Use Case: Lending Register` produces the exact suggestion
   `# Use case: lending-register`; the invalid title is not used as the
   document ID and the document does not become usable for dependent checks.
2. The exact-prefix invalid ID (`# Use case: Lending Register`) is also not
   recovered.
3. A case-mismatched but valid ID (`# Use Case: review-diagnostics-demo`)
   remains recoverable and still produces the complete batched report.
4. A malformed title does not produce
   `document schema does not recognise '## Rules'` (or any other
   missing-heading diagnostic) in the same run.
5. A malformed Level does not produce a missing-heading diagnostic or
   `leaf use case ... must declare at least one capability`.
6. A genuinely unknown heading (`## Guarantees`) is still reported, including
   when the title is invalid.
7. A valid kebab-case title and valid Level still pass the parser and the
   review check, with no title or missing-heading diagnostic.
8. All existing archived review-diagnostics scenarios still pass, including
   the one-pass batch, the exact case-mismatch correction, the missing-section
   batch, the single-report-per-document case, and the extension narrative
   case.
9. `givn lint --change review-diagnostic-corrections` reports only the expected
   `@wip` warnings before implementation; after implementation the `@wip` tags
   are removed and lint is clean.
10. No file outside the declared scope changes.

## Test commands

Change suite (the effective `verify.command`):

```sh
./run-tests.sh --non-e2e
```

E2E replay (the effective `verify.e2e_command`; this change adds no e2e
scenario, so the delta is not selected, but the existing corpus must stay
green):

```sh
./run-tests.sh --e2e
```

Parser unit tests:

```sh
cargo test --locked --lib usecase_md
```

One named scenario (use the exact title from the delta feature):

```sh
GIVN_FEATURES=givn/changes/review-diagnostic-corrections/specs \
  cargo test --locked --test features_runner -- --name "<scenario title>"
```

Strict-mode proof for the tasks setup (not committed):

```sh
GIVN_FEATURES="<scratch-dir>" \
  cargo test --locked --test features_runner -- --name "Undefined step fails"
```

The proof must exit non-zero. `tests/features_runner.rs` calls
`.fail_on_skipped()` on both builders (lines 55 and 151), so an undefined or
pending step is a failure.

## Validation notes

- `givn spec route --change review-diagnostic-corrections` reported signal for
  `EXTEND givn-workflow-enforcement` during proposal routing. Later advisory
  re-runs vary with proposal prose; the recorded `EXTEND` decision stands
  because the existing capability owns review-time diagnostics. Report the
  current route output in the workflow notes rather than treating a later tie
  as a capability change.
- `givn lint --change review-diagnostic-corrections` will report `@wip`
  warnings while the six delta scenarios are unimplemented. That is expected.
  The delta also triggers a `[SHAPE]` advisory between “Review corrects a
  non-kebab-case use-case title to its canonical identifier” and the permanent
  “A case-mismatched document title is refused with the exact correction”.
  They are distinct variants (invalid human title versus valid kebab-case ID);
  the review step records a `variant` disposition.
- A full `givn lint` on 2026-10-21 is clean apart from existing advisories
  (58 files checked, exit 0). There is no known blocking baseline lint issue.
  The inventory mismatch mentioned in the archived `review-friction-hardening`
  design (“The result helper writes four counts without a scope” not tagged
  `@e2e`) is no longer present; that scenario is tagged `@e2e` in the
  permanent feature.
- `givn status --change review-diagnostic-corrections` should show proposal,
  specs, and design done; arc42-docs, design-review, tasks, and review remain.

## What remains

1. **arc42-docs** — run `givn instructions arc42-docs --change
   review-diagnostic-corrections` and record the glossary terms **root
   diagnostic**, **dependent diagnostic**, **title line**, and **section
   schema** if absent. The runtime view changes only in the parser's diagnostic
   decision flow.
2. **design-review** — run `/givn-design-review review-diagnostic-corrections`
   in a fresh context. It is mandatory before tasks. Do not continue it from
   the session that wrote the design.
3. **tasks** — break the six scenarios into RED/GREEN/REFACTOR tasks, including
   the strict-mode setup proof and one commit per scenario.
4. **implementation** — follow `design.md`; keep the parser and step changes
   within the declared files; run the single-scenario command for each RED and
   GREEN; tick `tasks.md` immediately; commit each scenario atomically.
5. **review** — remove `@wip`, run the full change suite and e2e scope, record
   the review evidence and terminal verdict, and confirm the interaction
   inventory is unchanged.
6. **archive** — run the archive workflow only after review signs off.
