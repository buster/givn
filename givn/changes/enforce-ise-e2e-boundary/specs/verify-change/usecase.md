<!-- givn:base-sha256:ee6f6fb8ea5a9e3075d858fb7323e86db4f30bc0b72061ad7212cbaba260989e -->
# Use case: verify-change

## Level

!

## Actors

- Developer
- Supporting: Reviewer

## Goal

Check a change for structural validity, applicable verification evidence, and
completed review without treating advisory structural matches as proof or
blockers.

The delta: the specification-health check reports four advisory findings from
the project's declared `Specification`, and a project with `ise` off never
reads the declaration. With the `ise` addon effective, the check also holds the
change to the project's declared Specification boundary, refuses a declared
E2E acceptance set that is missing, links production source, or is not named by
the E2E runner command, runs the change-scoped E2E execution, and binds the
E2E identity of the accepted results into the evidence it issues.

## Trigger

The developer requests verification of a change, or the project-health check.

## Preconditions

- The change artifacts and active specifications are available.
- When the `ise` addon is effective, the project declares its retained roles and replaceable source roots (`declare-the-specification`).

## Main flow

1. The developer selects an explicit check; the system reads its applicable
   specifications and workflow artifacts and, when the `ise` addon is
   effective, the project's declared Specification.
2. The system collects independent cheap structural, identity, integrity,
   relationship, and capability-routing failures; checks depending on failed
   prerequisites are reported as not performed. When the `ise` addon is
   effective, this pass also holds the declared E2E package to its boundary:
   the package exists, links no production source, and is named by the E2E
   runner command.
3. For changed-scenario verification, the system clears prior runner output and
   captures the revision, selected delta features, runner command, the declared
   E2E identity when the `ise` addon is effective, and relevant inputs before
   execution.
4. The system requires fresh valid passing results and stable inputs after
   execution before issuing a bound receipt. An explicitly declared
   no-behavior-change change without deltas records changed-scenario
   verification as not applicable.
5. Completed-review checks inspect the review and applicable existing
   verification evidence, report blocking findings and the net delta, and
   execute no verification suite.
6. A bare check instead prints non-executing guidance to explicit checks,
   labeling which commands execute runners; it executes neither configured
   hooks nor runners.

## Extensions

- 1: the `ise` addon is effective and the declared Specification is absent or unreadable → the checks that read it are reported as not performed and the missing declaration is named; protects the minimal guarantee.
- 1: the developer selects the specification-health check and the `ise` addon is effective → the four declaration findings are reported as advisory and the lint exit code is unchanged; protects the minimal guarantee.
- 2: a structural or integrity prerequisite fails → independent cheap errors are reported together and dependent checks are marked not performed; protects the minimal guarantee.
- 2: the declared `e2e` role resolves no package → the check refuses, names the role and the unmatched entries, and issues no receipt; protects the minimal guarantee.
- 2: a resolved `e2e` package has a malformed manifest, or the declaration cannot be read → the check refuses because the boundary cannot be proven, and issues no receipt; protects the minimal guarantee.
- 2: a resolved `e2e` path sits under a `source` root, or a resolved package's tree contains a `source` root → the check refuses, names the path or package and the root, and issues no receipt; protects the minimal guarantee.
- 2: a resolved `e2e` package has a direct, transitive, or workspace-inherited dependency edge into `source` → the check refuses, names the package and the edge, and issues no receipt; protects the minimal guarantee.
- 2: a resolved `e2e` path resolves no package and a reference under it resolves into a `source` root → the check refuses, names the path and the imported package, and issues no receipt; protects the minimal guarantee.
- 2: the E2E runner command names no resolved `e2e` root → the check refuses and names the root and the command; protects the minimal guarantee.
- 3: tested inputs, including the E2E identity, change during execution → no passed receipt is issued for those changed inputs; protects the success guarantee.
- 4: fresh output is missing, malformed, has the wrong scope, has unreconciled or zero counts, or reports failures → verification fails without a passed receipt; protects the success guarantee.
- 4: the delta has no `@e2e` scenario → the receipt records the E2E scope as not applicable and no E2E runner executes; protects the success guarantee.
- 4: the E2E run leaves a step unbound → the runner reports it as a failed test, the run fails, and no passed receipt is issued; protects the success guarantee.
- 4: the E2E run reports a skipped step → the run fails and no passed receipt is issued; protects the success guarantee.
- 4: no delta exists without an explicit no-behavior-change declaration → verification refuses without substituting a permanent full-suite run; protects the success guarantee.
- 5: a required long-scenario disposition, actual review verdict, or README-impact decision is missing or invalid → completion is blocked; protects the success guarantee.
- 5: applicable verification evidence is missing or stale → completion is blocked; protects the success guarantee.

## Rules

- Structural failures outrank advisory findings; use-case schema and capability-routing failures are reported in one pass with the root cause first.
- Typed relationships resolve only within their declared use-case or fragment namespace. Extends cannot target a fragment; use-case Includes and Extends edges participate in cycle detection, while fragment inclusion does not invent a use-case edge.
- Interaction coverage matches capability plus scenario identity, including declarations owned by fragments; a same-titled scenario in another capability is not evidence.
- Shape matches are explicit advisory discovery only, with no mandatory disposition table and no review or archive veto. Outline reports likewise require no verdict, metric waiver, or fresh reviewer context.
- A scenario exceeding the length limit requires an explicit split or keep decision. Keep needs a brief reason; a chosen split must be completed before sign-off.
- Each checked scenario records its actual reachable batch revision containing its implementation or regression test. Multiple scenarios may share that revision; task-only, tag-only, and comment-only revisions are not behavior evidence.
- Plan a coherent named behavior batch and record actual RED, GREEN, and REFACTOR checks as they happen. No separate phase revisions, per-scenario production diff, retrospective commit-shape gate, or reconstructed TDD history is required.
- Implementation verification supplies evidence before review is written. Review consumes that evidence and records findings and a verdict; completed-review checks require neither an unwritten review during implementation nor a future archive receipt, and rerun no suites.
- A review PASS requires current green change-scoped evidence when applicable; an explicit no-behavior-change result without deltas is not applicable instead. Identity and integrity checks remain applicable independently of delta paths, and archive retains its merged-candidate verification.
- Receipts bind inputs captured before execution, not inputs first observed afterward. Relevant executable changes make evidence stale; documentation-only or evidence-only revisions can reuse unchanged tested inputs. This contract does not authenticate arbitrary runner honesty.
- Actual verdict declarations are parsed consistently: comments and fenced examples are ignored, conflicting actual verdicts fail, and negated or substring PASS prose is not a verdict.
- The README-impact decision remains a blocking completed-review requirement.
- Retained preservation comparison includes supported Backgrounds, inherited execution tags, steps, tables, docstrings, and all Examples content. Legitimate path moves are ignored; the result claims executable-content preservation, not semantic equivalence or an extra suite.
- Migration preservation failures require explicit retirement evidence. Honest regular and E2E scopes may cover identical lines; coverage equality is not a rejection or a replacement percentage/provenance gate.
- Expensive execution remains fail-fast; collecting independent cheap diagnostics adds neither new gates nor extra verification runs.
- The declared `e2e` role resolves the frozen acceptance package: one or more packages, each holding its own manifest, the runner, and every `@e2e` step definition, linking no production source. Several packages are accepted, and each is checked on its own manifest, including transitive and workspace-inherited edges. An empty resolution is refused.
- Feature files stay shared, so an `@e2e` scenario and a component scenario may live in the same `.feature` file; the tag is the package's scope, not a file boundary.
- The declared `e2e` roots and `source` roots are disjoint both ways: no resolved `e2e` path lies inside a `source` root, and no resolved `e2e` package's tree contains a `source` root.
- With the `ise` addon effective, the change check requires the resolved packages to exist, to have no dependency edge into `source`, and `verify.e2e_command` to name every resolved root. A resolved path in no package is checked by its path-resolving references instead of a manifest.
- The runner-target comparison is textual: it proves the command names each resolved `e2e` root, not that the process executed it. A run that reported the regular scope is refused by the result's scope.
- An unbound step is reported as a failed test and a reported skip is refused; neither yields a passed receipt, and each refusal states its own count.
- The verification receipt binds the digests of the resolved packages, the E2E runner command digest, the running Givn version, and the E2E scope outcome, so a replaced or weakened black-box suite changes the receipt.
- The E2E scope outcome is delta-aware: with an `@e2e` scenario in the delta the receipt requires a green E2E scope, and without one it records not applicable; either mismatch is stale.
- Only the resolved `End-to-end (E2E) test` packages are frozen as acceptance authority; component step definitions, fakes, and harness are implementation and are never inspected or diff-counted by the E2E identity, though a change to them still invalidates the receipt through the tested-content digest.
- The boundary is each resolved package's dependency graph plus a resolved path's path-resolving references, plus the runner's unbound or skipped step failure; no per-step phrase uniqueness rule exists, and the outline parsers are not repurposed for it.
- The four declaration findings are advisory on the specification-health check: a declared path missing from the tree, one path declared in two roles, a `.feature` file outside the `behavior` globs, and the declared `e2e` role resolving no package or resolving a package that carries a dependency edge into `source`. The `e2e` role's absent package is reported once by the boundary finding; the generic path-missing finding covers only the four roles other than `e2e`. The feature-retention scan excludes the active-change tree and the archive, so only permanent behavior is compared.
- One boundary resolver owns the condition text. The specification-health check renders the project-level placement conditions as advisory findings and vetoes nothing; the change-scoped tasks check renders the change-level acceptance conditions as refusals. The runner-target and runtime-step conditions belong to the tasks check alone; a malformed declaration remains the declaration's own blocking error.
- Advisory findings are rendered with `Severity::Advisory` and never change the lint exit code; checks that refuse on blocking findings keep their refusal.
- The specification-health check reports the project-level declaration state and vetoes no change, review, or archive. The change-scoped tasks check reads the declaration only while the change's locked add-on set has `ise` effective.
- A malformed declaration is an error of the declaration's own contract: the check names the offending key, role, or path and exits nonzero. It is not one of the four advisory findings.
- An empty role list retains nothing and produces no finding. A project or change with `ise` off never reads the declaration, even when a file is present.
- An `ise`-effective project whose declaration is absent or unreadable receives a blocking declaration error naming the path; the declaration checks are reported as not performed and no advisory finding is computed from a partial role set. The load runs before the empty-feature early return, so a project with no `.feature` file still reports it.
- The dependency-edge comparison reads Cargo manifests for each resolved package and stays static. A resolved path that resolves no package is checked by its path-resolving references only; a reference the reader cannot resolve to a project path, such as an external package name, is admitted. A malformed package manifest is refused by the change check because the boundary cannot be proven. The advisory pass keeps its existing findings and skips the edge comparison for a manifest it cannot read. The limit is documented in `docs/reference.md`.
- A change whose locked add-on set has `ise` off keeps today's receipt bytes, freshness rules, and single change-scoped runner; neither E2E field is serialized, and it gains no new refusal.
- The E2E identity is an additional stale reason, not a replacement for the tested-content digest. A component step definition remains tested content; the focused identity is what names the resolved protected packages.

## Examples

- A project's `behavior` globs miss `tests/features/extra.feature`: `givn lint` prints an advisory finding naming the file, and the exit code stays 0.
- The developer declares `limits: ["docs/performance.md"]` while that file does not exist; `givn lint` reports the missing path as advisory and still exits 0.
- One path is declared in both `structure` and `behavior`; `givn lint` reports the duplicate declaration once and still exits 0.
- The declared `e2e` package exists but its manifest names a package that sits under `source:`; `givn lint` reports the dependency edge and the package, and still exits 0.
- A project deletes `givn/specification.yaml` while `ise` is effective; `givn lint` exits nonzero, names the path, and reports the declaration checks as not performed instead of computing advisory findings.
- The same project runs `givn check review --change <id>`: no declaration finding becomes a blocking review finding unless the change's locked add-on set has `ise` effective.
- A `passuff`-style change with `ise` effective: an `@e2e` step imports a production module in process, so the tasks check refuses and names the dependency edge from the `e2e` package into `source`; no receipt is written.
- The declared `e2e` role matches two disjoint test crates; both resolve, each is checked on its own manifest, and the receipt carries one digest per crate.
- The declared `e2e` role matches the project root, so the resolved package contains `src/`; the check refuses and names the package and the `source` root instead of a separate project-root condition.
- A declared `e2e` path resolves no Cargo manifest; the check reads the references under it, refuses when one resolves into `src/`, and admits the path when every reference is external.
- The declared `e2e` package exists and links no production source, but `verify.e2e_command` names no resolved root; the check refuses and names the root the command must name.
- A delta has no `@e2e` scenario: the check records the E2E scope as not applicable, runs no E2E command, and still binds the package digests.
- An `@e2e` scenario's step definition is deleted: the run reports the unbound step as one failed test, the check fails, and no receipt is written.
- The E2E runner reports one skipped step: the run fails, the refusal names one skipped test, and the counts are not accepted as evidence.
- A project edits an assertion in a resolved `e2e` package after a receipt was issued: the package digest no longer matches, so review and archive refuse the earlier receipt and name the changed identity.
- A change whose locked add-on set has `ise` off runs the same single scope it ran before this change and writes a receipt with neither E2E key.
- Two checked scenarios share one reachable revision containing their regression tests; no separate production edit or phase commit is demanded.
- A long atomic import scenario records keep with a brief reason; a shape-matching short scenario needs no disposition.
- `givn check` explains that no checks ran and distinguishes static checks from runner-executing checks.

## Minimal guarantee

An invalid explicit check cannot pass; a bare check never implies verification
success or executes hooks or runners; an advisory finding never converts a
satisfiable request into a refusal.

## Success guarantee

The developer receives applicable, fresh, input-bound verification evidence or
actionable failures, with advisory findings and not-applicable checks
distinguished from completed proof. With the `ise` addon effective, the
specification-health check names the state of every declared role and the
declared acceptance package, the tasks check refuses a boundary violation, and
the evidence carries the E2E identity of the accepted results.

## Personas

- developer--primary-user
- agent--technical-role
- givn-maintainer--owner

## Capabilities

- validate-artifact-ids
- spec-overlap-gates
- review-disposition-gates
- resolve-change-directories
- givn-workflow-enforcement
- check-specification-health
- visual-review-gate

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| validate-artifact-ids | check an unknown artifact | Consumer invokes givn check with an unknown artifact and gets a helpful error |
| visual-review-gate | run the review check on a visual change | A visual change with complete evidence passes review |
| visual-review-gate | read the visual review guidance | Review guidance requires the visual verdict and persona lenses |
| spec-overlap-gates | detect a duplicate title | A duplicate title across two permanent spec files is a hard error |
| spec-overlap-gates | inspect duplicate JSON | The json output reports duplicate-title findings machine-readably |
| resolve-change-directories | run integrity in a change | Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected |
| resolve-change-directories | reject placeholder commit evidence | Integrity check detects a placeholder COMMIT hash in tasks.md and fails |
| check-specification-health | lint the specification corpus (the four declaration findings are data variants of this action) | All clean .feature files report a clean summary on stdout |
| givn-workflow-enforcement | run migration coverage check | Consumer runs migration coverage check and sees passed evidence |
| givn-workflow-enforcement | run the change-scoped checks | Consumer runs check tasks and receives a change verification receipt |
| givn-workflow-enforcement | request bare-check guidance without execution | Consumer invokes bare check and receives explicit non-executing guidance |
| givn-workflow-enforcement | write a runner result | The result helper writes four counts without a scope |
| givn-workflow-enforcement | run the review check on a change with use-case schema errors | Consumer runs review and receives every use-case schema problem in one pass |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Changing implementation code; automatic test execution from a bare check,
status, or completed-review inspection; retrospective reconstruction of
test-first history; preparing the trial (`prepare-an-ise-test`); declaring the
roles (`declare-the-specification`); publishing (`archive-change`).

## Diagram

```mermaid
flowchart LR
  Developer((Developer)) --> Check[[Run verification]]
  Check --> Boundary[[Check the declared E2E boundary]]
  Check --> Migration[[Compare migration evidence]]
  Check --> Findings[[Resolve findings]]
  Boundary --> Receipt[[Bind the E2E identity]]
```
