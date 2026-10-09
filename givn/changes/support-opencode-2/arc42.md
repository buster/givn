# Arc42 Impact Assessment: support-opencode-2

The change replaces the OpenCode attribution integration's plugin contract with
OpenCode 2's session, tool, and shell hooks. It touches one generated JavaScript
asset, its repository dogfood copy, the OpenCode attribution step glue, and the
architecture chapters that describe the generated integration's runtime path.

| # | Chapter area | Affected | Reason / change |
|---|---|---|---|
| 1 | Introduction and goals (01), quality goals (10) | Yes | The commit-attribution quality scenario Q-78 now observes an OpenCode 2 session; its precondition is reworded in chapter 10. No goal or stakeholder statement in chapter 1 changes, so chapter 1 is not edited. |
| 2 | Constraints (02) | Yes | TC-29 is reworded: the integration observes the provider and model per session through the session lifecycle, links each shell request to its issuing session, and stamps the environment when the shell is created; payload drift stays a non-blocking best-effort risk. |
| 3 | Context and scope (03) | No | The "OpenCode plugin boundary" row already states that the generated integration receives the current session model through OpenCode's plugin hooks; that stays true for OpenCode 2. |
| 4 | Solution strategy (04) | No | The commit-attribution strategy row already names OpenCode as a best-effort adapter; the adapter's contract version is not a strategic choice. |
| 5 | Building block view (05) | No | The generated attribution asset and its embedded ownership stay unchanged as a building block: still exactly one embedded OpenCode integration produced through the existing generation path. |
| 6 | Runtime view (06) | Yes | The OpenCode path paragraph in 6.1a is rewritten: session lifecycle events (`context`, `compaction`, `generate`) remember the provider, model identity, and reasoning variant per session; `execute.before`/`execute.after` link a shell command to its issuing session; `shell.create.before` stamps the existing Givn environment protocol; the fallback order is issuing session's model, last observed model, configured default, unavailable, and no shell is ever blocked. |
| 7 | Deployment view (07) | No | The deployment footprint is unchanged: one generated `.opencode/plugins/givn-attribution.js` when OpenCode is selected, no sibling file, no runtime version selection, and no new dependency. |
| 8 | Cross-cutting concepts (08) | No | The attribution environment protocol, agent target rules, and generated-rule boundaries are unchanged; only the integration that fills the environment moves to the current plugin contract. |
| 9 | Architecture decisions (09) | No | D1 (OpenCode 2 only), D2 (attribute to the issuing session), and D3 (simulated plugin lifecycle) are each fully implementable and reversible within this one change and touch one component, so they fail ADR qualification; their rationale lives in the proposal, design, and specs. No ADR is added, amended, or superseded, and the register and ADR index stay as they are. |
| 10 | Quality requirements (10) | Yes | Q-78's precondition is reworded to "An OpenCode 2 session supplies a selected provider and model before a shell execution"; the expected attribution identity, harness, and committer output is unchanged. |
| 11 | Risks and technical debt (11) | Yes | R-50 is reworded from payload drift in one hook to drift in the OpenCode 2 plugin contract (session, tool, and shell hooks); its mitigation keeps missing metadata non-blocking and executes the generated plugin against the replayed OpenCode 2 contract in the attribution E2E scenario. |
| 12 | Glossary (12) | Yes | The change introduces the domain term **Issuing session** — the session whose shell request created a command; attribution follows that session's remembered model, not the last model observed anywhere in the run. The entry is recorded in chapter 12; the existing Attribution context, Model identity, and Harness entries stay accurate. |

## Status

STATUS: DONE
