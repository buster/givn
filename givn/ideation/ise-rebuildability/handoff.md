# Handoff: ise-rebuildability

Status: applied 2026-11-14; term merge deferred to seed 1 and seed 2 as
recorded.

Confirmations recorded 2026-11-14:

1. The topic document was published to `docs/ideation/ise-rebuildability.md`
   and entered in `docs/INDEX.md`.
2. The three personas were promoted by appended entries; no earlier entry was
   rewritten, and the external-contributor segment stays unpromoted.
3. The six change seeds remain `Status: ready`; no change directory was
   created and no proposal was started.
4. The term merge is deferred: no `docs/glossary.md` row was merged. The exact
   eight rows, six rewrites, five anti-term additions, two scenario titles,
   and the arc42 sentence live in `add-ise-addon-and-specification`; the two
   `assets/instructions/specs.md` amendments and their pinned scenarios live
   in `enforce-ise-e2e-boundary`.

## 1. Topic document

- Draft: `givn/ideation/ise-rebuildability/topic-document.md`.
- Published: `docs/ideation/ise-rebuildability.md` on 2026-11-14; the draft
  stays as the authoring record.
- The published document names the same decisions as `frame.md`, `eventstorming.md`,
  `use-case-definitions.md`, `use-cases/*.md`, `questions.md`, and
  `domain-terms.md`. It reopens none of them.

## 2. Use-case to permanent mapping

| Use case | Status | Permanent document | Capability delta |
|---|---|---|---|
| `initialize-project` | modified | `givn/specs/initialize-project/` | `enable-optional-addons`: per-addon default, the `ise` prerequisite relation, and the declaration scaffold. |
| `upgrade-a-project` | modified | `givn/specs/upgrade-a-project/` | `apply-a-managed-upgrade`: the declaration scaffold joins the managed write and the staged allowlist; an existing declaration joins neither. |
| `create-change` | modified | `givn/specs/create-change/` | `prose-consolidation` (the canonical `@e2e` package policy); new `change-strategy`; new `spec-gap`. |
| `obtain-effective-guidance` | modified | `givn/specs/obtain-effective-guidance/` | `serve-extended-rules`: addon-bound serving of the `ise` instruction, and the core and stack instruction split. |
| `verify-change` | modified | `givn/specs/verify-change/` | `givn-workflow-enforcement`: the blocking `e2e` package boundary and the E2E-bound receipt; `check-specification-health`: the four advisory declaration findings. |
| `declare-the-specification` | new leaf | `givn/specs/declare-the-specification/` | `specification-declaration`. |
| `prepare-an-ise-test` | new leaf | `givn/specs/prepare-an-ise-test/` | `clean-room-preparation` and `trial-input-verification`. |
| `support` | modified fragment | `givn/specs/fragments/instruction-integrity.feature` | `instruction-integrity`: the `ise` obligations, the design and review record requirements, the stack instruction ids, and the reference check. |
| `archive-change` | unchanged | `givn/specs/archive-change/` | Consumes the E2E-bound receipt; the existing freshness check refuses a receipt whose relevant inputs changed. |
| `author-change` | unchanged summary | `givn/specs/author-change/` | Its children carry the delta. |
| `maintain-project` | unchanged summary | `givn/specs/maintain-project/` | Its children carry the delta. |

No new leaf is cut for `verify-an-ise-test`, `check-the-e2e-boundary`,
`enable-the-ise-addon`, `serve-the-stack-instruction`,
`report-the-declaration-warnings`, `classify-a-spec-gap`,
`record-the-change-strategy`, `scaffold-the-specification-declaration`,
`run-the-frozen-e2e-tests`, or a `specification-declaration` fragment. See
`use-case-definitions.md` for the rejection of each.

## 3. Term merge proposal

The glossary format is `| **Term** | Definition |`. The definitions below use
genus and differentia, a contrast, and named anti-terms.

| Term | Definition for `docs/glossary.md` |
|---|---|
| **Specification** | The declared durable authoring input a rebuild reads: behavior and invariants, vocabulary and structural model, contracts and schemas, technical limits, and black-box End-to-end (E2E) tests, held so a component's or product's Source code can be deleted and built again. Its roles are declared as path globs in `givn/specification.yaml`. Specification, not Source code: the Source code is the replaceable application code and internal structure, while the Specification is what stays unchanged across its replacement. Anti-terms: Replacement package (the rejected generic coinage), Permanent specification corpus (the narrower behavior aggregate), Product specification (the feature specification, not the umbrella). |
| **Source code** | The application code and internal structure behind a product's public interfaces (private modules, internal patterns, directory layout), the side of the Ise opposition a rebuild deletes and generates again. Source code, not Specification: the Source code is generated from the Specification and discarded on the next rebuild, while the Specification outlives it. Anti-term: Production source (the outline's measured production scope, not this term). |
| **End-to-end (E2E) test** | A black-box acceptance test that exercises a product solely through its real external interfaces (HTTP, CLI, UI, queues), shares no code, helpers, enums, or DTOs with production source, and must pass against a rebuild unchanged; the frozen artifact is the suite, its runner configuration, and the package's step definitions, held outside the builder's write authority during an Ise Test. End-to-end (E2E) test, not Component test: a Component test exercises one module boundary and its setup is rewritten with the stack, whereas an End-to-end (E2E) test crosses the product's real external interface and survives the stack change. Anti-terms: Black-box verifier (the rejected suite-plus-runner noun); do not use Verification scope as a synonym, because it names one runner execution's boundary. |
| **Constant Ise** | A change strategy (Ise Pattern 01) that updates one component's specification and scenarios, deletes that component's implementation and white-box unit tests inside its boundary, and rebuilds it from the updated specification while neighboring components stay untouched behind their contracts. Constant Ise, not Full rebuild: a Full rebuild regenerates all application source behind the product's external interfaces, whereas a Constant Ise bounds deletion and regeneration to one component behind its unchanged consumer contract; it is also not incremental modification, which edits the existing implementation in place. |
| **Ise Test** | One clean-room rebuild execution that sets the old Source code aside, freezes the black-box End-to-end (E2E) tests as its acceptance authority, hands the declared Specification to the builder as read-only input, builds the replacement from the Specification alone, and records whether the spec passed; Constant Ise names its one-unit scope, and Dry-Run Ise and Full replacement name its throwaway and production modes. Ise Test, not End-to-end (E2E) test: an End-to-end (E2E) test is one black-box test artifact, whereas an Ise Test is the execution that freezes and runs the suite. Anti-terms: Reconstruction trial (the rejected coinage), Ise Test result (the recorded pass or difference set, not the execution). |
| **Spec gap** | The classification record of one observed difference between the frozen End-to-end (E2E) test result and the declared Specification, holding exactly one of E2E test defect, implementation defect, specification defect, or acceptable undocumented change before any fix; only specification defect is a gap in the Specification, an E2E test defect is a gap in the test, an implementation defect is a gap in the build, and acceptable undocumented change is no gap. Spec gap, not Disposition: a Disposition records a review overlap finding, whereas a Spec gap records a rebuild difference and its corrective route. Anti-terms: Mismatch triage (the rejected coinage, colliding with Givn's blocking-mismatch senses), Coverage gap (a missing test, not this record). |
| **Change strategy** | The recorded decision, held as one `Decides D<n>` entry in a change's design, to pursue exactly one of incremental modification, Constant Ise, or a whole-product rebuild, with the Ise Test mode (Dry-Run Ise or Full replacement) recorded beside it when the change rebuilds and the affected consumers, the persisted state, and the rebuild boundary stated in the entry; it adds no mandatory design field. Change strategy, not Change: a Change is the aggregate that owns proposal, delta, design, review, and evidence, whereas a Change strategy is one recorded decision inside that design. Anti-term: Rebuild scope (the rejected narrower noun, colliding with Verification scope). |
| **Opt-in addon** | A registered Givn addon whose declared default is `false`, requiring explicit enablement (`givn addons enable <name>`) until promoted to default-on. Opt-in addon, not Addon default: the existing default-on addons resolve an omitted key to `true`, whereas an Opt-in addon resolves an omitted key to `false` so an experimental gate never alters an existing project silently. Anti-terms: Default-off addon (the rejected boolean-default name), Experimental addon (the rejected maturity-stage name), opt-in default (the rejected mixed posture that `Addon default` already names). |

### Rejected terms

| Rejected term | Rejection reason | Replacement |
|---|---|---|
| **Replacement package** | Rejected in `Q9` as a generic coinage; the record was oriented on The Ise Principle's own `specification` versus `source code` wording, and `Q10` confirmed the pair. | Specification, contrasted with Source code. |
| **Black-box verifier** | Rejected in `Q11`: it names the suite plus runner while also naming the party who runs it, and Ise names the kept layer End-to-end (E2E) tests. | End-to-end (E2E) test, with `black-box` as the no-shared-code qualifier. |
| **Component replacement** | Rejected in `Q12`: `docs/glossary.md` has no `Component` row and the definition lives only in the `architecture_principles` addon, so an `ise`-only project cannot resolve the name. | Constant Ise. |
| **Reconstruction trial** | Rejected in `Q13`: no Ise counterpart; every later artifact would need a mapping to one execution of the Ise Test. | Ise Test. |
| **Mismatch triage** | Rejected in `Q14`: `mismatch` already names a blocking document defect and a case-mismatched identity, and the name gives no clue that one outcome accepts the difference. | Spec gap. |
| **Default-off addon** | Rejected in `Q8`: it names the boolean default rather than the act a project performs, and promotion makes the name false. | Opt-in addon. |
| **Experimental addon** | Rejected in `Q8`: it names a maturity stage, leaves the boolean resolution implicit, and overlaps the parked WIP draft language. | Opt-in addon. |
| **Stack recipe** | Rejected in `Q15`: `recipe` asserts a prescribed procedure while the asset is guidance, and `stack` excludes `dialog-examples`, which holds a policy's examples rather than language mechanics. | No term; `Instruction` plus the id convention (`<id>-examples`, `<id>-<stack>`). |

### Collision flags

| Term | Existing row or text | Collision | Amendment before merge |
|---|---|---|---|
| **Opt-in addon** | `docs/glossary.md:8` `Addon`: "Every registered addon is enabled by default; a missing key in a partial block means enabled". | False once `ise` exists. | Rewrite to "Every registered addon except an Opt-in addon is enabled by default; a missing key resolves that addon's declared default; only an explicit `false` disables a default-on addon." |
| **Opt-in addon** | `docs/glossary.md:9` `Addon default`: "Anti-term: opt-in default (the rejected mixed posture)". | The anti-term reads as a rejection of the new posture. | Keep the anti-term only for the rejected mixed posture and add a cross-reference: "Anti-term: opt-in default (the rejected mixed posture; the Opt-in addon posture is separate)." |
| **Opt-in addon** | `givn/specs/fragments/match-guidance-to-enabled-addons.feature:23,86`: "the opt-in addon" means any addon switched on. | Two senses of the same phrase. | Disambiguate the two scenario titles to say "an enabled addon" and reserve `Opt-in addon` for the default posture. |
| **Opt-in addon** | `docs/arc42/01-introduction-and-goals.md:24`: "There is no default-off addon in the current registry". | False once `ise` is registered. | Amend the sentence to name `ise` as the one opt-in addon until promotion. |
| **Specification** | `docs/glossary.md:82` `Feature file`: "the executable behavior specification". | The narrow sense now reads as the umbrella. | Change to "the executable behavior artifact" and add: "A Feature file is one member of the Specification, not the umbrella." |
| **Specification** | `docs/glossary.md:87` `Permanent spec`: "used as the durable behavior source". | "Permanent spec" can read as a synonym for the umbrella. | Add: "A Permanent spec is a member of the Specification; the umbrella also holds structure, contracts, limits, and the `e2e` package." |
| **Specification** | `Permanent specification corpus` in `docs/domain-model.md:94` and the glossary's archive invariants (`permanent corpus`). | The corpus is the behavior aggregate (use cases, capabilities, scenarios), not the whole declared set. | Add: "The corpus is the behavior aggregate of the Specification; it does not include contracts, limits, or the `e2e` package." |
| **Specification** | `docs/glossary.md:169` `Use Case Elaboration` anti-term: "specification (reserved for the feature file)". | Directly contradicts the new umbrella. | Replace the anti-term with "mapping; a use-case phase is not a Specification". |
| **Source code** | `docs/glossary.md` `Production source` row. | Two senses: the replaceable application code versus the outline's measured production scope. | Add the anti-term above to `Source code`; leave `Production source` unchanged. |
| **End-to-end (E2E) test** | `docs/glossary.md:51` `Verification scope`: "the end-to-end scenarios". | The row's E2E means one runner scope, not the black-box acceptance term. | Add: "`Verification scope` names one runner execution's E2E boundary; it is not the End-to-end (E2E) test term." |
| **End-to-end (E2E) test** | `assets/instructions/specs.md:121-150` allows a library consumer's real public API calls to run in process under `@e2e`. | The no-shared-code rule contradicts the current allowance. | Amend the section when `enforce-ise-e2e-boundary` lands: in-process library calls move inside the declared `e2e` package or become `@component` scenarios. |
| **Constant Ise** | No `Component` row in `docs/glossary.md`; the definition lives in `assets/instructions/architecture-principles.md`. | The term's genus sentence uses `Component`. | No glossary row is added, per `Q6`. The definition says the `ise` addon requires `architecture_principles`, so `Component` always resolves. |
| **Spec gap** | `docs/glossary.md` `Disposition` row; coverage-gap and design-gap language in the instructions. | Three neighboring "gap" or review-record senses. | Add the anti-terms above and keep `Disposition` for review overlap findings. |
| **Change strategy** | Generic "strategy" in `docs/arc42/04-solution-strategy.md`, ADR-023, and the verb phrase at `assets/instructions/start.md:137`. | The bare word names several things. | No glossary row for "strategy"; prose keeps `change strategy` qualified, and every mode sentence says "Ise Test mode". |
| **Ise Test** | `Verification receipt`, `Change verification receipt`, and `Tested content`. | No direct collision, but a reader can confuse the execution with its evidence. | The definition contrasts the execution with the Ise Test result and the two test artifacts; no existing row changes. |

## 4. Persona promotion proposal

Three topic files are proposed. The external-contributor segment stays
unpromoted because `Q7` dropped it; its bare-clone and frozen-verifier lens is
carried by `givn-maintainer--owner`. The project files currently carry no
2026-11-13 or 2026-11-14 `ise-rebuildability` entries, so the entries below
are new, not duplicates. The topic files remain as historical record.

The parentheticals such as `(open: Q3)` are dated record content. `Q3`, `Q6`,
and `Q14` closed on 2026-11-14. If you prefer a live-status merge, add one
dated 2026-11-14 closure entry rather than editing these lines.

### `givn/personas/developer--primary-user.md`

```diff
--- a/givn/personas/developer--primary-user.md
+++ b/givn/personas/developer--primary-user.md
@@ Running record
+- 2026-11-13 — Need: `ise` stays opt-in, so an ordinary feature change pays
+  no new ceremony. (Frame, vision)
+- 2026-11-13 — Need: a replacement keeps persisted user data and state across
+  versions, not just compilable code. (Frame, Ise consequence "Data, state, and
+  contracts outlive the code")
+- 2026-11-13 — Need: `@e2e` suites strong enough to rebuild a module or the
+  whole product without reading the old source. (Frame, gaps G4/G7)
@@ Topic stances
+- 2026-11-13 — ise-rebuildability: a replacement that loses user data, or
+  needs a manual state migration step the specification never named, is not a
+  successful replacement.
+- 2026-11-13 — ise-rebuildability: a clean-room rebuild and a single-unit
+  replacement must be runnable from the same commands on Git and Fossil; the
+  SCM difference stays inside givn. (open: Q3)
+- 2026-11-13 — ise-rebuildability: wants the ordinary feature workflow
+  unchanged until `givn addons enable ise`, and no gate that fires on projects
+  that never opted in.
```

### `givn/personas/agent--technical-role.md`

```diff
--- a/givn/personas/agent--technical-role.md
+++ b/givn/personas/agent--technical-role.md
@@ Running record
+- 2026-11-13 — Need: instruction growth must not repeat the `v0.9.15`
+  collapse (`25/25` hidden grade kept, scenarios `130` -> `76`), so per-language
+  runner recipes load on demand. (Frame, gap G2)
+- 2026-11-13 — Need: deterministic `givn check` failures when `@e2e` imports
+  production code or the verifier is candidate-editable. (Frame, gaps G4/G5)
+- 2026-11-13 — Need: a stated boundary between normal feature co-evolution and
+  frozen clean-room reconstruction. (Frame, gap G6)
@@ Topic stances
+- 2026-11-13 — ise-rebuildability: reads a runner or driver recipe when the
+  stack needs it instead of carrying every stack in the step instructions.
+- 2026-11-13 — ise-rebuildability: the replacement package must be
+  enumerable; an undocumented schema, CLI flag, exit code, or viewport budget
+  is a blocked rebuild, not a judgment call.
+- 2026-11-13 — ise-rebuildability: every behavioral difference found during a
+  reconstruction trial is recorded as verifier defect, implementation defect,
+  specification defect, or acceptable undocumented change; never resolved as
+  silent drift.
+- 2026-11-13 — ise-rebuildability: `Q10` renamed the artifact above
+  `Specification`; the same demand reads "the Specification must be
+  enumerable", and an undocumented schema, CLI flag, exit code, or viewport
+  budget stays a blocked rebuild.
+- 2026-11-13 — ise-rebuildability: `Q11` renamed the frozen external-interface
+  suite `End-to-end (E2E) test`, with `black-box` as the no-shared-code
+  qualifier; the same demands read "deterministic `givn check` failures when
+  `@e2e` imports production code or the black-box E2E tests are
+  candidate-editable" and "every behavioral difference is recorded as an E2E
+  test defect, implementation defect, specification defect, or acceptable
+  undocumented change".
+- 2026-11-14 — ise-rebuildability: `Q13` renamed the bounded clean-room
+  execution `Ise Test`; the same demand reads "every behavioral difference
+  found in an Ise Test is recorded as an E2E test defect, implementation
+  defect, specification defect, or acceptable undocumented change; never
+  resolved as silent drift." (open: Q14)
+- 2026-11-14 — ise-rebuildability: `Q15` added no term, so the earlier demand
+  reads "the stack instruction loads on demand when the stack needs it
+  instead of every stack sitting in the step instruction"; the id convention
+  (`steps` -> `steps-rust`, `dialog` -> `dialog-examples`) is what the core
+  instruction names.
```

### `givn/personas/givn-maintainer--owner.md`

```diff
--- a/givn/personas/givn-maintainer--owner.md
+++ b/givn/personas/givn-maintainer--owner.md
@@ Running record
+- 2026-11-13 — Decision: confirmed the Frame for goal, scope boundary,
+  exclusions, and stakeholders. (User, Q2)
+- 2026-11-13 — Need: Catopia (`benchmarks/catopia-v2`,
+  `benchmarks/catopia-spring-v2`) plus Passuff evidence before `ise` becomes a
+  default. (Frame, vision)
+- 2026-11-13 — Need: Givn's own machinery rebuildable while authored
+  `assets/` stays product content. (Frame, gap G7)
+- 2026-11-13 — Need: the opt-in default leaves existing projects unchanged on
+  upgrade, so a missing `ise:` key never enables a gate. (Frame, gap G1;
+  open: Q6)
+- 2026-11-13 — Decision: the topic keeps three personas; the bare-clone
+  consumer lens moves here and `external-contributor--open-source-consumer`
+  is dropped. (User, Q7)
@@ Topic stances
+- 2026-11-13 — ise-rebuildability: a published repository or a frozen
+  replacement package must build, run, and verify through real external
+  interfaces with `givn` absent from PATH, with no dependency on deleted
+  internals or repository history.
+- 2026-11-13 — ise-rebuildability: treats the frozen verifier as the
+  contract; a suite that needs production code to start is not black-box.
+- 2026-11-13 — ise-rebuildability: no second binary, network service, or
+  language runtime in givn; `TC-2` and `TC-3` stand.
+- 2026-11-13 — ise-rebuildability: instruction size is a measured risk; a rule
+  addition that lowers scenario volume and coverage is a regression even when
+  the hidden grade holds.
+- 2026-11-13 — ise-rebuildability: no historical `catopia-v2` task prompt
+  mutates in place; a rebuild-permitting follow-up is a versioned variant.
+- 2026-11-13 — ise-rebuildability: a `givn ise` or `givn rebuild` subcommand,
+  if one is ever adopted, must clear Event Storming and use cases before its
+  CLI and filesystem contract is fixed. (open: Q3)
+- 2026-11-13 — ise-rebuildability: `Q10` renamed the frozen input above
+  `Specification` (contrasted with `Source code`); the same stance reads "a
+  published repository or a frozen Specification must build, run, and verify
+  with `givn` absent from PATH".
+- 2026-11-13 — ise-rebuildability: `Q11` renamed the frozen external-interface
+  suite `End-to-end (E2E) test`; the same stance reads "treats the frozen
+  black-box E2E test suite as the contract; a suite that needs production code
+  to start is not black-box".
```

Promotion summary:

| File | Running record entries | Topic stance entries | Duplicate against existing project entries |
|---|---|---|---|
| `developer--primary-user.md` | 3 | 3 | none |
| `agent--technical-role.md` | 3 | 5 | none |
| `givn-maintainer--owner.md` | 5 | 8 | none |

## 5. Change seeds

Six seeds are written under `changes/`. Each is `Status: ready` and is input
to `/givn-propose`; none has started a change.

| Seed | Use cases | Guarantee | Depends on | Start |
|---|---|---|---|---|
| `add-ise-addon-and-specification` | `initialize-project`, `upgrade-a-project`, `declare-the-specification`, `verify-change` (`check-specification-health`), `obtain-effective-guidance`, `create-change` (`change-strategy`, `spec-gap`), fragment `support` (`instruction-integrity`) | A project can enable `ise`, declare its Specification set once, receive the four placement findings, and be served the `ise` obligations, while every project that never enables it is unchanged; the change records the obligations name are shaped in the same seed. | none | 1 |
| `enforce-ise-e2e-boundary` | `verify-change` (`givn-workflow-enforcement`), `create-change` (`prose-consolidation`) | With `ise` effective, the declared E2E package is enforced as the only frozen acceptance authority and its identity is bound into verification evidence. | `add-ise-addon-and-specification` | 2 |
| `add-ise-clean-room-commands` | `prepare-an-ise-test` (`clean-room-preparation`, `trial-input-verification`) | A developer can produce a clean-room trial from a recorded revision and a verifier can confirm its frozen inputs, on Git and Fossil. | `add-ise-addon-and-specification`, `enforce-ise-e2e-boundary` | 3 |
| `split-stack-instructions` | `obtain-effective-guidance` (`serve-extended-rules`), fragment `support` (`instruction-integrity`) | Core step instructions stop growing because stack mechanics are served on request by id. | `add-ise-addon-and-specification` | 2 (parallel with `enforce-ise-e2e-boundary`) |
| `move-givn-e2e-to-black-box-package` | `verify-change`, `obtain-effective-guidance` | Givn's `@e2e` step definitions run from a package that links no production crate and `verify.e2e_command` runs that package, with every scenario keeping its wording and every retag recorded with a reason. | `enforce-ise-e2e-boundary` | 4 |
| `enable-ise-on-givn-and-run-a-dry-run` | `prepare-an-ise-test`, `declare-the-specification`, `create-change` | Givn declares its own Specification set, enables `ise`, runs one `Dry-Run Ise` against itself, and every observed difference is recorded as a `Spec gap` outcome. | `add-ise-clean-room-commands`, `move-givn-e2e-to-black-box-package` | 5 |

Dependency graph: seed 1 -> seeds 2, 3, 4; seed 2 -> seeds 3, 5; seed 3 ->
seed 6; seed 5 -> seed 6.

`S14` is a decision checkpoint and yields no seed: it assembles the evidence
table and the user decides the promotion, so it ships no deliverable guarantee
of its own. `plan.md` `S1` to `S5` is covered by seeds 1 to 4, and `S13` is
covered by seeds 5 and 6.

### Capability assignment

| Capability | Seed |
|---|---|
| `initialize-project`/`enable-optional-addons` | `add-ise-addon-and-specification` |
| `upgrade-a-project`/`apply-a-managed-upgrade` | `add-ise-addon-and-specification` |
| `declare-the-specification`/`specification-declaration` | `add-ise-addon-and-specification` |
| `verify-change`/`check-specification-health` | `add-ise-addon-and-specification` |
| `obtain-effective-guidance`/`serve-extended-rules` (addon-bound serving) | `add-ise-addon-and-specification` |
| `create-change`/`change-strategy` | `add-ise-addon-and-specification` |
| `create-change`/`spec-gap` | `add-ise-addon-and-specification` |
| `support`/`instruction-integrity` (the `ise` obligations and design and review requirements) | `add-ise-addon-and-specification` |
| `verify-change`/`givn-workflow-enforcement` | `enforce-ise-e2e-boundary` |
| `create-change`/`prose-consolidation` | `enforce-ise-e2e-boundary` |
| `prepare-an-ise-test`/`clean-room-preparation` | `add-ise-clean-room-commands` |
| `prepare-an-ise-test`/`trial-input-verification` | `add-ise-clean-room-commands` |
| `obtain-effective-guidance`/`serve-extended-rules` (stack split) | `split-stack-instructions` |
| `support`/`instruction-integrity` (stack ids and reference check) | `split-stack-instructions` |

Seeds 5 and 6 add no capability row: seed 5 moves Givn's `@e2e` steps into the
package the boundary already defines, and seed 6 applies the declaration,
preparation, and `Spec gap` records to Givn itself.

### Deviations from the seed table

1. **Seed 1 carries capabilities beyond the table's three use cases.**
   The guarantee's four placement findings belong to
   `verify-change`/`check-specification-health`; the `ise` obligations and the
   design and review requirements belong to
   `support`/`instruction-integrity`; the `Change strategy` and `Spec gap`
   record shapes belong to `create-change`; the upgrade scaffold belongs to
   `upgrade-a-project`. Without them the served obligations have no artifact
   and the four findings have no owner. The parent can move any of them to
   another seed.
2. **Seeds 1 and 4 both modify `serve-extended-rules` and
   `instruction-integrity`.** The split is stated per behavior: seed 1
   delivers addon-bound `ise` serving and the `ise` obligations; seed 4
   delivers the stack split and reference integrity. Neither seed alone
   completes the other's guarantee.
3. **`S13` yields two seeds and `S14` yields none.** The cut was corrected on
   2026-11-14: `ise` cannot be enabled on Givn before its `e2e` package stops
   linking production source, so `S13` splits into
   `move-givn-e2e-to-black-box-package` and
   `enable-ise-on-givn-and-run-a-dry-run`. `S14` is a decision checkpoint
   whose outcome is the user's promotion decision, not a shippable guarantee.
4. **`create-change` is split across seed 1 and seed 2.** Seed 2 ships
   `prose-consolidation`; seed 1 ships `change-strategy` and `spec-gap`. The
   parent can instead keep all three in seed 2 and move the record clause out
   of seed 1's guarantee.

### Seed open questions

These are implementation decisions and start gates the topic did not settle.
They do not change a closed decision. The limits below name where a guarantee
is wider than the settled scope.

| Seed | Open question | Blocks |
|---|---|---|
| `add-ise-addon-and-specification` | What active values does the first `givn/specification.yaml` scaffold write for the five roles and `source:`? | `S1`'s scaffold and the first trial on a project with no declared paths. |
| `add-ise-addon-and-specification` | Does the `ise` instruction belong to this seed or to `split-stack-instructions`? Current answer: this seed. | `S1`'s serving of `givn instructions ise` and the split seed's reference check. |
| `enforce-ise-e2e-boundary` | Which package manifests must the `e2e` dependency-edge check read in the first cut? | `S2`'s fourth advisory finding and `S3`'s blocking refusal on non-Cargo projects. |
| `enforce-ise-e2e-boundary` | Does the E2E identity extend `change-proof/v1` or require a new proof scope version? | `S3`'s receipt writer and archive freshness. |
| `split-stack-instructions` | Which stack instructions ship in the first split? | `S5`'s content split and the instruction line-budget gate. |
| `move-givn-e2e-to-black-box-package` | Which recorded evidence satisfies the external-evidence prerequisite? | The seed's start and the protocol claim for `S13`. |
| `enable-ise-on-givn-and-run-a-dry-run` | Which recorded evidence satisfies the external-evidence prerequisite? | The seed's start and the protocol claim for the self-rebuild. |

`--into` is settled, not open: `plan.md` `S4` makes it required and outside the
checkout, and `add-ise-clean-room-commands` records the accepted decision.

### Guarantee limits

These are the places where a seed's guarantee text is wider than what the
topic settled. None of them reopens a closed decision.

| Seed | Guarantee clause | Unsettled decision | Effect if left open |
|---|---|---|---|
| `add-ise-addon-and-specification` | "receive the four placement findings" | Which package manifests the `e2e` dependency-edge finding reads. | On a non-Cargo project, the fourth finding is silent. The seed must either declare a Cargo-only limit or name a broader reader. |
| `enforce-ise-e2e-boundary` | "its identity is bound into verification evidence" | Whether the identity extends `change-proof/v1` or needs a new scope version. | The binding is stateable either way; the receipt shape and the old-receipt refusal are not. |
| `split-stack-instructions` | "stack mechanics are served on request by id" | Which stacks receive an instruction in the first cut. | The mechanism is stateable; the coverage of existing stack bodies depends on the choice. |

The `--into` limit closed on 2026-11-14: `plan.md` `S4` makes `--into` required
and outside the checkout, and `add-ise-clean-room-commands` records that
decision.
