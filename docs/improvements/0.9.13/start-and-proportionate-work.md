# Replace explore with start and route work proportionately

## Status and purpose

Discussion handoff recorded on 2026-10-30 for implementation in a fresh context. The intended improvement target is Givn 0.9.12; the directory is not a release commitment.

This document preserves the confirmed behavior, rationale, rejected alternatives, integration evidence, and verification expectations. It is not a formal proposal, executable specification, technical design, or authorization to implement. The user authorized creating this document only. No new Givn change was started.

When this discussion began, `givn instructions` reported the existing change `prepare-0-9-10` as verification-pending and ready to archive. That was unrelated to this discussion. Recheck current state in the future session; do not attach this improvement to an existing change merely because it is active.

Before future work:

1. Run `givn instructions` as the first action of the session.
2. Interpret the returned state in relation to the user's current request, not as authorization to resume unrelated work.
3. Load `givn instructions dialog` before asking questions and the applicable examples from `givn instructions dialog-examples` before the first question of each indexed type.
4. Use the applicable domain-modeling guidance. Do not create artifacts or alter the glossary merely to resume this discussion.
5. Preserve the decisions below; do not re-ask them without new evidence requiring reconsideration.

## Problem

Users currently have to choose among overlapping entry points before discussing their problem:

- `givn-explore` handles vague ideas, investigation, and approach comparisons without artifacts.
- `givn-ideation` also starts from vague ideas, but performs structured, persistent domain work.
- `givn-propose` creates a formal change and develops proposal, specifications, and design.

This requires knowing Givn's internal distinctions before determining what work is justified. Renaming explore without changing its responsibility would not resolve the confusion.

There is a second problem: current onboarding and next-step guidance tends to funnel work into a formal change. A bounded bugfix or small documentation correction can cost more time and agent usage in workflow overhead than the artifacts contribute. Givn needs an explicit path for useful project modifications without creating a Givn change.

There is a third problem: the mandatory session-start command can sound like an unconditional assignment. `givn instructions` must establish state and applicable guidance without enrolling every request into the lifecycle or hijacking a request with an unrelated active change.

## Confirmed decisions

### Name and invocation surface

Resolution: replace `givn-explore` with `givn-start`. Remove the old explore entry point; retain no compatibility alias or separately maintained explore skill.

`start` means help determine what work, if any, is justified. It is useful for an existing project's twentieth improvement, not only initial onboarding.

Rejected:

- `givn-quickstart`: suggests installation, first-time onboarding, or a shortened lifecycle.
- Keeping `givn-explore` as an alias: the user explicitly rejected retaining it; overlapping names preserve confusion.
- Merely renaming the existing skill: the routing responsibility and proportionality principle must also change.

Skills remain the agent invocation surface. `givn-start` is not a new shell subcommand named `givn start`. Preserve the harness-specific invocation conventions used by the project.

### Responsibility of start

Resolution: investigate the user's problem, question, or improvement; clarify the intended outcome; recommend a proportionate next step.

Preserve the useful exploration behavior: inspect the existing project, compare approaches, challenge assumptions, discuss uncertainties, and allow the conclusion that no modification is warranted.

Start itself produces no workflow artifacts and makes no application changes. It does not create a change directory, topic state, use case, proposal, specification, design, task list, or archive record. Once the user authorizes a destination, start hands off; work performed there is not an artifact-producing phase inside start.

Rejected:

- A mandatory intake questionnaire that immediately asks the user to classify their work. The agent should investigate and explain the recommendation rather than relocate the existing confusion into a menu.
- A mandatory phase before every task. Experienced users may enter the appropriate workflow directly, and clearly authorized direct work need not undergo another routing conversation.
- Assuming every project modification needs a formal Givn change.

### Four outcomes

Resolution: start can recommend any of these outcomes, with none treated as a failure to adopt Givn.

| Outcome | Appropriate when | Concrete example |
|---|---|---|
| Answer and stop | Investigation answers the question, or establishes that no worthwhile modification is justified. | Explain an existing configuration option; reject a proposed change whose complexity is not justified by its benefit. |
| Direct work without a Givn change | The work is sufficiently understood and bounded that the formal lifecycle costs more than it contributes. | Correct a documentation typo; restore an existing specified behavior with a focused regression test. |
| `givn-ideation` | Domain understanding needs durable, structured development before selecting changes. | Establish actors, rules, terminology, and boundaries for team approvals. |
| `givn-propose` | A concrete improvement warrants the specification-driven lifecycle. | Introduce a new archive recovery behavior with observable guarantees. |

Answering without modifying anything and modifying the project without a formal change are distinct outcomes. Both must be explicit in the guidance.

### Proportionality, not a size threshold

Resolution: judge what the formal workflow buys for the actual work and whether that benefit justifies its cost.

Possible benefits: clarifying disputed behavior, establishing a durable contract, coordinating dependent work, exposing consequential decisions to review, and preserving useful implementation evidence.

Costs: artifact production and maintenance, review effort, latency, and agent usage/time/money.

Vagueness alone does not require ideation. A vague request can become a clear, bounded correction after inspecting the code. Ideation is warranted by domain work that needs to persist, not by how polished the initial request is.

Propose does not require every requirement or design decision to be settled beforehand: developing these is part of its existing responsibility.

A label is not an exemption. A one-line permissions fix may warrant substantially more scrutiny than a larger documentation correction. Do not route solely by line count, filename extension, or the user's use of the word "bugfix".

Rejected: numerical routing scores, exemption registries, a mandatory lightweight-change artifact, and a second miniature lifecycle. These recreate the overhead the direct-work outcome is intended to avoid.

### Direct work retains engineering obligations

Resolution: direct work skips formal change administration, not correctness or applicable project rules.

For direct work:

- Read relevant code, existing specifications, documentation, and the applicable contract.
- Make a focused correction within the authorized scope.
- Add or adjust regression coverage where appropriate and run proportionate checks.
- Keep existing specifications and documentation accurate; skipping a new change directory does not permit contract drift.
- Report the modification and verification evidence, including material limits.
- Do not create a proposal, design, task list, or archive ceremony solely to justify taking this path.
- Do not interpret authorization to edit as authorization to commit, push, archive, or perform unrelated work.

If investigation exposes disputed behavior, consequential scope, or domain uncertainty, report that evidence and recommend reconsidering the path. Do not silently promote a direct fix into a formal change.

### Recommendation and authorization

Resolution: recommend the destination and wait unless the user's existing request already authorizes continuation.

Example recommendation:

> The documented behavior is clear, and the defect is isolated to argument handling. I recommend a direct fix with a regression test, without creating a Givn change.

An ordinary request to discuss an improvement is not authorization to create a formal change, persist ideation state, or implement a direct fix.

An explicit request such as "discuss this, then create the proposal" can authorize the stated continuation without a redundant handoff confirmation. It does not authorize unrelated actions or waive questions about genuinely unresolved consequential decisions.

Rejected: automatically entering whichever workflow the agent selects after an ordinary start discussion. It saves an exchange but can turn discussion into durable work without consent.

### Direct entry remains available

Resolution: retain direct access to `givn-ideation` and `givn-propose`, as well as existing change continuation. Start is a default for new or unclassified work, not a gate placed in front of every skill.

A clear, applicable request to perform a bounded direct correction can also proceed without another start conversation. If the request is not sufficiently understood or its path is uncertain, investigate and recommend rather than guessing.

## Relationship with mandatory `givn instructions`

Resolution: keep `givn instructions` as the mandatory first action of every session. Separate state discovery from work selection.

- `givn instructions` establishes project state and applicable guidance, including the next lifecycle step of existing changes.
- `givn-start` helps select the appropriate path for the user's current request when that path is not already established.
- Reporting a lifecycle step does not authorize taking it.
- An active change does not imply that all subsequent requests belong to that change.
- No active change does not imply that a new one must be created.

### Session behavior

1. Run `givn instructions`.
2. Relate its state to the request:
   - Explicit continuation of an existing change: follow that change's next step.
   - New or unclassified work: use `givn-start`.
   - Explicit, applicable ideation or proposal request: enter that skill directly.
   - Clearly authorized and sufficiently understood direct work: proceed without creating a Givn change; investigate or use start when the appropriate scope/path is uncertain.
3. Respect the authorization boundary of the selected path.

This does not weaken lifecycle gates for work already being conducted as a formal change. It prevents lifecycle enrollment from being inferred from a state check.

### Existing change must not hijack a request

Concrete example from this conversation: `givn instructions` reported that `prepare-0-9-10` was ready to archive, while the user asked to discuss a different improvement. The correct action was discussion, not archiving the unrelated change.

Suggested output meaning, not a fixed literal or CLI schema:

> Existing change: `prepare-0-9-10`. Its next lifecycle step is archive.
> For new or unclassified work, use `givn-start`. This status does not authorize resuming an unrelated change.

When no active change exists, the guidance should point new/unclassified work toward start and explicitly allow outcomes with no formal change. It must not default to mandatory `givn new` or proposal creation.

Pending verification and archive recovery state must still be reported truthfully. These obligations are not erased by choosing to discuss unrelated work.

### Suggested generated agent guidance

> Run `givn instructions` as the first action of every session to load project state and workflow guidance. The reported next step applies when continuing the corresponding change; it does not override the user's current request or require every task to become a Givn change. For new or unclassified work, use `givn-start` to recommend discussion only, direct work, ideation, or a formal change. Honor an explicitly requested, applicable path without requiring another routing conversation.

The source template must change; do not hand-edit generated AGENTS.md blocks as the implementation.

## Suggested start instruction opening

> Investigate the problem, question, or improvement and recommend a proportionate next step: answer and stop, direct work without a Givn change, structured ideation, or a formal change. Do not assume that modifying the project requires a Givn change. Explain the recommendation briefly and wait unless continuation is already authorized. Create no workflow artifacts during this discussion.

This text is a drafting basis, not a requirement for byte-for-byte wording. The effective dialog policy applies to start, its recommendations and confirmation questions, and work outside a named workflow.

## Integration evidence and implementation entry points

References were inspected on 2026-10-30. Recheck locations and current behavior in the future checkout. This is an investigation map, not a completed technical design or exhaustive file inventory.

### Skill and instruction assets

- `assets/skills/givn-explore/SKILL.md.tmpl:1`: existing thin skill, description, and served-instruction pointer. Replace with start, including metadata suitable for discovery and the new responsibility.
- `assets/instructions/explore.md:1`: current exploration stance and transitions. Replace the supported explore instruction surface with start; preserve useful investigation safeguards, not its proposal-first assumptions.
- `assets/instructions/ideation.md:3`: currently describes ideation as the artifact-producing alternative to explore; its entry-point comparison begins at line 14. Explain the distinction from start and when persistent domain work is warranted.
- `assets/instructions/proposal.md:69`: propose's orchestration creates the change and continues through specs/design. Start should hand off to it, not duplicate its orchestration.
- `assets/instructions/dialog.md:5`: includes explore among covered contexts. Update current terminology without weakening dialog coverage outside named workflows.

### Skill registration, sync, and removal

- `src/skills/render.rs:99`: `default_skill_names()` currently registers `givn-explore` at line 109.
- `src/skills/targets.rs:251`: sync renders default skills into configured targets and refreshes project guidance.
- `src/skills/targets.rs:380`: inspected reconciliation removes disabled addon skills; removing explore from the default list alone does not demonstrate removal of previously generated explore files.
- `src/skills/targets.rs:415`: legacy command cleanup derives names from `default_skill_names()`. Renaming the skill can accidentally stop recognizing old generated explore command files. Preserve appropriate cleanup recognition without retaining a callable alias.
- `src/skills/targets.rs:491`: managed generated path enumeration also needs investigation when replacing a built-in skill.

Verify clean init and upgrades/sync for all five supported targets: Claude Code, OpenCode, OMP, Maki, and neutral agents. Existing generated explore skills must not remain available after migration. Establish how to distinguish managed output from user-authored content before deleting anything; the removal decision does not authorize indiscriminate deletion of custom files.

### State guidance and generated project instructions

- `assets/agents-project-hint.md:4`: currently says to run `givn instructions` first and that it tells the next step. Clarify request-relative interpretation and direct work here.
- `src/cli/instructions.rs:251`: no-active-change reporting currently sets `next_command` to `givn new <change-id>` when there are no unproven candidates, and recommends new/propose in text at line 303.
- `src/cli/instructions.rs:313`: multiple-change guidance.
- `src/cli/instructions.rs:350`: archived-change guidance.
- `src/cli/instructions.rs:398`: active-change reporting.
- `src/cli/instructions.rs:522`: generic workflow overview.
- `src/cli/instructions.rs:551`: JSON next-command output. Do not fix only prose while machine-readable guidance still unconditionally directs creation. Do not place an agent skill name in a field promising an executable shell command without deliberately addressing that contract.
- `src/cli/instructions.rs:38`: non-artifact served instructions resolve before active-change selection. Start must remain usable without an active change.
- `src/cli/init.rs:247`: onboarding currently recommends `/givn-propose`; align new/unclassified-work guidance with start.

### Public documentation and terminology sweep

- `README.md:66`: quick-start entry points and current explore/ideation/propose distinction.
- `docs/reference.md:49`: skill targets, invocation conventions, entry-point guidance, and skill table.
- `docs/workflow.md:24`: phases and optional pre-phases.
- `docs/arc42/12-glossary.md`: inspect existing relevant terms before introducing new definitions.

Search current assets, docs, configuration examples, harness material, tests, and executable specifications for explore references and assumptions that all modifications require a formal change. Update operative guidance consistently. Do not mechanically replace the ordinary verb "explore" or unrelated specification-discovery commands that happen to use that word.

Historical archived changes and superseded analysis are evidence of past behavior, not active invocation guidance. Do not rewrite history just to eliminate every textual match; distinguish historical references from supported current entry points.

### Existing test and specification entry points

- `tests/steps/prose_consolidation_e2e_steps.rs:18` and `:37`: current skill lists include explore.
- `tests/steps/instruction_integrity_steps.rs:856` and `:1596`: explore-related instruction/skill handling.
- `givn/specs/obtain-effective-guidance/`: owning use case and extended-guidance scenarios.
- `givn/specs/initialize-project/`: owning use case and initialization/addon scenarios.
- `givn/specs/author-change/`: inspect the owning use case and relevant current guidance scenarios.
- `givn/archive/dialog-question-quality/specs/author-change/prose-consolidation.feature` and `givn/archive/add-ideation-skill/specs/support/ideation-skill.feature`: historical regression context, not instructions to edit archived features.

Select the actual owning use cases from current contracts during formalization. Do not invent a new capability solely because the entry skill changes names.

## Verification expectations

The future implementation should establish observable coverage for at least:

1. Fresh init generates `givn-start`, not `givn-explore`, for supported targets.
2. Sync/upgrade replaces previously generated explore entry points without leaving an alias or deleting unrelated user-authored content.
3. Legacy generated explore command cleanup still works after default skill registration changes; no new generated command surface is introduced.
4. Served start instructions are available in an initialized project with no active change and include all four outcomes, proportionality, and authorization boundaries.
5. No-active-change guidance no longer implies mandatory change creation; text and machine-readable guidance agree in meaning.
6. Active-change guidance retains truthful next-step information but does not assign unrelated user requests to that change.
7. Explicit continuation of an existing change remains supported, including pending verification/recovery information.
8. Direct ideation/proposal entry and already authorized, sufficiently understood direct work do not acquire a mandatory start gate.
9. Start distinguishes answering without edits from direct modification without formal artifacts.
10. Discussion alone does not authorize change creation, persistent ideation, implementation, commit, push, or archive; existing explicit continuation authorization is honored without redundant approval.
11. Current onboarding, README/reference/workflow guidance, and generated agent instructions describe the same entry behavior.
12. Optional addons being disabled does not remove the start routing behavior or the generally applicable dialog obligation.

Use executable tests for generation, cleanup, instruction serving, and CLI output. For agent judgment, inspect the actual served wording and exercise representative conversations where the available harness supports it; string presence alone does not prove correct autonomous routing. Avoid introducing a runtime classifier solely to make conversational judgments unit-testable.

Representative conversation cases:

- "Does Givn already support Fossil?" -> answer; no modification or new change.
- "Fix this typo in the README." -> authorized direct correction; no proposal ceremony.
- "This documented argument behavior is broken." -> investigate the existing contract; recommend/proceed with focused regression-covered work according to authorization and consequences, not the bugfix label alone.
- "We need approvals, but do not yet know who can approve what." -> recommend durable ideation when the domain uncertainty warrants it.
- "Introduce a new archive recovery guarantee." -> recommend a formal proposal when the benefit warrants the lifecycle.
- "Discuss this possible improvement." while an unrelated change is ready to archive -> discuss; do not archive or create a change automatically.
- "Discuss this and then create the proposal." -> honor that conditional continuation once ready; ask only necessary unresolved questions.

## Technical details left for the future design

These do not reopen the confirmed user-facing decisions:

- Exact wording and representation of lifecycle-next-step information versus request-routing guidance, especially the existing JSON `next_command` contract and consumers.
- Safe managed-file recognition and cleanup for old generated explore skills/commands across target configurations, including handling custom overrides without silently deleting user work.
- How existing explore instruction extensions/overrides are surfaced during migration. No callable explore compatibility alias is allowed; do not promise automatic conversion without checking the cascade and upgrade contracts.
- Exact placement of shared proportionality/direct-work guidance so start, generic output, and generated agent instructions agree without duplicating a new rulebook.
- Final scenario ownership and the smallest coherent implementation batches under the current use-case contracts.

## Scope boundary and repository hygiene

This handoff does not redesign ideation's artifacts or propose's downstream lifecycle, add a new shell command, weaken existing formal-change gates, invent a minor-change workflow, authorize release/version edits, or authorize commits.

At recording time, the working tree already contained modifications to `CHANGELOG.md`, `README.md`, `docs/arc42/12-glossary.md`, and `docs/reference.md`, plus the existing untracked `docs/improvements/0.9.11/` handoff. Preserve and distinguish those unrelated edits. The document-only work for this discussion should add only this file.

For implementation, follow the repository's current validation requirements. Current guidance prefers `cargo check`, `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, and `./run-tests.sh` (nextest by default), with `./run-tests.sh --e2e` for relevant integration coverage. Acceptance prerequisites include Git, Fossil, pinned git-cliff 2.13.1, and util-linux `script`.

If implementing and archiving a formal change, follow the current archive/self-dogfood requirements before starting another change. Do not run archive, install, commit, or release work merely because this handoff exists.
