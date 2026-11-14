# Change seed: add-ise-addon-and-specification

Status: proposed by add-ise-addon-and-specification

- Use-case IDs: `initialize-project` (`enable-optional-addons`),
  `upgrade-a-project` (`apply-a-managed-upgrade`),
  `declare-the-specification` (`specification-declaration`),
  `verify-change` (`check-specification-health`),
  `obtain-effective-guidance` (`serve-extended-rules`, addon-bound serving),
  `create-change` (`change-strategy`, `spec-gap`), and fragment `support`
  (`instruction-integrity`, the `ise` obligations and the design and review
  requirements)
- Topic: `ise-rebuildability`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: first; no dependencies
- Handoff: seed only; `/givn-propose` not started (2026-11-14)

## Guarantee

A project can enable `ise`, declare its Specification set once, receive the
four placement findings, and be served the `ise` obligations. The
change-authoring surfaces those obligations name, the design `Change
strategy` entry and the four-way `Spec gap` record, are shaped in the same
seed. Every project that never enables `ise` is unchanged: no new key, no new
file, and no new gate.

The seed table assigned three use cases. The guarantee also requires the four
advisory findings (`verify-change`/`check-specification-health`), the `ise`
instruction content and design and review requirements
(`instruction-integrity`), the change records (`create-change`), and the
upgrade scaffold (`upgrade-a-project`). Those capabilities are listed below
and flagged in `handoff.md` for the parent's confirmation.

## Capabilities delivered

| Use case | Capability | Behavior this seed delivers |
|---|---|---|
| `initialize-project` | `enable-optional-addons` | `ise` joins the registry as the one `Opt-in addon`; a fresh init omits its key; the two prerequisite refusals print by name; the first enable scaffolds `givn/specification.yaml`; the all-default scenarios and glossary rows are amended. |
| `upgrade-a-project` | `apply-a-managed-upgrade` | When `ise` is effective and the declaration is absent, the upgrade writes and stages the scaffold as part of the one allowlist-only revision; an existing declaration is preserved byte-for-byte and reported as project-owned. |
| `declare-the-specification` | `specification-declaration` | The five roles and one `source:` list are written through a strict schema; unknown keys, unknown or repeated roles, omitted roles, an absent source list, and paths outside the project are refused by name and write nothing. |
| `verify-change` | `check-specification-health` | `givn lint` reports the four advisory declaration findings with exit 0: a declared path missing, a path in two roles, a `.feature` outside the `behavior` globs, and an `e2e` package missing or with a dependency edge into `source`. |
| `obtain-effective-guidance` | `serve-extended-rules` | `givn instructions ise` is served only while `ise` is effective; with the addon off it refuses and names the addon. The text and JSON paths carry the same content. |
| `create-change` | `change-strategy` | The design's existing `Decides D<n>` ledger records one strategy and, for a rebuild, the `Ise Test` mode, with no new mandatory design field. |
| `create-change` | `spec-gap` | A corrective change names one `Spec gap` record and its one outcome before any fix; the record shape and the corrective route are stated. |
| `support` fragment | `instruction-integrity` | The `ise` instruction carries the five obligations; the design, design-review, and review instructions require the change strategy, the `Spec gap` outcome, and the neighboring-components question. The stack-id split is reserved to `split-stack-instructions`. |

## Accepted decisions

### Q4: Dedicated `givn/specification.yaml`

- Context: The declaration had to carry the five retained roles and the
  replaceable roots, and a misspelled declaration must be visible. The
  candidate sites differed in whether the schema is strict.
- Options:
  1. **A block in `givn/config.yaml`**: In practice: a `specification:` block
     beside `artifacts:` and `apply:`. Gain: no new file, loader, template, or
     init step. Cost: the lenient `OverlayDoc` top level silently ignores a
     misspelled block, so a disabled declaration produces no finding.
  2. **Dedicated `givn/specification.yaml`**: In practice: a strict file with
     five roles and one `source:` list. Gain: unknown keys and roles are
     refused; the file has one purpose; a project without `ise` never gets it.
     Cost: a new file, loader, template, init path, and upgrade path, and
     `docs/reference.md` gains a third configuration authority.
  3. **Convention**: In practice: infer `givn/specs/**`, the glossary, the
     domain model, and `docs/reference.md`. Gain: no schema and no declaration
     work. Cost: convention cannot express the `source:` partition, so
     deletion becomes the inference the settled constraints forbid; contracts
     and limits stay invisible.
- Chosen: **Dedicated `givn/specification.yaml`** because the strict top-level
  schema refuses a misspelled role or block, and because the file declares no
  component list. Rejected: the `config.yaml` block because the lenient top
  level hides a misspelled declaration; convention because it cannot express
  the replaceable roots.

### Q5: One declared `e2e` package

- Context: The `@e2e` tag, the declared role, and the frozen set had to name
  the same thing, or a receipt cannot say what was protected. The existing
  policy allowed in-process library calls and one runner for both scopes.
- Options:
  1. **Tag coincidence and one package**: In practice: a separate `e2e`
     package holds the runner and every `@e2e` step definition, links no
     production source, and `verify.e2e_command` runs it. Gain: one rule
     replaces two, and it matches Ise Pattern 03. Cost: both pilots
     restructure; Givn's runner links `givn`, so its `e2e` target needs its
     own manifest or a consumer artifact.
  2. **Keep the in-process allowance**: In practice: the tag stays broad and
     the frozen set is the scenarios the declared target ran. Gain: no
     restructure. Cost: two meanings of `@e2e`, and the four-count result
     file cannot prove which scenarios ran.
  3. **Multiple E2E targets**: In practice: split the suite by area, each
     target dependency-free. Gain: no single-target bottleneck. Cost:
     multiple frozen digests, and the `e2e` role stops naming one package.
- Chosen: **Tag coincidence and one package**, narrowed to a package boundary.
  Feature files stay shared. Rejected: the in-process allowance because the
  receipt cannot prove the protected set; multiple targets because the role
  must name one frozen authority.

### Q6: `ise` requires `architecture_principles`

- Context: The `ise` guidance uses `Component`, `Port`, and contract
  vocabulary whose definitions live only in the `architecture_principles`
  addon.
- Options:
  1. **No coupling**: In practice: an `ise` project without
     `architecture_principles`; the guidance names paths and never says
     `Component`. Gain: one addon enables without another; no prerequisite
     concept. Cost: an `ise`-only project; the guidance cannot use the
     established nouns and every sentence names paths.
  2. **Prerequisite**: In practice: `givn addons enable ise` refuses until
     `architecture_principles` is on, and disabling the prerequisite refuses
     while `ise` is on. Gain: one vocabulary source and shorter guidance.
     Cost: no `ise`-only project; the registry, addon commands, and lock gain
     a prerequisite relation.
  3. **Auto-enable the prerequisite**: In practice: enabling `ise` writes
     both keys. Gain: convenience and the vocabulary is present. Cost: one
     action changes two recorded states, the lock shows drift, and the reverse
     command has no rule.
- Chosen: **Prerequisite** because the guidance uses the established
  vocabulary instead of invented nouns. Rejected: no coupling because the
  `ise` guidance would restate every path; auto-enable because it silently
  changes a second addon.

### Q8: `Opt-in addon`

- Context: The addon posture needed a name before the canvas and the seeds
  could rely on it. The existing `Addon default` row and its `opt-in default`
  anti-term made the name a collision risk.
- Options:
  1. **Default-off addon**: In practice: `givn addons list` reports `ise` as
     off. Gain: literal and hard to misread. Cost: names the boolean, not the
     act; promotion to default-on makes the name false.
  2. **Experimental addon**: In practice: `ise` is reported as experimental
     even after a project enables it. Gain: states why the default is off.
     Cost: says nothing about the stored default, and maturity changes.
  3. **Opt-in addon**: In practice: the project runs
     `givn addons enable ise`. Gain: names the act, contrasts with `Addon
     default`, and matches ordinary project speech. Cost: promotion makes the
     name false; the glossary anti-term and the `match-guidance-to-enabled-addons`
     wording need amendment.
- Chosen: **Opt-in addon**. Rejected: `Default-off addon` because it names
  the mechanism; `Experimental addon` because it names maturity instead of the
  default.

### Q10: `Specification`, contrasted with `Source code`

- Context: The durable input had no Givn name. `Replacement package` was
  rejected as generic, and the record was to use The Ise Principle's own
  wording.
- Options:
  1. **`Specification`, contrasted with `Source code`**: In practice:
     "rebuild from the specification alone". Gain: the source's own umbrella
     and opposition; no translation table. Cost: collides with `Feature file`,
     `Permanent spec`, and the `Use Case Elaboration` anti-term; the glossary
     needs amendments.
  2. **`Product specification`**: In practice: reuse the existing phrase for
     a feature specification. Gain: additive; no anti-term repealed. Cost:
     existing feature-specification sentences become false, and the umbrella
     stays ambiguous.
  3. **Widen `Permanent specification corpus`**: In practice: the rebuild
     reads the corpus. Gain: no new noun and no glossary edit. Cost: the
     corpus is narrower (use cases, capabilities, scenarios); widening it
     makes it broader than its members and loses the spec-versus-code
     opposition.
- Chosen: **`Specification`, contrasted with `Source code`**. Rejected:
  `Product specification` because it makes existing sentences false; widening
  the corpus because it loses the opposition.

### Q11: `End-to-end (E2E) test`

- Context: The frozen acceptance suite needed Ise's kept-list name, and the
  no-shared-code rule needed a qualifier.
- Options:
  1. **`Black-box E2E test`**: In practice: the section heading reads
     "Black-box E2E tests (`@e2e`)". Gain: the name and the qualifier come
     from one sentence. Cost: `... test`
     names only the tests, not the suite plus runner; it makes the current
     in-process allowance false.
  2. **`End-to-end (E2E) test`**: In practice: the existing `@e2e`,
     `verify.e2e_command`, and `Verification scope` keep their names. Gain:
     the smallest edit; no durable sentence becomes false. Cost: `E2E` alone
     does not carry the survival rule, so every precise sentence repeats
     `black-box`.
  3. **`Black-box verifier`**: In practice: one noun for the suite and its
     runner. Gain: covers exactly what the receipt digests and the builder
     cannot edit. Cost: no Ise counterpart; `verifier` also names the party
     who runs it; it says nothing about component tests.
- Chosen: **`End-to-end (E2E) test`**, with `black-box` as the no-shared-code
  qualifier. Rejected: `Black-box E2E test` because it falsifies the current
  allowance; `Black-box verifier` because it collides with the verifier
  party.

### Q12: `Constant Ise`

- Context: The `Change strategy` value for a one-component rebuild needed a
  name.
- Options:
  1. **`Component replacement`**: In practice: `change strategy: component
     replacement`. Gain: uses Givn's `Component`; no second noun. Cost: the
     glossary has no `Component` row and the definition lives only in the
     addon, so an `ise`-only project cannot resolve it; the boundary also
     differs from Ise's one module.
  2. **`Constant Ise`**: In practice: `change strategy: Constant Ise`.
     Gain: Pattern 01's exact name and tagline; the family leaves `Dry-Run
     Ise` a place to land. Cost: a proper noun needs a genus; "unit" maps to
     `Component`; the mixed phrase `Constant-Ise unit replacement` must go.
  3. **`Unit replacement`**: In practice: `change strategy: unit
     replacement`. Gain: matches "the impacted unit" and resolves without the
     addon. Cost: `unit` collides with the outline measurement word and loses
     the `Component` tie.
- Chosen: **`Constant Ise`**. Rejected: `Component replacement` because the
  noun is unavailable in an `ise`-only project; `Unit replacement` because
  `unit` already names an outline measurement.

### Q13: `Ise Test`

- Context: One clean-room execution needed a name, and the frozen-input and
  triage invariants apply to both the throwaway and production modes.
- Options:
  1. **`Ise Test`**: In practice: "one Ise Test". Gain: the source's act
     noun; one noun covers both modes; the invariants are written once. Cost:
     it can mean the act, one execution, or the verdict; `test` is generic;
     sentences need "the builder in an Ise Test".
  2. **`Dry-Run Ise` / `Full replacement` as the names**: In practice: record
     the mode as the name. Gain: the exact card words and an explicit mode;
     no container. Cost: no shared noun for the invariants; `Dry-Run Ise`
     names only the throwaway build; `Full replacement` sits one word from a
     whole-product rebuild.
  3. **`Reconstruction trial`**: In practice: "during a trial". Gain: a
     natural container; no sweep of existing occurrences. Cost: no Ise
     counterpart; every artifact needs a mapping; `reconstruction` is Givn's
     word, not Ise's.
- Chosen: **`Ise Test`**, with `Constant Ise` as the scope and `Dry-Run Ise`
  or `Full replacement` as the mode. Rejected: the mode names alone because
  the invariants have no shared noun; `Reconstruction trial` because it needs
  a mapping.

### Q14: `Spec gap`

- Context: The four-outcome classification needed a name. Ise uses "a gap in
  the spec" for one outcome, not for all four.
- Options:
  1. **`Spec gap`**: In practice: `Spec gap: implementation defect`. Gain:
     the source's phrase, no coinage, and a name that points at the
     corrective act. Cost: it asserts a spec gap for two outcomes that are
     not, and it collides with coverage-gap and design-gap language.
  2. **`Difference triage`**: In practice: `Difference triage: implementation
     defect`. Gain: covers all four outcomes; `triage` is the source's verb.
     Cost: a Givn coinage; the root repeats awkwardly; `difference` is
     generic.
  3. **`Mismatch triage`**: In practice: `Mismatch triage: acceptable
     undocumented change`. Gain: no sweep and a precise observation. Cost:
     it collides with Givn's blocking-mismatch senses; no Ise counterpart;
     it hides that one outcome accepts the difference.
- Chosen: **`Spec gap`**, with the limitation recorded in its definition.
  Rejected: `Difference triage` because it is a coinage; `Mismatch triage`
  because `mismatch` already means a blocking document defect.

### Q16: `Change strategy`

- Context: The design needed a name for the recorded choice among incremental
  modification, `Constant Ise`, and a whole-product rebuild.
- Options:
  1. **No new term**: In practice: the ledger records the decision without a
     noun. Gain: no noun for a ledger entry. Cost: the no-rebuild branch has
     no name; the `Constant Ise` genus and the `Ise Test` example are
     rewritten; a whole-product rebuild has no scope name.
  2. **`Change strategy`**: In practice: `Decides D4: change strategy
     Constant Ise; Ise Test mode Dry-Run Ise`. Gain: the only name covering
     the no-rebuild branch; gives the guidance a subject. Cost: a mandatory
     template line and design requirement; `strategy` is generic; every mode
     sentence must say "Ise Test mode".
  3. **`Rebuild scope`**: In practice: `Rebuild scope: Constant Ise`. Gain:
     names only how much is rebuilt; an incremental change has no field.
     Cost: two records, an absent field means incremental, and `scope`
     collides with `Verification scope` and the Frame's scope boundary.
- Chosen: **`Change strategy`** because it is the only name that covers the
  no-rebuild branch and it fits the existing `Decides D<n>` ledger. Rejected:
  no term because the common incremental case loses its name; `Rebuild scope`
  because it splits the record and collides with existing scope language.

### Deferred glossary merge and collision amendments

- Context: The user confirmed the eight terms but chose to publish the topic
  and the seeds now, before any change starts. Merging the rows at handoff
  would make `docs/glossary.md` assert `ise`, `Opt-in addon`, and the `e2e`
  boundary before a command honors them, so a project that never enables
  `ise` would read false statements. The merge and its collision amendments
  move to the seeds whose code makes each row true.
- Options:
  1. **Merge at handoff**: In practice: `docs/glossary.md` and the neighboring
     rows change before this seed lands. Gain: the durable terms exist
     immediately. Cost: the glossary asserts an addon, a default posture, and
     a boundary that no command honors yet; a project without `ise` reads
     false statements.
  2. **Defer to this seed with the exact rows recorded**: In practice: this
     seed's implementation batch merges the eight rows and applies the six
     rewrites, five anti-term additions, two scenario-title changes, and one
     arc42 amendment. Gain: each row lands with the code that makes it true,
     and the seed carries the exact text. Cost: the glossary stays silent
     about the eight terms until this seed lands, and the implementer must
     read the rows below instead of guessing.
  3. **Withdraw the merge**: In practice: no glossary row is added. Gain: no
     work. Cost: the confirmed terms never reach the durable vocabulary, and
     specs, design, and glossary drift.
- Chosen: **Defer to this seed with the exact rows recorded** because the
  durable glossary must not assert a state that does not exist yet. Rejected:
  merging at handoff because it publishes false state; withdrawing because it
  loses the confirmed vocabulary.

**Landing rule.** Every row below lands with the code that makes it true: the
`Addon` and `Addon default` rewrites and the arc42 sentence land with the
`ise` registry entry and the prerequisite code; `Feature file`,
`Permanent spec`, the corpus note, and the `Use Case Elaboration` anti-term
land with the declaration and addon-bound serving; the
`match-guidance-to-enabled-addons` titles land with the same serving; the
five anti-term additions land with the rows that carry them.

**Eight rows to merge into `docs/glossary.md`.** The table is copied from
`handoff.md` section 3 and uses the glossary format `| **Term** | Definition |`.

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

**Six existing rows to rewrite.** The replacement text is exact; the rest of
each row stays unchanged unless the table says otherwise.

| Location | Current text | Replacement |
|---|---|---|
| `docs/glossary.md:8` `Addon` | "Every registered addon is enabled by default; a missing key in a partial block means enabled, and only an explicit `false` ... opts out." | "Every registered addon except an Opt-in addon is enabled by default; a missing key resolves that addon's declared default; only an explicit `false` disables a default-on addon." |
| `docs/glossary.md:9` `Addon default` | "Anti-term: opt-in default (the rejected mixed posture)." | "Anti-term: opt-in default (the rejected mixed posture; the Opt-in addon posture is separate)." |
| `docs/glossary.md:82` `Feature file` | "the executable behavior specification" | Change to "the executable behavior artifact" and add "A Feature file is one member of the Specification, not the umbrella." |
| `docs/glossary.md:87` `Permanent spec` | "used as the durable behavior source." | Add "A Permanent spec is a member of the Specification; the umbrella also holds structure, contracts, limits, and the `e2e` package." |
| `docs/domain-model.md:94` corpus row, and the `permanent corpus` archive invariants at `docs/glossary.md:41,313` | The corpus reads as the whole durable set. | Add "The corpus is the behavior aggregate of the Specification; it does not include contracts, limits, or the `e2e` package." |
| `docs/glossary.md:169` `Use Case Elaboration` | "Anti-terms: mapping, specification (reserved for the feature file)." | "Anti-terms: mapping; a use-case phase is not a Specification." |

**Five anti-term additions.**

| Owning row | Addition |
|---|---|
| `Source code` | Anti-term `Production source`; leave the existing `Production source` row unchanged. |
| `End-to-end (E2E) test` | "`Verification scope` names one runner execution's E2E boundary; it is not the End-to-end (E2E) test term." |
| `Spec gap` | Anti-term `Mismatch triage`, the rejected coinage colliding with Givn's blocking-mismatch senses. |
| `Spec gap` | Anti-term `Coverage gap`, a missing test rather than this record; keep `Disposition` for review overlap findings. |
| `Change strategy` | Anti-term `Rebuild scope`, the rejected narrower noun colliding with `Verification scope`. |

**Two scenario titles to disambiguate.** Both titles mean any addon switched
on; `an enabled addon` states that, and `Opt-in addon` stays reserved for the
default posture.

| Location | Current title | Replacement |
|---|---|---|
| `givn/specs/fragments/match-guidance-to-enabled-addons.feature:23` | `Scenario: Architecture principles guidance follows the opt-in addon` | `Scenario: Architecture principles guidance follows an enabled addon` |
| `givn/specs/fragments/match-guidance-to-enabled-addons.feature:86` | `Scenario Outline: Outline guidance follows the opt-in addon` | `Scenario Outline: Outline guidance follows an enabled addon` |

**One arc42 amendment.** Name `ise` as the one opt-in addon while the
registry carries it.

| Location | Current text | Replacement |
|---|---|---|
| `docs/arc42/01-introduction-and-goals.md:24` | "There is no default-off addon in the current registry." | "`ise` is the one opt-in addon in the current registry until promotion; every other registered addon is default-on." |

## Open questions

### What does the first `givn/specification.yaml` scaffold write?

- Blocks: `S1`'s scaffold and the first trial on a project with no declared
  paths.
- Context: Q4 settled the strict schema and that the scaffold content is the
  documented shape, not a project decision. It did not settle the values the
  first write uses. A project that enables `ise` has no declaration yet, and
  the loader requires the five roles and one `source:` list.
- Question: When `givn addons enable ise` or `givn upgrade` writes the
  scaffold, what active values does it write for the five roles and the
  `source:` list?
- Options:
  1. **Empty role lists and a placeholder `source:` list**: In practice: the
     file validates but retains nothing until the developer edits it. Gain: no
     invented paths and no language knowledge. Cost: the first trial refuses
     because the declaration retains nothing, so the developer must edit
     before any rebuild.
  2. **Commented examples with empty active lists**: In practice: comments
     show `givn/specs/**`, the glossary, the domain model, and `src/**`, while
     the active lists stay empty. Gain: guidance without invented paths.
     Cost: the developer still edits before a trial, and the strict loader
     must accept comments in the file.
  3. **Detected paths from the existing tree**: In practice: the scaffold
     writes `givn/specs/**` when that directory exists and `src/**` for the
     source root. Gain: a declaration that works immediately. Cost: detection
     is inference about a project's paths, and the settled constraints forbid
     inferring what a test is, not what a source root is, so the boundary
     needs a rule.
- Example: a fresh project runs `givn addons enable ise`; its
  `givn/specification.yaml` either has empty lists and fails the first
  `givn ise prepare`, or carries enough paths to prepare a trial without an
  edit.

### Does the `ise` instruction belong to this seed or to `split-stack-instructions`?

- Blocks: this seed's serving of `givn instructions ise` and the
  `split-stack-instructions` content split.
- Context: This seed delivers the five `ise` obligations and the design,
  design-review, and review requirements they bind. `split-stack-instructions`
  delivers the stack instruction bodies and the instruction-id reference
  check. The `ise` instruction is both a new addon-bound instruction, served
  by `serve-extended-rules` here, and a new instruction file that the
  reference check and the line budget will see.
- Question: Does this seed serve the `ise` instruction, or does
  `split-stack-instructions` add it with the stack split?
- Current answer: **This seed**, because the obligations are released only if
  the instruction that states them is served in the same guarantee;
  `split-stack-instructions` changes where existing stack bodies live, not
  which addon-bound instructions exist.
- Options:
  1. **This seed**: In practice: `givn instructions ise` is served as soon as
     `ise` is effective, and `split-stack-instructions` only adds the stack-id
     reference check for it. Gain: the obligations and the records they
     require ship together. Cost: this seed grows by one instruction file and
     its serving tests.
  2. **`split-stack-instructions`**: In practice: the obligations and their
     records ship first, and the instruction text arrives with the split.
     Gain: this seed stays smaller. Cost: an enabled project has the `ise`
     gate, the declaration, and the change records but no served instruction
     for one seed, and the split seed's guarantee is about stack growth, not
     addon serving.
  3. **Both seeds**: In practice: this seed serves it and
     `split-stack-instructions` adds it to the line-budget and reference
     checks. Gain: ownership is explicit on both sides. Cost: the instruction
     and its tests are touched twice, and the two seeds must agree on its id
     before either lands.
- Example: a project enables `ise` after this seed archives;
  `givn instructions ise` either prints the five obligations or refuses the id
  as unknown until `split-stack-instructions` lands.
