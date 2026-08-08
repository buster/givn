# Design: fix-archive-merge-docstrings

## Problem

`givn archive --change <id>` merges a change's delta `.feature` scenarios
into the permanent specs (`givn/specs/`). For `@givn.added` and
`@givn.modified` scenarios the merge reconstructs the scenario text from the
parsed AST instead of reusing the original text. That reconstruction renders
each step as `{keyword} {value}` and drops the step's attachments — docstrings
(`"""..."""` blocks) and data tables. Docstring-bearing steps in the
permanent spec lose their payload, so the merged scenario no longer behaves
as written.

Root cause is in `render_scenario_stripped` (`src/gherkin/merge.rs:209`):
the step-rendering loop emits only `step.keyword` and `step.value`. The
`gherkin` crate's `Step` carries the attachments as `docstring:
Option<String>` (already dedented by the crate's parser) and `table:
Option<Table>` (`rows: Vec<Vec<String>>`), and the renderer ignores both.

## Solution

Extend `render_scenario_stripped` so that after each step line it re-emits
the step's attachments:

- **Docstring** (from `step.docstring()`): one line of `    """` (4 spaces —
  same indent as the step) is NOT the convention; the repo uses 6 spaces for
  docstring delimiters and content under a 4-space step (see
  `givn/specs/lint-command/lint-command.feature`). Emit:

  ```
        """
        <content line>
        """
  ```

  Each content line is re-indented to 6 spaces (the crate stores content
  already dedented, so reconstruction re-indents it). Empty content lines are
  emitted as empty lines. The `"""` delimiter is always used (the crate does
  not preserve the original delimiter style or any content-type annotation);
  verification treats `"""` docstrings as the canonical form.

- **Data table** (from `step.table()`): emit each row as `| c1 | c2 | ... |`
  at 6-space indentation, one row per line. Parser-side cell trimming makes
  this round-trip losslessly for plain cells. (Cells containing unescaped
  `|`/`\` are not claimed preserved; no spec in this repo uses them and this
  change does not extend scope to escapement.)
- **Combined attachment order**: on the rare step carrying both (the crate
  allows `docstring()? table()?`), render the docstring block first, then the
  table rows, each block newline-terminated.
- **Empty docstring**: when `step.docstring()` is `Some("")` (an explicitly
  empty `"""\n"""`), still emit the two `"""` delimiter lines — never drop a
  present-but-empty docstring.

No other part of the merge changes: text-level scenario splitting
(`parse_scenarios_from_text`) and header extraction already preserve
docstrings for scenarios that are *not* replaced; only the AST-reconstruction
path was dropping them. `@givn.*` tag stripping is unchanged.

## Files affected

- `src/gherkin/merge.rs` — extend `render_scenario_stripped` to render step
  docstrings and data tables. Add unit tests for docstring + table
  preservation through `apply_deltas`.
- `tests/steps/preserve_docstrings_on_archive_steps.rs` (new) — step
  definitions for this capability, one file per capability (project
  convention).
- `tests/steps/mod.rs` — register the new module.

## Step definitions

New fixture-builder and assertion steps (in the file above):

- `a permanent spec for capability "{cap}" contains a scenario named
  "{title}"` — writes `givn/specs/<cap>/<cap>.feature` with a `Feature:` and
  the named scenario (baseline for `@givn.modified`).
- `a completed change "{id}" whose delta (modifies|adds) scenario "{title}"
  within capability "{cap}" and gives a step the docstring:` (docstring
  argument) — wins the archive gate: writes proposal/design/design-review
  (`DESIGN-REVIEW: PASS`)/tasks (`- [x] ...`)/review (`REVIEW: PASS`)/arc42
  (`STATUS: DONE`), writes the delta feature at
  `givn/changes/<id>/specs/<cap>/<cap>.feature` tagged `@givn.delta @<cap>`
  with the scenario carrying a `Given` + `And <step>:` + the docstring, and
  patches `givn/commands.yaml` (verify `true`, e2e empty, coverage addon with
  dummy Cobertura XMLs) so the post-merge verify/coverage gates pass in the
  temp project.

**Fixture commands.yaml (mandatory, identical for every fixture step):** write
`givn/commands.yaml` by hand with all commands set so every archive gate passes
in the temp project (no Rust runner exists there):

```yaml
verify:
  command: "true"
  e2e_command: ""
coverage:
  non_e2e_command: "true"
  e2e_command: "true"
  non_e2e_path: "coverage/non-e2e-cobertura.xml"
  e2e_path: "coverage/e2e-cobertura.xml"
addons:
  arc42: true
  absolute_mode: true
  readme: true
  coverage: true
  domain_modeling: true
  dev_principles: true
  grill_me: true
  grill_with_docs: true
```

The coverage commands MUST be `true` (not the `givn missing-coverage`
sentinel that `givn init` writes) and both dummy Cobertura XMLs must be
written to `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`
with distinct content (the coverage gate requires the two reports to differ).

**Fixture deltas must NOT carry `@e2e`:** the merged permanent spec of the
fixture's capability must contain no `@e2e` scenario so that, with
`e2e_command: ""`, the archive's post-merge verify-e2e gate takes its silent
empty-pass path (it only hard-fails when a merged spec gains `@e2e`). This
isolates RED strictly to the docstring/table content assertion.
- `a completed change "{id}" whose delta (modifies|adds) scenario "{title}"
  within capability "{cap}" and gives a step a data table containing the text
  "{cell}"` — same setup but the step carries a data table whose row contains
  the given cell text.
- `a givn project with a completed change "{id}" whose delta modifies
  scenario "{title}" within capability "{cap}" and gives a step the
  docstring:` (e2e; `world.env = make_init_root()` + writes the permanent
  spec baseline + everything above).
- `the permanent spec file for "{cap}" contains the text "{needle}"` (Then) —
  reads `givn/specs/<cap>/<cap>.feature` and asserts it contains `needle`.

Reused existing steps: `a project root with givn/ initialised`, `the user
runs \`...\``, `the user invokes \`...\``, `the command exits 0`, `the
command succeeds`.

All new regex step definitions MUST be fully anchored (`^...$`) to avoid
cucumber-rs 0.23 ambiguous-match errors against the existing step library
(cucumber-rs 0.23 auto-ambiguity on multi-match), and their literal text is
disjoint from every existing step.

## Unit tests

In addition to the .feature scenarios, `src/gherkin/merge.rs` gets unit tests
driving `apply_deltas` directly (fast, no subprocess), covering:
- modified scenario keeps a step docstring (non-empty);
- added scenario keeps a step docstring;
- empty docstring (`Some("")`) still emits `"""` delimiters;
- step data table is preserved;
- step carrying both docstring and data table renders docstring then table.

## Verify command

```
cargo test --test features_runner
```

Both runners execute the `.feature` files through cucumber-rs (the
`features_runner` binary, `tests/features_runner.rs`), which loads paths from
`GIVN_FEATURES` (default `givn/specs/`) — so it runs `givn/specs/**` and, with
`GIVN_FEATURES` pointed at the change, the change's `specs/**`.

### Single-scenario command

```
GIVN_FEATURES=givn/changes/fix-archive-merge-docstrings/specs \
  cargo test --test features_runner -- --name "<scenario title>"
```

(`--name <regex>` is cucumber-rs 0.23's native scenario filter.)

E2E command (filtered to the `@e2e` tag):

```
GIVN_FEATURES=givn/changes/fix-archive-merge-docstrings/specs \
  cargo test --test features_runner -- --tags @e2e
```

## Strict mode

`cucumber-rs` is already strict via `.fail_on_skipped()` in
`tests/features_runner.rs:20` — an undefined or pending step fails the run.
The existing suite relies on this; no extra configuration.

## Local runnability & digital twins

**Interface type: CLI.** `givn` is the system; its real interface is a
subprocess invocation of the compiled binary. No browser, server, database,
queue, or third-party service is involved. There are no external dependencies
and therefore no digital twins and no network: the per-scenario temp
directory (`common::make_init_root`) plus the compiled binary is the complete
local test environment. Local run command for a human:

```
cargo build && cargo test --test features_runner
```

No anticipated interface obstacles: the scenarios drive `givn archive` via
the same subprocess seam the suite already uses.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Archive a change; the merged permanent spec keeps step docstrings/tables | Consumer archives a modified scenario and the permanent spec keeps the docstring | CLI (givn binary) | `common::run_givn` subprocess running `givn archive --change real-modify` in a temp project; assertion reads the on-disk `givn/specs/<cap>/<cap>.feature` |

The e2e scenario's `Then` asserts PRIMARILY on what the interface produced
(the merged permanent spec file on disk after the real archive run). This is
the interface's output; there is no repository/DB secondary check.

## Version freshness

- `gherkin` 0.16.0 — present in `Cargo.lock` (registry src at
  `gherkin-0.16.0`). `Step.docstring`/`Step.table` fields confirmed in
  `gherkin-0.16.0/src/lib.rs`.
- `cucumber` 0.23.0 — present in `Cargo.lock`; `--name`/`--tags` CLI filters
  confirmed in `cucumber-0.23.0/src/cli.rs`.
