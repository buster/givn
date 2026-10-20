# Handoff: review-friction-hardening

## Evidence sources

- Harness finding: `/home/buster/projects/givn-harness/docs/findings/2026-10-20-clean-comparison.md`
- C2 initial stage events: `/tmp/opencode/compare-runs/candidate-c2/20261020T194021Z-candidate-c2-r1/stages/initial/events.stage.jsonl`
- C2 follow-up stage events: `/tmp/opencode/compare-runs/candidate-c2/20261020T194021Z-candidate-c2-r1/stages/followup/events.stage.jsonl`

## What this change fixes

Two defects from the clean B3/C2 comparison. Both are gate-correct but cause
avoidable iterations.

### 1. `givn check review` reports one use-case schema/routing error at a time

C2's initial stage spent six review iterations on this sequence (verbatim
`error:` lines from the stage output):

```text
decision NEW in lending-register: usecase.md failed to parse: line 1: expected '# Use case: <kebab-case-id>', got '# Use Case: lending-register',
decision NEW in lending-register: usecase.md failed to parse: line 19: unknown heading '## Guarantees'
decision NEW in lending-register: usecase.md failed to parse: section '## Capabilities' must contain one capability per bullet
decision NEW in lending-register: usecase.md failed to parse: interaction references undeclared capability 'register-readiness'
decision NEW in lending-register: usecase.md failed to parse: section '## Examples' must contain one example per bullet
```

Each fix exposed the next error. The same parse failure was also repeated once
per delta capability (five times in the observed run). The target behavior is
one report with every schema/routing problem, each line named once.

### 2. `givn archive` refuses a green proof after documentation-only edits

C2 had one archive failure in each stage. Exact observed refusal:

```text
error: archive blocked: the change verification receipt is stale for 'lending-register'. receipt head 08ec128de971303d3545b125bdbbe339499f23fe, current head d5ec60de47a1e41b69d3058fe0b34d1323d4a55b; receipt features 5256de975f66cbcd52f33f6cdc0c577062afa3667181c6171171dec9f6c8c813, current features 5256de975f66cbcd52f33f6cdc0c577062afa3667181c6171171dec9f6c8c813; receipt command 1031237dac4a7328aa009dc5586392005a11f5408a7dc7b8a4726efc55a93a8e, current command 1031237dac4a7328aa009dc5586392005a11f5408a7dc7b8a4726efc55a93a8e. Run `givn check tasks --change lending-register` again.
```

The feature and command digests were unchanged; only the head differed, because
the receipt itself was committed after the proof. The agent reran
`givn check tasks`, and archive succeeded in the same stage. The target
behavior is reuse without a change-suite rerun, with a tested-content digest
that still refuses code, delta-feature, verify-command, and other tested
content changes.

## Non-negotiable constraints

- Keep the terminal interface check: `Interface: terminal` requires
  `VISUAL-REVIEW: PASS` and committed transcript evidence. Do not add
  `VISUAL-REVIEW: NOT APPLICABLE` for terminal.
- Keep the Persona gate strict: terminal changes require a confirmed Persona
  file and `PERSONAS: <persona>`. Do not add `PERSONAS: none`.
- Do not invent a Persona or change Persona validation.
- Do not change the benchmark or harness, and run no paid benchmark.
- Keep the archive's merged regular/e2e suite run exactly once. Add no extra
  test run.
- Keep one coherent release note: both fixes ship as this change.

## Decisions already made

- Keep the use-case schema strict. Do not accept `# Use Case`, case-insensitive
  headings, prose in bullet lists, or undeclared interaction capabilities.
  Batch the diagnostics and suggest the exact spelling.
- Batch only the use-case schema and capability-routing family. Preserve the
  existing review gate order (integrity, run declaration, overlap, retirement,
  routing, receipt). Structural failures still outrank later findings.
- Bind the receipt with `tested_content_digest` and
  `content_scope: "change-proof/v1"`. Schema 2. Schema 1 keeps strict head
  equality and needs one rerun.
- Share the freshness rule between `givn check review` and `givn archive`.
- Keep the archive's merged regular/e2e suite run exactly once. Add no test
  run.
- Keep the terminal gate: `Interface: terminal` requires
  `VISUAL-REVIEW: PASS` and committed transcripts. Do not add
  `VISUAL-REVIEW: NOT APPLICABLE` for terminal.
- Keep the Persona gate strict: terminal changes require a confirmed Persona
  file and `PERSONAS: <persona>`. Do not add `PERSONAS: none`.
- Do not invent a Persona or change Persona validation.
- Do not change the benchmark or harness. No paid benchmark runs.

## Files to change

### Production

- `src/gherkin/usecase_md.rs` — diagnostic collector; parser recovery; exact
  suggestions; `parse_usecase_md_diagnostics` / `parse_fragment_md_diagnostics`;
  existing semantic checks unchanged.
- `src/spec/routing_table.rs` — parse each referenced use-case/fragment document
  and every active delta narrative once; collect all routing errors;
  deduplicate; group by document; keep the existing error prefix.
- `src/proof.rs` — receipt schema 2; `tested_content_digest`; `content_scope`;
  documentation/evidence path predicate; digest; `freshness` result; schema 1
  fallback.
- `src/scm.rs`, `src/git.rs`, `src/fossil.rs` — `Backend::content_paths()`
  (tracked plus untracked non-ignored; Git NUL-separated `ls-files`, Fossil
  `ls` plus `extras`).
- `src/cli/check.rs` — pass the backend when writing the receipt; use the
  shared freshness result in review; retain the no-test-run behavior.
- `src/cli/archive.rs` — use the shared freshness result before mutation; print
  the reused message; refuse with every stale reason; do not alter the merged
  suite or candidate flow.
- `src/integrity_hook.rs` — no change; the integrity gate stays first and
  fail-fast.

### Delta specs (already written)

- `givn/changes/review-friction-hardening/specs/verify-change/givn-workflow-enforcement.feature`
- `givn/changes/review-friction-hardening/specs/archive-change/archive-proof.feature`
- `givn/changes/review-friction-hardening/specs/verify-change/usecase.md`
  (delta narrative; base hash points at the current permanent document)
- `givn/changes/review-friction-hardening/specs/archive-change/usecase.md`
  (delta narrative; base hash points at the current permanent document)

### Tests

- `tests/steps/workflow_enforcement_steps.rs` — review diagnostic unit steps.
- `tests/steps/workflow_enforcement_e2e_steps.rs` — real `givn check review`
  E2E step.
- `tests/steps/archive_proof_steps.rs` — proof-reuse and stale-boundary steps.
- `tests/steps/archive_proof_e2e_steps.rs` — real `givn archive` E2E step and
  invocation-log assertions.
- `tests/steps/archive_proof_support.rs` — update `refresh_change_receipt` for
  the new `write_change_receipt` signature; add docs-only commit helpers.
- `src/proof.rs`, `src/gherkin/usecase_md.rs`, `src/spec/routing_table.rs`
  unit tests.

### Documentation (arc42-docs step)

- `docs/arc42/02-architecture-constraints.md` — TC-39 receipt binding.
- `docs/arc42/04-solution-strategy.md` — change verification reuse.
- `docs/arc42/05-building-block-view.md` — receipt/parser responsibilities.
- `docs/arc42/06-runtime-view.md` §6.7.1 — documentation-only reuse and stale
  reasons.
- `docs/arc42/12-glossary.md` — change verification receipt definition and
  `Documentation-only revision` term.
- Permanent `givn/specs/verify-change/usecase.md` and
  `givn/specs/archive-change/usecase.md` are replaced by the delta narratives
  at archive; do not edit them by hand during implementation.

## Exact docs-only boundary

`is_documentation_or_evidence_path` returns true only for:

- any `*.md` directly in `givn/changes/<change>/` (including `proposal.md`,
  `design.md`, `design-review.md`, `tasks.md`, `review.md`, `arc42.md`,
  `HANDOFF.md`);
- `givn/changes/<change>/evidence/**`,
  `givn/changes/<change>/verification/**`,
  `givn/changes/<change>/results/**`,
  `givn/changes/<change>/verification.json`;
- `givn/changes/<change>/specs/**/usecase.md`,
  `givn/changes/<change>/specs/**/fragment.md`,
  `givn/specs/**/usecase.md`, `givn/specs/fragments/fragment.md`;
- `givn/personas/**`, `givn/archive/**`, `givn/history/**`;
- `givn/.spec-index.json`, `givn/.spec-*-cache.json`;
- `docs/arc42/**`, `README.md`.

Everything else is tested content. Never docs-only: `src/**`, `tests/**`,
`assets/**`, `run-tests.sh`, `measure-coverage.sh`, `merge-coverages.sh`,
`Cargo.toml`, `Cargo.lock`, `givn/commands.yaml`, `givn/config.yaml`, any
`*.feature` under the active change or `givn/specs/**`, `docs/**` outside
`docs/arc42/**`, `CHANGELOG.md`.

## Acceptance criteria

1. A single `givn check review` on a use-case contract with a case-mismatched
   title, an unknown `## Guarantees` heading, a non-bullet capability line, an
   undeclared interaction capability, and a non-bullet example line reports all
   of them, each distinct problem once, and exits non-zero.
2. The same review still refuses `# Use Case:` and suggests
   `# Use case: <id>`; no schema variant is accepted.
3. A review with several missing/undeclared capabilities reports all routing
   errors in one pass, not one per capability and not one per run.
4. A review validates an active delta `usecase.md`/`fragment.md` narrative even
   when its capability decision is `EXTEND`; a schema error there is reported
   before archive.
5. `givn review` after a documentation-only revision passes without running
   the change suite and prints that the proof was reused.
6. `givn archive` after a documentation-only revision (including a commit that
   contains only `verification/tasks-verification.json`) proves the archive
   without invoking the change suite. The merged regular scope runs once with
   `givn/specs`; no `givn/changes/.../specs` invocation occurs.
7. Archive refuses with `verification receipt is stale` and no candidate when:
   - a delta feature changes;
   - the verify command changes;
   - source code or any other tested content changes.
8. Schema 1 receipts keep strict head equality; unknown newer schemas are
   refused.
9. The terminal `VISUAL-REVIEW: PASS` + transcript gate and the Persona gate
   still behave exactly as before.
10. Existing permanent scenarios still pass, especially:
   - `Review does not run the full suite`
   - `Review refuses a receipt after the delta specs changed`
   - `Review accepts a current verification receipt`
   - `Archive refuses a stale change verification receipt before merging`
   - `Archive reuses the change receipt and runs only the merged suite`
   - `Consumer runs check tasks and receives a change verification receipt`

## Test commands

```sh
# Supporting Rust unit tests
cargo test --locked --lib

# Change-scoped suite (the effective verify.command)
./run-tests.sh --non-e2e

# Merged/e2e suite (the effective verify.e2e_command)
./run-tests.sh --e2e

# One named delta scenario
GIVN_FEATURES=givn/changes/review-friction-hardening/specs \
  cargo test --locked --test features_runner -- --name "<scenario title>"

# Strict-mode proof (must exit non-zero; scratch feature is not committed)
scratch=$(mktemp -d)
printf '%s\n' \
  'Feature: Strictness probe' \
  '  Scenario: Undefined step fails' \
  '    Given no step definition exists for this sentence' > "$scratch/probe.feature"
GIVN_FEATURES="$scratch" \
  cargo test --locked --test features_runner -- --name "Undefined step fails"
```

## Evidence to capture

Two committed transcripts under
`givn/changes/review-friction-hardening/evidence/visual/<scenario-slug>/transcript.txt`:

1. `consumer-runs-review-and-receives-every-use-case-schema-problem-in-one-pass/transcript.txt`
2. `consumer-archives-a-change-after-documentation-only-review-edits-and-the-proof-is-reused/transcript.txt`

Each records the scenario title, exact argv, working-directory disposition,
exit code, and raw stdout/stderr. The terminal gate requires these; there is no
`NOT APPLICABLE` waiver.

## Expected lint warnings

`givn lint --change review-friction-hardening` exits 2 while the delta scenarios
carry `@wip`: one WIP finding per scenario (8 at the time of handoff). Remove
`@wip` scenario by scenario as implementation lands; the count must reach 0.

The subset advisories are heuristics, not blocking findings. Two are
intentional and must be dispositioned at review:

- `Archive reuses a proof committed with its receipt` is a narrower case of the
  documentation-only archive E2E; it preserves the exact C2 receipt-commit
  case.
- `Archive refuses a proof after tested content changed` overlaps the
  permanent `Archive refuses a stale change verification receipt before
  merging` scenario; the outline broadens that refusal from delta features to
  every tested-content boundary.

The remaining subset advisories compare unrelated Given-only scenarios and are
noise. The overlap gate, not lint, is the blocking authority.

Pre-existing baseline finding, not caused by this change: a full `givn lint`
(no `--change`) exits 3 with one blocking inventory mismatch in
`verify-change`: `givn-workflow-enforcement` currently declares 3
interactions and has 2 `@e2e` scenarios; after this delta merges it will
declare 4 and 3. In both states `The result helper writes four counts without
a scope` is the advertised E2E scenario that has no `@e2e` tag. This predates
the change and is outside its scope. The delta-only `givn lint --change review-friction-hardening` does not
report it; it reports only the WIP findings above and subset advisories. If the
archive gate or the release path starts requiring a clean full lint, treat
that as a separate decision, not as an implicit change to this scope.

## What remains

1. `arc42-docs` — `givn status` reports it next after `design`. It was
   intentionally not written by the architect, who stopped after `design.md`.
   Update the chapters listed under "Documentation (arc42-docs step)".
2. Design review in a fresh context (`/givn-design-review review-friction-hardening`).
3. Task breakdown (`givn instructions tasks --change review-friction-hardening`).
4. Implementation with RED/GREEN/REFACTOR per scenario, one commit per
   scenario, ticking `tasks.md` immediately.
5. Review, then archive under the unchanged terminal Persona and visual
   evidence gates.

Do not write `design-review.md`, `tasks.md`, or `review.md` from this handoff;
those are separate workflow steps.
