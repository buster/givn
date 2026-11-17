# Ise rebuildability: rebuilding from the Specification

Status: published 2026-11-14 (topic handoff; term merge deferred to the seed
that makes each row true). The topic state under
`givn/ideation/ise-rebuildability/` is the record; this document is the
readable summary. It reopens no closed decision.

## 1. The problem

A project can pass every Givn gate and still fail the Ise Test. Givn's gates
check that a change has scenarios, a design, review evidence, and a receipt.
They do not check that the acceptance suite would still test the product if
the Source code were deleted.

Two repository facts make the failure concrete.

**Givn's own acceptance runner links the production crate.** One
`tests/features_runner.rs` binary runs both the regular and the `@e2e` scope.
Nine files under `tests/steps/` contain `use givn::*`. A total of 108
scenarios carry `@e2e` across 51 files. A rebuild of Givn would need the old
`givn` crate linked to run its own acceptance suite.

**The `@e2e` policy allowed in-process library calls and one shared runner.**
`assets/instructions/specs.md:121-150` permits a library consumer's real
public API calls to run in process. The runner has no dependency rule. In the
Passuff (`gh-notify`) pilot, `scripts/verify.sh:57` runs the same
`features_runner` binary for both scopes,
`crates/passuff-cli/tests/steps/tui.rs` calls
`passuff::tui::keys::parse_keys`, and
`crates/passuff-cli/tests/support/world.rs:572` builds the inbox through
`InboxService`. The suite is green, but it exercises library functions in
process, not the TUI through a real PTY.

Those facts expose four failure modes.

| Failure mode | Evidence | Consequence for an Ise Test |
|---|---|---|
| **Unprotected verification semantics** | The shared runner, the in-process calls above, and no dependency rule in the E2E target | The frozen suite can pass without touching the real interface, so a rebuild is never tested for the behavior the product exposes. |
| **Incomplete durable Specification** | CLI flags, exit codes, persisted schemas, state migrations, and viewport budgets sit in `src/`, change history, or prose rather than in a declared Specification set. | The agent cannot state what the replacement must preserve, and no file says which paths may be deleted. |
| **All-enabled addons and divided instruction ownership** | Nine addons default to `true`; the four core step instructions carried every runner's rows. In `givn-harness` the `v0.9.15` guidance addition kept the Catopia hidden grade at `25/25` while scenario volume fell from `130` to `76` and coverage from `87.3/79.9` to `67.7/60.8`. | Enabling `ise` without a stack-instruction split leaves a rule with more than one apparent owner and adds prose; the split keeps one owner per rule. |
| **No separation between co-evolution and frozen reconstruction** | Givn instructions assume specifications, tests, and code change together. | A rebuild has no clean-room protocol, no frozen acceptance authority, and no classification of differences. |

The four failure modes correspond to the Frame's seven gaps (`G1` to `G7`):
no opt-in addon default, instruction context pressure, an undefined
Specification, an
unenforced `@e2e` boundary, an incomplete E2E identity in receipts, no
reconstruction workflow, and Givn's own self-rebuild coupling.

## 2. What the Ise Principle requires

Source: The Ise Principle, Draft v0.5 (`https://iseprinciple.org/`; local
copy at `/home/buster/projects/iseprinciples/index.html`). The topic quotes
the source's rules and kept/discarded table, then names each element in
Givn's vocabulary.

| Rule | Source wording | What it forces in Givn |
|---|---|---|
| 1 | The spec must be enough to build from scratch. | A rebuild reads a declared Specification and cannot read the Source code it replaces. |
| 2 | Anything not in the spec is free to change. | Every observed difference receives an explicit classification; undocumented behavior is not preserved by accident. |
| 3 | Rebuild by routine. | The Ise Test is repeatable: prepare a clean room, freeze acceptance, build, run, classify. |

The kept/discarded table decides what survives a rebuild.

| Ise element | Fate in the source | Givn consequence |
|---|---|---|
| **Application code and internal structure** | Discarded: everything behind the public interfaces. | `Source code` is deleted and rebuilt: private modules, internal patterns, directory layout. |
| **Internal unit tests** | Discarded: tests coupled to private structure. | They are named paths in the deletion set. Givn never infers a unit test from `#[cfg(test)]`, `test_*.py`, a Maven test root, or a file-name convention. |
| **Component tests** | Scenarios kept, test setup rewritten. | The `.feature` scenarios stay in the Specification; step definitions, fakes, and harness are implementation and may be rewritten with the stack. |
| **End-to-end (E2E) tests** | Kept unchanged: they must pass against the rebuild without changes. | The declared `e2e` package is frozen as acceptance authority: the black-box suite plus its runner configuration. |
| **Behavior and invariants; contracts and schemas; technical limits; automated black-box tests** | Belong to the spec. | The five Specification roles of the declared Specification: `behavior`, `structure`, `contracts`, `limits`, `e2e`. |

Two consequences from the source shape the model. **Data, state, and
contracts outlive the code**, so a rebuild preserves persisted state and
public contracts. **Completeness is proven by building, not by reviewing**,
so the topic records a spec copy and its differences rather than trusting a
reading of the Specification.

## 3. The settled model

### 3.1 The declared Specification set

`givn/specification.yaml` declares exactly two things:

- the five Specification roles as path globs;
- one `source:` list of replaceable roots: production code plus component-test
  roots.

It declares no component, no internal component directory, and no
per-component file list. The rebuilt unit is a command input: `givn ise
rebuild-component <path>` names its paths, or `givn ise rebuild-all` removes
every declared source root. Neither is a permanent project declaration.

| Role | What it retains | Ise source element | Example carrier |
|---|---|---|---|
| `behavior` | Behavior and invariants; use-case contracts; Gherkin scenarios. | Behavior and invariants; automated tests (scenario text). | `givn/specs/**` |
| `structure` | Vocabulary and structural model; the terms and boundaries the agent must share. | Not a separate Ise heading; Givn adds it so the agent shares the domain language. | `docs/glossary.md`, `docs/domain-model.md` |
| `contracts` | Public APIs, events, wire formats, database schemas, externally visible UI states. | Contracts and schemas. | `docs/reference.md` |
| `limits` | Target stack, allowed dependencies, security and compliance rules, latency and resource budgets. | Technical limits. | `docs/arc42/02-architecture-constraints.md` |
| `e2e` | One package holding the E2E runner and every `@e2e` step definition. | Automated black-box tests. | `tests/e2e` |
| `source:` (not a role) | Replaceable roots: production code and component-test roots. | Application code and internal structure; internal unit tests. | `src/**`, `crates/**` |

The example in
`givn/ideation/ise-rebuildability/use-cases/declare-the-specification.md` is a Givn
self-rebuild declaration: `behavior: ["givn/specs/**"]`, `structure:
["docs/glossary.md", "docs/domain-model.md"]`, `contracts:
["docs/reference.md"]`, `limits:
["docs/arc42/02-architecture-constraints.md"]`, `e2e: ["tests/e2e"]`, and
`source: ["src/**"]`. A project chooses its own paths.

The write is strict. A declaration with an unknown key or role, a repeated
role, an omitted role or an absent `source:` list, or a path outside the
project is refused by name and nothing is written. A refused write leaves the
previous declaration byte-for-byte unchanged.

The project-health pass reports four advisory findings from `givn lint`:

| Finding | Meaning |
|---|---|
| **Declared path missing** | A role glob matches no tracked path. |
| **Path declared in two roles** | One path is matched twice. |
| **`.feature` outside the `behavior` globs** | A scenario file is not matched by a role. |
| **`e2e` package missing or with a dependency edge into `source`** | The acceptance package cannot be frozen, or it links production code. |

The findings are advisory: they do not change the lint exit code. The
blocking `@e2e` boundary belongs to `givn check tasks`, which refuses before a
receipt is written.

The scaffold arrives through setup, not through a project decision. The first
effective enable of `ise` writes `givn/specification.yaml` when the file is
absent. `givn upgrade` writes and stages it when `ise` is effective and the
file is absent. An existing declaration is project-owned and is preserved
byte-for-byte by both paths.

### 3.2 The E2E package boundary

With `ise` effective, the declared `e2e` role names one package:

- it holds the E2E runner and every `@e2e` step definition;
- it links no production source;
- `verify.e2e_command` runs it;
- an unbound or skipped step fails the run, and no passed receipt is issued.

Feature files stay shared. An `@e2e` scenario and a component scenario may
live in the same `.feature` file, because the package boundary, not the file,
selects what the E2E runner executes.

The boundary is the package manifest dependency graph plus the runner's unbound
or skipped step failure. There is no static import scanner and no phrase
uniqueness rule. Component step definitions, fakes, and harness are
implementation: they are never frozen and never diff-counted.

The change verification receipt binds three values: the `e2e` package
digests, the E2E runner command digest, and the running Givn version. Archive
freshness compares them, so a weakened black-box suite changes the receipt.
For an Ise Test, the End-to-end checksum covers the `@e2e` scenarios selected by
tag plus every file of the `e2e` package.

### 3.3 The three commands

All three commands are bound to `addons.ise`. With the addon off they refuse by
name and read no declaration. The addon requires `architecture_principles`, so
the guidance that uses `Component` and the other established nouns always has
its vocabulary present.

| Command | Inputs | Result |
|---|---|---|
| `givn ise rebuild-all --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | A revision (`HEAD` when omitted) and a target outside the checkout. | A plain spec copy in which the whole product is rebuilt: every declared source root is removed from it. The five counts, the two checksums, and, when `--manifest` names a file, the manifest. |
| `givn ise rebuild-component <path>... --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | A revision, at least one tracked file or directory inside the declared `source:` roots, and a target outside the checkout. | A plain spec copy in which one component is rebuilt: only the named paths are removed, and neighbouring components and their contracts stay. The same report and manifest option. |
| `givn ise compare --manifest <file> --copy <dir>` | A written manifest and the spec copy; the manifest's `copy_directory` must name the copy. | The two subject lines and one `- <path>` or `+ <path>` line per difference. |

A rebuild refuses, before any write, on twelve conditions.

| Refusal | Condition |
|---|---|
| 1 | The revision records the declaration but the backend returns no blob; the refusal names the path, the revision, and `returned no blob`. |
| 2 | The declaration bytes at the revision are not UTF-8; the refusal names the path and the decoding error. |
| 3 | The declaration at the revision is malformed; the refusal names the path and the loader's reason. |
| 4 | The revision does not record the declaration, or a declared Specification entry matches nothing; the refusal names the path, the revision, and `does not record`, or the entry and the revision. |
| 5 | The `ise` addon, or its `architecture_principles` prerequisite, is not effective. |
| 6 | The checkout is dirty; the refusal names every dirty path. |
| 7 | The backend is missing, mixed, or nested. |
| 8 | A component path falls outside every declared `source:` root; the refusal names `rebuild-all`. |
| 9 | A component path, or a tracked file under a named directory, is matched by a Specification role or is the declaration. |
| 10 | The target is inside the checkout or already exists, or the manifest is inside the copy, an ancestor of it, or already exists. |
| 11 | A repository link or nested repository sits among the files written into the spec copy. |
| 12 | A symlink has an absolute target, even one that resolves inside the copy, or a relative target that resolves outside the spec copy. |

A refusal names its condition, exits non-zero, and leaves the checkout and the
target path unchanged. `rebuild-component` with no path and an omitted `--into`
are usage errors (exit `2`) naming the missing path list or target. `--dry-run`
performs every read, the partition, every refusal, the counts, and the
checksums, writes no directory and no manifest, and prints
`Copy directory (would write):` and, only when a manifest path was given,
`Manifest (not written):` instead of the two write lines.

The report prints eleven fixed lines and, only when a manifest path was given,
the `Manifest` line: twelve with the manifest path.

| Line | Meaning |
|---|---|
| `Backend` | The selected backend, `git` or `fossil`. |
| `Revision` | The resolved revision the copy was read from. |
| `Specification files` | The count of tracked files the copy was written with: the declaration and every Specification-role match the End-to-end (E2E) tests do not cover. |
| `Source code kept` | The count of tracked files under a declared source root that the command did not remove and that are not Specification files and not End-to-end (E2E) matches. |
| `Source code removed` | The count of tracked files the component paths, or every declared source root under `rebuild-all`, removed. |
| `Outside the spec copy` | The count of tracked files outside every declared role and source root that the copy neither holds nor removes. |
| `Files beside removed paths` | The direct tracked files in the removed files' parent directories that the command did not name and that are not themselves removed; a containment count, not a partition member. |
| `Specification checksum` | The rollup of the Specification files, printed as 12 lowercase hex characters. |
| `End-to-end checksum` | The rollup of the `@e2e` scenarios selected by tag and every file of the `e2e` package, printed as 12 lowercase hex characters. |
| `End-to-end tests` | `not copied (<n> files)`, where `<n>` is the End-to-end (E2E) set's own size. |
| `Copy directory` | The spec copy directory. |
| `Manifest` | The manifest file, only when `--manifest` was given. |

`Files beside removed paths` is a containment count, never a refusal. It counts
the direct tracked files in the removed files' parent directories that the
command did not name and that are not themselves removed; it is the same count
under both commands, and under `rebuild-all` the files it shows are the
Specification and End-to-end (E2E) matches in those directories. `Outside the
spec copy` counts tracked files that are neither errors nor copy content.

The spec copy is a plain directory outside the checkout, never a branch or a
worktree. A branch carries history, and a Fossil checkout carries the
repository database, so either would hand the agent the old Source code. The
copy preserves relative paths, the executable bit, and relative symlink targets
that resolve inside the copy. It carries no version-control metadata and no
ignored or untracked files, because a rebuild reads the selected revision, not
the working tree.

The manifest carries the scope, the revision, the removed paths, the
Specification and End-to-end (E2E) path inventories, the five counts, the three
checksum maps, the two rollups, the copy directory, and the running Givn
version. Without `--manifest` no manifest is written and none is printed, and
`givn ise compare` requires an explicit path. `compare` refuses a manifest whose
`copy_directory` is not the given copy or whose file lies inside the copy.

### 3.4 The spec copy rule

Four sentences decide the copy and its comparison.

> The spec copy is written with the project's Specification files. During a
> rebuild the agent adds files, and those are the rebuilt Source code. The agent
> may edit Source code kept because it is the product, but it does not delete
> any file the copy was written with. The agent does not modify any
> Specification file the copy was written with; adding a file under a declared
> Specification path is a Specification change, not the product, and is refused.

The comparison is their enforcement. It prints `Specification files` and
`End-to-end tests`, each `match` or its `changed`, `missing`, and `added`
counts, one `- <path>` line per changed or missing path and one `+ <path>` line
per added path, and exits `1` on any difference. It re-checks the manifest's
Specification paths against the copy, and against the checkout except a feature
path carrying a selected `@e2e` scenario, which the End-to-end (E2E) line owns
on the checkout side; the copy side still compares that file's bytes under
`Specification files`. It refuses a file the copy
adds under a declared Specification path, and refuses a copy that lost a path
the manifest lists among the Source code kept. It counts a checkout path
matched by a declared Specification role as an added Specification file, except
a content path in the checkout's declared End-to-end (E2E) set or a feature
path carrying a selected `@e2e` scenario, which it counts as an added End-to-end
test; E2E-set membership or a selected `@e2e` scenario owns the path on the
checkout side, so one line prints even when another Specification role also
matches it. A checkout
Specification file that differs prints `Specification changed after the copy was
written`.

The End-to-end (E2E) package and the selected `@e2e` scenarios decide the
`End-to-end tests` line; a recorded Specification file is still compared with
the checkout under `Specification files`, except the feature path that carries
a selected `@e2e` scenario, which the End-to-end (E2E) line owns there. The
declared Specification is recorded by checksum
so the developer can prove what the agent received, but the End-to-end (E2E)
test is what decides pass or fail. An edited black-box scenario, step
definition, or runner file is a different Ise Test, not a passing one. A
changed component step definition outside those inputs produces no difference,
because component step definitions are implementation.

### 3.5 Qualification before freeze

A new E2E scenario enters the frozen set only after it was observed failing
against the pre-change implementation for the intended reason. The freeze is
a per-Ise-Test boundary, not a ban on correcting the test before the Ise Test
starts.

A test defect found mid-rebuild is a `Spec gap: E2E test defect`. It stops the
Ise Test. The E2E test is corrected, and the Ise Test restarts. The `e2e` package
is part of the Specification, so correcting the test is a Specification
correction, not an exception to the freeze. The result is never accepted
under a changed test.

## 4. The two records a change writes

### 4.1 `Change strategy`

Every change records one strategy as a `Decides D<n>` entry in its design
ledger. The entry has no new mandatory design field. It names the affected
consumers, the persisted state, and the rebuild boundary in prose.

| Value | Meaning | `Ise Test` mode |
|---|---|---|
| **incremental modification** | Edit the existing code in place; no Ise Test runs. | none |
| **Constant Ise** | Update one component's specification, delete that component's implementation and white-box unit tests inside its boundary, and rebuild it behind its unchanged consumer contract. | `Dry-Run Ise` or `Full replacement` |
| **whole-product rebuild** | Regenerate all application source behind the product's external interfaces. | `Dry-Run Ise` or `Full replacement` |

A rebuilding change records the mode beside the strategy: `Dry-Run Ise` for a
throwaway clean-room build, `Full replacement` for the production switch.
Prose writes `change strategy` in full because "strategy" is generic, and
every mode sentence says "Ise Test mode" because `Full replacement` sits one
word from a whole-product rebuild.

An unresolved or unrecorded strategy, or a rebuild with no recorded mode, is
a design-review finding. It is not a missing artifact.

### 4.2 `Spec gap`

A `Spec gap` is the classification record of one observed difference between
the frozen E2E test result and the declared Specification. It holds exactly
one outcome, recorded before any fix.

| Outcome | What is wrong | Route |
|---|---|---|
| **`E2E test defect`** | The test. | Correct the E2E test; the Ise Test stops and restarts. A corrective change edits the E2E suite. |
| **`implementation defect`** | The build. | Return to the agent inside the Ise Test. No corpus or suite edit. |
| **`specification defect`** | The Specification. | Open a corrective change that adds the missing rule, then rebuild. |
| **`acceptable undocumented change`** | Nothing that matters. | Record the difference in the corpus. No silent drift. |

Only `specification defect` is a gap in the Specification. An `E2E test
defect` is a gap in the test, an `implementation defect` is a gap in the
build, and an `acceptable undocumented change` is no gap at all. The record
names the artifact that is wrong, not who found it.

## 5. What Givn does not do

- **No unit-test inference.** Deletion is set arithmetic on the files named
  with `rebuild-component` and on the source-root union under `rebuild-all`,
  each inside a declared `source:` root. Givn does not read `#[cfg(test)]`,
  `test_*.py`, a Maven test root, or a file-name convention, and it never
  expands a glob.
- **No component list.** `givn/specification.yaml` has no `components:` key
  and no per-component file list. Internal components and their files do not
  belong in a permanent project declaration.
- **No phrase-uniqueness rule.** A step definition in the declared `e2e`
  package cannot reach production source, so the component tree's copy of a
  phrase is never compiled by the E2E runner. The package boundary is the
  rule.
- **No blast-radius gate.** A change to a neighboring component is legitimate.
  Review asks one question: name the neighboring components changed outside
  the replaced component and say why. No gate, receipt field, or diff count
  enforces it.
- **No branch or worktree.** The spec copy is a plain directory outside the
  checkout. A branch carries history and a Fossil checkout carries the
  repository database.
- **No containment of the agent.** Givn withholds the replaced Source code
  and history from the spec copy and records what it handed over. It does not
  sandbox the agent's process or prove what the agent read. Isolating the
  agent's context is the caller's obligation, with the harness or the
  developer.
- **No built-in rebuild orchestration for other repositories.** Givn prepares
  the clean room and re-checks its frozen inputs. It does not build the
  replacement or run the frozen tests for `givn-harness` or Passuff. Those
  repositories run their own phases against the same three commands.

## 6. The decision record

The topic closed sixteen questions. The full options, gains, costs, and
rejected alternatives stay in
`givn/ideation/ise-rebuildability/questions.md`.

| Q | Decision | Outcome | Rejected alternative that mattered |
|---|---|---|---|
| `Q1` | Entry-point sequence | Ideation in Givn first, paired with the Catopia harness track, then Passuff. | Passuff-first, because it delays the domain design and automated signal. |
| `Q2` | Frame | Goal, four Givn capability areas, companion tracks, five exclusions, and four stakeholder types confirmed. | Committing to a CLI surface before Event Storming fixed the contract ahead of the use cases. |
| `Q3` | CLI surface | `givn ise rebuild-all`, `givn ise rebuild-component`, and `givn ise compare`, bound to `addons.ise`. | Check-only enforcement, which leaves the clean room to each project's script. |
| `Q4` | Declaration site | Dedicated `givn/specification.yaml` with five roles and a `source:` list, no component list. | A block in `givn/config.yaml` (the lenient top level hides a misspelled block) and convention (cannot express the `source` partition). |
| `Q5` | E2E boundary | One `e2e` package holding the runner and every `@e2e` step definition, no production link, unbound or skipped step fails, feature files shared. | Keeping the tag's in-process library allowance, and splitting the suite into multiple targets. |
| `Q6` | Addon coupling | `ise` requires `architecture_principles` in both directions. | No prerequisite (an `ise`-only project), and auto-enabling the prerequisite. |
| `Q7` | Personas | Three personas; the bare-clone and End-to-end (E2E) test lens sits under `givn-maintainer--owner`. | A fourth external-contributor persona, and separate personas for the agent and the End-to-end (E2E) test. |
| `Q8` | Addon posture name | `Opt-in addon`. | `Experimental addon`, which names maturity rather than the stored default. |
| `Q9` | Term process | Revisit all seven Event Storming terms one by one on The Ise Principle's wording. | Keeping the coined terms. |
| `Q10` | Durable input name | `Specification`, contrasted with `Source code`. | `Product specification`, which makes existing feature-specification sentences false. |
| `Q11` | Acceptance suite name | `End-to-end (E2E) test`, with `black-box` as the no-shared-code qualifier. | `Black-box E2E test` and `Black-box verifier`. |
| `Q12` | One-unit strategy name | `Constant Ise`. | `Component replacement`, whose `Component` definition is unavailable in an `ise`-only project. |
| `Q13` | Execution name | `Ise Test`, with `Constant Ise` as the scope and `Dry-Run Ise` / `Full replacement` as the modes. | `Reconstruction trial`, a Givn coinage with no Ise counterpart. |
| `Q14` | Difference record name | `Spec gap`, with exactly four outcome values. | `Mismatch triage`, which collides with Givn's blocking-mismatch senses. |
| `Q15` | On-demand instruction name | No new term; `Instruction` plus the id convention (`<id>-examples`, `<id>-<stack>`). | `Stack recipe`, which asserts a prescribed procedure and excludes `dialog-examples`. |
| `Q16` | Change-level choice name | `Change strategy`, the design's `Decides D<n>` entry. | `Rebuild scope`, which splits the record and collides with `Verification scope`. |

## 7. The plan

The plan tests one claim: a Specification declared in `givn/specification.yaml`,
frozen with the declared `e2e` package and recorded by digest, is sufficient
for a fresh agent to rebuild a component or a whole product without reading
the replaced Source code or repository history, while the frozen E2E package
stays green. The evidence decides whether `ise` can ever become default-on.

The phase detail, the dependency graph, and every gate live in
`givn/ideation/ise-rebuildability/plan.md`.

| Phase group | Repository | Deliverable | Deciding gate |
|---|---|---|---|
| `P0` Handoff | `givn` | Topic document, confirmed terms, promoted personas, change seeds. | Every seed passes to `/givn-propose` without the topic conversation. |
| `S1` to `S5` | `givn` | The `ise` addon and declaration, the advisory findings, the `@e2e` gate and receipt identity, the three commands, and the stack-instruction split. | Fresh init omits `ise`; both prerequisite refusals print; each declaration finding exits 0; the twelve refusals write nothing; the receipt binds the E2E identity; the core instruction carries no per-runner body. |
| `S6a` to `S9` | `givn-harness` | Stage snapshots, target-stage grading, state continuity, sandboxed candidate grading, versioned `ise` task variants, and Catopia E1 to E4. | `ise=on` reaches `25/25`, at least 119 regular plus 11 `@e2e` scenarios, and at least `87.33/79.85` coverage; every difference carries one `Spec gap`; a Constant Ise rebuild leaves the consumer contract unchanged. |
| `S10` to `S12` | Passuff | A separate `e2e` package with PTY `80x24` and `120x40` and an HTTP mock, a Constant Ise rebuild, and a full-product `Dry-Run Ise`. | The package declares no production dependency; every seeded defect fails a named scenario; the Constant Ise rebuild names every changed neighboring component; the full-product rebuild passes or classifies every difference. |
| `S13` | `givn` | Givn rebuilds itself: `@e2e` steps leave the production-linked target, `ise` is enabled, and a `Dry-Run Ise` runs. | The `e2e` package has no dependency on `givn`; every retagged scenario has a reason; every gap has one outcome. |
| `S14` | `givn` | Promotion decision for `ise`. | The evidence table is complete; without it `ise` stays opt-in. |

The plan's stop rules are explicit: a phase does not skip its predecessor's
recorded evidence; a failing gate is reported with the command and the
observed value; a benchmark task file is never edited in place.

## 8. Results

The gates in section 7 are the plan's thresholds. The arms below are what was
measured: seven paid runs, $2.1804 committed, on the `catopia-v2-ise` `0.3.0`
benchmark. Arm-level detail, including bundle paths, revisions, and per-run
spend, is in `docs/experiments/ise-rebuild-experiment.md` in the
`givn-harness` worktree `/home/buster/projects/givn-harness-ise`.

| Arm | Outcome | Grade |
|---|---|---|
| `initial`, `ise` on | `archived_and_failed` | 13/14, `R8` |
| `initial`, `ise` off | `archived_and_passed` | 14/14 |
| cold rebuild, first two runs | `stalled`, then `delivered_and_failed` | `G1` only |
| cold rebuild, replicate | `delivered_and_passed` | 16/16, frozen verifier 15/15 |
| `Constant Ise` follow-up | re-graded 2/2, but chose incremental | 2/2 |
| state continuity, two stages | `archived_and_passed` | 9/9 |

**The protocol works for a cold rebuild.** The replicate rebuilt the executable
surface from the declared Specification, passed the frozen verifier 15/15 and
every graded behaviour check, and fixed the seed's `R8` failure. Its record
reads `Source code kept 0; Source code removed 1`.

**The End-to-end (E2E) test is weaker than the contract.** The first rebuild
carried neither `contract.md` nor `law.md`, because the declaration left both
outside every role, and the frozen test still passed 15/15. A rebuild can pass
with the contracts it never received.

**Three protocol defects were found and fixed.** `guard-retained-paths-across-archive`
refuses an archive that moves a declared Specification path, after an agent declared
a change artifact as one and the archive moved it. `retain-declaration-in-trial`
copies `givn/specification.yaml` into every spec copy and re-checks it per path,
after the first spec copy carried no declaration. Harness `4f477bc` grades a stage
with `require_archive = false` in-stage, after the rebuild stage stalled
waiting for an archive that its `require_archive = false` setting never
produces.

**The gap-record failure did not reproduce.** One rebuild failed `G1` for gap
records naming a passing check and an artifact outside every source root; the
replicate ran the same seed on the same binary and passed with no gap file.

**Two questions stay open.** `Constant Ise` remains unmeasured, because the
only follow-up arm recorded `incremental modification` and its snapshot holds
no manifest; and the End-to-end (E2E) test-strength question stays open,
because no arm tests whether the End-to-end (E2E) test, reading only the
declared roles, can certify a rebuild that the declared Specification does not
describe.

**The S14 evidence is compiled.** The promotion tables for `ise` sit in
`## S14 evidence` of `givn/ideation/ise-rebuildability/plan.md`: eight archived
changes with `proven` receipts and the two withdrawn ones, the instruction
split (1,630 to 1,544 lines for the four core steps, six stack instructions,
4,561-line corpus after the numeric cap was withdrawn), a cold rebuild at 20 of
20 with an 18 of 18 frozen verifier in two runs, two `Constant Ise` follow-ups
replacing the same three paths with every named neighbour unchanged, 9 of 9
continuity, an `ise` off and on pair that differs only on `R8`, the Passuff
ladder from 8 of 8 with the tests visible to 3 of 8 blind to 8 of 8 after the
first amendment to 25 of 29 and 29 of 29 after the widened suite and the third
amendment, and the limits that stay: a 29-scenario acceptance suite, an
unenforced contamination rule, `gap-03`'s unstated post-migration observation,
and no Givn self-rebuild.

## 9. Open items

- **Promotion to default-on.** `S14` decides. Until then `ise` remains an
  `Opt-in addon` with `default_enabled: false`.
- **`Strangler Ise`.** The incremental cutover pattern is not modeled. It
  would need its own topic and a specification for partially replaced
  products.
- **WIP drafts `Draft 05` to `Draft 07`.** `Shadow Ise`, `Blind Verification`,
  and `Adversarial Breaker` stay out. `ise` v0.1 must work in a single
  context with no subagents.
- **Excluded Givn work.** In-binary model calls, network services, language
  runtimes, user-defined custom workflows, and a per-component declaration
  are outside the plan.
- **Historical benchmarks.** `catopia-v2` and `catopia-spring-v2` stay
  byte-identical. Any rebuild-permitting experiment is a new versioned
  benchmark.
- **Static import scanning and phrase uniqueness.** Both were rejected for
  the E2E boundary. The package dependency graph and the runner's unbound or
  skipped step failure are the settled checks.
- **Presentation choices.** The 12-hex checksum in the report and the eleven
  fixed report lines (plus `Manifest` only when a manifest is written) are
  fixed in the command reference. The command defaults (`--at` optional,
  `rebuild-component` requiring at least one path, `--dry-run`, no manifest
  without `--manifest`) are settled and stated there.
