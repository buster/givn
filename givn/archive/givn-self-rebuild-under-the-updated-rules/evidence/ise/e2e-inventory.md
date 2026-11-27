# End-to-end inventory

One row per selected `@e2e` entry in `spec-coverage.json` at the authoring revision: `givn spec coverage --format json`. The frozen runner expands five Scenario Outline examples, so the run total is 120 while this table holds 115 definitions over 56 feature paths.

| Capability | Feature path | Scenario title |
| --- | --- | --- |
| `report-change-status` | `inspect-change-state/report-change-status.feature` | Detail mode — active change with all required done (text) |
| `archive-proof` | `archive-change/archive-proof.feature` | Consumer archives a completed change and receives a proven archive |
| `measure-coverage-per-project` | `fragments/measure-coverage-per-project.feature` | An initialized project documents reusable test and coverage scripts |
| `change-release-note` | `create-change/change-release-note.feature` | The proposal guidance requires a release note |
| `givn-workflow-enforcement` | `verify-change/givn-workflow-enforcement.feature` | The result helper writes four counts without a scope |
| `specification-declaration` | `declare-the-specification/specification-declaration.feature` | The declared roles are read by the project-health pass |
| `enable-commit-attribution` | `fragments/enable-commit-attribution.feature` | Consumer initializes a Fossil project and receives the givn commit path without a hook |
| `change-release-note` | `create-change/change-release-note.feature` | A new change's proposal carries the release-note field |
| `describe-the-release` | `publish-a-release/describe-the-release.feature` | Repeated revisions of one change yield one changelog entry |
| `givn-workflow-enforcement` | `verify-change/givn-workflow-enforcement.feature` | Consumer runs check tasks and receives a change verification receipt |
| `attribute-commits` | `fragments/attribute-commits.feature` | Consumer initializes a project and receives shared attribution plumbing |
| `enable-optional-addons` | `initialize-project/enable-optional-addons.feature` | Consumer enables the outline addon and it is listed as enabled |
| `givn-workflow-enforcement` | `verify-change/givn-workflow-enforcement.feature` | Consumer invokes bare check and receives explicit non-executing guidance |
| `givn-workflow-enforcement` | `verify-change/givn-workflow-enforcement.feature` | Consumer runs migration coverage check and sees passed evidence |
| `visual-review-gate` | `verify-change/visual-review-gate.feature` | Review guidance requires the visual verdict and persona lenses |
| `resolve-change-directories` | `verify-change/resolve-change-directories.feature` | Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected |
| `declare-the-product-start` | `operate-product/declare-the-product-start.feature` | Archive passes once the run declaration is complete |
| `start-the-declared-product` | `operate-product/start-the-declared-product.feature` | An interrupt stops the started product |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer expands a workflow phase and sees its lanes |
| `preserve-scenario-docstrings` | `archive-change/preserve-scenario-docstrings.feature` | Consumer archives a modified scenario and the permanent spec keeps the docstring |
| `declare-the-product-start` | `operate-product/declare-the-product-start.feature` | The review check reports an unconfigured run declaration |
| `instruction-extension` | `extend-an-instruction/instruction-extension.feature` | An upgrade preserves the extension and the next request serves it |
| `readme-enforcement` | `keep-documentation-current/readme-enforcement.feature` | Instructions surface an unfinished README |
| `instruction-extension` | `extend-an-instruction/instruction-extension.feature` | A developer records an extension and it governs the next guidance request |
| `guide-domain-modeling` | `fragments/guide-domain-modeling.feature` | An agent requests the specs guidance and is routed to the domain skill |
| `archive-proof` | `archive-change/archive-proof.feature` | Consumer inspects an unproven archive candidate |
| `attribute-commits` | `fragments/attribute-commits.feature` | Maki attributes a commit using its current model |
| `read-task-progress` | `inspect-change-state/read-task-progress.feature` | Consumer runs givn status on a change with uppercase checkboxes |
| `start-the-declared-product` | `operate-product/start-the-declared-product.feature` | A caller-provided port is handed to the product |
| `change-release-note` | `create-change/change-release-note.feature` | The generated implementation guidance shares the release note |
| `review-architecture-documentation` | `fragments/review-architecture-documentation.feature` | Generated review guidance stays aligned after skill sync |
| `development-record` | `inspect-development-history/development-record.feature` | Consumer reads the development record as machine-readable output |
| `enforce-post-merge-e2e-gate` | `archive-change/enforce-post-merge-e2e-gate.feature` | Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner |
| `outline-report` | `assess-code-structure/outline-report.feature` | A developer runs a outline report and sees the classified units |
| `match-guidance-to-enabled-addons` | `fragments/match-guidance-to-enabled-addons.feature` | A consumer synchronizes skills and the generated guidance omits disabled-addon sections |
| `keep-audit-output-clean` | `discover-specifications/keep-audit-output-clean.feature` | The JSON duplicate audit is valid JSON without progress wording |
| `verify-repository-ownership` | `fragments/verify-repository-ownership.feature` | Repository-wide review accepts explicit overlap dispositions |
| `readme-enforcement` | `keep-documentation-current/readme-enforcement.feature` | The check passes on a complete README |
| `guide-domain-modeling` | `fragments/guide-domain-modeling.feature` | An agent synchronizes skills and the domain skill is routed |
| `attribute-commits` | `fragments/attribute-commits.feature` | An attributed commit exposes the current LLM and harness |
| `review-architecture-documentation` | `fragments/review-architecture-documentation.feature` | Arc42-enabled review instructions require conformance evidence |
| `declare-the-product-start` | `operate-product/declare-the-product-start.feature` | Upgrade backfills the run declaration into an existing project |
| `verify-repository-ownership` | `fragments/verify-repository-ownership.feature` | Archive publishes consolidated permanent specifications |
| `guide-domain-modeling` | `fragments/guide-domain-modeling.feature` | An agent requests the design guidance and is routed to the domain skill |
| `canonical-usecase-contract` | `make-usecases-canonical/canonical-usecase-contract.feature` | Use-case tree reports explicit use-case ownership |
| `route-a-change` | `fragments/route-a-change.feature` | Routing a proposal prints ranked candidates with evidence |
| `declare-the-product-start` | `operate-product/declare-the-product-start.feature` | The sentinel start command prints setup guidance |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer zooms the timeline to a shorter time range |
| `initialize-a-project` | `initialize-project/initialize-a-project.feature` | Consumer re-syncs skills after init and the selection is honoured |
| `start-the-declared-product` | `operate-product/start-the-declared-product.feature` | Print mode emits the start plan without starting |
| `enable-optional-addons` | `initialize-project/enable-optional-addons.feature` | A consumer disables a required addon and the refusal names ise |
| `detect-the-project-repository` | `fragments/detect-the-project-repository.feature` | A Git worktree at the project root selects the Git backend |
| `declare-the-product-start` | `operate-product/declare-the-product-start.feature` | The check names a damaged run managed block |
| `validate-artifact-ids` | `verify-change/validate-artifact-ids.feature` | Consumer invokes givn check with an unknown artifact and gets a helpful error |
| `refresh-generated-guidance` | `fragments/refresh-generated-guidance.feature` | Synchronized guidance explains how to resolve the deterministic gates |
| `initialize-a-project` | `initialize-project/initialize-a-project.feature` | Consumer runs init non-interactively and gets opencode skills |
| `validate-change-ids` | `inspect-change-state/validate-change-ids.feature` | Consumer invokes givn status with a path traversal change id and gets a clear error |
| `readme-enforcement` | `keep-documentation-current/readme-enforcement.feature` | Scaffolding marks the unfilled sections |
| `apply-a-managed-upgrade` | `upgrade-a-project/apply-a-managed-upgrade.feature` | A clean project without migration requirements receives one managed upgrade commit |
| `measure-coverage-per-project` | `fragments/measure-coverage-per-project.feature` | Archiving publishes line and branch badges from the merged report |
| `instruction-integrity` | `fragments/instruction-integrity.feature` | Author starts a session with the universal dialog obligation |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer generates the development report for a change |
| `spec-duplicates` | `discover-specifications/spec-duplicates.feature` | Exact duplicates in one feature are reported together |
| `attribute-commits` | `fragments/attribute-commits.feature` | OpenCode prepares attribution using its selected model |
| `archive-proof` | `archive-change/archive-proof.feature` | Consumer repairs a blocked archive and the rerun proves it |
| `apply-a-managed-upgrade` | `upgrade-a-project/apply-a-managed-upgrade.feature` | A consumer invoking the retired history rewrite command is told it does not exist |
| `agent-activity-reporting` | `inspect-development-history/agent-activity-reporting.feature` | Consumer installs the agent reporter and its reported activity lands in the record |
| `archive-proof` | `archive-change/archive-proof.feature` | Consumer archives a change after documentation-only review edits and the proof is reused |
| `match-guidance-to-enabled-addons` | `fragments/match-guidance-to-enabled-addons.feature` | A consumer enables the addons a generated section needs and the guidance gains it |
| `spec-gap` | `create-change/spec-gap.feature` | The review guidance requires one spec gap outcome before any fix |
| `enable-optional-addons` | `initialize-project/enable-optional-addons.feature` | Consumer lists addons and sees each addon's materialized files reported |
| `trial-input-verification` | `prepare-an-ise-test/trial-input-verification.feature` | Verifier compares the spec copy with the manifest |
| `attribute-commits` | `fragments/attribute-commits.feature` | Claude attributes a commit using its current model |
| `visual-review-gate` | `verify-change/visual-review-gate.feature` | A visual change with complete evidence passes review |
| `givn-workflow-enforcement` | `verify-change/givn-workflow-enforcement.feature` | Consumer runs review and receives every use-case schema problem in one pass |
| `start-the-declared-product` | `operate-product/start-the-declared-product.feature` | The started server reports readiness and forwards a clean exit |
| `apply-a-managed-upgrade` | `upgrade-a-project/apply-a-managed-upgrade.feature` | A completed maintenance migration can be archived without a product specification |
| `show-skipped-artifacts` | `inspect-change-state/show-skipped-artifacts.feature` | Consumer runs givn status on a change with skipped artifacts and sees them correctly |
| `show-coverage-in-the-readme` | `fragments/show-coverage-in-the-readme.feature` | Coverage-enabled project is guided to place the coverage badge directly after the README title |
| `adr-qualification-gate` | `fragments/adr-qualification-gate.feature` | An author requests the strict ADR guidance through the Givn CLI |
| `find-existing-scenarios` | `fragments/find-existing-scenarios.feature` | Lexical search locates a permanent scenario by a step text token |
| `enable-commit-attribution` | `fragments/enable-commit-attribution.feature` | Consumer upgrades an existing project with attribution plumbing missing |
| `attribute-commits` | `fragments/attribute-commits.feature` | omp attributes a commit using its current model |
| `grouped-merge-for-new-capabilities` | `archive-change/grouped-merge-for-new-capabilities.feature` | Consumer archives a new capability and the merged spec lands in its declared use case |
| `prose-consolidation` | `create-change/prose-consolidation.feature` | Generated guidance points to the canonical E2E policy |
| `instruction-integrity` | `fragments/instruction-integrity.feature` | An agent requests machine-readable artifact guidance |
| `visual-design-contract` | `create-change/visual-design-contract.feature` | Synchronized guidance defines the contract approval and capture mechanics |
| `extract-coverage-gate` | `archive-change/extract-coverage-gate.feature` | Consumer archives a change with coverage enabled and the coverage summary appears in README |
| `review-architecture-documentation` | `fragments/review-architecture-documentation.feature` | The review template includes the Arc42 conformance block |
| `coverage-readme-example` | `fragments/coverage-readme-example.feature` | Coverage-enabled project receives a complete README coverage example |
| `spec-overlap-gates` | `verify-change/spec-overlap-gates.feature` | The json output reports duplicate-title findings machine-readably |
| `apply-a-managed-upgrade` | `upgrade-a-project/apply-a-managed-upgrade.feature` | A consumer invoking a retired engine command is told it does not exist |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer opens the report of a scope with no recorded activity |
| `guide-domain-modeling` | `fragments/guide-domain-modeling.feature` | An agent requests next-step guidance and is routed to the domain skill |
| `development-record` | `inspect-development-history/development-record.feature` | Consumer runs a lifecycle step and reads its recorded actions |
| `visual-design-contract` | `create-change/visual-design-contract.feature` | Author requests the visual design contract guidance |
| `resolve-change-directories` | `verify-change/resolve-change-directories.feature` | Integrity check detects a placeholder COMMIT hash in tasks.md and fails |
| `prose-consolidation` | `create-change/prose-consolidation.feature` | Canonical E2E policy is emitted by specs instructions |
| `change-strategy` | `create-change/change-strategy.feature` | The design guidance requires one recorded change strategy |
| `find-existing-scenarios` | `fragments/find-existing-scenarios.feature` | The spec tree reports per-use-case counts and the use-case narrative |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer opens the generated development report in a browser |
| `initialize-a-project` | `initialize-project/initialize-a-project.feature` | Consumer selects agent targets during interactive init |
| `declare-the-product-start` | `operate-product/declare-the-product-start.feature` | Scaffolding ships an unconfigured run declaration |
| `check-specification-health` | `verify-change/check-specification-health.feature` | All clean .feature files report a clean summary on stdout |
| `enable-commit-attribution` | `fragments/enable-commit-attribution.feature` | Consumer sees the commit attribution installation report |
| `clean-room-preparation` | `prepare-an-ise-test/clean-room-preparation.feature` | Developer rebuilds the whole product from its Specification |
| `readme-enforcement` | `keep-documentation-current/readme-enforcement.feature` | A recorded impact decision passes the review |
| `enable-optional-addons` | `initialize-project/enable-optional-addons.feature` | A consumer lists addons and each declared default is reported |
| `require-merged-step-definitions` | `archive-change/require-merged-step-definitions.feature` | Consumer archives a change whose delta step definitions are already in the permanent test tree |
| `spec-overlap-gates` | `verify-change/spec-overlap-gates.feature` | A duplicate title across two permanent spec files is a hard error |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer reads a change's linked ideation group |
| `serve-extended-rules` | `obtain-effective-guidance/serve-extended-rules.feature` | An agent receives the extended rules through the Givn CLI |
| `development-report` | `inspect-development-history/development-report.feature` | Consumer selects a timeline element and reads its details |
| `require-durable-retained-paths` | `archive-change/require-durable-retained-paths.feature` | Consumer archives a change whose declared retained paths survive the move |
| `readme-enforcement` | `keep-documentation-current/readme-enforcement.feature` | Archiving refreshes the development block |
