# Design review: make-user-questions-decision-ready

## Review boundary and provenance

FreshPlanGrill performed the mandatory independent fresh-context grilling before this hardening. Its complete 654-line result (`agent://FreshPlanGrill`), not just a preview, is the retained handoff: `questions=[]`, eight mechanical findings F1–F8, three resolved facts R1–R3, all branch dispositions, independent chapter selection/comparison, ADR verdicts and evidence. This hardening read that full result, all planning artifacts and effective design-review, dialog, specs, design and arc42-docs instructions, plus the named repository mechanisms and selected Persona records.

No new approval, deferral or human tradeoff was found. The proposal's already-approved effective-policy ownership (D1) and proportional multi-shot teaching (D2) remain authoritative. proposal.md is unchanged. Only planning artifacts and affected durable architecture documentation are changed. No tasks, production implementation, test glue, installed guidance, scenario execution, build or formatter is created/run here. A plan PASS is not delivery, behavioral GREEN, deployed rendering, operator visual approval or archive evidence.

## Grilling log index

- Fresh independent grilling source: `agent://FreshPlanGrill`, complete result.
- Questions asked of the user: **0**.
- Full new question/answer records: **none**; no answers or rejected human alternatives are manufactured.
- Existing decision records: proposal.md, `D1 — One canonical dialog policy with universal entry-point references` and `D2 — Multi-shot contrasts and proportional question treatment`.
- Codebase-resolved findings: F1–F8 and R1–R3 below. An empty question list did not itself constitute sign-off; all eight findings required concrete hardening.

## Resolved by codebase exploration

| ID / branch | Verified finding and concrete hardened disposition | Rejected incorrect mechanism / evidence boundary |
|---|---|---|
| F1 — Testability | Current script/plain/nextest discovery executes removed placeholders. design.md now requires runner-owned non-removed eligibility using the exported `givn::gherkin::delta::is_removed_placeholder`, shared by plain `filter_run`, both nextest list phases and exact execution. Explicit removed-unit requests fail by name. Mandatory tasks WIP protection and `run_verify_hook` remain; their unchanged declared script reaches the corrected runner under cargo/nextest, including the result-file forced cargo path. Future edit: tests/features_runner.rs only. Expected delta execution is 19 total, 14 regular, 5 E2E; five removals contribute no pass/skip/undefined count. | No unconditional-success placeholder handler, zero-match success presented as removal proof, claim that current runner already excludes removals, tag/name combination or weakened strictness. The reviewer ran discovery only and saw 13 original records, including the removal; this is not scenario evidence. |
| F2 — E2E fidelity / Inventory | The fragment now has request-by-id and session-start actions only. Specs/design/review/dialog, JSON and active context are inputs. Retain the specs-only machine-readable E2E; explicitly remove redundant design/review E2E identities, preserving their meaningful contracts through modified existing regular design/review scenarios with real returned guidance. The matrix and comments agree. Future bindings move into the regular instruction module, with no duplicate step definitions/source readers. | No inherited-title justification for excess actions, no claim specs JSON exercised dialog, no silent E2E tag stripping and no regular duplicate of an unchanged E2E outcome. Preserved variants assert setup/capture or disposition requirements absent from the JSON shape happy path. |
| F3 — E2E fidelity | The existing selection E2E writes a line to piped stdin. Its modified delta now requires a real labeled PTY prompt and selected integrations. Reuse/move the existing script-backed helper into tests/common for separate E2E and existing regular callers, with its timeout/status semantics. Fossil/no-SCM rows are init preconditions: remove their E2E identities explicitly and preserve attribution, warning, generated guidance and later-refusal contracts as new regular titles. | Piped input is not terminal evidence. The old product prompt is not alleged broken. No new target-selection behavior, backend action, prompt redesign or unrelated initialization contract rewrite. Current retained references are not claimed exercised. |
| F4 — Architecture docs | Original independent row 1 Yes/author No is corrected to hardened author Yes. Chapter 01 has a narrow pending stakeholder/agent-ergonomics objective linked to Q-93–Q-95; architecture index, arc42 assessment/affected-file list and design descriptions include chapter 01. Existing priorities and stakeholders remain. All 12 original independent and author values are preserved below, including compatible row 7 No/Yes. | No erasure of independent provenance, relabeling the original omission as a match, new stakeholder or goal reprioritization. STATUS: DONE is not evidence of implemented behavior. |
| F5 — Testability / Visual | design.md now maps **every one of 19 executable deltas** to the old defect or preserved boundary, concrete actual-consumer assertion and discriminating mechanism. OLD_PRODUCT_RED, ORACLE_RED, DRIVER_RED and already-correct baseline contracts are distinguished. Controlled omissions target the relevant delivered guidance, leaving other evidence intact; all five E2Es have concrete terminal baseline and final transcript obligations. | Undefined/unimplemented strictness failures and compilation failures are not behavioral RED. No fabricated RED, production fault switch, source-text test, whole-prose snapshot, general Markdown/NLP engine or model-dependent permanent test. Static motion/focus/hover/breakpoint nonapplicability is explicit, not deferred. |
| F6 — Boundaries / Missing scenarios | Added regular `Successful project skill and command replacements are preserved without policy injection`. Valid LF-terminated project-owned givn-propose skill/command replacements deliberately omit default obligations and contain no variables/conditionals. Capture exact source bytes; sync; compare all three selected skill consumer files and the opencode command to independent replacement bytes, without hidden prefix/suffix; source bytes remain exact and managed startup still loads the effective policy. Specific source/consumer paths are in design.md. | Existing malformed-override errors do not prove successful authority. No default-only fixture presented as replacement proof, no production-renderer expected-output echo, no automatic reconciliation/injection or new E2E action. This boundary was already approved. |
| F7 — Assertion cutover | Added the remaining modified no-active-change dialog caller to the delta and future assertion migration. Retain actual exit-0 policy availability and untouched unknown-id nonzero/known-artifact error behavior. Assert returned universal/proportional preparation guidance rather than the old named-artifact sentence. Retire obsolete phrase/source/full-record handlers only after all affected callers migrate. | Do not re-pin the old sentence to revised wording, delete an availability contract or keep source readers to make obsolete assertions pass. |
| F8 — Rendered reference / Vocabulary | Added `references/plain-dialog-reference.md` and its actual 96-column, no-ANSI Rich Markdown terminal render `.txt`: approved D1 direction and all five complete hypothetical records/bad/good/contrast examples from the unchanged proposal. It is a planning view, outside evidence/visual. The glossary now qualifies screenshots/states as Web UI evidence and names terminal transcripts; Question record semantics distinguish actual decisions from brief information requests. No new appearance options are needed because plain direction and content do not change beyond D1/D2. | No newly proposed appearance is claimed approved. The viewer is not a new product dependency/renderer or source of style requirements. This is not compiled consumer output, final transcript, Persona judgment or VISUAL-REVIEW PASS. Final captures and human judgment are mandatory, with no approved deferral. |
| R1 — Scope / Technology | Current generation is 13 core skills plus domain-modeling and dev-principles, and 13 core commands plus grill-me/grill-with-docs: 30 source templates. Commands are opencode-only; domain-modeling requires both domain_modeling and arc42. The renderer has four substitutions and addon conditions, not includes. Keep explicit thin references and independently enumerate installed families. | Reject runtime includes, hidden preambles, optional-family expectations in the addon-free 13/13 fixture, commands under agents/claude, or treating pointer distribution as prose compliance. |
| R2 — Decision ledger | D1 and D2 are settled in proposal.md and resolve to named design realization paths. There is no ideation seed and thus no missing seed link. | Do not re-ask approval, invent a seed or turn a mechanical verification correction into an operator tradeoff. |
| R3 — Risk | The likely substantive failure is reference-without-compliance or competing partial rules. Touched ideation full-option wording and optional tone policy are concrete normalization targets. Keep real delivered-document checks separate from fresh unseen authoring contexts and actual human judgments. | No model-score guarantee, live-chat interception, example repetition presented as generalization evidence or operator fact-finding question about researchable assertions. |

### Mechanical evidence homes

- Removal/strictness: tests/features_runner.rs:38–64,94–133; run-tests.sh:29–38,62–70; src/gherkin/delta.rs:93–105; src/lib.rs public gherkin export; src/cli/check.rs:108–129; src/hooks.rs:114–118 (dispatch) and its GIVN_FEATURES hook contract.
- Real input/PTY distinction: tests/steps/init_agent_target_selection_steps.rs:140–156,168–234; tests/common/mod.rs:333–369; src/cli/init.rs:182–221; src/skills/targets.rs:36–39 establishes opencode/agents/claude order.
- Real consumers/authority: src/config/cascade.rs:55–103; src/skills/render.rs:29–65; src/skills/targets.rs:217–260,501–539; src/cli/skills.rs:14–22; src/cli/instructions.rs:349–409,473–508. Consumer equality is a user-owned replacement contract, not a source implementation assertion.
- Old dialog callers/source assertions: tests/steps/instruction_integrity_steps.rs:389–516; permanent instruction-integrity.feature:73–99. The current specs-only JSON E2E is in instruction_integrity_e2e_steps.rs:142–173.
- Teaching content and approval provenance: proposal.md, D1/D2 and `Multi-shot examples for publication in the dialog instructions`; no conversation timestamp or new approval is invented.

## Open questions

**None.** There are zero unresolved human decisions, zero new approvals and zero approved deferrals. F1–F8 have concrete planning resolutions. Implementation/review obligations are not open design questions: runner exclusion and strictness proof, real executable assertions, final CLI/PTY transcripts, actual confirmed-Persona/human visual review and unseen question exercises must still be completed before delivery/review/archive. This planning sign-off does not waive or claim those results.

## Required branch dispositions

| Branch | Disposition |
|---|---|
| Scope | Proposal scope matches the hardened plan: canonical policy, universal references, five hypothetical contrasts, override preservation, no runtime enforcement. F1–F8 harden planning fidelity, not a question API, provider, override injection or hypothetical migration product. Proposal unchanged. |
| Decision ledger | D1/D2 resolve with named realization paths. No seed, carried decision or new human question. |
| Technology choices | Existing assets, cascade, shared overview, managed-block updater and CLI/Cucumber mechanisms suffice. No dependency or runtime subsystem; F1/F5 repair verification mechanics. |
| Missing scenarios / Errors / Boundaries | F6 fills successful skill/command replacement preservation; F7 migrates no-active-change assertions. Unknown-id refusal remains. Unknown compatibility/source facts and hypothetical boundaries are document-contract assertions, not fictional product fixtures. |
| Testability | fail_on_skipped and plain failed/skipped nonzero handling remain. Rust unimplemented! is a strictness stub only. Exact name-only targeting remains; F1 excludes removal records and F5 distinguishes old-product, driver and oracle failures. No scenario/probe execution claimed. |
| E2E fidelity | Real CLI subprocesses and public generated documents are the interface. PTY is required for terminal selection. F2/F3 normalize variants and remove redundancies explicitly; primary report/prompt assertions accompany document checks. |
| Interaction coverage | Every normalized reviewed inventory row has a matrix title and concrete driver. One new session-start action; instruction ids/JSON/backend values are variants. Generated-distribution and initializer preservation are distinct capability invariants over the same physical sync action, not invented commands. Unaffected rows are references only. |
| Risk | Mitigate reference-without-compliance, partial-rule conflict, stale owner replacements, hypothetical-source confusion and stale binary use through the named deterministic and unseen-context checks. No new human tradeoff. |
| Use-case context / Scope rejection criteria | create-change remains one Developer goal with an initialized-project precondition, actor trigger and step-numbered invalid-id extension. initialize-project remains one setup goal with step-numbered invalid-selection refusal. No goal conjunction, missing primary actor, misplaced mutation, unreferenced extension, flow-established precondition, oversized leaf or conditional Extends misuse. Inherited mechanism detail is not a new domain boundary. |
| Capability ownership / Typed relations | Retrieval stays instruction-integrity in reusable support; distribution stays prose-consolidation in create-change; startup refresh stays initialize-a-project in initialize-project. Both leaf contracts include support unconditionally and declare no Extends. No NEW capability/migration ledger. |
| Confirmed Personas | Reuse the three existing selected records and their recorded provenance; no invention/re-promotion. Per-Persona relevance and evidence below. Personas are review lenses, not Gherkin actors. |
| Visual contract / Approval | Terminal/plain Markdown direction and D1/D2 content remain. F5/F8 supply baseline applicability and actual rendered planning view. Existing content approval is not final operator visual approval; normal visual/Persona review remains mandatory. |
| Interface classification, opposite tested | Browser-rendered HTML/CSS/JS would require Web UI/browser evidence. Actual CLI prints text/JSON and writes Markdown, with no browser/page/endpoint. Later editor viewing does not make this capability a Web UI. |
| Mandatory-check deferrals | None requested/approved. Strictness, execution, five final delta transcripts, Persona/visual judgment and unseen exercises remain mandatory implementation/review checkpoints; not silently deferred or presented as done. |
| ADR qualification | All three candidates below are NOT_QUALIFIED; canonical lower homes are explicit. No moved accepted boundary, new MADR, amendment or supersession is warranted. |
| Architecture documentation | All 12 chapter files contain actual content. Correct row 1 omission; retain compatible broader row 7 selection. The reviewer inspected all chapters and rendered all seven Mermaid blocks without writing diagram sources; changes here are narrow prose, not scaffold or diagram revisions. |
| Vocabulary | Reuse Dialog policy, Question record, Cascade, Generated skill, Slash command and AGENTS.md ownership. Glossary now explicitly preserves proportional information requests and terminal-vs-Web evidence. No new term. |

### Confirmed Persona dispositions

| Confirmed Persona | Relevance / existing evidence | Required review boundary |
|---|---|---|
| Primary user (developer--primary-user) | Existing record owns project/repository authority and judges workflow/preservation. Existing owning use-case contracts already select it. Informed consent and preserved human instructions directly affect it. | Human-readable decision contents and boundaries, real transcript readability and exact preservation must be reviewed; no approval inferred from CLI success. |
| Technical role (agent--technical-role) | Existing record consumes machine contracts and explicitly cannot judge prose/domain/visual quality. Startup/direct/delegated loading and active JSON integrity affect it. | Machine/delivery checks are appropriate; it cannot substitute for the required human semantic/visual judgment. |
| Givn maintainer (givn-maintainer--owner) | Existing record owns specs, vocabulary, architecture and harness with recorded operator-confirmed Frame/persona provenance. One policy owner and honest verification affect it. | Review semantic scope/ownership/evidence fidelity; do not claim a marker or pointer guarantees compliance. |

## Architecture documentation check — all 12 rows

**Independent assessment preceded marker access.** FreshPlanGrill recorded its complete twelve-row assessment in its evaluation kernel/output before first opening arc42.md. The original comparison is preserved below; hardening is not a new independent assessment and does not rewrite that history. The original author values were read from the pre-hardening marker. Selection row 1 concerns both chapter 01 and chapter 10; row 10 independently selects chapter 10.

| # / selection subject | Original independent | Original author | Hardened author | Match / reason | Actual chapter content status |
|---|---|---|---|---|---|
| 1 — Goals, stakeholders, quality (01 + 10) | Yes | No | Yes | Original omission F4; now match. Approved informed-question outcomes refine existing agent ergonomics without changing priorities. | 01 has actual purpose/requirements/goals/stakeholders plus the new narrow pending objective; 10 has actual quality tree/scenarios plus pending Q-93–Q-95. No implementation claim. |
| 2 — Constraints (02) | No | No | No | Match; no new legal/tech/org/dependency/authority constraint. | Actual Rust/embedded/default precedence, gate, addon and acceptance constraints remain consistent; no scaffold-only chapter. |
| 3 — Interfaces/context (03) | Yes | Yes | Yes | Match; universal discovery and effective-policy public content scope, not chat interception. | Actual context/interfaces and AGENTS ownership plus explicit pending universal-discovery exclusion match design. |
| 4 — Strategy (04) | No | No | No | Match; existing embedded-asset/cascade/managed-block strategy reused. | Actual strategy table and feature-free/configurability facts remain valid; no new includes/injection strategy. |
| 5 — Building blocks (05) | Yes | Yes | Yes | Match; existing owners receive changed content responsibilities, no new component. | Actual Rust owners/dependencies and pending instructions/templates/project-hint ownership/evidence distinction match design. |
| 6 — Runtime flows (06) | Yes | Yes | Yes | Match; startup/once-per-context/fresh-delegation authoring flow changes. | Actual flows/diagrams plus pending unconditional managed block, shared overview, preserved metadata/human content and presenter responsibility match design. |
| 7 — Deployment (07) | No | Yes | Yes | Compatible original difference retained: deployment mechanism unchanged, author documents broader content adoption through rebuild/normal sync. Not a reviewer Yes/author No omission. | Actual deployment content plus pending existing-footprint refresh, owner replacements untouched, no service/file type or compliance claim. |
| 8 — Crosscutting concepts (08) | Yes | Yes | Yes | Match; universal proportional policy spans workflows/addons with effective precedence. | Actual cascade/instruction/JSON/ownership concepts plus pending full policy owner/thin references/addon independence/evidence separation match. |
| 9 — ADRs (09) | No | No | No | Match; no qualified new/refined/replaced architecture boundary. | Genuine chapter register and ADR-directory index link separate existing records. No missing qualified MADR or empty scaffold; no new record required. |
| 10 — Quality scenarios (10) | Yes | Yes | Yes | Match; discovery, preservation and unseen presentation quality need acceptance targets. | Q-93–Q-95 and verification signals distinguish deterministic delivery from human/fresh-context judgment and remain pending, not claims of model compliance. |
| 11 — Risks/debt (11) | Yes | Yes | Yes | Match; missing entry obligations, partial rules, stale overrides and reference/compliance confusion are explicit. | Actual risk/debt register; R-67, R-73, R-74 have concrete pending mitigations. No qualified ADR consequence is missing. |
| 12 — Glossary (12) | Yes | Yes | Yes | Match; existing Dialog policy meaning refined, proportional Question record preserved; F8 evidence qualification corrected. | Genuine glossary already holds all selected terms; pending dialog target, actual decision-vs-information distinction, Web UI screenshots/states and terminal transcript alternative now agree. |

There is **no remaining independent Yes / hardened author No or omitted row**. Every chapter selected by either party has actual matching content, not placeholders. Chapters 2/4/9 also remain real and coherent despite No selection. arc42.md STATUS: DONE is an author completion marker, not independent proof or production evidence.

The independent chapter-integrity scan found no forbidden box drawings or non-table ASCII pseudo-diagrams; chapter 08's fenced lists/formula/file layout/TDD labels/status literals are not spatial diagrams. All seven existing Mermaid blocks were rendered by FreshPlanGrill with exit 0 and nonempty SVG: chapter 01 (14,392 bytes), 03 (14,365), 05 (33,441), 06 (28,048; 26,353; 24,574), 10 (36,082). These are attributed independent diagram checks, not rerun here; none of those diagrams is changed by hardening.

## ADR qualification and canonical homes

The complete procedure remains owned by the effective arc42-docs instruction. This record reports candidate-specific verdicts/evidence, not a parallel qualification policy. `must_be_shared=SUPPORTING` does not override a failed mandatory dimension.

| Candidate | Alternatives | Architectural impact | Durable consequence | Lower-level artifact | Existing ADR check | Verdict / route / exactly one canonical home |
|---|---|---|---|---|---|---|
| Universal effective dialog ownership/discovery (D1) | PASS | FAIL | FAIL | FAIL | PASS | NOT_QUALIFIED; CANONICAL_ARTIFACT; assets/instructions/dialog.md; target_adr=null, replacement_adr=null |
| Five hypothetical contrasts/proportional treatment (D2) | PASS | FAIL | FAIL | FAIL | PASS | NOT_QUALIFIED; CANONICAL_ARTIFACT; assets/instructions/dialog.md; target_adr=null, replacement_adr=null |
| Thin explicit template/delegation references versus includes/injection | PASS | FAIL | FAIL | FAIL | PASS | NOT_QUALIFIED; CANONICAL_ARTIFACT; givn/changes/make-user-questions-decision-ready/design.md; target_adr=null, replacement_adr=null |

- **D1 evidence:** proposal D1 records project-only, dialog-only, universal references, full copies and interception alternatives and the selected/rejected reasons. The design retains component owners, project-first precedence, managed ownership and JSON shape. Guidance rebuild/sync is locally revisable with no migration/new protocol. The complete policy rationale already has its specific process home: assets/instructions/dialog.md; distribution HOW is not another ADR rationale.
- **D2 evidence:** proposal D2 records multi-shot versus rules-only/universal-form alternatives and selects worked demonstrations. These hypothetical records add teaching, not storage, recovery, compatibility, deployment or module boundaries. No example migration is implemented or promised. The policy content is locally revisable and fully owned by the same canonical process asset.
- **Thin-reference evidence:** design contrasts short explicit entry/prompt references with a new include mechanism/hidden preamble. The chosen realization retains renderer substitutions/conditions and authority boundaries, imposes no new durable component protocol, and belongs completely in this design. Template file count alone does not make an architectural decision.
- **Existing-record/duplicate check evidence:** FreshPlanGrill read both active and historical registers and searched recursive effective/dialog/include/ownership references; hardening also read the chapter/index and the relevant accepted records. ADR-005:55–62 preserves project-first facets/stale-owner review; ADR-013:55–79 preserves selected target/fenced ownership and existing real-terminal selection; ADR-048:48–62,80–83 already contrasts/rejects runtime includes and owns effective policy/JSON metadata; ADR-069 retains the single model-free binary. No accepted authority boundary is refined or replaced. Gherkin/declared-runner boundaries ADR-011/015 remain unchanged.
- **Falsification dispositions for all three:** the policy/local realization is implementable and reversible as guidance within one change without moving component boundaries; shared author knowledge is supporting, not a new multi-component future contract; question-authoring/teaching behavior has a complete lower-level home. The local HOW choice does not adopt the structural include/injection alternative.
- **MADR/consequences:** no candidate passes all mandatory gates, no NEW_ADR exists and no AMEND_ADR/SUPERSEDE_ADR is qualified. Therefore no new separate MADR/index entry or new ADR consequence is missing in chapters 09/11. Existing R-67/R-73/R-74 document the real guidance risks. No unrelated ADR, duplicate rationale in ADR archive or claimed scaffold is created.

## Changes made during hardening

| Artifact | Concrete change |
|---|---|
| design.md | Correct counts; runner exclusion/edit set and tasks path; normalized matrix; complete 19-row assertion/RED map; exact replacement bytes/paths; caller/helper cutovers; PTY driver; baseline applicability; rendered-reference and final evidence obligations; chapter-01 impact. |
| specs/fragments/instruction-integrity.feature | Preserve one request-by-id E2E; explicit design/review E2E removals; migrate their existing regular contracts to returned CLI content; modify no-active-change assertion caller. |
| specs/fragments/fragment.md | Two normalized instruction actions, honest specs-only JSON reference; preserve the base witness once. |
| specs/create-change/prose-consolidation.feature | Add successful skill/command replacement/no-injection and still-present managed-start invariant. |
| specs/initialize-project/initialize-a-project.feature | Primary report assertions; real PTY selection E2E correction; explicit backend E2E removals and regular meaningful new backend titles. |
| specs/initialize-project/usecase.md | Three normalized init actions; backend/no-SCM preconditions on noninteractive initialization, not independent actions. |
| arc42.md | Correct row 1 and affected files; qualify glossary update, final verification boundary and independent provenance. |
| docs/arc42/01-introduction-and-goals.md | Narrow pending stakeholder quality objective, no priority/stakeholder changes. |
| docs/arc42/README.md | Include chapter-01 stakeholder objective in pending target status. |
| docs/arc42/12-glossary.md | Proportional decision-record definition; Web UI screenshot/state qualification and terminal transcript alternative. |
| references/plain-dialog-reference.md / .txt | Exact approved direction/five contrasts plus accurate provenance/evidence boundary; actual terminal Markdown render, not synthetic product execution. |
| design-review.md | Full independent grilling index/dispositions, original/hardened twelve-row comparison, ADR qualification evidence, changes and final sign-off. |

Intentionally unchanged: proposal.md (no fundamental scope mismatch); specs/create-change/usecase.md (existing goal/ownership/inventory remain valid); the other durable chapters (their narrow pending content already matches); all production/test/installed assets, permanent corpus and command declarations. No tasks.md exists in this deliverable.

### Corrected delta counts

| Capability | Added | Modified | Removed | Executable | Surviving E2E |
|---|---|---|---|---|---|
| instruction-integrity | 5 | 5 | 2 | 10 | 1 |
| prose-consolidation | 3 | 1 | 1 | 4 | 1 |
| initialize-a-project | 2 | 3 | 2 | 5 | 3 |
| **Total** | **10** | **9** | **5** | **19** | **5** |

There are 24 delta records and 14 regular executable scenarios. Four removed records retain @e2e to make their former classification explicit; they do not count toward five surviving E2Es or executable results. All 19 executable scenarios remain @wip. Counts were obtained from the actual corrected tag/title records; no runner execution is inferred.

### Rendered-reference evidence

Actual render: Rich `Markdown` via a recorded terminal `Console`, width 96, color_system=None, no ANSI styles. The output was observed and written to references/plain-dialog-reference.txt (17,222 UTF-8 bytes; 16,482 characters). It contains the real rendered three-goal table, short directory clarification, both availability choices, material conversion/recovery contrast and no-question diagnostic report, with all five hypothetical record contexts. The corresponding Markdown preserves the supplied local links and approved content (12,838 bytes). Viewer chrome is nonnormative, not a new product appearance option. No Givn binary was represented as producing this target content.

## Final lint verification

Ran **once after all planning/spec/document/reference edits**:

```sh
givn lint --change make-user-questions-decision-ready
```

Observed exit **2**, summary `givn lint: 3 file(s) checked, 19 finding(s)`.
The complete output contains **19 expected WIP findings** (one per executable
delta), **166 SUBST advisories**, and no parse, duplicate-title or inventory
mapping failure. Five removed records produce no WIP finding. The captured
complete tool output is `artifact://38`; this command executed no scenarios.

Subset advisories remain visible, not suppressed or misreported as GREEN.
Some compare intentionally preserved backend content/attribution contracts;
many compare a generic exit/file assertion with unrelated lifecycle scenarios.
The scenario map supplies the distinct trigger/invariant, and normal review
must still disposition every actual reported pair. No later overlap review,
behavioral RED/GREEN, transcript or archive gate is claimed passed here.
No scenario text changed after this final lint; the edit below records its
actual result and planning sign-off only.

## Sign-off checks

- [x] Mandatory independent fresh grilling completed; all required branches have dispositions.
- [x] Zero new user questions recorded honestly; D1/D2 preserved; no fabricated approval/deferral.
- [x] F1–F8 mechanically hardened and R1–R3 grounded; meaningful contracts preserved without silent tag stripping.
- [x] All 19 executable deltas have concrete discriminating assertion mechanisms; strictness and old-product failures are distinct.
- [x] Normalized inventories/matrix, real CLI/PTY driver and primary assertion plans agree.
- [x] All twelve architecture rows retain original independent/author provenance; row 1 corrected, row 7 compatible; actual matching chapter content and vocabulary confirmed.
- [x] ADR gates have candidate-specific evidence/canonical homes; no qualified MADR/consequence omission.
- [x] Actual rendered planning reference exists; final transcript/Persona/VISUAL-REVIEW obligations are not waived or claimed complete.
- [x] Planning-only scope retained; proposal, production/test/installed guidance and tasks untouched.
- [x] Final lint ran once, exited 2 for 19 deliberate WIP findings, with no parse error; 166 advisories remain explicit.

All eight mechanical planning findings are resolved with concrete changes.
No human question, unapproved deferral, architecture omission or missing
prerequisite remains for design-review. This PASS authorizes the next planning
stage only; implementation and its mandatory evidence remain unperformed.

DESIGN-REVIEW: PASS
