# Proposal: fix-opencode-v2-attribution

## Release note

Givn initialization and regeneration provide commit attribution compatible with recent OpenCode V1 and OpenCode V2 without changing the supplied attribution behavior.

## Use-Case Context

- Source: direct user defect report and supplied attribution behavior; no ideation topic or change seed for this change. No seed is stamped.
- Owner: the reusable support fragment (`givn/specs/fragments/fragment.md`) owns attributing commits (`attribute-commits`) and enabling commit attribution (`enable-commit-attribution`). A fragment has no independent actor goal and is not a use case.
- Consumer contracts: initializing a project (`initialize-project`) sets up chosen agent integrations and backend-aware guidance; upgrading a project (`upgrade-a-project`) refreshes managed project content with clean-state protection and reviewable history. Both include the support fragment. Their interaction inventories are not duplicated here.
- Confirmed affected Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`, as named by the initialization contract; the upgrade contract names the developer and maintainer. Confirmation provenance: `givn/ideation/multi-scm-support/session.md`, the existing confirmed Frame and support/attribution elaboration. This is historical Persona provenance, not this change's source topic.
- Actors remain Developer, supporting Authoring agent, Project maintainer, and OpenCode acting for the developer. Personas are review lenses, not Gherkin actors.

## Problem / Opportunity

A developer's manually updated OpenCode attribution integration works with OpenCode V2. Initializing Givn or synchronizing skills replaces it with an integration usable only by OpenCode V1. Target additions, addon changes, and managed upgrades can also regenerate the same integration. The developer must repair it repeatedly and can lose LLM, reasoning-mode, and committer attribution after routine maintenance.

## Proposed Solution

- Whenever OpenCode is a selected agent target, initialization and every regeneration action provide the same commit-attribution compatibility: OpenCode V1 1.18.29 and newer within the V1 generation, and OpenCode V2.
- Cover initializing a project, synchronizing skills, adding an agent target, enabling an addon, disabling an addon, and performing a managed upgrade. Reinitialization without force and initialization without addons must not regress compatibility.
- After these actions, the developer can use OpenCode to execute an attributed shell command without manually repairing the integration. Existing installation reports continue naming the integration.
- Preserve the supplied model remembering and reasoning-mode retention, the last observed model fallback, and OpenCode V2's configured-default fallback when no remembered model is usable. Missing usable context remains unavailable and does not prevent shell execution or human commits.
- This is best-effort attribution, not proof of which session issued a command. A shell command without identifiable session context can receive the last model observed in another session or the configured default. Preserve that behavior at the user's explicit direction and state the limitation; do not describe fallback values as verified current-session facts.
- Preserve chosen targets, addon choices, Git/Fossil attribution boundaries, and the managed-upgrade clean-state and publication guarantees.

## Capability Routing

Routing this completed draft on 2026-10-04 with `givn spec route --change fix-opencode-v2-attribution` exited 0 and reported `EXTEND apply-a-managed-upgrade` in `upgrade-a-project`, score 17.00, confidence `signal`. Its evidence was the proposal's routing-command text, three goal/rules words, and the named consumer use case. This recommendation identifies a consumer, not the existing reusable owner of attribution. The decisions below intentionally deviate from it.

| Proposed capability | Route's recommendation | Decision | Rationale for deviation |
|---|---|---|---|
| Attribute commits across supported OpenCode generations | `EXTEND apply-a-managed-upgrade` | `EXTEND attribute-commits` | Existing support behavior already owns initialization of shared attribution and OpenCode shell attribution. Compatibility and retained fallback variants extend that contract; OpenCode is a harness, not a new actor goal. The recommendation's lexical evidence does not supersede that ownership. |
| Enable compatible attribution during regeneration | `EXTEND apply-a-managed-upgrade` | `EXTEND enable-commit-attribution` | Existing support behavior owns installation reports and managed-upgrade retrofit. Skill synchronization, target addition, and addon toggles refresh the same attribution behavior; they do not justify a channel-shaped capability or duplicate placement in including use cases. Managed-upgrade behavior outside attribution is unchanged. |

## Out of Scope

- Supporting OpenCode V1 older than 1.18.29 or guaranteeing unreleased future plugin APIs.
- Changing the supplied attribution logic, including improving cross-session accuracy, clearing remembered variants, adding title-model remembering, or making configured-default lookup failures fatal.
- Changing generic commit stamping, Claude attribution, author identity, human co-author preservation, SCM selection, hook activation, upgrade authorization, or migration-plan early returns.
- Preserving arbitrary manual customization of generated attribution content through backup, merge, override, force, or runtime-version selection features. Regeneration still replaces the generated integration; it must replace it with compatible content.
- Producing separate generated integrations for each OpenCode generation or modifying either user-supplied plugin file during this change's planning.

## Open Questions

No unresolved operator decision remains. The two binding dialogs are retained in full below, including the rejected alternatives. Resolution plumbing belongs in the technical design, not this proposal.

### Supported OpenCode generations (D1)

- Status: answered by the user in the originating dialog, supplied to this planning session on 2026-10-04.
- Blocks: the promised support window and compatibility evidence.
- Context: the official migration guide documents a shared V1/V2 integration starting with OpenCode V1 1.18.29; older V1 compatibility cannot be inferred from a manually combined integration.
- Question: “The official migration guide documents a single plugin with V1 server() and V2 setup() for OpenCode V1 1.18.29 and newer. Your named V1 export plus V2 default export is not documented as compatible with older V1 loaders. Which compatibility scope should the generated attribution plugin guarantee? This determines the plugin entrypoint and test matrix.”
- Options:
  1. **Recent V1 plus V2 (Recommended).** Meaning: guarantee V1 1.18.29+ and V2 using the documented shared entrypoint. Gain: one generated `.opencode/plugins/givn-attribution.js`, without version detection. Cost: older V1 is outside the guarantee. Example: after init or skills sync, the same integration is usable by either supported generation.
  2. **Include older V1.** Meaning: retain older V1 support and investigate and test its loader. Gain: a wider support window. Cost: an additional compatibility matrix and potentially version-specific installation. Example: a pre-1.18.29 project still works after sync.
- User selection: **“Recent V1 plus V2 (Recommended)”**.
- Disposition: answered — user choice binds the proposal and the design's support matrix. Reject including older V1 because it adds unrequested support and installation complexity outside the chosen window. Do not reopen the choice.

### Preserve supplied attribution behavior (D2)

- Status: answered by explicit user constraint in the originating dialog, supplied to this planning session on 2026-10-04.
- Blocks: whether compatibility work may change attribution fallback semantics.
- Context: the prototype can label a command with a model observed in another session or with OpenCode's configured default. The existing generic attribution wording says unavailable facts remain unknown; compatibility must not silently strengthen that into a claim the prototype does not fulfill.
- Question: “Your plugin falls back to the last observed model from any session, then to OpenCode's configured default. For example, after session A uses model A, session B can be attributed to model A before B supplies model metadata. The existing attribution contract says unavailable facts must remain unknown. When a shell command's session model cannot be established, what should the generated plugin record?”
- Options:
  1. **Unavailable (Recommended).** Meaning: identify the harness as OpenCode, but leave model and reasoning unavailable unless tied to the command's session. Gain: no false cross-session label. Cost: some missing attribution. Example: unknown session B is not stamped with model A.
  2. **Keep best-effort fallback.** Meaning: keep the last-model fallback and then the supplied configured-default fallback. Gain: more attribution labels. Cost: a label may name a model that did not issue the command, requiring contract clarification. Example: unknown session B can be stamped with model A.
- Exact user response: **“No changes to the plugin logic itself.”** This is a constraint, not a claim that inferred labels are accurate.
- Disposition: answered — preserve the prototype behavior, with a narrow adapter exception to generic no-fabrication wording and an explicit retained risk. Reject changing unknown-session behavior to unavailable because the user expressly disallowed logic changes. Reject broader fallback redesign, additional hooks, and fatal lookup errors for the same reason. The generic stamper continues to consume supplied context and never fabricate missing values.
