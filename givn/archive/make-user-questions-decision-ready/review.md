# Review: make-user-questions-decision-ready

README-IMPACT: none

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
$ grep -n 'fn .*{ *}\|=> {}\|{}$' tests/steps/instruction_integrity_steps.rs \
    tests/steps/instruction_integrity_e2e_steps.rs tests/steps/prose_consolidation_steps.rs \
    tests/steps/prose_consolidation_e2e_steps.rs tests/steps/init_agent_target_selection_steps.rs \
    tests/steps/initialize_a_project_e2e_steps.rs
(no matches)
$ grep -rn 'unimplemented!\|todo!' tests/steps/
(no matches)
0 empty step bodies found across 6 touched step-definition files (the temporary strictness probe was
removed in the setup task).
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a matching commit that touches
production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| runner strict-mode proof and runner-owned removal exclusion | `3c4d6fc5` | test/evidence only (see disposition) |
| The canonical dialog policy is retrievable through the Givn CLI | `900d4113` | yes: assets/instructions/dialog.md |
| The canonical dialog policy demonstrates the full question record | `4b62e672` | yes: assets/instructions/dialog.md |
| Dialog teaching contrasts show complete decisions rather than bare labels | `ad0c8873` | yes: assets/instructions/dialog.md |
| Migration consent does not imply an unverified recovery guarantee | `637206b1` | yes: assets/instructions/dialog.md |
| The dialog policy is retrievable without an active change | `516c8361` | yes: assets/instructions/dialog.md |
| Design guidance requires the complete E2E setup record | `a4672291` | test/evidence only (see disposition) |
| Review guidance names the deterministic disposition gates | `b2a21aa9` | yes: assets/instructions/review.md |
| Machine-readable startup guidance retains the dialog obligation while reporting an active change | `feb04512` | yes: src/cli/instructions.rs |
| Disabling optional addons does not disable the dialog obligation | `5b6883bb` | yes: assets/addons/absolute-mode/policy.md, assets/commands/givn-archive.md, assets/commands/givn-characterize.md… |
| Delegated question preparation carries the policy without assuming inherited context | `8ee968d9` | yes: assets/commands/givn-characterize.md, assets/commands/givn-design-review.md, assets/commands/givn-implement.md… |
| Successful project skill and command replacements are preserved without policy injection | `05b5be02` | yes: assets/agents-project-hint.md |
| A project dialog override remains the effective policy after synchronization | `2b01c32a` | test/evidence only (see disposition) |
| Fossil initialization names the supported attribution path without Git plumbing | `3f6d677b` | test/evidence only (see disposition) |
| Initialization without SCM preserves guidance and names the later archive refusal | `af60abc6` | test/evidence only (see disposition) |
| e2e step skeletons, strict-mode proof and scope isolation | `cfe1951a` | test/evidence only (see disposition) |
| Author starts a session with the universal dialog obligation | `8644f419` | yes: src/cli/instructions.rs |
| Generated guidance points to the canonical E2E policy | `4fde0ad6` | yes: assets/commands/grill-me.md, assets/commands/grill-with-docs.md, assets/skills/givn-dev-principles/SKILL.md.tmpl… |
| Consumer runs init non-interactively and gets opencode skills | `b3cc273f` | test/evidence only (see disposition) |
| Consumer re-syncs skills after init and the selection is honoured | `d365e66a` | test/evidence only (see disposition) |
| Consumer selects agent targets during interactive init | `b782fc33` | test/evidence only (see disposition) |

Dispositions for the commits marked `test/evidence only`:
- Setup: runner-owned removal exclusion in `tests/features_runner.rs` — test infrastructure the design
  named; no production source is involved by construction.
- Design guidance requires the complete E2E setup record, A project dialog override remains the effective
  policy after synchronization: assertion migration/coverage of retained or already-delivered behavior;
  `design.md`'s scenario map classifies them BASELINE_GREEN with an ORACLE_RED calibration.
- Fossil initialization names the supported attribution path without Git plumbing, Initialization without
  SCM preserves guidance and names the later archive refusal: precondition/backend variants whose real
  assertions were already bound; the delta reclassifies them from `@e2e` to regular coverage and their
  commits record the verified evidence (`design.md` §Interaction Coverage Matrix and the scenario map).
- E2E scenarios: the change's E2E work is test code plus committed terminal transcripts; production files
  were delivered by the earlier scenarios they assert against.

**Promised-architecture cross-check** — every component design.md said would be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Canonical policy content | yes | `assets/instructions/dialog.md` |
| Unconditional project-start obligation | yes | `assets/agents-project-hint.md` |
| Shared startup overview obligation | yes | `src/cli/instructions.rs::generic_overview()` |
| Tone-policy deference | yes | `assets/addons/absolute-mode/policy.md` |
| Shipped templates (15 skills + 15 commands) | yes | `assets/skills/*/SKILL.md.tmpl`, `assets/commands/*.md` |
| Runner-owned removal exclusion | yes | `tests/features_runner.rs` |
| E2E step modules (one per capability) | yes | `tests/steps/instruction_integrity_e2e_steps.rs`, `tests/steps/prose_consolidation_e2e_steps.rs`, `tests/steps/initialize_a_project_e2e_steps.rs` |
| Shared PTY helper | yes | `tests/common/mod.rs::run_init_in_terminal` |
| No new dependency, module, JSON field, include mechanism or renderer preamble | confirmed absent | `git diff --stat` shows no `Cargo.toml`/manifest/config changes |

**Strict-mode proof** — confirmed present and passing in the tasks setup task:

```
$ GIVN_FEATURES=<probe dir> cargo test --locked --test features_runner -- --name '^Probe with an undefined step$'
  1 scenario (1 failed)  → exit status: 1     EXIT=1
$ ... --name '^Probe with an unimplemented step$'
  1 scenario (1 failed)  → exit status: 1     EXIT=1
$ GIVN_FEATURES=<probe dir> ./run-tests.sh --e2e
  Summary [0.332s] 1 test run: 0 passed, 1 failed, 2 skipped   EXIT=100
```
Recorded in tasks.md's setup task with the removal-exclusion listings (19/14/5/14 units, direct removed-unit
refusal EXIT=2) and the permanent-corpus exit 0.

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps checked for a primary assertion on
the observable output of the real interface named in design.md (CLI subprocess / real PTY), not
repository-only:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Author starts a session with the universal dialog obligation | yes — real `givn instructions` stdout | N/A (CLI, not Web UI) | no |
| Generated guidance points to the canonical E2E policy | yes — real `givn skills sync` stdout plus delivered consumer documents | N/A | no |
| Consumer runs init non-interactively and gets opencode skills | yes — real `givn init` stdout plus delivered managed block | N/A | no |
| Consumer re-syncs skills after init and the selection is honoured | yes — real `givn skills sync` stdout plus refreshed block | N/A | no |
| Consumer selects agent targets during interactive init | yes — real PTY terminal stream | N/A | no |

**E2e scope check** — each capability has one `@e2e` scenario per distinct happy-path action, not one per
variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| instruction-integrity | 1 (`instruction-integrity: request session-start instructions without id`) | yes — request-by-id keeps its retained regular/JSON variants | no |
| prose-consolidation | 1 (`synchronize selected guidance`) | yes — distribution over one action; addon/delegation/replacement cases are regular invariants | no |
| initialize-a-project | 3 (interactive selection, non-interactive init, resynchronization) | yes — three distinct consumer actions per the reviewed matrix | no |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command` invokes, and confirmation the
tree was searched for a parallel e2e implementation:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| all | `./run-tests.sh --e2e` → `cargo nextest run --test features_runner` with `GIVN_CUCUMBER_TAGS=@e2e`; steps in `tests/steps/*_e2e_steps.rs` | no — `git status` clean; the only e2e binding files are the three modules plus the retained specs/JSON e2e module | N/A |

**Local environment check** — design.md's Local Runnability section declares no service to start (the
product is a CLI; fixtures are tempfile roots):

```
$ ./run-tests.sh            EXIT=0    (permanent corpus, 573 tests)
$ ./run-tests.sh --non-e2e  EXIT=0    (permanent regular scope, 482 tests)
no server, port, database, digital twin or network dependency is involved.
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read literally from
`givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 14 (delta `--non-e2e`); 482 permanent | 5 (delta `--e2e`); 91 permanent | yes (5 < 19) |

The archive verification receipt does not exist yet for this change (unproven candidate); the counts above
come from the same runner invocation the archive will perform, recorded in tasks.md.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | create-change rules 39-40 ("every user-facing question follows the effective dialog policy…") and initialize-project rules ("managed project-start block requires the effective dialog policy…") are realized by `assets/instructions/dialog.md`, `assets/agents-project-hint.md` and the templates | yes |
| Capability ownership and typed relationships preserved | fragment `support` still declares `instruction-integrity`; `prose-consolidation` and `initialize-a-project` extend their existing owners; no `Extends` added | yes |
| Confirmed Persona-relevant outcomes preserved | developer answers informed questions and keeps human guidance; the authoring agent finds the policy at startup/direct/delegated entry; the maintainer keeps one policy owner | yes |
| Actors remain distinct from Persona biographies | Gherkin actors stay author/consumer; Personas are review lenses only | yes |
| Migration ledger and coverage evidence complete, if applicable | no migration ledger applies (content and test infrastructure only); coverage receipt arrives with the archive | yes |

PERSONAS: developer--primary-user

Also reviewed as lenses (recorded evidence): `agent--technical-role` (runtime/machine contracts) and
`givn-maintainer--owner` (one policy owner, honest evidence).

## Deferred mandatory checks

None deferred. Strictness proofs, all scenario executions, the five E2E transcripts and the Persona/visual
judgement were performed or are recorded, not waived.

## Design.md conformance check

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Effective policy ownership | one canonical `dialog` instruction plus universal short references in hint/overview/templates; no renderer include, injection, JSON field or middleware | `assets/instructions/dialog.md` rewritten; hint + `generic_overview()` + 30 templates carry the short obligation; `src/config/cascade.rs` untouched | yes | N/A |
| Multi-shot content | five contrasts published in the canonical policy with anchors `example-N-decision-record` and hypothetical labels | published verbatim with anchors and illustrative record links | yes | N/A |
| Removal exclusion | runner-owned `is_removed_placeholder` filter in `tests/features_runner.rs` for plain/nextest discovery/nextest exact paths; no dummy-success steps | implemented exactly; counts 19/14/5 and named refusal proven | yes | N/A |
| E2E fidelity | real CLI subprocesses; the interactive selection action must use the script-backed PTY; the PTY helper moves to `tests/common/mod.rs` | done; piped stdin removed from that action; helper moved with all callers updated | yes | N/A |
| Step definition locations | extend `instruction_integrity_steps.rs`/`_e2e_steps.rs`, `prose_consolidation_steps.rs`, `init_agent_target_selection_steps.rs`; add `prose_consolidation_e2e_steps.rs` and `initialize_a_project_e2e_steps.rs`; register only the two new modules | exactly these files; two new modules registered | yes | N/A |

No deviation was taken silently; the only post-review artifact correction is the proposal's Capability
Routing decision (see "Routing decision correction" below), which changes no design decision.

## Interaction coverage verification

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| instruction-integrity: request effective instructions by id | yes | `An agent requests machine-readable artifact guidance` (retained) | real CLI subprocess | `run_givn` subprocess in `instruction_integrity_e2e_steps.rs` | yes |
| instruction-integrity: request session-start instructions | yes | `Author starts a session with the universal dialog obligation` | real CLI subprocess | `run_givn(["instructions"])` | yes |
| prose-consolidation: synchronize selected guidance | yes | `Generated guidance points to the canonical E2E policy` | real CLI subprocess plus delivered consumer documents | `run_givn(["skills","sync"])` plus file assertions | yes |
| initialize-a-project: select agent targets in terminal | yes | `Consumer selects agent targets during interactive init` | real PTY (`script -qec`) | `common::run_init_in_terminal` | yes |
| initialize-a-project: initialize with explicit noninteractive selection | yes | `Consumer runs init non-interactively and gets opencode skills` | real CLI subprocess | `run_givn(["init","--no-addons","--agents","opencode"])` | yes |
| initialize-a-project: resynchronize selected targets | yes | `Consumer re-syncs skills after init and the selection is honoured` | real CLI subprocess | `run_givn(["skills","sync"])` | yes |
| retained inventory references (visual-design-contract, change-release-note) | yes | unaffected retained scenarios | real CLI subprocess | unchanged | yes |

## Visual Review

Interface: terminal (no Web UI; the opposite classification — a rendered page — does not hold because the
product prints text/JSON and writes Markdown, with no browser renderer, page or endpoint).

| @e2e Scenario | Evidence kind | Evidence file | Present and non-empty? |
|---|---|---|---|
| Author starts a session with the universal dialog obligation | transcript | `givn/changes/make-user-questions-decision-ready/evidence/visual/author-starts-a-session-with-the-universal-dialog-obligation/transcript.txt` | yes |
| Generated guidance points to the canonical E2E policy | transcript | `…/generated-guidance-points-to-the-canonical-e2e-policy/transcript.txt` | yes |
| Consumer runs init non-interactively and gets opencode skills | transcript | `…/consumer-runs-init-non-interactively-and-gets-opencode-skills/transcript.txt` | yes |
| Consumer re-syncs skills after init and the selection is honoured | transcript | `…/consumer-re-syncs-skills-after-init-and-the-selection-is-honoured/transcript.txt` | yes |
| Consumer selects agent targets during interactive init | transcript | `…/consumer-selects-agent-targets-during-interactive-init/transcript.txt` | yes |

| confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | answer informed questions and keep owned guidance | managed block preserves human bytes; obligation names the effective policy; terminal shows readable labels | none — passes |
| agent--technical-role | find the policy at startup/direct/delegated entry | obligation present in the overview, every delivered entry and every delegated prompt; no full policy copy | none — passes |
| givn-maintainer--owner | one policy owner and honest evidence | single canonical policy; five transcripts; no claim of model compliance | none — passes |

**Baseline (terminal):** headings identify their subject; no internal-identifier-only labels; no duplicated
visible content (exactly one managed block, one obligation per entry); the configured language is kept;
error paths (unknown instruction id, no-SCM refusal) remain named. Motion/focus/hover/viewport rules are
nonapparent to static terminal content by construction, not deferred.

VISUAL-REVIEW: PASS

## Retirements

RETIRE: none — this change retires no capability (the five `@givn.removed` records delete individual
scenarios, not a capability).

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Stakeholder quality objective (informed questions) | `docs/arc42/01-introduction-and-goals.md` | Yes (pending target note) | §Decisions and §Use-case traceability | dialog scenarios 1-4 | `assets/instructions/dialog.md` | match |
| Public context and generated-document scope | `docs/arc42/03-context-and-scope.md` | Yes | §Scope of universal references | distribution tasks | hint + templates + overview | match |
| Building blocks / owners unchanged | `docs/arc42/05-building-block-view.md` | Yes | §Production edit set | all production tasks | no new module or dependency | match |
| Runtime flows (startup, once-per-context, delegation) | `docs/arc42/06-runtime-view.md` | Yes | §Canonical instructions | startup + delegation tasks | `generic_overview()`, delegation prompts | match |
| Deployment footprint unchanged | `docs/arc42/07-deployment-view.md` | Yes (broader note) | §Override and refresh | init/sync tasks | `givn/commands.yaml` unchanged; no new files | match |
| Crosscutting rule spans workflows/addons | `docs/arc42/08-crosscutting-concepts.md` | Yes | §Scope of universal references | addon-disabled task | addon-free sync assertion | match |
| Accepted ADRs unchanged | `docs/arc42/09-architecture-decisions.md` | No new ADR | §Architecture impact | none | no ADR added | match |
| Question-quality acceptance targets | `docs/arc42/10-quality-requirements.md` | Yes (pending) | §Fresh-context verification | scenario map | delivered-contract assertions + planned fresh-context exercise | match |
| Risks R-67/R-73/R-74 | `docs/arc42/11-risks-and-technical-debt.md` | Yes | §Risks | scenario map, ORACLE_RED categories | evidence categories recorded per scenario | match |
| Dialog policy glossary meaning | `docs/arc42/12-glossary.md` | Yes | §Existing behavior and policy wording | dialog tasks | glossary wording matches the delivered policy | match |

- [x] Every chapter marked affected by either the independent review or `arc42.md` was opened and checked.
- [x] Every Arc42 fact relevant to this change is represented in `design.md` and mapped to tasks.
- [x] The completed implementation follows those facts; evidence names files, commands and results.
- [x] Architecture changes discovered in the completed implementation are recorded in the durable chapters.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Coverage result:** no measured receipt exists yet for this change; the archive measures it against the
committed candidate and records it in `givn/archive/<id>/verification.json` and the README.

```
unproven candidate — no receipt yet. Static classification below; the archive receipt will carry the
measured line/branch percentages.
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `assets/instructions/dialog.md`, `assets/agents-project-hint.md`, `assets/instructions/review.md`, `assets/addons/absolute-mode/policy.md`, `assets/skills/**`, `assets/commands/**` | content assets | not executable code | asserted as delivered documents by the delta scenarios and the E2E transcripts |
| `src/cli/instructions.rs::generic_overview()` | new obligation sentence | missing test coverage | covered by the session-start E2E, its JSON variant and the `generic_overview` unit test |
| `tests/common/mod.rs::run_init_in_terminal`, `tests/features_runner.rs` | test infrastructure | not production code | exercised by the existing PTY scenarios and the new interactive-selection E2E |

## Scenario execution evidence

```
unproven candidate — no receipt yet. Pre-archive delta scopes from the same runner the archive invokes:
  ./run-tests.sh (delta)          19 scenarios, 19 passed, 0 failed       EXIT=0
  ./run-tests.sh --non-e2e        14 scenarios, 14 passed, 0 failed       EXIT=0
  ./run-tests.sh --e2e             5 scenarios,  5 passed, 0 failed       EXIT=0
Permanent corpus: ./run-tests.sh → 573 passed, 0 failed                   EXIT=0
```

**Receipt status:** unproven (candidate not yet archived)

## Overlap dispositions

The overlap gate's scan reports no shape match involving this delta, so its required table is empty:

| Scenario A | Scenario B | Disposition |
|---|---|---|

`givn lint --change make-user-questions-decision-ready` still reports 165 subset **advisories**
whose files include this delta (they share only generic steps — exit codes, file existence, named
output — with unrelated permanent or neighboring scenarios). Every one of them is dispositioned by
trigger and invariant in `evidence/overlap-advisories.md`: distinct capabilities/actions are
`boundary`, same-capability invariant variations are `variant`, and no pair is a duplicate.


## Split-or-keep

The overlap gate reports no long-scenario findings for this delta (`givn lint` count 0), so no
split-or-keep row is required.

| Scenario | Decision |
|---|---|

## Routing decision correction

`givn check review` blocked the archive with "capability routing decisions do not match the built delta:
decision EXTEND instruction-integrity: capability does not exist in the permanent corpus; record 'NEW in
<usecase-id>' instead". `instruction-integrity` is declared in `givn/specs/fragments/fragment.md` and its
delta lives under `specs/fragments/`; fragment capabilities carry no use-case directory, so the routing
table must name the fragment. The proposal's Capability Routing row was corrected to `NEW in fragments`
with the reason recorded inline; no design decision, scenario, or task changed, and design.md is
unaffected (so design-review does not re-open).

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified, all promised components exist,
      no downgraded e2e scenarios, no HTTP/fetch substitution, no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope (pre-archive runs above; the archive receipt will
      carry the authoritative measured result).
- [x] Every coverage gap is classified and resolved (or justified) from static inspection; no measured
      receipt exists yet.
- [x] No `@wip` tags remain in the delta spec (`givn check tasks` passed).
- [x] The delta spec contains no implementation-layer detail beyond the deliberate CLI/command names the
      interaction policy requires.
- [x] Local run command starts the declared product cleanly; no service stack is involved.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and the tree was searched for a
      parallel e2e implementation before accepting it.
- [x] `verify.e2e_command` is not identical to `verify.command`; counts prove real isolation (5 < 19).
- [x] Implementation (commands, file layout, framework/driver) matches design.md as reviewed; the only
      post-review correction is the proposal routing decision recorded above.
- [x] Interaction coverage verified: every spec inventory entry maps to a design matrix row, an existing
      `@e2e` scenario, and step definitions using the promised driving mechanism.
- [x] design.md carries no `## Internal Primitives` section because the change introduces no new internal
      primitives (no new struct/fn/enum in production code; only content and one existing function's
      string).
- [x] arc42 slug-mapping verified: `arc42.md` names only chapters that map to real `docs/arc42/<slug>.md`
      files.
- [x] No finding anywhere in this report is excused with a classification outside the three buckets.

REVIEW: PASS
