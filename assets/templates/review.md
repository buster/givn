# Review: {{CHANGE_ID}}

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
<!-- paste grep/scan command and result, e.g.
"0 empty step bodies found across N step definition files"
or list each finding with file:line -->
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| | | |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| | | |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
<!-- paste reference to the proof-of-strictness evidence in tasks.md, or
re-run it now and paste the output -->
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a primary assertion on the observable output, return value,
acknowledgement, or visible state of the interface named in design.md, not
repository-only; for a browser-UI capability, the step definitions checked
for actually driving the browser (click/type/navigate), not an HTTP request or
in-page `fetch()`:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| | | | |

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action, not one per variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| | | | |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git
status`) was searched for any other/parallel e2e implementation of the same
capability that might be stronger than the one being reviewed:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| | | | |

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
<!-- paste local-run command + confirmation of clean startup -->
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`; if identical, this is always a finding
(the "e2e run" is not isolating `@e2e` scenarios at all). Prove isolation
with the per-scope scenario counts recorded in the archive verification
receipt (`givn/archive/<id>/verification.json`):

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| | | | | | |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | | |
| Capability ownership and typed relationships preserved | | |
| Confirmed Persona-relevant outcomes preserved | | |
| Actors remain distinct from Persona biographies | | |
| Migration ledger and coverage evidence complete, if applicable | | |

Record the persona disposition as a real `PERSONAS:` line outside any code
fence — a declared visual interface needs at least one confirmed Persona:

```
PERSONAS: <confirmed persona id>
```

A change without a user-visible surface records the reason instead:

```
PERSONAS: none — <reason + operator confirmation>
```

## Deferred mandatory checks

A mandatory check may be deferred only after an operator question. Record the
pair as real lines outside any code fence; a `DEFERRAL:` without a matching
`OPERATOR-DECISION:` fails the review. Omit both lines when nothing is
deferred.

```
DEFERRAL: <mandatory check>
OPERATOR-DECISION: <the operator's recorded decision>
```

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named. Any deviation implemented without first updating
design.md and re-running design-review is a finding, not an acceptable
shortcut:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| | | | | |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions. Every inventory entry must have a matrix row,
a matching `@e2e` scenario, and step definitions that use the promised
driving mechanism:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| | | | | | |

## Visual Review

N/A when the change's design declares no visual interface. Otherwise record
the contract, the evidence completeness, the baseline, and the persona lens
findings here.

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| | | | |

| confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| | | | |

**Baseline:** every control labeled, no internal identifier visible, a real
page title and heading, no duplicated visible content, a designed error state,
the configured language, reduced-motion honored.

VISUAL-REVIEW: PASS <!-- or VISUAL-REVIEW: UNVERIFIED until a human signs off -->

## Retirements

For every capability retired by this change (a delta tagged
`@givn.retired`), one line naming it and why:

RETIRE: <capability-id> — <reason>

N/A if this change retires nothing.

## Arc42 implementation conformance

**N/A when the arc42 addon is disabled.** When `addons.arc42` is enabled, this
is a mandatory comparison of the durable `docs/arc42/` chapters and the
change-level `arc42.md` assessment against `design.md`, `tasks.md`, and the
completed implementation. This is a fresh implementation review, not a copy
of the design-review table.

First derive the architectural facts and affected chapters from the completed
change and the current durable documentation. Then compare the change-level
Arc42 claims. Read each chapter named by either assessment and record concrete
evidence in the table:

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| | | | | | | |

- [ ] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [ ] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [ ] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [ ] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [ ] No mismatch remains. A mismatch was fixed in the affected documentation,
      design, tasks, or implementation and this comparison was repeated; a
      design change also triggered design-review again.

ARC42 CONFORMANCE: CLEAN / FINDINGS

**Result:** CLEAN / FINDINGS (if findings, list remediation taken — reopened
tasks, re-run RED/GREEN/REFACTOR/COMMIT, real-interface obstacle fixed per
design.md, or design.md corrected + design-review re-run — before
continuing this review).

---

## Coverage

Coverage is measured once by the archive against the committed candidate and
recorded in the verification receipt. Review does not run an instrumented
pass; classify gaps from the most recent measured evidence plus static
inspection, and say explicitly when no measured evidence exists yet.

**Coverage result:** <!-- paste line/branch percentages from the merged report recorded in givn/archive/<id>/verification.json and README, or state explicitly that no receipt exists yet. -->

```
<!-- paste the coverage summary from the archive receipt or the last
     published merged report. The archive receipt is the durable measured
     result; review classification uses it rather than re-running the suite. -->

## Coverage gap classification

Every uncovered region must be classified as exactly one of:
1. **Dead code** (YAGNI/KISS) → delete it.
2. **Missing test coverage** → fix step definitions, or add a scenario and run RED/GREEN/REFACTOR.
3. **Legitimately hard to test** — rare; requires concrete technical justification.

These three buckets are exhaustive — **no fourth bucket**. Do not excuse a
gap with invented language such as "acceptable for this iteration scope,"
"sufficient for current scope," or "future scope" — a missing or downgraded
`@e2e` scenario is always bucket 2 (missing test coverage).

Coverage is **complete** when every gap is classified and resolved.
There is no minimum percentage; a high number is a side effect, not the target.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| | | | |

---

## Scenario execution evidence

Review does not re-run the suite. Record the last archive verification receipt
for this change (or state that the change has not been archived yet):

```
<!-- paste the receipt's per-scope status and counts from
     givn/archive/<id>/verification.json, or state "unproven candidate — no
     receipt yet". Never paste a fresh manual full-suite run here: the
     archive is the single full-suite authority. -->
```

**Receipt status:** proven / unproven / not archived yet

## Overlap dispositions

For every shape-match finding involving this change's delta, one row.
Decision must be one of: `duplicate`, `variant`, `boundary`.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| | | |

## Split-or-keep

For every scenario longer than the lint threshold, one row: `split` or `keep`.

| Scenario | Decision |
|---|---|
| | |

## Sign-off

- [ ] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [ ] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [ ] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [ ] No `@wip` tags remain in the delta spec.
- [ ] The delta spec contains no implementation-layer detail.
- [ ] Local run command starts the full stack, including digital twins,
      cleanly.
- [ ] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [ ] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (or "all scenarios are @e2e" is stated).
- [ ] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [ ] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [ ] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces; no
      production-code struct/fn/enum that the step definitions rely on is
      omitted (or the section states "none" if the change has no new
      primitives).
- [ ] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md
      was mapped to a real `docs/arc42/<slug>.md` file per
      `src/integrity_hook.rs:check_arc42_conformance`.
- [ ] No finding anywhere in this report is excused with a classification
      outside the three buckets (no "acceptable for now," "future scope,"
      "sufficient for current scope," or similar invented exception).

**REVIEW: PASS** <!-- change to FAIL if any sign-off item is unchecked -->
