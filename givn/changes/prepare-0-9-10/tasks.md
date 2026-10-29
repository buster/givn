# Tasks: prepare-0-9-10

## Governing decisions and setup

Independent design review signed off as preparation-reviewer; PR-01–PR-06 are closed. The operator-approved design supersedes current contradictory instructions about RED records, skills/commands, outline/shape gates and scope-count heuristics. Every task remains incomplete until its actual batch verification and explicit-path implementation/regression commit are recorded. No placeholders or schema-only commits count as implementation.

Existing runner: tests/features_runner.rs uses fail_on_skipped and failed/skipped exit checks; run-tests.sh executes actual Gherkin. Strictness proof is conditional on runner/config changes, not repeated per behavior. Binding/compile/environment failure is not behavioral RED. Write/assert tests before production edits and record real RED command/outcome below as they happen. Immediately passing distinct tests are explicitly existing behavior protected and require no production edit for that requirement. Never reconstruct RED or rewrite commits. New unimplemented steps panic rather than silently succeed.

Local fixtures: disposable repos/directories and loopback readiness child, actual givn subprocess/PTY. No live third-party dependency or fabricated containers. Terminal E2Es capture actual transcript after primary interface assertions. Supported outline runs during REFACTOR once per completed batch for changed Rust/Python+neighbors; unsupported source assessed directly. Concrete cleanup is separate GREEN refactoring commit; no minor-review loops.

Named batches below partition scenarios by capability; batches may share one coherent production fix if the actual evidence covers each requirement. Per-capability regular/E2E binding locations and exact runner/architecture contracts are in design.md. All fixes use existing module APIs and retain mandatory modularity/Personas/initial arc42 baseline. Keep user-owned .maki/permissions.toml untouched. No release/version bump or archive until explicitly requested.

## Batch: archive-proof

Design constraints: use the archive-proof owner and scenarios in specs/archive-change/archive-proof.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: A rejected finalization publication leaves the archive unproven (regular)
  COMMIT: not recorded
- [ ] Scenario: Repairing a failed candidate forward proves it without repeating the merge (regular)
  COMMIT: not recorded
- [ ] Scenario: Evidence-only recovery reuses successful scopes and runs only the failed scope (regular)
  COMMIT: not recorded
- [ ] Scenario: A changed relevant input invalidates each affected scope before reuse (regular)
  COMMIT: not recorded
- [ ] Scenario: Retry uses cheap input checks rather than blanket review revalidation (regular)
  COMMIT: not recorded
- [ ] Scenario: Archive-directory evidence exemptions remain broad during proof reuse (regular)
  COMMIT: not recorded
- [ ] Scenario: A stale passing scope result cannot prove a new archive execution (regular)
  COMMIT: not recorded
- [ ] Scenario: Scope inputs are captured before archive verification and checked afterward (regular)
  COMMIT: not recorded
- [ ] Scenario: No-behavior-change archive retains merged-candidate verification (regular)
  COMMIT: not recorded
- [ ] Scenario: Consumer repairs a blocked archive and the rerun proves it (@e2e)
  COMMIT: not recorded
- [ ] Scenario: An inconsistent no-behavior-change archive exemption is refused before mutation (regular)
  COMMIT: not recorded

## Batch: extract-coverage-gate

Design constraints: use the extract-coverage-gate owner and scenarios in specs/archive-change/extract-coverage-gate.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: A coverage failure proves on rerun after the repair (regular)
  COMMIT: not recorded
- [ ] Scenario: Equal coverage from honest scope executions does not block proof (regular)
  COMMIT: not recorded
- [ ] Scenario: A successful coverage command cannot reuse an old report (regular)
  COMMIT: not recorded
- [ ] Scenario: Fresh coverage reports must be valid (regular)
  COMMIT: not recorded
- [ ] Scenario: Evidence-only finalization retry reuses valid coverage (regular)
  COMMIT: not recorded

## Batch: grouped-merge-for-new-capabilities

Design constraints: use the grouped-merge-for-new-capabilities owner and scenarios in specs/archive-change/grouped-merge-for-new-capabilities.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Consumer archives a new capability and the merged spec lands in its declared use case (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Archive is blocked when a new capability has no declared routing decision (regular)
  COMMIT: not recorded
- [ ] Scenario: Nothing is merged when one delta file of a change has no routing decision (regular)
  COMMIT: not recorded
- [ ] Scenario: Review and archive resolve existing capability owners consistently (regular)
  COMMIT: not recorded
- [ ] Scenario: A new capability preserves its supported feature structure (regular)
  COMMIT: not recorded
- [ ] Scenario: An unsupported Rule in one new capability prevents every delta mutation (regular)
  COMMIT: not recorded

## Batch: preserve-scenario-docstrings

Design constraints: use the preserve-scenario-docstrings owner and scenarios in specs/archive-change/preserve-scenario-docstrings.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: A modified scenario keeps its step docstring after archiving (regular)
  COMMIT: not recorded
- [ ] Scenario: An added scenario keeps its step docstring after archiving (regular)
  COMMIT: not recorded
- [ ] Scenario: A modified scenario keeps its step data table after archiving (regular)
  COMMIT: not recorded
- [ ] Scenario: Merging a modified scenario preserves its supported Background setup (regular)
  COMMIT: not recorded
- [ ] Scenario: Supported execution tags retain their inheritance on archive (regular)
  COMMIT: not recorded
- [ ] Scenario: A supported Scenario Outline retains every Examples block (regular)
  COMMIT: not recorded
- [ ] Scenario: An unsupported Rule is refused before archive mutation (regular)
  COMMIT: not recorded
- [ ] Scenario: A conflicting Background replacement is refused before archive mutation (regular)
  COMMIT: not recorded
- [ ] Scenario: An explicit non-English Gherkin dialect is refused before archive mutation (regular)
  COMMIT: not recorded

## Batch: outline-report

Design constraints: use the outline-report owner and scenarios in specs/assess-code-structure/outline-report.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: A developer runs a outline report and sees the classified units (@e2e)
  COMMIT: not recorded
- [ ] Scenario: The report fails when the scope has no production sources (regular)
  COMMIT: not recorded
- [ ] Scenario: Unsupported-only production sources are not reported as absent (regular)
  COMMIT: not recorded
- [ ] Scenario: The JSON report carries analyzer capabilities and limitations (regular)
  COMMIT: not recorded
- [ ] Scenario: A report is available in machine-readable form (regular)
  COMMIT: not recorded
- [ ] Scenario: Unavailable outline measurements are not reported as measured zero (regular)
  COMMIT: not recorded
- [ ] Scenario: Loose class cohesion is omitted when it is not independently measured (regular)
  COMMIT: not recorded

## Batch: prose-consolidation

Design constraints: use the prose-consolidation owner and scenarios in specs/create-change/prose-consolidation.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Generated guidance points to the canonical E2E policy (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Canonical policy requires a rendered-state assertion (regular)
  COMMIT: not recorded
- [ ] Scenario: The propose and archive commands carry the seed stamping instructions (regular)
  COMMIT: not recorded
- [ ] Scenario: Disabling optional addons does not disable the dialog obligation (regular)
  COMMIT: not recorded
- [ ] Scenario: Delegated question preparation carries the policy without assuming inherited context (retirement)
  COMMIT: not recorded
- [ ] Scenario: Successful project skill and command replacements are preserved without policy injection (regular)
  COMMIT: not recorded
- [ ] Scenario: Canonical policy accepts the library public API as its real interface (regular)
  COMMIT: not recorded
- [ ] Scenario: Canonical policy selects E2E scope explicitly rather than by command shape (regular)
  COMMIT: not recorded
- [ ] Scenario: Justified E2E reclassification preserves real-interface coverage (regular)
  COMMIT: not recorded

## Batch: spec-duplicates

Design constraints: use the spec-duplicates owner and scenarios in specs/discover-specifications/spec-duplicates.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Exact duplicates in one feature are reported together (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Exact duplicates across features are reported together (regular)
  COMMIT: not recorded
- [ ] Scenario: Same assertion shape without exact behavior is reported as a structural match (regular)
  COMMIT: not recorded
- [ ] Scenario: Normalized executable differences do not establish literal equality (regular)
  COMMIT: not recorded
- [ ] Scenario: Different inherited setup prevents an exact duplicate label (regular)
  COMMIT: not recorded
- [ ] Scenario: Equal supported executable arguments remain exact duplicates (regular)
  COMMIT: not recorded
- [ ] Scenario: Repeating an unchanged audit produces the same report (regular)
  COMMIT: not recorded

## Batch: adr-qualification-gate

Design constraints: use the adr-qualification-gate owner and scenarios in specs/fragments/adr-qualification-gate.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: The guidance defines strict qualification as an all-gates contract (regular)
  COMMIT: not recorded
- [ ] Scenario: The structured verdict has fixed gate and route fields (retirement)
  COMMIT: not recorded
- [ ] Scenario: Route targets are conditional and unused targets stay empty (retirement)
  COMMIT: not recorded
- [ ] Scenario: All effective guidance sources use the same ADR policy (regular)
  COMMIT: not recorded
- [ ] Scenario: The ADR guidance requires an adversarial qualification record (regular)
  COMMIT: not recorded
- [ ] Scenario: An author requests the strict ADR guidance through the Givn CLI (@e2e)
  COMMIT: not recorded
- [ ] Scenario: A reviewer requests design-review guidance through the Givn CLI (regular)
  COMMIT: not recorded
- [ ] Scenario: The impact assessment records the verdict before drafting (regular)
  COMMIT: not recorded
- [ ] Scenario: The ADR template ships with no pre-passed qualification values (regular)
  COMMIT: not recorded
- [ ] Scenario: The design template keeps decision justification in the design (regular)
  COMMIT: not recorded
- [ ] Scenario: Design-review requires the cheaper-home challenge for a new ADR (regular)
  COMMIT: not recorded
- [ ] Scenario: Ordinary ADR downsides are not automatically risk debt (regular)
  COMMIT: not recorded
- [ ] Scenario: The guidance carries the three falsification tests (regular)
  COMMIT: not recorded

## Batch: attribute-commits

Design constraints: use the attribute-commits owner and scenarios in specs/fragments/attribute-commits.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Consumer initializes a project and receives shared attribution plumbing (@e2e)
  COMMIT: not recorded
- [ ] Scenario: An attributed commit exposes the current LLM and harness (@e2e)
  COMMIT: not recorded
- [ ] Scenario: The SCM-neutral commit path attributes a Git revision (regular)
  COMMIT: not recorded
- [ ] Scenario: The SCM-neutral commit path attributes a Fossil check-in (regular)
  COMMIT: not recorded
- [ ] Scenario: The commit wrapper records only the named paths on Fossil (regular)
  COMMIT: not recorded
- [ ] Scenario: The commit wrapper without attribution context leaves the message unchanged (regular)
  COMMIT: not recorded
- [ ] Scenario: The stage-all path excludes AGENTS.md and generated rules under local-only (retirement)
  COMMIT: not recorded
- [ ] Scenario: The versioned stage-all path still excludes AGENTS.md (retirement)
  COMMIT: not recorded
- [ ] Scenario: A commit without explicit paths fails before staging (regular)
  COMMIT: not recorded
- [ ] Scenario: Git commits the selected working content without consuming an older staged version (regular)
  COMMIT: not recorded
- [ ] Scenario: Unsafe explicit-path isolation is refused without consuming other work (regular)
  COMMIT: not recorded
- [ ] Scenario: Repeated hook installation never dispatches its own launcher (regular)
  COMMIT: not recorded
- [ ] Scenario: An unavailable optional attribution helper does not fail a human commit (regular)
  COMMIT: not recorded
- [ ] Scenario: An explicit selected tracked deletion is committed successfully (regular)
  COMMIT: not recorded
- [ ] Scenario: Unrelated staged additions and deletions survive an explicit Git commit (regular)
  COMMIT: not recorded

## Batch: enable-commit-attribution

Design constraints: use the enable-commit-attribution owner and scenarios in specs/fragments/enable-commit-attribution.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Consumer sees the commit attribution installation report (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Consumer upgrades an existing project with attribution plumbing missing (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook (@e2e)
  COMMIT: not recorded

## Batch: instruction-integrity

Design constraints: use the instruction-integrity owner and scenarios in specs/fragments/instruction-integrity.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Design guidance requires the complete E2E setup record (regular)
  COMMIT: not recorded
- [ ] Scenario: Review guidance names the deterministic disposition gates (regular)
  COMMIT: not recorded
- [ ] Scenario: The canonical dialog policy demonstrates the full question record (regular)
  COMMIT: not recorded
- [ ] Scenario: Dialog teaching contrasts show complete decisions rather than bare labels (regular)
  COMMIT: not recorded
- [ ] Scenario: Migration consent does not imply an unverified recovery guarantee (regular)
  COMMIT: not recorded
- [ ] Scenario: Relevant dialog examples are required before the first question of their type (regular)
  COMMIT: not recorded
- [ ] Scenario: Author starts a session with the universal dialog obligation (@e2e)
  COMMIT: not recorded
- [ ] Scenario: The characterization guidance requests the reverse review from Givn (regular)
  COMMIT: not recorded
- [ ] Scenario: Every instruction id named by generated guidance resolves through the Givn CLI (regular)
  COMMIT: not recorded
- [ ] Scenario: Every generated skill and command directs to exactly one served instruction id (regular)
  COMMIT: not recorded
- [ ] Scenario: Givn shipped templates render parser-compatible documents and examples (regular)
  COMMIT: not recorded
- [ ] Scenario: Current shipped guidance and help exclude harness-project leakage without rewriting history (regular)
  COMMIT: not recorded
- [ ] Scenario: Givn release distribution excludes local recordings and harness fixtures (regular)
  COMMIT: not recorded

## Batch: match-guidance-to-enabled-addons

Design constraints: use the match-guidance-to-enabled-addons owner and scenarios in specs/fragments/match-guidance-to-enabled-addons.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled (regular)
  COMMIT: not recorded
- [ ] Scenario: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled (regular)
  COMMIT: not recorded
- [ ] Scenario: An unbalanced conditional in a command template fails the synchronization (retirement)
  COMMIT: not recorded
- [ ] Scenario: A consumer synchronizes skills and the generated guidance omits disabled-addon sections (@e2e)
  COMMIT: not recorded
- [ ] Scenario: A consumer enables the addons a generated section needs and the guidance gains it (@e2e)
  COMMIT: not recorded
- [ ] Scenario: An unbalanced conditional in a skill template fails the synchronization (regular)
  COMMIT: not recorded
- [ ] Scenario: Project skill overrides retain conditional addon behavior without command generation (regular)
  COMMIT: not recorded

## Batch: refresh-generated-guidance

Design constraints: use the refresh-generated-guidance owner and scenarios in specs/fragments/refresh-generated-guidance.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Synchronized guidance explains how to resolve the deterministic gates (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Review instructions require deterministic gate dispositions (regular)
  COMMIT: not recorded
- [ ] Scenario: Generated guidance carries no copy of a built-in rule (regular)
  COMMIT: not recorded

## Batch: review-architecture-documentation

Design constraints: use the review-architecture-documentation owner and scenarios in specs/fragments/review-architecture-documentation.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Arc42-enabled review instructions require conformance evidence (@e2e)
  COMMIT: not recorded
- [ ] Scenario: The review template includes the Arc42 conformance block (@e2e)
  COMMIT: not recorded
- [ ] Scenario: The first completed Arc42 change establishes a real twelve-chapter baseline (regular)
  COMMIT: not recorded
- [ ] Scenario: Later Arc42 changes maintain affected chapters rather than reapprove the baseline (regular)
  COMMIT: not recorded
- [ ] Scenario: Architecture review checks substantive boundaries from the first implementation (regular)
  COMMIT: not recorded
- [ ] Scenario: Review closure is bounded to findings and regressions from their fixes (regular)
  COMMIT: not recorded
- [ ] Scenario: Final architecture review uses the advisory outline without a second verdict (regular)
  COMMIT: not recorded

## Batch: enable-optional-addons

Design constraints: use the enable-optional-addons owner and scenarios in specs/initialize-project/enable-optional-addons.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: givn init --no-addons produces no skill or command addon files (regular)
  COMMIT: not recorded
- [ ] Scenario: givn addons enable generates skill and command files (regular)
  COMMIT: not recorded
- [ ] Scenario: givn addons disable removes generated command files (regular)
  COMMIT: not recorded
- [ ] Scenario: givn skills sync includes skill and command addons when enabled (regular)
  COMMIT: not recorded
- [ ] Scenario: givn skills sync excludes addon skills and commands when disabled (regular)
  COMMIT: not recorded
- [ ] Scenario: The absolute-mode policy block carries the dialog context rule (retirement)
  COMMIT: not recorded
- [ ] Scenario: givn init enables every addon by default (regular)
  COMMIT: not recorded
- [ ] Scenario: givn addons list reports every addon enabled by default (regular)
  COMMIT: not recorded
- [ ] Scenario: givn init --no-addons writes an explicit opt-out for every addon (regular)
  COMMIT: not recorded
- [ ] Scenario: a partial addons block keeps unnamed addons enabled (regular)
  COMMIT: not recorded
- [ ] Scenario: Consumer enables the outline addon and it is listed as enabled (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Communication policy replaces absolute mode without a compatibility alias (regular)
  COMMIT: not recorded
- [ ] Scenario: Retired grill settings require manual transition (regular)
  COMMIT: not recorded
- [ ] Scenario: Challenge is optional and introduces no lifecycle gate (regular)
  COMMIT: not recorded
- [ ] Scenario: Challenge guidance researches the subject before asking unresolved decisions (regular)
  COMMIT: not recorded
- [ ] Scenario: Addon edits preserve unrelated configuration or refuse unsupported YAML (regular)
  COMMIT: not recorded
- [ ] Scenario: Configuration fields are enforced in their owning file (regular)
  COMMIT: not recorded

## Batch: initialize-a-project

Design constraints: use the initialize-a-project owner and scenarios in specs/initialize-project/initialize-a-project.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Consumer runs init non-interactively and gets opencode skills (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Consumer re-syncs skills after init and the selection is honoured (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Skills and commands are not installed to an agent excluded from the selection (regular)
  COMMIT: not recorded
- [ ] Scenario: The omp target receives its own guidance and attribution integration (regular)
  COMMIT: not recorded
- [ ] Scenario: An omp project keeps the generated omp rules out of the repository (regular)
  COMMIT: not recorded
- [ ] Scenario: Every supported target discovers thin skills without generated commands (regular)
  COMMIT: not recorded
- [ ] Scenario: Maki guidance does not promise generated slash commands (regular)
  COMMIT: not recorded
- [ ] Scenario: A custom legacy command is preserved and its manual transition is reported (regular)
  COMMIT: not recorded
- [ ] Scenario: Init reentry preserves explicit addon opt-outs (regular)
  COMMIT: not recorded
- [ ] Scenario: Required skill synchronization failure leaves an accurately reported partial initialization (regular)
  COMMIT: not recorded
- [ ] Scenario: Retrying skill synchronization completes partial initialization without losing choices (regular)
  COMMIT: not recorded
- [ ] Scenario: A new project declares verification runners as unconfigured (regular)
  COMMIT: not recorded
- [ ] Scenario: The served runner setup example fulfills the verification contract when executed (regular)
  COMMIT: not recorded
- [ ] Scenario: Target selection edits preserve unrelated YAML or refuse before writing (regular)
  COMMIT: not recorded

## Batch: report-change-status

Design constraints: use the report-change-status owner and scenarios in specs/inspect-change-state/report-change-status.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Detail mode — active change with all required done (text) (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Status reports receipt freshness separately from artifact completion (regular)
  COMMIT: not recorded
- [ ] Scenario: Detail mode — archived change (text) (regular)
  COMMIT: not recorded
- [ ] Scenario: Detail mode — archived change (JSON) (regular)
  COMMIT: not recorded
- [ ] Scenario: An archive candidate without current proof is not a proven archive (regular)
  COMMIT: not recorded
- [ ] Scenario: Inspecting status never executes verification (regular)
  COMMIT: not recorded

## Batch: agent-activity-reporting

Design constraints: use the agent-activity-reporting owner and scenarios in specs/inspect-development-history/agent-activity-reporting.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Consumer installs the agent reporter and its reported activity lands in the record (@e2e)
  COMMIT: not recorded
- [ ] Scenario: A rejected report never fails the agent action it reports (regular)
  COMMIT: not recorded
- [ ] Scenario: A stalled reporting helper cannot block primary agent work (regular)
  COMMIT: not recorded
- [ ] Scenario: Recording guidance does not confuse ignored paths with untracked history (regular)
  COMMIT: not recorded
- [ ] Scenario: Recording warnings disclose sensitivity without promising privacy filtering (regular)
  COMMIT: not recorded

## Batch: readme-enforcement

Design constraints: use the readme-enforcement owner and scenarios in specs/keep-documentation-current/readme-enforcement.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: The check names a damaged managed block (regular)
  COMMIT: not recorded
- [ ] Scenario: Archiving refreshes the development block (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Ambiguous managed markers are rejected before README modification (regular)
  COMMIT: not recorded

## Batch: serve-extended-rules

Design constraints: use the serve-extended-rules owner and scenarios in specs/obtain-effective-guidance/serve-extended-rules.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Served implementation guidance requires behavioral RED before production edits (regular)
  COMMIT: not recorded
- [ ] Scenario: Immediately passing tests protect existing behavior without a production edit (regular)
  COMMIT: not recorded
- [ ] Scenario: Skipped RED is admitted rather than reconstructed from history (regular)
  COMMIT: not recorded
- [ ] Scenario: A coherent behavior batch is completed with its actual explicit-path revision (regular)
  COMMIT: not recorded
- [ ] Scenario: Refactor guidance uses the outline once for supported changed components and their neighbors (regular)
  COMMIT: not recorded
- [ ] Scenario: Unsupported outline languages receive source assessment without a waiver (regular)
  COMMIT: not recorded
- [ ] Scenario: Encountered cleanup is a separate GREEN refactoring batch (regular)
  COMMIT: not recorded
- [ ] Scenario: Served development guidance requires modular boundaries from the start (regular)
  COMMIT: not recorded

## Batch: start-the-declared-product

Design constraints: use the start-the-declared-product owner and scenarios in specs/operate-product/start-the-declared-product.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: The started server reports readiness and forwards a clean exit (@e2e)
  COMMIT: not recorded
- [ ] Scenario: A readiness timeout abandons the start (regular)
  COMMIT: not recorded
- [ ] Scenario: The readiness probe gives up after the timeout (regular)
  COMMIT: not recorded
- [ ] Scenario: A stalled readiness exchange cannot outlive the total deadline (regular)
  COMMIT: not recorded
- [ ] Scenario: Readiness retries share one deadline across connection and exchange (regular)
  COMMIT: not recorded

## Batch: describe-the-release

Design constraints: use the describe-the-release owner and scenarios in specs/publish-a-release/describe-the-release.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: The migration catalog defines complete release-owned bundles (regular)
  COMMIT: not recorded
- [ ] Scenario: Synthetic future migration bundles retain complete metadata validation (regular)
  COMMIT: not recorded

## Batch: apply-a-managed-upgrade

Design constraints: use the apply-a-managed-upgrade owner and scenarios in specs/upgrade-a-project/apply-a-managed-upgrade.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: A clean project without migration requirements receives one managed upgrade commit (@e2e)
  COMMIT: not recorded
- [ ] Scenario: A clean Fossil project receives the same managed upgrade revision shape (regular)
  COMMIT: not recorded
- [ ] Scenario: An unclean project is protected from automatic migration commits (regular)
  COMMIT: not recorded
- [ ] Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits (regular)
  COMMIT: not recorded
- [ ] Scenario: A project outside Git is protected from a partial automatic migration (regular)
  COMMIT: not recorded
- [ ] Scenario: A local-only upgrade publishes rule removals without AGENTS.md (regular)
  COMMIT: not recorded
- [ ] Scenario: A clean 0.7.0 project receives a reviewable aggregate migration plan (retirement)
  COMMIT: not recorded
- [ ] Scenario: A project below the oldest migration route receives no migration change (retirement)
  COMMIT: not recorded
- [ ] Scenario: An existing migration plan is never overwritten (regular)
  COMMIT: not recorded
- [ ] Scenario: Pre-1.0 projects require manual transition instead of embedded migration plans (regular)
  COMMIT: not recorded
- [ ] Scenario: Unsupported old configuration is refused before upgrade writes (regular)
  COMMIT: not recorded
- [ ] Scenario: Upgrade failures report actual partial state and operator recovery (regular)
  COMMIT: not recorded
- [ ] Scenario: Future supported migration routes remain reviewable (regular)
  COMMIT: not recorded
- [ ] Scenario: Repeating the upgrade does not duplicate the migration (regular)
  COMMIT: not recorded
- [ ] Scenario: The migration change remains honest until the project review passes (regular)
  COMMIT: not recorded
- [ ] Scenario: A completed maintenance migration can be archived without a product specification (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Upgrade reconciles only recognizable generated commands to skills (regular)
  COMMIT: not recorded

## Batch: check-specification-health

Design constraints: use the check-specification-health owner and scenarios in specs/verify-change/check-specification-health.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: All clean .feature files report a clean summary on stdout (@e2e)
  COMMIT: not recorded
- [ ] Scenario: Explicit relationship namespaces disambiguate equal identifiers (regular)
  COMMIT: not recorded
- [ ] Scenario: A relationship cannot resolve through the other namespace (regular)
  COMMIT: not recorded
- [ ] Scenario: A fragment cannot be an extension target even when a use case has the same identifier (regular)
  COMMIT: not recorded
- [ ] Scenario: Cycles follow use-case includes and extends relationships (regular)
  COMMIT: not recorded
- [ ] Scenario: A fragment inclusion does not invent an edge to a same-named use case (regular)
  COMMIT: not recorded
- [ ] Scenario: Interaction evidence cannot be credited from another capability (regular)
  COMMIT: not recorded
- [ ] Scenario: Fragment interactions are checked in their declaring fragment (regular)
  COMMIT: not recorded
- [ ] Scenario: A malformed Unicode use-case title produces a diagnostic instead of a panic (regular)
  COMMIT: not recorded

## Batch: givn-workflow-enforcement

Design constraints: use the givn-workflow-enforcement owner and scenarios in specs/verify-change/givn-workflow-enforcement.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Consumer runs check tasks and receives a change verification receipt (@e2e)
  COMMIT: not recorded
- [ ] Scenario: A successful runner without new output cannot reuse a stale result (regular)
  COMMIT: not recorded
- [ ] Scenario: Invalid fresh runner output cannot prove changed scenarios (regular)
  COMMIT: not recorded
- [ ] Scenario: Inputs changed during execution cannot receive a passed receipt (regular)
  COMMIT: not recorded
- [ ] Scenario: An explicit no-behavior-change declaration needs no delta scenarios (regular)
  COMMIT: not recorded
- [ ] Scenario: Omitting delta specs without declaring no behavior change is not an exemption (regular)
  COMMIT: not recorded
- [ ] Scenario: Integrity remains applicable without delta specs (regular)
  COMMIT: not recorded
- [ ] Scenario: Review accepts an explicit no-behavior-change verification result (regular)
  COMMIT: not recorded
- [ ] Scenario: Review does not run the full suite (regular)
  COMMIT: not recorded
- [ ] Scenario: Implementation verification does not require the review it will inform (regular)
  COMMIT: not recorded
- [ ] Scenario: Two scenarios sharing a batch commit pass the task check (regular)
  COMMIT: not recorded
- [ ] Scenario: A shared batch revision without a batch label fails the task check (retirement)
  COMMIT: not recorded
- [ ] Scenario: A commit message that omits the scenario title fails the task check (retirement)
  COMMIT: not recorded
- [ ] Scenario: Batch traceability rejects an unreachable revision (regular)
  COMMIT: not recorded
- [ ] Scenario: Final review does not reconstruct TDD order from commit shape (regular)
  COMMIT: not recorded
- [ ] Scenario: Archive re-runs the static review gates (regular)
  COMMIT: not recorded
- [ ] Scenario: Preservation detects changes to supported executable content (regular)
  COMMIT: not recorded
- [ ] Scenario: Moving unchanged supported executable content preserves behavior (regular)
  COMMIT: not recorded
- [ ] Scenario: Review ignores verdict examples rather than treating them as decisions (regular)
  COMMIT: not recorded
- [ ] Scenario: Non-executable FAIL examples do not override an actual PASS (regular)
  COMMIT: not recorded
- [ ] Scenario: Conflicting actual verdicts cannot pass review (regular)
  COMMIT: not recorded
- [ ] Scenario: Negated PASS prose is not a passing verdict (regular)
  COMMIT: not recorded
- [ ] Scenario: Independent cheap failures are reported while dependent checks are deferred (regular)
  COMMIT: not recorded
- [ ] Scenario: Consumer invokes bare check and receives explicit non-executing guidance (@e2e)
  COMMIT: not recorded

## Batch: outline-review-gate

Design constraints: use the outline-review-gate owner and scenarios in specs/verify-change/outline-review-gate.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Retire the blocking outline capability and its implementation/callers/tests without deleting advisory reporting.
  COMMIT: not recorded

## Batch: review-disposition-gates

Design constraints: use the review-disposition-gates owner and scenarios in specs/verify-change/review-disposition-gates.feature; preserve exact observable assertions, relevant use-case guarantees and confirmed Personas. Apply the focused design contract, actual subprocess/binding locations, no extra suites/heuristic gates and explicit-path publication.

RED: not run.
GREEN: not run.
REFACTOR: not run.

- [ ] Scenario: Review passes when every shape match is disposed and reports the net-delta receipt (retirement)
  COMMIT: not recorded
- [ ] Scenario: Review blocks when a shape match involving the change's delta is undisposed (retirement)
  COMMIT: not recorded
- [ ] Scenario: Review blocks when a delta scenario shape-matches a permanent scenario without disposition (retirement)
  COMMIT: not recorded
- [ ] Scenario: Consumer passes review without shape dispositions and receives the net-delta receipt (regular)
  COMMIT: not recorded
- [ ] Scenario: Shape matches do not block review or archive (regular)
  COMMIT: not recorded
- [ ] Scenario: Review blocks with a clear message when review.md is missing (regular)
  COMMIT: not recorded
- [ ] Scenario: Review passes when a long scenario has a split-or-keep entry (regular)
  COMMIT: not recorded
- [ ] Scenario: A keep disposition must contain an explicit decision and a reason (regular)
  COMMIT: not recorded
- [ ] Scenario: A promised split cannot pass review while the long scenario remains unsplit (regular)
  COMMIT: not recorded
- [ ] Scenario: A completed split satisfies the scenario-length disposition (regular)
  COMMIT: not recorded

## Documentation and deferred work closeout

- [ ] Reconcile current docs/glossary/Personas/ADRs, remove duplicate workflow-details authority and obsolete0.x operational bundles; preserve historical evidence.
- [ ] Track concrete post1.0 matching, outline, recording and unrelated findings in existing risks/debt chapter without reopening acceptance.
- [ ] Complete arc42 assessment honestly and capture terminal evidence for applicable E2Es.
- [ ] Execute agreed project validation and one substantive final review by preparation-reviewer, then targeted closure only.

No box above is complete merely because the planning phases are committed.
