# Review: bound-review-context

Review kind: self-review
Author: agent
Reviewer context: agent

README-IMPACT: none

PERSONAS: agent--technical-role

## Fabrication audit

- Step definitions: no empty or no-op bodies in
  `tests/steps/workflow_enforcement_steps.rs`, `tests/steps/outline_review_gate_steps.rs`,
  or the touched fixture files. The new scenarios reuse existing assertions.
- Commit cross-check: `53ab78b6` (review-context gate), `f1e0dbf5`
  (review-context guidance), `3be6838b` (E2E fixture maintenance),
  `dc33067f` (outline review context), `ab101ae6` (ready delta specs), and
  `4c78c681` (task evidence). Each touches production source, guidance assets,
  or the scenario's test surface.
- Promised architecture: `src/review_context.rs`, the integrity-hook call, the
  archive call, and the outline identity check all exist in the tree.
- Strict mode: `tests/features_runner.rs` calls `.fail_on_skipped()`; the
  undefined-step probe failed in setup as required.
- No `@e2e` tag was removed. The change adds no new E2E action; the existing
  CLI driver is the real interface.
- `verify.e2e_command` is `./run-tests.sh --e2e`, distinct from
  `./run-tests.sh --non-e2e`.
- Interaction coverage: the new behavior runs through the existing `check
  review` action; no new inventory entry was added.

## Use-Case And Persona Conformance

The `verify-change` contract gains the review-context record and the outline
identity cross-check. Its existing guarantees and capability ownership are
preserved. `agent--technical-role` and `developer--primary-user` are the
confirmed review lenses; the Gherkin actor remains the developer.

## Visual review

The design declares `Interface: none`. No screenshot, transcript, or visual
verdict is required. The persona disposition is recorded above.

## Retirements

RETIRE: none.

## Arc42 implementation conformance

The 12-row assessment is complete. The touched chapters are 02 (TC-43/44), 05
(`review_context` and `outline::gate`), 06 (check-review step), 08 (review
context and outline identity), 10 (Q-120), 11 (R-124/R-125), and 12 (review
terms). The content matches `design.md`, `tasks.md`, and the built code.

ARC42 CONFORMANCE: CLEAN

## Coverage

The change has no archive receipt yet. Coverage is measured once by the archive
against the committed candidate. No production path was added outside the new
gate module and the outline identity check; the delta scenarios exercise the
accept and refusal branches.

## Scenario execution evidence

`givn check tasks --change bound-review-context` recorded a current receipt:
15 scenarios, 0 failed, 0 skipped at `ab101ae6`. The full non-E2E suite ran
713 tests with 713 passed; the E2E suite ran 112 tests with 112 passed.

**Receipt status:** proven for the change scope.

## Overlap dispositions

Every pair is a `boundary`: each delta scenario asserts a distinct review-kind
or outline-identity rule, even when it shares a Given/Then shape with a
permanent scenario.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A recorded self-review is accepted | An independent review by a different context is accepted | boundary |
| A recorded self-review is accepted | A legacy review record without a kind keeps its existing gate result | boundary |
| An independent review by a different context is accepted | A legacy review record without a kind keeps its existing gate result | boundary |
| An independent review whose reviewer is the author is refused | A self-review cannot satisfy a recorded independent request | boundary |
| A self-review outline uses the self context | An independent outline uses the recorded reviewer context | boundary |
| A self-review outline uses the self context | A legacy outline without a review kind keeps the reviewer-differs rule | boundary |
| An independent outline uses the recorded reviewer context | A legacy outline without a review kind keeps the reviewer-differs rule | boundary |
| An outline reviewer that differs from the recorded context is refused | An outline author that differs from the recorded author is refused | boundary |
| An outline reviewer that differs from the recorded context is refused | A legacy outline with the author as reviewer is refused | boundary |
| An outline author that differs from the recorded author is refused | A legacy outline with the author as reviewer is refused | boundary |
| givn addons disable when already disabled is idempotent | A recorded self-review is accepted | boundary |
| givn addons disable when already disabled is idempotent | An independent review by a different context is accepted | boundary |
| givn addons disable when already disabled is idempotent | A legacy review record without a kind keeps its existing gate result | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | A recorded self-review is accepted | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | An independent review by a different context is accepted | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | A legacy review record without a kind keeps its existing gate result | boundary |
| The check passes on a complete README | A recorded self-review is accepted | boundary |
| The check passes on a complete README | An independent review by a different context is accepted | boundary |
| The check passes on a complete README | A legacy review record without a kind keeps its existing gate result | boundary |
| Updated impact decision passes the review | A recorded self-review is accepted | boundary |
| Updated impact decision passes the review | An independent review by a different context is accepted | boundary |
| Updated impact decision passes the review | A legacy review record without a kind keeps its existing gate result | boundary |
| A disabled addon needs no impact decision | A recorded self-review is accepted | boundary |
| A disabled addon needs no impact decision | An independent review by a different context is accepted | boundary |
| A disabled addon needs no impact decision | A legacy review record without a kind keeps its existing gate result | boundary |
| A recorded impact decision passes the review | A recorded self-review is accepted | boundary |
| A recorded impact decision passes the review | An independent review by a different context is accepted | boundary |
| A recorded impact decision passes the review | A legacy review record without a kind keeps its existing gate result | boundary |
| Integrity hook fails on placeholder commit hash | An independent review whose reviewer is the author is refused | boundary |
| Integrity hook fails on placeholder commit hash | A self-review cannot satisfy a recorded independent request | boundary |
| Integrity hook fails on missing step-definition file | An independent review whose reviewer is the author is refused | boundary |
| Integrity hook fails on missing step-definition file | A self-review cannot satisfy a recorded independent request | boundary |
| Integrity hook fails on empty step-definition file | An independent review whose reviewer is the author is refused | boundary |
| Integrity hook fails on empty step-definition file | A self-review cannot satisfy a recorded independent request | boundary |
| Integrity hook passes when arc42 is disabled | A recorded self-review is accepted | boundary |
| Integrity hook passes when arc42 is disabled | An independent review by a different context is accepted | boundary |
| Integrity hook passes when arc42 is disabled | A legacy review record without a kind keeps its existing gate result | boundary |
| Integrity hook fails on unedited arc42 chapter | An independent review whose reviewer is the author is refused | boundary |
| Integrity hook fails on unedited arc42 chapter | A self-review cannot satisfy a recorded independent request | boundary |
| Integrity hook passes on edited arc42 chapter | A recorded self-review is accepted | boundary |
| Integrity hook passes on edited arc42 chapter | An independent review by a different context is accepted | boundary |
| Integrity hook passes on edited arc42 chapter | A legacy review record without a kind keeps its existing gate result | boundary |
| Archive refuses a change without a signed design-review | An independent review whose reviewer is the author is refused | boundary |
| Archive refuses a change without a signed design-review | A self-review cannot satisfy a recorded independent request | boundary |
| An unreadable outline fails while the metrics are clean | An outline reviewer that differs from the recorded context is refused | boundary |
| An unreadable outline fails while the metrics are clean | An outline author that differs from the recorded author is refused | boundary |
| An unreadable outline fails while the metrics are clean | A legacy outline with the author as reviewer is refused | boundary |
| A PASS without a responsibility and call-order explanation fails | An outline reviewer that differs from the recorded context is refused | boundary |
| A PASS without a responsibility and call-order explanation fails | An outline author that differs from the recorded author is refused | boundary |
| A PASS without a responsibility and call-order explanation fails | A legacy outline with the author as reviewer is refused | boundary |
| A missing reviewer fails | An outline reviewer that differs from the recorded context is refused | boundary |
| A missing reviewer fails | An outline author that differs from the recorded author is refused | boundary |
| A missing reviewer fails | A legacy outline with the author as reviewer is refused | boundary |
| A reviewer cannot be the author | An outline reviewer that differs from the recorded context is refused | boundary |
| A reviewer cannot be the author | An outline author that differs from the recorded author is refused | boundary |
| A reviewer cannot be the author | A legacy outline with the author as reviewer is refused | boundary |
| Conflicting outline verdicts fail | An outline reviewer that differs from the recorded context is refused | boundary |
| Conflicting outline verdicts fail | An outline author that differs from the recorded author is refused | boundary |
| Conflicting outline verdicts fail | A legacy outline with the author as reviewer is refused | boundary |
| A change with no production sources fails | An outline reviewer that differs from the recorded context is refused | boundary |
| A change with no production sources fails | An outline author that differs from the recorded author is refused | boundary |
| A change with no production sources fails | A legacy outline with the author as reviewer is refused | boundary |
| Dirty production paths fail closed before analysis | An outline reviewer that differs from the recorded context is refused | boundary |
| Dirty production paths fail closed before analysis | An outline author that differs from the recorded author is refused | boundary |
| Dirty production paths fail closed before analysis | A legacy outline with the author as reviewer is refused | boundary |
| A change directory without a reachable commit fails closed | An outline reviewer that differs from the recorded context is refused | boundary |
| A change directory without a reachable commit fails closed | An outline author that differs from the recorded author is refused | boundary |
| A change directory without a reachable commit fails closed | A legacy outline with the author as reviewer is refused | boundary |
| A shallow clone fails closed | An outline reviewer that differs from the recorded context is refused | boundary |
| A shallow clone fails closed | An outline author that differs from the recorded author is refused | boundary |
| A shallow clone fails closed | A legacy outline with the author as reviewer is refused | boundary |
| A changed Python test module is not production scope | A self-review outline uses the self context | boundary |
| A changed Python test module is not production scope | An independent outline uses the recorded reviewer context | boundary |
| A changed Python test module is not production scope | A legacy outline without a review kind keeps the reviewer-differs rule | boundary |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A recorded self-review is accepted | boundary |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | An independent review by a different context is accepted | boundary |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A legacy review record without a kind keeps its existing gate result | boundary |
| Review blocks when a shape match involving the change's delta is undisposed | Design review and final review share one independent context | boundary |
| Review blocks when a long scenario in the delta has no split-or-keep entry | An independent review whose reviewer is the author is refused | boundary |
| Review blocks when a long scenario in the delta has no split-or-keep entry | A self-review cannot satisfy a recorded independent request | boundary |
| Review passes when a long scenario has a split-or-keep entry | A recorded self-review is accepted | boundary |
| Review passes when a long scenario has a split-or-keep entry | An independent review by a different context is accepted | boundary |
| Review passes when a long scenario has a split-or-keep entry | A legacy review record without a kind keeps its existing gate result | boundary |
| Review blocks when a delta scenario shape-matches a permanent scenario without disposition | An independent review whose reviewer is the author is refused | boundary |
| Review blocks when a delta scenario shape-matches a permanent scenario without disposition | A self-review cannot satisfy a recorded independent request | boundary |
| Review passes when a shape match exists only between permanent scenarios | A recorded self-review is accepted | boundary |
| Review passes when a shape match exists only between permanent scenarios | An independent review by a different context is accepted | boundary |
| Review passes when a shape match exists only between permanent scenarios | A legacy review record without a kind keeps its existing gate result | boundary |
| A modified delta matching its merge target in the same capability is allowed | A recorded self-review is accepted | boundary |
| A modified delta matching its merge target in the same capability is allowed | An independent review by a different context is accepted | boundary |
| A modified delta matching its merge target in the same capability is allowed | A legacy review record without a kind keeps its existing gate result | boundary |
| A removed delta referencing its target by title is allowed | A recorded self-review is accepted | boundary |
| A removed delta referencing its target by title is allowed | An independent review by a different context is accepted | boundary |
| A removed delta referencing its target by title is allowed | A legacy review record without a kind keeps its existing gate result | boundary |
| The default scope excludes the archive tree | A recorded self-review is accepted | boundary |
| The default scope excludes the archive tree | An independent review by a different context is accepted | boundary |
| The default scope excludes the archive tree | A legacy review record without a kind keeps its existing gate result | boundary |
| givn check with known artifact proceeds to run hooks | A recorded self-review is accepted | boundary |
| givn check with known artifact proceeds to run hooks | An independent review by a different context is accepted | boundary |
| givn check with known artifact proceeds to run hooks | A legacy review record without a kind keeps its existing gate result | boundary |
| A visual change with complete evidence passes review | A recorded self-review is accepted | boundary |
| A visual change with complete evidence passes review | An independent review by a different context is accepted | boundary |
| A visual change with complete evidence passes review | A legacy review record without a kind keeps its existing gate result | boundary |
| A visual change with incomplete evidence fails review | An independent review whose reviewer is the author is refused | boundary |
| A visual change with incomplete evidence fails review | A self-review cannot satisfy a recorded independent request | boundary |
| A change without a visual contract is exempt from the visual gate | A recorded self-review is accepted | boundary |
| A change without a visual contract is exempt from the visual gate | An independent review by a different context is accepted | boundary |
| A change without a visual contract is exempt from the visual gate | A legacy review record without a kind keeps its existing gate result | boundary |
| A terminal change passes review with a transcript | A recorded self-review is accepted | boundary |
| A terminal change passes review with a transcript | An independent review by a different context is accepted | boundary |
| A terminal change passes review with a transcript | A legacy review record without a kind keeps its existing gate result | boundary |
| A change that declares no visual interface is exempt | A recorded self-review is accepted | boundary |
| A change that declares no visual interface is exempt | An independent review by a different context is accepted | boundary |
| A change that declares no visual interface is exempt | A legacy review record without a kind keeps its existing gate result | boundary |
| A web contract with complete state evidence passes review | A recorded self-review is accepted | boundary |
| A web contract with complete state evidence passes review | An independent review by a different context is accepted | boundary |
| A web contract with complete state evidence passes review | A legacy review record without a kind keeps its existing gate result | boundary |
| A visual change with a confirmed persona disposition passes review | A recorded self-review is accepted | boundary |
| A visual change with a confirmed persona disposition passes review | An independent review by a different context is accepted | boundary |
| A visual change with a confirmed persona disposition passes review | A legacy review record without a kind keeps its existing gate result | boundary |
| A deferral with an operator decision passes review | A recorded self-review is accepted | boundary |
| A deferral with an operator decision passes review | An independent review by a different context is accepted | boundary |
| A deferral with an operator decision passes review | A legacy review record without a kind keeps its existing gate result | boundary |
| A change without a visual interface passes review with a reasoned none | A recorded self-review is accepted | boundary |
| A change without a visual interface passes review with a reasoned none | An independent review by a different context is accepted | boundary |
| A change without a visual interface passes review with a reasoned none | A legacy review record without a kind keeps its existing gate result | boundary |
| A machine-only CLI records not applicable and passes review | A recorded self-review is accepted | boundary |
| A machine-only CLI records not applicable and passes review | An independent review by a different context is accepted | boundary |
| A machine-only CLI records not applicable and passes review | A legacy review record without a kind keeps its existing gate result | boundary |

## Split-or-keep

No delta scenario exceeds the long-scenario threshold.

| Scenario | Decision |
|---|---|

## Deferrals

None.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production source or
      its guidance surface.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope.
- [x] Coverage classified from inspection; no new gap.
- [x] No `@wip` tags remain in the delta specs.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command unchanged; no external dependency.
- [x] `verify.e2e_command` isolation proved by the existing scope counts.
- [x] Implementation matches `design.md`.
- [x] No finding excused outside the three coverage buckets.

OUTLINE-COMMAND: givn outline report --path src
OUTLINE: AUTHOR agent
OUTLINE: REVIEWER agent
OUTLINE: PASS module:src/cli/archive.rs | runs the archive transaction from prerequisite and receipt checks through merge, candidate publication, verification, coverage, and finalization; checks precede mutation and verification follows the published candidate
OUTLINE: PASS module:src/integrity_hook.rs | runs the review-time integrity checks for one change; revision and promised-file evidence are checked first, then review context, visual/persona, deferral, arc42, coverage, and README constraints
OUTLINE: PASS module:src/main.rs | dispatches one CLI invocation to its command; parsing and the optional development-record wrapper run before the selected command's dispatch
OUTLINE: PASS module:src/outline/gate.rs | evaluates the change-relative outline review; it resolves the base and head, analyzes both trees, parses the review markers, validates the recorded review identity, and renders one outcome
OUTLINE: PASS module:src/review_context.rs | parses and validates the proposal, design-review, and review records; each artifact is parsed first, then kinds, authors, and reviewer contexts are compared before the single result
OUTLINE: PASS module:src/spec/routing_table.rs | checks proposal routing decisions against the delta corpus; it collects delta capabilities, parses the routing table, resolves each use case or fragment, validates the decision and interaction, and emits deterministic findings
OUTLINE: PASS type:src/cli/archive.rs::PruneRules | decides whether an archive path is pruned; the relative-path predicate is the only call
OUTLINE: PASS type:src/outline/gate.rs::GateFailure | names and orders one gate failure; the stable sort key is derived from the failure group, variant, path, unit, and rendered message
OUTLINE: PASS type:src/outline/gate.rs::GateOutcome | holds the gate result and renders it; rendering walks the final line and the collected failure messages in sort order
OUTLINE: PASS type:src/outline/gate.rs::GateRequest | carries the change and outline scope into the gate; construction resolves the OUTLINE-COMMAND scope and root rendering exposes the selected source roots
OUTLINE: PASS type:src/outline/gate.rs::Side | labels the base or head side of the comparison; the label is the only call
OUTLINE: PASS type:src/review_context.rs::ReviewKind | represents the recorded review kind; parsing normalizes one value and the label returns the canonical spelling
OUTLINE: PASS type:src/spec/routing_table.rs::DocumentKind | labels the cached document kind; the label is the only call
OUTLINE: PASS

REVIEW: PASS
