# Proposal: ise-rebuild-decision-and-working-tree-ship

## Release note

The ise guidance records one Ise rebuild decision, states whether rebuilt code ships or is discarded, and lets a Single-Component Rebuild ship from the current project working tree without a spec copy.

## Use-Case Context

- Use-case ID: `create-change`, `prepare-an-ise-test`, `declare-the-specification`; the `instruction-integrity` capability is owned by the `support` fragment that the guidance use cases include.
- Ideation topic: `ise-rebuildability`.
- Seed path: none. The change revises the topic's own vocabulary and adds the working-tree practice; it does not originate from a change seed.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

## Review Context

- Review kind: independent
- Reason: the change rewrites the served guidance that decides its own review; the guidance text and vocabulary are direct work committed before this change's delta, and the delta carries the new regression scenarios. A self-review cannot test that self-reference; the independent reviewer context is recorded in `design-review.md` and reused by `review.md`.

## Problem / Opportunity

The ise add-on has one documented rebuild: prepare an external spec copy from a recorded revision, rebuild inside it, and keep the result out of the project. Every ordinary change that replaces one Component's Source code in place has no ise guidance. Reviewers of such a change ask for a spec copy, a candidate revision, or an early archive merge that the practice does not need.

The vocabulary audit (`givn/ideation/ise-rebuildability/vocabulary-review.md`) and the Passuff trials record four costs:

| Observation | Evidence | Cost to the reader |
|---|---|---|
| One concept appears under more than one label: `trial`, `spec copy`, `clean-room trial`, `Written path`; `record`, `receipt`, `trial record`; eight placement nouns across four surfaces. | `vocabulary-review.md`, Surface consistency | A reader maps labels before reading output. |
| `Change strategy` mixes the decision, its scope, and what happens to the rebuilt code; `Constant Ise` is a borrowed proper noun; `Dry-Run Ise` names a rebuild while the CLI `--dry-run` writes nothing. | `vocabulary-review.md` F1-F16; `docs/glossary.md`; `assets/instructions/ise.md` | A reader cannot tell whether a change rebuilds, what it rebuilds, or whether the result ships. |
| `checkout` means both the current files and the recorded state a copy is read from; `declaration`, `role`, `path`, and `file` form four abstractions over one YAML file and its matches. | `docs/glossary.md`; `docs/reference.md:366-531` | An agent asked to rebuild cannot tell which files are inputs and which are evidence. |
| `Neighbor record` is defined three ways: files changed outside the replacement, a consolidation, and a file the rebuild merely read; the clean-room scan is also recorded in it. | `assets/instructions/ise.md:49-50,63-68,108`; `docs/reference.md:494` | The review records rows that are not effects of the rebuild. |

The site repeats the split: it calls the one-Component pattern `Constant Ise`, offers `Incremental / Single module`, `Dry run`, and `Full replacement` as three peer cards, and says `spec bug` for every observed difference.

## Proposed Solution

### One recorded decision

An `ise`-enabled change records one **Ise rebuild decision** in the design's existing `Decides D<n>` entry. The decision is one of:

| Decision | Meaning |
|---|---|
| `No rebuild` | The change keeps working with the existing Source code. No `Rebuilt code` line is recorded. |
| `Single-Component Rebuild` | One Component's Specification is updated, its Source code and coupled unit tests are replaced, and its Component Interface keeps its consumers. |
| `Whole-system rebuild` | Every declared `source:` entry's Source code is replaced behind the system's external interfaces; persisted data and external contracts stay. |

A rebuilding change adds a plain **`Rebuilt code`** line: `discard` or `ship`. `ship` adopts the verified replacement into the change. `discard` does not adopt it. The entry still names the affected consumers, the persisted state, and the rebuild boundary, and adds no mandatory design field.

### The working-tree ship sequence

The `ise` instruction states the sequence a `Single-Component Rebuild` with `Rebuilt code: ship` follows in the current project working tree:

1. The updated Specification is written as the active change's delta; unchanged permanent rules stay where they are.
2. The previous Component remains in revision history; the change records the delta while that implementation is still recorded.
3. The Component's implementation and coupled unit tests are removed from the working tree. The Component Interface, other Components, and the acceptance suite stay.
4. The coding agent rebuilds in the working tree. It is instructed not to consult the previous implementation in accessible history.
5. The replacement is verified against the active delta and the applicable End-to-end (E2E) tests, committed as part of the change, and archived normally. Archive merges the delta once, after the rebuilt code passes the change checks.

The instruction states the limitation: a passing verification proves the replacement's tested behavior, not that the Specification alone was sufficient, because history stays accessible.

If the Specification or the End-to-end (E2E) suite changes after the rebuild starts, the change records a new frozen revision and reruns the relevant verification against it. The acceptance target is never moved to make the rebuilt code pass.

- `Rebuilt code: discard` keeps the external spec-copy path: `givn ise rebuild-component` prepares a copy outside the project and never edits the working tree.

### Reference direction

Givn cites the principle at https://iseprinciple.org in the served `ise` instruction and the README. The site stays framework-agnostic and names no Givn command, flag, or record; it uses the shared vocabulary (`Specification`, `Source code`, `Component`, `Component Interface`, `Single-Component Rebuild`, `Whole-system rebuild`, `No rebuild`, `Ise Test`) and describes the outcome as discarding or shipping the rebuilt code without a record heading.

### Truthful `Ise Test` wording

An **Ise Test** is one rebuild execution with the Specification and the acceptance suite held fixed. A copy-based Ise Test reads a selected revision and copies its inputs; a working-tree Ise Test replaces the Component in the current project working tree and relies on the instruction not to consult accessible history. Only the copy-based path isolates history; the working-tree path's passing checks do not prove the Specification alone was sufficient.

### Review record

The trial review records a **Rebuild impact record**: every file added, modified, deleted, moved, or renamed outside the declared Specification and outside the replaced unit, plus every consolidation (a merge, split, move, or rename of rebuilt paths), each with its reason. It is one review question, never a gate, and not a behavior diff. The clean-room scan becomes its own review line. A file the rebuild read but the Specification did not declare is a `specification defect`, not a record row.

### Vocabulary

| Object | Wording |
|---|---|
| Tracked `givn/specification.yaml` | **Specification Manifest** |
| One of its five YAML groups | **Specification category** (`behavior`, `structure`, `contracts`, `limits`, `e2e`) |
| A literal path or glob in a category | **manifest entry** |
| A file an entry selects | **matched file** |
| A matched file copied into the copy | **Specification file in the spec copy** |
| Generated `ise-manifest/v1` evidence | **spec-copy manifest** |
| Effective artifact rules and `givn/config.yaml` | **the Givn configuration** |
| Package build definition | the concrete filename, for example `Cargo.toml` |
| Current files | **project working tree** |
| Recorded state a copy reads | **selected revision** |
| The built thing | **System**; a rebuild replaces the system's Source code |
| Callable consumer boundary | **Component Interface** |
| Files the build reads | **files the rebuild needs** |
| Supplied tools | **installed toolchain and dependency cache** |

The glossary remains the sole vocabulary authority. This change reverses the `specification manifest` anti-term explicitly, defines `Component` and `System` as standalone rows, and sweeps the rejected nouns from current served prose, reference, Arc42, durable use cases, and the scenarios that assert them, except the unchanged copy-path scenario bodies recorded under D4. Historical archive evidence, the `ise-manifest/v1` scope, and literal command-output labels are deliberate exceptions.

### Unchanged

- `givn ise rebuild-all`, `rebuild-component`, and `compare` keep their commands, flags, exit codes, report labels, and strict `ise-manifest/v1` fields.
- The delta stays the active change's Specification; archive merges it once.
- `givn check tasks` verifies the active delta and the applicable E2E scope; `givn check review` checks the projected merge.

## Capability Routing

`givn spec route --change ise-rebuild-decision-and-working-tree-ship` reported signal `EXTEND givn-workflow-enforcement` (17.00) on proposal template vocabulary (`givn check`, `route`), not on the served guidance this change edits. The guidance capabilities were not ranked.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `ise-rebuild-decision` | `EXTEND givn-workflow-enforcement` (17.00) | `EXTEND ise-rebuild-decision` | The capability exists after the direct 1:1 rename of `change-strategy`; the delta adds the working-tree practice and the `Rebuilt code` scenarios under it. |
| `instruction-integrity` | no signal | `EXTEND instruction-integrity` | The `ise` and design-review served-text scenarios are owned by the `support` fragment. |

The `change-strategy` and `specification-declaration` files were renamed directly, before this change's delta: `ise-rebuild-decision.feature` and `specification-manifest.feature` exist in the permanent corpus, and their use cases name them. No retirement delta is needed because no active change owns a direct rename. The copy-path and spec-gap capabilities are unchanged by this delta; their served and reference nouns were swept as direct work, and their feature bodies are the D4 exception.

## Out of Scope

- No new command, flag, exit code, persisted format, or report label. The CLI `--dry-run` is not renamed.
- No change to the strict `ise-manifest/v1` fields or to the refused `ise-trial/v1` shape.
- No candidate branch, sandbox, or early archive merge.
- No claim that an instruction not to consult history makes history inaccessible.
- The site edit happens in its own repository (`/home/buster/projects/iseprinciples`); it uses the shared Ise vocabulary and names no Givn command, flag, or record, and is not part of this change's delta.
- The guidance text and vocabulary are direct work committed before this change's delta. The delta carries the new regression scenarios for the working-tree practice, the truthful `Ise Test` limitation, the rebuild impact record content, the copy distinction, the restart rule, and the `No rebuild` line.
- Historical archive evidence is not rewritten.

## Open Questions

### D1 — Name of the whole-system decision value

- Question: Which value names the decision to replace every declared `source:` entry's Source code?
- Options:
  1. **`Whole-system rebuild`** — the value uses the site's noun, "system"; gain: one noun in both projects and a value that reads without a glossary lookup; cost: introduces `System` into Givn vocabulary beside `Product type`, so "product" sentences need checking and a `System` row is required; in practice: `Decides D1: ise rebuild decision whole-system rebuild; Rebuilt code: ship`.
  2. **`whole-product rebuild`** — keep Givn's existing noun; gain: no new noun; cost: the site and Givn name one scope with two nouns; in practice: `Decides D1: ise rebuild decision whole-product rebuild; Rebuilt code: ship`.
- Disposition: answered — the operator selected `Whole-system rebuild` in this session.
- Follow-up: the `System` glossary row states that a rebuild replaces the system's Source code and keeps persisted data and external interfaces.

### D2 — Naming scheme for the four "manifest" objects

- Question: How should the tracked `givn/specification.yaml`, the generated `ise-manifest/v1` evidence, the effective artifact rules, and package build files be named?
- Options:
  1. **Full split** — Specification Manifest, spec-copy manifest, the Givn configuration, concrete build filenames; gain: one noun per object; cost: reverses an anti-term and rewrites ~40 reference mentions; in practice: "Declare the `e2e` category in the Specification Manifest; `--manifest` writes the spec-copy manifest."
  2. **Keep Specification declaration, split the rest** — gain: no anti-term reversal; cost: "declaration" stays beside the run declaration; in practice: "the Specification declaration … the spec-copy manifest".
  3. **Specification Manifest, nothing else renamed** — gain: smallest edit; cost: "manifest" still names three objects; in practice: "the manifest's artifact rules live in `givn/config.yaml`".
- Disposition: answered — the operator selected the full split, with the effective artifact configuration named **the Givn configuration** (`givn/config.yaml`).

### D3 — Contents of the rebuild review record

- Question: Which effects does the review record of a rebuild contain?
- Options:
  1. **Effects only, named `Rebuild impact record`** — changed-outside files and consolidations; gain: every row is an effect and the scan stops pretending to be a change; cost: the scan gets its own review line and the read-only undeclared input becomes a `specification defect`; in practice: `src/legacy/adapter.rs` deleted (moved behind the Component Interface) plus the consolidation row.
  2. **Include read rows under the same name** — gain: one table for every observation; cost: "impact" covers preconditions and evidence; in practice: an extra "read but undeclared" row.
  3. **Keep `Neighbor record`** — gain: no rename; cost: "neighbor" names both a Component and a review row; in practice: "Neighbor record: 3 rows".
- Disposition: answered — the operator selected the effects-only content and the name `Rebuild impact record`.

### D4 — Scope of the `Specification role` / `path` / `file` sweep

- Question: Do the three capitalized terms stay with a plain-language gloss, or are they removed across the corpus?
- Options:
  1. **Keep and explain** — gain: no delta rewrite of the copy-preparation scenarios; cost: the terms survive in reference and specs; in practice: the instruction explains categories and entries while the use case still says `Specification path`.
  2. **Full sweep** — gain: one vocabulary in specs, reference, and served prose; cost: ~200 occurrences, ~15 rewritten scenarios, and every bound step phrase; in practice: "the `e2e` category's entry `e2e/**` matches files that are never copied".
  3. **Served prose and glossary only** — gain: agents read new words; cost: served prose and the corpus disagree; in practice: the instruction says "category" while its verifying step asserts `Specification role`.
- Disposition: answered — the operator selected the full sweep, then directed that plain 1:1 renamings land as direct work outside a Givn change to avoid a huge search-and-replace delta. The direct work covers the served guidance, reference, Arc42, and changed scenario text; the unchanged copy-path narrative and its two feature files keep their step text as the recorded exception, because their report labels remain `Specification files:` and their behavior is unchanged.

### D5 — Where the working-tree ship sequence is stated

- Question: Where does the in-place sequence live, and does `givn ise rebuild-component` change?
- Options:
  1. **Instruction only; the command stays a copy preparer** — gain: no product code change and no false claim that a command rebuilds application code; cost: the ship path is a written practice, not an enforced tool; in practice: the `ise` instruction states the five steps; `rebuild-component` still writes an external copy.
  2. **Extend `rebuild-component` to read the working tree** — gain: one command entry point; cost: breaks its "clean recorded revision" contract and needs a new compatibility change; in practice: the command would delete code in place.
- Disposition: answered — the operator selected option 1 in the handoff; the command's contract is preserved.

### D6 — Strength of the `Ise Test` claim

- Question: How does the general term describe both the copy-based and the working-tree practice without overclaiming?
- Options:
  1. **Redefine `Ise Test` for both practices, state the limitation** — gain: one truthful term; cost: the strict clean-room wording is reserved for the copy path; in practice: "the working-tree path relies on an instruction not to consult accessible history".
  2. **Reserve `Ise Test` for the copy-based path** — gain: strict meaning; cost: the working-tree practice needs a second execution noun and the site's section 03 no longer covers it; in practice: "a working-tree rebuild is not an Ise Test".
- Disposition: answered — the operator selected option 1 in the handoff.
