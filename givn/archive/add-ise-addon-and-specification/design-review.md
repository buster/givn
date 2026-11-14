# Design Review: add-ise-addon-and-specification

Review kind: independent
Author: add-ise-addon-and-specification author (omp)
Reviewer context: fd3e0fd7-9ae1-4767-b2b2-dc6ddd46984f (the review context that produced the finding brief)

The proposal records `Review kind: independent` and its reason. The reviewer
context produced the ranked finding brief; this authoring context applied the
hardening edits and did not re-decide the review's questions. The reviewer is
not the author and did not receive the author's conversation history beyond
the brief and the artifacts named below.

## User decisions applied before tasks

### A — Merge the glossary now

**Resolution:** the eight confirmed terms (`Specification`, `Source code`,
`End-to-end (E2E) test`, `Constant Ise`, `Ise Test`, `Spec gap`, `Change
strategy`, `Opt-in addon`), the six existing-row rewrites (`Addon`, `Addon
default`, `Feature file`, `Permanent spec`, `Use Case Elaboration`,
`Verification scope`), the five anti-term additions, the two permanent scenario
titles in `givn/specs/fragments/match-guidance-to-enabled-addons.feature`, the
`docs/arc42/01-introduction-and-goals.md` sentence, and the
`docs/domain-model.md` corpus clarification are merged as part of this change,
before tasks. Each statement is true by the end of the change because the same
change lands the registry, the declaration loader, and the two records.

**Consequences:** `docs/glossary.md:8-9,41-45,54,72,85,90,140-142,172,195`
carries the merged vocabulary; `docs/domain-model.md:94` carries the corpus
clarification; the permanent title rename is performed by the delta's
`@givn.removed`/`@givn.added` pair, so archive applies it once. Rows that
describe code commit in the batch whose code makes them true; the glossary is
not committed ahead of the behavior it names.

**Rejected:** deferring the merge to the implementation or a later change.
That fails the handoff rule that no seed may leave a confirmed rule untrue and
would let the new code use `Opt-in addon` and `Specification` while the
project's sole vocabulary authority still rejects them.

### B — `e2e` stays a list of path globs

**Resolution:** the `e2e` role is a list of path globs like the other four. The
union of its matches is evaluated; a package is identified by the nearest
project-relative package manifest walking up from each matched path, bounded at
the project root; the distinct manifests form the dependency-check set,
including transitive path dependencies; zero matches or zero manifests produces
the fourth finding naming the role entry and the reason; the later frozen
digest covers the resolved files, not one directory.

**Consequences:** the design's health table and bullets
(`design.md:333-360`), the `declare-the-specification` rule
(`specs/declare-the-specification/usecase.md:51`), the `verify-change` rules
(`specs/verify-change/usecase.md:67,72-73`), `docs/reference.md:133-142`, and
the missing-package, no-manifest, and transitive-edge scenarios
(`specs/verify-change/check-specification-health.feature:41-78`). No sixth
declaration refusal is added; the `e2e` role is not special-cased in the schema.

**Rejected:** a sixth refusal requiring exactly one directory. It would make
`e2e` unlike the four other roles, duplicate the schema's role list, and make a
legal multi-package acceptance layout undeclarable.

## Grilling by branch

### Scope

**Question:** does the delta contain exactly the proposal's observable
behavior, with no extra gate and no missing surface?

**Resolution:** the proposal's eight bullets map to the seven touched
capabilities and the support fragment: opt-in registration and prerequisite
(`enable-optional-addons`), declaration scaffold (`enable-optional-addons`,
`apply-a-managed-upgrade`), strict schema (`specification-declaration`), four
advisory findings (`check-specification-health`), addon-bound serving
(`serve-extended-rules`), and the two records (`change-strategy`, `spec-gap`).
The blocking boundary gate, `givn ise prepare|verify`, the stack split, and
self-enablement stay out of scope and no scenario asserts them.

**Rejected:** adding a blocking `@e2e` gate or a `givn ise` command to this
change. Both belong to later seeds; adding either would make a project that
never enables `ise` pay a new gate.

### Tech choices

**Question:** is the stack the right fit, and is there a simpler alternative?

**Resolution:** the change stays inside the existing Rust binary, the existing
`serde_yaml_ng` and cucumber-rs runner, and one new `toml` reader for Cargo
manifests. No glob crate is added: a 60-line matcher plus one bounded walker
covers `*`, `**`, `?`, and literals for four advisory findings. No command
invocation is added to `givn lint`.

**Rejected:** `cargo metadata` or `npm ls` (makes lint non-static and can fail
on a missing toolchain) and a general glob dependency (a second walker for one
advisory finding). Recorded in `design.md:141-152` and the risk list.

### Missing scenarios

**Question:** which observable behaviors implied by the proposal were absent?

**Resolution:** the review found four classes and all now have scenarios:
absent declaration and unreadable/malformed/non-list declarations
(`specification-declaration.feature:31-49`, `check-specification-health.feature:6-11`),
the `--no-addons` all-ten case (`enable-optional-addons.feature:19-25`), the
lock written before `ise` existed and the disabled-addon declaration
(`enable-optional-addons.feature:101-122`), and the transitive, no-manifest,
and malformed-manifest edges (`check-specification-health.feature:58-78`).

**Rejected:** treating the empty scaffold as covering absence. The scaffold
writes six empty lists, so it cannot exercise the absent/ unreadable path the
`verify-change` extension requires.

### Testability

**Question:** can every scenario fail in RED with no production code, and are
the assertions concrete?

**Resolution:** every scenario asserts an exit code plus a named path, role,
addon, title, or exact refusal string. The runner is cucumber-rs with
`.fail_on_skipped()` at both registration points. The single-scenario command
is recorded at `design.md:500-503`. The setup batch proves strictness with one
knowingly unbound step. `@wip` is not the strictness mechanism: `run-tests.sh`
selects `@e2e`/`not @e2e` and excludes only `@givn.removed`, while `givn check
tasks` refuses a selected delta scenario carrying `@wip`
(`design.md:528-534,642-645`).

**Rejected:** relying on the scope tag expression to exclude `@wip`. The
runner includes it, so that claim was false and would have misdescribed the RED
proof.

### E2E fidelity

**Question:** are the normalized action scope, real driver, and primary
assertions applied without redefinition?

**Resolution:** the product is a CLI, so every `@e2e` scenario drives the real
`givn` binary as a subprocess; no HTTP client is substituted for a browser
interface, and no browser is invented for a CLI. Each distinct consumer action
has one `@e2e` scenario; refusal cases, flags, and data variants are regular
scenarios. The primary assertion is on stdout/stderr and the exit code produced
by the binary.

**Rejected:** an in-process call to the command functions as E2E evidence. It
would bypass argument parsing and process exit mapping, which are part of the
real interface.

### Interaction Coverage

**Question:** does every inventory entry of every owning use case appear as a
matrix row with a driving mechanism?

**Resolution:** the matrix now has 66 rows: initialize-project 6,
upgrade-a-project 4, declare-the-specification 1, verify-change 13,
obtain-effective-guidance 1, create-change 9, and the support fragment 32. The
row count before hardening was 17 (the four complete inventories plus one
verify-change row, one obtain row, two create-change rows, and two support
rows). Every row names a real CLI invocation and the action it performs.

**Rejected:** mapping only the capabilities whose scenarios this change edits.
The design-review instruction applies the owning use-case inventories, and the
support fragment is included by every touched use case.

### Risk

**Question:** what is the single most likely way this plan fails in
implementation?

**Resolution:** the declaration matcher and the `e2e` package resolution. The
mitigations are a bounded walker with a fixed exclusion set, a flat schema whose
duplicate-key scan is exact, one loader for every role, and scenarios for
transitive, zero-match, zero-manifest, and malformed-manifest inputs. The
second risk is guidance-asset drift between the embedded instruction and the
served text; the mitigation is one resolved `content` value for text and JSON
and scenarios that assert the served text, not the asset file.

**Rejected:** widening the first cut to npm and NuGet manifests to reduce the
limit. That adds two parsers and two naming conventions with no reference
project, and the limit is now stated in the contract and reference.

### Use-case and Persona context

**Question:** do the touched use cases keep their identity, contract,
capability ownership, typed relationships, and confirmed Persona coverage?

**Resolution:** `declare-the-specification` is a new leaf with one goal, one
primary actor, one capability, `Includes: none`, `Extends: none`, and the three
confirmed Personas. The six modified use cases keep their goals, primary
actors, capability lists, and relationships; `create-change` gains two
capabilities, both goal-shaped. No Persona is invented or promoted, and no
Persona biography appears as a Gherkin actor. Running the scope rejection
criteria against the touched contracts: no goal conjunction, no missing primary
actor, no placement failure, no observing-goal mutation, no extension without a
step reference, no inconsistent precondition, no mechanism vocabulary in goal or
guarantees, no component- or channel-shaped name, no oversized leaf, and no
relationship misuse.

**Rejected:** cutting a separate `enable-the-ise-addon` or
`report-the-declaration-warnings` leaf. Both fail the placement test because
their actor goal is already owned by `initialize-project` and `verify-change`.

### Decision ledger

**Question:** does every proposal question resolve or carry with an owner?

**Resolution:** D1 (scaffold values) is decided at `design.md:157-162`; D2
(instruction ownership) at `design.md:163-167`; D3 (manifest coverage) at
`design.md:168-174`. No question is carried: the three answers all support a
scenario in this change, so they are foundational, and carrying them would make
a scenario depend on an unresolved answer. Contracts are current requirements,
not carried questions.

**Rejected:** deferring D3 to implementation because the first Cargo-only cut is
narrow. The proposal requires a decision and the finding's limit is visible to
users, so it is answered now.

### Contracts and components

**Question:** does every consumer use case have one contract and exactly one
implementing component, with no implementation imports?

**Resolution:** `AddonRegistry` → `config::addons`; `SpecificationDeclaration`
→ `config::declaration`; `SpecificationHealth` → the lint health functions;
`InstructionServing` → `cli::instructions::run_artifact`. Each component has one
named responsibility, consumer-shaped methods, and a split trigger. The
registry and declaration live in `config` because `cli` and the lock consume
them; `cli::addons` imports the registry contract and re-exports nothing;
`config` imports no `cli` type. `SpecificationHealth` and `InstructionServing`
are change-local, so they stay in `design.md` rather than the building-block
view. The change has no external service, so no port/adapter split is required;
the one external boundary is the Cargo manifest reader inside the health
component.

**Rejected:** leaving `AddonSpec` in `cli::addons` and having `config::lock`
import it. That reverses the module direction and makes `config` depend on
`cli` for a policy table.

### Decision timing

**Question:** is any foundational decision carried, or any carried question
missing its default, trigger, switch cost, and owner?

**Resolution:** no decision is carried. D1 is a project-visible file format,
D2 decides whether an instruction is served at all, and D3 fixes the reach of
an advertised finding; each is needed by a scenario, so each is answered. No
speculative abstraction is built for a later seed: no `givn ise` command, no
component list, and no frozen digest.

**Rejected:** building the frozen-digest shape now so the next change can reuse
it. The next seed owns that contract; building it here would add an untested
abstraction and a second vocabulary for the `e2e` resolution.

### Visual design contract

**Question:** does the change declare the right interface and evidence?

**Resolution:** `design.md:32-43` records `Interface: terminal`, no viewports,
no states, and a transcript per `@e2e` scenario at
`evidence/visual/<scenario-slug>/transcript.txt`. The design names a terminal
text surface, so the previous `Interface: none` contradicted the evidence
requirement. The modified interactive-init E2E now carries `And the terminal
transcript is captured for this scenario`
(`specs/initialize-project/initialize-a-project.feature:18`), so replacing the
permanent scenario does not drop the evidence obligation.

**Rejected:** keeping `Interface: none` and treating the transcript as
non-visual evidence. `none` is reserved for a change with no rendered screen;
a terminal interface is visual and requires a transcript.

### Deferrals

**Question:** is any mandatory check deferred?

**Resolution:** none. The E2E runner is configured (`./run-tests.sh --e2e`),
every retained `@e2e` tag stays in place, and no operator decision is needed.
The known Cargo-only limit is a documented product limit, not a deferred check:
the boundary finding still runs and names an unresolved package.

**Rejected:** removing or downgrading an `@e2e` tag to avoid the boundary. The
tags are retained and the scenario count is unchanged.

### Persona disposition

**Question:** does every confirmed Persona have a recorded disposition?

**Resolution:** `design.md:616-630` records `developer--primary-user`,
`agent--technical-role`, and `givn-maintainer--owner`. `agent--technical-role`
is the reader of every served instruction and refusal; the developer owns the
enable, declaration, and findings; the maintainer owns the one-time scaffold
and the staged upgrade. A declared visual contract is present, so at least one
confirmed Persona is required; three are.

**Rejected:** recording `none`. The change visibly changes CLI text, which is a
user-visible surface.

### Interface classification

**Question:** is the opposite classification (`web`) defensible?

**Resolution:** no. The product renders no HTML, CSS, or JavaScript; its
interface is the process's stdout/stderr and exit code. `Interface: terminal`
is correct, and a browser driver would test an interface the product does not
have.

**Rejected:** `Interface: web` because the instruction mentions a JSON path.
JSON is a machine-readable CLI output, not a rendered page.

### ADR qualification

**Question:** does the qualified candidate have a MADR entry and a cheaper
home checked?

**Resolution:** `arc42.md:20-37` records `AMEND_ADR` to ADR-010. The cheaper
homes were named: a `specification:` block in `givn/config.yaml` (rejected: the
lenient overlay hides a misspelled block) and convention (rejected: it cannot
express the `source:` partition). The amendment is in
`docs/arc42/adr/ADR-010-manifest-overlay.md:66-78`, the chapter-09 register row
names it, and the consequence (a third read-only authority) is in chapter 08.
The opt-in posture, the prerequisite relation, and the advisory findings were
rejected as ADR candidates with reasons.

**Rejected:** a new ADR for the declaration. ADR-010 already owns configuration
authority; a second record would split one decision across two files.

### Architecture documentation (arc42)

**Question:** is the recorded chapter set independently reproducible, and does
each affected chapter match the design?

**Resolution:** the reviewer walked all 12 rows before opening `arc42.md`:

| # | Chapter | Expected | `arc42.md` | Match |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes — addon posture sentence | Yes | Yes |
| 2 | 02 architecture-constraints | Yes — `TC-14` and the addon constraint | Yes | Yes |
| 3 | 03 context-and-scope | Yes — a new project input file | Yes | Yes |
| 4 | 04 solution-strategy | Yes — addon posture row | Yes | Yes |
| 5 | 05 building-block-view | Yes — registry and declaration components | Yes | Yes |
| 6 | 06 runtime-view | Yes — enable, scaffold, upgrade, and health flows | Yes | Yes |
| 7 | 07 deployment-view | No — single binary, no topology change | No | Yes |
| 8 | 08 crosscutting-concepts | Yes — third configuration authority and exit code | Yes | Yes |
| 9 | 09 architecture-decisions | Yes — ADR-010 amendment | Yes | Yes |
| 10 | 10 quality-requirements | No — no new quality scene or metric | No | Yes |
| 11 | 11 risks-and-technical-debt | Yes — the Cargo-only limit | Yes | Yes |
| 12 | 12 glossary | Yes — the vocabulary changed | Yes | Yes |

All 12 chapter files exist and carry content; chapter 12 is the intended link
to the standalone glossary, not a scaffold. No box-drawing characters or
pseudo-diagrams were found. `05-building-block-view.md:76-78` now names both
`config::addons` and `config::declaration`. Chapter 11 carries `D-28` for the
documented limit; chapter 09 names the ADR-010 amendment.

**Rejected:** treating `arc42.md`'s `STATUS: DONE` as verification. The
reviewer re-derived the table and found the stale statements corrected below
rather than trusting the self-report.

### Ubiquitous language

**Question:** are the design's terms already in `docs/glossary.md`, and are
they used consistently?

**Resolution:** the eight confirmed terms, six rewrites, and five anti-term
additions are merged (`docs/glossary.md:8-9,41-45,54,72,85,90,140-142,172,195`).
`Opt-in addon` names only the default-`false` posture; "an enabled addon"
names any addon switched on, and the two permanent titles were renamed
accordingly. `Spec gap` is not `Disposition`; only `specification defect` is a
gap in the Specification. `Specification` is not used as a compound modifier;
prose says "the declaration of the Specification" or "the declared
Specification".

**Rejected:** adding a `Component` glossary row for `Constant Ise`. The `ise`
addon requires `architecture_principles`, which defines `Component`, so the
term always resolves; a glossary row would create a second definition.

### Structural model

**Question:** does `docs/domain-model.md` reflect the new boundaries?

**Resolution:** the `Permanent specification corpus` aggregate now states that
the corpus is the behavior aggregate of the Specification and does not include
contracts, limits, or the `e2e` package (`docs/domain-model.md:94`). The
`Givn workflow` context already owns the effective guidance and the corpus;
the declaration is a project input read by that context, not a new aggregate,
so no new context or relationship row is warranted.

**Rejected:** adding a `Specification` aggregate. The declaration is a
project-owned input read by existing flows; giving it aggregate ownership would
duplicate the use-case contract and overstate its consistency boundary.

## Blockers found and fixed

| # | Blocker | Fix that landed | Where it shows |
|---|---|---|---|
| 1 | `AddonRegistry` named `cli::addons::KNOWN_ADDONS` while the prose kept the registry in `config` | `AddonSpec` and `KNOWN_ADDONS` move to `src/config/addons.rs`; `cli::addons` consumes and re-exports nothing; `src/config/mod.rs` declares the module; no `config` module imports `cli` | `design.md:7-13,91-95,130,135-139,246-260` |
| 2 | A directory at the declaration path exited 0 and reported preservation | The declaration write checks `is_file()`, not `exists()`; a non-regular path attempts the write and fails by name with the reason and the applied setting; disable reports project-owned | `design.md:288-303`; `specs/initialize-project/enable-optional-addons.feature:125-130` |
| 3 | Empty scaffold contradicted the findings; the walk excluded only the archive | Findings iterate declared non-empty lists only; walk excludes `givn/changes` and `givn/archive` | `design.md:339-350`; `specs/verify-change/check-specification-health.feature:98-104` |
| 4 | An absent declaration loaded as `Ok(None)` with no finding | The blocking declaration finding now covers `Ok(None)` while `ise` is effective, names the path, and states the checks were not performed | `design.md:326-337`; `specs/verify-change/check-specification-health.feature:5-11` |
| 5 | The empty-feature early return preceded any declaration read | The declaration loads before the early return; the ordering is stated in the design | `design.md:326-329` |
| 6 | D3 claimed a Cargo-only limit documented nowhere | The sentence is now in the `check-specification-health` contract and `docs/reference.md`, and D3 is corrected for absent and malformed manifests | `design.md:168-174`; `specs/verify-change/usecase.md:73`; `docs/reference.md:122-142` |
| 7 | Stale arc42 statements about default-on addons and the scaffold trigger | 02 names the opt-in exception; 08 and ADR-010 name the upgrade write and the absent-file condition | `docs/arc42/02-architecture-constraints.md:81-82`; `docs/arc42/08-crosscutting-concepts.md:52-59`; `docs/arc42/adr/ADR-010-manifest-overlay.md:66-78` |
| 8 | Visual contract said `Interface: none` while requiring a transcript | Recorded `Interface: terminal`; the modified interactive-init E2E carries the transcript step | `design.md:32-43`; `specs/initialize-project/initialize-a-project.feature:18` |
| 9 | A scenario title and the matrix used the rejected anti-term `opt-in default` | Retitled to `A consumer lists addons and each declared default is reported`; matrix and use-case row updated | `specs/initialize-project/enable-optional-addons.feature:32`; `design.md:71,548`; `specs/initialize-project/usecase.md:153` |
| 10 | Interaction matrix mapped 17 of 66 inventory rows | All 66 rows now mapped, with counts recorded | `design.md:539-613` |
| 11 | `--no-addons` had no scenario | Added the all-ten explicit-false scenario | `specs/initialize-project/enable-optional-addons.feature:19-25` |
| 12 | `spec-gap` guidance omitted the artifact rule and three of four routes | Proposal guidance covers all four routes and the artifact rule; review guidance covers the E2E stop/restart | `specs/create-change/spec-gap.feature:16-38`; `design.md:415-416` |
| 13 | Missing adversarial scenarios | Added malformed YAML, non-list, unreadable, absent, transitive edge, no manifest, malformed manifest, forced re-init, old lock, and disable-ise preservation | `specs/initialize-project/enable-optional-addons.feature:101-122`; `specs/declare-the-specification/specification-declaration.feature:31-49`; `specs/verify-change/check-specification-health.feature:5-11,58-78` |
| 14 | A missing `e2e` package double-reported through the path-missing and boundary findings | `e2e` is excluded from `specification_path_missing`, whose scope is the four roles other than `e2e`; the boundary finding reports the package once; the scenario asserts one finding | `design.md:334,341-345`; `specs/verify-change/check-specification-health.feature:41-48` |
| 15 | The design claimed `@wip` was excluded by the scope expression | Stated the actual mechanism: the runner selects `@wip`, `givn check tasks` refuses it, and RED uses the single-scenario command | `design.md:528-534,642-645` |
| 16 | "Specification declaration" used the confirmed term as a compound | Prose says "the declaration of the Specification" or "the declared Specification"; the capability id `specification-declaration` is unchanged | `design.md:10-11`; `specs/initialize-project/usecase.md:182` |

## Interaction matrix

Rows before: 17. Rows after: 66. The 49 added rows are the 12 missing
`verify-change` entries, the 7 missing `create-change` entries, and the 30
missing `support` fragment entries; no inventory row was removed.

## Verification

```text
givn lint --change add-ise-addon-and-specification   -> exit 2
  All findings are @wip notices or advisory [SHAPE]/[SUBST] notes; no blocking
  finding. Exit 2 is the expected delta state before implementation.
givn status --change add-ise-addon-and-specification -> design-review next
grep -c '^## ' givn/changes/add-ise-addon-and-specification/specs/*/*.feature
  -> 0 in every file (features use `Feature:`/`Scenario:`, not `##` headings)
```

No grilling question remains unresolved, no decision is carried, every
declared contract has one implementing component, and every hardening edit is
applied. The change may proceed to `tasks.md`.

DESIGN-REVIEW: PASS
