# Design: align-ise-rules-with-passuff-evidence

## Purpose and approach

Align the technology-agnostic ise rules with the evidence from the three Passuff trials and the Givn self-rebuild. The change edits the served `ise` and `review` instructions, adds the matching reference rows and three glossary terms, and rebinds the permanent scenarios that assert the changed sentences. No command, exit code, report line, manifest field, checksum, declaration key, or verification command changes.

The instruction carries each rule once. The reference states the value classes, the role home, and the scan roots an implementer needs. The scenarios assert the served text through the existing CLI boundary.

## Architecture impact

| Path | Change | Role |
|---|---|---|
| `assets/instructions/ise.md` | Add the build-input, fixed-value, acceptance-corpus, clean-room, neighbor-record, and dry-run rules; name the three glossary terms in the vocabulary paragraph | Served `ise` instruction |
| `assets/instructions/review.md` | Broaden the neighbor question to neighbors and consolidations | Served `review` instruction |
| `docs/reference.md` | Add the rebuild rules table, the environment-input sentence, and the corpus-split sentence | `contracts` role of the Specification |
| `docs/glossary.md` | Add `Build input`, `Neighbor record`, and `Replaced unit` | Vocabulary authority |
| `givn/specs/fragments/instruction-integrity.feature` | One permanent scenario body written in place by the implementation batch, plus the archive merge of seven added scenarios | Permanent behavior corpus |
| `givn/specs/create-change/prose-consolidation.feature` | One permanent scenario body written in place by the implementation batch; the archive merge replaces it with identical text | Permanent behavior corpus |
| `tests/steps/instruction_integrity_steps.rs` | Add the new `Then` bindings and rephrase the review neighbor binding | Component tier |
| `tests/steps/prose_consolidation_steps.rs` | Add the two acceptance-corpus assertions | Component tier |

No `src/**` file changes. The instructions are embedded assets resolved by `src/instructions.rs`; the edit changes their bytes, not the resolver.

The structural model is unchanged: `docs/domain-model.md` models aggregates and relationships, and the three new terms are process records, not aggregates.

No README change: the README names the `givn ise` commands and their arguments, not the trial rules.

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Served instruction corpus (`givn instructions <id>`) | `obtain-effective-guidance`, `create-change` | Authoring agent, reviewer | `src/instructions.rs` | A policy needs its own resolution rule or addon condition. |
| `prepare-an-ise-test` (`clean-room-preparation`, `trial-input-verification`): `givn ise rebuild-all`, `givn ise rebuild-component`, `givn ise compare` | `prepare-an-ise-test` | Developer, Independent verifier | `src/ise`, entered through `src/cli/ise.rs` | A second materialization mode (branch or worktree) or a second record scope. |

The changed reference section states the rules of the existing `prepare-an-ise-test` contract; its command behavior is unchanged. The change adds no adapter and no external boundary.

## Use-case traceability

| Guarantee, rule, or capability | Decision | Evidence |
|---|---|---|
| `create-change`: component scenarios stay in the Specification; only step definitions and fakes are implementation | D4 | Modified scenario `Served ise guidance keeps component scenarios in the Specification` |
| `create-change`: one `change strategy` with no mandatory design field | D1 | Unchanged scenario `The design guidance requires one recorded change strategy` |
| `create-change`: one `Spec gap` outcome names the wrong artifact | D2 | Unchanged scenario `A spec gap record names the artifact rather than its finder` |
| `obtain-effective-guidance`: the `ise` instruction is served while `ise` is effective and carries the rebuild obligations | D2, D3, D4, D5, D6, D7 | Seven added scenarios in `instruction-integrity` |
| `prepare-an-ise-test`: the spec copy rule, the declared acceptance package, and the environment inputs | D4, D5, D6, D7 | Reference rows; existing `clean-room-preparation` and `trial-input-verification` scenarios remain |
| `instruction-integrity`: a served instruction's rules stay complete and machine-readable | D2, D3, D5, D6 | Added and modified `instruction-integrity` scenarios |

Confirmed Personas and impact, with the artifact each reads:

| Persona | Impact | Evidence |
|---|---|---|
| `developer--primary-user` | The rules state what to declare and what to record before a rebuild, so the developer can read the trial result without reconstructing it. | The `Specification completeness` and record-row text in the served `ise` instruction, asserted by `The ise instruction makes the declaration cover every build input` and `The ise instruction records a rebuild's neighbors and consolidations`. |
| `agent--technical-role` | The builder receives the role home, the quarantine obligation, and the environment-input boundary in the instruction it reads. | The clean-room section and the `Fixed values` sentence, asserted by `The ise instruction keeps a published copy out of the clean room` and `The ise instruction requires a relied-on contract to enumerate its fixed values`. |
| `givn-maintainer--owner` | The reference and glossary give the maintainer the value classes, the role home, and the scan roots beside the command contract. | The `Rebuild rules` table in `docs/reference.md` and the three glossary rows. |

## Exact ise instruction text

### Vocabulary paragraph

Append this sentence to the existing vocabulary paragraph:

```markdown
`Build input`, `Neighbor record`, and `Replaced unit` are defined by the project
glossary.
```

### Specification completeness bullet

Replace the current first obligation with:

```markdown
- **Specification completeness** — every behavior, contract, limit, acceptance
  test, and build input the rebuild relies on is declared in
  `givn/specification.yaml` and reachable from the Specification paths. A build
  input is declared under the role that owns its content class: a build manifest
  and a lockfile under `contracts`, a toolchain pin under `limits`, a structural
  template under `structure`, a schema or migration asset under `contracts`, and
  served instruction or skill text under `behavior`. The frozen End-to-end (E2E)
  tests fix the bytes they assert; a structural template or a schema asset fixes
  its content through the contract that names its shape. A build input the
  declaration leaves outside the Specification roles is a `specification defect`,
  and a neighbor when it lies outside the replaced unit; the trial records it
  with its consequence. An asset the rebuild generates inside the replaced unit
  whose bytes no frozen test fixes is an `acceptable undocumented change`, not a
  neighbor. The dependency cache and the installed toolchain are environment
  inputs, never declared files: the lockfile declares the resolved dependencies,
  the toolchain pin declares the toolchain, the trial records how both were
  supplied, and a locked dependency the cache cannot resolve blocks the rebuild;
  the blocked rebuild is a setup failure, not a `Spec gap`, and it is not a
  `specification defect`. A contract the frozen tests rely on enumerates every
  value it fixes. A declared Specification path must survive `givn archive`: a
  change's artifacts move from `givn/changes/<id>/` to `givn/archive/<id>/`, so a
  Specification role names the archived path or another durable path, never an
  active change's artifact. `givn archive` refuses such a declaration before
  moving the change. The spec copy always carries the declaration itself:
  `givn/specification.yaml` is an implicit Specification entry, not a sixth role,
  so a project need not list it and the agent reads the declared roles from the
  copy the selected revision records.
```

The punctuation of the five obligation labels is unchanged: the two replaced bullets keep the em dash the three retained bullets use.

### two-layer test boundary bullet

Replace the current second obligation with:

```markdown
- **two-layer test boundary** — the `@e2e` scenarios stay in the Specification
  under the `behavior` role with their exact titles, and run from the declared
  `e2e` package, which holds the runner and every `@e2e` step definition and
  survives a rebuild. Component scenarios stay in the Specification unchanged;
  only their step definitions and fakes are implementation and may be rewritten
  inside the component boundary.
```

### Clean-room boundary section

Insert after `## The spec copy rule`, before `## Obligations`:

```markdown
## The clean-room boundary

The builder reads no product source outside the spec copy and the declared
inputs. Before the rebuild, scan the local package registry's source, cache,
and git stores for a published copy of the product, and quarantine every hit
outside the builder's reach; a store that does not exist has no hit. Record the
scan result in the neighbor record. A published copy the builder can read is not
a clean-room rebuild.
```

### Dry-run sentence

Insert at the end of the `## The spec copy rule` paragraph, before the new `## The clean-room boundary` section:

```markdown
A `Dry-Run Ise` trial promotes nothing: the rebuilt work stays in the copy, and
the trial review records that no rebuilt file was promoted.
```

### Records table

Change the sentence `An ise change carries two records:` to `An ise change carries three records:` and add this row:

```markdown
| `Neighbor record` | the trial review's evidence directory | every file changed outside the declared Specification or the replaced unit, and every consolidation, each with its reason; the trial review records the neighbor record |
```

Insert after the table:

```markdown
A consolidation merges or splits rebuilt paths; it is recorded with its reason
even when no file lies outside the Specification. A rebuild with no neighbor
records an empty table.
```

The five obligation labels and the existing records stay unchanged. The instruction still carries five obligations.

## Exact review guidance text

Replace the `Neighboring components` bullet in `assets/instructions/review.md` with:

```markdown
- **Neighbors and consolidations.** Ask which files changed outside the
  declared Specification or the replaced unit, and every consolidation the
  rebuild made, with the reason for each. Record the answer. This is one review
  question, never a gate.
```

The American spelling matches the review instruction's existing `neighbors` sentence and the unchanged scenario title `The review guidance asks the neighboring-components question without a gate`. The pre-existing `neighbouring components` wording in the `givn ise rebuild-component` rows of both instructions is outside this change's sentence set and stays.

## Reference rows

Insert after the `**The rule the copy carries.**` paragraph in the clean-room section:

```markdown
**Rebuild rules.** The `ise` instruction carries these rules; the reference
states what each requires.

| Rule | Requirement |
| --- | --- |
| Build inputs | A file the build reads that is not Source code and whose content the build cannot resolve from the Specification text is declared under the role that owns its content class: a build manifest and a lockfile under `contracts`, because they fix the dependency and build-target schema; a toolchain pin under `limits`; a structural template such as `assets/templates/domain-model.md` or `assets/templates/glossary.md` under `structure`; a schema or migration asset under `contracts`; and served instruction or skill text under `behavior`. The frozen End-to-end (E2E) tests fix the bytes they assert; a structural template or a schema asset fixes its content through the contract that names its shape. The `e2e` package's own manifest and lockfile are the test build's inputs, not the product build's. A build input the declaration leaves outside the Specification roles is a `specification defect`, and a neighbor when it lies outside the replaced unit, each with its consequence. |
| Fixed values | A contract the frozen End-to-end (E2E) tests rely on enumerates every value it fixes: literal text, headings, labels, error prefixes, exit codes, file formats, and test seams. |
| Acceptance corpus | The `@e2e` scenarios stay in the Specification under the `behavior` role with their exact titles. The declared `e2e` package holds the runner and every `@e2e` step definition. |
| Clean-room boundary | The builder reads no product source outside the spec copy and the declared inputs. Before the rebuild, scan the local package registry's source, cache, and git stores for a published copy of the product, quarantine every hit outside the builder's reach, and record the scan result in the neighbor record; a store that does not exist has no hit. |
| Neighbor record | Every file changed outside the declared Specification or the replaced unit, and every consolidation, with its reason. The trial review records the neighbor record; it is a review question, never a gate. |
| Dry-Run Ise | A `Dry-Run Ise` trial promotes nothing: the rebuilt work stays in the copy, and the trial review records that no rebuilt file was promoted. |

An environment input is never declared: the lockfile declares the resolved
dependencies, the toolchain pin declares the toolchain, and the trial records
how the dependency cache and the installed toolchain were supplied. A locked
dependency the cache cannot resolve blocks the rebuild; the blocked rebuild is a
setup failure, not a `Spec gap`, and it is not a `specification defect`.
```

Append this sentence to the `**End-to-end enumeration.**` paragraph:

```markdown
The scenarios themselves stay in the `behavior` role; the `e2e` role holds the
runner and every `@e2e` step definition.
```

## Glossary terms

Insert after the `Specification` row:

```markdown
| **Build input** | A file the product build reads that is not Source code and whose content the build cannot resolve from the Specification text: a build manifest, a lockfile, a toolchain pin, a structural template, a schema or migration asset, or served instruction or skill text. Build input, not Source code: the Source code is rebuilt from the Specification, whereas a build input is declared under the role that owns its content class and carried into the spec copy; its content is fixed either by asserted bytes or by a declared content contract, so the frozen End-to-end (E2E) tests fix the bytes they assert and a structural template or a schema asset fixes its content through the contract that names its shape. Anti-term: generated file (an output of the build, not an input it reads). |
```

Insert before the `Spec gap` row:

```markdown
| **Neighbor record** | The trial review's table of every file changed outside the declared Specification or the replaced unit, and every consolidation, each with its reason. Neighbor record, not Change verification receipt: a receipt binds the verified change identity, whereas a neighbor record compares a rebuilt boundary with the checkout. |
```

Insert after the `Spec copy` row:

```markdown
| **Replaced unit** | The Source code one rebuild replaces: one component's declared paths under Constant Ise, or the whole product's Source code under a whole-product rebuild. Replaced unit, not Component: the Component is the permanent responsibility behind a contract, whereas the replaced unit is the Source code span the one Ise Test deletes and rebuilds. |
```

The glossary has no `Change record` term; the `Neighbor record` contrast uses the existing `Change verification receipt` term.

## Delta scenarios and rebinding

Permanent scenarios whose assertions touch a changed sentence:

| Permanent title | File | Edit |
|---|---|---|
| `The review guidance asks the neighboring-components question without a gate` | `givn/specs/fragments/instruction-integrity.feature` | Modified. The single neighbor assertion becomes files outside the declared Specification or the replaced unit, every consolidation, and the reason for each. The not-a-gate assertion stays. |
| `Served ise guidance keeps component scenarios in the Specification` | `givn/specs/create-change/prose-consolidation.feature` | Modified. Adds the `@e2e` exact-title location and the declared `e2e` package runner and step-definition assertions. The three existing assertions stay and stay green. |
| `The ise instruction carries the five rebuild obligations` | `givn/specs/fragments/instruction-integrity.feature` | Unchanged. The five labels remain; the build-input and fixed-value statements sit inside Specification completeness. |
| `The ise instruction names only instruction ids that resolve` | `givn/specs/fragments/instruction-integrity.feature` | Unchanged. |
| `Served ise guidance states that the trial always carries the declaration` | `givn/specs/create-change/prose-consolidation.feature` | Unchanged. |
| `A spec gap record names the artifact rather than its finder` | `givn/specs/create-change/spec-gap.feature` | Unchanged. |
| `The design guidance requires one recorded change strategy` | `givn/specs/create-change/change-strategy.feature` | Unchanged. |

Added scenarios, all in `givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature`:

| Added title | Guards |
|---|---|
| `The ise instruction makes the declaration cover every build input` | The build-input classes, their role home under the content-class principle, and the declaration's classification |
| `The ise instruction accepts an asset the rebuild generates inside the replaced unit` | The generated-asset classification from S12 G19 and its free-bytes boundary |
| `The ise instruction keeps the dependency cache in the environment` | The S13 environment inputs, the blocked rebuild, and its setup-failure classification |
| `The ise instruction requires a relied-on contract to enumerate its fixed values` | The fixed-value sentence |
| `The ise instruction keeps a published copy out of the clean room` | The clean-room boundary section, including an absent registry store |
| `The ise instruction records a rebuild's neighbors and consolidations` | The third record row and the consolidation sentence |
| `The ise instruction keeps a dry run out of the product` | The dry-run sentence in the spec copy rule |

The added scenarios share the standard ise setup with many permanent scenarios, so the overlap review records `keep` for each with its distinct invariant: build-input coverage, generated-asset classification, environment inputs, fixed values, quarantine, neighbor record, or dry run. The `[SUBST]` subset advisories the lint prints for the shared setup are the same pattern the existing ise scenarios produce. No scenario is split.

The two delta files are:

- `givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature`, tagged `@givn.delta @instruction-integrity`
- `givn/changes/align-ise-rules-with-passuff-evidence/specs/create-change/prose-consolidation.feature`, tagged `@givn.delta @prose-consolidation`

No `@e2e` tag is added: the change adds no user action. The existing interaction inventory covers both capabilities.

## Step Definitions

| Capability | Step definition file |
|---|---|
| `instruction-integrity` | `tests/steps/instruction_integrity_steps.rs` |
| `prose-consolidation` | `tests/steps/prose_consolidation_steps.rs` |

New `Then` phrases are bound directly in these permanent files. No delta step copy is authored, so archive has no delta phrase without a permanent binding.

## New Then phrases and asserted text

One binding is added per new `Then` phrase; no count is taken from the obligation or record tables.

| Capability | New `Then` phrase | Asserted served text |
|---|---|---|
| `instruction-integrity` | the served instruction names the build manifest, the lockfile, the toolchain pin, a structural template, a schema or migration asset, and served instruction text as build inputs | `build input`, `a build manifest and a lockfile`, `a toolchain pin`, `a structural template`, `a schema or migration asset`, `served instruction or skill text` |
| `instruction-integrity` | the served instruction places the build manifest and the lockfile under contracts, the toolchain pin under limits, a structural template under structure, a schema or migration asset under contracts, and served instruction text under behavior | the four role keys `contracts`, `limits`, `structure`, `behavior` beside the named input classes |
| `instruction-integrity` | the served instruction states that a build input the declaration leaves outside the Specification roles is a specification defect, and a neighbor when it lies outside the replaced unit | `leaves outside the Specification roles`, `specification defect`, `a neighbor when it lies outside the replaced unit` |
| `instruction-integrity` | the served instruction treats an asset the rebuild generates inside the replaced unit whose bytes no frozen test fixes as an acceptable undocumented change | `whose bytes no frozen test fixes`, `acceptable undocumented change` |
| `instruction-integrity` | the served instruction states that a generated asset is not a neighbor | `not a neighbor` |
| `instruction-integrity` | the served instruction names the dependency cache and the installed toolchain as environment inputs, never declared files | `environment inputs, never declared files` |
| `instruction-integrity` | the served instruction states that a locked dependency the cache cannot resolve blocks the rebuild | `a locked dependency the cache cannot resolve blocks the rebuild` |
| `instruction-integrity` | the served instruction states that the blocked rebuild is a setup failure, not a Spec gap | `the blocked rebuild is a setup failure`, `Spec gap` |
| `instruction-integrity` | the served instruction states that the blocked rebuild is not a specification defect | `not a specification defect` |
| `instruction-integrity` | the served instruction states that a contract the frozen tests rely on enumerates every value it fixes | `enumerates every value it fixes` |
| `instruction-integrity` | the served instruction requires the builder to read no product source outside the spec copy and the declared inputs | `reads no product source outside the spec copy and the declared inputs` |
| `instruction-integrity` | the served instruction requires a scan of the local package registry's source, cache, and git stores | `scan the local package registry's source, cache, and git stores` |
| `instruction-integrity` | the served instruction requires every registry hit to be quarantined outside the builder's reach | `quarantine every hit outside the builder's reach` |
| `instruction-integrity` | the served instruction states that a store that does not exist has no hit | `a store that does not exist has no hit` |
| `instruction-integrity` | the served instruction records the registry scan result in the neighbor record | `Record the scan result in the neighbor record` |
| `instruction-integrity` | the served instruction records every file changed outside the declared Specification or the replaced unit | `every file changed outside the declared Specification or the replaced unit` |
| `instruction-integrity` | the served instruction records every consolidation with its reason | `every consolidation, each with its reason` |
| `instruction-integrity` | the served instruction states that the trial review records the neighbor record | `the trial review records the neighbor record` |
| `instruction-integrity` | the served instruction states that a Dry-Run Ise trial promotes nothing | `promotes nothing` |
| `instruction-integrity` | the served instruction states that the rebuilt work stays in the copy | `the rebuilt work stays in the copy` |
| `instruction-integrity` | the served review guidance asks for the files changed outside the declared Specification or the replaced unit | `files changed outside the declared Specification or the replaced unit` |
| `instruction-integrity` | the served review guidance asks for every consolidation the rebuild made | `every consolidation the rebuild made` |
| `instruction-integrity` | the served review guidance asks for the reason for each | `the reason for each` |
| `prose-consolidation` | the guidance states that the @e2e scenarios stay in the Specification with their exact titles | `stay in the Specification under the behavior role with their exact titles` |
| `prose-consolidation` | the guidance states that the declared e2e package holds the runner and every @e2e step definition | `the declared e2e package, which holds the runner and every @e2e step definition` |

The rephrased review binding:

- `review_guidance_asks_neighboring_components` keeps its function name and its text normalization, is registered for the phrase `the served review guidance asks for the files changed outside the declared Specification or the replaced unit`, and asserts `files changed outside the declared Specification or the replaced unit`.

Reused bindings, not duplicated:

- `the command exits 0` covers every added scenario's exit assertion.
- `the served review guidance states that the answer is a review question, never a gate` covers the modified review scenario's last `Then`.
- The three existing `prose-consolidation` `Then` phrases stay registered and stay green.

Because the rephrased binding replaces the old phrase, the implementation batch writes the permanent review scenario body in place to the delta's `@givn.modified` body in the same commit as the rebind. The archive merge replaces that body by title with identical text, so the merge is idempotent for it; the seven added scenarios are still merged. The permanent `prose-consolidation` body is written in place the same way. Both bodies therefore match their delta text before the review, and the full `verify.command` is green pre-archive.

## Test runner and strict mode

| Scope | Command |
|---|---|
| Single scenario | `GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs cargo test --locked --test features_runner -- 'givn/changes/align-ise-rules-with-passuff-evidence/specs/fragments/instruction-integrity.feature::The ise instruction makes the declaration cover every build input' --exact --nocapture` |
| Delta batch | `GIVN_FEATURES=givn/changes/align-ise-rules-with-passuff-evidence/specs ./run-tests.sh --non-e2e` |
| Change gate | `givn check tasks --change align-ise-rules-with-passuff-evidence` |
| Permanent scope | `GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e` |
| E2E verification | `GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e` |

Every verification command names its corpus through `GIVN_FEATURES`. The plain runner default is `givn/specs/` (`tests/features_runner.rs::features_path`), so the single-scenario command without the prefix selects the permanent corpus, matches no scenario named in the delta, prints `0 features / 0 scenarios / 0 steps`, and exits `0`. Verified against this checkout at `95b83225` on 2026-11-17: the unprefixed command prints that summary and exits `0`; the prefixed command prints `1 feature`, `1 scenario (1 failed)`, `4 steps (3 passed, 1 failed)` and exits `101` on the unbound Then. The scenario declares six steps; cucumber-rs stops a scenario at its first failed step, so the summary counts the four steps it executed. A `0 scenarios` summary is never evidence of a pass.

Count assertion: the bare runner cannot assert a count. `filter_run_and_exit` fails only on a failed step, a parse error, or a hook error, so zero scenarios exit `0`. The single-scenario command therefore asserts the printed summary (`1 feature`, `1 scenario`) in its recorded output. The count is machine-checked at the change gate: `givn check tasks` runs `verify.command` with `GIVN_FEATURES=<change specs>`, `GIVN_RESULT_FILE`, and `GIVN_RESULT_SCOPE=change`, and `RunnerResult::read_counts` refuses a result whose `total` is `0` with `declares zero scenarios`.

Strict mode is the `cucumber-rs` mechanism named by `givn instructions steps-rust`: `.fail_on_skipped()` on the builder, plus the runner's exit rule. `tests/features_runner.rs` sets `.fail_on_skipped()`, marks the run failed when `failed > 0` or `skipped > 0`, and writes the counted result file. An undefined step fails its scenario; a skipped step becomes a failure.

## E2E smoke test infrastructure

- **E2E runner command:** `GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e`, with `GIVN_RESULT_SCOPE=e2e` set by the script. The runner selects every `@e2e` scenario from the permanent corpus and skips `@givn.removed` merge records.
- **E2E step definition location:** `e2e/tests/steps/instruction_integrity_e2e_steps.rs` and `e2e/tests/steps/prose_consolidation_e2e_steps.rs`, separate from the regular step files.
- **Local test infrastructure:** a release or debug `givn` binary exported as `GIVN_BIN`; no browser, server, database, container, or fake service. The package links no product crate and re-reads `e2e/Cargo.lock` on every run.
- **Framework and justification:** `cucumber-rs` 0.23, read from `e2e/Cargo.toml` at design time. It matches the CLI boundary: the runner drives the built binary through `GIVN_BIN` and asserts stdout, stderr, and exit code.
- **Strict-mode proof:** `e2e/tests/e2e_runner.rs` sets `.fail_on_skipped()` and exits nonzero when `failed > 0`, `skipped > 0`, or no scenario ran; the preflight refuses a lockfile that names a product package. The independent proof stays the existing `Design guidance requires the complete E2E setup record` scenario and `An ise-effective change whose E2E run reports a skipped step fails the tasks check`; this change adds no runner configuration.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `instruction-integrity`: resolve an instruction and expose the machine contract | `An agent requests machine-readable artifact guidance` | CLI | `e2e_runner` invokes `givn instructions specs --json` and reads stdout. |
| `instruction-integrity`: request normal session-start instructions | `Author starts a session with the universal dialog obligation` | CLI | `e2e_runner` invokes `givn instructions` and reads stdout. |
| `prose-consolidation`: request canonical E2E guidance | `Canonical E2E policy is emitted by specs instructions` | CLI | `e2e_runner` invokes `givn instructions specs --change <id>` and reads stdout. |
| `prose-consolidation`: synchronize and inspect generated E2E guidance | `Generated guidance points to the canonical E2E policy` | CLI | `e2e_runner` runs `givn skills sync` and reads the generated skill files and stdout. |

No row is added or removed. The change adds no action, so the inventory stays normalized at one `@e2e` scenario per action.

## Visual Design Contract

- Interface: none
- Viewports: not applicable; the change renders no screen, HTML, CSS, or JavaScript
- Design system: `docs/design/design-system.md` (unchanged)

The changed output is served instruction text and reference prose, read through the CLI. The existing CLI-output scenarios are its evidence; no screenshot or transcript is added. Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

## Black-box-first policy

No new internal test. The new assertions read the served text through the real CLI, the same boundary the existing `instruction-integrity` and `prose-consolidation` scenarios use. No unit test is retained because no production behavior changes.

## External boundary cases

The external boundary is the `givn instructions` command. The adversarial cases already exist and stay:

- `givn instructions ise` with the addon off is refused by name.
- An unknown instruction id is refused with the known ids.
- The machine-readable path carries the same content as the text path.

The change adds no external input, so it adds no new adversarial case. The served text is asserted in the delta scenarios at that boundary.

## Local runnability and digital twins

- **Local run command:** `./run-tests.sh --non-e2e` runs the regular suite, formatting, and clippy; `./run-tests.sh --e2e --package e2e` runs the E2E package.
- **Isolated network:** not applicable. The change adds no dependency and starts no service.
- **Digital twins:** none. The product has no third-party dependency in this change.
- **Interface obstacles:** none. The existing `e2e` runner already builds and exports `GIVN_BIN`.

## Version freshness

No version changes. `cucumber-rs` 0.23 is the existing dependency read from `e2e/Cargo.toml` at design time. The Rust toolchain is the repository's existing pin; this change adds no version-bearing decision.

## Coverage process boundaries

N/A. The change adds no instrumented process, no production code, and no measurement command. The existing coverage scope and exclusions are unchanged.

## Arc42 impact

Row 12 is Yes: three terms are added to `docs/glossary.md`. Chapter 12 is a link to that file, so no chapter file is edited. Every other row is No. The full twelve-row assessment is `givn/changes/align-ise-rules-with-passuff-evidence/arc42.md`; no chapter file under `docs/arc42/` changes and no `.givn-skip` remains.

## Decisions

Decides D1: `change strategy incremental modification`. The change edits the served instructions, the reference, the glossary, and the two permanent scenario files in place; their contracts and responsibilities stay. Affected consumers: the authoring agent and reviewer that request the instructions. Persisted state: none. Rebuild boundary: none; no `Ise Test` runs. Realized by the two delta feature files and the step bindings. Rejected: `Constant Ise`, because no component is replaced behind a frozen boundary; `whole-product rebuild`, because no product is rebuilt.

Decides D2: a build input is declared under the Specification role that owns its content class: a build manifest, a lockfile, and a schema or migration asset under `contracts`, because they fix the dependency, build-target, and schema content; a toolchain pin under `limits`, because it fixes one technical constraint on the build; a structural template under `structure`, because it fixes the generated structural model; and served instruction or skill text under `behavior`, because those bytes are observable product behavior. An authored asset's content is fixed either by the bytes the frozen tests assert or by the contract that names its shape. The `e2e` package's own manifest and lockfile are the test build's inputs, not the product build's. The dependency cache and the installed toolchain stay environment inputs; a build input the declaration leaves outside the Specification roles is a `specification defect` and, outside the replaced unit, a neighbor; a blocked rebuild is a setup failure, not a `Spec gap` and not a `specification defect`; an asset the rebuild generates inside the replaced unit whose bytes no frozen test fixes is an `acceptable undocumented change`. Realized by the ise bullet, the reference row, the glossary term, and three added scenarios. Rejected: a sixth `build:` list in `givn/specification.yaml`, because it adds a loader key, the unknown-key refusal text, a second copy path, `givn ise compare` and archive-guard cases, and new declaration scenarios to a guidance change, while the five roles already name content classes and every content-role match is already copied. Default: the five roles carry the build inputs, with no `build:` list. Revisit trigger: an input whose content fits no content role, for example a binary-only tool. Switch cost: a loader key, the unknown-key refusal text, a second copy path, `givn ise compare` and archive-guard cases, and new declaration scenarios. Owner: `givn-maintainer--owner`. Rejected: recording the build input the declaration leaves outside the Specification roles as an `acceptable undocumented change`, because S13 named the declaration as the wrong artifact.

Decides D3: a contract the frozen tests rely on enumerates every value it fixes. Realized by the ise sentence and the reference row naming literal text, headings, labels, error prefixes, exit codes, file formats, and test seams. Rejected: keeping the rule in the reference only, because the builder reads the instruction; listing every value in the instruction, because the list is unbounded and technology-specific.

Decides D4: the `@e2e` scenarios stay in the Specification under the `behavior` role with their exact titles, and the declared `e2e` package holds the runner and every `@e2e` step definition. Realized by the two-layer bullet, the reference row, and the modified `prose-consolidation` scenario. Rejected: keeping the scenarios in the package, because S10 moved them out; duplicating the corpus in both places, because that creates two acceptance authorities.

Decides D5: the builder reads no product source outside the spec copy and the declared inputs; before the rebuild, scan the local package registry's source, cache, and git stores and quarantine every hit outside the builder's reach; an absent store has no hit; the neighbor record carries the scan result. Realized by the clean-room boundary section and the reference row. Rejected: a warning without quarantine, because S11's registry held published product source; a command-enforced scan, because the registry is outside the checkout and the command owns the copy, not the builder's environment.

Decides D6: the trial review records every file changed outside the declared Specification or the replaced unit, and every consolidation, with the reason for each; the answer stays a review question, never a gate. Realized by the third record row, the consolidation sentence, the review bullet, and the reference row. Rejected: a separate consolidation gate, because the review question already owns the judgement; a file-only record, because S12 consolidated 17 revision files into fewer paths.

Decides D7: a `Dry-Run Ise` trial promotes nothing; the rebuilt work stays in the copy and the review records that no rebuilt file was promoted. Realized by the spec-copy sentence and the reference row. Rejected: treating a green trial as a promotion authorization, because S11 and S12 kept the copy unpromoted.

Decides D8: no arc42 chapter file changes. Row 12 is Yes for the three glossary terms and chapter 12 links the glossary, so the assessment lives in `arc42.md` and the term rows live in `docs/glossary.md`. Realized by `arc42.md` and the glossary edit. Rejected: editing chapter 12 to repeat the glossary rows, because the chapter is a link by contract.

## Risks and tradeoffs

- **Instruction growth.** `assets/instructions/ise.md` grows by about 30 lines. Mitigation: one sentence per rule; the reference carries the value classes, the role home, and the scan roots.
- **Broader review question.** Extending the neighbor question to consolidations widens one recorded answer. Mitigation: it stays a review question, never a gate, and the record has one row per path or consolidation.
- **Rebound permanent scenarios.** Two permanent scenario bodies change while their titles stay. Mitigation: the implementation batch writes both bodies in place to the delta text, so the full `./run-tests.sh --non-e2e` preflight is green before the review; the archive merge replaces them by title with identical text, so the merge is idempotent for them, and the delta records both bodies in full.
- **Independent review.** The change asserts its own served text. Mitigation: every assertion is a mechanical substring check through the CLI, the design lists the exact sentences and the asserted substring per binding, and an independent context re-derives the served text and the row-12 assessment. The proposal records `Review kind: independent`; `design-review.md` and the final `review.md` record one reviewer context.

## Verification plan

1. Add the `Then` bindings, the rephrased review binding, and the two permanent scenario bodies, without touching the assets. Run the single-scenario command: expect `1 feature`, `1 scenario (1 failed)`, exit `101`, failing on the asserted sentence and not on an unbound step. Record the failing sentence as the RED reason.
2. Run the delta batch: every added scenario fails on its asserted sentence. Record the failing output per scenario batch.
3. Edit `assets/instructions/ise.md`, `assets/instructions/review.md`, `docs/reference.md`, and `docs/glossary.md`. Run the delta batch: every delta scenario passes, and formatting and clippy pass.
4. Run `givn check tasks --change align-ise-rules-with-passuff-evidence`. The change receipt records the delta scope with a non-zero scenario count; a result that declares zero scenarios is refused by name.
5. Pre-archive permanent scope: `GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e` passes. The implementation batch writes the two replaced permanent scenario bodies in place, so no permanent scenario carries an unbound step before archive; the archive merge later replaces them with identical text. Record the run's scenario count; a red scenario or a `0 scenarios` summary fails this step.
6. Run `GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e` for the existing `@e2e` scenarios; none asserts a changed sentence.
7. Run `givn lint --change align-ise-rules-with-passuff-evidence` and `givn check review --change align-ise-rules-with-passuff-evidence` in the review step.
8. After `givn archive --change align-ise-rules-with-passuff-evidence`, the archive gate merges the delta and runs the permanent and E2E scopes against the merged candidate; then run `GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e` and `GIVN_FEATURES=givn/specs ./run-tests.sh --e2e --package e2e` as the self-dogfood step.
