# Givn workflow in detail

Companion to the README quickstart. This document lists every step in order,
what it creates, what is checked, and how givn enforces process quality —
Visual renderings of this
process: [`workflow.svg`](workflow.svg) (GitHub-safe) and
[`workflow.html`](workflow.html) (self-contained HTML).

## 1. Ordering and state

The default artifact chain is defined in the embedded manifest
(`assets/default-manifest.yaml`, overridable per project via `givn/config.yaml`):

```
proposal → specs → design → design-review → tasks → review
apply.requires = [tasks, review]        # these gate givn archive
verify.command = <sentinel until configured>
```


- Order is a topological sort over `requires` edges (`src/state.rs`).
- There is no state file. State is computed entirely from which files exist.
- Between phases the order is advisory only: `givn instructions` inspects the
  filesystem and recommends the next command. Hard enforcement happens at
  exactly three chokepoints: `givn check tasks`, `givn check review`, and
  `givn archive`.
- The chain is optionally preceded by an ideation flow (§3): a domain-driven
  event storming that shapes a vague idea into use cases before any change
  exists. It lives outside the manifest and produces no change artifact.

## 2. Phase-by-phase

| # | Phase | Invoked via | Creates under `givn/changes/<id>/` | Done when | Notes |
|---|---|---|---|---|---|
| 0 | scaffold | `givn new <id> [--skip a,b,c]` | change directory + `proposal.md` stub | — | `--skip` marks artifacts as intentionally absent (reverse engineering) |
| 1 | propose | skill `/givn-propose` | `proposal.md` — WHAT and WHY, domain language, no implementation detail | file exists | |
| 2 | spec | skill `/givn-spec` | `specs/**/*.feature` tagged `@givn.delta @<capability>`; scenarios `@givn.added/modified/removed`; `@wip` while unimplemented; mandatory User Interaction Inventory; ≥1 `@e2e` scenario per capability | any feature file exists | merge errors (duplicate title, modified/removed title not found) surface at archive time |
| 3 | design | skill `/givn-design` | `design.md` — runner command, strict-mode config, single-scenario run command, Step Definitions table (one file per capability), E2E infrastructure, Interaction Coverage Matrix, version freshness rule | file exists | |
| 4 | design-review | skill `/givn-design-review` | `design-review.md` ending in `DESIGN-REVIEW: PASS` or `FAIL` | marker present in text | without the marker the artifact counts as partial and blocks archiving |
| 5 | tasks | skill `/givn-tasks` | `tasks.md` — setup task first, then one RED/GREEN/REFACTOR/COMMIT section per scenario; non-`@e2e` first, `@e2e` last | all checkboxes `[x]` | leftover `@wip` blocks `givn check tasks` |
| 6 | implement | skill `/givn-implement` (+ `/givn-steps`) | production code + ticked `tasks.md` + one atomic commit per scenario | see §4 | no artifact of its own |
| 7 | review | skill `/givn-review` | `review.md` ending in `REVIEW: PASS` | marker present | coverage gap classification: dead code / missing test / legitimately hard to test — no percentage threshold |
| 8 | archive | `givn archive --change <id>` (+ skill `/givn-archive`) | moves the change to `givn/archive/<id>/`, merges deltas into `givn/specs/` | see §7 | |

## 3. Optional upstream: ideation

Before any change exists, `/givn-ideation` runs a domain-driven event
storming flow over the problem domain: domain events in past tense first,
then the commands, actors, and policies behind them, until the storm slices
into use cases — one per future givn change.

It is the artifact-producing alternative to `/givn-explore`: explore answers
"do we want this, and which option?" without artifacts; ideation answers
"how does the solution look, domain-wise?" and ends with change-ready use
cases.

| Phase | Storm role | Produces |
|---|---|---|
| Frame | storm scoping — problem, stakeholders, research of the existing project | `frame.md`; personas (`personas/<type>--<segment>.md`) as domain experts |
| event storming | domain events in past tense, reverse narrative ("which events must happen for this?"), commands/actors/policies, hotspots parked | `eventstorming.md`, `domain-terms.md`, `questions.md` |
| use case definition | slice event sequences that culminate in value; goal levels, bounded contexts; personas rank value | `use-case-definitions.md` with affected stakeholder types |
| use case elaboration | per use case: rules, concrete examples, contract — Gherkin-ready | `use-cases/<uc-id>.md`, persona reviews |
| handoff | publish and promote | `docs/ideation/<topic-id>.md`, change seeds under `changes/` (each with `Status: ready`), term merges, persona promotion into `givn/personas/` |

Properties:

- **Multi-session.** State lives under `givn/ideation/<topic-id>/`;
  `session.md` carries the resume point, so a topic survives across sessions.
- **One question at a time.** Unknowns are parked in `questions.md` and
  worked around, never guessed.
- **Persona binding.** Every artifact that affects a stakeholder type is
  reviewed by that type's personas before the phase closes.
- **Handoff never starts a change.** Each change seed becomes a change via
  `/givn-propose` when the user is ready.

Enforcement: none. Ideation produces no manifest artifact — no `givn status`
stage, no gates, no `check` involvement. It is visible only through its state
directory and the published topic document.

Use-case model: the canonical definition, the eight scoping rules, the
authoring requirements, and the review rejection criteria live in
[`use-case-model.md`](use-case-model.md). The ideation skill carries the
authoring rules; the review instruction carries the criteria.

Term record: crystallizing terms are kept in `givn/ideation/<topic-id>/domain-terms.md`.
Each entry records the term, its status (`candidate`, `confirmed`, or
`rejected`), a one-sentence definition, a contrast, its first use, and the
recorded collision check; a `confirmed` term has passed the term introduction
test and a recorded search-before-coin check. The canonical schema lives in
the `givn-ideation` skill and the test in the `givn-domain-modeling` skill;
this document does not duplicate either.

## 4. The implement loop

Per scenario, normative rules:

```
RED      Remove @wip from THIS scenario only. Run the runner targeting this
         scenario only (name, line, or tag). Paste the output. MUST exit
         non-zero. Exit 0 = hard stop (runner not strict or step body empty).
GREEN    Replace stubs with real assertions, write minimum production code,
         list every production file touched. MUST exit zero.
REFACTOR Clean up with no behaviour change. Re-run this scenario. MUST stay zero.
COMMIT   One revision for RED+GREEN+REFACTOR via
         `givn commit -m "feat(<capability>): <release note>"` — the revision
         message carries the release note as its message subject, with the
         scenario title in the message body (works on Git and Fossil; raw Git
         commits are stamped by the hook, raw Fossil check-ins are not).
         Record the identifier in tasks.md immediately.
```

- Setup task comes first: proof that the runner is strict (an empty step must
  fail) before any scenario starts.
- Check off each box immediately after finishing the scenario. An unchecked
  box means the scenario is not done. Mass-checking boxes at the end is
  prohibited.
- `@e2e` tags are immutable — removing them to bypass the e2e gate is a
  procedure violation. The `@e2e` loop runs only after all non-e2e scenarios
  are GREEN; commit prefix `test(e2e):`.
- Deviating from `design.md` requires stopping, updating `design.md`, and
  re-running design-review (the sign-off un-gates).
- A scenario without its own atomic commit is not done.

## 5. Checks and gates

### Advisory (never block)

| Check | Command | What it reports |
|---|---|---|
| duplicates / coverage / tree | `givn spec …` | deterministic duplicate and coverage reports over the permanent corpus; no model or network dependency |
| lint warnings | `givn lint [--change <id>]` | `shape_match` overlaps, subsets, scenarios longer than 14 steps |
| seed status | `givn lint` (default scope) | missing or unrecognised seed status, a `proposed by` change that does not exist, a `proposed by` change that is already archived; advisory, exit code unchanged |
| missing index notice | `givn check review` | hints to run `givn spec index` first |

### Blocking

| Gate | Trigger | Checks |
|---|---|---|
| lint hook | built-in hook (`verify`, `lint`, …) | parse errors (severity 1) outrank duplicate titles (3), which outrank `@wip`/PENDING (2); exit code = highest severity. Built-in hooks can never be downgraded via `allow_failure`. |
| verify sentinel | hooks running `verify.command` | fails until a real Gherkin runner is configured |
| verify-e2e anti-bypass | hooks | `@e2e` scenarios exist but `verify.e2e_command` is still the sentinel ⇒ fail; cross-checks tasks.md intent so tag removal cannot bypass it |
| integrity | part of `givn check review` + `integrity` hook | every `[x] COMMIT:` line records a real revision identifier (no placeholders, no duplicates, at least one required); Step Definitions table entries exist and are non-empty; arc42 chapters marked "Affected: Yes" differ from template; coverage config complete if addon enabled |
| `@wip` gate | `givn check tasks` | any non-removed `@wip` scenario in delta features ⇒ block |
| overlap dispositions | `givn check review` | every `shape_match` finding involving delta files has a disposition row in `review.md`; missing `review.md` ⇒ block |
| conventional commits | `givn commit`, or the local Git `commit-msg` hook | message matches the Conventional Commits regex (this repository's own setup); the hook exists only on Git |

Chokepoints:

```
givn check tasks    → @wip gate + verify hooks
givn check review   → verify → verify-e2e → integrity → overlap dispositions
givn archive        → artifact existence/completeness + PASS markers + all required hooks
                      + fresh similarity gate + delta merge + post-merge tests + coverage
```

## 6. Status

`givn status [--change <id>] [--json]` derives everything from the filesystem:

- stage label (`empty` → `<artifact>-done` → `in-progress` → `ready-to-archive`)
- artifact checklist in topological order: `✓` done, `~` partial, `·` pending,
  `⊘` skipped; `(gates archive)` tags on `apply.requires` members
- task progress `n/m` with the first pending task lines for partial `tasks.md`
- next required command, or "Ready to archive"

## 7. Archiving step by step

`givn archive --change <id>` (forward-only, SCM-backed, two revisions):

1. Preconditions: the selected backend (a Git worktree or a Fossil checkout)
   whose root is the project root; a committed checkout (only declared
   evidence paths may be dirty); the change exists and is active; all
   `apply.requires` artifacts exist and are complete
   (`tasks.md` fully checked, `REVIEW: PASS` marker present).
2. Structural gates run before any mutation: configured archive hooks, the
   run-declaration gate, the README completeness gate, use-case/delta
   prevalidation, and the E2E-runner configuration check.
3. The accepted use-case narratives and delta features merge into the
   permanent specs; `@givn.*` tags are stripped, `@e2e` preserved. Title
   collisions or missing modified/removed titles abort.
4. The merged state is published as the **archive candidate commit** with a
   pending `verification.json`, and the change directory moves to
   `givn/archive/<id>/`.
5. Each verification scope runs once against that commit and must write a
   fail-closed runner result (`GIVN_RESULT_FILE`): reconciling counts, at least
   one scenario, zero failures, zero skips. The E2E scope is `not_applicable`
   when the merged corpus has no `@e2e` scenario.
6. Coverage gate (if enabled): measure → validate Cobertura → merge → update
   managed README blocks. A failure leaves the candidate unproven; nothing is
   rolled back.
7. Finalization guards `HEAD == tested commit` and that every dirty path is a
   declared evidence path; then the proven receipt is written and committed as
   the **evidence-only finalization commit**.

A failure after the candidate commit leaves the unproven candidate in place;
repair the committed state and rerun `givn archive --change <id>` to resume
without repeating the merge. An already proven archive is refused.

Afterwards, before starting the next change, rebuild and resync the tooling
(see [`reference.md`](reference.md#repository-development-scripts)).

## 8. Reverse engineering (peer flow)

For codebases without baseline specs, the peer flow is
`reverse-engineer → characterize → archive`:

- Scaffolded with `givn new <id> --skip proposal,design,design-review`
  (skipped artifacts show as `⊘ skipped`).
- No proposal/design layer: black-box boundary and suspected bugs live in the
  Feature description block; seam strategy goes into tasks.md.
- Characterization corrects the spec, never the code; suspected bugs are
  marked as `TODO: REVERSE-ENGINEERING NOTE:` comments. Reverse review does
  not delete dead code.
- After the baseline exists, normal changes proceed through the standard
  workflow against it. During this loop `givn check <artifact>` is not used;
  `check` applies only to the review epilogue.
