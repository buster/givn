# Proposal: fix-opencode-v2-attribution

## Release note

Givn initialization and regeneration provide commit attribution compatible with recent OpenCode V1 and OpenCode V2, and on V2 attribute each agent's shell commands, including subagents', to that agent's model.

## Use-Case Context

- Source: direct user defect report and supplied attribution behavior; no ideation topic or change seed for this change. No seed is stamped.
- Owner: the reusable support fragment (`givn/specs/fragments/fragment.md`) owns attributing commits (`attribute-commits`) and enabling commit attribution (`enable-commit-attribution`). A fragment has no independent actor goal and is not a use case.
- Consumer contracts: initializing a project (`initialize-project`) sets up chosen agent integrations and backend-aware guidance; upgrading a project (`upgrade-a-project`) refreshes managed project content with clean-state protection and reviewable history. Both include the support fragment. Their interaction inventories are not duplicated here.
- Confirmed affected Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`, as named by the initialization contract; the upgrade contract names the developer and maintainer. Confirmation provenance: `givn/ideation/multi-scm-support/session.md`, the existing confirmed Frame and support/attribution elaboration. This is historical Persona provenance, not this change's source topic.
- Actors remain Developer, supporting Authoring agent, Project maintainer, and OpenCode acting for the developer. Personas are review lenses, not Gherkin actors.

## Problem / Opportunity

A developer's manually updated OpenCode attribution integration works with OpenCode V2. Initializing Givn or synchronizing skills replaces it with an integration usable only by OpenCode V1. Target additions, addon changes, and managed upgrades can also regenerate the same integration. The developer must repair it repeatedly and can lose LLM, reasoning-mode, and committer attribution after routine maintenance.

The manually updated V2 integration also mislabels subagent work. OpenCode V2 runs a subagent in its own session, often on another model and in parallel with the main session, and its shell hook does not identify the session. Commands from either session therefore receive whichever model was observed last (D3).

## Proposed Solution

- Whenever OpenCode is a selected agent target, initialization and every regeneration action provide the same commit-attribution compatibility: OpenCode V1 1.18.29 and newer within the V1 generation, and OpenCode V2.
- Cover initializing a project, synchronizing skills, adding an agent target, enabling an addon, disabling an addon, and performing a managed upgrade. Reinitialization without force and initialization without addons must not regress compatibility.
- After these actions, the developer can use OpenCode to execute an attributed shell command without manually repairing the integration. Existing installation reports continue naming the integration.
- On OpenCode V2, attribute each shell command to the model of the session that issued it, including subagent child sessions that run another model, by linking the command through the tool hooks (D3).
- Preserve the supplied model remembering and reasoning-mode retention, the last observed model fallback, and OpenCode V2's configured-default fallback when no remembered model is usable; the configured-default lookup reads the real V2 response. Missing usable context remains unavailable and does not prevent shell execution or human commits.
- This is best-effort attribution, not proof of which session issued a command. A shell command without identifiable session context can receive the last model observed in another session or the configured default. Preserve that behavior at the user's explicit direction and state the limitation; do not describe fallback values as verified current-session facts.
- Preserve chosen targets, addon choices, Git/Fossil attribution boundaries, and the managed-upgrade clean-state and publication guarantees.

## Capability Routing

Routing this completed draft on 2026-10-04 with `givn spec route --change fix-opencode-v2-attribution` exited 0 and reported `EXTEND apply-a-managed-upgrade` in `upgrade-a-project`, score 17.00, confidence `signal`. Its evidence was the proposal's routing-command text, three goal/rules words, and the named consumer use case. This recommendation identifies a consumer, not the existing reusable owner of attribution. The decisions below intentionally deviate from it.

| Proposed capability | Route's recommendation | Decision | Rationale for deviation |
|---|---|---|---|
| attribute-commits | `EXTEND apply-a-managed-upgrade` | `NEW in fragments` | The capability lives in the reusable support fragment; the routing check resolves only use-case-owned capabilities, so fragment deltas record `NEW in fragments`. Existing support behavior already owns initialization of shared attribution and OpenCode shell attribution. Compatibility and retained fallback variants extend that contract; OpenCode is a harness, not a new actor goal. |
| enable-commit-attribution | `EXTEND apply-a-managed-upgrade` | `NEW in fragments` | Same fragment rule; the capability lives under the support fragment. Existing support behavior owns installation reports and managed-upgrade retrofit. Skill synchronization, target addition, and addon toggles refresh the same attribution behavior; they do not justify a channel-shaped capability or duplicate placement in including use cases. |

## Out of Scope

- Supporting OpenCode V1 older than 1.18.29 or guaranteeing unreleased future plugin APIs.
- Changing the attribution logic beyond D3: clearing remembered variants, adding title-model remembering, removing the last-observed-model fallback, or making configured-default lookup failures fatal. Linking V2 shell commands to their session (including subagent sessions) and reading the real configured-default response are in scope.
- Changing generic commit stamping, Claude attribution, author identity, human co-author preservation, SCM selection, hook activation, upgrade authorization, or migration-plan early returns.
- Preserving arbitrary manual customization of generated attribution content through backup, merge, override, force, or runtime-version selection features. Regeneration still replaces the generated integration; it must replace it with compatible content.
- Producing separate generated integrations for each OpenCode generation or modifying either user-supplied plugin file during this change's planning.

## Open Questions

No unresolved operator decision remains. The binding dialogs are retained in full below, including the rejected alternatives. Resolution plumbing belongs in the technical design, not this proposal.

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
- Superseded in part by D3 (2026-10-06): the user asked for a rewrite of the plugin for OpenCode V2, including subagents. The retained fallback chain (last observed model, then configured default) stays; "no changes to the plugin logic" no longer binds.

### Rewrite the V2 half for subagents and the real V2 API (D3)

- Status: answered by the user on 2026-10-06 (rewrite request plus the fallback choice below).
- Blocks: the plugin's V2 session correlation and its configured-default lookup.
- Context: released OpenCode V2 2.0.22 runs a subagent in a child session with its own model, possibly in parallel with the main session. The V2 shell hook (`create.before`) receives only command, working directory, timeout, shell, and environment. `OPENCODE_SESSION_ID` is added after that hook, so the prototype's session lookup is always empty or stale and every V2 command falls through to the last model seen in any session. For example, when the main session runs `litellm/gpt-6-luna#high` and an `explore` subagent runs `anthropic/haiku`, a main-session commit can be stamped `anthropic/haiku`. The configured-default lookup also reads the wrong shape: `ctx.model.default()` returns `{location, data}`, not the model.
- Question: “When a V2 shell command cannot be linked to a session (a user `!` command, or identical commands from two sessions at once), what should the commit record?”
- Options:
  1. **Unavailable (Recommended).** Gain: no commit names a model that did not issue it. Cost: some commits lose attribution.
  2. **Main-session model.** Gain: subagent models never label unlinked commands. Cost: still a guess with parallel top-level sessions.
  3. **Keep last-model guess.** Gain: most commits carry a label. Cost: an unlinked command can carry another session's model.
- User selection: **“Keep last-model guess”**.
- Disposition: answered — rewrite the V2 half to link each shell command to its session through the tool hooks (`execute.before` carries the session and the command, and precedes `create.before`), so main-session and subagent commands each get their own model. Keep the last-observed-model fallback, then the configured default, for commands that cannot be linked. Fix the configured-default lookup to read the real response shape. V1 behavior is unchanged, because V1's `shell.env` already receives the child session for subagents. State the limitation: an unlinked command can receive a model that did not issue it. Reject unavailable and main-session-only fallbacks because the user chose to keep the guess.

### Stale pending shell requests (D3 refinement)

- Status: answered by the user on 2026-10-06 during design review.
- Blocks: how the V2 session link forgets a shell request that never reaches shell creation.
- Context: the V2 link remembers each requested shell command until OpenCode creates the shell or reports the request finished. A request can be vetoed by another plugin or aborted before either happens. It then stays pending, and a later identical command from another session can claim it. For example, session A asks for `git commit`, the request is aborted, and later session B runs `git commit`: B's commit is labeled with A's model.
- Question: “How should the plugin forget a pending shell request that never reached shell creation?”
- Options:
  1. **Expire by age.** Meaning: drop a pending request after a fixed time. Gain: also clears sessions that go quiet. Cost: a time constant, and a slow permission prompt can expire a legitimate request, so its command falls back to the last observed model.
  2. **Drop older same-session entries (Recommended).** Meaning: when a session requests a new shell command from a newer message, drop that session's pending requests from older messages. Gain: no timer; a session that keeps working cannot leave a stale request behind. Cost: requests of a session that goes quiet stay pending until the plugin unloads.
  3. **Accept the risk.** Meaning: keep stale requests until the plugin unloads. Gain: simplest. Cost: an aborted request can label any later identical command from any session.
- User selection: **“Drop older same-session entries”**.
- Disposition: answered — on each V2 shell request the plugin drops that session's pending requests from an older message; a finished request is removed by its session and request identifier; the claimed command is read from the stored request; pending requests belong to one plugin instance. Residual risk, stated in design and risks: a quiet session's stale request stays pending until the plugin unloads. Reject expiry by age because a time constant mislabels slow but legitimate requests; reject accepting the risk because it leaves the mislabel unbounded.

### Subagent E2E scenario scope (D3 refinement)

- Status: answered by the user on 2026-10-06 (twice during the review follow-ups).
- Blocks: whether the V2 subagent E2E scenario must discriminate the D3 link from the old last-observed-model fallback.
- Context: against real V2 2.0.22 the old fallback also stamps the child's own model, because the child's `context` event is the last observed one at its shell creation. The first decision (scope of the follow-ups: defects, documentation, and making the E2E discriminate) asked for a discriminating E2E. Two constructions were tried against the real runtime. (1) The main session resumes after its subagent and runs its own command: the real asset gives model-a/high for the main command and model-b/low for the child's, but the immutable prototype also passes (`1 scenario (1 passed) 14 steps (14 passed)`), because the main session's resumed request fires `context` with model-a and re-sets the last-observed model. (2) The main session asks for the subagent and its own command in one step: the runtime runs the tool calls concurrently and the main shell ran first in 3 of 3 runs, so no deterministic order exists.
- Question: “Keep the E2E as characterization, or accept a racy construction (parallel subagents with different models)?”
- User selection: **keep as characterization (option A); do not try parallel subagents**.
- Disposition: answered — the E2E stays real-loader characterization (it now also runs the main session's own command after the child, and records the child's `execute.before` session and the absence of `OPENCODE_SESSION_ID` at `create.before`); it carries a description saying it does not discriminate the old fallback. The regular scenario “A subagent's shell command is attributed to the subagent's own model” is the discriminator. This replaces the earlier decision to keep the scenario as written, and the follow-up scope decision to make it discriminate. Reject the racy parallel-subagent construction.
