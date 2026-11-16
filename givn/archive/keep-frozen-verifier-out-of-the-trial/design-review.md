# Design Review: keep-frozen-verifier-out-of-the-trial

Review kind: independent
Author: keep-frozen-verifier-out-of-the-trial author (omp)
Reviewer context: fd3e0fd7-9ae1-4767-b2b2-dc6ddd46984f (the review context that produced the grill brief)

The proposal records `Review kind: independent` and its reason. The reviewer
context produced the ranked question list and its recommended answers; this
authoring context applied the hardening edits and did not re-decide the
review's questions. The reviewer received the artifacts below, not the author's
conversation history.

## Grilling record

Every branch was walked. Each entry names the question, the resolution, the
rejected alternatives, and where the resolution landed.

### Scope

**Q: Does the spec contain exactly what the proposal asked for?** Yes, after
narrowing. The proposal's six observable behaviors map one-to-one to the delta
scenarios: the absent package (`clean-room-preparation.feature:5-19`), the
record field (`:21-29`), the empty case (`:31-37`), the declaration exemption
(`:39-46`), the dry run (`:48-55`), and the refusal plus its compatibility and
boundary cases (`trial-input-verification.feature:5-43`). One behavior was
missing: the verifier-side declaration exemption had no scenario, so a trial
prepared under `e2e: ["givn/**"]` would have been refused by the new test
without any failing scenario. The grill's decision 1 requires the exemption, so
the missing scenario was added at `trial-input-verification.feature:38-43`.
**Resolution:** add it as a regular scenario; the action (re-check) already has
one `@e2e` scenario, so this is a variant, not a second inventory entry.
**Rejected:** verifying the exemption in-process through `require_frozen_absent`
(it is not the consumer interface); leaving the exemption untested (the defect
that decision 1 fixes would then be invisible).

**Q: Is "done" unambiguous?** Yes. The frozen set is defined once
(`design.md:164-195`), the four filters and their order are
(`design.md:339-368`), the count basis is stated per artifact
(`design.md:184-193`), and the refusal's exempt set is stated
(`design.md:316-323`).

### Tech choices

**Q: Is the stack right?** Yes. No new language, framework, dependency, or
version; the change uses the existing `cucumber-rs` runner, the existing
`E2eBoundary`, and the existing record. Version freshness: the design names no
new version (`design.md:246-248`), and the reviewer confirmed no lockfile or
toolchain change is implied.

**Q: Is there a simpler alternative?** Two were tested and rejected by the
review: a verification-only rule (a trial prepared by another path stays
unchecked) and a preparation-only rule (a trial that gains the package after
preparation is verified clean). Both are recorded as rejected options in D5
(`design.md:312-315`). The chosen split uses the writer's existing partition and
the verifier's existing declaration load, so it adds one filter set, one accessor,
one record field, one report line, one traversal, and one contract method.

### Missing scenarios

- **Frozen path planted after preparation** — covered by
  `trial-input-verification.feature:5-11` and `:21-27`.
- **Whole package copied in, bounded refusal** — covered by `:21-27`.
- **Trial prepared before the rule** — covered by `:29-36`.
- **Declaration covered by the resolved package** — added at `:38-43`; see the
  Scope finding.
- **The `--path` refusal** — no new scenario. The `e2e` role remains a declared
  retained role, so refusal 7 fires through the same code path the permanent
  scenario `Preparation refuses a path matched by a retained role`
  (`givn/specs/prepare-an-ise-test/clean-room-preparation.feature:93-96`) already
  proves. The use-case example that claimed `Replaced: 0` was corrected to the
  refusal (`usecase.md:107`).
- **An `e2e: []` project** — covered by `clean-room-preparation.feature:31-37`.
- **`excluded_roles` removed from a record** — covered by `:29-36` and recorded
  as a residual risk, not a new scenario; the field is optional by design.

### Testability

**Q: Can every scenario fail in RED?** Yes, and the mechanism is named.
`cucumber-rs` binds the delta's phrases; `.fail_on_skipped()` on both builders
(`design.md:517-528`) makes an undefined or pending step fail the run, and the
design names the not-implemented pattern (`todo!`) and the strict-mode probe.
The single-scenario commands for both the `@e2e` preparation scenario and the
verification refusal are stated (`design.md:499-514`). Each Then asserts a
concrete value: a stdout line, a receipt key, a trial path's presence or
absence, a stderr refusal, a count, or an exit code. No scenario asserts "it
ran".

**Q: Is the unit-test surface justified?** The four partition filters are also
exercised by the CLI scenarios; the retained unit tests cover the filter order
when one path is both frozen and under a source root, which a fixture project
cannot distinguish per filter (`design.md:544-556`).

### E2E fidelity

The spec instruction's normalized interaction scope, real driver, and primary
assertion rules are applied without redefinition (`design.md:530-543`):

| Rule | Applied |
|---|---|
| One `@e2e` scenario per distinct action | Preparation keeps one; verification keeps one, unchanged |
| Real interface | Compiled binary as a subprocess (`ise replace`, `ise verify`), asserting stdout, stderr, exit code, and the written tree |
| Primary assertion | The report line and the trial content for preparation; the clean report and the refusal for verification |
| Variants stay regular | The bounded refusal, the older-record compatibility case, and the new declaration-coverage case |
| No tag removal | No `@e2e` tag was removed; the change modifies one existing `@e2e` scenario and adds regular ones |

Finding: the design claimed a transcript for a non-`@e2e` scenario and claimed
two modified `@e2e` scenarios. Corrected at `design.md:71-78` and
`design.md:537-543`: only the preparation `@e2e` scenario commits a transcript,
and only one existing `@e2e` scenario is modified.

### Interaction Coverage

The inventory lives in the owning `usecase.md` §Interactions
(`usecase.md:159-164`) and both entries have a matrix row
(`design.md:532-535`). No entry is added, removed, or re-scoped. The new
verification scenario is a variant of the re-check action, so it adds no row.

### Risk

**Q: Most likely failure mode?** The builder loses its only local acceptance
signal, so a structural mismatch surfaces at the verifier. Recorded as risk 1
and `R-154`, with the component's own scenarios as the local feedback and the
operator-visible line and record field as the explanation (`design.md:637`,
`docs/arc42/11-risks-and-technical-debt.md:140`). The grill's decision 6 added
the contradiction path: the builder does not adjust the suite; the difference is
recorded as a `Spec gap` and the trial restarts after the Specification or the
suite is corrected (`usecase.md:73`; risk 1 and `R-154`).

### Use-case and Persona context

The use case keeps its goal, main flow, extensions, rules, guarantees,
capabilities, typed relationships (`Includes`/`Extends` = none), and Persona
IDs. The scope rejection criteria were applied: the change stays inside
`prepare-an-ise-test`; it invents no Persona and uses no biography as a Gherkin
actor; the actors remain `Developer` and `Independent verifier`. Decision 3's
`--path` refusal keeps the use case consistent with refusal 7 rather than
inventing a second refusal site.

Persona disposition:

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Reads the frozen line, hands the trial over | `design.md:80-83`, `usecase.md:148-152` |
| `agent--technical-role` | Rebuilds from the Specification; loses local suite access | same |
| `givn-maintainer--owner` | Owns the frozen-boundary rule and record shape | same |

### Decision ledger

| ID | Resolution | Landing |
|---|---|---|
| D1 | Nothing in place of the package; the declaration, report, and record state the exclusion | `design.md:253-265` |
| D2 | Keep the checkout comparison and its per-path reasons | `design.md:267-277` |
| D3 | One conditional line; the count is the frozen set, not `e2e_files.len()` | `design.md:279-289` |
| D4 | Add `excluded_roles`, keep `ise-trial/v1` | `design.md:291-304` |
| D5 | Writer exclusion plus a verifier refusal that exempts the declaration and recorded retained paths | `design.md:306-325` |
| D6 | Component scenarios as local feedback; the loss recorded as `R-154` | `design.md:326-337` |

All six are resolved; none is carried; no proposal question ID is left
unrecorded. The proposal records no ideation seed, so no seed reference applies.

### Contracts and components

`CleanRoomTrial` has one consumer use case, one consumer (`crate::cli::ise`), and
one implementing component (`crate::ise`); the change adds no second contract or
adapter inside it (`design.md:143`). `E2eBoundary` gains the containment rule on
its existing contract (`design.md:142-151`): `contains` is a method on the
boundary, the only new consumer is `crate::ise`, and the review explicitly
rejected re-exporting the private `is_under` helper. Consumers import the
contract; the implementation stays inside `crate::e2e_boundary`; no composition
root change is needed.

### Decision timing

Every scenario depends on the resolved answer, so all six are foundational and
answered now. No carried decision remains, so no default/trigger/switch-cost/owner
row is owed. No speculative abstraction is built for an out-of-scope question:
the proposal's out-of-scope list (the tasks-side boundary check, the `behavior`
role, the archive guard, the delivery block, the nine refusals, and the
replacement build) has no design work behind it.

### Visual design contract

`Interface: terminal` with viewports recorded as not applicable
(`design.md:58-78`). The opposite classification was tested: no scenario loads
HTML, CSS, or JavaScript, and no screen is rendered, so `web` does not hold. No
new screen or visible redesign is introduced; one terminal report line and one
stderr refusal are added. Motion and reduced-motion do not apply. The capture
mechanism is the terminal transcript for the one `@e2e` scenario, at the path
recorded in the contract.

### Deferrals

None. No mandatory check is deferred, and no `@e2e` tag is removed.

### ADR qualification

The assessment is recorded once in `arc42.md:20-40`: architectural commitment,
real alternatives, durable consequence, and existing-decision search, with the
cheaper home named (`prepare-an-ise-test` rules plus chapter 02 own the behavior
and constraint; the ADR owns the rejected alternatives and the compatibility
consequence). ADR-078 exists with the MADR checklist and the qualification
verdict. The review confirms the recorded result rather than re-running the
qualification.

### Architecture documentation (arc42)

The reviewer re-derived the chapter set from the proposal, specs, and design
before opening `arc42.md`. The derivation and the recorded claim now agree:

| # | Chapter | Re-derived | `arc42.md` |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No |
| 2 | 02 architecture-constraints | Yes — TC-47 and TC-51 | Yes |
| 3 | 03 context-and-scope | Yes — the two interface rows | Yes (was No) |
| 4 | 04 solution-strategy | No | No |
| 5 | 05 building-block-view | Yes — `ise`, `cli::ise`, `e2e_boundary` | Yes |
| 6 | 06 runtime-view | Yes — replace, dry run, report, verify | Yes |
| 7 | 07 deployment-view | No | No |
| 8 | 08 crosscutting-concepts | Yes — §8.29 record facts and the frozen-package section | Yes |
| 9 | 09 architecture-decisions | Yes — ADR-078 and the register count | Yes |
| 10 | 10 quality-requirements | No | No |
| 11 | 11 risks-and-technical-debt | Yes — `R-154` | Yes |
| 12 | 12 glossary | Yes — the frozen report line and record basis | Yes |

Chapter 3 was the omission: `arc42.md` marked it No while both `givn ise`
interface rows change. `arc42.md:9` now marks it Yes with the reason, and
`docs/arc42/03-context-and-scope.md:62-63` carries the content. `design.md`
also omitted `docs/reference.md`; the Architecture Impact table names it
(`design.md:135-137`).

For every Yes chapter the reviewer opened the file and matched it against the
design: 02 (`:55`, `:60`), 03 (`:62-63`), 05 (`:62-63`, `:91`), 06
(`:1048-1055`, `:1106-1112`), 08 (`:1414`, `:1422-1428`, `:1460-1466`,
`:1494-1500`), 09 (register row at `docs/arc42/adr/README.md:65`), 11 (`:140`),
12 (pointer to the standalone glossary). No placeholder or stale contradiction
remains in the Yes chapters; the 02/08/06 texts were corrected to state the
exempt set and the count basis.

Chapter-file integrity: all twelve files exist, all carry content beyond a
scaffold, and the box-drawing and `+---`/`|---` scans returned no match, so no
ASCII-art diagram violates the Mermaid policy.

Chapter 09 check: ADR-078 is a new MADR entry linked from the register; it is not
an amendment to ADR-076, so no row was added to §"Current Amendments". ADR-076
links ADR-078 in its relationships and chapter 02/08/11 carry the consequences,
including the "Bad, because" residual for a record whose field is deleted.

### Ubiquitous language

`docs/glossary.md` already carries `Frozen package`, `Trial record`, `Trial
directory`, and `Trial E2E digest`. The review found one inconsistency: the
`Frozen package` entry claimed the frozen set is recorded as `e2e_files`, while
`e2e_files` is the resolved package and may include the declaration. The entry
now states the resolved-set relation and the report's frozen-only count
(`docs/glossary.md:78`). `Trial record` (`:74`) and `Trial directory` (`:77`)
already state the `excluded_roles` and absence facts and needed no hardening
edit. No new
term is introduced; `Frozen package` keeps its single meaning and its anti-terms.

### Structural model

`docs/domain-model.md` was assessed and needs no edit: its stated scope excludes
"internal Rust components, command parsing, or file formats", and the trial
directory and record are command and file-format artifacts. The change moves no
aggregate, bounded context, or relationship; it moves one artifact's content
rule, which the use case, glossary, and ADR own. A no-op is recorded here
explicitly rather than presented as maintenance.

## Decisions and blockers applied

### Decisions

1. **Exempt the declaration and recorded retained paths.** Landed at
   `design.md:29-36` (overview), `:110-117` (traceability), `:316-323` (D5),
   `:397-405` (verify order), `:419-421` (refusal note), `:431-434` (boundary
   rows); verifier side in `usecase.md:93` and `docs/arc42/02:55`. The missing
   scenario is `trial-input-verification.feature:38-43`.
2. **The `Frozen package` line counts the frozen set.** Landed at
   `design.md:184-193` (per-artifact table), `:219-234` (report), `:279-289`
   (D3), `:355` (partition); `usecase.md:64`, `:88`;
   `docs/arc42/02:60` (TC-51); `docs/arc42/08:1462-1464`; the feature keeps
   `(2 files)` at `clean-room-preparation.feature:13` and `(1 file)` at `:44`,
   both counting frozen paths.
3. **Keep the `--path` refusal.** Landed at `design.md:345-348` (the filter
   changes only `--full`; `--path` refuses at step 3) and
   `usecase.md:107` (the example now shows the refusal and names the `e2e`
   role).
4. **Update reference and chapter 03.** Landed at
   `docs/reference.md:357-360`, `:365-383`, `:405-431`, `:454-472`;
   `docs/arc42/03-context-and-scope.md:62-63`;
   `design.md:135-137`; `arc42.md:9` (chapter 03 now Yes).
5. **Recompute the passuff example.** 32 retained with 12 frozen leaves 20
   retained after exclusion. Landed at `usecase.md:102` (`Retained: 20`,
   `Frozen package: e2e, not copied (12 files)`, with the arithmetic stated)
   and `docs/reference.md:368,375` (same numbers).
6. **State the builder's contradiction path.** Landed at `usecase.md:73`,
   `design.md:637` (risk 1), and
   `docs/arc42/11-risks-and-technical-debt.md:140` (`R-154`).
7. **Name the containment function in the contract.** Landed at
   `design.md:142-151` (`contains` is a method on `E2eBoundary`; no helper
   re-export) and `docs/arc42/05-building-block-view.md:91`.

### Blockers

1. **ADR register red.** The register counts 30 accepted records
   (`docs/arc42/adr/README.md:30`); the permanent scenario and its binding
   asserted 29. Fixed in the `c6b9eb6b` shape: a delta `@givn.modified`
   scenario at `givn/changes/keep-frozen-verifier-out-of-the-trial/specs/fragments/adr-qualification-gate.feature:5-17`,
   the permanent scenario moved to 30
   (`givn/specs/fragments/adr-qualification-gate.feature:123`), and the binding
   moved to 30 with the ADR-078 row assertion
   (`tests/steps/adr_qualification_gate_steps.rs:881-902`). Both the permanent
   and the delta scenario ran green.
2. **The refusal contradicts the permanent contract.** `usecase.md:63` and
   `TC-51` hold; `TC-47` now states the exempt set
   (`docs/arc42/02-architecture-constraints.md:55`). The checkout-root test
   still covers any other path.
3. **Frozen count inconsistency.** Closed by decision 2 with the per-artifact
   table (`design.md:184-193`): the report carries the frozen set's size, the
   use case states the same, and the record keeps one identity per resolved
   package path.
4. **Step Definitions table incomplete and partly false.** Replaced with the
   exact-phrase table (`design.md:450-480`): the changed phrases at
   `clean-room-preparation.feature:12-13,44,46` and
   `trial-input-verification.feature:35` are present, the false
   `the trial holds "<path>"` claim is replaced by a binding created in the
   owning batch, and the positive `Frozen package` binding is listed. The
   missing-binding rule is stated at `design.md:483-487`.
5. **A permanent sentence was removed.** Restored at `usecase.md:68`: the
   declaration is retained on the same terms, so a `source:` root that covers it
   replaces nothing.
6. **Risk row contradicts ADR-078.** Corrected at `design.md:638`: the field is
   optional, its absence declares the older trial content, and the refusal is a
   record property; the same correction appears in the boundary table
   (`design.md:431`) and in ADR-078's consequences.
7. **Interface and transcript.** Restated at `design.md:71-78` (only the
   `@e2e` preparation scenario commits a transcript) and `design.md:537-543`
   (one existing `@e2e` scenario modified, not two).
8. **Grep sweep.** `grep -rn 'eleven lines\|e2e_files.len()\|Replaced: 0\|29 accepted'
   givn/changes/keep-frozen-verifier-out-of-the-trial docs/reference.md docs/arc42
   docs/glossary.md` returns only intentional text: the three "not
   `e2e_files.len()`" correction statements (`design.md:228`, `:355`,
   `docs/arc42/02:60`, `docs/arc42/08:1427`), the fixture-scoped equality note
   (`design.md:460`), and the typo-example `Replaced: 0`
   (`usecase.md:121`). No `eleven lines` or `29 accepted` occurrence remains.

## Verification

- `givn lint --change keep-frozen-verifier-out-of-the-trial`: exit 2, ten
  findings, all `@wip` markers on not-yet-implemented delta scenarios plus
  shape/subset advisories. No error-severity finding. The new declaration-
  coverage scenario is flagged as a shape/subset neighbour of the other clean
  re-checks; it stays separate because it proves the exemption rather than a
  generic clean run.
- `givn status --change keep-frozen-verifier-out-of-the-trial`: proposal, specs,
  design, and arc42-docs complete; next required is design-review.
- `./run-tests.sh --non-e2e`: see the transcript below.
- ADR reconciliation: the permanent scenario
  (`fragments/adr-qualification-gate.feature::The migration preserves
  independent architecture history across three proposed decisions`) passes with
  30 accepted decisions, and the delta scenario at the change path passes with
  the same binding.

```text
cargo fmt --all -- --check                 (clean, no output)
cargo clippy --locked --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.74s
cargo nextest run --no-tests=warn --locked --test features_runner
     Summary [ 195.475s] 1075 tests run: 1075 passed, 114 skipped
EXIT: 0
```

## Residuals

- The record's `excluded_roles` is optional, so deleting it disables the
  trial-side refusal for that record. Accepted in ADR-078 and recorded as risk 2.
- The delta `usecase.md` base digest must be re-derived if
  `add-delivery-identity-to-trial-record` archives first; both changes replace
  the same permanent document. Stated in `design.md:96-103` for the tasks author.

DESIGN-REVIEW: PASS
