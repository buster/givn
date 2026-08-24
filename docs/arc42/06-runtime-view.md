# 6. Runtime View

This chapter records the runtime order implemented by the current checkout. It
distinguishes manifest-driven hooks from binary mandatory checks and separates
the retrieval surfaces instead of presenting them as one pipeline.

## 6.1 `givn init`

The sequence is:

1. Resolve the current working directory as the project root.
2. Create `givn/specs`, `givn/changes`, and `givn/archive`.
3. Write `givn/config.yaml` from the embedded, versioned scaffold unless the
   file already exists and `--force` is absent.
4. For a newly written config, read the agent-target selection. Empty input or
   EOF selects `.opencode`; recognized selections are `.opencode`, `.agents`,
   and `.claude`. The selection is appended as `skills.targets`.
5. Write `givn/commands.yaml` from the embedded project command template unless
   it already exists and `--force` is absent. The current template contains
   `./run-tests.sh`, `./run-tests.sh --e2e`, and coverage script/path entries.
6. Unless `--no-addons` is set, enable every registered addon. In the current
   registry this scaffolds Arc42, manages the Absolute Mode fenced block, may
   scaffold README content, and enables other registered extras.
7. Synchronize skills and opencode commands to the selected targets. The sync
   updates only givn-owned fenced blocks in `AGENTS.md` and leaves all other
   content unchanged.

The embedded default manifest has addon flags disabled, but initialized projects
normally receive enabled addon settings because the current `init` command
enables them. Existing Arc42 files are not overwritten by addon scaffolding.

## 6.2 `givn new <change-id>`

1. Validate the id as non-empty ASCII lowercase letters, digits, and hyphens,
   with no leading or trailing hyphen.
2. Resolve `givn/changes/<id>` through the shared path helper.
3. Reject an existing directory or an uninitialized `givn/` directory.
4. Validate any `--skip` ids against the effective manifest.
5. Create the change directory and write `proposal.md` from the cascade unless
   `proposal` is skipped.
6. If requested, write `.givn-skip` with one skipped artifact id per line.

The proposal template is written verbatim; no change-id substitution is
performed. A leading-hyphen argument supplied without `--` can be rejected by
`clap` before the shared validator, while `givn new -- -foo` reaches the
validator.

## 6.3 `givn instructions`

With an artifact id, the command loads the effective manifest, validates the
artifact, resolves its instruction through the project-overrides-first cascade,
and prints either the text or one JSON object containing `id`, `generates`,
`requires`, and `instruction`. A sole active change is auto-selected when no
`--change` is given; multiple active changes require an explicit id.

Without an artifact id, the command reports context-aware generic guidance from
the project state. It does not create lifecycle artifacts.

## 6.4 `givn lint`

`givn lint` parses feature files and runs no project tests. Its default scope is
all `.feature` files below `givn/`, excluding `givn/archive/`. `--change` scopes
to that change's specs directory; explicit paths are used as supplied.

For each parsed file it reports `@wip` and `PENDING`. Removed placeholders are
excluded from the WIP result. The default whole-tree scope additionally runs
the deterministic overlap scan:

- duplicate active scenario titles are errors and produce exit code `3`;
- shape matches, assertion subsets, and scenarios over 14 steps are warnings;
- parse errors produce exit code `1`;
- WIP/PENDING produces exit code `2`.

Precedence is parse error `1`, duplicate title `3`, WIP `2`, then clean `0`.
JSON output is a findings array; it is not test-runner output.

## 6.5 `givn status` and `givn graph`

Both commands load the effective manifest and derive state from the change
filesystem. `status` reports artifact paths, task checkbox progress, sign-off
markers, optionality, next required artifact, and skipped artifacts. `graph`
adds dependency edges, archive-gate flags, and configured post-hook names.

The state rules are:

- a generated path is required for an artifact to exist;
- `tasks.md` is complete only when all recognized `[ ]`, `[x]`, and `[X]`
  checkboxes are done;
- `design-review.md`, `review.md`, and `arc42.md` use their respective marker
  strings for completion;
- `.givn-skip` makes a named artifact `Skipped`;
- `apply.requires` identifies archive-gating artifacts.

## 6.6 `givn check`

The command resolves command and coverage settings from the effective manifest.
The current project obtains them from `givn/commands.yaml`, which is applied as
the later overlay after `givn/config.yaml`.

### Explicit artifact check

For `givn check tasks`, the binary runs the non-configurable mandatory checks:

1. For a change delta, reject non-removed scenarios that still have `@wip`.
2. Run `verify.command` with `GIVN_FEATURES` set to the active change specs.

For `givn check review`, it runs:

1. `verify.command`.
2. `verify-e2e` using `verify.e2e_command`.
3. The integrity hook for task hashes, promised files, and enabled Arc42 or
   coverage checks.
4. For a named change, the overlap disposition gate.
5. For a named change with retrieval available, the semantic report gate and
   the configured retrieval review path.

These mandatory checks are in `src/cli/check.rs` and cannot be removed by
removing a `post` entry. Built-in hook names also ignore `allow_failure` when
they are configured in a post list.

`verify-e2e` skips only when its sentinel is effective, there are no direct
`@e2e` scenarios in scope, and `tasks.md` contains no E2E intent. If the
sentinel is effective while E2E is present or planned, it fails. A non-sentinel
E2E command is invoked by the mandatory review path.

### No artifact argument

`givn check` without an artifact runs only configured `post` hooks for applicable
outputs. It does not invoke the explicit `tasks` or `review` mandatory-check
branch. With the embedded default manifest, standard post lists are empty.

### Semantic report refresh order

The blocking report entrypoint validates the change directory and specs path,
then refreshes the permanent index, and only after that parses the delta
scenarios. If delta parsing fails after a valid specs directory was found, this
ordering can already have rewritten the permanent index. The advisory
`givn spec review --change` path likewise refreshes the index before collecting
the delta. Change-aware `givn spec explore --change`, by contrast, parses and
collects the delta before refreshing the index.

## 6.7 `givn archive --change <id>`

The archive command executes the following stages:

1. Validate the change id and active directory.
2. Load the effective manifest and ensure every `apply.requires` artifact is
   complete or skipped.
3. Run configured post hooks for each `apply.requires` artifact.
4. Run retrieval review when `spec-retrieval` is compiled. The strict E5 gate is
   active only when `spec-similarity-gate` is compiled; otherwise this path is
   advisory. A feature-free build skips this retrieval review stage.
5. Back up `givn/specs/` and `README.md` as needed.
6. Parse each delta feature, reject WIP, and merge Added/Modified/Removed
   scenarios into `givn/specs/<capability>/<capability>.feature`.
7. Run `verify.command` against `givn/specs/`. A failure rolls permanent specs
   back from the backup.
8. Run post-merge E2E only when merged permanent specs contain direct `@e2e`
   scenarios. An empty or sentinel E2E command blocks in that case; a configured
   command must pass.
9. If coverage is enabled, run the project measurement/merge pipeline, validate
   Cobertura reports, update managed README blocks, and roll back specs,
   README, and coverage files on failure.
10. Copy the complete change directory to `givn/archive/<id>/`, then remove the
    active directory.
11. With retrieval enabled, refresh the permanent index as a best-effort step
    after archive success; this maintenance refresh reuses existing model
    artifacts and does not initiate a new model download. A refresh failure is
    reported as a warning.

Archive's post-merge checks are hard-coded in `src/cli/archive.rs`; they are not
the same thing as configurable artifact `post` hooks. Archive does not call the
explicit `check review` mandatory branch before these stages.

## 6.8 Retrieval Primitives

All retrieval commands require `spec-retrieval`; without it they return the
explicit unavailable error.

### 6.8.1 `givn spec index`

The index command scans current permanent `.feature` files under `givn/specs/`,
serializes complete scenarios, and preflights the E5 tokenizer. Matching
identity/content-hash entries are reused. Changed or new fitting scenarios are
embedded; over-cap scenarios become `e5_token_caps` metadata. The resulting
permanent-only index is written through a temporary file and rename.

Long-running phases report status on stderr: artifact acquisition announces
its start (with the cache directory) before any download, per-file download
progress during acquisition, acquisition completion, the pending embedding
count before batch embedding, and completion after. A warm, validated cache
produces no download announcements. Disabled evidence layers are never
announced as downloads.

### 6.8.2 `givn spec search`

Search reads the existing `.spec-index.json`; it does not refresh it. It embeds
the free-text query with the `query:` role, ranks vector entries by cosine, and
prints at most five results. This path is advisory and E5-only by default.

With `--evidence`, the same five displayed results are passed to the BGE and,
when compiled, NLI second stages exactly like Explore: the raw query text is
the query side of the pair, candidate scenario text is resolved from the
permanent specs, and per-candidate verdicts plus a layer-availability summary
are appended to the output. The layer is reported as unavailable (and never as
filtered) when it cannot score; the flag does not change the E5 ranking.

### 6.8.3 `givn spec explore`

The command accepts exactly one complete scenario from `--file` or `--stdin`.
It parses that input before any refresh. Without `--change`, it refreshes the
permanent index, embeds the query with the query role, ranks permanent entries,
and retains at most five results. A file query excludes only its exact normalized
`(file, title)` identity; other scenarios in the same file remain eligible.

With `--change <id>`, it validates the id, confirms the change/specs directory,
parses all delta features, and then refreshes the permanent index. Only active
Added and Modified scenarios are embedded in memory; Removed scenarios are not
candidates. Modified scenarios carry both delta-source and projected-target
identity for self-filtering. The persistent index remains permanent-only.

Explore output adds source scope and owning change. Optional BGE/NLI evidence
attaches to the five retained results and remains advisory.

### 6.8.4 Advisory `givn spec review --change`

The advisory review path first refreshes the permanent index, reads it, and then
collects delta scenarios. In E5-only mode it reports at most five ranked
advisory matches per active delta. If BGE or NLI is selected, the existing
surface-specific retrieval pool is passed independently to each layer. This
command does not create `semantic-review.md`.

The same-file rules depend on that mode. E5-only advisory review ranks index
entries without the semantic self-identity filter. The second-stage advisory
candidate builder excludes the projected target tuple from delta-to-permanent
neighbors for every delta operation and excludes same-file pairs in its
projected-permanent neighbor pass. The strict E5 gate and the blocking semantic
report do not use that projected same-file exclusion: they exclude only a
Modified delta's own projected target in delta-to-permanent comparisons. The
Duplicate Audit evaluates same-file pairs, and Explore excludes only the exact
file/title query identity.

## 6.9 Blocking Semantic Review Report

Only `givn check review --change <id>` invokes the dedicated semantic report
entrypoint. It compares active Added/Modified delta scenarios at E5 `0.9500` in
two scopes:

- `delta-to-permanent`: each active delta against permanent indexed entries;
- `delta-to-delta`: each distinct active pair in the same change, emitted once
  after canonical endpoint ordering.

Removed scenarios and permanent-permanent pairs are excluded. A Modified delta
excludes only its own projected `(target file, title)` permanent endpoint;
another scenario in the same target file remains eligible. The current report
entrypoint refreshes the index before parsing delta content, as described in
section 6.6.

### Candidate identity and evidence

The candidate id is the 16-character lowercase hexadecimal FNV-1a hash of:

```text
scope|left_scope|left_file|left_title|right_scope|right_file|right_title
```

It does not include the score, endpoint evidence hashes, or report schema. The
candidate endpoint evidence hash is a separate value containing the canonical
evidence JSON, operation, projected target, model/index, and evidence schema
values. Classification carry-forward requires the candidate id and both
endpoint evidence hashes to match, plus a non-empty rationale.

### Report writing and prior parsing

The writer emits fixed headers, field order, compact JSON values, `null` for
absent values, and a final newline. It atomically replaces the report before a
blocked result is returned. The current prior-state extraction is not a strict
wire-grammar parser: it scans recognized prefixed lines for candidate id,
classification, rationale, and evidence hashes. It accepts only
`DUPLIKAT`, `VARIANTE`, and `VALID-BOUNDARY` for carry-forward, ignores missing
fields, and disables carry-forward when duplicate candidate-id lines are found.
Unknown or reordered fields are not generally rejected by that extraction path.

The report's `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, and `UNCERTAIN`
classifications remain agent-owned. Filtered second-stage evidence is outside
the blocking classification set; unscored and token-cap findings remain
blocking. `semantic-review.md` is retained when the existing archive copy/remove
operation archives the change.

## 6.10 Surface-Specific BGE and NLI Pools

The current code does not use one universal top-five pool:

| Surface | BGE input | NLI input |
|---|---|---|
| Duplicate Audit | Union of up to five E5 neighbors for each endpoint, after deterministic exclusions; then E5-score ordering and budget. | Full E5 semantic candidate set remaining after deterministic findings; then independent E5-score ordering and budget. |
| Explore | The at-most-five ranked Explore results; then budget. | The same at-most-five results; independent scoring and budget. |
| E5-only advisory review | At most five index-ranked entries per active delta; no semantic self-identity filter. | No NLI branch. |
| Second-stage advisory review | The existing delta/permanent and projected retrieval pool, with top-five neighbor selection and its projected same-file filter; then budget. | The same advisory pool, independently scored and budgeted. |
| Blocking review | All generated semantic-report candidates at E5 `0.9500`; then budget. | The same report candidate list, independently budgeted. It is not a Duplicate Audit top-five union. |

BGE scores both directions and averages raw logits. NLI scores both directions,
reports directional probabilities, and computes a symmetric probability vector
from averaged logits. In combined mode the recommended view is the intersection
of participating recommended sets; neither model receives only the other's
survivors.

## 6.11 Token Budget and Cap States

E5 preflight uses the complete `passage:` or `query:` serialization with the
actual E5 tokenizer. BGE/NLI preflight uses complete pair encodings with each
layer's actual tokenizer, without truncation or padding. Pair counts include
pair special tokens. Exactly `512` fits and `513` is over.

- An over-cap E5 passage is retained as `E5_TOKEN_CAP` metadata, has no vector,
  and cannot participate in an E5 semantic pair.
- A selected over-cap BGE/NLI pair receives `BGE_TOKEN_CAP` or `NLI_TOKEN_CAP`
  with exact left/right/pair/limit counts, null model score/probabilities, and
  blocking unresolved semantics.
- A partial second-stage run scores fitting pairs and remains enabled.
- A layer with no fitting selected pair reports run-level unavailable with
  reason `token-cap` and retains per-candidate cap evidence.
- A candidate omitted by the positive pair budget is `BGE_UNSCORED` or
  `NLI_UNSCORED`; it is not tokenized by that layer and is not filtered.

Production uses real tokenizers and artifacts from the XDG cache or explicit
local paths. Regular and E2E tests use a deterministic local artifact-source
fixture and do not contact the live Hugging Face service or construct large
model artifacts.

### 6.12.1 Benchmark model export and comparison

The benchmark helpers are maintainer-facing shell interfaces, not product
commands. `export_nli_model.sh` resolves its flat export destination from
`XDG_CACHE_HOME` or `HOME`; `run_comparison.sh` passes that same default to the
Python comparison process. `compare.py` keeps its generated index under the
project's `test-corpus/indexes/` while E5, BGE, and NLI model defaults point to
the shared user cache. An explicit command argument may override a model path,
but no default points to the project or `/tmp`.

## 6.12 Addons, Upgrade, and Completion Commands

`givn addons list` reads enabled flags from `givn/config.yaml`. `enable arc42`
sets the flag, scaffolds missing fixed-path chapter files, and expands the
effective graph; `disable arc42` leaves existing docs in place. The same
commands manage other registered addon flags.

`givn skills sync` resolves the selected targets, writes generated skills and
opencode commands, refreshes the `givn-project` block, and applies/removes the
Absolute Mode block according to its addon flag. `givn upgrade` rewrites the
managed config scaffold, preserves uncommented user blocks, and auto-enables
newly known addons not represented in the old config.

`givn completions <shell>` writes no project files and prints a shell completion
script to stdout.

## 6.13 `givn upgrade` and declared migration bundles

The command selects a declared migration path from the active config marker to
the installed package version. It first verifies that the project is inside a
Git worktree and that
`git status --porcelain --untracked-files=all` is empty. A failed preflight
returns before changing any project file.

The command then refreshes `givn/config.yaml` and synchronizes generated skills,
commands, and Givn-owned `AGENTS.md` blocks. It stages an explicit allowlist,
never a complete directory or worktree, and creates one managed commit when
files changed. If the catalog returns one or more bundles, the command then
writes one aggregate `givn/changes/<aggregate-id>/` with Proposal, Design,
`migration.yaml`, and `.givn-skip` for `specs`; its Design records the managed
commit hash and all bundle phases in order. A second commit contains only that
aggregate plan. A gap with no declared route creates no migration change.

An existing active or archived migration plan is reported and never
overwritten. The generated plan contains no product feature delta and no
fabricated design-review, task, implementation, or review evidence. The
project LLM must continue the normal workflow and archive the maintenance plan
after all project-specific decisions and verification are complete.

At archive time, a maintenance change whose `.givn-skip` lists `specs` has no
delta to merge or compare semantically. Its global verify and E2E hooks run
against the permanent project feature path, and the change-specific semantic
gate is skipped. Tasks, review, and all other archive gates remain required.

## 6.14 Repository-Owned Acceptance Flow

The Cucumber test binary creates an isolated temporary project, invokes the
compiled CLI, captures exit status and stdout/stderr, and performs secondary
filesystem assertions. Review/archive success cases assert the CLI receipt
first. Failure cases snapshot permanent specs and verify rollback. No consumer
repository or external service participates.
