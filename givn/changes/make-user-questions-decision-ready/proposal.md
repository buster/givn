# Proposal: make-user-questions-decision-ready

## Release note

Givn guides every user-facing question through one canonical dialog policy with brief sourced context, concrete answer consequences, material change costs, and multiple bad/good examples.

## Use-Case Context

- Use-case IDs: `create-change` (primary), `initialize-project` (supporting).
- Permanent contracts: `givn/specs/create-change/usecase.md` and `givn/specs/initialize-project/usecase.md`.
- Ideation topic: none — direct operator request following question simulations and independent GPT-6.1 Sol assessments.
- Seed path: none.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`; reuse the existing confirmed project records without creating or promoting personas.
- Persona impact: the developer can give informed answers without memorizing all artifacts; the authoring agent can find the effective policy at startup, direct skill invocation, and delegation; the maintainer has one policy to change rather than competing copies.

## Problem / Opportunity

The current dialog policy is discoverable through `givn instructions dialog`, but only some skills and commands direct agents to it. Session-start guidance does not explicitly require it for every user-facing question. Entering a different skill or a fresh delegated context can therefore omit the interaction contract.

Questions can count a set of user goals without showing those goals, cite a decision without explaining its relevant effect, offer implementation labels without outcomes, or solicit consent without saying what the answer authorizes. A project-familiar reader still lacks the information necessary to decide. Conversely, agents can ask the human to research facts or revisit decisions the project already settles.

The recent simulations deliberately sampled weak and strong question forms. Three independent GPT-6.1 Sol reviewers agreed on 21 of 23 overall verdicts; none met the complete revised rubric. This is evidence about those presentations, not a measured defect rate in live Givn sessions or proof that agents spontaneously obey a policy. Clear partial examples also exposed invalid alternatives and omitted migration commitments.

Decision sources: the operator's request to apply the rules to every user-facing question, approval of option 3, and request for multi-shot examples in this conversation. Evaluation evidence: `/tmp/givn-question-sim/reassessment-summary-en.md`; this temporary report is supplementary evidence, not a required dependency or the canonical policy.

## Proposed Solution

### One canonical policy, visible at every entry point

Adopt option 3: the generated `AGENTS.md` project guidance requires the effective dialog policy, and every available Givn skill and command references that same policy. The normal `givn instructions` startup guidance reinforces the obligation. The policy remains retrievable through `givn instructions dialog`, including before a change exists.

The obligation is unconditional: it applies with optional addons disabled and regardless of workflow phase, direct skill invocation, clarification, confirmation, review, or delegation. It includes questions an agent asks while working in a Givn project outside a named skill. Optional tone policies must not redefine or weaken it.

An agent loads the effective policy once per context, before its first user-facing question, rather than repeatedly retrieving it before every question. Direct skill guidance requires loading it if that context has not already done so. Delegated agents receive the same obligation explicitly; the agent presenting a delegated question remains responsible for its quality. No inheritance of the parent's loaded context is assumed.

Generated project guidance is refreshed through normal initialization and skill synchronization, while unrelated human-authored guidance is preserved. Detailed rules and multi-shot examples live only in the effective dialog policy; project, skill, command, and delegation guidance carry short consistent references rather than competing full policies. Existing project instruction, skill, and command overrides remain authoritative and are not overwritten, secretly augmented, or bypassed by this change. Bundled defaults supply the universal references; owners must reconcile stale replacement templates with the same authoring obligation. Synchronization alone must not be represented as updating an override.

### Decision-ready questions for project-familiar readers

The canonical policy requires:

1. **Show the decision subject.** Display the actual goals, relevant changes, boundaries, findings, or actions being approved. Counts, ID ranges, document existence, and generic directories do not stand in for those contents. Name artifacts in domain language, with IDs as optional traceability aids.
2. **Explain the context briefly.** State why the choice matters now and what it blocks. Assume the project's purpose, ordinary vocabulary, and basic rules are known; explain decision-relevant artifact contents, not the whole project.
3. **Provide a useful detail source.** Point to the relevant existing document and section, rule, scenario, or finding. A source supplements the decision-critical facts shown in the question. If no recorded source exists, state that honestly and show the assumptions; never fabricate a citation.
4. **Make every answer consequential.** For each genuine alternative, explain what it means, its benefit and cost, and a concrete example of what choosing it causes. Confirmations show accept and reject/revise outcomes, including what work or authority is and is not authorized.
5. **Disclose material later-change costs.** Prefer revisable choices where suitable. If a decision changes stored information, public expectations, access, or substantial downstream work, explain the constraints and work involved in changing it later. Name the benefit purchased by a hard-to-change commitment. State unknown recovery or compatibility facts before consent; never invent rollback guarantees. Do not add ceremonial reversibility prose to an ordinary local choice with no material commitment.
6. **Keep questions necessary and independent.** Apply settled rules and research discoverable facts before escalating. Ask only unresolved human goals, priorities, authority, or tradeoffs. Do not re-ask confirmed directions without a new reason. Separate independent decisions, avoid false dilemmas, and exclude alternatives that silently violate a confirmed constraint.
7. **Match the treatment to the question.** A short information request explains how the answer will be used and its immediate effect; it does not invent an option menu or artificial benefits. A substantive choice receives the full treatment above. Ask one independent question, wait, and retain the selected and rejected alternatives with reasons when recording a decision.

The contract governs text questions and question-tool presentations alike. A tool's labels, file links, or recommendation chip do not replace the context and consequences the person needs before answering. An agent checks its question against the effective policy before presenting it; this authoring obligation is not advertised as runtime enforcement.

### Multi-shot examples in the canonical dialog instructions

Publish the five complete contrasting examples below with the policy. They illustrate different question shapes, not a mandatory verbose template. Each consists of a scenario record, a bad presentation, a good presentation or a non-question report, and the reason for the contrast.

The examples are explicitly hypothetical. Their local source links identify the scenario record supplied with the example, not a real project file, current product behavior, implemented recovery mechanism, or new project decision. Their records and local links must remain intact when published in the canonical dialog instructions. Preserve the existing full decision-record/rejected-alternative example as a separate record-shape illustration; do not re-pin its incidental wording.

### Acceptance criteria

- Session-start project guidance and the normal Givn startup instructions direct an agent to the effective dialog policy before any user-facing question, independent of optional tone addons.
- Every available bundled-default skill and command has the same effective-policy obligation, including reference skills and optional skills when enabled; no skill-specific exception narrows it. Project-owned replacements retain the override-preservation boundary above.
- Delegation guidance explicitly passes the obligation to fresh contexts and retains responsibility with the presenting agent.
- The canonical dialog instructions contain the audience definition, necessary-question rules, brief sourced context, answer consequences, proportional treatment, material revision costs, and all five worked contrasts below.
- The list-confirmation example shows every proposed goal; the migration example names the hard-to-change commitment and its purchased advantage; the clarification example asks for information without an artificial option list; the diagnostic example replaces an unnecessary preference question with evidence-based action.
- Existing rejected-alternative record semantics and effective project overrides remain intact. No new gate claims to enforce free-form chat.
- Verification distinguishes distribution from behavior: inspect real CLI/generated consumer surfaces for policy availability, then use fresh agent contexts with unseen decision situations to evaluate the presentations. Repeating the teaching examples or merely matching their wording is not evidence of general question quality. No model-specific test dependency, online service, numerical model-score guarantee, or live-chat interception is introduced.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| instruction-integrity | `EXTEND prose-consolidation` (create-change, signal; proposal-wide recommendation) | `EXTEND instruction-integrity` | Reuse the existing instruction fragment for the retrieved canonical dialog policy and its question contract; generated-policy distribution remains under prose-consolidation. The proposal-wide recommendation favors distribution, while the already-existing instruction fragment owns effective retrieval semantics. |
| prose-consolidation | `EXTEND prose-consolidation` (create-change, signal; score 20.50) | `EXTEND prose-consolidation` | Follow the recommendation: the existing capability already makes generated interactive guidance point at the canonical dialog policy; broaden that obligation to every skill, command, and delegated context. |
| initialize-a-project | `EXTEND prose-consolidation` (create-change, signal; proposal-wide recommendation) | `EXTEND initialize-a-project` | Initialization and resynchronization belong to the existing initialize-project actor goal, not the create-change goal. The route also lists initialize-a-project (score 14.00). Extend its unconditional project-start obligation rather than inventing a new initialization capability. |

## Out of Scope

- Full policy copies in `AGENTS.md`, every skill, or every command.
- A new question API, structured-question runtime gate, model provider, or interception of arbitrary agent chat.
- A claim that policy references guarantee compliance.
- Changing an initialized project's selected addons or agent targets without its owner's request.
- Overwriting project instruction overrides or unrelated project guidance.
- Rewriting historic project decisions, adopting hypothetical example decisions, or changing archive gates and verification requirements.
- New mandatory artifact types, new persona identities, new glossary terminology, or a new architecture decision solely to store these question rules.
- Product behavior described inside the teaching examples, including migrations or compatibility support.
- Implementing and deploying the guidance in this proposal step; specifications, design, review, and implementation follow through the normal workflow.

## Open Questions

No unresolved operator decision blocks specification. The following entries record decisions already made; they are not live questions to ask again.

### D1 — One canonical dialog policy with universal entry-point references

- Context: agents enter through session startup, direct skills, and fresh delegated contexts. One isolated policy is easy to miss; several full copies can disagree.
- Question: where should the universal user-question contract live, and how should every context discover it?
- Options:
  1. **Full policy in project guidance only** — meaning: the startup file carries all rules; benefit: visible at startup; cost: direct skills and fresh agents can miss it. In practice: an agent starting directly with review has no explicit policy-loading instruction in that skill.
  2. **Dialog instructions only** — meaning: maintain one retrievable policy without universal pointers; benefit: one owner; cost: missing entry points remain. In practice: a directly invoked characterization skill can proceed without loading it.
  3. **Canonical dialog policy plus universal references** — meaning: project startup, skills, commands, and delegated contexts require the effective policy; benefit: one maintained contract with broad discovery; cost: agent compliance still requires behavioral review. In practice: a fresh review agent loads `givn instructions dialog` before drafting a disposition question, without receiving a separate copy of the rules.
  4. **Full copies everywhere** — meaning: every entry point includes all rules; benefit: each is self-contained; cost: copies drift and make amendments expensive. In practice: updating the migration-consent rule requires changing many policy copies.
  5. **Runtime question interception** — meaning: a new interface tries to reject incomplete questions; benefit: structural enforcement at that interface; cost: arbitrary chat remains outside it and filled fields do not prove understandable consequences. In practice: a question can have all required fields while still asking the human to approve an unseen goal list.
- Disposition: answered — the operator selected option 3 in this conversation. Reject options 1 and 2 because they leave discovery gaps; reject option 4 because it duplicates the contract; exclude option 5 because it is a separate architecture change and cannot guarantee prose quality. Detailed realization belongs to design; the scope decision is settled.

### D2 — Multi-shot contrasts and proportional question treatment

- Context: one generic question-record example does not distinguish bulk approval, small clarification, behavioral choices, material commitments, and unnecessary engineering questions.
- Question: should the canonical policy teach several contrasting situations or rely only on abstract rules and one universal form?
- Options:
  1. **Multiple worked bad/good contrasts** — meaning: the policy teaches the five question shapes below alongside the rules; benefit: shows what the answer changes and when not to ask; cost: a longer canonical instruction. In practice: an agent sees why counting goals is not consent and why a directory clarification does not need artificial gains and costs.
  2. **Rules only with one universal question form** — meaning: omit the contrasting demonstrations; benefit: shorter policy; cost: leaves room for superficially formatted but uninformed questions and excessive boilerplate. In practice: an agent supplies option labels and examples yet overlooks that one option violates an invariant.
- Disposition: answered — the operator requested multi-shot examples in this conversation. Choose option 1; reject option 2 because the assessed questions demonstrate the need for concrete contrasting treatment. Keep examples in the canonical policy rather than repeating them in each entry point.

## Multi-shot examples for publication in the dialog instructions

### Example 1 decision record

Hypothetical neighborhood-library planning record: three proposed member goals are reserve an available copy, cancel one's own reservation, and view one's queue position. Staff acquisition and catalog editing are excluded. Confirmation authorizes elaborating these goals, not implementation, change count, or delivery order. Changing a goal later requires revisiting its affected elaboration, not treating the original confirmation as irrevocable.

#### Bad — approving a count instead of the goals

> The cut contains two changing goals and one observing goal. It is not three changes or a delivery order. Adopt this cut for elaboration?

#### Good — show the whole approval subject

> Should these three member goals become the basis for use-case elaboration?
>
> We need their boundaries before drafting flows. Details: [the planning record](#example-1-decision-record).
>
> | Proposed goal | Member outcome | Boundary |
> |---|---|---|
> | Reserve an available copy | A copy is reserved for the member. | Does not acquire new books. |
> | Cancel one's own reservation | The member's reservation is removed. | Does not cancel another member's reservation. |
> | View one's queue position | The member can see their position. | Read-only; does not edit the catalog. |
>
> **Adopt this cut:** elaborate these three goals with those boundaries. For example, canceling another member's reservation is not added to the cancel flow. Benefit: a bounded basis for elaboration; cost: omitted goals need separate consideration. This does not approve implementation or delivery order.
>
> **Revise the cut first:** identify a goal or boundary to change before elaborating it. For example, if staff cancellation is needed, settle its actor and outcome first. Benefit: avoids elaborating the wrong scope; cost: delays the affected flow.
>
> Goals can be amended later, but affected rules and flows must be reviewed again. Adopt these goals or revise a named boundary first?

Contrast: all approved goals are visible; the count is not the evidence, and both answers have bounded consequences.

### Example 2 decision record

Hypothetical target-selection record: both `/work/catalog` and `/work/catalog-tools` are initialized projects the operator owns. The repository inspection identifies both but cannot establish which project the operator intends to work on. Selecting one sets the scope for subsequent commands; selecting it alone writes nothing. Neither current-directory inference nor another artifact settles the intent.

#### Bad — an unexplained information request

> Which directory?

#### Good — short clarification, no artificial option ceremony

> Both `/work/catalog` and `/work/catalog-tools` are initialized projects; inspection cannot tell which one you intend to change. Details: [the inspected target record](#example-2-decision-record).
>
> Which project root should subsequent commands use? For example, answering `/work/catalog` limits the work to the catalog project, not catalog-tools. Selecting a root alone writes nothing; we can switch before any write.

Contrast: the agent establishes discoverable facts first, asks only the missing human intent, and explains how the answer will be used without invented gains or costs for arbitrary paths.

### Example 3 decision record

Hypothetical rule-reader project: general collaboration guidance can be read through `tool instructions dialog`; task-specific guidance is read through `tool instructions task`. No decision yet settles whether general guidance requires an active task. Task-specific guidance does require one. The draft rejects all instruction requests when no task exists. No data migration or permission change is involved. Altering an established command expectation later may require updating callers and guidance.

#### Bad — a preference poll about implementation labels

> D1 is open. Fallback first or task resolution first? Recommend fallback.

#### Good — decide observable behavior

> Should general collaboration rules be readable before an active task exists?
>
> The draft rejects `tool instructions dialog` with `No active task`, which prevents an agent from reading the collaboration rules before planning. Details: [the draft behavior and boundary](#example-3-decision-record).
>
> **Allow general guidance without a task — recommended.** The command prints the collaboration rules; task-specific instructions still require a task. Example: in a fresh project, `tool instructions dialog` prints the rules while `tool instructions task` returns `No active task`. Benefit: rules are available before planning. Cost: lookup must distinguish general from task-specific guidance.
>
> **Require a task for both.** Example: both commands return `No active task` until the operator creates or selects a task. Benefit: one prerequisite for all lookup. Cost: the agent cannot retrieve the rules through this command before planning.
>
> No stored information changes. The behavior is revisable, but callers and guidance relying on it may need updates. Which availability rule should the project establish?

Contrast: the reader decides a real product expectation, not an internal ordering detail. The source is specific, both results are shown, and the local choice does not receive migration-style rollback ceremony. If the project already confirms availability, report and apply that decision instead of asking again.

### Example 4 decision record

Hypothetical saved-project upgrade: the old format records a preview setting called Backend Port; the proposed format calls the same setting Candidate Port. The port value is preserved. Under this example's compatibility assumptions, old releases cannot open the new format. Downgrading an upgraded project therefore needs a separately verified reverse conversion; merely reinstalling an old release is insufficient. No implemented recovery support is assumed. If approval includes upgrading saved projects, verifying forward conversion and recovery is part of the approved work before rollout, not a claim that recovery already works.

#### Bad — hiding a lasting commitment in a label

> Include the saved-format rename now or later? Recommend now for consistency.

#### Good — show the hard-to-change cost and its purchased benefit

> Should this terminology change include upgrading existing saved projects?
>
> Documentation will call the preview setting Candidate Port. We must choose whether saved projects change with it. Details: [the upgrade and compatibility record](#example-4-decision-record).
>
> **Upgrade saved projects in this change.** Example: a project saved as Backend Port = 8080 becomes Candidate Port = 8080; the number stays the same. Benefit: documentation and saved projects use one name, so future work does not translate between them. Cost: implement and verify conversion and recovery before rollout. Under the stated assumptions, older releases cannot open upgraded projects. Changing course after rollout requires verified reverse conversion, not just reinstalling the old release. Such recovery is required work, not demonstrated support.
>
> **Keep saved projects unchanged for now.** Example: documentation says Candidate Port = 8080 while the saved project still calls it Backend Port. Benefit: this change does not introduce that format-related downgrade restriction. Cost: the project must maintain the documented distinction; later alignment still needs a conversion.
>
> Both options preserve the configured port. The second leaves conversion open; the first purchases immediate consistency with a harder-to-reverse format commitment. Include the conversion and its required recovery verification, or leave saved projects out of this change?

Contrast: the difficult option states its benefit and compatibility cost; the safer option is concrete. The example does not promise an unimplemented rollback. If real compatibility or recovery constraints are unknown, establish them or narrow approval to investigation before asking for rollout consent.

### Example 5 decision record

Hypothetical review record: a status scenario succeeds whenever the output contains the word active. Its declared requirement is that the project status is active. A response saying the project is inactive but an active-profile label exists still passes the substring check. The recorded contract already requires checking the actual project status. The assertion weakness and remedy are discoverable; no policy choice remains. A separate confirmed rule also prohibits processing one repeated write as two answers.

#### Bad — delegating diagnosis or offering an invalid option

> Which of the 22 status scenarios proves nothing? Should we keep the broad substring check or choose exact whole-output matching?
>
> Alternatively, should a repeated answer become a second answer so a human can decide? The run might continue twice.

#### Good — do the research and report; do not ask a preference question

> The status check accepts an inactive project when an unrelated label contains active. Details: [the assertion counterexample and status contract](#example-5-decision-record).
>
> I will make the assertion distinguish the actual project status while preserving the existing output contract. The inactive-project counterexample will fail; an active project with the required status will pass. This changes verification, not product behavior. Whole-output formatting is not required by the supplied contract.
>
> I will not offer processing a repeated write as a second answer: that would violate the confirmed one-answer rule. If a future request intentionally changes that rule, it needs a separate explicit requirement decision and its consequences, not a silent exception.

Contrast: investigate unseen assertions instead of asking the human to memorize them; reject a false dilemma and an invariant-violating alternative. The policy's good outcome is sometimes a report with no user-facing question.
