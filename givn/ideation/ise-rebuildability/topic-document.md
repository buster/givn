# Ise rebuildability: rebuilding from the Specification

Status: draft for handoff, 2026-11-14. The topic state under
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
| **Incomplete durable Specification** | CLI flags, exit codes, persisted schemas, state migrations, and viewport budgets sit in `src/`, change history, or prose rather than in a declared retained set. | A builder cannot state what the replacement must preserve, and no file says which paths may be deleted. |
| **All-enabled addons and instruction budget** | Nine addons default to `true`; `assets/instructions/` holds 4,397 lines; in `givn-harness` the `v0.9.15` guidance addition kept the Catopia hidden grade at `25/25` while scenario volume fell from `130` to `76` and coverage from `87.3/79.9` to `67.7/60.8`. | Enabling `ise` without a stack-instruction split adds prose and can lower scenario volume and coverage. |
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
| **Behavior and invariants; contracts and schemas; technical limits; automated black-box tests** | Belong to the spec. | The five retained roles of the declared Specification: `behavior`, `structure`, `contracts`, `limits`, `e2e`. |

Two consequences from the source shape the model. **Data, state, and
contracts outlive the code**, so a rebuild preserves persisted state and
public contracts. **Completeness is proven by building, not by reviewing**,
so the topic records a trial and its differences rather than trusting a
reading of the Specification.

## 3. The settled model

### 3.1 The declared Specification set

`givn/specification.yaml` declares exactly two things:

- the five retained roles as path globs;
- one `source:` list of replaceable roots: production code plus component-test
  roots.

It declares no component, no internal component directory, and no
per-component file list. The replaced unit is trial input, passed to
`givn ise prepare --replace <path>`, not a permanent project declaration.

| Role | What it retains | Ise source element | Example carrier |
|---|---|---|---|
| `behavior` | Behavior and invariants; use-case contracts; Gherkin scenarios. | Behavior and invariants; automated tests (scenario text). | `givn/specs/**` |
| `structure` | Vocabulary and structural model; the terms and boundaries a builder must share. | Not a separate Ise heading; Givn adds it so a builder shares the domain language. | `docs/glossary.md`, `docs/domain-model.md` |
| `contracts` | Public APIs, events, wire formats, database schemas, externally visible UI states. | Contracts and schemas. | `docs/reference.md` |
| `limits` | Target stack, allowed dependencies, security and compliance rules, latency and resource budgets. | Technical limits. | `docs/arc42/02-architecture-constraints.md` |
| `e2e` | One package holding the E2E runner and every `@e2e` step definition. | Automated black-box tests. | `tests/e2e` |
| `source:` (not a role) | Replaceable roots: production code and component-test roots. | Application code and internal structure; internal unit tests. | `src/**`, `crates/**` |

The example in `use-cases/declare-the-specification.md` is a Givn
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
| **Path declared in two roles** | One path is retained twice. |
| **`.feature` outside the `behavior` globs** | A scenario file is not retained. |
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
For an Ise Test, the frozen digest covers the `@e2e` scenarios selected by tag
plus every file of the `e2e` package.

### 3.3 The two commands

Both commands are bound to `addons.ise`. With the addon off they refuse by
name and read no declaration. The addon requires `architecture_principles`,
so the guidance that uses `Component` and the other established nouns always
has its vocabulary present.

| Command | Inputs | Result |
|---|---|---|
| `givn ise prepare [--at <revision>] [--replace <path>...] --into <dir> [--receipt <path>] [--dry-run]` | A revision (`HEAD` when omitted), the component's paths inside the declared `source:` roots when a component is replaced, and a target outside the checkout. | A plain trial directory, the partition counts, the retained and E2E identities, and the trial record; a receipt path writes the record to a file. |
| `givn ise verify --trial <receipt> --at <dir>` | A written trial record and the trial directory. | A per-path drift report for the retained set and the `e2e` package. |

The preparation refuses, before any write, on nine conditions.

| Refusal | Condition |
|---|---|
| 1 | The declaration is missing. |
| 2 | The `ise` addon is not effective. |
| 3 | The checkout is dirty. |
| 4 | The backend is missing, mixed, or nested. |
| 5 | A retained path is missing at the selected revision. |
| 6 | A replaced path falls outside every declared `source:` root. |
| 7 | A replaced path is matched by a retained role. |
| 8 | The target is inside the checkout or already exists. |
| 9 | A repository link or nested repository sits among the files written into the trial. |

A refusal names its condition, exits non-zero, and leaves the checkout and
the target path unchanged. `--dry-run` performs every read, the partition,
every refusal, the counts, and the digests, writes no directory and no
record, and prints the paths it would use.

The report prints eleven lines, the last only when a receipt path was given.

| Line | Meaning |
|---|---|
| `Backend` | The selected version-control backend. |
| `Revision` | The revision read for the trial. |
| `Retained` | The count of retained files written. |
| `Kept source` | The count of source files outside the replaced paths that were still written. |
| `Replaced` | The count of paths named with `--replace`. |
| `Outside the declaration` | The count of tracked files outside every declared role that were not copied. |
| `Not replaced` | The count of tracked files in the replaced paths' directories that were not named. |
| `Retained digest` | The tamper-evident identity of the retained set. |
| `E2E digest` | The identity of the `@e2e` scenarios selected by tag plus every file of the `e2e` package. |
| `Written path` | The trial directory. |
| `Receipt path` | The record file, when `--receipt` was given. |

`Not replaced` is a containment count, never a refusal. It shows what the
replacement unit still contains. `Outside the declaration` counts tracked
files that are neither errors nor trial content.

The trial is a plain directory outside the checkout, never a branch or a
worktree. A branch carries history, and a Fossil checkout carries the
repository database, so either would hand the builder the old Source code.
The trial preserves relative paths, the executable bit, and symlink targets.
It carries no version-control metadata and no ignored or untracked files,
because preparation reads the selected revision, not the working tree.

The trial record carries the revision, the paths named with `--replace`, the
counts, the retained and E2E identities, the backend, and the running Givn
version. An omitted `--receipt` prints the record to stdout only and writes
nothing, so `givn ise verify --trial <receipt>` requires an explicit path.

### 3.4 The freeze as a digest

Preparation re-reads every written file and compares it to the file recorded
at the selected revision. It records three identities: the retained set, the
`e2e` package, and the replaced paths. The first two are printed as
`Retained digest` and `E2E digest`; the replaced-paths identity sits in the
trial record. A written file that differs from its revision content fails
preparation and writes no record.

Only the `End-to-end (E2E)` test package is frozen as acceptance authority.
The declared Specification is the builder's read-only input and is recorded
by digest so a verifier can prove what the builder received, but the E2E
suite is what decides pass or fail. The freeze is a digest, not a filesystem
lock: the builder receives the package as retained content and may not edit
it, and `givn ise verify` detects an edit per path. An edited black-box
scenario, step definition, or runner file is a different Ise Test, not a
passing one.

`givn ise verify` re-checks the retained and `e2e` identities against the
written trial and the current `e2e` package. It reports drift per path. A
changed component step definition outside the frozen inputs produces no
drift, because component step definitions are implementation.

### 3.5 Qualification before freeze

A new E2E scenario enters the frozen set only after it was observed failing
against the pre-change implementation for the intended reason. The freeze is
a per-trial boundary, not a ban on correcting the test before the trial
starts.

A test defect found mid-trial is a `Spec gap: E2E test defect`. It stops the
trial. The E2E test is corrected, and the trial restarts. The `e2e` package
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
| **`E2E test defect`** | The test. | Correct the E2E test; the trial stops and restarts. A corrective change edits the E2E suite. |
| **`implementation defect`** | The build. | Return to the builder inside the Ise Test. No corpus or suite edit. |
| **`specification defect`** | The Specification. | Open a corrective change that adds the missing rule, then rebuild. |
| **`acceptable undocumented change`** | Nothing that matters. | Record the difference in the corpus. No silent drift. |

Only `specification defect` is a gap in the Specification. An `E2E test
defect` is a gap in the test, an `implementation defect` is a gap in the
build, and an `acceptable undocumented change` is no gap at all. The record
names the artifact that is wrong, not who found it.

## 5. What Givn does not do

- **No unit-test inference.** Deletion is set arithmetic on the paths named
  with `--replace`, each inside a declared `source:` root. Givn does not read
  `#[cfg(test)]`, `test_*.py`, a Maven test root, or a file-name convention.
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
- **No branch or worktree.** The trial is a plain directory outside the
  checkout. A branch carries history and a Fossil checkout carries the
  repository database.
- **No containment of the builder.** Givn withholds the replaced Source code
  and history from the trial and records what it handed over. It does not
  sandbox the builder's process or prove what the builder read. Isolating the
  builder's context is the caller's obligation, with the harness or the
  developer.
- **No built-in rebuild orchestration for other repositories.** Givn prepares
  the clean room and re-checks its frozen inputs. It does not build the
  replacement or run the frozen tests for `givn-harness` or Passuff. Those
  repositories run their own phases against the same two commands.

## 6. The decision record

The topic closed sixteen questions. The full options, gains, costs, and
rejected alternatives stay in `questions.md`.

| Q | Decision | Outcome | Rejected alternative that mattered |
|---|---|---|---|
| `Q1` | Entry-point sequence | Ideation in Givn first, paired with the Catopia harness track, then Passuff. | Passuff-first, because it delays the domain design and automated signal. |
| `Q2` | Frame | Goal, four Givn capability areas, companion tracks, five exclusions, and four stakeholder types confirmed. | Committing to a CLI surface before Event Storming fixed the contract ahead of the use cases. |
| `Q3` | CLI surface | `givn ise prepare` and `givn ise verify`, bound to `addons.ise`. | Check-only enforcement, which leaves the clean room to each project's script. |
| `Q4` | Declaration site | Dedicated `givn/specification.yaml` with five roles and a `source:` list, no component list. | A block in `givn/config.yaml` (the lenient top level hides a misspelled block) and convention (cannot express the `source` partition). |
| `Q5` | E2E boundary | One `e2e` package holding the runner and every `@e2e` step definition, no production link, unbound or skipped step fails, feature files shared. | Keeping the tag's in-process library allowance, and splitting the suite into multiple targets. |
| `Q6` | Addon coupling | `ise` requires `architecture_principles` in both directions. | No prerequisite (an `ise`-only project), and auto-enabling the prerequisite. |
| `Q7` | Personas | Three personas; the bare-clone and frozen-verifier lens sits under `givn-maintainer--owner`. | A fourth external-contributor persona, and separate builder and verifier personas. |
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
for a fresh builder to rebuild a component or a whole product without reading
the replaced Source code or repository history, while the frozen E2E package
stays green. The evidence decides whether `ise` can ever become default-on.

The phase detail, the dependency graph, and every gate live in `plan.md`.

| Phase group | Repository | Deliverable | Deciding gate |
|---|---|---|---|
| `P0` Handoff | `givn` | Topic document, confirmed terms, promoted personas, change seeds. | Every seed passes to `/givn-propose` without the topic conversation. |
| `S1` to `S5` | `givn` | The `ise` addon and declaration, the advisory findings, the `@e2e` gate and receipt identity, the two commands, and the stack-instruction split. | Fresh init omits `ise`; both prerequisite refusals print; each declaration finding exits 0; the nine refusals write nothing; the receipt binds the E2E identity; the core instruction line budget holds. |
| `S6a` to `S9` | `givn-harness` | Stage snapshots, target-stage grading, state continuity, sandboxed candidate grading, versioned `ise` task variants, and Catopia E1 to E4. | `ise=on` reaches `25/25`, at least 119 regular plus 11 `@e2e` scenarios, and at least `87.33/79.85` coverage; every difference carries one `Spec gap`; a Constant Ise rebuild leaves the consumer contract unchanged. |
| `S10` to `S12` | Passuff | A separate `e2e` package with PTY `80x24` and `120x40` and an HTTP mock, a Constant Ise trial, and a full-product `Dry-Run Ise`. | The package declares no production dependency; every seeded defect fails a named scenario; the Constant Ise trial names every changed neighboring component; the full-product trial passes or classifies every difference. |
| `S13` | `givn` | Givn rebuilds itself: `@e2e` steps leave the production-linked target, `ise` is enabled, and a `Dry-Run Ise` runs. | The `e2e` package has no dependency on `givn`; every retagged scenario has a reason; every gap has one outcome. |
| `S14` | `givn` | Promotion decision for `ise`. | The evidence table is complete; without it `ise` stays opt-in. |

The plan's stop rules are explicit: a phase does not skip its predecessor's
recorded evidence; a failing gate is reported with the command and the
observed value; a benchmark task file is never edited in place.

## 8. Open items

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
- **Presentation choices.** The abbreviated digest in the report and the
  `Receipt path: stdout` line are fixed in `S4`'s design. The command
  defaults (`--at` optional, `--dry-run`, `--receipt` to stdout when omitted)
  are settled and stated there.
