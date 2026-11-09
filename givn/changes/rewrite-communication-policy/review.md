# Review: rewrite-communication-policy

Review kind: self-review
Author: givn-agent
Reviewer context: givn-agent (self)

## Implementation Evidence Check (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```text
grep -rn --include=*.rs -E "fn [a-z_]+\([^)]*\) *(-> *[^ {]+ *)?\{\s*\}" tests/steps
→ 13 matches, all pre-existing: fixture string literals in archive_proof_support.rs,
  outline_report_support.rs, and eight intentional no-op assertion steps
  (dialect_has_no_rule, english_only, archive_receipt_missing,
  equal_outline_content, human_review_confirms, no_separate_revision,
  no_semantic_classifier).
→ 0 empty bodies among the five step functions added by this change
  (tests/steps/default_all_addons_steps.rs).
```

**Commit cross-check** — every checked-off task in tasks.md has a matching
commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| The communication block binds documentation to named artifacts | `f9644a9f` | Yes — `assets/addons/communication-policy/policy.md`, `src/cli/addons.rs`, `tests/steps/default_all_addons_steps.rs`, `AGENTS.md` |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `CommunicationPolicyBlock` (asset plus renderer) | Yes | `assets/addons/communication-policy/policy.md`; `src/skills/targets.rs:358` (`sync_communication_policy`) |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```text
tasks.md "Runner and strict-mode proof": probe scenario with an undefined step
failed (`Step doesn't match any function`, 1 scenario failed), probe removed,
real suite then passed 7/7.
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps checked
for a primary assertion on the observable output of the named interface:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Consumer enables the outline addon and it is listed as enabled | Yes — reads `givn addons list` stdout from a real subprocess | N/A (CLI) | No — untouched by this change |

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| `enable-optional-addons` | 1 | Yes — addon enablement through the CLI | No |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and the tree (including untracked files) searched for a parallel e2e
implementation:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| `enable-optional-addons` | `tests/features_runner.rs` via `./run-tests.sh --e2e` | No — `git status` shows only the change directory and the reviewed edits | N/A |

**Local environment check** — the local run command starts the product
cleanly:

```text
cargo run --quiet -- instructions
→ prints the project-state line and "Change: rewrite-communication-policy  [tasks-done]"
```

**E2E scope check** — `verify.command` and `verify.e2e_command` read literally
from `givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Explicit scope selection | Regular scope evidence | E2e scope evidence | Zero failed or skipped? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | `run-tests.sh` derives scope from `--non-e2e`/`--e2e` and exports `GIVN_RESULT_SCOPE` | not archived yet | not archived yet | pending archive |

**Design.md conformance check** — implementation diffed against what design.md
explicitly named:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Block source | One asset; `AGENTS.md` carries the byte-identical rendered copy | One asset; fences identical | Yes | N/A |
| Rule set | 11 rules in `Tone` and `Documentation` | Same 11 rules | Yes | N/A |
| Layout policy | No documentation file names, no README budget | None; asserted absent | Yes | N/A |
| Size | At most 300 words | 237 words including the begin marker | Yes | N/A |
| Test home | `tests/steps/default_all_addons_steps.rs` | Same file | Yes | N/A |
| E2E | No new `@e2e`; enablement normalizes to the existing scenario | Same | Yes | N/A |

**Interaction coverage verification**:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| enable an addon (all addon enablement normalizes here) | Yes | Consumer enables the outline addon and it is listed as enabled | spawn `givn addons enable outline` | existing CLI subprocess step | Yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `initialize-project` rule on `communication_policy` updated in the change narrative to cover documentation formats and reject repository layout. | Yes |
| Capability ownership and typed relationships preserved | `enable-optional-addons` still owns the block; no relationship changes. | Yes |
| Confirmed Persona-relevant outcomes preserved | The block remains readable text for the developer and the authoring agent. | Yes |
| Actors remain distinct from Persona biographies | Gherkin uses the developer actor; Personas stay review lenses. | Yes |
| Migration ledger and coverage evidence complete, if applicable | The addon rename migration path is unchanged. | Yes |

PERSONAS: developer--primary-user

## Deferred mandatory checks

None.

## Visual Review

N/A — the design declares `Interface: none`; the change alters generated
Markdown text.

VISUAL-REVIEW: PASS

## Retirements

N/A — this change retires no capability.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Addon scope: presentation and documentation | `docs/arc42/02-architecture-constraints.md` §2.3 | Yes | Architecture-impact row | Batch `communication-block` | Ch. 2 text updated in `f9644a9f` | Match |
| Managed fenced blocks in `AGENTS.md` | `docs/arc42/03-context-and-scope.md` §3.1 | Yes | Architecture-impact row | Batch `communication-block` | Ch. 3 text updated in `f9644a9f` | Match |
| Sync writes the addon-owned block | `docs/arc42/06-runtime-view.md` step 7 | Yes | Architecture-impact row | Batch `communication-block` | Ch. 6 text updated in `f9644a9f` | Match |
| Guidance ownership stays with served instructions; the addon owns presentation and documentation only | `docs/arc42/08-crosscutting-concepts.md` | Yes | Architecture-impact row | Batch `communication-block` | Ch. 8 text updated in `f9644a9f` | Match |
| No ADR: content choice inside an existing asset | `docs/arc42/09-architecture-decisions.md`, `arc42.md` qualification | No | Decisions ledger (D1) | Batch `communication-block` | Chapter 9 unchanged; `arc42.md` records `ADR action: NONE` | Match |
| Glossary term "Communication policy" | `docs/glossary.md` (chapter 12 links it) | Yes | Architecture-impact row | Batch `communication-block` | Glossary row updated in `f9644a9f` | Match |

- [x] Every chapter marked affected by either the review or `arc42.md` was opened and checked for stale, contradictory, or placeholder content.
- [x] Every Arc42 fact relevant to this change is represented in `design.md` and mapped to the batch in `tasks.md`.
- [x] The completed implementation follows those facts; evidence names the built files and commit.
- [x] No architecture change discovered in the implementation that the durable chapters and the change-level assessment do not record.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN

---

## Coverage

Coverage is measured once by the archive against the committed candidate.
No receipt exists yet; review classifies from static inspection.

**Coverage result:** no measured evidence yet — the archive will record the merged report.

```text
unproven candidate — no archive receipt yet
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| none identified | — | — | All new step functions execute assertions on the rendered block. |

---

## Scenario execution evidence

```text
unproven candidate — no receipt yet
```

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| | | |

## Split-or-keep

| Scenario | Decision |
|---|---|
| | |

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies in the change, commit verified, promised component exists, no downgraded e2e scenarios, no over-produced e2e scope).
- [x] Scenario execution evidence will be recorded per scope in the archive verification receipt.
- [x] Every coverage gap is classified and resolved from static inspection; the archive will record measured evidence.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the product cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified; no parallel implementation exists.
- [x] The E2E execution selects its scope explicitly (`GIVN_RESULT_SCOPE`); both executions must report zero failures and skips at archive.
- [x] Implementation matches design.md as reviewed; no deviation.
- [x] Interaction coverage verified: every inventory entry maps to a matrix row, an existing `@e2e` scenario, and step definitions using the promised mechanism.
- [x] Design.md's internal primitives: the change introduces one internal helper (`communication_block`), recorded here; no production struct/fn/enum is added.
- [x] arc42 slug-mapping verified: every chapter slug in `arc42.md` maps to a real `docs/arc42/<slug>.md`.
- [x] No finding is excused with a classification outside the three coverage buckets.

README-IMPACT: none

**REVIEW: PASS**
