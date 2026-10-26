# Givn workflow details

Companion to the [workflow overview](workflow.md). This document lists every
phase, artifact, gate, and failure condition. The lifecycle diagram is
[`workflow.svg`](workflow.svg).

## 1. Ordering and state

The default artifact chain is defined in the embedded manifest
(`assets/default-manifest.yaml`, overridable per project via
`givn/config.yaml`):

```
proposal → specs → design → design-review → tasks → review
apply.requires = [tasks, review]        # these gate givn archive
```

With the arc42 addon enabled, `arc42-docs` sits between `design` and
`design-review`, `design-review` requires it, and `apply.requires` gains
`arc42-docs`.

- Order is a topological sort over `requires` edges (`src/state.rs`).
- There is no state file. State is computed from which files exist, which
  checkboxes are ticked, and which sign-off markers are present.
- The chain is optionally preceded by ideation: domain event storming that
  shapes an idea into use cases before a change exists. Ideation produces no
  manifest artifact.
- Between phases the order is advisory: `givn instructions` and
  `givn status` recommend the next command. Hard enforcement happens at
  `givn check tasks`, `givn check review`, and `givn archive`.

## 2. Phase by phase

| # | Phase | Invoked via | Creates under `givn/changes/<id>/` | Done when | Gate |
|---|---|---|---|---|---|
| 0 | scaffold | `givn new <id> [--skip a,b]` | change directory + `proposal.md`; `.givn-skip` for skipped artifacts | — | none |
| 1 | propose | `/givn-propose` | `proposal.md` — what and why, domain language, release note, capability placement, open questions | file exists | none |
| 2 | spec | `/givn-spec` | `specs/<usecase-slug>/<capability>.feature` tagged `@givn.delta @<capability>`; `@givn.added/modified/removed`; `@wip` while unimplemented | any feature file exists | `givn lint` (advisory until review) |
| 3 | design | `/givn-design` | `design.md` — runner, strict mode, single-scenario command, step files, decisions ledger, interaction matrix, visual contract, local runnability | file exists | none |
| 3a | arc42 (addon) | `/givn-design` | `arc42.md` plus affected `docs/arc42/` chapters | `STATUS: DONE` | archive requires it when enabled |
| 4 | design-review | `/givn-design-review` | `design-review.md` ending in `DESIGN-REVIEW: PASS` | marker is the final non-empty line | archive refuses without it |
| 5 | tasks | `/givn-tasks` | `tasks.md` — setup task, behavior batches, evidence and commit fields | all checkboxes `[x]` | `givn check tasks` |
| 6 | implement | `/givn-implement` | production code, ticked `tasks.md`, batch commits | every scenario has `TEST-FIRST:` evidence and a `COMMIT:` revision | see §4 |
| 7 | review | `/givn-review` | `review.md` ending in `REVIEW: PASS` | marker is the final non-empty line | `givn check review --change` |
| 8 | archive | `givn archive --change <id>` | moves to `givn/archive/<id>/`, merges deltas into `givn/specs/` | proven verification receipt | `givn archive` |

`--skip` marks artifacts as intentionally absent. A skipped artifact appears
as `⊘ skipped` in `givn status` and satisfies `apply.requires`.

## 3. Optional upstream: ideation

Before a change exists, `/givn-ideation` storms the problem domain: domain
events in past tense, commands, actors, aggregates, and policies, until the
storm slices into use cases.

| Phase | Produces |
|---|---|
| Scope | `scope.md`; personas as domain experts |
| Event storming | `eventstorming.md`, `domain-terms.md`, `questions.md` |
| Use-case definition | `use-case-definitions.md` with affected stakeholder types |
| Use-case detailing | `use-cases/<usecase-slug>.md` with rules, examples, and contract |
| Handoff | published topic document and change seeds with `Status: ready` |

Properties:

- State lives under `givn/ideation/<topic-id>/`; `session.md` carries the
  resume point.
- One question at a time. Unknowns are parked in `questions.md`.
- Personas review artifacts that affect their stakeholder type.
- Handoff never starts a change. A seed becomes a change through
  `/givn-propose`.

Enforcement: none. Ideation produces no manifest artifact and no gate.

## 4. The implement loop

Work in test-first behavior batches. A batch is a coherent group of related
scenarios, not a single scenario. One batch may satisfy several scenarios.

```
TEST-FIRST:     Bind every Given/When/Then phrase. Given establishes real
                preconditions, When invokes the intended boundary, Then
                asserts the observable result. Add or update tests and step
                glue. Run against the current system. Capture the test
                revision and results. A failing test must fail on its
                assertion, not on discovery, registration, binding, import,
                fixture, or environment setup. A passing test is allowed when
                it protects a distinct requirement; keep it and record the
                evidence. Do not change production code.
IMPLEMENTATION: Implement the batch or wire the real implementation. List
                every production file created or modified. Run the relevant
                scenarios; they must exit zero. Capture the output.
REFACTOR:       Clean up without changing behaviour. Re-run the relevant
                scenarios; they must still exit zero. Capture the output.
COMMIT:         One batch commit. The message carries the release note as its
                subject and names the batch in the body. Scenarios record
                `COMMIT: <hash> -- Batch: <batch-name>`.
```

- The setup task comes first: install the runner, configure strict mode, set
  `verify.command`, create one step file per capability, and prove that an
  undefined or pending step fails the run. Remove the probe afterwards and
  return to a green baseline.
- Tick each scenario box only when its batch evidence is complete. An
  unchecked box means the scenario is not done.
- Non-e2e scenarios first, `@e2e` last. The e2e setup task brings up the
  local stack, configures `verify.e2e_command`, and proves that the e2e filter
  is narrower than `verify.command`.
- `@e2e` tags are immutable. Removing one to bypass the e2e gate is detected
  by the verify-e2e hook and by lint.
- Deviating from `design.md` requires stopping, updating `design.md`, and
  re-running design-review.

`TEST-FIRST` evidence and commit identity are machine-checked at
`givn check tasks`. Empty step bodies and downgraded e2e assertions are
checked by the review's implementation evidence check.

## 5. Checks and gates

### Advisory (never block)

| Check | Command | Reports |
|---|---|---|
| duplicates / coverage / tree | `givn spec …` | deterministic reports over the permanent corpus |
| lint warnings | `givn lint` | shape matches, subsets, long scenarios, seed status, persona references |
| README findings | `givn readme check` | open placeholders, remnants, damaged managed blocks |

### Blocking

| Gate | Trigger | Checks |
|---|---|---|
| tasks | `givn check tasks --change <id>` | non-removed `@wip`; every checked scenario has a real reachable commit whose message names the scenario or its batch; clean committed tree; the delta suite exits zero and writes a fail-closed result; writes the change verification receipt |
| review | `givn check review --change <id>` | integrity (revision identifiers, promised step files, visual review, persona disposition, deferrals, arc42 conformance, coverage config, README impact); run declaration; change-scoped lint; overlap dispositions; retirement disclosure; capability placement consistency; current change receipt; outline gate when enabled |
| archive | `givn archive --change <id>` | archive state and SCM preconditions; clean project tree; current change receipt; `apply.requires` artifacts complete; signed design-review; re-run of the static review gates (integrity, overlap, retirement, placement, outline); delta step definitions bound in the permanent test tree; archive hooks; run declaration; README completeness; use-case/delta prevalidation; e2e runner configured; merge validity; both verification scopes; coverage; finalization guards |
| lint | `givn lint` | exit 1 parse error, 2 `@wip`, 3 duplicate title, empty feature, invalid use-case document, or interaction mismatch |
| commit | `givn commit`, commit-msg hook | conventional commit message; attribution trailer |

`givn check review` without `--change` runs only the integrity hook and the
run-declaration check. The change-scoped checks require a change id.

Sign-off markers (`REVIEW: PASS`, `DESIGN-REVIEW: PASS`, `STATUS: DONE`)
count only when they are the last non-empty line outside fenced code blocks
and HTML comments. A template example does not satisfy the gate.

### Verification scope timing

- `givn check tasks` runs the configured suite once against the change's delta
  features and writes the change receipt.
- `givn check review` does not re-run the suite; it requires the current
  receipt.
- `givn archive` runs the regular scope and the e2e scope once against the
  merged permanent corpus.
- The e2e scope runs after review sign-off. A red e2e suite fails the archive,
  not the review.

## 6. Status

`givn status [--change <id>] [--json]` derives everything from the filesystem:

- stage: `empty` → `<artifact>-done` → `in-progress` → `ready-to-archive`;
- artifact checklist in topological order: `✓` done, `~` partial, `·` pending,
  `⊘` skipped; `(gates archive)` marks `apply.requires` members;
- task progress `n/m` and the first pending task lines;
- the next required command, or “Ready to archive”.

## 7. Archiving step by step

`givn archive --change <id>` is forward-only and SCM-backed:

1. Preconditions: a Git worktree or Fossil checkout whose root is the project
   root; a committed checkout (only declared evidence paths may be dirty);
   the change exists and is active; all `apply.requires` artifacts are
   complete; `design-review` is signed unless skipped; `tasks.md` is complete;
   `REVIEW: PASS` is present.
2. Structural gates run before any mutation: the static review gates,
   configured archive hooks, the run-declaration gate, the README
   completeness gate, use-case and delta prevalidation, the step-definition
   binding check, and the e2e runner configuration check. No test suite runs
   in this stage.
3. The accepted use-case narratives and delta features merge into the
   permanent corpus; `@givn.*` tags are stripped and `@e2e` is preserved.
   Title collisions or missing modified/removed titles abort.
4. The merged state is published as the archive candidate commit with a
   pending receipt, and the change directory moves to `givn/archive/<id>/`.
5. Each verification scope runs once against that commit and must write a
   fail-closed runner result: counts reconcile, at least one scenario, zero
   failures, zero skips. The e2e scope is `not_applicable` when the merged
   corpus has no `@e2e` scenario.
6. The coverage gate (when enabled) measures, validates, merges, and updates
   the managed README blocks.
7. Finalization guards `HEAD == tested commit` and that dirty paths are
   declared evidence; then the proven receipt is committed as the
   evidence-only finalization commit.

A failure after the candidate commit leaves the unproven candidate in place.
Repair the committed state and rerun archive to resume without repeating the
merge. Resume skips the structural gates — including the static review gates
— because they ran before the candidate was created. An already proven archive
is refused.

## 8. Reverse engineering

For codebases without baseline specs: `reverse-engineer → characterize →
archive`.

- Scaffold with `givn new <id> --skip proposal,design,design-review`.
- Black-box boundary and suspected bugs live in the Feature description;
  seam strategy goes into `tasks.md`.
- Characterization corrects the spec, never the code. Suspected bugs are
  marked `TODO: REVERSE-ENGINEERING NOTE:`.
- After the baseline exists, normal changes proceed against it.

## 9. What the gates do not check

- Design-review was run in a genuinely fresh context; the marker is a
  self-report.
- A bound step body asserts something; the strict-mode proof only covers
  undefined and pending steps.
- A commit touches production code; the task gate checks revision identity
  and reachability, not the diff.
- A screenshot shows the right thing; visual evidence presence is checked,
  the Persona judgment is a review marker.
- The planning artifacts are semantically complete; status treats an existing
  file as done.
