# Review: align-ise-rules-with-passuff-evidence

Review kind: independent
Author: align-ise-rules-with-passuff-evidence author
Reviewer context: independent-design-review-2026-11-17

PERSONAS: developer--primary-user

ARC42 CONFORMANCE: CLEAN

README-IMPACT: none

## Scope and review context

Independent review in one fresh context. The reviewer did not author the change
and received no author conversation. Reviewed the committed tree at `543ceded`
on `ise/implementation`; the design review at `95b83225` covers the plan with
the change directory uncommitted.

Artifacts read: `proposal.md`, `design.md`, `design-review.md`, `arc42.md`,
`tasks.md`, both delta feature files, both permanent scenario bodies,
`assets/instructions/ise.md`, `assets/instructions/review.md`, the
`docs/reference.md` rebuild rules, the three `docs/glossary.md` rows, the two
modified regular step-definition files, the two e2e step-definition files, the
e2e runner wiring, `run-tests.sh`, `givn/commands.yaml`, and
`docs/arc42/04-solution-strategy.md` plus `docs/arc42/12-glossary.md`.

The change is major under the review-context rule: it touches two capabilities
and asserts its own served text, and the operator requested an independent
design review. This record reuses the design review's reviewer context, as
`src/review_context.rs` requires for one independent review.

## Implementation evidence check

- **`@e2e` tag integrity.** Both delta files are tagged `@givn.delta`; nine
  scenarios carry `@givn.added` or `@givn.modified`; no `@e2e` tag was added,
  removed, or changed. The change adds no user action.
- **Step bodies.** 0 empty or trivial step bodies found across the two modified
  step-definition files (`tests/steps/instruction_integrity_steps.rs`,
  `tests/steps/prose_consolidation_steps.rs`): no `{}`, `todo!`,
  `unimplemented!`, `panic!("not implemented")`, or assertion-free body. Every
  new `Then` calls `assert_served_text_contains` or asserts a `loaded_guidance`
  substring with a named failure message.
- **Scenario commits.** All nine scenario tasks record `9f99bb36` with
  `Batch: served-rules`. `git show --stat 9f99bb36` carries the two served
  instruction assets, both step-definition files, both permanent bodies, and
  both delta files. `givn check tasks` validated revision reachability and the
  per-scenario commits; the receipt records 9 scenarios at `543ceded`.
- **Promised components.** `design.md` promises no new component: the change
  edits served text consumed through `src/instructions.rs` and the existing
  `src/ise` contract. No `src/**` or `e2e/src/**` file changes
  (`git diff --stat 95b83225..HEAD -- src/ e2e/src/` is empty). The two
  reference/glossary documents, both step files, both permanent bodies, and the
  two delta files exist as designed.
- **Strict-mode proof.** `tasks.md` records the unbound-step probe at
  `/tmp/ise-strictness-probe` failing with `Step doesn't match any function` and
  exit `100`; the probe is deleted. Both builders call `.fail_on_skipped()`
  (`tests/features_runner.rs:55,155`, `e2e/tests/e2e_runner.rs:46`). The regular
  runner exits nonzero when `failed > 0` or `skipped > 0`.
- **E2E assertions and driver.** No delta `@e2e` scenario exists. The four
  existing inventory scenarios assert CLI stdout through the real binary:
  `the user runs` resolves to `common::run_givn_with_envs`, and `An agent
  requests machine-readable artifact guidance` uses `givn instructions specs
  --json`. No scenario asserts only repository state.
- **Invoked E2E files.** `givn/commands.yaml` binds `verify.e2e_command` to
  `./run-tests.sh --e2e --package e2e`. That command runs the standalone
  `e2e/` package (`e2e/tests/e2e_runner.rs`) over the `@e2e` scenarios in
  `givn/specs`. `run-tests.sh` sets `GIVN_RESULT_SCOPE=e2e` for that scope and
  excludes `@e2e` and `@givn.removed` from the regular scope. A tree-wide
  search for a second e2e implementation for these capabilities found only
  `e2e/tests/steps/instruction_integrity_e2e_steps.rs` and
  `e2e/tests/steps/prose_consolidation_e2e_steps.rs`; no parallel weaker
  implementation exists.
- **Scope isolation.** The change receipt's scopes are `change` (9 scenarios,
  regular tags) and `e2e` (`not_applicable`, because the delta carries no
  `@e2e` scenario). The independent e2e suite reports 56 features, 120
  scenarios, 784 steps, all passed.

## Design match

`design.md`'s named values match the as-built tree:

| Design decision | As-built evidence |
|---|---|
| `verify.command` = `./run-tests.sh --non-e2e` | `givn/commands.yaml` |
| `verify.e2e_command` = `./run-tests.sh --e2e --package e2e` | `givn/commands.yaml` |
| New `Then` bindings in the permanent per-capability step files | Both files modified in `9f99bb36`; no delta step copy |
| E2E step files under `e2e/tests/steps/` | Both files exist and drive `run_givn` |
| `cucumber` 0.23 in `e2e/Cargo.toml` | `e2e/Cargo.toml:16` |
| Every exact instruction block, the review bullet, the records row, the clean-room section, the dry-run sentence, the six reference rows, and the three glossary rows | Present verbatim; compared after whitespace normalization against the design's fenced blocks (all true) |
| Two permanent bodies written to the delta text | Both are identical line-by-line to their delta `@givn.modified` bodies |
| No `src/**`, no `docs/arc42/`, no README edit | `git diff --stat 95b83225..HEAD` excludes them |

No deviation from the reviewed design exists, so no design update and no
design-review rerun was required.

## Interaction coverage

| Inventory entry (owning document) | Matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `instruction-integrity`: resolve an instruction and expose the machine contract (`fragments/fragment.md`) | yes | `An agent requests machine-readable artifact guidance` | `run_givn` invokes `givn instructions specs --json`; asserts stdout JSON |
| `instruction-integrity`: request normal session-start instructions (`fragments/fragment.md`) | yes | `Author starts a session with the universal dialog obligation` | `run_givn` invokes `givn instructions`; asserts stdout |
| `prose-consolidation`: request canonical E2E guidance (`create-change/usecase.md`) | yes | `Canonical E2E policy is emitted by specs instructions` | `the user runs` invokes the built binary; asserts stdout |
| `prose-consolidation`: synchronize and inspect generated E2E guidance (`create-change/usecase.md`) | yes | `Generated guidance points to the canonical E2E policy` | `givn skills sync` through the built binary; asserts generated files and stdout |

Every inventory entry has a matrix row and one existing `@e2e` scenario; the
change adds no inventory entry and no `@e2e` action.

## Mutation probes

Each probe edits one sentence in the served text, runs the named scenario, and
reverts with `git checkout --`; `git status --porcelain` was empty after each
revert. All probes failed on the asserted sentence, never on an unbound step.

| # | Mutation | Scenario | Observed failure |
|---|---|---|---|
| A | Delete the `Dry-Run Ise` sentence from `assets/instructions/ise.md` | `The ise instruction keeps a dry run out of the product` | `And the served instruction states that a Dry-Run Ise trial promotes nothing`; `the served text is missing "promotes nothing"`; exit 101 |
| B | Delete `A contract the frozen tests rely on enumerates every value it fixes.` | `The ise instruction requires a relied-on contract to enumerate its fixed values` | missing `enumerates every value it fixes`; exit 101 |
| C | Delete `, not a neighbor` from the generated-asset sentence | `The ise instruction accepts an asset the rebuild generates inside the replaced unit` | first two `Then` steps passed; `the served instruction states that a generated asset is not a neighbor` failed with missing `not a neighbor`; exit 101 |
| D | Delete `and every consolidation the rebuild made` from the review bullet | `The review guidance asks the neighboring-components question without a gate` | missing `every consolidation the rebuild made`; exit 101 |
| E | Delete `a schema or migration asset under \`contracts\`` | `The ise instruction makes the declaration cover every build input` | first `Then` failed with missing `a schema or migration asset`; exit 101 |
| E2 | Change the schema asset role to `limits` | `The ise instruction makes the declaration cover every build input` | first `Then` passed; second `Then` failed with missing `a schema or migration asset under \`contracts\``; exit 101 |
| F | Delete `under the \`behavior\` role with their exact titles` | `Served ise guidance keeps component scenarios in the Specification` | missing the behavior-role exact-title sentence; exit 101 |

The probe set covers both capabilities, both newly modified permanent
scenarios, and both new text locations (instruction and review asset). The
blocked-rebuild binding's second needle (`Spec gap`) also occurs elsewhere; its
discriminating needle is `the blocked rebuild is a setup failure`, which the
sentence deletion removes, so the row still discriminates its claim.

## Permanent bodies and served-rule consistency

Both permanent scenario bodies are byte-identical (modulo indentation) to their
delta `@givn.modified` bodies, so the archive merge is idempotent for them and
the pre-archive permanent suite is honestly green.

- The build-input obligation, the reference `Build inputs` row, and the spec
  copy rule agree: a declared build input is carried and checksummed; an input
  outside the Specification roles is a `specification defect` and a neighbor
  outside the replaced unit; a generated asset inside the unit whose bytes no
  frozen test fixes is an acceptable undocumented change, not a neighbor.
- The environment-input paragraph matches the manifest/compare semantics: the
  cache and toolchain are supplied, not declared; an unresolvable locked
  dependency is a setup failure, not a `Spec gap` and not a `specification
  defect`.
- The acceptance-corpus rule and the reference's `End-to-end enumeration`
  sentence agree: scenarios stay under `behavior`, the declared `e2e` package
  holds the runner and every `@e2e` step definition.
- The neighbor/consolidation rule appears once in the instruction, once in the
  reference, and as one review question; it is not a gate.
- The dry-run rule matches the existing command surface: no promotion command
  is named.
- The served text names no language, package manager, build tool, or manifest
  file; it states classes and roles only. The reference's two example template
  paths are project paths, not language specifics.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Chapter 12 glossary holds the three new terms | `docs/glossary.md` rows `Build input`, `Neighbor record`, `Replaced unit` | Row 12 Yes; chapter 12 links the glossary and no chapter file is edited | `## Glossary terms` gives the exact rows; D8 records the link-only route | Task `Add the three glossary terms` (C3) | The three rows exist at `docs/glossary.md:147,150,152`, verbatim to design; `docs/arc42/12-glossary.md` still links the glossary | Yes |
| Chapter 4 fact: a `Dry-Run Ise` trial promotes nothing | `docs/arc42/04-solution-strategy.md:33` | Row 4 No; the durable statement already holds | D7 and the dry-run sentence state the same rule | Task `The ise instruction keeps a dry run out of the product` | Instruction sentence plus scenario; durable chapter unchanged and consistent | Yes |
| No other chapter is affected | `docs/arc42/01` to `11` unchanged | Rows 1-11 No | `## Architecture impact` names no chapter edit | Carried constraint: no file under `docs/arc42/` changes | `git diff --stat 95b83225..HEAD -- docs/arc42` is empty | Yes |
| Chapter 12 remains a link, not a copied term table | `docs/arc42/12-glossary.md` | Row 12 reason | D8 rejects repeating the rows in the chapter | Glossary task only | Chapter 12 contains the link and no term rows | Yes |

- [x] Every chapter marked affected by the review or `arc42.md` was opened and
      checked for stale, contradictory, or placeholder content.
- [x] Every Arc42 fact or decision relevant to this change is represented in
      `design.md` and mapped to a task in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; the
      evidence names the built file, row, or scenario.
- [x] No architecture change was discovered in the completed implementation.
- [x] No mismatch remains, so no documentation, design, task, or
      implementation fix was needed and no comparison repeat was triggered.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language and structural model

`Build input`, `Neighbor record`, and `Replaced unit` are recorded in
`docs/glossary.md` with their contrasts and anti-terms. The served `ise`
instruction names the three terms in its vocabulary paragraph; the reference
table and the scenarios reuse them. `spec copy`, `manifest`, `checksum`,
`Spec gap`, `Specification file`, and `Specification role` retain their existing
meanings.

Structural model: unchanged and correct. The three terms are process records,
not aggregates; `docs/domain-model.md` and its rendered views gain no entity or
relationship, and the change edits no structural document.

One pre-existing spelling difference remains outside the new sentences:
`assets/instructions/ise.md:26` and `README.md:105` say "neighbouring
components" while the new text uses "neighbor". It predates this change, the
design records that the command-table rows are outside the changed sentence
set, and the new text uses the glossary spelling consistently. No action.

## Persona dispositions and visual contract

- `developer--primary-user`: accepted. The declaration and record rules let the
  developer read what to declare and what to record before a rebuild; asserted
  by `The ise instruction makes the declaration cover every build input` and
  `The ise instruction records a rebuild's neighbors and consolidations`.
- `agent--technical-role`: accepted. The builder reads the role home, the
  quarantine obligation, and the environment-input boundary; asserted by
  `The ise instruction keeps a published copy out of the clean room` and
  `The ise instruction requires a relied-on contract to enumerate its fixed
  values`.
- `givn-maintainer--owner`: accepted. The reference and glossary give the
  maintainer the value classes, role home, and scan roots; the reference table
  and glossary rows are present.

The design declares `Interface: none`; the change renders no screen, terminal
transcript, HTML, CSS, or JavaScript. The conditional visual review block
therefore does not apply, no screenshot or transcript is required, and no
Persona finding exists. No Persona was invented.

## Change strategy and Spec gap

`Decides D1` records `incremental modification`; no `Ise Test` runs and no
component or product is rebuilt. No frozen-test difference was observed, so no
`Spec gap` outcome arises and no fix followed a classified difference. The
change strategy value is one of the three recorded values.

## Coverage classification

The change adds no `src/**` production region, so there is no new instrumented
line or branch in the product. The delta scenarios exercise the two served
assets and the review bullet through the real CLI boundary; the reference and
glossary rows are documentation that state the same rules. Classification:

1. Dead code: none added; no dead port or contract member.
2. Missing test coverage: none; every new sentence the scenarios claim has a
   discriminating binding (probes A-F).
3. Legitimately hard to test: none; the served text is asserted through the
   existing CLI boundary.

The archive verification receipt will carry the measured coverage for the
merged candidate.

## Overlap dispositions

`givn lint --change align-ise-rules-with-passuff-evidence` reported no `[SHAPE]`
finding. It reported `[SUBST]` subset advisories against
`verify-change/visual-review-gate.feature`, which is the shared ise setup the
existing ise scenarios also produce; no row is required.

| Scenario A | Scenario B | Disposition |
|---|---|---|

## Split-or-keep

No delta scenario exceeds the 14-step lint threshold; the longest declares 8
steps. No split is needed.

| Scenario | Decision |
|---|---|

## Verification gates

| Gate | Result |
|---|---|
| `givn lint --change align-ise-rules-with-passuff-evidence` | exit 0; `2 file(s) checked — clean`; only `[SUBST]` advisories |
| `GIVN_FEATURES=<change specs> ./run-tests.sh --non-e2e` | exit 0; 9 tests, 9 passed, 0 skipped; 55 steps passed |
| `givn check tasks --change align-ise-rules-with-passuff-evidence` | exit 0; change receipt 9 scenario(s), 0 failed, 0 skipped at `543ceded`; E2E verification `not_applicable`; `verify passed` |
| `./run-tests.sh --non-e2e` (payload tree) | exit 0; 1108 tests run, 1108 passed; 115 nextest units are the `@e2e` complement the regular tag expression excludes, no runner step is skipped |
| `./run-tests.sh --e2e --package e2e` (payload tree) | exit 0; 56 features, 120 scenarios (120 passed), 784 steps (784 passed) |
| `givn check review --change align-ise-rules-with-passuff-evidence` | exit 0; `net delta: 7 added, 2 modified, 0 removed`; overlap dispositions, scenario commits (9), and the change receipt at `543ceded` all passed |

The e2e run regenerated six archived `development-timeline` screenshots; all
six were restored before this record was committed.

## Findings and dispositions

No blocking finding. No fix was made to assets, docs, specs, steps, or code by
the reviewer.

| Finding | Disposition |
|---|---|
| Probe A-F: every probed new sentence discriminates its named scenario | closed: probes pass; each mutation produced the expected missing-text failure and was reverted |
| Pre-existing "neighbouring components" spelling at `assets/instructions/ise.md:26` and `README.md:105` | closed: predates the change, outside the changed sentence set, glossary spelling used in the new text |
| `[SUBST]` shared-setup advisories from `givn lint` | closed: existing ise shared setup; no split, recorded by design |
| Blocked-rebuild binding's second needle (`Spec gap`) is not unique | closed: first needle (`the blocked rebuild is a setup failure`) discriminates the sentence; probe-level deletion fails the row |

## README impact

`README-IMPACT: none`. The change adds no command, flag, config key, exit code,
report line, or documented workflow. The README's `givn ise` command rows are
unchanged and remain correct; the changed rule list lives in
`docs/reference.md` and the served `ise` instruction.

## Receipt status

**not archived yet** — the change verification receipt at
`verification/tasks-verification.json` records scope `change`, total 9, passed
9, failed 0, skipped 0 at `543ceded68817fa704284f9bcd214040406762dd`; its e2e
scope is `not_applicable`. The archive receipt will carry the merged-candidate
suites.

## Sign-off

- [x] Fabrication audit: clean (0 empty step bodies, all nine scenario commits
      verified, no promised component missing, no downgraded e2e scenario, no
      HTTP/fetch substitute, no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope; the review does not rerun
      the suite for evidence.
- [x] Every coverage gap is classified and resolved from inspection; no
      fourth bucket.
- [x] No `@wip` tag remains in either delta spec.
- [x] Neither delta spec carries implementation-layer detail.
- [x] The local run commands start cleanly; the change runs no server or
      digital twin.
- [x] The exact files `verify.e2e_command` invokes were identified, and the tree
      was searched for a parallel e2e implementation before accepting the
      existing one.
- [x] The E2E execution selects its scope explicitly (`GIVN_RESULT_SCOPE=e2e`);
      the change scope and the e2e scope report zero failures and skips.
- [x] Implementation matches `design.md`; no silent deviation.
- [x] Interaction coverage maps every inventory entry to a matrix row, an
      existing `@e2e` scenario, and the promised CLI driving mechanism.
- [x] `design.md` introduces no internal primitive; the change adds no
      production struct, enum, or function.
- [x] `arc42.md` names chapters by number and maps each to a real
      `docs/arc42/<chapter>.md` file.
- [x] No finding is excused with a classification outside the three buckets.

REVIEW: PASS
