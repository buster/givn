# Design: seed-artifact-lifecycle

## Technical Approach

Guidance edits plus one advisory check in the static lint command. No new
dependency, manifest entry, artifact, or runtime state is introduced. The
existing asset cascade, skill renderer, and lint pipeline are reused.

## Seed Content Schema

`assets/skills/givn-ideation/SKILL.md.tmpl` gains the seed schema next to the
Handoff section, following the use-case schema precedent in the same file:

```markdown
# Change seed: <use-case-id>

Status: ready

- Use-case ID: <stable use-case-id>
- Topic: <topic-id>
- Personas: <persona-id>, or none — <reason>
- Suggested start: <order position and dependencies>
- Handoff: <exact handoff status>

## Accepted decisions

### <decision title>

- Context: <what forced the decision>
- Options:
  1. **<option name>** — <what it means>
     In practice: <concrete example of what the user sees or the system does>
     Gain: <what improves>
     Cost: <what gets harder or is given up>
- Chosen: **<option name>** because <reason>. Rejected: <option> because <reason>.

## Open questions

### <short title>

<full standalone question: what it blocks, the context, the options with
meaning, gain, cost, and a concrete example each>
```

Status values, stated with the schema: `Status: ready`,
`Status: proposed by <change-id>`, `Status: implemented in <change-id>`,
`Status: superseded by <reference>`, `Status: withdrawn`.

## Guidance Edits

| Asset | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Seed schema and status values in Handoff; Handoff writes every seed with `Status: ready`. |
| `assets/commands/givn-ideation.md` | Guardrail: Handoff writes every seed with `Status: ready` and the seed schema. |
| `assets/commands/givn-propose.md` | When the change originates from a seed: read it, record its path in the proposal's Use-Case Context, and set the seed status to `proposed by <change-id>`. |
| `assets/instructions/proposal.md` | Record the seed path; stamping rule with the exact status string. |
| `assets/templates/proposal.md` | Use-Case Context gains the seed path field. |
| `assets/commands/givn-archive.md` | On success, stamp the originating seed `implemented in <change-id>` or `withdrawn` with a one-line reason; when the proposal records no seed, record "no seed" explicitly. |

Stamping is prose guidance in this change, not runtime code: the handover
scopes `givn new --from-seed` and automatic transitions out, so no CLI
command reads or writes seeds. The advisory check is the only seed-aware
runtime behavior.

## Advisory Seed-Status Check

New function in `src/cli/lint.rs`: `check_seed_status(project_root) -> Vec<Finding>`.

- Scan root: `givn/ideation/*/changes/*.md`, sorted; a missing
  `givn/ideation/` directory returns no findings (no-op).
- Runs only in the default whole-tree scope (`paths` empty and no `--change`).
  Explicit paths and `--change` keep their current scope and do not emit seed
  advisories.
- Runs before the "no .feature files found" early return, so a project with
  seeds and no features still reports them. That early return keeps its
  existing stdout line (`givn lint: no .feature files found under givn/`),
  renders any seed findings on stderr first, and keeps exit code 0; its JSON
  output emits the seed findings instead of an empty array.
- The finding-count summary applies only when at least one feature file was
  checked. An advisory-only run therefore exits 0 while still being visible
  on stderr and in the count.

Parse rule (tolerant, status line only):

1. Read the file; find the first line whose trimmed text starts with
   `Status:`. Prose before and after is ignored; the first match wins.
2. Classify the value: `ready`, `implemented in …`, `superseded by …`, or
   `withdrawn` are valid; `proposed by <id>` triggers the change lookup; an
   absent, unrecognised, or empty-reference value is the missing-status case.
3. For `proposed by <id>`: a directory under `givn/changes/<id>` is valid; a
   directory under `givn/archive/<id>` is the stale case; neither is the
   dangling case.

Findings:

| Case | Message |
|---|---|
| missing / unrecognised status | `change seed has no valid status line (expected ready, proposed by <change-id>, implemented in <change-id>, superseded by <reference>, or withdrawn)` |
| `proposed by` with no such change | `change seed is proposed by '<id>', but no such change exists` |
| `proposed by` an archived change | `change seed is proposed by '<id>', but that change is already archived` |

Finding shape: `kind = "seed_status"`, `level = "warning"`, one file path, one
message. Rendered on stderr with the distinct label `[SEED ]`.

Exit-code contract: advisory findings are counted in the summary's finding
count (so the summary stays truthful) but never set the parse-error, duplicate,
empty-feature, use-case-error, or @wip flags. An advisory-only run exits 0.
JSON output carries the same findings with `kind`/`level`.

## Spec Coverage

| Delta scenario | Asserted surface |
|---|---|
| The ideation guidance carries the seed schema and its status values | generated givn-ideation guidance phrases: "change seed", "Status: ready", "proposed by", "implemented in", "superseded by", "withdrawn", "Accepted decisions", "Suggested start" |
| A change seed without a status line is reported as an advisory | stderr `[SEED ]` + "no valid status line", a clean seed not reported, exit 0 |
| A change seed proposed by a missing change is reported as an advisory | stderr `[SEED ]` + the id on the no-features path, a seed proposed by an active change not reported, exit 0 |
| A change seed proposed by an archived change is reported as stale | stderr `[SEED ]` + "already archived", exit 0 |
| An unrecognised seed status is reported and a status after prose is honoured | `[SEED ]` for `Status: done`, prose-preceded `Status: ready` not reported |
| An advisory seed finding is counted in the summary and still exits 0 | stderr `givn lint: 1 file(s) checked, 1 finding(s)`, exit 0 |
| The propose and archive commands carry the seed stamping instructions | generated givn-propose command contains "proposed by"; generated givn-archive command contains "implemented in" |

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| Generated ideation guidance is synchronized from canonical assets (author-change rule) | `givn/specs/author-change/usecase.md` | Seed schema and status values live in the ideation skill template | The ideation guidance carries the seed schema and its status values |
| Lint is a static check with a stable exit-code contract (verify-change rules) | `givn/specs/verify-change/usecase.md` | Advisory findings never affect the exit code | The three advisory scenarios assert exit 0 |
| Ideation parks unknowns as full questions | `givn/specs/fragments/ideation-skill.feature` | Seed open questions use the dialog policy's option treatment | Existing ideation guidance scenarios remain green |

Persona impact: `agent--technical-role` consumes the generated guidance and
the lint output; `givn-maintainer--owner` owns the rule corpus and the seed
lifecycle convention. No Persona is invented or promoted; Gherkin actors stay
the Developer and the Authoring agent.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| ideation-skill | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-guidance regex steps; no new file |
| lint-command | `tests/steps/mod.rs` | Real `givn lint` subprocess; adds one fixture Given for writing a change seed file and one `stderr does not contain` assertion |
| prose-consolidation | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-command regex steps; no new file |

The new Given is fixture setup (`a change seed at "<path>" with:`), matching
the existing `a .feature file at "<path>" with:` step. Assertions reuse the
existing `the command exits 0`, `stderr contains`, and `stdout contains a
line` steps, plus the new negative stderr assertion.

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/seed-artifact-lifecycle/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI. Scenarios drive the compiled `givn` binary as a
  real subprocess.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| lint-command: run the static lint check | A duplicate title across two permanent spec files is a hard error (existing, sibling `spec-overlap-gates` capability) | CLI | Real `givn lint` subprocess |
| lint-command: inspect lint JSON | The json output reports duplicate-title findings machine-readably (existing, sibling `spec-overlap-gates` capability) | CLI | Real `givn lint --json` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill (existing) | CLI | Real `givn skills sync` subprocess |

No new `@e2e` scenario is added: running the static lint check is already
driven end-to-end by the two sibling `spec-overlap-gates` scenarios, and the
new scenarios are regular variants (advisory findings, guidance phrases). The
change adds no browser UI, so a browser driver would be the wrong mechanism.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | `givn::cli::lint::check_seed_status` and the status parser |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

`givn lint` gains one advisory finding family; its exit-code contract and
scan scopes are unchanged. Guidance assets gain the seed schema and the
stamping rules. No manifest, artifact graph, archive gate, or runtime service
changes.

## Risks

- The lint check must tolerate free-form seed prose. The parse rule reads only
  the first `Status:` line and never parses the rest of the file.
- The check is advisory by design; a project that ignores advisories keeps its
  seeds stale. That is the decided posture (ideation state is convention, not
  machine-enforced), and change 08 later adds severity routing.
