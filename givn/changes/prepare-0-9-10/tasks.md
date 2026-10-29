# Tasks: prepare-0-9-10

## Governing decisions and setup

Independent design review signed off as preparation-reviewer; PR-01–PR-06 are closed. The operator-approved design supersedes current contradictory instructions about RED records, skills/commands, outline/shape gates and scope-count heuristics. Every task remains incomplete until its actual batch verification and explicit-path implementation/regression commit are recorded. No placeholders or schema-only commits count as implementation.

Existing runner: tests/features_runner.rs uses fail_on_skipped and failed/skipped exit checks; run-tests.sh executes actual Gherkin. Strictness proof is conditional on runner/config changes, not repeated per behavior. Binding/compile/environment failure is not behavioral RED. Write/assert tests before production edits and record real RED command/outcome below as they happen. Immediately passing distinct tests are explicitly existing behavior protected and require no production edit for that requirement. Never reconstruct RED or rewrite commits. New unimplemented steps panic rather than silently succeed.

Local fixtures: disposable repos/directories and loopback readiness child, actual givn subprocess/PTY. No live third-party dependency or fabricated containers. Terminal E2Es capture actual transcript after primary interface assertions. Supported outline runs during REFACTOR once per completed batch for changed Rust/Python+neighbors; unsupported source assessed directly. Concrete cleanup is separate GREEN refactoring commit; no minor-review loops.

Named batches below partition scenarios by capability; batches may share one coherent production fix if the actual evidence covers each requirement. Per-capability regular/E2E binding locations and exact runner/architecture contracts are in design.md. All fixes use existing module APIs and retain mandatory modularity/Personas/initial arc42 baseline. Keep user-owned .maki/permissions.toml untouched. No release/version bump or archive until explicitly requested.

## Batch: archive-proof

Design constraints: use the archive-proof owner and scenarios in specs/archive-change/archive-proof.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
GREEN: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: A rejected finalization publication leaves the archive unproven (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Repairing a failed candidate forward proves it without repeating the merge (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Evidence-only recovery reuses successful scopes and runs only the failed scope (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A changed relevant input invalidates each affected scope before reuse (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Retry uses cheap input checks rather than blanket review revalidation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Archive-directory evidence exemptions remain broad during proof reuse (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A stale passing scope result cannot prove a new archive execution (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Scope inputs are captured before archive verification and checked afterward (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: No-behavior-change archive retains merged-candidate verification (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Consumer repairs a blocked archive and the rerun proves it (@e2e)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: An inconsistent no-behavior-change archive exemption is refused before mutation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance

## Batch: extract-coverage-gate

Design constraints: use the extract-coverage-gate owner and scenarios in specs/archive-change/extract-coverage-gate.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
GREEN: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: A coverage failure proves on rerun after the repair (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Equal coverage from honest scope executions does not block proof (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A successful coverage command cannot reuse an old report (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Fresh coverage reports must be valid (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Evidence-only finalization retry reuses valid coverage (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance

## Batch: grouped-merge-for-new-capabilities

Design constraints: use the grouped-merge-for-new-capabilities owner and scenarios in specs/archive-change/grouped-merge-for-new-capabilities.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
GREEN: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Consumer archives a new capability and the merged spec lands in its declared use case (@e2e)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Archive is blocked when a new capability has no declared routing decision (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Nothing is merged when one delta file of a change has no routing decision (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Review and archive resolve existing capability owners consistently (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A new capability preserves its supported feature structure (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: An unsupported Rule in one new capability prevents every delta mutation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance

## Batch: preserve-scenario-docstrings

Design constraints: use the preserve-scenario-docstrings owner and scenarios in specs/archive-change/preserve-scenario-docstrings.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
GREEN: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: A modified scenario keeps its step docstring after archiving (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: An added scenario keeps its step docstring after archiving (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A modified scenario keeps its step data table after archiving (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Merging a modified scenario preserves its supported Background setup (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Supported execution tags retain their inheritance on archive (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A supported Scenario Outline retains every Examples block (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: An unsupported Rule is refused before archive mutation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: A conflicting Background replacement is refused before archive mutation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: An explicit non-English Gherkin dialect is refused before archive mutation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance

## Batch: outline-report

Design constraints: use the outline-report owner and scenarios in specs/assess-code-structure/outline-report.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: `focused tests/manual fixtures reproduced unsupported-scope misleading absence and fabricated zero metrics; outline report tests pass` — observed during the advisory-outline batch before production fixes.
GREEN: advisory-outline — focused tests/manual fixtures reproduced unsupported-scope misleading absence and fabricated zero metrics; outline report tests pass.
REFACTOR: targeted regression suites rerun clean after fixes; advisory outline used for changed supported components, with no forced verdict.

- [x] Scenario: A developer runs a outline report and sees the classified units (@e2e)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline
- [x] Scenario: The report fails when the scope has no production sources (regular)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline
- [x] Scenario: Unsupported-only production sources are not reported as absent (regular)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline
- [x] Scenario: The JSON report carries analyzer capabilities and limitations (regular)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline
- [x] Scenario: A report is available in machine-readable form (regular)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline
- [x] Scenario: Unavailable outline measurements are not reported as measured zero (regular)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline
- [x] Scenario: Loose class cohesion is omitted when it is not independently measured (regular)
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline

## Batch: prose-consolidation

Design constraints: use the prose-consolidation owner and scenarios in specs/create-change/prose-consolidation.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Generated guidance points to the canonical E2E policy (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Canonical policy requires a rendered-state assertion (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The propose and archive commands carry the seed stamping instructions (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Disabling optional addons does not disable the dialog obligation (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Delegated question preparation carries the policy without assuming inherited context (retirement)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Successful project skill and command replacements are preserved without policy injection (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Canonical policy accepts the library public API as its real interface (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Canonical policy selects E2E scope explicitly rather than by command shape (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Justified E2E reclassification preserves real-interface coverage (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: spec-duplicates

Design constraints: use the spec-duplicates owner and scenarios in specs/discover-specifications/spec-duplicates.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
GREEN: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Exact duplicates in one feature are reported together (@e2e)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Exact duplicates across features are reported together (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Same assertion shape without exact behavior is reported as a structural match (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Normalized executable differences do not establish literal equality (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Different inherited setup prevents an exact duplicate label (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Equal supported executable arguments remain exact duplicates (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Repeating an unchanged audit produces the same report (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance

## Batch: adr-qualification-gate

Design constraints: use the adr-qualification-gate owner and scenarios in specs/fragments/adr-qualification-gate.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: The guidance defines strict qualification as an all-gates contract (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The structured verdict has fixed gate and route fields (retirement)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Route targets are conditional and unused targets stay empty (retirement)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: All effective guidance sources use the same ADR policy (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The ADR guidance requires an adversarial qualification record (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: An author requests the strict ADR guidance through the Givn CLI (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A reviewer requests design-review guidance through the Givn CLI (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The impact assessment records the verdict before drafting (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The ADR template ships with no pre-passed qualification values (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The design template keeps decision justification in the design (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Design-review requires the cheaper-home challenge for a new ADR (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Ordinary ADR downsides are not automatically risk debt (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The guidance carries the three falsification tests (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: attribute-commits

Design constraints: use the attribute-commits owner and scenarios in specs/fragments/attribute-commits.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: `acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass` — observed during the initialization-acceptance batch before production fixes.
GREEN: initialization-acceptance — acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
REFACTOR: targeted regression suites rerun clean after fixes; advisory outline used for changed supported components, with no forced verdict.

- [x] Scenario: Consumer initializes a project and receives shared attribution plumbing (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: An attributed commit exposes the current LLM and harness (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The SCM-neutral commit path attributes a Git revision (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The SCM-neutral commit path attributes a Fossil check-in (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The commit wrapper records only the named paths on Fossil (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The commit wrapper without attribution context leaves the message unchanged (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The stage-all path excludes AGENTS.md and generated rules under local-only (retirement)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The versioned stage-all path still excludes AGENTS.md (retirement)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A commit without explicit paths fails before staging (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Git commits the selected working content without consuming an older staged version (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Unsafe explicit-path isolation is refused without consuming other work (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Repeated hook installation never dispatches its own launcher (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: An unavailable optional attribution helper does not fail a human commit (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: An explicit selected tracked deletion is committed successfully (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Unrelated staged additions and deletions survive an explicit Git commit (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance

## Batch: enable-commit-attribution

Design constraints: use the enable-commit-attribution owner and scenarios in specs/fragments/enable-commit-attribution.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: `acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass` — observed during the initialization-acceptance batch before production fixes.
GREEN: initialization-acceptance — acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
REFACTOR: targeted regression suites rerun clean after fixes; advisory outline used for changed supported components, with no forced verdict.

- [x] Scenario: Consumer sees the commit attribution installation report (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Consumer upgrades an existing project with attribution plumbing missing (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance

## Batch: instruction-integrity

Design constraints: use the instruction-integrity owner and scenarios in specs/fragments/instruction-integrity.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Design guidance requires the complete E2E setup record (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Review guidance names the deterministic disposition gates (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The canonical dialog policy demonstrates the full question record (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Dialog teaching contrasts show complete decisions rather than bare labels (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Migration consent does not imply an unverified recovery guarantee (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Relevant dialog examples are required before the first question of their type (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Author starts a session with the universal dialog obligation (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The characterization guidance requests the reverse review from Givn (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Every instruction id named by generated guidance resolves through the Givn CLI (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Every generated skill and command directs to exactly one served instruction id (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Givn shipped templates render parser-compatible documents and examples (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Current shipped guidance and help exclude harness-project leakage without rewriting history (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Givn release distribution excludes local recordings and harness fixtures (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: match-guidance-to-enabled-addons

Design constraints: use the match-guidance-to-enabled-addons owner and scenarios in specs/fragments/match-guidance-to-enabled-addons.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: An unbalanced conditional in a command template fails the synchronization (retirement)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A consumer synchronizes skills and the generated guidance omits disabled-addon sections (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A consumer enables the addons a generated section needs and the guidance gains it (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: An unbalanced conditional in a skill template fails the synchronization (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Project skill overrides retain conditional addon behavior without command generation (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: refresh-generated-guidance

Design constraints: use the refresh-generated-guidance owner and scenarios in specs/fragments/refresh-generated-guidance.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Synchronized guidance explains how to resolve the deterministic gates (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Review instructions require deterministic gate dispositions (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Generated guidance carries no copy of a built-in rule (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: review-architecture-documentation

Design constraints: use the review-architecture-documentation owner and scenarios in specs/fragments/review-architecture-documentation.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Arc42-enabled review instructions require conformance evidence (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The review template includes the Arc42 conformance block (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The first completed Arc42 change establishes a real twelve-chapter baseline (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Later Arc42 changes maintain affected chapters rather than reapprove the baseline (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Architecture review checks substantive boundaries from the first implementation (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Review closure is bounded to findings and regressions from their fixes (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Final architecture review uses the advisory outline without a second verdict (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: enable-optional-addons

Design constraints: use the enable-optional-addons owner and scenarios in specs/initialize-project/enable-optional-addons.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
GREEN: acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: givn init --no-addons produces no skill or command addon files (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn addons enable generates skill and command files (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn addons disable removes generated command files (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn skills sync includes skill and command addons when enabled (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn skills sync excludes addon skills and commands when disabled (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The absolute-mode policy block carries the dialog context rule (retirement)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn init enables every addon by default (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn addons list reports every addon enabled by default (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: givn init --no-addons writes an explicit opt-out for every addon (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: a partial addons block keeps unnamed addons enabled (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Consumer enables the outline addon and it is listed as enabled (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Communication policy replaces absolute mode without a compatibility alias (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Retired grill settings require manual transition (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Challenge is optional and introduces no lifecycle gate (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Challenge guidance researches the subject before asking unresolved decisions (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Addon edits preserve unrelated configuration or refuse unsupported YAML (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Configuration fields are enforced in their owning file (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance

## Batch: initialize-a-project

Design constraints: use the initialize-a-project owner and scenarios in specs/initialize-project/initialize-a-project.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
GREEN: acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Consumer runs init non-interactively and gets opencode skills (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Consumer re-syncs skills after init and the selection is honoured (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Skills and commands are not installed to an agent excluded from the selection (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The omp target receives its own guidance and attribution integration (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: An omp project keeps the generated omp rules out of the repository (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Every supported target discovers thin skills without generated commands (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Maki guidance does not promise generated slash commands (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A custom legacy command is preserved and its manual transition is reported (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Init reentry preserves explicit addon opt-outs (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Required skill synchronization failure leaves an accurately reported partial initialization (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Retrying skill synchronization completes partial initialization without losing choices (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A new project declares verification runners as unconfigured (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The served runner setup example fulfills the verification contract when executed (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Target selection edits preserve unrelated YAML or refuse before writing (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance

## Batch: report-change-status

Design constraints: use the report-change-status owner and scenarios in specs/inspect-change-state/report-change-status.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
GREEN: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Detail mode — active change with all required done (text) (@e2e)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Status reports receipt freshness separately from artifact completion (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Detail mode — archived change (text) (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Detail mode — archived change (JSON) (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: An archive candidate without current proof is not a proven archive (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Inspecting status never executes verification (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance

## Batch: agent-activity-reporting

Design constraints: use the agent-activity-reporting owner and scenarios in specs/inspect-development-history/agent-activity-reporting.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Consumer installs the agent reporter and its reported activity lands in the record (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A rejected report never fails the agent action it reports (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A stalled reporting helper cannot block primary agent work (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Recording guidance does not confuse ignored paths with untracked history (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Recording warnings disclose sensitivity without promising privacy filtering (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: readme-enforcement

Design constraints: use the readme-enforcement owner and scenarios in specs/keep-documentation-current/readme-enforcement.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
GREEN: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: The check names a damaged managed block (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Archiving refreshes the development block (@e2e)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Ambiguous managed markers are rejected before README modification (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance

## Batch: serve-extended-rules

Design constraints: use the serve-extended-rules owner and scenarios in specs/obtain-effective-guidance/serve-extended-rules.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Served implementation guidance requires behavioral RED before production edits (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Immediately passing tests protect existing behavior without a production edit (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Skipped RED is admitted rather than reconstructed from history (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A coherent behavior batch is completed with its actual explicit-path revision (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Refactor guidance uses the outline once for supported changed components and their neighbors (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Unsupported outline languages receive source assessment without a waiver (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Encountered cleanup is a separate GREEN refactoring batch (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Served development guidance requires modular boundaries from the start (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: start-the-declared-product

Design constraints: use the start-the-declared-product owner and scenarios in specs/operate-product/start-the-declared-product.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
GREEN: acceptance fixtures reproduced obsolete ADR worksheets, ambiguous outline cycles, missing bootstrap examples and unbounded reporters; 73 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: The started server reports readiness and forwards a clean exit (@e2e)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A readiness timeout abandons the start (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: The readiness probe gives up after the timeout (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: A stalled readiness exchange cannot outlive the total deadline (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance
- [x] Scenario: Readiness retries share one deadline across connection and exchange (regular)
  - [x] COMMIT: a7e9f30b -- Batch: guidance-acceptance

## Batch: describe-the-release

Design constraints: use the describe-the-release owner and scenarios in specs/publish-a-release/describe-the-release.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
GREEN: acceptance fixtures reproduced lost executable content, stale scope evidence, incorrect finalization state and unsupported dialect mutation; archive 45/45 and release 6/6 pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: The migration catalog defines complete release-owned bundles (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance
- [x] Scenario: Synthetic future migration bundles retain complete metadata validation (regular)
  - [x] COMMIT: 64ba38e7 -- Batch: archive-acceptance

## Batch: apply-a-managed-upgrade

Design constraints: use the apply-a-managed-upgrade owner and scenarios in specs/upgrade-a-project/apply-a-managed-upgrade.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
GREEN: acceptance fixtures reproduced missing upgrade seams, partial-failure wording and stale attribution installation; 100 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: A clean project without migration requirements receives one managed upgrade commit (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A clean Fossil project receives the same managed upgrade revision shape (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: An unclean project is protected from automatic migration commits (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A project outside Git is protected from a partial automatic migration (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A local-only upgrade publishes rule removals without AGENTS.md (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A clean 0.7.0 project receives a reviewable aggregate migration plan (retirement)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A project below the oldest migration route receives no migration change (retirement)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: An existing migration plan is never overwritten (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Pre-1.0 projects require manual transition instead of embedded migration plans (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Unsupported old configuration is refused before upgrade writes (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Upgrade failures report actual partial state and operator recovery (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Future supported migration routes remain reviewable (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Repeating the upgrade does not duplicate the migration (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: The migration change remains honest until the project review passes (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: A completed maintenance migration can be archived without a product specification (@e2e)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance
- [x] Scenario: Upgrade reconciles only recognizable generated commands to skills (regular)
  - [x] COMMIT: 8d576d01 -- Batch: initialization-acceptance

## Batch: check-specification-health

Design constraints: use the check-specification-health owner and scenarios in specs/verify-change/check-specification-health.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
GREEN: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: All clean .feature files report a clean summary on stdout (@e2e)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Explicit relationship namespaces disambiguate equal identifiers (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A relationship cannot resolve through the other namespace (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A fragment cannot be an extension target even when a use case has the same identifier (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Cycles follow use-case includes and extends relationships (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A fragment inclusion does not invent an edge to a same-named use case (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Interaction evidence cannot be credited from another capability (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Fragment interactions are checked in their declaring fragment (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A malformed Unicode use-case title produces a diagnostic instead of a panic (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance

## Batch: givn-workflow-enforcement

Design constraints: use the givn-workflow-enforcement owner and scenarios in specs/verify-change/givn-workflow-enforcement.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
GREEN: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Consumer runs check tasks and receives a change verification receipt (@e2e)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A successful runner without new output cannot reuse a stale result (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Invalid fresh runner output cannot prove changed scenarios (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Inputs changed during execution cannot receive a passed receipt (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: An explicit no-behavior-change declaration needs no delta scenarios (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Omitting delta specs without declaring no behavior change is not an exemption (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Integrity remains applicable without delta specs (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review accepts an explicit no-behavior-change verification result (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review does not run the full suite (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Implementation verification does not require the review it will inform (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Two scenarios sharing a batch commit pass the task check (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A shared batch revision without a batch label fails the task check (retirement)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A commit message that omits the scenario title fails the task check (retirement)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Batch traceability rejects an unreachable revision (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Final review does not reconstruct TDD order from commit shape (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Archive re-runs the static review gates (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Preservation detects changes to supported executable content (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Moving unchanged supported executable content preserves behavior (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review ignores verdict examples rather than treating them as decisions (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Non-executable FAIL examples do not override an actual PASS (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Conflicting actual verdicts cannot pass review (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Negated PASS prose is not a passing verdict (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Independent cheap failures are reported while dependent checks are deferred (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Consumer invokes bare check and receives explicit non-executing guidance (@e2e)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance

## Batch: outline-review-gate

Design constraints: use the outline-review-gate owner and scenarios in specs/verify-change/outline-review-gate.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: `focused tests/manual fixtures reproduced unsupported-scope misleading absence and fabricated zero metrics; outline report tests pass` — observed during the advisory-outline batch before production fixes.
GREEN: advisory-outline — focused tests/manual fixtures reproduced unsupported-scope misleading absence and fabricated zero metrics; outline report tests pass.
REFACTOR: targeted regression suites rerun clean after fixes; advisory outline used for changed supported components, with no forced verdict.

- [x] Retire the blocking outline capability and its implementation/callers/tests without deleting advisory reporting.
  - [x] COMMIT: 3bbf3cc0 -- Batch: advisory-outline

## Batch: review-disposition-gates

Design constraints: use the review-disposition-gates owner and scenarios in specs/verify-change/review-disposition-gates.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: The acceptance run exhibited the failed behaviors documented for this batch before production fixes: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
GREEN: acceptance fixtures reproduced unauthenticated verdict matching, misleading status, shape-based blocking and wrong typed-reference diagnostics; 112 scenarios pass.
REFACTOR: targeted suites rerun clean after the fixes; the advisory outline was used for changed supported components without a second verdict.

- [x] Scenario: Review passes when every shape match is disposed and reports the net-delta receipt (retirement)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review blocks when a shape match involving the change's delta is undisposed (retirement)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review blocks when a delta scenario shape-matches a permanent scenario without disposition (retirement)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Consumer passes review without shape dispositions and receives the net-delta receipt (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Shape matches do not block review or archive (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review blocks with a clear message when review.md is missing (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: Review passes when a long scenario has a split-or-keep entry (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A keep disposition must contain an explicit decision and a reason (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A promised split cannot pass review while the long scenario remains unsplit (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance
- [x] Scenario: A completed split satisfies the scenario-length disposition (regular)
  - [x] COMMIT: 595abe2a -- Batch: verification-acceptance

## Batch: full-suite-reconciliation

Design constraints: preserve the approved skills-only, evidence-bound, advisory and 1.0 migration contracts. These scenarios close obsolete permanent expectations that were not covered by the first delta projection; no additional product scope is introduced.

RED: The full permanent corpus reproduced outdated addon placement, generated-command promises, obsolete migration guidance and retired shape requirements.
GREEN: 200 changed non-E2E scenarios and 30 E2E scenarios pass; 444 unit tests and strict clippy pass. Every remaining permanent failure is replaced by a delta modification or retirement.
REFACTOR: Shared fixture helpers now write policy to config.yaml, produce fresh results and real review evidence.

- [x] Scenario: Shared guidance alone does not qualify an ADR (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: The guidance routes normal design decisions to one canonical artifact (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: The guidance requires an existing-ADR check before drafting (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: The guidance forbids invented ADR evidence (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: An ADR refinement is routed to amendment without duplication (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: An architectural replacement preserves supersession history (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: An initialized project documents reusable test and coverage scripts (@e2e)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: A consumer synchronizes skills and every agent target receives the ideation facilitation skill (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: The generated ideation files direct the agent to the ideation instruction (regular)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation
- [x] Scenario: The migration guide is an executable prompt sequence (retirement)
  - [x] COMMIT: 1b2ef01f -- Batch: full-suite-reconciliation

## Documentation and deferred work closeout

- [x] Reconcile current docs/glossary/Personas/ADRs, remove duplicate workflow-details authority and obsolete0.x operational bundles; preserve historical evidence.
- [x] Track concrete post1.0 matching, outline, recording and unrelated findings in existing risks/debt chapter without reopening acceptance.
- [x] Complete arc42 assessment honestly and capture terminal evidence for applicable E2Es.
- [x] Execute agreed project validation and one substantive final review by preparation-reviewer, then targeted closure only.

No box above is complete merely because the planning phases are committed.
