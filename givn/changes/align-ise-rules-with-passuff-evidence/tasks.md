# Tasks: align-ise-rules-with-passuff-evidence

The change edits served guidance, reference rows, glossary terms, and the two
permanent scenarios that assert the changed sentences. It adds no command,
output format, exit code, persisted state, declaration key, or verification
behavior. Every delta scenario asserts served text through the existing CLI
boundary; no `src/**` file changes.

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both already set in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` on both cucumber
builders (`tests/features_runner.rs`, `e2e/tests/e2e_runner.rs`). No runner,
framework, or fixture infrastructure is added by this change; the setup task
proves strictness once and returns to a green baseline.

Release note (subject of every behaviour batch commit, from `proposal.md`):

```text
The ise rules now require the Specification to cover every build input and every value a frozen-test contract fixes, keep the acceptance scenarios in the Specification, quarantine a published product copy before a rebuild, record every neighbor and consolidation, and leave a dry run unpromoted.
```

**Commit rule.** One atomic `givn commit` with explicit paths per batch. A
behaviour batch uses the release note as its subject; a docs or evidence batch
uses `docs(<change>): ...` or `chore(<change>): ...`. The body is
`Batch: <batch-name>`, plus the scenario title for a single-scenario batch.
Every task of a batch records the same revision as
`COMMIT: <hash> -- Batch: <batch-name>`. The commit contains the batch's
implementation or its regression test; a comment-only, task-only, or `@wip`
only change is not evidence. `@wip` is removed from a scenario when its batch
starts, never in bulk.

**Placeholders.** Every `COMMIT:` line starts with `___`. A checked line with a
placeholder, a revision that does not exist, or a revision not reachable from
`HEAD` fails the review and archive gates; fill the real identifier, never a
reword. The setup evidence pastes and ticks are committed with C2, so the plan
task records C1 while the two evidence tasks record C2.

**Evidence fields.** Per the served `tasks` instruction this file records no
test-first order and no RED/GREEN checkbox. Each batch map entry names the
planned RED proof, and each batch section carries one evidence slot where the
implementer pastes the observed run. A scenario whose requirement is already
implemented is recorded as existing behaviour protected and manufactures no
production diff.

**Commit numbering.**

| Commit | Content |
|---|---|
| C1 | plan commit: the change directory, including this file |
| C2 | batch `served-rules`, including the setup evidence pastes |
| C3 | batch `reference-and-glossary` |
| C4 | batch `verification`: suites and evidence |
| C5 | the `givn check tasks` receipt and its evidence |
| C6 | batch `review-and-archive`: `review.md`, the review transcript, and the archive result |
| C7 | post-archive suites and self-dogfood transcript |

## Batch map

| Batch | Tasks | Scenarios | Commit | Purpose |
|---|---|---|---|---|
| `setup` | 3 | 0 | C1, C2 | Plan commit, runner confirmation, strictness probe. |
| `served-rules` | 10 | 9 | C2 | Bind the new `Then` phrases, write the two permanent bodies, and add the served rules to the `ise` and `review` instructions. |
| `reference-and-glossary` | 2 | 0 | C3 | The `Rebuild rules` table and the environment-input and corpus-split sentences; the three glossary terms. |
| `verification` | 5 | 0 | C4, C5 | Nine single-scenario runs, the delta suite, the permanent suite, the E2E suite, and the change gate. |
| `review-and-archive` | 4 | 0 | C6, C7 | Independent review, archive, post-archive suites, self-dogfood. |
| **Total** | **24** | **9** | | |

Batch `served-rules` planned RED proof: after the new `Then` bindings and the
two permanent bodies are in place and before either instruction asset is edited,
the design's prefixed single-scenario command for
`The ise instruction makes the declaration cover every build input` prints
`1 feature`, `1 scenario (1 failed)`, `4 steps (3 passed, 1 failed)`, and exits
`101` on the asserted sentence, not on an unbound step; the delta batch then
fails every added scenario on its asserted sentence. A failure caused by an
unknown subcommand, an unbound step, a panic, a missing fixture, or a
non-compiling tree is not accepted as RED.

## Interaction coverage pre-flight

The two owning inventories (`givn/specs/fragments/fragment.md` for
`instruction-integrity`, `givn/specs/create-change/usecase.md` for
`prose-consolidation`) hold four entries; each has a row in `design.md`
`## Interaction Coverage Matrix` and an existing `@e2e` scenario. This change
adds no action and no `@e2e` scenario.

| Inventory entry (owning document) | Matrix row | Existing `@e2e` scenario |
|---|---|---|
| `instruction-integrity`: resolve an instruction and expose the machine contract (`fragments/fragment.md`) | yes | `An agent requests machine-readable artifact guidance` |
| `instruction-integrity`: request normal session-start instructions (`fragments/fragment.md`) | yes | `Author starts a session with the universal dialog obligation` |
| `prose-consolidation`: request canonical E2E guidance (`create-change/usecase.md`) | yes | `Canonical E2E policy is emitted by specs instructions` |
| `prose-consolidation`: synchronize and inspect generated E2E guidance (`create-change/usecase.md`) | yes | `Generated guidance points to the canonical E2E policy` |

No gap: every inventory entry has a matrix row and an existing tagged scenario.

## Domain constraints

Copied from the owning use-case documents and the proposal; every task below
carries the rows it needs.

| Capability | Guarantee or rule | Persona |
|---|---|---|
| `obtain-effective-guidance` | The text path and the machine-readable path carry the same effective content; a request never mutates project state; the `ise` instruction is served only while the `ise` addon is effective; the built-in rules stay technology-agnostic. | `agent--technical-role`, `developer--primary-user` |
| `create-change` | Component scenarios stay in the Specification unchanged; only their step definitions and fakes are implementation; the canonical `@e2e` policy states the declared `e2e` package boundary; regular and E2E executions select scope explicitly. | `developer--primary-user`, `givn-maintainer--owner` |
| `prepare-an-ise-test` | The spec copy carries the declaration and the declared Specification; a refusal leaves the checkout unchanged; the comparison exits 0 only when both subject lines say `match`. | `agent--technical-role`, `givn-maintainer--owner` |
| `instruction-integrity` | A served instruction's rules stay complete and machine-readable; every named instruction id resolves; a regression asserts focused meaningful content, never a whole-output substring. | `agent--technical-role`, `givn-maintainer--owner` |

Carried by every task: no `src/**` change; the served text is the only behavior
under test; no command, exit code, output format, persisted state, declaration
key, or verification command changes; no README change; no `@e2e` tag is added
or removed; both delta files stay tagged `@givn.delta`; no file under
`docs/arc42/` changes.

---

## Setup

- [x] Land the plan on the change branch
  - Paths: `givn/changes/align-ise-rules-with-passuff-evidence/` (proposal,
    specs, design, design-review, arc42, config-lock, tasks).
  - Run:
    ```sh
    givn commit givn/changes/align-ise-rules-with-passuff-evidence \
      -m 'docs(align-ise-rules-with-passuff-evidence): plan the change with the served-rule alignment'
    git rev-parse HEAD
    ```
  - Acceptance: exit 0; the commit holds the change directory; the 40-character
    revision printed by `git rev-parse HEAD` is C1 and is written into this
    task's `COMMIT:` line in the next commit; the tree is clean except for this
    line's later tick edit.
  - Design constraints: served `tasks` instruction commit rule; served
    `implement` instruction plan commit. No production file changes.
  - Evidence: `givn commit givn/changes/align-ise-rules-with-passuff-evidence`
    printed `committed revision a0dc6c025c4976fdd137c56e5e4101f7c2b90aa2 on the
    Git backend`; `git rev-parse HEAD` printed the same revision.
  - [x] COMMIT: `a0dc6c025c4976fdd137c56e5e4101f7c2b90aa2` -- Batch: setup

- [x] Confirm the runner configuration and the delta discovery count
  - Paths: none; paste the evidence into this task.
  - Run:
    ```sh
    sed -n '/^verify:/,/^$/p' givn/commands.yaml
    grep -n 'fail_on_skipped' tests/features_runner.rs e2e/tests/e2e_runner.rs
    GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
      cargo nextest list --locked --test features_runner | grep -c '::'
    ```
  - Acceptance: `verify.command` is `./run-tests.sh --non-e2e`;
    `verify.e2e_command` is `./run-tests.sh --e2e --package e2e`; both builders
    call `.fail_on_skipped()`; the delta discovery count is 9, all regular,
    because the change adds no `@e2e` scenario. No configuration file changes.
  - Evidence:
    ```text
    $ sed -n '/^verify:/,/^$/p' givn/commands.yaml
    verify:
      command: "./run-tests.sh --non-e2e"
      e2e_command: "./run-tests.sh --e2e --package e2e"

    $ grep -n 'fail_on_skipped' tests/features_runner.rs e2e/tests/e2e_runner.rs
    tests/features_runner.rs:55:        .fail_on_skipped()
    tests/features_runner.rs:155:        .fail_on_skipped()
    e2e/tests/e2e_runner.rs:46:        .fail_on_skipped()

    $ GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
        cargo nextest list --locked --test features_runner | grep -c '::'
    9
    ```
  - Design constraints: design `## Test runner and strict mode` and the
    design's E2E smoke test infrastructure section; the change adds no runner
    configuration.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: setup

- [x] Proof-of-strictness, once
  - Paths: `/tmp/ise-strictness-probe/` (scratch, outside the checkout).
  - Run:
    ```sh
    mkdir -p /tmp/ise-strictness-probe
    printf 'Feature: Strictness probe\n  Scenario: An unbound step fails\n    Given this step is deliberately unbound\n' \
      > /tmp/ise-strictness-probe/probe.feature
    GIVN_FEATURES=/tmp/ise-strictness-probe \
      cargo nextest run --locked --test features_runner
    ```
  - Acceptance: non-zero exit with the runner's unbound-step message
    (`Step doesn't match any function`); paste the command and output here.
    Delete the probe (`rm -rf /tmp/ise-strictness-probe`) and re-run
    `./run-tests.sh --non-e2e`; it exits 0 with the delta scenarios still
    tagged `@wip`. Do not proceed while the probe exits 0.
  - Evidence:
    ```text
    $ mkdir -p /tmp/ise-strictness-probe
    $ printf 'Feature: Strictness probe\n  Scenario: An unbound step fails\n    Given this step is deliberately unbound\n' \
        > /tmp/ise-strictness-probe/probe.feature
    $ GIVN_FEATURES=/tmp/ise-strictness-probe cargo nextest run --locked --test features_runner
        FAIL [   0.922s] (1/1) givn::features_runner probe.feature::An unbound step fails
        Feature: Strictness probe
          Scenario: An unbound step fails
           ✘  Given this step is deliberately unbound
              Step failed:
              Defined: tmp/ise-strictness-probe/probe.feature:3:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
            Summary [   0.923s] 1 test run: 0 passed, 1 failed, 0 skipped
    exit 100

    $ rm -rf /tmp/ise-strictness-probe
    $ ./run-tests.sh --non-e2e
            Summary [ 209.882s] 1108 tests run: 1108 passed, 115 skipped
    exit 0
    ```
    The 115 nextest-skipped units are the `@e2e` complement the regular tag
    expression excludes; no runner step is skipped. The first full run of this
    command failed `agent-activity-reporting.feature::A paused Maki subagent
    stays open until it finishes`, which passes in isolation (`1 scenario (1
    passed)`, exit 0) and passed in the rerun above; that scenario is unrelated
    to this change.
  - Design constraints: served `tasks` instruction proof-of-strictness rule;
    `tests/features_runner.rs` sets `.fail_on_skipped()` and fails a skipped
    step, so an empty or undefined step can never report PASS.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: setup

## Batch 1: `served-rules`

Files modified: `assets/instructions/ise.md`, `assets/instructions/review.md`,
`tests/steps/instruction_integrity_steps.rs`,
`tests/steps/prose_consolidation_steps.rs`,
`givn/specs/fragments/instruction-integrity.feature`,
`givn/specs/create-change/prose-consolidation.feature`, and the two delta
feature files (`@wip` removed). The two permanent bodies are written in place in
the same commit as the rebind, so the archive merge replaces them by title with
identical text and the pre-archive permanent scope is green.

- [x] Evidence for this batch
  - Run the single-scenario RED proof, the delta RED run, and the delta GREEN
    run; paste all three outputs here.
    ```sh
    GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
      cargo test --locked --test features_runner -- \
      'givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature::The ise instruction makes the declaration cover every build input' \
      --exact --nocapture
    GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
      ./run-tests.sh --non-e2e
    ```
  - Acceptance: the single-scenario run prints `1 feature`,
    `1 scenario (1 failed)`, `4 steps (3 passed, 1 failed)`, and exits `101`;
    its failure names the served sentence the binding asserts, never an unbound
    step. The delta RED run fails all nine scenarios on their asserted
    sentences. After the asset edits the delta run prints 9 passed, 0 failed,
    0 skipped, and exits 0. No `0 scenarios` summary is accepted as evidence.
  - Evidence:
    ```text
    # RED, bindings in place and assets unedited
    $ GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
        cargo test --locked --test features_runner -- \
        'givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature::The ise instruction makes the declaration cover every build input' \
        --exact --nocapture
    Feature: Instruction and skill guidance remains complete and machine-readable
      Scenario: The ise instruction makes the declaration cover every build input
       ✔  Given a Givn project with the ise addon enabled and the skills synchronized
       ✔  When the consumer requests the `ise` instruction
       ✔  Then the command exits 0
       ✘  And the served instruction names the build manifest, the lockfile, the toolchain pin, a structural template, a schema or migration asset, and served instruction text as build inputs
          Matched: tests/steps/instruction_integrity_steps.rs:3801:1
          Step panicked. Captured output: the served text is missing "build input": # Instruction: ise ...
    [Summary]
    1 feature
    1 scenario (1 failed)
    4 steps (3 passed, 1 failed)
    exit 101

    # Delta RED, same unedited assets
    $ GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
        ./run-tests.sh --non-e2e
       ✘  And the served instruction names the build manifest, ...
       ✘  And the served instruction names the dependency cache and the installed toolchain as environment inputs, never declared files
       ✘  Then the guidance states that the @e2e scenarios stay in the Specification with their exact titles
       ✘  And the served instruction treats an asset the rebuild generates ...
       ✘  And the served instruction records every file changed outside the declared Specification or the replaced unit
       ✘  And the served instruction requires the builder to read no product source outside the spec copy and the declared inputs
       ✘  And the served instruction states that a contract the frozen tests rely on enumerates every value it fixes
       ✘  And the served review guidance asks for the files changed outside the declared Specification or the replaced unit
       ✘  And the served instruction states that a Dry-Run Ise trial promotes nothing
         Summary [   1.587s] 9 tests run: 0 passed, 9 failed, 0 skipped
    exit 100

    # Delta GREEN after the asset edits
    $ GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
        ./run-tests.sh --non-e2e
         Summary [   1.647s] 9 tests run: 9 passed, 0 skipped
    exit 0
    ```
    The first delta GREEN attempt failed one step bound to "not a specification
    defect": the served sentence carries the term in backticks, so the binding
    asserts "it is not a `specification defect`", which keeps the term and the
    required classification.
  - Design constraints: design `## Verification plan` items 1 to 3 and the
    batch-map RED proof; the bindings land before the asset edits so the RED is
    the missing sentence, not a missing binding.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction makes the declaration cover every build input
  - Design constraints: design `### Specification completeness bullet` and
    `### Vocabulary paragraph`; decision D2. Every behavior, contract, limit,
    acceptance test, and build input is declared and reachable; each build
    input sits under the role that owns its content class (build manifest and
    lockfile under `contracts`, toolchain pin under `limits`, structural
    template under `structure`, schema or migration asset under `contracts`,
    served instruction or skill text under `behavior`); an input the
    declaration leaves outside the Specification roles is a
    `specification defect`, and a neighbor when it lies outside the replaced
    unit. Append the vocabulary sentence naming the three glossary terms.
  - Asserted text: `build input`, `a build manifest and a lockfile`, `a
    toolchain pin`, `a structural template`, `a schema or migration asset`,
    `served instruction or skill text`; the four role keys `contracts`,
    `limits`, `structure`, `behavior` beside the named classes; `leaves outside
    the Specification roles`, `specification defect`, `a neighbor when it lies
    outside the replaced unit`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`,
    `givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature`
    (`@wip` removed).
  - Acceptance: both bound `Then` phrases assert the listed substrings; the
    scenario passes; the five obligation labels keep their em dashes.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction accepts an asset the rebuild generates inside the replaced unit
  - Design constraints: design `### Specification completeness bullet`; decision
    D2. An asset the rebuild generates inside the replaced unit whose bytes no
    frozen test fixes is an `acceptable undocumented change`, not a neighbor.
    The boundary sentence sits in the same bullet before the environment-input
    sentence.
  - Asserted text: `whose bytes no frozen test fixes`, `acceptable
    undocumented change`, `not a neighbor`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`, the delta feature (`@wip`
    removed).
  - Acceptance: both bound phrases assert the listed substrings; the scenario
    passes; the generated-asset classification is not applied to a declared
    build input.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction keeps the dependency cache in the environment
  - Design constraints: design `### Specification completeness bullet`; decision
    D2. The dependency cache and the installed toolchain are environment
    inputs, never declared files; the lockfile declares the resolved
    dependencies and the toolchain pin declares the toolchain; a locked
    dependency the cache cannot resolve blocks the rebuild, and the blocked
    rebuild is a setup failure, not a `Spec gap` and not a
    `specification defect`.
  - Asserted text: `environment inputs, never declared files`, `a locked
    dependency the cache cannot resolve blocks the rebuild`, `the blocked
    rebuild is a setup failure`, `Spec gap`, `not a specification defect`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`, the delta feature (`@wip`
    removed).
  - Acceptance: all four bound phrases assert the listed substrings; the
    scenario passes; the setup-failure classification is stated once.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction requires a relied-on contract to enumerate its fixed values
  - Design constraints: design `### Specification completeness bullet`; decision
    D3. A contract the frozen tests rely on enumerates every value it fixes:
    literal text, headings, labels, error prefixes, exit codes, file formats,
    and test seams. The list stays in the reference; the instruction states the
    obligation once.
  - Asserted text: `enumerates every value it fixes`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`, the delta feature (`@wip`
    removed).
  - Acceptance: the bound phrase asserts the sentence; the scenario passes; the
    instruction does not grow a technology-specific value list.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction keeps a published copy out of the clean room
  - Design constraints: design `### Clean-room boundary section`; decision D5.
    The builder reads no product source outside the spec copy and the declared
    inputs; before the rebuild it scans the local package registry's source,
    cache, and git stores for a published copy of the product, quarantines
    every hit outside its reach, treats an absent store as no hit, and records
    the scan result in the neighbor record. Insert the section after
    `## The spec copy rule` and before `## Obligations`.
  - Asserted text: `reads no product source outside the spec copy and the
    declared inputs`, `scan the local package registry's source, cache, and git
    stores`, `quarantine every hit outside the builder's reach`, `a store that
    does not exist has no hit`, `Record the scan result in the neighbor
    record`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`, the delta feature (`@wip`
    removed).
  - Acceptance: all five bound phrases assert the listed substrings; the
    scenario passes; the section names no product-specific package manager.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction records a rebuild's neighbors and consolidations
  - Design constraints: design `### Records table`; decision D6. The sentence
    reads `An ise change carries three records:`; the third row is
    `Neighbor record` with its home and content; a consolidation merges or
    splits rebuilt paths and is recorded with its reason even when no file lies
    outside the Specification; a rebuild with no neighbor records an empty
    table; the trial review records the neighbor record. The five obligation
    labels and the two existing records stay unchanged.
  - Asserted text: `every file changed outside the declared Specification or the
    replaced unit`, `every consolidation, each with its reason`, `the trial
    review records the neighbor record`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`, the delta feature (`@wip`
    removed).
  - Acceptance: all three bound phrases assert the listed substrings; the
    scenario passes; the instruction still carries five obligations and three
    records.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The ise instruction keeps a dry run out of the product
  - Design constraints: design `### Dry-run sentence`; decision D7. Insert the
    sentence at the end of the `## The spec copy rule` paragraph, before the
    new clean-room section: a `Dry-Run Ise` trial promotes nothing, the rebuilt
    work stays in the copy, and the trial review records that no rebuilt file
    was promoted.
  - Asserted text: `promotes nothing`, `the rebuilt work stays in the copy`.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/instruction_integrity_steps.rs`, the delta feature (`@wip`
    removed).
  - Acceptance: both bound phrases assert the listed substrings; the scenario
    passes; the sentence states no promotion command.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: The review guidance asks the neighboring-components question without a gate
  - Design constraints: design `## Exact review guidance text` and
    `## New Then phrases and asserted text` (the rephrased binding). Replace the
    `Neighboring components` bullet in `assets/instructions/review.md` with the
    exact bullet. In `tests/steps/instruction_integrity_steps.rs`, keep the
    function `review_guidance_asks_neighboring_components` and its text
    normalization, register it for the phrase `the served review guidance asks
    for the files changed outside the declared Specification or the replaced
    unit`, and assert that sentence; add the bindings for `every consolidation
    the rebuild made` and `the reason for each`. Write the permanent body in
    `givn/specs/fragments/instruction-integrity.feature` in place to the
    delta's `@givn.modified` body (same title, same four `Then` steps), in this
    commit, so the archive merge replaces it with identical text.
  - Asserted text: `files changed outside the declared Specification or the
    replaced unit`, `every consolidation the rebuild made`, `the reason for
    each`, `review question, never a gate`.
  - Paths: `assets/instructions/review.md`,
    `tests/steps/instruction_integrity_steps.rs`,
    `givn/specs/fragments/instruction-integrity.feature`, the delta feature
    (`@wip` removed).
  - Acceptance: the four `Then` steps pass; the permanent scenario passes
    without an unbound step; the `givn ise rebuild-component` rows carrying the
    pre-existing `neighbouring components` wording stay unchanged.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

- [x] Scenario: Served ise guidance keeps component scenarios in the Specification
  - Design constraints: design `### two-layer test boundary bullet`; decision
    D4. Replace the second obligation in `assets/instructions/ise.md`: the
    `@e2e` scenarios stay in the Specification under the `behavior` role with
    their exact titles and run from the declared `e2e` package, which holds the
    runner and every `@e2e` step definition and survives a rebuild; component
    scenarios stay in the Specification unchanged, and only their step
    definitions and fakes are implementation and may be rewritten inside the
    component boundary. Add the two `Then` bindings in
    `tests/steps/prose_consolidation_steps.rs`. Write the permanent body in
    `givn/specs/create-change/prose-consolidation.feature` in place to the
    delta's `@givn.modified` body (same title, same five `Then` steps), in this
    commit, so the archive merge replaces it with identical text.
  - Asserted text: `stay in the Specification under the behavior role with
    their exact titles`, `the declared e2e package, which holds the runner and
    every @e2e step definition`; the three existing assertions stay.
  - Paths: `assets/instructions/ise.md`,
    `tests/steps/prose_consolidation_steps.rs`,
    `givn/specs/create-change/prose-consolidation.feature`, the delta feature
    (`@wip` removed).
  - Acceptance: all five `Then` steps pass; the permanent scenario passes
    without an unbound step; no `@e2e` tag is added or removed.
  - [x] COMMIT: `9f99bb367b66cb87ec4e3cc3dc353205f00a5de2` -- Batch: served-rules

## Batch 2: `reference-and-glossary`

Files modified: `docs/reference.md`, `docs/glossary.md`. No delta scenario
asserts these rows directly; the batch is documentation that states what the
instruction requires. It is committed after the served text exists so the
reference and the instruction cannot disagree at review time.

- [x] Add the rebuild rules and the two sentences to the reference
  - Paths: `docs/reference.md`.
  - Change: insert the `**Rebuild rules.**` paragraph and its six-row table
    (Build inputs, Fixed values, Acceptance corpus, Clean-room boundary,
    Neighbor record, Dry-Run Ise) immediately after the
    `**The rule the copy carries.**` paragraph and before `**Manifest.**`;
    append the environment-input paragraph after the table; append the
    corpus-split sentence to the `**End-to-end enumeration.**` paragraph.
    Copy the text from design `## Reference rows` exactly, including the full
    `Build inputs` row.
  - Acceptance: the table has six rows; `An environment input is never
    declared:` names the lockfile, the toolchain pin, and the setup-failure
    classification; the enumeration paragraph ends with the sentence that the
    scenarios stay in the `behavior` role and the `e2e` role holds the runner
    and every `@e2e` step definition. No other reference section changes.
  - Design constraints: design `## Reference rows`; the reference states the
    value classes, the role home, and the scan roots; the instruction keeps the
    rule once.
  - [x] COMMIT: `787dcaf4ce6ac8c92138cde03469e161328d9209` -- Batch: reference-and-glossary

- [x] Add the three glossary terms
  - Paths: `docs/glossary.md`.
  - Change: insert `Build input` after the `Specification` row, `Neighbor
    record` before the `Spec gap` row, and `Replaced unit` after the `Spec
    copy` row, using the design's exact rows. Each row carries its `not`
    contrast and anti-term.
  - Acceptance: the three rows are present in the named positions;
    `Neighbor record` contrasts with `Change verification receipt`; `Build
    input` and `Replaced unit` contrast with `Source code` and `Component`;
    no other row changes.
  - Design constraints: design `## Glossary terms`; decision D8; the glossary
    is the vocabulary authority and chapter 12 links it.
  - [x] COMMIT: `787dcaf4ce6ac8c92138cde03469e161328d9209` -- Batch: reference-and-glossary

## Batch 3: `verification`

The batch runs the design's verification plan items 4 to 7. The `givn check
tasks` run needs a clean committed tree before it starts, so C4 lands the suite
evidence first; C5 lands the receipt and the gate evidence.

- [x] Run the nine prefixed single-scenario commands
  - Paths: none; paste the nine summaries in this task.
  - Run the design's exact command, changing only the feature path and the
    scenario title:
    ```sh
    GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
      cargo test --locked --test features_runner -- \
      'givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature::The ise instruction makes the declaration cover every build input' \
      --exact --nocapture
    ```
    Use double quotes around the filter for
    `The ise instruction records a rebuild's neighbors and consolidations`
    because the title carries an apostrophe.
  - Titles: the seven added `instruction-integrity` scenarios from batch 1, the
    modified `The review guidance asks the neighboring-components question
    without a gate`, and the modified `Served ise guidance keeps component
    scenarios in the Specification`.
  - Acceptance: each run prints `1 feature`, `1 scenario (1 passed)`, 0 failed,
    0 skipped, and exits 0. A `0 scenarios` summary, an unbound step, or a
    skip fails this task. Paste the nine summaries.
  - Evidence:
    ```text
    The ise instruction makes the declaration cover every build input
      1 feature / 1 scenario (1 passed) / 6 steps (6 passed) / exit 0
    The ise instruction accepts an asset the rebuild generates inside the replaced unit
      1 feature / 1 scenario (1 passed) / 5 steps (5 passed) / exit 0
    The ise instruction keeps the dependency cache in the environment
      1 feature / 1 scenario (1 passed) / 7 steps (7 passed) / exit 0
    The ise instruction requires a relied-on contract to enumerate its fixed values
      1 feature / 1 scenario (1 passed) / 4 steps (4 passed) / exit 0
    The ise instruction keeps a published copy out of the clean room
      1 feature / 1 scenario (1 passed) / 8 steps (8 passed) / exit 0
    The ise instruction records a rebuild's neighbors and consolidations
      1 feature / 1 scenario (1 passed) / 6 steps (6 passed) / exit 0
    The ise instruction keeps a dry run out of the product
      1 feature / 1 scenario (1 passed) / 5 steps (5 passed) / exit 0
    The review guidance asks the neighboring-components question without a gate
      1 feature / 1 scenario (1 passed) / 7 steps (7 passed) / exit 0
    Served ise guidance keeps component scenarios in the Specification
      1 feature / 1 scenario (1 passed) / 7 steps (7 passed) / exit 0
    ```
    All nine exited 0; none printed `0 scenarios` or an unbound step.
  - Design constraints: design `## Test runner and strict mode`; every command
    names its corpus through `GIVN_FEATURES`; the bare runner cannot assert a
    count, so the printed summary is the evidence.
  - [x] COMMIT: `8bb50f67ed1821d4b2c6186334620c5b313affa6` -- Batch: verification

- [x] Run the delta suite
  - Run:
    ```sh
    GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
      ./run-tests.sh --non-e2e
    ```
  - Acceptance: exit 0; 9 scenarios passed, 0 failed, 0 skipped; the printed
    summary names 9 scenarios. Paste the run's summary and the exit code.
  - Evidence:
    ```text
    $ GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs \
        ./run-tests.sh --non-e2e
         Summary [   1.612s] 9 tests run: 9 passed, 0 skipped
    exit 0
    ```
  - Design constraints: design `## Verification plan` item 2; the delta batch
    is the change's own behavior corpus.
  - [x] COMMIT: `8bb50f67ed1821d4b2c6186334620c5b313affa6` -- Batch: verification

- [x] Run the permanent scope before archive
  - Run:
    ```sh
    GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e
    ```
  - Acceptance: exit 0; a non-zero scenario count; 0 failed, 0 skipped; the two
    replaced permanent scenario bodies pass. Paste the run's summary and the
    exit code. A red scenario or a `0 scenarios` summary fails this task.
  - Evidence:
    ```text
    $ GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e
         Summary [ 209.097s] 1108 tests run: 1108 passed, 115 skipped
    exit 0
    ```
    The 115 nextest-skipped units are the `@e2e` complement the regular tag
    expression excludes; the runner reported no skipped step and both replaced
    scenario bodies passed.
  - Design constraints: design `## Verification plan` item 5; the permanent
    bodies are written in batch 1 so no permanent scenario carries an unbound
    step before archive.
  - [x] COMMIT: `8bb50f67ed1821d4b2c6186334620c5b313affa6` -- Batch: verification

- [x] Run the E2E scope before archive
  - Run:
    ```sh
    GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e
    ```
  - Acceptance: exit 0; a non-zero total; 0 failed, 0 skipped. No `@e2e`
    scenario asserts a changed sentence; the suite still must be green. Paste
    the summary and the exit code.
  - Evidence:
    ```text
    $ GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e
    [Summary]
    56 features
    120 scenarios (120 passed)
    784 steps (784 passed)
    exit 0
    ```
  - Design constraints: design `## Verification plan` item 6 and the design's
    E2E smoke test infrastructure section; the change adds no runner
    configuration and no `@e2e` scenario.
  - [x] COMMIT: `8bb50f67ed1821d4b2c6186334620c5b313affa6` -- Batch: verification

- [ ] Run the change gate
  - Precondition: the batch 3 suite tasks are ticked, their evidence is pasted,
    and C4 is committed; the tree is clean (`git status --porcelain` prints
    nothing) because `run_change_verification` refuses a dirty tree. Every one
    of the nine scenario tasks carries a checked `COMMIT:` line with a real
    revision reachable from `HEAD`.
  - Run:
    ```sh
    givn check tasks --change align-ise-rules-with-passuff-evidence
    ```
  - Acceptance: exit 0; `changed-scenario verification` runs with the delta
    scope and a non-zero scenario count; the receipt at
    `givn/changes/align-ise-rules-with-passuff-evidence/verification/tasks-result.json`
    records `total: 9`; `RunnerResult::read_counts` does not refuse it with
    `declares zero scenarios`. The `⚠ task preflight: review.md is missing`
    warning is expected until batch 4 writes `review.md`. Paste the output and
    the receipt's counts. The receipt and this task's tick are committed as C5.
  - Evidence:
    ```text
    $ givn check tasks --change align-ise-rules-with-passuff-evidence
    [Summary]
    2 features
    9 scenarios (9 passed)
    55 steps (55 passed)
      ✓ change verification receipt: 9 scenario(s), 0 failed, 0 skipped at 8bb50f67ed1821d4b2c6186334620c5b313affa6
      ✓ E2E verification: not_applicable
      ✓ verify passed
    exit 0

    verification/tasks-result.json
    {"failed":0,"passed":9,"scope":"change","skipped":0,"total":9}

    verification/tasks-verification.json
    "status": "passed", "head": "8bb50f67ed1821d4b2c6186334620c5b313affa6",
    "result": {"scope": "change", "total": 9, "passed": 9, "failed": 0, "skipped": 0},
    "e2e_scope": {"status": "not_applicable"}
    ```
  - Design constraints: design `## Verification plan` item 4; the change gate
    is the receipt the review and archive read.
  - [ ] COMMIT: `___` -- Batch: verification

## Batch 4: `review-and-archive`

- [ ] Write `review.md` and pass the review gate
  - Paths:
    `givn/changes/align-ise-rules-with-passuff-evidence/review.md`; paste the
    transcript in this task.
  - Run:
    ```sh
    givn lint --change align-ise-rules-with-passuff-evidence
    givn check review --change align-ise-rules-with-passuff-evidence
    ```
  - Acceptance: `review.md` records `Review kind: independent`, `Author:`, and
    `Reviewer context: independent-design-review-2026-11-17` (the context the
    design review records); `README-IMPACT: none`; the `PERSONAS:` line; the
    design-match check; the `## Overlap dispositions` table with `variant` for
    each shape-match pair the gate reports, naming the distinct invariant
    (build-input coverage, generated-asset classification, environment inputs,
    fixed values, quarantine, neighbor record, or dry run), and `keep` for any
    scenario over the split threshold with its reason; and the `[SUBST]`
    advisories recorded as the existing shared-setup pattern with no split.
    `givn lint` exits 0 with no WIP finding. `givn check review` exits 0. The
    transcript and `review.md` are committed as C6.
  - Design constraints: design `## Risks and tradeoffs` (independent review)
    and `## Delta scenarios and rebinding` (no split; each added scenario
    protects its distinct invariant); the review instruction requires the
    machine-checkable markers and the gate's disposition vocabulary.
  - [ ] COMMIT: `___` -- Batch: review-and-archive

- [ ] Run `givn archive` and record the receipts
  - Paths: `givn/archive/align-ise-rules-with-passuff-evidence/`.
  - Run:
    ```sh
    givn archive --change align-ise-rules-with-passuff-evidence
    givn commit -m 'docs(align-ise-rules-with-passuff-evidence): record the independent review and archive the change'
    ```
  - Acceptance: `givn archive` exits 0; the archive holds `verification.json`
    and the archive result files; the seven added scenarios are merged into
    `givn/specs/fragments/instruction-integrity.feature`; the two modified
    bodies are replaced by title with identical text; the permanent and E2E
    scopes run by the archive gate are green. The commit stages the archive
    directory, the change-directory removal, and the review receipt together;
    run it with no path.
  - Design constraints: design `## Verification plan` item 8; the archive gate
    merges the delta and runs both scopes against the merged candidate; the
    change adds no promotion beyond the corpus merge.
  - [ ] COMMIT: `___` -- Batch: review-and-archive

- [ ] Run the post-archive suite scopes
  - Run:
    ```sh
    GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e
    GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e
    ```
  - Acceptance: both exit 0; non-zero counts; 0 failed, 0 skipped. Paste both
    summaries and exit codes into
    `givn/archive/align-ise-rules-with-passuff-evidence/evidence/post-archive.txt`.
    Commit the transcript as C7.
  - Design constraints: design `## Verification plan` item 8; the merged
    permanent corpus is the tested tree.
  - [ ] COMMIT: `___` -- Batch: review-and-archive

- [ ] Run the self-dogfood sequence
  - Paths:
    `givn/archive/align-ise-rules-with-passuff-evidence/evidence/self-dogfood.txt`.
  - Run:
    ```sh
    set -o pipefail
    { cargo install --path . --locked --force
      givn skills sync
      givn instructions
      ./run-tests.sh
      ./run-tests.sh --e2e --package e2e
    } 2>&1 | tee givn/archive/align-ise-rules-with-passuff-evidence/evidence/self-dogfood.txt
    ```
  - Acceptance: every command exits 0; the installed binary reports the
    checkout's `Cargo.toml` version; the transcript is committed as C7.
  - Design constraints: `AGENTS.md` `## Self-dogfood after each change`;
    design `## Verification plan` item 8. This is a post-archive maintenance
    task, not a delta scenario.
  - [ ] COMMIT: `___` -- Batch: review-and-archive

## Closeout: merged step definitions

No delta step copy is authored. Every new `Then` phrase is bound in the
permanent per-capability files (`tests/steps/instruction_integrity_steps.rs`,
`tests/steps/prose_consolidation_steps.rs`), and the rephrased review binding
replaces its old phrase in the same file. At archive time no delta phrase lacks
a permanent binding, and `givn archive`'s merged-step check has nothing to
refuse.
