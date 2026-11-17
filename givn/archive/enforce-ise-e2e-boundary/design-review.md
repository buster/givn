# Design Review: enforce-ise-e2e-boundary

Review kind: independent
Author: enforce-ise-e2e-boundary hardening author (omp, db765d4c)
Reviewer context: 9111d088-7854-4cfa-8bdc-08976f308d35 (fresh read-only grilling context; produced the branch coverage, the four questions, and the blocker set)

The proposal records `Review kind: independent` and its reason (blocking
acceptance boundary, second change scope, durable receipt field). The reviewer
context produced the branch coverage, the four questions, and the blocker set;
this authoring context applied the agreed decisions and closures, and recorded
two evidence-based deviations below. No grilling question is unresolved.

## Decisions applied

**A: the `e2e` role is a package set with an import fallback.** The role now
resolves every matched path to its nearest manifest; each distinct package
directory is one resolved package and each is checked on its own manifest,
including transitive and workspace-inherited edges. A matched path that
resolves no manifest is checked by its path-resolving references instead of
being refused. Empty resolution refuses. `e2e` roots and `source` roots are
disjoint both ways: a resolved path inside a `source` root refuses, and a
resolved package whose tree contains a `source` root refuses, which is what a
project-root package produces. `MultiplePackages` and `PackageAtProjectRoot`
are deleted everywhere: `proposal.md:54`, `design.md:193` (D4), `design.md:337`
(Boundary Resolution), `design.md:241` (D8), `specs/verify-change/usecase.md:67`
and `:99`, `docs/reference.md:146`, `docs/arc42/02-architecture-constraints.md:53`,
`docs/arc42/05-building-block-view.md:83`, `docs/arc42/06-runtime-view.md:262`.
The seed's single-manifest rule is recorded as a deviation in D4
(`design.md:206`).

**B: the runner check is textual and no longer overclaims.** `design.md:214`
(D6) states that the rule proves naming, not execution; the result binds the
recorded counts and the scope the runner declared, and a regular-scope run is
refused by the result's `scope` field. R-138 is rewritten
(`docs/arc42/11-risks-and-technical-debt.md:125`) and the design's risk row
(`design.md:678`) drops the "identity binds what actually ran" claim.

**C: strict-mode proof is a real-runner probe.** `design.md:551` deletes the
nonexistent `@allow.skipped` claim, cites the locked `cucumber-0.23.0`
`FailOnSkipped` writer (a skipped event is rewritten to a failed step carrying
`NotFound`, so an unbound step reaches the writer as `failed = 1`), and
prescribes a scratch delta `@e2e` probe through the real runner with the counts
and nonzero exit recorded before the first progress task. The unbound and
skipped refusal scenarios assert distinct counts
(`specs/verify-change/givn-workflow-enforcement.feature:71` and `:79`), and the
fixtures write `failed = 1` and `skipped = 1` respectively
(`design.md:631`).

**D: terminal evidence follows the gate.** `design.md:43` removes the false
`evidence/visual/<scenario-slug>/transcript.txt` path, states that the delta
adds no `@e2e` scenario, that the transcript rule collects nothing, and that
the terminal baseline is the served refusal text. `Interface: terminal` stays.
No deferral is recorded, because there is no `@e2e` subject to defer.

## Blockers

1. **Freshness row 6 is delta-aware.** The table now reads: the delta has an
   `@e2e` scenario and the scope is absent, `not_applicable`, or not green →
   `E2eScopeMissing`; `passed` while the delta has none → `E2eScopeUnexpected`
   (`design.md:418`). The rule and its re-read of the delta are stated at
   `design.md:433`. Both halves are asserted: the check records
   `not_applicable` for a delta without `@e2e`
   (`givn-workflow-enforcement.feature:97`) and the review Outline covers
   `not_applicable` with an `@e2e` delta and `passed` without one
   (`:122`).
2. **The component plan is one-way.** The shared path/executable/bytes hash
   rule moves from `proof` into `src/e2e_boundary/hash.rs`
   (`design.md:110`), the Mermaid diagram draws consumer→provider arrows and
   `freshness --> hash` (`design.md:114`), the primitive table records the
   ownership (`design.md:546`), and the contracts paragraph states that
   `e2e_boundary` depends on no consumer (`design.md:150`). `proof` is the only
   importer, so `proof -> e2e_boundary` is one-way.

## Gaps

3. **Receipt shape.** `design.md:288` and `:313` specify
   `#[serde(default, skip_serializing_if = "Option::is_none")]` on both fields:
   an `ise`-off receipt serializes neither key, not a null, and keeps its
   previous bytes. `docs/reference.md:284` and `:288`,
   `docs/arc42/06-runtime-view.md:496`, the scenario
   (`givn-workflow-enforcement.feature:136`) and the use-case rule
   (`specs/verify-change/usecase.md:117`) say the same.
4. **Missing scenarios.** Added: the empty and sentinel runner-command rows
   (`givn-workflow-enforcement.feature:57`), the import-mode refusal (`:49`),
   and the source-inside-package refusal (`:41`) replacing the project-root
   case; `E2eIdentityUnavailable` as a row (`:105`), `E2eScopeMissing` and
   `E2eScopeUnexpected` as rows (`:122`), the delta-aware `not_applicable`
   scenario (`:97`), and the served `ise` guidance scenario
   (`specs/create-change/prose-consolidation.feature:32`). Plan S3's archive
   exit criterion is stated honestly: the mechanism is the shared receipt gate
   that `givn check review` and `givn archive` both call
   (`design.md:443`), the identity reasons are asserted by the two stale
   Outlines, and the archive path itself by the permanent
   `Archive refuses a stale change verification receipt before merging`.
5. **Wording consistency.** "from" the declared package is used everywhere:
   `assets/instructions/specs.md:139`, the modified pinned scenario
   (`prose-consolidation.feature:9`), the `create-change` rule
   (`specs/create-change/usecase.md:62`), and the seed's replacement sentence,
   which already read "only from" and needed no edit.
6. **The `ise` instruction is corrected.** `assets/instructions/ise.md:20`
   keeps the phrase `two-layer test boundary` and now says component scenarios
   stay in the Specification unchanged while only their step definitions and
   fakes are implementation. The scenario asserts both halves and the phrase
   (`prose-consolidation.feature:32`); the guidance table
   (`design.md:459`) and the `create-change` rule
   (`specs/create-change/usecase.md:80`) record why.
7. **The plan deviation is recorded.** D7a (`design.md:233`) states that plan
   S3's claim was wrong: the tested-content digest already invalidates the
   receipt on a component step edit. The E2E identity stays scoped to the
   resolved packages, and the corrected claim replaces the plan's.
   `docs/arc42/adr/ADR-070-forward-only-archive-proof.md:372` records the same
   consequence.

## Deviations from the brief

1. **No refusal of a command string identical to `verify.command`.** The brief
   asked for it; the served canonical policy this same change keeps admits a
   shared runner whose executions are distinguished by `GIVN_RESULT_SCOPE`
   (`assets/instructions/specs.md:179`), and the modified pinned scenario
   asserts that equal command strings do not invalidate honest isolation
   (`prose-consolidation.feature:17`). The failure mode the clause targets (the
   E2E slot running the regular scope) is already refused by the result's
   `scope` field. Recording the clause would have made the enforcement
   contradict its own pinned scenario, so B's substance (textual, no overclaim)
   is applied without it. If the reviewer wants the refusal anyway, it is one
   condition in D6 plus a row in the runner-command Outline.
2. **The vocabulary is `package`, not `unit`.** The brief's decisions speak of
   units. `docs/glossary.md` already holds `End-to-end (E2E) test`, and
   `Constant Ise` already uses "one-unit scope" for the rebuild scope, so a
   second "unit" noun would collide. Every artifact, including Gherkin, says
   `e2e package`; no new glossary term is introduced.

## Scenario set

`verify-change/givn-workflow-enforcement.feature` (13 scenarios and Outlines):

| Line | Scenario | Rule distinguished |
|---|---|---|
| 6 | declared `e2e` role resolves no readable package | empty resolution; malformed manifest |
| 19 | package reaches a source package | direct, transitive, workspace-inherited edges |
| 33 | path sits under a source root | root inside path |
| 41 | package contains a source root | path inside root (replaces project-root case) |
| 49 | path resolves no package and imports a source package | import fallback |
| 57 | runner command names no declared root | foreign target, empty, sentinel |
| 71 | E2E run leaves a step unbound | `failed = 1` |
| 79 | E2E run reports a skipped step | `skipped = 1` |
| 87 | receipt binds package, runner, version | two-package fixture; per-package digests |
| 97 | delta without `@e2e` records not applicable | delta-aware scope |
| 105 | receipt whose identity no longer matches | five identity stale reasons |
| 122 | receipt whose scope no longer matches the delta | missing or unexpected scope |
| 136 | ise off keeps the receipt without an E2E identity | previous bytes |

`create-change/prose-consolidation.feature` (5 scenarios): two modified pinned
scenarios (lines 6 and 14) and three added (23, 32, 40).

## Branch walk

- **Scope.** The proposal's five behavior bullets map to the 18 delta
  scenarios; nothing extra. "Done" is unambiguous: each refusal names its
  condition, the receipt carries the named fields, the served policy states the
  boundary, and an `ise`-off receipt is byte-identical.
- **Tech choices.** No new dependency. The Cargo reader and the glob/bounded
  walk move rather than duplicate; the import fallback avoids four stack
  readers; cucumber-rs stays the runner. The hash move is the only structural
  change, and it removes a cycle.
- **Missing scenarios.** Six behaviours implied by the decisions were absent
  and are now asserted (gap 4). Two obsolete cases were deleted
  (`MultiplePackages`, project root) and replaced by the rule that subsumes
  them.
- **Testability.** Every Then asserts a named reason, a count, or a receipt
  key; no scenario asserts only "it ran". RED is reachable for the check and
  receipt scenarios: today the check has no boundary path and the receipt has
  no E2E key, so those steps fail before implementation. The three served-text
  scenarios may pass on their first run, because hardening already applied the
  `assets/instructions/specs.md` and `assets/instructions/ise.md` wording; that
  is the permitted "protects a distinct requirement" case, and their value is
  drift prevention, not RED evidence. Strict mode is `.fail_on_skipped()` with
  the real-runner probe (`design.md:562`); the not-implemented pattern is
  stated (`design.md:510`): steps drive the real binary, no stub reports PASS.
  The single-scenario command is named (`design.md:527`).
- **E2E fidelity.** `givn instructions specs` was read; the delta redefines no
  scope, interface, or primary-assertion rule. The interface is a CLI driven as
  a real subprocess; no browser, HTTP, or in-process shortcut is planned. No
  `@e2e` tag is added or removed.
- **Interaction Coverage.** The matrix is unchanged and every row has a
  non-empty driving mechanism (real `givn` subprocess). The delta adds no
  inventory entry: the boundary refusals, the receipt inspection, and the
  served policy are variants of actions that already own their `@e2e`
  scenarios (`design.md:605`).
- **Risk.** Most likely failure: the import fallback refusing a legitimate
  fixture because a relative reference resolves into a `source` root. Bounded:
  the fallback applies only to paths that resolve no package, and R-139 records
  the residual depth. Second: the delta-aware freshness re-read drifting from
  the check; both call `gherkin::parse::has_e2e_scenarios`, and the scope
  Outline asserts both directions.
- **Use-case and Persona context.** Every touched contract element is traced to
  a decision and a scenario (`design.md:66`). Personas are confirmed in the
  proposal and used as review lenses only; Gherkin names `the developer` and
  `the author`, never a Persona. No invented Persona.
- **Decision ledger.** D1 to D8 plus D7a resolve in `## Decisions`
  (`design.md:165`); D1 and D2 keep their seed-question references, D4 keeps
  `seed Q5`. No decision is carried, so no default/trigger/switch-cost/owner
  block is required; contracts are current requirements, not open questions.
- **Contracts and components.** `E2eBoundary` and `ChangeReceipt` are the two
  contracts (`design.md:143`); each has one implementing component, and each
  consumer use case has exactly one contract. No unit owns two contracts,
  capabilities, or adapters. Consumers import the contract; the composition
  root and implementation tests are the only exception. The split trigger is
  named per row.
- **Decision timing.** All four foundational questions (receipt scope, reader
  set, effectiveness, package set) are answered now; nothing speculative is
  built for the deferred stack readers or the `components:` key.
- **Visual design contract.** `Interface: terminal`, no viewports or states,
  no screen introduced or changed. The contract is style-level and the baseline
  list is present (`design.md:50`). No rendered reference is required for a
  terminal change with no `@e2e` scenario; no deferral is recorded and none is
  needed.
- **Deferrals.** None. No mandatory check is deferred; every `@e2e` tag in the
  repository is untouched.
- **Persona disposition.** `developer--primary-user` (reads refusals and the
  receipt; confirmed), `agent--technical-role` (reads the served policy and
  stale reasons; confirmed), `givn-maintainer--owner` (owns the declaration and
  runner command; confirmed). Evidence: proposal `Confirmed Personas`, the
  design's Persona impact table, and the capability scenarios. A user-visible
  terminal surface therefore has confirmed Personas.
- **Interface classification.** Tested the opposite: nothing in the change
  renders HTML, CSS, or JavaScript, so `web` does not hold. The surface is
  terminal text plus a JSON file, so `terminal` is correct and the panel-free
  classification is justified rather than assumed.
- **ADR qualification.** Recorded once in `arc42.md:20` as `AMEND_ADR` to
  ADR-070. The cheaper homes are named: behavior in the `verify-change` and
  `create-change` specs, the field contract in `design.md`, the limits in
  chapter 11. ADR-070 carries alternatives and consequences; its "Bad,
  because" items map to R-140 and R-141. No duplicate qualification worksheet.
- **Architecture documentation.** The 12 rows were re-derived before reading
  `arc42.md`:

  | # | Chapter | My verdict | `arc42.md` | Match |
  |---|---|---|---|---|
  | 1 | 01/10 goals and quality | No | No | yes |
  | 2 | 02 constraints | Yes | Yes | yes |
  | 3 | 03 context and scope | No | No | yes |
  | 4 | 04 strategy | Yes | Yes | yes |
  | 5 | 05 building blocks | Yes | Yes | yes |
  | 6 | 06 runtime | Yes | Yes | yes |
  | 7 | 07 deployment | No | No | yes |
  | 8 | 08 crosscutting | Yes | Yes | yes |
  | 9 | 09 decisions | Yes | Yes | yes |
  | 10 | 10 quality scenarios | No | No | yes |
  | 11 | 11 risks | Yes | Yes | yes |
  | 12 | 12 glossary | Yes (link) | Yes | yes |

  Two self-report cells were wrong and are fixed: row 11 said "Cargo-only
  refusal" (`arc42.md:17`) and row 12 claimed the `E2E identity` term was new
  (`arc42.md:18`), while `docs/glossary.md:73` already records it. All twelve
  chapter files exist with content, and a scan for box-drawing characters and
  `+---`/`|---` pseudo-diagrams found none. Chapter 02 (`TC-45:53`), 05
  (`e2e_boundary:83`), 06 (`262`, `495`, `514`), 09 register (`25`), ADR-070
  (`365`, `378`) and chapter 11 (`125` to `128`) now match `design.md`.
- **Ubiquitous language.** `docs/glossary.md` holds `End-to-end (E2E) test`,
  `E2E identity`, `Change verification receipt`, `Verification scope`,
  `Specification`, and `Source code`; the artifacts use those terms. The sweep
  removed the second noun "unit" (see deviation 2), so no unrecorded term
  remains.
- **Structural model.** The change adds no bounded context, aggregate, or
  domain relationship. `docs/domain-model.md:7` scopes the model to domain
  units and explicitly excludes internal Rust components and file formats, so
  the structural impact is assessed as none and requires no edit; the
  `E2E identity` stays an evidence field, not an aggregate.

## Verification

- `givn lint --change enforce-ise-e2e-boundary` exits 2 with 18 findings, one
  `@wip` per delta scenario (expected before implementation), plus 2 shape
  advisories and 120 `SUBST` advisories from the subset heuristic.
  Disposition: the two stale Outlines stay separate, because one asserts the
  identity values and the other the delta-aware scope, so removing either loses
  a distinct failure. `SUBST` rows pair unrelated scenarios that merely share a
  step count; no action.
- `grep -rn 'exactly one Cargo package\|@allow.skipped\|evidence/visual'
  givn/changes/enforce-ise-e2e-boundary/design.md` returns no line.
- Instruction render probes: with `ise` off the repository serves the `{{else}}`
  branch (`givn instructions specs` line 144); in a throwaway project with
  `ise` enabled, `givn instructions specs` served the declared-package
  paragraph and the shared-runner sentence, and `givn instructions ise` served
  the corrected two-layer bullet. The throwaway project was removed.

DESIGN-REVIEW: PASS
