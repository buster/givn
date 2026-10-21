# Review: review-friction-hardening

README-IMPACT: updated - Description
VISUAL-REVIEW: PASS
PERSONAS: developer--primary-user

## Fabrication audit

- **@e2e tag integrity:** both delta `@e2e` scenarios retain the tag
  (`Consumer runs review...`, `Consumer archives a change...`). No tag was
  removed.
- **Empty step bodies:** 0 empty bodies in the step files this change touched
  (`workflow_enforcement_steps.rs`, `workflow_enforcement_e2e_steps.rs`,
  `archive_proof_steps.rs`, `archive_proof_e2e_steps.rs`). The only empty-body
  grep hit is a pre-existing helper in `llm_attribution_e2e_steps.rs`, outside
  this change.
- **Scenario commits:** every checked scenario records one unique revision.
  The shared parser and proof production code landed in the parser/proof
  commits; each later scenario commit adds its executable regression scenario
  and step definitions. No checked task is spec-only.
- **Promised components:** `UsecaseDiagnostic(s)`, the routing document cache,
  `Backend::content_paths()`, schema 2 receipt fields, `ProofStatus`,
  `StaleReason`, and the shared freshness call sites all exist in the tree.
- **Strict mode:** the proof is in `tasks.md`; an undefined step exits 1.
- **E2E primary assertions:** both delta `@e2e` scenarios assert CLI exit code,
  stdout/stderr, the reused-proof line, and the merged-scope invocation. They
  are not repository-only assertions.
- **E2E command scope:** `verify.command` is `./run-tests.sh --non-e2e` and
  `verify.e2e_command` is `./run-tests.sh --e2e`; they are not equal. The delta
  scope has 12 scenarios and 2 `@e2e` scenarios, so the e2e scope is strictly
  smaller.
- **Design conformance:** the implementation uses the exact files, commands,
  and step locations named in `design.md`: `src/gherkin/usecase_md.rs`,
  `src/spec/routing_table.rs`, `src/proof.rs`, `src/scm.rs`, `src/git.rs`,
  `src/fossil.rs`, `src/cli/check.rs`, `src/cli/archive.rs`, and the four step
  files. No silent deviation.
- **Interaction coverage:** the full inventory cross-reference is below. The
  design matrix maps the two new interactions; every unchanged entry keeps its
  existing permanent `@e2e` scenario and no new entry is unmapped.

### Interaction coverage

| Use case | Capability | Consumer action | E2E scenario | Delta? |
|---|---|---|---|---|
| verify-change | validate-artifact-ids | check an unknown artifact | Consumer invokes givn check with an unknown artifact and gets a helpful error | permanent |
| verify-change | visual-review-gate | run the review check on a visual change | A visual change with complete evidence passes review | permanent |
| verify-change | visual-review-gate | read the visual review guidance | Review guidance requires the visual verdict and persona lenses | permanent |
| verify-change | spec-overlap-gates | detect a duplicate title | A duplicate title across two permanent spec files is a hard error | permanent |
| verify-change | spec-overlap-gates | inspect duplicate JSON | The json output reports duplicate-title findings machine-readably | permanent |
| verify-change | review-disposition-gates | pass a disposed review | Review passes when every shape match is disposed and reports the net-delta receipt | permanent |
| verify-change | resolve-change-directories | run integrity in a change | Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected | permanent |
| verify-change | resolve-change-directories | reject placeholder commit evidence | Integrity check detects a placeholder COMMIT hash in tasks.md and fails | permanent |
| verify-change | givn-workflow-enforcement | run migration coverage check | Consumer runs migration coverage check and sees passed evidence | permanent |
| verify-change | givn-workflow-enforcement | run the change-scoped checks | Consumer runs check tasks and receives a change verification receipt | permanent |
| verify-change | givn-workflow-enforcement | write a runner result | The result helper writes four counts without a scope | permanent |
| verify-change | givn-workflow-enforcement | run the review check on a change with use-case schema errors | Consumer runs review and receives every use-case schema problem in one pass | new |
| archive-change | archive-proof | archive a completed change | Consumer archives a completed change and receives a proven archive | permanent |
| archive-change | archive-proof | repair a blocked archive | Consumer repairs a blocked archive and the rerun proves it | permanent |
| archive-change | archive-proof | inspect an unproven archive candidate | Consumer inspects an unproven archive candidate | permanent |
| archive-change | archive-proof | archive a change after documentation-only review edits | Consumer archives a change after documentation-only review edits and the proof is reused | new |
| archive-change | grouped-merge-for-new-capabilities | archive a new capability | Consumer archives a new capability and the merged spec lands in its declared use case | permanent |
| archive-change | grouped-merge-for-new-capabilities | archive without routing | Archive is blocked when a new capability has no declared routing decision | permanent |
| archive-change | grouped-merge-for-new-capabilities | archive a multi-file change | Nothing is merged when one delta file of a change has no routing decision | permanent |
| archive-change | preserve-scenario-docstrings | archive a modified scenario | Consumer archives a modified scenario and the permanent spec keeps the docstring | permanent |
| archive-change | extract-coverage-gate | archive with coverage | Consumer archives a change with coverage enabled and the coverage summary appears in README | permanent |
| archive-change | enforce-post-merge-e2e-gate | archive an E2E delta | Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner | permanent |


## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| TC-39 receipt binding | `02-architecture-constraints.md` | Affected | `ChangeReceipt` schema 2 | Scenario 6 | `src/proof.rs` `tested_content_digest` / `content_scope` | yes |
| Change verification strategy | `04-solution-strategy.md` | Affected | Shared freshness and batched diagnostics | Scenarios 1-8 | `src/proof.rs::freshness`, `src/spec/routing_table.rs` | yes |
| Building-block responsibilities | `05-building-block-view.md` | Affected | Parser, routing cache, proof, SCM, CLI | Scenarios 1-10 | `src/gherkin/usecase_md.rs`, `src/spec/routing_table.rs`, `src/proof.rs`, `src/scm.rs` | yes |
| Archive/review receipt flow | `06-runtime-view.md` §6.7.1 | Affected | `Current` / `DocumentationOnly` / `Stale` | Scenarios 6-10 | `src/cli/check.rs`, `src/cli/archive.rs` | yes |
| Review check order | `08-crosscutting-concepts.md` §8.12 | Affected | Integrity, run, overlap, retirement, routing, receipt | Scenarios 1-8 | `src/cli/check.rs` `run_mandatory_checks` | yes |
| ADR-070 amendment | `09-architecture-decisions.md`, `adr/README.md` | Affected | Receipt tested-content binding | Scenario 6 | `docs/arc42/adr/ADR-070-forward-only-archive-proof.md` | yes |
| Quality scenarios Q-105/Q-118 | `10-quality-requirements.md` | Affected | Reuse and complete diagnosis | Scenarios 1-10 | Gherkin scenarios and transcripts | yes |
| Risks R-116/R-117/R-118 | `11-risks-and-technical-debt.md` | Affected | Allowlist, hashing, Fossil existence | Scenarios 6-8 | `src/proof.rs`, `src/fossil.rs` | yes |
| Glossary terms | `12-glossary.md` | Affected | Receipt, documentation-only revision, tested content | All | Same terms in specs and code | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary now defines **change verification receipt**,
**documentation-only revision**, and **tested content**, and distinguishes
`Current` (same revision) from `DocumentationOnly` (same tested content under a
different available revision). The specs, design, and code use those terms
consistently; the anti-terms `review proof` and `test cache` do not appear in
the implementation.

## Coverage

Coverage is measured once by the archive against the merged candidate; review
uses static inspection and the change-scoped receipt. No dead code was added:
`is_current` remains the strict compatibility helper used by tests, and the
new diagnostic types are consumed by the parser and routing gate. The parser
recovery branches, digest stability, schema 1 equality, unknown schema/scope,
documentation-only reuse, and Fossil content paths have unit coverage. The
black-box scenarios cover the two user-visible operations.

## E2E coverage

The terminal interface is the real CLI. `verify.e2e_command` is
`./run-tests.sh --e2e`, which filters the delta `@e2e` scenarios. The delta
scope has 12 scenarios and 2 `@e2e` scenarios. The docs-only archive
interaction is a distinct state transition: its observable contract is proof
reuse plus the absence of a change-suite invocation, which the ordinary
archive outline does not assert. The review scenario asserts the complete
stderr diagnosis from a real `givn check review` subprocess.

## Deferrals

None. The pre-existing full-lint capability inventory mismatch is recorded in
the proposal as out of scope; it is not a deferred mandatory check of this
change.

## Use-case and Persona conformance

The delta narratives keep the owning use cases' goals, guarantees, rules,
capabilities, and typed relationships. Base hashes match the permanent
documents. The implementation preserves `verify-change` (structural failures
outrank other findings; complete diagnosis; strict schema) and `archive-change`
(receipt before mutation; no change-suite rerun; merged suite once; only a
proven archive is complete). The confirmed Personas are
`developer--primary-user`, `agent--technical-role`, and
`givn-maintainer--owner`; no Persona was invented or promoted.

## Visual review

`Interface: terminal`. Both delta `@e2e` scenarios have committed transcripts
under `evidence/visual/<scenario-slug>/transcript.txt`, each recording the
scenario title, argv, working-directory disposition, exit code, and raw
stdout/stderr. The review transcript shows the complete diagnostic set; the
archive transcript shows the reused-proof line, the single merged regular
`givn/specs` invocation, exit 0, and the proven archive. The terminal gate is
unchanged: no `VISUAL-REVIEW: NOT APPLICABLE` path is added.

VISUAL-REVIEW: PASS

## Overlap dispositions

No shape match involving this change's delta scenarios was reported. The gate
reported a net delta of 10 added, 0 modified, 0 removed; no disposition row is
required.

## Split-or-keep

No delta scenario exceeds the long-scenario threshold.

## Sign-off

- Fabrication audit clean.
- Every checked task has a revision containing its implementation or its
  executable regression scenario.
- Strict-mode proof present.
- No `@wip` remains; `givn lint --change review-friction-hardening` is clean.
- Every coverage gap is classified. No dead code, no missing scenario, no
  hard-to-test exception was needed.
- Implementation matches the reviewed design.
- Arc42 conformance clean; ubiquitous language clean.
- No deferral without an operator decision.

REVIEW: PASS
