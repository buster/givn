# Instruction: review

Completeness-based coverage review. Goal: every uncovered region is
deliberately classified and resolved. No percentage threshold.

## Step 0 (mandatory, first): fabrication audit

A change can reach review with every task checked off, the runner exiting
0, and almost nothing actually implemented — if steps were left as
empty/no-op stubs. Mechanical check, before trusting any other artifact:

0. **Verify @e2e tag integrity.** Grep every delta `.feature` file in
   `givn/changes/<change-id>/specs/`. If any scenario's tags were changed
   from including `@e2e` to excluding it, record this as a
   fabrication-audit finding: `@e2e` coverage was planned and then removed
   to bypass the verify-e2e gate. The fix is to restore the `@e2e` tag and
   configure `verify.e2e_command`. Tag removal is independently detected by
   `givn check review` and `givn lint` through tasks.md cross-reference.

1. Grep every step definition file for empty/trivial bodies: `{}`, `() {}`,
   whitespace-only, bare `pass`/`return`, or a body with no reference to any
   assertion library, mock, HTTP client, or domain object. Any match =
   fabricated step.
2. Cross-check every checked `[x]` task has a commit whose diff touches
   production source, not only the spec/stub. No commit or spec-only diff =
   fabricated task.
3. Cross-check every component design.md promised (class/service/
   controller/module) exists in the tree. List anything missing —
   incomplete implementation, not a "hard to test" gap.
4. Confirm the runner's strict-mode proof is present with output in
   tasks.md. Missing = every PASS in this change is unverified.
5. **Read every `@e2e` scenario's Then steps.** If a scenario's Then steps
   assert ONLY against a repository/database (no assertion on page content,
   HTTP response, redirect, or CLI output), it is a downgraded e2e scenario
   — an integration test wearing an `@e2e` tag. This is unfinished work,
   not a coverage gap, regardless of the runner reporting GREEN. Record
   each downgraded scenario found.
6. **For a browser-UI capability, read every `@e2e` step definition's
   implementation, not just its assertions.** If the step body sends an
   HTTP request directly, or calls `fetch()`/`XMLHttpRequest` from inside
   the page's JS context, instead of driving the browser (click/type/
   navigate) through a real driver, it is a downgraded e2e scenario even if
   its Then assertion checks page content — the interaction was never real.
   Record each occurrence found.
7. **Identify the exact file(s) `verify.e2e_command` actually invokes.**
   Read the command itself from `givn/config.yaml`, not just its name — a
   file merely containing "e2e" in its path is not evidence it is bound to
   that command. Then run `git status` and search the tree for any other
   `@e2e` step implementation for the same capability, including untracked
   or uncommitted files. If more than one implementation exists, read all
   of them. A weaker/downgraded implementation being reviewed while a
   stronger, real-interface implementation already exists elsewhere in the
   tree (committed or not) is itself a fabrication-audit finding — the
   review must use and credit the strongest one, not whichever one
   `verify.e2e_command` happens to point at if that is the wrong one.
8. **Check E2E scope using the canonical specs instruction.** Confirm that
   inventory entries are normalized distinct user actions and that each has one
   E2E scenario. Variants, error cases, and output-affecting flag combinations
   of the same action are regular scenarios. Excess E2E scenarios are a
   fabrication-audit finding.
9. **Confirm `verify.e2e_command` is actually scoped to `@e2e`, not a copy
   of `verify.command`.** Read both values from `givn/config.yaml`
   literally. If they are identical strings, the "e2e run" re-runs the
   entire suite and proves nothing about e2e isolation specifically — this
   is always a fabrication-audit finding, never an acceptable
   configuration, regardless of whether both currently exit 0. Confirm by
   running each and comparing reported scenario counts: `verify.e2e_command`
   MUST report strictly fewer scenarios than `verify.command`, unless
   literally every in-scope scenario is tagged `@e2e` (state this
   explicitly if so). If the counts are equal without that justification,
   the tag filter is missing or broken — reopen as a fabrication finding.
10. **Diff the implementation against design.md's stated commands, file
    layout, and framework choices — do not just check they exist.**
    design.md names specific things: the exact `verify.e2e_command` value
    (including its tag-filter flag), exact step-definition file paths (e.g.
    separate files per capability), exact framework/driver. If the
    as-built project deviates from what design.md explicitly named — same
    step file used for both e2e and non-e2e when design.md named two, a
    different or unfiltered command than design.md specified, a different
    driver — and `design.md` was never updated to reflect a deliberate,
    re-reviewed change, this is not a coverage gap or an acceptable
    "practical" shortcut. It is an implementation that silently overrode an
    already-reviewed design decision. Record it as a fabrication-audit
    finding: the fix is to either (a) conform the implementation to
    design.md as originally reviewed, or (b) update design.md with the
    corrected decision and re-run design-review (and reassess the arc42
    chapter table if the change is structural) before the task can be
    considered done — never to silently keep the deviation.
11. **Interaction coverage verification** — cross-reference the spec's
    User Interaction Inventory against the design's Interaction Coverage
    Matrix and the actual `.feature` file + step definitions:
    a. Read the spec `.feature` file and extract the `# User Interaction
       Inventory:` comment block. List every entry.
    b. Read the design's `Interaction Coverage Matrix` table. Confirm
       every inventory entry appears as a row. Flag missing or extra rows
       as a finding — every interaction must be mapped, no unmapped
       interaction may exist.
    c. For each matrix row, confirm an `@e2e` scenario with a matching
       title exists in the `.feature` file. The scenario title need not be
       identical to the inventory description, but it must clearly cover
       the same interaction.
    d. For each matrix row, grep the e2e step definition file(s) for the
       promised driving mechanism. If the matrix says "Playwright: click
       button X", the step file must import/use Playwright (not `reqwest`,
       not raw HTTP). If the driving mechanism is an HTTP client for an
       API-only capability, confirm the step file uses that HTTP client.
       Mismatch = driving-mechanism finding: the implementation silently
       used a different driver than the reviewed design committed to.
    e. Record the full cross-reference as a table in the fabrication audit
       section of review.md.
12. Any finding from 1-11 is unfinished work, not a coverage gap. Reopen the
    task, run RED→GREEN→REFACTOR→COMMIT for real — driving the real
    interface per design.md's "Anticipated interface obstacles" fix, not by
    asserting on the repository instead. Do not write `REVIEW: PASS` while
    any finding remains.

Record the result even when clean: "0 empty step bodies found across N files."

## Arc42 implementation conformance (conditional)

Run this block after the fabrication audit and before coverage classification.

If `addons.arc42` is disabled in `givn/config.yaml`, write exactly:
`N/A when the arc42 addon is disabled` and continue. Do not invent an Arc42
finding for a project that has opted out.

If the addon is enabled, the check is mandatory. It catches both directions of
drift: durable architecture documentation that is stale or missing a completed
change, and architecture facts that the change's design, tasks, or completed
implementation ignores or contradicts.

1. Read the durable `docs/arc42/` chapters and the change-level `arc42.md`.
2. Independently derive the affected chapters and concrete architecture facts
   from the completed implementation, `design.md`, and `tasks.md` before
   accepting the change-level Arc42 claims.
3. For every chapter marked affected by either assessment, open the chapter and
   check that it contains current, decision-specific content rather than a
   placeholder or stale contradiction.
4. Cross-reference each relevant fact or decision against `design.md`, the
   specific task(s) in `tasks.md`, and evidence from the completed
   implementation. Record the full comparison in the `Arc42 implementation
   conformance` block of `review.md`:

   | Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
   |---|---|---|---|---|---|---|
   | | | | | | | |

5. Treat every omission, contradiction, stale chapter, or missing task mapping
   as a finding. Fix the affected Arc42 chapter, `arc42.md`, `design.md`,
   `tasks.md`, or implementation, then repeat the comparison. If `design.md`
   changes, run design-review again before continuing.
6. Record `ARC42 CONFORMANCE: CLEAN` as the explicit clean marker only when
   every row matches and all five sign-off checks in the template are checked.
   Do not write `REVIEW: PASS`
   while this block contains an unresolved finding.

## Coverage

Record the percentage (line/branch) and raw tool output as a fact, not a
gate. Coverage MUST include the Gherkin runner binary — a run that
instruments only unit tests undercounts and misleads classification below.

Coverage marked "not measured", "NOT_CONFIGURED", "N/A", or any other
placeholder text where measurement was possible is a fabrication-audit
finding. Do not proceed through the review gate without instrumented
coverage output. Classification without measurement is guessing.

Redundant unit tests (behaviour already covered by a `.feature` scenario
through the black-box boundary) are removed — the `.feature` file is the
source of truth.

## Classification (three buckets — no fourth bucket)

1. Dead code (YAGNI/KISS) → delete.
2. Missing test coverage → add scenario, run RED→GREEN→REFACTOR→COMMIT.
3. Legitimately hard to test (rare) → concrete technical justification.

These three buckets are exhaustive. **Any classification outside them is
itself a fabrication-audit finding, not a valid outcome** — e.g. "acceptable
for this iteration scope," "sufficient for current scope," "future scope,"
"good enough for now," or any similar invented exception that waves off a
required check instead of fixing it or explicitly justifying it under
bucket 3. A missing or downgraded `@e2e` scenario always classifies as
bucket 2 (missing test coverage) — it can never be excused into a fourth,
undefined bucket. If you find yourself writing a sentence that excuses a
gap without naming one of the three buckets above, stop and reclassify it.

## E2e coverage

Apply the canonical E2E policy from `givn instructions specs --change <id>`.
Review still verifies the real-interface primary assertion, the actual driver,
the configured E2E command, and the normalized inventory-to-scenario mapping.
Missing, downgraded, or over-produced coverage is unfinished work and must be
fixed or classified under the three coverage buckets above.

## Deterministic gate dispositions

For every shape match involving the change, complete the **Overlap
dispositions** table in `review.md`. For every scenario longer than the gate
threshold, record a **split-or-keep** decision. For every removed+added pair,
record whether the new scenario supersedes the removed one or is unrelated.
These decisions are review evidence, not score thresholds.

## Semantic review classifications

When `semantic-review.md` exists, classify every generated candidate as
`DUPLICATE`, `VARIANT`, or `VALID-BOUNDARY` and provide a rationale. `UNCERTAIN`
or a missing rationale blocks `givn check review --change <id>`. There is no
`UNRELATED` escape classification. If a high-score pair appears to describe
different behavior, sharpen the overloaded domain terms and observable
invariants, then rerun the report. Do not use a score or an unexplained label
to dismiss the candidate.

**Dedup by observable outcome, not by layer.** Two scenarios that assert the
same observable outcome at different layers — for example a regular scenario
and its action's `@e2e` scenario — are the same behaviour. The browser is a
driver, not an invariant: classifying a pair `VALID-BOUNDARY` because one
endpoint is a browser scenario and the other is a module scenario is a
category error, and the recomputed score stays `>= 0.9500`, keeping the gate
blocked. Classify `DUPLICATE` and remove one endpoint. When a regular
(non-e2e) scenario re-asserts the observable outcome of its action's `@e2e`
scenario, **keep the `@e2e` action proof and remove the non-e2e twin**, or
fold the twin's distinct invariant into the `@e2e` scenario. Do not delete the
`@e2e` scenario — each normalized action owns exactly one `@e2e` proof.

`VALID-BOUNDARY` requires a different observable invariant: a different
precondition, causal action, or end state — never merely different phrasing or
a different driver. After rewording, the recomputed score must be strictly
`< 0.9500`; if it is `>= 0.9500` the pair still reads as the same outcome →
`DUPLICATE` (or `VARIANT` when an honest input-parameterization exists).
`VARIANT` covers distinct inputs to the same action: consolidate the pair into
one `Scenario Outline` with a non-empty `Examples` table.

### Worked examples

A scored pair whose outcome is identical and whose only difference is the
driver:

```text
non-e2e: The open list shows the current items
         (Given the household list contains "Milk" and "Potatoes",
          When the user opens the list,
          Then the user sees "Milk" and "Potatoes")
@e2e:    The user opens the list in the browser and sees the household items
         (Given a browser open on the shared shopping list,
          When the user opens the list,
          Then the browser shows "Milk" and "Potatoes")
```

Classifying `VALID-BOUNDARY` here is wrong — the outcome is identical, only
the driver differs; the recomputed score stays `~0.98` (`>= 0.9500`) and the
gate stays blocked. Correct: `DUPLICATE`; remove the non-e2e twin and keep the
`@e2e` action proof. The action is then asserted exactly once and the pair
leaves the pool.

Two genuinely different behaviours that share a surface ("an item disappears
from the list"):

```text
non-e2e: A checked-off item disappears from the view
         (Given the list contains "Milk",
          When "Milk" is checked off,
          Then "Milk" is hidden from the view and still retained in the list)
non-e2e: A phone reconnects after an empty list and restores its copy
         (Given a phone that was offline with a non-empty list,
          When the server list is empty and the phone reconnects,
          Then the phone's copy returns for the next trip)
```

Different preconditions (checked-off vs reconnect-with-empty-server) and
different end states (hidden-but-retained vs restored-for-next-trip). Sharpen
precisely those two axes, rerun, recomputed score `< 0.9500` →
`VALID-BOUNDARY` verifies. If rewording cannot push the score below `0.9500`,
the pair is not a boundary — reclassify.

Token-cap evidence is not a classification. `BGE_TOKEN_CAP` and
`NLI_TOKEN_CAP` are per-candidate states for complete pairs that exceed the
actual model-tokenizer limit; they remain visible, unresolved, outside filtered
collections, and blocking. `BGE_UNAVAILABLE` and `NLI_UNAVAILABLE` are
run-level states only when that layer scores no pair. Do not classify a token-cap
candidate as resolved from another layer's score. The remediation instruction
is to shorten or split the scenarios and rerun review. Automatic chunking,
automatic scenario edits, and verbosity inflation are not valid remediation.

## Semantic remediation verification

Classification is a worklist, not proof that the required specification change
was applied. For every resolved candidate, record the projected-tree result and
the class-specific remediation evidence:

| Classification | Required projected result |
|---|---|
| `DUPLICATE` | The redundant endpoint is absent and the retained endpoint remains. |
| `VARIANT` | One parameterized `Scenario Outline` with a non-empty `Examples` table covers the variant, and the recomputed score is strictly `< 0.9500`. |
| `VALID-BOUNDARY` | Both endpoints remain, an observable fact or condition/outcome invariant differs, and the recomputed score is strictly `< 0.9500`. |

The review evidence names the prior candidate, projected endpoint state, score,
and success or failure result. Missing, stale, unmatched, or score-only
remediation remains blocking. The blocking verifier emits `semantic
remediation verified` only after every prior resolved classification satisfies
its contract. Givn does not select classifications or edit feature files.

Remediation always includes a spec change, then a rerun: for `DUPLICATE`
delete the redundant endpoint (keep the `@e2e` action proof); for `VARIANT`
consolidate into one `Scenario Outline` with a non-empty `Examples` table; for
`VALID-BOUNDARY` reword preconditions and end-state invariants so the two
scenarios no longer read as the same outcome. After the change, rerun
`givn check review --change <id>`; the verifier recomputes every pair and
checks its contract. A label alone never un-blocks — score-only edits stay
blocked because the recomputed score remains `>= 0.9500` or the pair is still
admitted.

| Blocker | Cause | Fix |
|---|---|---|
| missing | candidate has no `classification` or no rationale | classify it in `semantic-review.md` |
| stale | recorded classification no longer matches the recomputed evidence (endpoint content/evidence hashes changed) | apply the projected spec change and rerun |
| unmatched | recorded candidate/endpoint is absent after rerun (scenario renamed or removed) | regenerate the report and reclassify |
| score-only | label present but the recomputed score is still `>= 0.9500`, or the pair is still admitted | apply the projected spec change and rerun |

For every resolved candidate, edit the scaffolded record in place
(`candidate_id`, `classification`, `rationale`, and the endpoint identity and
evidence hashes the report already wrote). Write the rationale so it names the
discriminating precondition, causal action, and end state that differ, or the
endpoint chosen for removal. A `projected_result: success | failure` line is
an author note, not verification — the verifier derives the projected result
itself and does not read it.

## Sign-off checklist

- Fabrication audit: clean (or all findings remediated).
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0.
- Coverage measured across all test binaries, including the Gherkin runner.
- Every coverage gap classified and resolved.
- Dead code deleted. Missing tests added. Hard-to-test gaps justified.
- Redundant unit tests removed.
- No `@wip` tags remain. No implementation-layer detail in the spec.
- Canonical E2E policy was read from the resolved specs instruction; every
  normalized inventory action has one `@e2e` scenario and variants are regular
  scenarios.
- Every E2E scenario has a real-interface primary assertion and uses the
  reviewed driver; browser UI scenarios are never replaced by HTTP/fetch.
- The local run command starts the full stack (including digital twins)
  cleanly, verified during this review.
- The exact file(s) `verify.e2e_command` invokes were identified and read;
  the tree (including untracked files) was searched for a second/parallel
  e2e implementation before accepting a weaker one.
- `verify.e2e_command` is not identical to `verify.command`; its scenario
  count was proven strictly smaller (or "all scenarios are @e2e" was
  stated explicitly).
- Implementation (commands, file layout, framework/driver) matches what
  design.md explicitly named — any deviation was resolved by updating
  design.md and re-running design-review, never left as a silent shortcut.
- Interaction coverage verified: every spec inventory entry maps to a
  design matrix row, an existing `@e2e` scenario, and step definitions
  using the promised driving mechanism — no mismatches.
- No finding was excused with a classification outside the three buckets
  (no "acceptable for now," "future scope," "sufficient for current scope,"
  or similar invented exception anywhere in this report).

## Result

End the file with `REVIEW: PASS` or `REVIEW: FAIL`. The archive gate reads
this marker.
