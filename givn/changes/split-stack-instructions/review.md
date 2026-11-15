# Review: split-stack-instructions

Review kind: independent
Author: omp hardening author
Reviewer context: fd3e0fd7-9ae1-4767-b2b2-dc6ddd46984f

## Scope and result

The change moves the per-runner step and runner mechanics out of `steps` and
`design` into six policy stack instructions, registers their ids, and adds an
instruction-id reference check over served guidance. The delta holds 14
declarations (13 `@givn.added`, one `@givn.modified`, none `@givn.removed`)
and 29 executed rows.

The implementation evidence is mostly sound: the six stack ids serve, the two
core instructions name each id with its load condition and carry no stack
body, the reference check resolves every shipped direction, and the review
guidance keeps its full obligation set. Two findings block sign-off:

1. the delta support fragment still states the rejected `ise` rule (the ise
   instruction cannot name the project's runner's stack instruction);
2. the `steps-go` body row does not discriminate its empty-body stub, because
   the stub cell token also appears in the strict-mode row.

The verdict is `REVIEW: FAIL`; both findings need the author.

## Step 0 implementation evidence check

### 0. `@e2e` tag integrity

No finding. The delta declares no `@e2e` scenario, and no `@e2e` tag was added
or removed:

```text
$ grep -rn '@e2e' givn/changes/split-stack-instructions/specs
fragments/instruction-integrity.feature:44  Then the served text carries "`@e2e` tag integrity"
fragments/instruction-integrity.feature:49  And the served text carries "second `@e2e` file"
```

Both matches are step text asserting review markers, not scenario tags. The
feature-tag diff between `e52b4667` and `HEAD` removes only `@wip`.

### 1. Empty or fabricated step bodies

0 empty step bodies found across the seven changed files
(`tests/common/instruction_references.rs`, `tests/common/mod.rs`,
`tests/steps/mod.rs`, `tests/steps/instruction_integrity_steps.rs`,
`tests/steps/instruction_integrity_e2e_steps.rs`,
`tests/steps/serve_extended_rules_steps.rs`,
`tests/steps/adr_qualification_gate_steps.rs`).

Every new `#[given]`/`#[when]`/`#[then]` body drives the built `givn` binary
through `run_givn`/`run_givn_at` and asserts a named sentence, an absence, a
refusal, a finding, or an exit status. The only `unimplemented!()`/`todo!()`
matches are the two string constants in `STUB_FORMS`
(`tests/steps/serve_extended_rules_steps.rs:1695-1696`), which are assertion
data, not bodies. The `return;` matches in `tests/common/mod.rs` and
`tests/steps/mod.rs` are early returns in helpers, not step stubs.

### 2. Commit evidence

Every checked scenario records a batch revision whose commit contains that
batch's regression test or production change. Spot checks:

```text
$ git show --stat 4507387f   # reference-check batch
tests/common/instruction_references.rs | 321 +++++
tests/common/mod.rs                    |  34 +-
tests/steps/mod.rs                     |   6 +
tests/steps/instruction_integrity_steps.rs | 430 +++++++++-
...
$ git show --stat 8f324c0c   # stack-id-registration batch
src/cli/instructions.rs                |   6 +
tests/steps/serve_extended_rules_steps.rs | ...
$ git show --stat cc72c251   # adr-register-count batch
tests/steps/adr_qualification_gate_steps.rs | 14 +-
```

| Batch | Revision | Batch files |
|---|---|---|
| `setup` | `03070b80` | `tasks.md` setup evidence |
| `design-corrigenda` | `508c76e8` | `design.md`, `design-review.md` |
| `reference-check` | `4507387f` | new module, common/steps wiring, integrity steps |
| `stack-load-conditions` | `eb5a9456` | `serve_extended_rules_steps.rs` |
| `stack-bodies` | `6280a1f2` | delta feature + `serve_extended_rules_steps.rs` |
| `core-absence` | `951cce63` | `serve_extended_rules_steps.rs` |
| `tasks-guidance` | `ce46d09a` | `serve_extended_rules_steps.rs` |
| `stack-id-registration` | `8f324c0c` | `src/cli/instructions.rs` + steps |
| `ise-mapping` | `2dab715d` | `instruction_integrity_steps.rs` |
| `review-obligations` | `43b21724` | `instruction_integrity_steps.rs` |
| `adr-register-count` | `cc72c251` | `adr_qualification_gate_steps.rs` |

`2a`: no `CHARACTERIZATION` marker; this is a normal change.
`2b`: discrimination probes are recorded under "Probes" below.

### 3. Promised components and contracts

Every component `design.md` names exists:

| Promised | Evidence |
|---|---|
| Six stack instruction assets | `assets/instructions/steps-{rust,python,node,java,go,ruby}.md` |
| Six ids in `POLICY_INSTRUCTION_IDS` | `src/cli/instructions.rs:20-25` |
| `tests/common/instruction_references.rs` | present, registered at `tests/common/mod.rs:14` |
| `named_instruction_ids` moved, one implementation | imported at `tests/steps/instruction_integrity_steps.rs:3`; no second extractor |
| Step definitions in the three named files | `serve_extended_rules_steps.rs`, `instruction_integrity_steps.rs`, `adr_qualification_gate_steps.rs` |

The served-guidance contract is unchanged: the request parameters, JSON
fields, and refusal text are the same; the id set grows. The reference-check
module has one consumer (the acceptance suite) and no consumer imports an
implementation it does not own. The composition root exception does not
apply because no production component was added.

### 4. Strict-mode proof

Present in `tasks.md` `## Setup`, with output:

```text
A: GIVN_FEATURES=target/strict-probe cargo test --locked --test features_runner
   -> 1 feature, 1 scenario (1 failed), 1 step (1 failed)
      "Step doesn't match any function"; exit 1
B: ... -- --exact "probe.feature::Probe strictness probe scenario"
   -> 1 scenario (1 failed); panic "1 step failed"; exit 101
```

`.fail_on_skipped()` is present at both registration points
(`tests/features_runner.rs:55,155`). Missing proof would invalidate every
PASS in this change; it is not missing.

### 5-7. E2E scope, driver, and parallel implementation

No finding. The delta adds no `@e2e` scenario, so no delta interaction was
downgraded. `verify.e2e_command` is `./run-tests.sh --e2e` from
`givn/commands.yaml`; it runs `tests/features_runner.rs` with the `@e2e` tag
scope. The `serve-extended-rules` permanent `@e2e` scenario's bindings live in
`tests/steps/serve_extended_rules_steps.rs` (`the served content carries the
built-in proposal rules`, `the agent requests the proposal instructions
through the Givn CLI`) and drive the compiled CLI as a subprocess.
`tests/steps/serve_extended_rules_e2e_steps.rs` is a five-line doc-only
placeholder with no alternate bindings, and the tree search found no second
implementation of the same capability. The delta's regular scenarios use the
same subprocess boundary; they assert absence, resolution, and marker text
that the `@e2e` rows do not.

### 8. E2E scope normalization

No finding. Requesting a stack instruction is the same consumer action as
requesting any other instruction id with another id (a variant), so the
normalized inventory is unchanged and no new `@e2e` scenario is required. The
permanent inventory rows remain:

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario |
|---|---|---|---|
| `specs/obtain-effective-guidance/usecase.md` | 1 | 1 | `An agent receives the extended rules through the Givn CLI` |
| `specs/fragments/fragment.md` (`instruction-integrity`) | 2 | 2 | `An agent requests machine-readable artifact guidance`; `Author starts a session with the universal dialog obligation` |

### 9. Explicit E2E scope selection

The two scopes are separate invocations (`--non-e2e` selects `not @e2e`,
`--e2e` selects `@e2e`), and `run-tests.sh` derives or honors
`GIVN_RESULT_SCOPE`. The change verification receipt records
`scope: change`, `total: 29`, `passed: 29`, `failed: 0`, `skipped: 0` at
`4eb829d9`. The tasks closeout records the permanent scopes at 1041 passed /
113 skipped (regular) and 113 passed / 1041 skipped (e2e); the skips are the
out-of-scope scenarios of the other tag scope, not in-scope skips. No delta
`@e2e` row exists, so the e2e scope for this change carries only permanent
rows.

### 10. Design match

| Decision (from design.md) | design.md says | Actually built | Matches? | If not |
|---|---|---|---|---|
| Verify command | `./run-tests.sh --non-e2e` | same | yes | |
| E2E command | `./run-tests.sh --e2e` | same | yes | |
| Single scenario | `GIVN_FEATURES=... cargo test --test features_runner -- --exact "<title>"` | same; every probe ran it | yes | |
| Step definition files | the three named files; module registered in `common/mod.rs`; no delta step copy | same; `find ... -name '*steps*'` in the delta prints nothing | yes | |
| Runner/framework | `cucumber-rs` 0.23.0 with `.fail_on_skipped()` | `Cargo.lock` 0.23.0; lines 55/155 | yes | |
| Reference-check extractor | `(line, line_text, id)`; policy id with `--change` is a finding; artifact id without the flag is not | `named_instruction_ids`, `named_change`, `unresolved_instruction_ids`; probe results below | yes | |
| Six-id registration | `POLICY_INSTRUCTION_IDS`, not the manifest | `src/cli/instructions.rs:20-25` | yes | |
| Twelve paired files | rejected; one file per stack named by both core instructions | six files; `steps` and `design` name the same ids | yes | |

No deviation was silently implemented.

### 11. Interaction coverage

| Inventory entry | Matrix row | `@e2e` scenario | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| `serve-extended-rules`: request effective instructions by id (artifact, policy, step, stack, addon-bound, challenge subject, JSON, change context) | present (design "Interaction Coverage Matrix") | `An agent receives the extended rules through the Givn CLI` | CLI subprocess, stdout asserted | `run_givn` with `givn instructions proposal ...`; stdout asserted | yes |
| `instruction-integrity`: resolve an instruction and expose the machine contract | present (permanent design; unchanged) | `An agent requests machine-readable artifact guidance` | CLI `--json`, stdout parsed | `run_givn` with `--json`; stdout parsed | yes |
| `instruction-integrity`: request normal session-start instructions | present (permanent design; unchanged) | `Author starts a session with the universal dialog obligation` | CLI, stdout asserted | `run_givn` with no id; stdout asserted | yes |

The delta adds no inventory entry; all three rows are permanent and exist.

### Probes (reviewer-run)

With the tree restored after each probe (`git status --short` empty):

| Probe | Command | Expected | Observed |
|---|---|---|---|
| Dangling id in an effective addon instruction | append `givn instructions steps-rustt` to `assets/addons/coverage/instructions.md`, run `Every instruction id a served instruction names resolves` | fails naming `steps-rustt` at `addons/coverage` | failed: `addons/coverage:82 — ... unknown artifact 'steps-rustt'` |
| Dangling id in a core instruction | append the same direction to `assets/instructions/steps.md` | fails naming `steps-rustt` at `steps` | failed: `steps:257 — ... unknown artifact 'steps-rustt'` |
| Policy id carrying `--change` | append `givn instructions dialog --change <id>` to `steps.md` | fails naming `dialog` and the ignored context | failed: ``givn instructions dialog`: ... names `--change instruction-integrity`, which the CLI ignores` |
| Artifact id without `--change` stays silent | run `A policy instruction named with a change context is a finding` (its second Given appends `givn instructions proposal`) | no finding for `proposal` | passed; `it reports no finding for "proposal"` |
| Load-condition mapping | swap `steps-go`'s runner to `cucumber-js` in `steps.md`, run the load-condition outline | the `steps-go` row fails | failed on ``the served text names "steps-go" ... "godog"`` |
| Stack-body assertion | delete the `Strict mode` row from `steps-go.md`, run the body outline | the row fails | failed on the missing backticked `godog.ErrPending` (the next assertion in the row) |
| Stub-row discrimination | delete `Unimplemented body: return godog.ErrPending` from `steps-go.md`, run the body outline | the row fails | **6/6 rows passed** — finding F2 |
| ADR register count | add a second accepted row to `docs/arc42/adr/README.md`, run both scopes | both fail `left: 30 right: 29` | delta: 30 vs 29; permanent: 30 vs 29 |
| Register baseline | restore the register; delta receives 29/29, permanent row green | passes | receipt `4eb829d9` 29/29; resume probe green |

### 12. Step 0 findings

- **F2 (bucket 2, missing test coverage).** The `steps-go` row of
  `Every stack instruction is served only on its own request with its runner
  body` does not assert the stub row. The Examples cell is
  `` `godog.ErrPending` `` (`serve-extended-rules.feature:83`), and the only
  backticked occurrence in `assets/instructions/steps-go.md` is the strict row
  (`:7`); the stub row is `` `return godog.ErrPending` `` (`:6`). Deleting the
  stub row leaves the row green. The fix is the Examples cell
  `` `return godog.ErrPending` `` (or a separate unique stub token), followed by
  a RED/GREEN run of the outline.
- No other Step 0 finding.

## Arc42 implementation conformance

The 12-row impact table was re-derived from `proposal.md`, the specs, and
`design.md` before reading `arc42.md`. Affected rows: 02, 04, 05, 08, 09, 10,
11, 12. Rows 01, 03, 06, and 07 were checked and stay unaffected (no new goal,
context surface, runtime flow, or deployment unit).

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `TC-48`: core instructions carry the universal rules and the id mapping; stack instructions are policy ids served on request; no length cap | `docs/arc42/02-architecture-constraints.md:57` | Yes; `TC-48` records the split and id contract | Overview 1-2; Decisions D1-D4; composition rule | `stack-load-conditions`, `stack-id-registration`, `core-absence` batches | served text of `steps`/`design`; `POLICY_INSTRUCTION_IDS`; body outline | yes |
| Guidance strategy row names on-demand stack instructions | `docs/arc42/04-solution-strategy.md:19` | Yes; the Guidance row records it | Overview; D1/D3 | `stack-load-conditions` | same served text | yes |
| `cli::instructions` policy id set includes the six stack ids | `docs/arc42/05-building-block-view.md:59` | Yes; responsibility names the set | Architecture Impact; D4 | `stack-id-registration` | `src/cli/instructions.rs:20-25`; refusal probe | yes |
| Acceptance support grades instruction ids with `instruction_references` and stack composition as text | `docs/arc42/05-building-block-view.md:65` | Yes; test-support row records it | Contract table; reference-check section | `reference-check` | `tests/common/instruction_references.rs`; delta scenarios | yes |
| Guidance composition rule and reference-check context/addon sources | `docs/arc42/08-crosscutting-concepts.md:1440-1483` | Yes; split rule, check, no-cap rule | "Instruction Composition Rule", "Instruction Reference Check", boundary cases | `reference-check`, `stack-bodies`, `core-absence` | probes above; delta assertions | yes |
| `ADR-077` accepted; register counts 29 | `docs/arc42/adr/README.md:64`, `docs/arc42/adr/ADR-077-stack-instructions.md` | Yes; `ADR-077` added, count 29, scenario `@givn.modified` | Decisions; ADR-077 paragraph | `adr-register-count` | register `grep -c '| accepted |'` = 29; both scopes' probe | yes |
| `Q-126`: maintainability boundary with served-text acceptance target | `docs/arc42/10-quality-requirements.md:122` | Yes; `Q-126` records the rule | composition rule; Decisions D2 | `stack-load-conditions`, `core-absence` | presence/absence assertions | yes |
| `R-148` and `D-29` retired; no size ceiling remains | `docs/arc42/11-risks-and-technical-debt.md:141` | Yes; names them retired | Operator decision; Risks | `core-absence` | `grep R-148\|D-29` finds only the retirement note; no cap scenario | yes |
| `Stack instruction` glossary row with genus, differentia, contrast | `docs/arc42/12-glossary.md` links `docs/glossary.md:93` | Yes; one row | Personas/term; Decisions | `stack-id-registration` | glossary row present; term reused in specs/design/code | yes |
| Rows 01/03/06/07 unaffected | chapters 01, 03, 06, 07 | No | no design claim | no batch claims | no flow, context, or deployment change | yes |

- [x] Every chapter marked affected by either the review or `arc42.md` was
      opened and checked for stale, contradictory, or placeholder content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains; no design change was needed during review.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`Stack instruction` is recorded at `docs/glossary.md:93` with the genus
(served instruction), differentia (`<id>-<stack>`, one runner's mechanics,
policy id without a change context), the contrast `Stack instruction, not
Stack recipe`, and one domain sentence. The specs, design, `ise.md`, and the
scenario titles reuse the term. The code symbol is the id set
(`POLICY_INSTRUCTION_IDS`), not a new noun.

Finding F1 (below) is a use-case-contract contradiction: the delta support
fragment says the ise instruction names the stack instruction for the
project's runner, while the use case, design D6, `ise.md`, and the delta
scenario state that it names `steps` as the mapping and `steps-rust` as one
worked example. That is one authoritative rule versus a second, false
statement, and it will merge into the permanent fragment.

## Structural model

`docs/domain-model.md` is unchanged and correct: the model covers domain units
and effective guidance, and it explicitly keeps instruction composition out
of the domain structure (`## Open modeling questions`: revisit if guidance
composition becomes independently mutable). This change moves served text and
adds a test-support check; it does not alter a domain unit or relationship.
No model edit is claimed.

## Coverage

No merged coverage receipt exists yet for this change. `givn/commands.yaml`
configures `./measure-coverage.sh` and `./merge-coverages.sh`; the archive
runs the instrumented scopes against the committed candidate and records the
merged result. Review classifies from static inspection as the instruction
requires.

Static classification:

| Region | Classification | Evidence |
|---|---|---|
| Six `POLICY_INSTRUCTION_IDS` entries (`src/cli/instructions.rs:20-25`) | covered; no gap | the refusal scenario asserts the known list contains `steps-rust`; the reference check serves all six ids |
| `tests/common/instruction_references.rs` | test support, outside the measured production population | exercised by the three `instruction-integrity` scenarios |
| Six stack asset files | guidance corpus, outside the code population | body outline asserts binding, stub, strict, command per stack |
| Redundant unit tests | none added | every new assertion drives the built binary through the black-box boundary |
| `steps-go` stub row | **bucket 2, missing test coverage** (F2) | probe: deleting the stub row leaves the body outline green |

Coverage is not complete: the F2 gap is classified and unresolved. No gap is
classified outside the three buckets, no branch instrumentation was removed,
and no line-only report is used.

## Visual Review

The design declares `Interface: terminal` (a served text surface; no rendered
screen). No delta `@e2e` scenario exists, so no transcript is required and no
visual evidence is missing. The terminal baseline was checked against the
served output:

| Baseline rule | Evidence | Result |
|---|---|---|
| A stack sentence names the id and the condition, never a bare id | `givn instructions steps`/`design` served text; load-condition probes | passes |
| A refusal names the requested id and lists the known ids | `givn instructions steps-rustt` refusal; scenario `An unknown stack instruction id is refused by name with the known list` | passes |
| No Givn implementation symbol appears in served guidance | `grep src/\|POLICY_INSTRUCTION_IDS\|run_artifact` over `assets/instructions/*.md`: only consumer paths (`src/` under application source, `src/test/java/...`) | passes |
| Text and JSON carry the same content | `The machine-readable stack path carries the same content as the text path` via `without_skill_route` | passes |

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| `agent--technical-role` | load only its runner's mechanics | none: the core names the id and condition, the stack id serves the body | accepted |
| `developer--primary-user` | read the rules without every stack's rows | none: the core text is complete for a change that touches no named stack | accepted |
| `givn-maintainer--owner` | one owner per rule; a dangling id fails by name | none on the served surface; the stale delta-fragment rule is F1, recorded under use-case conformance | F1 blocks; served surface passes |

PERSONAS: agent--technical-role

VISUAL-REVIEW: PASS

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `obtain-effective-guidance` main flow 1-6, rules, success guarantee; `support` fragment | no: F1 (fragment rule contradicts the use case and implementation) |
| Capability ownership and typed relationships preserved | `serve-extended-rules`, `instruction-integrity`, `adr-qualification-gate` ownership unchanged; delta adds no capability | yes |
| Confirmed Persona-relevant outcomes preserved | three Personas dispositioned above | yes |
| Actors remain distinct from Persona biographies | Gherkin actors are the authoring agent/developer; Personas are review lenses | yes |
| Migration ledger and coverage evidence complete, if applicable | register count 29; no data migration; coverage pending archive | yes |

The failure is F1: `specs/fragments/fragment.md:89` reads "The `ise`
instruction names the stack instruction for the project's runner and the two
record owners". The owning use case (`usecase.md:66`) states it names `steps`
as the mapping and one stack instruction id as a worked example; `design.md`
D6 rejects the runner-specific claim because a static addon instruction cannot
know the runner; `assets/instructions/ise.md:34-36` implements the mapping;
the delta scenario asserts the same. The sentence merges into the permanent
support fragment at archive.

## Deterministic gate dispositions

### Overlap dispositions

| Scenario A | Scenario B | Decision |
|---|---|---|
| `The machine-readable ise path carries the same content as the text path` | `The machine-readable stack path carries the same content as the text path` | variant: one subject is an addon-bound id resolved through the addon gate, the other a policy stack id resolved from the embedded asset; the resolution paths differ and both outcomes are asserted. The first is the permanent row, the second the delta row. |

The remaining `[SUBST]` advisories are generic-step subset matches with
permanent scenarios outside this capability; they are advisory discovery, not
shape matches, and no scenario is merged or removed.

### Split-or-keep

No delta scenario exceeds the 14-step threshold
(`LONG_SCENARIO_STEP_THRESHOLD`), so no split-or-keep row is required. The
three 6-row Outlines have short per-row bodies.

## README impact

README-IMPACT: none

The change adds no command, flag, configuration key, or README-documented
workflow. The policy-id list lives in `docs/reference.md`, which this change
updated; the README's `givn instructions <id>` sentence describes the generic
request, which is unchanged.

## Deferred mandatory checks

None. No mandatory check was deferred, and no budget or size check was
withdrawn into an exception here; the operator decision of 2026-11-15 is
recorded in `design.md` and `design-review.md`.

## Findings

### F1 — the delta support fragment restates the rejected ise rule (blocker)

- Location: `givn/changes/split-stack-instructions/specs/fragments/fragment.md:89`.
- Evidence: the sentence claims the ise instruction names the stack
  instruction for the project's runner; `usecase.md:66`,
  `design.md:198-206` (D6), `design-review.md` decision 3, `assets/instructions/ise.md:34-36`,
  and `instruction-integrity.feature:38-46` all state `steps` plus one worked
  example. The sentence is not covered by a scenario and will merge into
  `givn/specs/fragments/fragment.md`.
- Required action: correct the sentence to the mapping-plus-worked-example
  rule in a spec batch with its own commit, then rerun the affected delta and
  permanent scenarios. This is a spec edit; the reviewer does not make it.

### F2 — the steps-go body row does not assert its stub (blocker, bucket 2)

- Location:
  `givn/changes/split-stack-instructions/specs/obtain-effective-guidance/serve-extended-rules.feature:83`
  (stub cell `` `godog.ErrPending` ``) against
  `assets/instructions/steps-go.md:6-7`.
- Evidence: the strict row contains the only backticked
  `` `godog.ErrPending` ``; the stub row is `` `return godog.ErrPending` ``.
  Deleting the stub row left all 6 rows green; deleting the strict row failed
  on the stub assertion, proving the assertion was satisfied by the strict
  row.
- Required action: change the Examples stub cell to
  `` `return godog.ErrPending` `` (or another stub-only token), rerun the
  outline RED before the change and GREEN after, and record the commit on the
  `stack-bodies` scenario.

### M1 — marker `no fourth bucket` also matches a section heading (minor, no repair)

- Location: `assets/instructions/review.md:349` (heading
  `## Classification (three buckets — no fourth bucket)`) and `:499`
  (sign-off bullet).
- Effect: the marker would still pass if only the sign-off bullet were
  deleted. The obligation itself remains at its canonical statement in the
  classification section, so no review obligation is lost; the scenario is a
  presence check, not a per-bullet discriminator. No artifact edit is required
  for this change.

## Review gate

```text
$ givn check review --change split-stack-instructions
  ✓ run declaration passed
  [SHAPE] advisory ... 'The machine-readable ise path carries the same content as the text path'
          and 'The machine-readable stack path carries the same content as the text path' share a shape
  givn lint: 3 file(s) checked — clean
  net delta: 13 added, 1 modified, 0 removed
  ✓ overlap dispositions passed
  ✓ scenario commits passed (14 scenario(s))
  ✓ change verification receipt reused across documentation-only revision fb0520db (tested at 4eb829d9, 29 scenario(s))
error: review checks failed:
  integrity checks failed:
  review.md actual verdict is FAIL
exit 1
```

The only gate failure is the recorded verdict. The author's fixes for F1 and
F2 are followed by a re-review that flips the verdict.

## Sign-off

- [ ] Fabrication audit clean: step bodies and commits verified, all promised
      components exist, no downgraded `@e2e`, no delta `@e2e` driven by
      HTTP/fetch — **F2 blocks** (the `steps-go` stub row is not
      discriminated).
- [x] Every checked task has a verified commit touching per-batch production
      code or its regression test.
- [x] Every consumer uses only its declared contract.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded in the change receipt; review did
      not rerun the suite.
- [ ] Coverage classification complete: **F2 is an unresolved bucket-2 gap**;
      the archive receipt will carry the measured result.
- [x] Dead code deleted / none added. Missing tests added except F2.
      Hard-to-test gaps: none claimed.
- [x] Redundant unit tests removed / none added.
- [x] No `@wip` tags remain. No implementation-layer detail in the delta spec.
- [x] Canonical E2E policy read from the resolved specs instruction; no
      inventory action added; variants stay regular scenarios.
- [x] Every E2E scenario has a real-interface primary assertion; no browser UI
      was replaced by HTTP/fetch.
- [x] The local run command starts cleanly: `cargo run -- instructions`,
      `... instructions steps`, and `... instructions steps-rust` exit 0.
- [x] The exact `verify.e2e_command` target was identified; the tree was
      searched for a parallel e2e implementation.
- [x] The E2E scope is selected explicitly (`--e2e`/`--non-e2e` plus
      `GIVN_RESULT_SCOPE`); required executions report zero failures and
      in-scope skips.
- [x] Implementation matches design.md's commands, file layout, and
      framework/driver.
- [x] Interaction coverage maps every inventory entry to a matrix row, an
      existing `@e2e` scenario, and the promised driver.
- [x] No finding is excused with a classification outside the three buckets.
- [ ] Use-case conformance: **F1 remains** (the delta fragment contradicts the
      use case and implementation).
- [x] Arc42 conformance: clean.
- [x] README impact decision recorded: `none`.
- [x] No deferrals.

REVIEW: FAIL
