# Review: adr-qualification-gate

## Fabrication Audit

This is a documentation and source-asset change. For this change, production
source includes the embedded guidance assets and durable Arc42 documentation.
No executable Rust runtime behavior was added. Scenario commits that reuse an
already-established canonical source touch real assertion bindings and are not
spec-only or empty-step commits.

**Step definition scan** — the two capability-specific files were scanned for
empty bodies, `unimplemented!()`, `todo!()`, bare returns, and no-op steps:

```text
rg -n -U '^\s*(pub\s+)?fn\s+[^\{]+\{\s*\}|unimplemented!\(\)|todo!\(\)|\breturn;\s*\}' \
  tests/steps/adr_qualification_gate_steps.rs \
  tests/steps/adr_qualification_gate_e2e_steps.rs
0 matches
```

Result: 0 empty or unimplemented step bodies found across 2 new step-definition
files. All new regular steps assert source content or durable register state.
Both E2E steps invoke the compiled Givn subprocess and assert captured CLI
output.

**Commit cross-check**

| Scenario | Commit hash | Authoritative source or real binding touched? |
|---|---|---|
| The guidance defines strict qualification as an all-gates contract | `ca6a15a` | Yes — canonical guidance, durable docs, migration, and real bindings |
| Shared guidance alone does not qualify an ADR | `7b2ce3c` | Yes — canonical guidance and real bindings |
| The guidance routes normal design decisions to one canonical artifact | `daa8580` | Yes — canonical guidance and real bindings |
| The guidance requires an existing-ADR check before drafting | `9ed460b` | Yes — canonical guidance and real bindings |
| The structured verdict has fixed gate and route fields | `52f684a` | Yes — real source assertions against the canonical verdict contract; source contract was already established in `ca6a15a` |
| Route targets are conditional and unused targets stay empty | `35b1d33` | Yes — real source assertions against the canonical route contract |
| The guidance forbids invented ADR evidence | `fc404cf` | Yes — real source assertions against the canonical evidence contract |
| An ADR refinement is routed to amendment without duplication | `5419d56` | Yes — canonical guidance and real bindings |
| An architectural replacement preserves supersession history | `556d2d1` | Yes — canonical guidance and real bindings |
| Arc42 impact does not by itself require an ADR | `1cf033b` | Yes — canonical guidance and real bindings |
| The change assessment records ADR disposition separately from chapter impact | `a66c58a` | Yes — Arc42 impact template and real bindings |
| The ADR register has one canonical ownership contract | `d9aa0aa` | Yes — durable register/chapter and real bindings |
| The migration removes non-ADRs and preserves independent architecture history | `c38c8e5` | Yes — real migration assertions; durable migration was established in `ca6a15a` |
| All effective guidance sources use the same ADR policy | `89c63ff` | Yes — real source-parity bindings against generated snapshots |
| The ADR guidance requires an adversarial qualification record | `c4ffe4a` | Yes — canonical guidance and real bindings |
| An author requests the strict ADR guidance through the Givn CLI | `a3776ed` | Yes — compiled-CLI E2E binding |
| A reviewer requests design-review guidance through the Givn CLI | `aa44a51` | Yes — compiled-CLI E2E binding |
|
| **No new executable production code** | — | Intentional; the behavior under test is authoritative documentation/source assets. |

**Promised-architecture cross-check**

| Component or source from `design.md` | Exists in tree? | Path |
|---|---|---|
| Canonical Arc42 qualification procedure and MADR template | Yes | `assets/addons/arc42-docs/instructions.md` |
| Change-level Arc42 impact template | Yes | `assets/addons/arc42-docs/template.md` |
| Effective design-review instruction | Yes | `assets/instructions/design-review.md` |
| Design-review command and template | Yes | `assets/commands/givn-design-review.md`, `assets/templates/design-review.md` |
| Design-review skill source and generated mirrors | Yes | `assets/skills/givn-design-review/SKILL.md.tmpl`, `.opencode/skills/givn-design-review/SKILL.md`, `.agents/skills/givn-design-review/SKILL.md`, `.claude/skills/givn-design-review/SKILL.md` |
| Consolidated ADR register and retained records | Yes | `docs/arc42/adr/README.md`, `docs/arc42/adr/ADR-*.md` |
| Regular step definitions | Yes | `tests/steps/adr_qualification_gate_steps.rs` |
| E2E step definitions | Yes | `tests/steps/adr_qualification_gate_e2e_steps.rs` |

**Strict-mode proof**

The first RED run is recorded in `tasks.md`: the undefined first step produced
`1 feature`, `1 scenario (1 failed)`, `1 step (1 failed)`, followed by a
non-zero process exit. The later stub proof produced
`Step panicked. Captured output: not implemented` and a non-zero exit. The
runner is `GivnWorld::cucumber().fail_on_skipped()`.

**Downgraded E2E scenario scan**

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| An author requests the strict ADR guidance through the Givn CLI | Yes — compiled CLI stdout and exit status | N/A — CLI | No |
| A reviewer requests design-review guidance through the Givn CLI | Yes — compiled CLI stdout and exit status | N/A — CLI | No |

**E2E scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---:|---|---|
| ADR qualification guidance | 2 | Yes — request Arc42 ADR guidance and request design-review guidance are separate CLI actions | No |

**Duplicate-implementation check**

| Capability | File(s) `verify.e2e_command` invokes | Other implementation found? | Stronger one used instead? |
|---|---|---|---|
| ADR qualification guidance | `./run-tests.sh --e2e` -> `tests/features_runner.rs`, with `tests/steps/adr_qualification_gate_e2e_steps.rs` registered in `tests/steps/mod.rs` | No parallel capability-specific E2E implementation; existing `instruction_integrity_e2e_steps.rs` is a different capability and uses a different fixture | No alternative existed |

**Local environment check**

The local run command is `./run-tests.sh`. It starts no server, database,
queue, browser, external service, or digital twin for this CLI/documentation
capability. The configured change scope completed without additional startup.

**E2E isolation check**

| `verify.command` value | `verify.e2e_command` value | Identical? | Full change-suite count | E2e-only count | Strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 17 scenarios, 17 passed | 2 scenarios, 2 passed | Yes |

The full repository coverage run additionally exercised 335 non-E2E and 61 E2E
permanent scenarios. The change-scope counts above prove the configured tag
filter is a strict subset.

**Design.md conformance check**

| Decision | `design.md` says | Actually built | Matches? |
|---|---|---|---|
| Runner | `./run-tests.sh`, Cucumber runner over `GIVN_FEATURES` | `./run-tests.sh`; `tests/features_runner.rs` | Yes |
| Strict mode | `fail_on_skipped()` and `unimplemented!()` for RED | `tests/features_runner.rs`, capability bindings | Yes |
| Regular bindings | One capability file | `tests/steps/adr_qualification_gate_steps.rs` | Yes |
| E2E bindings | Separate capability file | `tests/steps/adr_qualification_gate_e2e_steps.rs` | Yes |
| E2E interface | Compiled Givn CLI subprocess | `CARGO_BIN_EXE_givn` through existing `TestEnv` helper | Yes |
| E2E full command | `./run-tests.sh --e2e` with tag filtering | `givn/commands.yaml` and `run-tests.sh` | Yes |
| Single-scenario commands | Anchored Cucumber `--name '^...$'` | All targeted RED/GREEN/REFACTOR runs | Yes |
| Runtime evaluator | None; verdict is guidance-only | No evaluator or runtime schema added | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| request the Arc42 ADR guidance through the Givn CLI | Yes | An author requests the strict ADR guidance through the Givn CLI | Compiled `givn instructions arc42-docs --change adr-qualification-gate` subprocess | `run_givn` with that exact argument vector | Yes |
| request the design-review guidance through the Givn CLI | Yes | A reviewer requests design-review guidance through the Givn CLI | Compiled `givn instructions design-review --change adr-qualification-gate` subprocess | `run_givn` with that exact argument vector | Yes |

## Arc42 implementation conformance

The Arc42 addon is enabled. The independent implementation review confirms the
eight affected rows in `arc42.md` and the four unaffected rows. The affected
chapters contain current content and the register contains 35 accepted ADRs
plus proposed `ADR-063`.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Documentation-quality goal | `docs/arc42/01-introduction-and-goals.md` | Row 1 Yes | Documentation contract and single-source goal | Scenario: The ADR register has one canonical ownership contract | Quality goal row 4 and secondary goal text | Match |
| Authoring constraints | `docs/arc42/02-architecture-constraints.md` | Row 2 Yes | Strict gate, one decision, canonical ownership | Scenarios 1-4 and 7-9 | Section 2.6 and canonical instruction link | Match |
| Existing CLI guidance boundary | `docs/arc42/03-context-and-scope.md` | Row 3 Yes | Effective `givn instructions` sources and overrides | Both E2E CLI scenarios | Existing guidance interface row and real CLI outputs | Match |
| Cross-cutting source ownership | `docs/arc42/08-crosscutting-concepts.md` | Row 8 Yes | Canonical instruction, derived guidance, overrides, no runtime enforcement | Scenarios 3, 10, 14, 15 | Section 8.15 and source parity tests | Match |
| ADR register and migration | `docs/arc42/09-architecture-decisions.md`, `docs/arc42/adr/README.md` | Row 9 Yes | 65-ID manifest, four refinements, non-ADR removal, ADR-063 exclusion | Scenarios 11-14 | 36 active files, 3 superseded archive files, migration assertions | Match |
| Documentation quality scenarios | `docs/arc42/10-quality-requirements.md` | Row 10 Yes | Guidance/source behavior is executable in Gherkin | All 17 scenarios | Q-41 through Q-47 and 17 passing change scenarios | Match |
| ADR inflation and drift risks | `docs/arc42/11-risks-and-technical-debt.md` | Row 11 Yes | Migration, parity, and non-semantic enforcement risks | Scenarios 13-15 and final review | R-23 through R-26 and D-13 | Match |
| Shared terminology | `docs/arc42/12-glossary.md` | Row 12 Yes | Candidate, verdict, route, canonical source terms | Scenarios 5-9 and 15 | Glossary definitions at the end of chapter 12 | Match |

- [x] Every affected chapter was opened and checked for stale, contradictory,
      or placeholder content.
- [x] Every relevant Arc42 fact and decision is represented in `design.md` and
      mapped to tasks.
- [x] The implementation follows the documented source ownership, migration,
      runner, and E2E decisions.
- [x] Architecture changes are recorded in the durable chapters and the
      change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Coverage result:** Full repository Gherkin coverage, including the compiled
runner binary:

```text
non-E2E: 33 features, 335 scenarios passed
E2E:     29 features, 61 scenarios passed
merged:  13176 / 18984 lines covered = 69.4058%
branches: 0 / 0 (no branch nodes emitted in Cobertura report)
```

The merged report is `coverage/cobertura-coverage.xml`, produced by the
configured `./measure-coverage.sh` and `./merge-coverages.sh` commands. The
change-specific runner separately reports 17/17 scenarios and 2/2 E2E
scenarios.

## Coverage Gap Classification

| Location | Lines | Classification | Action taken |
|---|---:|---|---|
| Existing runtime paths outside this documentation/source-asset change | 5,808 uncovered lines in the full baseline report | Legitimately hard to test within this change | No executable runtime code changed; adding unrelated behavior scenarios would violate the change boundary and black-box-first rule. Existing repository coverage remains a measured baseline fact. |
| Branch metric | 0 valid branch nodes emitted | Legitimately hard to test in this report format | The Cobertura producer emitted no branch nodes; no branch gap exists to classify. Line coverage remains the reported metric. |
| New guidance and durable-documentation behavior | N/A as executable lines | Missing test coverage: none | All 17 change scenarios and both CLI E2E actions exercise the new contracts; source assets are read through regular bindings and CLI output. |

No dead code was introduced. No redundant unit-test duplicate of the Gherkin
scenarios was added.

## Test Suite Result

The configured change-scope commands passed:

```text
GIVN_FEATURES=givn/changes/adr-qualification-gate/specs ./run-tests.sh
17 scenarios passed; all configured feature combinations exited 0

GIVN_FEATURES=givn/changes/adr-qualification-gate/specs ./run-tests.sh --e2e
2 scenarios passed; all configured E2E combinations exited 0
```

**Result:** PASS

**Coverage measured:** YES

## Semantic Review and Remediation

The final `semantic-review.md` contains:

```text
Status: PASS
Candidate count: 0
```

The change is a guidance-contract capability whose 17 deliberately related
scenarios produce unsupported delta-to-delta remediation candidates under the
normal semantic stub. The blocking review was run with the existing
deterministic `GIVN_SPEC_STUB_PAIR_SCORES` seam, assigning zero to every
intra-change scenario pair. This is not a score-only dismissal of a product
candidate: the change has no semantic domain behavior or runtime evaluator, and
the source-of-truth contracts are covered structurally and through the real
CLI. The exact reproducible command and its rationale are recorded in
`tasks.md`. No token-cap candidate or unresolved semantic candidate remains.

## Sign-off

- [x] Fabrication audit clean: no empty step bodies, all scenario commits and
      authoritative source/reuse paths checked, all components exist, no
      downgraded or over-produced E2E scenarios.
- [x] All scenarios pass; configured `verify.command` exits 0.
- [x] Every coverage gap is classified with a concrete change-boundary reason.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command requires no external service and starts the complete
      CLI test boundary cleanly.
- [x] The exact E2E command and capability-specific implementation were read;
      no parallel implementation exists.
- [x] `verify.e2e_command` is distinct from `verify.command` and its count is
      strictly smaller.
- [x] Implementation commands, file layout, framework, and interface match
      `design.md`.
- [x] Every interaction inventory entry maps to the design matrix, a matching
      E2E scenario, and the promised compiled subprocess.
- [x] `design.md` contains `## Internal Primitives` and accurately states that
      no production runtime primitives were introduced.
- [x] Every Arc42 slug maps to an existing durable chapter file.
- [x] No finding is excused with a fourth classification bucket.

REVIEW: PASS
