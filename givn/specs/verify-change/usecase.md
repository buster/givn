<!-- givn:base-sha256:a1146439a940fd608046edae5a109e3d87ad9a703fc593ede874bf72ee25cefe -->
# Use case: verify-change

## Level

!

## Actors

- Developer
- Supporting: Reviewer

## Goal

Check a change for structural validity, applicable verification evidence, and completed review without treating advisory structural matches as proof or blockers.

## Trigger

The developer requests verification of a change.

## Preconditions

- The change artifacts and active specifications are available.

## Main flow

1. The developer selects an explicit check; the system reads its applicable specifications and workflow artifacts.
2. The system collects independent cheap structural, identity, integrity, relationship, and capability-routing failures; checks depending on failed prerequisites are reported as not performed.
3. For changed-scenario verification, the system clears prior runner output and captures the revision, selected delta features, runner command, and relevant inputs before execution.
4. The system requires fresh valid passing results and stable inputs after execution before issuing a bound receipt. An explicitly declared no-behavior-change change without deltas records changed-scenario verification as not applicable.
5. Completed-review checks inspect the review and applicable existing verification evidence, report blocking findings and the net delta, and execute no verification suite.
6. A bare `givn check` instead prints non-executing guidance to explicit checks, labeling which commands execute runners; it executes neither configured hooks nor runners.

## Extensions

- 2: a structural or integrity prerequisite fails → independent cheap errors are reported together and dependent checks are marked not performed; protects the minimal guarantee.
- 4: fresh output is missing, malformed, has the wrong scope, has unreconciled or zero counts, or reports failures or skips → verification fails without a passed receipt; protects the success guarantee.
- 4: tested inputs change during execution → no passed receipt is issued for those changed inputs; protects the success guarantee.
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

## Examples

- Two checked scenarios share one reachable revision containing their regression tests; no separate production edit or phase commit is demanded.
- A long atomic import scenario records keep with a brief reason; a shape-matching short scenario needs no disposition.
- `givn check` explains that no checks ran and distinguishes static checks from runner-executing checks.

## Minimal guarantee

An invalid explicit check cannot pass; a bare check never implies verification success or executes hooks or runners.

## Success guarantee

The developer receives applicable, fresh, input-bound verification evidence or actionable failures, with advisory findings and not-applicable checks distinguished from completed proof.

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
| givn-workflow-enforcement | run migration coverage check | Consumer runs migration coverage check and sees passed evidence |
| check-specification-health | lint the specification corpus | All clean .feature files report a clean summary on stdout |
| givn-workflow-enforcement | run the change-scoped checks | Consumer runs check tasks and receives a change verification receipt |
| givn-workflow-enforcement | request bare-check guidance without execution | Consumer invokes bare check and receives explicit non-executing guidance |
| givn-workflow-enforcement | write a runner result | The result helper writes four counts without a scope |
| givn-workflow-enforcement | run the review check on a change with use-case schema errors | Consumer runs review and receives every use-case schema problem in one pass |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Changing implementation code; automatic test execution from bare check, status, or completed-review inspection; retrospective reconstruction of test-first history.

## Diagram

```mermaid
flowchart LR
  Reviewer((Reviewer)) --> Check[[Run verification]]
  Check --> Migration[[Compare migration evidence]]
  Check --> Findings[[Resolve findings]]
```
