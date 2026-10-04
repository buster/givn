# Design: make-user-questions-decision-ready

## Status and implementation boundary

This is a planning design, not a claim that the guidance is deployed. The proposal's option-3 scope and five multi-shot contrasts are approved. After independent hardening the delta contains 24 records: 10 added, 9 modified, and 5 removed; 19 executable scenarios comprise 14 regular and 5 E2E scenarios. Four removal records explicitly retain their former `@e2e` classification; they are merge instructions, not executable evidence. Every surviving executable delta remains `@wip` until implementation proves it.

No production source, installed guidance, test glue, or task record is changed in this planning step. The independent fresh grilling and hardening sign-off are recorded in design-review.md. Durable architecture notes and the rendered planning reference describe the approved target, not deployed behavior.

## Decisions

### Effective policy ownership (Decides D1)

Decides D1: use the effective `dialog` instruction as the only complete user-question policy, with unconditional project/startup references, a consistent short obligation in all shipped skill and command templates, and explicit delegation handoff. Realization: `assets/instructions/dialog.md`, `assets/agents-project-hint.md`, `src/cli/instructions.rs::generic_overview`, the 30 template files enumerated below, and their self-contained delegation prompts.

Preserve the existing cascade and managed-block boundaries. No renderer include facility, automatic policy injection into project-owned overrides, new manifest artifact, JSON field, middleware, provider client, or question API is introduced. This is content and a small existing-overview update, not a new runtime subsystem.

### Multi-shot instruction content (Decides D2)

Decides D2: publish the proposal's five complete scenario-record/bad/good/contrast examples inside `assets/instructions/dialog.md`. Realization: the canonical policy's example section, after its normative rules and retained question-record illustration. The approved example text is in proposal.md, section `Multi-shot examples for publication in the dialog instructions`; do not invent another product decision while transferring it.

Keep local heading anchors `example-1-decision-record` through `example-5-decision-record` intact. Declare all five examples hypothetical and their in-file source links illustrative records, not verified product files or current behavior. Preserve the separate chosen/rejected-alternative decision-record example. Do not make examples into runtime fixtures that pretend a hypothetical migration or library already exists.

### Scope of universal references and project ownership

Universality is an authoring obligation in every context. The deterministic generated-reference contract covers the bundled templates available under the selected targets/addons, with no project skill/command replacements in that fixture. A project-owned replacement template remains authoritative and is not secretly prefixed or edited; its owner must reconcile stale question guidance. The unconditional managed project block and startup guidance still direct the agent to the effective policy. This is the proposal's override-preservation boundary, not an exception allowing an agent to ignore the loaded policy.

### Existing behavior and policy wording

Use the existing `Dialog policy`, `Question record`, `Cascade`, `Generated skill`, `Slash command`, and `AGENTS.md ownership` vocabulary. Refine the existing dialog-policy scope; mint no new domain term or persona.

Normalize old partial references and imperative `every question carries the full option treatment` wording to the canonical policy. Preserve substantive workflow confirmations, open-question records, one-question-at-a-time semantics, and rejected-alternative reasons. Clarification is not forced into fictional alternatives; material decisions retain their full treatment. Do not retain incidental prose merely to satisfy old substring assertions: retire or replace affected wording-only assertions with the actual consumer contract. No unrelated capability is rewritten.

## Use-case and Persona traceability

| Contract/source | Requirement or guarantee | Capability | Realization | Evidence and confirmed Persona impact |
|---|---|---|---|---|
| create-change, main flow 2 and rules 39-40 | one effective policy; derived guidance points to it | prose-consolidation | short top-level references and explicit delegation prompts | changed generated-guidance E2E plus addon/delegation variants; `agent--technical-role` discovers the policy, `givn-maintainer--owner` maintains one owner |
| initialize-project, rules and success guarantee | selected integrations do not misstate the project; managed guidance preserves user content | initialize-a-project | existing init/sync refresh of the unconditional project-hint asset; correct the selection E2E to the existing script-backed PTY | two policy/preservation E2Es plus the terminal-selection fidelity correction; `developer--primary-user` retains human instructions and target choices |
| support fragment, instruction-integrity | effective instruction retrieval with preserved project precedence | instruction-integrity | canonical dialog content and existing non-artifact instruction cascade | dialog/design/review regular variants, authoritative override and retained specs-only JSON E2E; no claimed dialog execution by that E2E |
| support fragment, session-start interaction | normal instructions identify the question obligation before a change is selected | instruction-integrity | shared generic overview, including existing JSON `overview` | one new startup E2E and active-change/JSON variant; developer is not required to know every artifact |
| proposal acceptance 1-7 | decision subject, brief relevant context, useful source, concrete consequences, material revision costs, necessary independent questions, proportionality | instruction-integrity | canonical rules, record example, five worked contrasts | delivered-document contract assertions plus fresh-context semantic review; developer can give informed answers |

Actors in Gherkin remain the author/consumer; the confirmed Personas are review lenses, not extra actors. The selected Persona IDs are `developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner`. No new persona or promotion is required.

## Production edit set

### Canonical instructions and unconditional entry points

| Path | Planned edit | Preserved boundary |
|---|---|---|
| `assets/instructions/dialog.md` | broaden scope to every user-facing question; state the project-familiar audience; add necessary-question/proportionality/source/consequence/material-change rules and all five contrasts | one complete effective policy; existing decision-record semantics |
| `assets/agents-project-hint.md` | add a short unconditional policy-loading/delegation bullet after the existing startup instruction | edit the source asset, not generated AGENTS.md by hand |
| `src/cli/instructions.rs`, `generic_overview()` | append the same short obligation to the shared overview | preserve status, next action, existing JSON schema, and state-report branches |
| `assets/addons/absolute-mode/policy.md` | make question-content requirements explicitly defer to the effective dialog policy; keep tone rules compatible with brief necessary context | the addon controls tone and is not the policy's enabling switch |

Suggested entry obligation, used consistently without the full rule set:

> For every user-facing question, follow the effective policy returned by `givn instructions dialog`. Load it once in this context before the first question, including clarification and confirmation outside a named workflow. Pass this obligation explicitly to fresh delegated contexts; the presenting agent remains responsible for the question.

Place it after a skill's H1 and after a command's introductory summary, never inside an addon condition. The default reference mentions no optional-addon vocabulary. Existing context/state guidance and artifact-specific mechanics remain in place. If no Givn project is initialized, startup guidance still tells the agent to initialize first; it must not claim policy retrieval succeeded in an uninitialized directory.

`generic_overview()` currently supplies every text report and the JSON `overview` field. Add no `dialog_policy` field. Active startup JSON keeps its existing `id`, `next_command`, `next_skill`, status/stage data and next-action semantics. The JSON variant fixture prepares an active change awaiting specs, rather than relying on this change's evolving current stage.

### Complete shipped-template inventory

Core skill templates (`assets/skills/<name>/SKILL.md.tmpl`):

- givn-propose
- givn-spec
- givn-design
- givn-tasks
- givn-implement
- givn-review
- givn-archive
- givn-explore
- givn-design-review
- givn-steps
- givn-reverse-engineer
- givn-characterize
- givn-ideation

Optional skill templates, carrying the same unconditional obligation whenever installed:

- givn-domain-modeling (existing domain_modeling + arc42 availability)
- givn-dev-principles (existing dev_principles availability)

Core command templates (`assets/commands/<name>.md`): the same 13 core names above. Optional commands: `grill-me.md` and `grill-with-docs.md`. There is no generated domain-modeling or dev-principles command. Skills go to selected `.opencode`, `.agents`, and `.claude` targets; commands go only to `.opencode/commands` when opencode is selected. Assertions must not claim commands exist in the other targets.

The renderer currently supports addon conditionals and four variable substitutions, not includes. Repeat only the short reference at these existing entry points. Reject a new include mechanism or render-time automatic preamble: it adds an unnecessary protocol and can change authoritative project overrides. Repeated pointers are discovery, not competing policy copies.

### Delegation edit set

Insert the policy obligation inside each self-contained prompt, not merely in the parent's surrounding command prose:

| Command template | Self-contained delegated context |
|---|---|
| givn-propose.md | thinking architect |
| givn-design-review.md | fresh grilling reviewer and independent hardening reviewer |
| givn-implement.md | implementation agent |
| givn-review.md | review agent |
| givn-characterize.md | characterization agent and reverse-review agent |
| givn-reverse-engineer.md | discovery agent |

Also amend the spawning/review passages in skills givn-implement, givn-design-review, givn-review, givn-characterize, givn-ideation, and givn-reverse-engineer. Each fresh context loads the effective policy before preparing a question, without assuming parent's context inheritance. The orchestrator verifies a delegated question before sending it to the human. Question-tool labels/recommendations are not substitutes for visible context and effects.

Keep reference-only scope in givn-steps and principles: they still carry the obligation if their instructions lead an agent to ask a question, without inventing a new interaction step in those skills.

### Override and refresh behavior

Preserve the existing precedence:

| Surface | Project-owned replacement | Behavior |
|---|---|---|
| dialog instructions | `givn/artifacts/dialog/instructions.md` | resolves instead of bundled dialog content; text/JSON both use it |
| skill template | `givn/skills/<name>/SKILL.md.tmpl` | replaces the bundled template, without hidden injection |
| command template | `givn/commands/<name>.md` | replaces the bundled command, without hidden injection |
| project-start block | Givn's fenced `givn-project` block in AGENTS.md | refreshed from the embedded hint on init/sync; outside text is preserved |

Do not modify `src/config/cascade.rs`, the template-rendering protocol, target selection, addon config, rule tracking, or the artifact graph. Existing overrides remain owner-controlled. An override lacking the new policy requires owner review, not an automatic rewrite or a false claim that sync upgraded it.

Defaults are compile-time embedded. After implementation, consumer smoke uses the rebuilt checkout binary, not a stale global installation. Normal downstream refresh is install/upgrade the new binary, then `givn skills sync`; repository self-dogfood follows AGENTS.md after archive.

## Specification delta and cutover

Files: `specs/fragments/instruction-integrity.feature`, `specs/create-change/prose-consolidation.feature`, and `specs/initialize-project/initialize-a-project.feature`.

- Modify canonical-dialog retrieval and full-record scenarios, plus the no-active-change retrieval caller, preserving availability and unknown-id refusal behavior while retiring incidental wording/source assertions.
- Add teaching-contrast, recovery-disclosure, dialog-override, session-start and active JSON startup variants.
- Normalize request-by-id to one E2E: retain the existing specs-JSON happy path; explicitly remove design/review E2E identities and migrate their content contracts into the existing regular design/review scenarios using returned CLI guidance.
- Modify the generated-guidance E2E for the complete shipped entry-point contract; remove the obsolete regular `Generated interactive guidance points to the canonical dialog policy` check rather than duplicate it.
- Add addon-disabled, self-contained-delegation and successful project skill/command replacement/no-injection invariants, distinct from the default distribution happy path.
- Modify init/sync E2Es for addon independence, real report assertions, human-byte preservation and persisted selection. Modify interactive-selection E2E for the real PTY; do not change the product prompt.
- Explicitly remove the Fossil and no-SCM E2E identities and add regular backend-precondition contracts with new titles. Preserve their attribution/warning/refusal invariants; backend choice is not another init action.

Every removed record has exactly one placeholder step. Runner exclusion below is REQUIRED before any delta execution, including `givn check tasks`; removal is not a passing scenario. Surviving E2Es keep `@e2e` unchanged. No silent tag stripping or unrelated corpus rewrite is authorized.

Change-side `usecase.md` snapshots preserve the existing actor goals, guarantees, relationships and base-hash witnesses; append only the dialog/ownership rules and examples. The support `fragment.md` records the affected instruction actions, not a new capability or a duplicated included-use-case inventory. Permanent corpus documents are untouched until normal archive merge.

## Step definitions and deterministic assertion design

| Capability | Regular bindings | Separate E2E bindings |
|---|---|---|
| instruction-integrity | extend `tests/steps/instruction_integrity_steps.rs` | extend `tests/steps/instruction_integrity_e2e_steps.rs` |
| prose-consolidation | extend `tests/steps/prose_consolidation_steps.rs` | add `tests/steps/prose_consolidation_e2e_steps.rs` |
| initialize-a-project | extend `tests/steps/init_agent_target_selection_steps.rs` for backend variants; share its existing PTY mechanism | add `tests/steps/initialize_a_project_e2e_steps.rs` for changed init/sync/selection consumers |

Register only the two new E2E modules in `tests/steps/mod.rs`. No new production dependency or alternate hand-maintained test suite is introduced.

Reuse `TestEnv`, real `CARGO_BIN_EXE_givn` subprocess helpers, isolated roots, exit-code and generated-consumer checks. Noninteractive actions use the existing subprocess helpers; piped stdin does not prove interactive selection. New Given steps create the declared precondition with real CLI commands. Remove the dialog-request helper's hardcoded `--change dialog-question-quality` coupling; request `instructions dialog` against the prepared initialized root. Keep active-change creation only where declared.

Capture actual command output in the existing world fields. Read generated consumer files from the selected target paths; do not substitute asset-source reads for distribution evidence. Compare user-owned override/AGENTS/config bytes before and after to prove preservation. Expected installed families come from the independent public template inventory above and the fixture's explicit targets/addons, not the production generator's own output list, so a missing family cannot make both actual and expected silently shrink.

Assertions target the delivered document contract, not a verbatim prose snapshot: scope and audience, meaningful decision-record fields, the supplied hypothetical record and local link relationships, displayed goal rows and bounded alternatives, and explicit recovery/compatibility distinctions. For references, the public command `givn instructions dialog` and context-loading/delegation obligations are contract semantics, not an incidental spelling of an explanatory paragraph. Parse relevant blocks and generated self-contained prompts; do not satisfy a missing prompt obligation with text elsewhere in the same file. Use small borrowed section slices and existing parsing facilities; no new generalized Markdown renderer or NLP layer.

Migrate every affected dialog assertion caller, including `The dialog policy is retrievable without an active change`; keep its real CLI exit-0/general-policy boundary and the untouched unknown-id nonzero/known-artifact error contract. Then remove the obsolete named-artifact/option/standalone/rejected-alternative substring handlers and source-asset-loading/full-record handlers in `instruction_integrity_steps.rs:411-510` when no callers remain. Replace the design/review regular source readers with real returned guidance for the preserved variants. Do not re-pin new wording or keep source-text tests. Unrelated specs stay unchanged. Deterministic document contracts do not prove understandable agent questions; fresh-context review below is separate.

Move the obsolete design/review E2E request bindings and their retained consumer-contract assertions into `instruction_integrity_steps.rs` when removing those E2E identities; share subprocess mechanism, never register duplicate step text or retain stale source readers. Move the existing PTY process helper from `init_agent_target_selection_steps.rs` into `tests/common/mod.rs` for the new separate initializer E2E and its existing regular PTY callers. Preserve raw-key semantics, timeout and propagated status; update every helper caller in that cutover. This is the future test-fidelity edit set, not a source edit made by the planning review.

### Scenario-by-scenario assertion and behavioral RED map

Implement real Then assertions before changing their delivered surface. Run them against the pre-change checkout binary/output with the exact prepared scenario fixture; record assertion failures, not undefined bindings or compilation errors. Where one guidance edit couples several scenarios, use a controlled omission of only the relevant delivered guidance in an isolated fixture, through an existing project instruction/template override or a temporary omitted consumer excerpt, and rerun the same assertion. This calibrates discrimination; it is not proof that the old product failed an already-correct preservation contract. Restore/remove the fixture omission immediately; never add a production switch, exception, fake success, source-text test or model-dependent permanent test.

`OLD_PRODUCT_RED` below names missing target behavior in current delivered guidance. `ORACLE_RED` names a controlled missing-guidance counterexample when the old contract already passes or a coupled prior edit is already GREEN. `DRIVER_RED` is a genuine old harness fidelity defect, not a product regression. Record these categories honestly in later scenario evidence; none has been executed in this review. Existing assertions such as exit 0, ownership preservation or full rejected-alternative records may already pass and are retained boundaries, not invented old defects. Strictness probes are separate.

| Executable delta scenario | Old defect or retained boundary | Meaningful actual-consumer assertion | Behavioral RED mechanism |
|---|---|---|---|
| The canonical dialog policy is retrievable through the Givn CLI | Old scope is limited to named interactive workflows; it lacks the project-familiar audience and explicit necessary/proportional/material-cost treatment. | Inspect returned normative sections for universal scope, audience, subject/context/source/consequence obligations and research-before-escalation; do not accept a teaching quote as the normative rule. | OLD_PRODUCT_RED on current CLI policy; after coupled content delivery, ORACLE_RED by omitting the universal normative section through a temporary dialog override. |
| The canonical dialog policy demonstrates the full question record | Existing chosen/rejected record fields work, but old full-option treatment does not distinguish routine clarification or bounded confirmations. | Returned decision illustration retains blocks, context, genuine alternatives, concrete consequences and chosen/rejected reasons; normative clarification treatment requires use/immediate effect without invented alternatives; confirmations bound accept/revise authority. | OLD_PRODUCT_RED on missing proportional/confirmation treatment; ORACLE_RED omit only that treatment if the policy was delivered by an earlier scenario, leaving the record illustration intact. |
| Dialog teaching contrasts show complete decisions rather than bare labels | Current policy contains none of the five supplied records/contrasts. | Require all five record sections and resolvable local links to the corresponding record. Example 1 has exactly the three supplied goal rows and excludes staff/cross-member authority; example 2 asks only target intent; example 3 shows both general/task availability outcomes; example 4 preserves 8080 and names compatibility cost/benefit; example 5 reports the inactive/active-label counterexample and rejects a second answer. All are hypothetical, not project files. | OLD_PRODUCT_RED on missing records. ORACLE_RED remove one goal row or one linked record from a delivered override while retaining all headings; the same assertions must fail. |
| Migration consent does not imply an unverified recovery guarantee | Current dialog has no material-commitment contrast or unknown-recovery boundary. | Example 4 distinguishes conversion/recovery required before rollout from recovery already implemented, states older-release incompatibility and unchanged saved projects for the less-committing answer; normative rule establishes unknown facts or narrows consent. | OLD_PRODUCT_RED on current policy; ORACLE_RED omit the recovery/compatibility qualification only, leaving the conversion choice and port value intact. No real saved-format migration fixture is invented. |
| A project dialog override remains the effective policy after synchronization | Cascade already preserves overrides; new synchronized references must still address the effective policy, never a bundled full copy. | Sync then request text and JSON dialog; instruction bytes match the project replacement, its source bytes are unchanged, and actual selected entry documents name effective retrieval without bundled contrasts. | OLD_PRODUCT_RED on missing entry loading obligation, not on already-working precedence; ORACLE_RED omit that reference from one consumer entry while preserving the override. |
| Author starts a session with the universal dialog obligation | Current shared overview omits loading before the first question and fresh-delegation obligations. | Actual no-id stdout requires once-per-context effective retrieval for questions inside/outside skills and fresh handoff, while retaining `givn new` begin-change guidance; assert the applicable terminal baseline. | OLD_PRODUCT_RED on current no-change startup output; ORACLE_RED omit the loading/handoff sentence but preserve next-action text. |
| Machine-readable startup guidance retains the dialog obligation while reporting an active change | Current JSON overview lacks the obligation; existing state/next-action semantics must survive. | Fixture uses exact change id `decision-ready-json`, with a completed proposal and specs next: JSON id is that value, next command requests specs for that id, next skill is givn-spec, overview has the obligation, and no extra policy field/full contrasts are added. | OLD_PRODUCT_RED on missing overview obligation; ORACLE_RED omit only that overview obligation after a coupled edit. Identity/next-action checks discriminate unrelated active changes and wrong stage, not arbitrary nonempty JSON. |
| Design guidance requires the complete E2E setup record | Existing design content already defines setup; this is preserved input coverage, replacing a redundant E2E and source-reader contract. | Actual `instructions design` requires isolated E2E scope, undefined/pending proof, separate bindings, real driver, infrastructure, framework justification, matrix, and terminal-transcript vs browser-viewport capture. | Existing product may be BASELINE_GREEN. ORACLE_RED retrieve a temporary design override with the scope/strictness requirement omitted and rerun its real document assertions. Do not claim an old-product defect. |
| Review guidance names the deterministic disposition gates | Existing review content already defines dispositions; migrate source-reader coverage, not the product behavior. | Actual `instructions review` requires every reported overlap pair's reasoned disposition and each long scenario's split/keep, distinguishing advisory evidence from verified completion. | BASELINE_GREEN permitted; ORACLE_RED omit the per-pair or split/keep requirement in a temporary review override. A missing binding is never its behavioral RED. |
| The dialog policy is retrievable without an active change | Exit-0 general-policy availability is already implemented; the remaining old phrase assertion would pin incidental wording. | Real `instructions dialog` without change succeeds and returns the effective universal/proportional question-preparation contract without requiring change selection. Keep unknown-id nonzero `unknown artifact` behavior untouched. | Availability may be BASELINE_GREEN; OLD_PRODUCT_RED on missing explicit proportional/universal treatment, or ORACLE_RED omit only that normative clause after coupled delivery. |
| Generated guidance points to the canonical E2E policy | Old nine-reference surface is incomplete across 30 templates and contexts. | Independently expect 15 skill families in each of three selected targets and 15 commands only in opencode; every delivered entry has its own load-before-first-question/effective-policy obligation, canonical specs references remain, no full contrast copies, and actual sync report identifies success. Assert terminal baseline. | OLD_PRODUCT_RED on a currently missing entry obligation. ORACLE_RED omit one reference in a real project template replacement then sync; every expected consumer must still be checked. |
| Disabling optional addons does not disable the dialog obligation | Current addon-free defaults have no universal obligation. | Independently require the 13 enumerated core skill families in every selected target and the 13 core commands in opencode; optional families are absent, no disabled-addon vocabulary leaks into the core obligation, and exact selected targets/addon configuration are preserved. | OLD_PRODUCT_RED on a core entry without the obligation; ORACLE_RED omit one core reference while leaving optional families disabled. Expected families never come from the generator's actual list. |
| Delegated question preparation carries the policy without assuming inherited context | A top-level reference does not enter each self-contained delegated prompt. | Slice every enumerated question-preparing prompt/passage and require its own effective retrieval, fresh-context load and presenter responsibility; question-tool labels cannot stand in for consequences. | OLD_PRODUCT_RED on current nested prompt. ORACLE_RED leave the file's top-level obligation intact but omit it in one nested prompt; the prompt-local assertion must fail. |
| Successful project skill and command replacements are preserved without policy injection | Successful replacement authority already works; new defaults must not secretly augment replacements, while managed startup discovery must still work. | Use the concrete source/consumer byte contract below; source bytes unchanged, all three skill outputs and opencode command exactly equal independent rendered replacement bytes, no bundled prefix, and managed project block still loads effective policy. | Preservation may be BASELINE_GREEN; OLD_PRODUCT_RED on the current managed block's missing obligation. ORACLE_RED omit that managed reference after coupled delivery; calibrate no-injection assertion against a consumer with an added default prefix without changing production. |
| Consumer runs init non-interactively and gets opencode skills | Old addon-free managed project hint lacks the effective-policy obligation. | Actual report names opencode/disabled addons; generated managed block requires first-question loading and explicit handoff with no worked examples; human bytes stay exact; only opencode outputs are installed. Assert terminal baseline. | OLD_PRODUCT_RED against current init with the declared fixture; ORACLE_RED omit only managed reference, retaining successful install report and files. |
| Consumer re-syncs skills after init and the selection is honoured | Current sync refreshes blocks but cannot install the new obligation. | Actual report names selected guidance destinations; exactly one begin/end givn-project block replaces stale content; outside-human bytes and full saved configuration remain exact; opencode/claude outputs exist and agents outputs do not. Assert terminal baseline. | OLD_PRODUCT_RED on missing refreshed obligation. ORACLE_RED leave one stale block or omit the new reference in an isolated delivered excerpt; successful exit alone cannot pass. |
| Consumer selects agent targets during interactive init | Old E2E pipes a line and never proves the terminal prompt. Product prompt behavior stays unchanged. | Real PTY transcript shows the labeled choice and selected integrations; persisted selection is exactly opencode/claude, selected consumers exist and agents is absent; terminal baseline covers readable option labels. | DRIVER_RED: run the new prompt assertion against the pre-change piped E2E output; it has no checkbox prompt. Later script-backed helper must satisfy it. Do not claim the old product's actual PTY prompt failed. |
| Fossil initialization names the supported attribution path without Git plumbing | Existing Fossil behavior is preserved as an init precondition, not a new E2E action. | Real stdout names givn commit-only attribution; no Git hook directory is created; generated implementation guidance names the backend's commit path and the lack of raw-Fossil attribution. | BASELINE_GREEN permitted. ORACLE_RED omit the relevant delivered backend-attribution guidance excerpt and rerun that assertion, preserving the report; this is oracle calibration, not an old-product failure. |
| Initialization without SCM preserves guidance and names the later archive refusal | Existing no-SCM warning, successful generation and later refusal are retained boundaries. | Real init exits 0, core skill is delivered, stdout names archive/upgrade unavailability, and an actual later archive returns the named no-SCM failure. | BASELINE_GREEN permitted. ORACLE_RED omit the returned warning from the captured consumer transcript and rerun its warning assertion. The later-refusal boundary must use the real second command, never a dummy output. |

### Successful replacement fixture and consumer paths

Initialize with all three agent targets. Write valid LF-terminated project replacements at `givn/skills/givn-propose/SKILL.md.tmpl` (frontmatter name `givn-propose`, nonempty description, one project-owned scope instruction) and `givn/commands/givn-propose.md` (valid command frontmatter and one project-owned scope instruction). Deliberately include no bundled dialog obligation, template variables or addon conditions. Capture exact source bytes before sync. With this simple fixture the independent expected rendered bytes are the replacement bytes themselves, not a call to the production renderer.

After real `givn skills sync`, assert unchanged source bytes and exact equality of `.opencode/skills/givn-propose/SKILL.md`, `.agents/skills/givn-propose/SKILL.md`, `.claude/skills/givn-propose/SKILL.md` and `.opencode/commands/givn-propose.md` to those expected bytes. Equality rejects any hidden default prefix or suffix. No command is expected under agents/claude. Separately inspect the still-present managed `givn-project` block in AGENTS.md for effective `givn instructions dialog` loading and fresh handoff. This is the approved ownership invariant, not a mechanism to inject the policy into owner files.


## Interaction Coverage Matrix

Normalization follows the effective specs policy: an instruction id, JSON output, active change, addon set or SCM backend is an input/precondition variant, not a new action. The reviewed instruction fragment therefore has only request-by-id and session-start actions; initialization has interactive target selection, explicit noninteractive initialization and synchronization. Different capability contracts may assert different invariants over the same physical command, but do not create new commands/actions. Unaffected capability rows below are inventory references only: they were not executed or reclassified by this review.

| Capability / normalized consumer action | E2E scenario title | Real interface | Driving mechanism / evidence disposition |
|---|---|---|---|
| instruction-integrity: request effective instructions by id | An agent requests machine-readable artifact guidance | CLI | retained compiled `instructions specs --change instruction-integrity --json` subprocess; specs JSON only, not dialog evidence; design/review/dialog regular input contracts |
| instruction-integrity: request session-start instructions without id | Author starts a session with the universal dialog obligation | CLI/terminal text | compiled `givn instructions` in initialized root without change; stdout and begin-change guidance; new E2E |
| prose-consolidation: request canonical E2E guidance | Canonical E2E policy is emitted by specs instructions | CLI | retained `givn instructions specs --change ...` reference for this unaffected canonical-policy contract; no execution claimed |
| prose-consolidation: synchronize selected guidance | Generated guidance points to the canonical E2E policy | CLI and generated consumer documents | compiled `givn skills sync`; assert returned sync report and independent complete installed-family/obligation contract; modified E2E |
| initialize-a-project: select agent targets in terminal | Consumer selects agent targets during interactive init | CLI/PTY | reuse script-backed `run_init_in_terminal`, `script -qec`, key bytes to keep opencode and mark claude; assert real prompt, selection report, config and outputs; current piped-stdin driver is inadequate, not retained passing evidence |
| initialize-a-project: initialize with explicit noninteractive selection | Consumer runs init non-interactively and gets opencode skills | CLI and managed document | compiled `givn init --no-addons --agents opencode`; report plus managed obligation/human preservation; modified E2E; Fossil and no-SCM are regular precondition variants |
| initialize-a-project: resynchronize selected targets | Consumer re-syncs skills after init and the selection is honoured | CLI and managed document | compiled `givn skills sync` with stale block and surrounding human bytes; report, exact one-block refresh, preservation and selected outputs; modified E2E |
| visual-design-contract: request design guidance | Author requests the visual design contract guidance | CLI | unaffected retained `givn instructions design` inventory reference; no execution claimed |
| visual-design-contract: synchronize design/step guidance | Synchronized guidance defines the contract approval and capture mechanics | CLI/documents | unaffected retained sync inventory reference for its contract; not a new action or exercised evidence |
| change-release-note: create a change | A new change's proposal carries the release-note field | CLI/document | unaffected retained `givn new` reference; no release-note behavior change or execution claim |
| change-release-note: request proposal guidance | The proposal guidance requires a release note | CLI | unaffected retained proposal instruction input contract; no execution claimed |
| change-release-note: synchronize implementation guidance | The generated implementation guidance shares the release note | CLI/document | unaffected retained sync reference; no execution claimed |

Explicit redundant E2E removals: `An author requests the technical E2E design guidance`, `A reviewer requests disposition-aware review guidance`, `Consumer initializes a Fossil checkout and receives backend-aware guidance`, and `Consumer initializes a plain directory and receives a named warning`. Their real distinct content/precondition invariants remain regular contracts, not additional action rows. No browser, HTTP API or live agent service is introduced.

## Runner, versions and local runnability

Version evidence read on 2026-10-04: Cargo.lock pins cucumber 0.23.0, gherkin 0.16.0, and tokio 1.53.1. Local commands reported rustc/cargo 1.98.1 and cargo-nextest 0.9.143; those are observed workstation versions, not new repository pins. Keep `--locked`; add or upgrade no dependency.

Effective project commands come from `givn/commands.yaml`:

- `verify.command`: `./run-tests.sh`.
- `verify.e2e_command`: `./run-tests.sh --e2e`.
- Local product start: `cargo run --locked -- instructions` (checkout equivalent of the existing run declaration).
- Manual instruction surface from the initialized source checkout: `cargo run --locked -- instructions dialog`. For isolated fixture smoke, build once from the checkout and invoke its absolute `target/debug/givn` path with the initialized fixture root as cwd; do not try to run cargo from a fixture that has no Cargo manifest.

The current no-argument script includes removed placeholders as well as real scenarios; it does not already exclude removals and is not a `not @e2e` scope. The future runner-owned exclusion below fixes plain and nextest paths without changing the declared `./run-tests.sh` or `./run-tests.sh --e2e` commands. After that edit, the default delta command executes all 19 non-removed scenarios; `--non-e2e` executes 14 and `--e2e` executes 5. The permanent corpus runs through normal archive/post-merge verification; running it alone does not exercise this delta.

```sh
GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs ./run-tests.sh --non-e2e
GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs ./run-tests.sh --e2e
GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs ./run-tests.sh
```

Single-scenario RED/GREEN command, using the existing Cucumber name regex and no competing tag filter:

```sh
GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs cargo test --locked --test features_runner -- --name '^Author starts a session with the universal dialog obligation$'
```

Substitute the exact escaped scenario title for other scenarios. Do not combine `--name` and `--tags`: cucumber makes them mutually exclusive. The nextest custom harness also supports `<relative-feature-path>::<scenario-title> --nocapture --exact`; the cargo invocation above is the simplest targeted command.

Isolation: each scenario uses its own tempfile root; fixture helpers clear parent `GIVN_FEATURES` before invoking production commands. No server, port, database, external model service, or network dependency is needed. Therefore no digital twin or isolated service network is required. The existing full acceptance prerequisites (`git`, `fossil`, `git-filter-repo`, and `script`) remain unchanged; this change introduces none.

### Runner-owned removal exclusion (future implementation edit set)

Change only the verification runner `tests/features_runner.rs`, not Givn's public runtime architecture. Use the exported `givn::gherkin::delta::is_removed_placeholder(&Scenario)` helper, the same direct borrowed-tag check used by the tasks WIP guard. Do not allocate or parse a second feature corpus just to filter plain execution. One non-removed eligibility predicate excludes removed records independently of caller name/tag filters; all these deltas place the removal tag on Scenario, as required by merge semantics.

| Runner path | Required future edit | Proof at implementation |
|---|---|---|
| plain Cucumber main | use `filter_run` with non-removed eligibility instead of unfiltered `run`; retain normal CLI tag/name selection, `fail_on_skipped`, stats and failed/skipped nonzero exit | same delta has 19 total, 14 non-E2E or 5 E2E executions; removal placeholders create no undefined/skipped/passed result |
| nextest discovery | apply non-removed eligibility before user tag matching AND before the `--ignored` complement | removal identities are absent from both list phases for empty, regular and E2E filters; expected execution units are 19/14/5 |
| nextest exact execution | compose the same eligibility with the selected name predicate; reject an explicitly requested removed unit with a named nonzero error before Cucumber | direct removed-unit request cannot turn a zero-match run into successful behavioral evidence |
| mandatory `givn check tasks --change make-user-questions-decision-ready` | preserve current check.rs WIP guard and hooks.rs `run_verify_hook` dispatch with GIVN_FEATURES; the unchanged declared script reaches the fixed runner under nextest or cargo | after all 19 executable WIP tags are legitimately cleared, verification runs 19 units and excludes all five removals; before then the WIP guard still fails. GIVN_RESULT_FILE's forced single-process path also reports only actual units |

The repository's current corpus has no Rule blocks; mirror its feature/scenario tag semantics and do not invent a new tag protocol. No change to `src/cli/check.rs`, `src/hooks.rs`, declared verification commands or archive strictness is required. Never bind removal placeholders to dummy-success steps, inject a broad success exit, or add `--tags 'not @givn.removed'` to an invocation that already uses `--name`.

The existing three script invocations above remain exact commands after this runner edit. The anchored single-scenario cargo command remains name-only; the runner's eligibility predicate does the removal exclusion. For plain-path parity use `GIVN_TEST_RUNNER=cargo` before each script invocation; nextest remains the default. Discovery/count checks and a direct removed-unit refusal are scoped verification-fidelity proof, not implemented in this planning turn.


### Strict-mode mechanism and proof

`tests/features_runner.rs` uses `.fail_on_skipped()` in both the plain and nextest paths. The plain path also exits nonzero for failed or skipped counts. Undefined steps and `unimplemented!()` placeholders must fail; no empty step, source-text mock, or unconditional success handler is acceptable.

During implementation setup, prove undefined and deliberately pending/unimplemented steps fail with nonzero exits in regular and E2E scopes of the same runner; remove the throwaway probes immediately. These are strictness proofs, NOT behavioral RED or completed scenarios. No build/undefined-step failure can substitute for the discriminating assertions mapped below. This review executed neither probes nor scenarios.

## Visual Design Contract

- Interface: terminal.
- Viewports: not applicable to CLI text; no Web UI is introduced.
- Design system: existing plain CLI/Markdown guidance conventions; no color, animation, browser dependency or new component library.
- Surfaces covered: normal `givn instructions` stdout/JSON overview, dialog instruction text, initialization/synchronization reports, and the managed/generated Markdown delivered by those actions.
- Tokens/primitives: existing plain text and Markdown headings, lists, tables, code commands, and descriptive source links; preserve existing instruction/status labels and language. No spatial layout or pixel contract.
- Accessibility: headings identify their subject, commands are readable text, cross-references name their subject, and examples distinguish meaning from authority. The five teaching records label hypothetical facts explicitly. No new interactive controls, focus order, animation, or motion state is introduced.
- Rendered planning reference: `references/plain-dialog-reference.md` and its actual terminal render `references/plain-dialog-reference.txt`. It renders the unchanged, already-approved five hypothetical contrasts from proposal.md in the existing plain Markdown direction. D1/D2 establish policy/example content, not a newly approved visual style; no new appearance is proposed. This reference is not compiled output, operator visual sign-off, consumer execution evidence or a final transcript.
- States: fresh initialized project without a change; active change awaiting specs in JSON; addon-free init; refreshed existing managed block; project-owned effective instruction override. These are scenario fixtures, not browser visual states.
- Capture: all five surviving delta E2Es capture actual argv, cwd disposition, exit, raw stdout/stderr or the real PTY stream, and path-labeled relevant delivered excerpts after final assertions. Preserve raw output; do not pretend file excerpts were stdout. The retained request-by-id E2E also retains the normal transcript obligation when exercised for review; no existing reference is claimed passing here.
- Evidence path: `givn/changes/make-user-questions-decision-ready/evidence/visual/<scenario-slug>/transcript.txt`, with the existing `givn::integrity_hook::scenario_slug` rule. The separate planning reference lives outside `evidence/visual` and cannot satisfy consumer evidence completeness. Terminal evidence needs no viewport screenshot.
- Persona review: all three confirmed Personas above apply. Normal review must record actual `VISUAL-REVIEW` and `PERSONAS:` evidence; the technical-role Persona cannot provide human prose/domain/visual judgment. Existing confirmed identities require no re-promotion, but future human judgment is still mandatory. There is no approved deferral, no manufactured approval and no new semantic/vision gate.

The opposite classification would be Web UI if Givn rendered HTML/CSS/JavaScript in a browser. These actions print text/JSON and write Markdown, with no browser renderer, page or endpoint; later reading Markdown in an editor does not change that boundary. Noninteractive changed actions need no PTY. Interactive selection MUST use the existing script-backed terminal helper, not its current line-piped E2E binding. Static content has no new motion, focus, hover or breakpoint behavior; the explicit applicability/assertion table below is not a deferral.

### Applicable terminal visual baseline assertions

Run the applicable baseline assertion helper from each surviving changed E2E Then sequence against each observed report/prompt and each path-labeled changed consumer document, before final transcript capture. This is planned assertion glue in the separate E2E modules, not an extra policy engine or source-text test.

| Canonical baseline | Applicability and concrete assertion |
|---|---|
| Visible control labels | Static noninteractive stdout/Markdown adds no controls. The interactive-selection E2E must see the meaningful target-selection prompt and readable opencode/agents/claude labels in its PTY stream; no blank/opaque option labels. |
| No internal identifiers/state names as visible labels | Human-readable sections identify their decision/authority in domain language, not just D1/UC1 codes; assert subject headings and descriptive record links. Public commands, paths and JSON ids are intentional machine/traceability content, not banned internal state labels. |
| Real title and heading | Normal startup retains the project/workflow subject and begin-change guidance; instruction/consumer documents have named policy/skill headings; init retains its installation/ready headings and sync its skills/commands report heading. Inspect actual delivered sections, not a full-output snapshot. |
| No duplicated visible content | Require exactly one managed givn-project block, one obligation per entry/prompt context rather than repeated full rules, each supplied record/goal row once, and no copied full worked contrasts in entry guidance. Legitimate reports/paths in separate actions are not global duplicates. |
| Error heading and way back | The five delta E2Es are successful paths with no new error state. Existing prompt empty-selection regular coverage keeps a named target requirement and re-prompt; no-SCM regular coverage keeps named warning/refusal and version-control prerequisite. Existing unknown-id errors remain intact. No error-path waiver is inferred from success. |
| Configured language | This contract retains the current English CLI/Markdown language. Returned headings, obligation and descriptive links use English; JSON keeps its public field names. Add no locale setting or translation subsystem. |
| Reduced motion, focus, hover, contrast, breakpoints | No new animation, color, browser layout, control or keyboard-focus behavior is introduced. Static terminal/Markdown content has no prefers-reduced-motion/focus/hover/viewport mechanism to assert; plain text uses the terminal's existing contrast. The retained PTY action uses existing arrow/space/enter behavior and observes selection. These checks are nonapplicable to the changed static content, not deferred. |

PTY implementation reuses the existing util-linux `script -qec` helper, including its 60-second deadline and propagated child exit status. Make its mechanism reusable from the separate selection E2E binding without migrating piped regular-input behavior. With the verified order opencode, agents, claude and only opencode premarked, send `\x1b[B\x1b[B\x20\n` to mark claude and confirm, not the string `opencode, .claude`. PTY output is one terminal stream; do not claim separate stdout/stderr fidelity from its duplicated world fields. Capture its final raw transcript plus selected configuration after the assertions.


## Fresh-context behavioral verification

Distribution checks establish that an agent can find the effective policy and that its published records/links/obligations are intact. They do not establish arbitrary model compliance.

At the implementation/review checkpoint, use fresh authoring-agent contexts in the available harness with the actual delivered policy and unseen decision situations. Human reviewers may evaluate the resulting presentations. Do not give authors prior verdicts or the teaching examples as tasks to repeat. Include these distinct unseen probes:

1. A delivery-planning approval over four explicitly supplied shipping goals, including one read-only goal; the author must show the actual goals and bound adoption versus revision, not approve a count.
2. Granting a remote auditor access to already published customer records; supply the access constraints and facts needed for decision, including that revocation cannot make previously disclosed information unseen. The author must explain the access benefit, consequences and material limit, not invent a reversible disclosure.
3. A test-oracle issue whose contract and concrete counterexample are supplied and already settle the remedy; the author should report the evidence-based action rather than ask a preference question. Add a genuine missing human event date separately to verify a short clarification without manufactured options.

The reviewer assumes project familiarity but not omniscience. For each actual presentation, record clarity, brief relevant context, usable source, consequence examples for every genuine answer, material revision cost/benefit, and whether a question is necessary. Record concrete defects and their disposition in normal review evidence. A formatting checklist, an exact example match, or a numeric model score is not a semantic pass.

This is a required authoring/review exercise, not a mandatory online service, permanent model-dependent test, added parser gate, or guarantee of model behavior. If an example is clear but violates an invariant, it fails this review. If real source, compatibility or authority facts are unknown, the author must say so and research or narrow the requested authorization.

## Architecture impact and ADR classification

Existing accepted boundaries remain: ADR-005 (project facet precedence), ADR-013 (selected guidance and fenced-block ownership), ADR-048 (effective instruction/metadata contract), ADR-011/015 (Gherkin and declared runners), and ADR-069 (feature-free binary without model dependencies).

No new ADR or amendment is required. The complete question-policy rationale belongs in the canonical dialog instruction; distribution mechanics belong here. No component, authority, dependency, data migration, JSON schema or interception boundary moves. The three independent non-qualification verdicts and twelve-row comparison are recorded in design-review.md; arc42.md keeps the author assessment. Durable chapters 1, 3, 5, 6, 7, 8, 10, 11 and the existing glossary receive narrow pending target notes; chapters 2, 4 and 9 retain constraints, strategy and accepted records. Row 1's original author No is corrected without reprioritizing goals; row 7's independent No/author Yes remains a compatible broader content-adoption note.

## Risks and implementation sequencing

- References are not compliance: keep deterministic and behavioral evidence distinct.
- Stale project overrides are owner-controlled: do not claim sync updates them or hide an injected preamble.
- Redundant partial rules can still defeat proportional clarification: normalize those touched surfaces, including the optional tone policy, rather than just adding another top-level sentence.
- Broken in-file links or fictional facts can undermine the teaching examples: retain records/anchors and hypothesis labels.
- All addon-free generated entries must omit disabled-addon vocabulary; the universal obligation names no addon and stays outside conditionals.
- A stale installed binary can make consumer smoke appear unchanged: use the rebuilt checkout artifact.
- Generic exit-success/file-existence assertions produce advisory subset matches against unrelated permanent scenarios. Final hardening lint records the actual findings in design-review.md. Keep meaningful report/preservation assertions and disposition every reported pair at review by its distinct trigger/invariant; do not suppress warnings or rewrite unrelated scenarios. No later review gate is predeclared passed.

Implementation order: canonical policy and meaningful retrieval/record assertions; unconditional hint and startup overview; complete template/delegation references and their override/addon variants; init/sync preservation and transcript capture; unseen fresh-context review. One atomic scenario commit follows the repository's RED/GREEN/REFACTOR requirements. Do not write tasks from this design before fresh design-review sign-off.

## Planning verification performed

- Resolved and read the actual specs/design/arc42-docs instructions for this change.
- Inspected current production generation/cascade/startup paths and strict-runner/source configuration, with two read-only scout handoffs.
- Executed real `givn init --help`, current startup JSON and local tool version commands to ground option/metadata/version statements.
- Earlier author lint parsed the three feature files with twelve WIP findings. Independent grilling and hardening corrected the delta to 24 records/19 executable scenarios; the final hardening lint result is recorded in design-review.md. No runtime scenario, build or strictness probe result is claimed.

Design-review sign-off covers the hardened plan only. Tasks and implementation follow that sign-off; production delivery, final transcripts, actual Persona/visual review and unseen fresh-context question evidence remain mandatory lifecycle work, not waived checks.
