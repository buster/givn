# Proposal: enforce-spec-interaction-integrity

## Release note

Block review and archive when the projected specification corpus has an interaction-inventory mismatch, including drift that predates the change.

## Use-Case Context

- Use-case ID: `verify-change`, `archive-change` (repaired contracts: `initialize-project`, `fragments`)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`

## Review Context

- Review kind: independent
- Reason: public CLI contract change and multiple capabilities (`verify-change`, `archive-change`) with high blast radius; the review and archive gates change behavior.

## Problem / Opportunity

The permanent specification corpus is the durable executable record. A change can add an `@e2e` scenario to a capability and leave the owning interaction inventory unchanged. `givn check review` does not see that mismatch because it evaluates only the permanent inventory, while its change-scoped filter keeps only findings attributed to the change's delta. `givn archive` runs its static gates before the merge and never lints the merged corpus.

The current corpus already has 11 blocking mismatches. Ten fragment capabilities declare fewer interactions than their features contain, and one init scenario is declared as an E2E interaction but lacks the `@e2e` tag. A second gap is structural: a feature file under a use-case directory that the use case does not declare in its capability list is never read, so a new capability can add `@e2e` scenarios and leave the inventory silent. A third gap is placement: change-scoped lint does not require a delta capability to belong to the projected corpus, so an unplaceable delta can pass the lint gate while archive refuses it.

The impact is a false green: a project can archive a change that leaves its permanent corpus contradicting its E2E inventory, and the E2E suite can omit a scenario its use case declares. The documentation also disagrees with the binary on the lint exit code for these findings and describes an unconfigured comparison value as an executable sentinel.

## Proposed Solution

A review or change-scoped lint evaluates the specification corpus the change would leave behind, before the change is accepted. The check fails when that projected corpus declares an interaction that no `@e2e` scenario proves, contains an `@e2e` scenario that the owning interaction inventory does not list, or contains a feature file that the owning use case does not declare in its capability list. A delta capability that cannot be placed in the projected corpus is a blocking finding, not a silent skip. The refusal names the use case, the capability, and the missing or unlisted scenario. Archive performs the same evaluation before it moves the change or writes a receipt, so the check keeps the project's block-before-mutation rule.

A consistent projection passes without a new advisory finding. The project's own permanent inventory is repaired so the current corpus passes: the ten fragment capabilities gain their missing interaction rows, and the noninteractive init scenario gains the `@e2e` tag its use case declares. The reference documentation now states the lint exit code the binary uses for use-case and interaction findings, and describes the unconfigured E2E runner value as a comparison value intercepted by the E2E gate, not as an executable command.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` | The route's top candidate; the review gate owns the new refusal. |
| `check-specification-health` | no signal | `EXTEND check-specification-health` | The change-scoped lint is the developer-facing surface for the same projected inventory check. |
| `archive-proof` | no signal | `EXTEND archive-proof` | The archive gate owns the pre-mutation refusal and the guarantee that a proven archive left a consistent corpus. |
| `initialize-a-project` | no signal | `EXTEND initialize-a-project` | The repair adds the missing `@e2e` tag to an existing scenario; no new capability is introduced. |

## Out of Scope

- Changing the archive merge algorithm, receipt schema, or publication sequence.
- Adding semantic or LLM-based specification checks.
- Rewriting archived deltas, proven receipts, or historical evidence.
- Changing the E2E runner or adding E2E coverage beyond the repaired init tag.
- Changing lint exit codes for other finding classes or other commands.
- Adding a `givn missing-e2e-runner` subcommand; the documentation correction is the chosen repair.
- Changing the default add-on set or the interaction-inventory model beyond declaring that a feature absent from the capability list is a mismatch.
- Repairing unrelated advisory findings such as shape matches.

## Open Questions

### D1 — Where is the projected interaction-inventory invariant enforced?

- Question: Should a projected mismatch block review and archive before mutation, block only after the merge, or only fail CI after the change lands?
- Options:
  1. **Project before mutation** — review, change-scoped lint, and the archive static gates evaluate the corpus the change would produce; gain: feedback while the change is still active, no red archive candidate, and the existing block-before-mutation promise is preserved; cost: the check must project the merge without writing and both gates invoke it; in practice: a delta adds an `@e2e` scenario without an interaction row, review fails naming the scenario, and no archive candidate is created.
  2. **Check after merge** — review stays unchanged; archive merges and then lints the permanent corpus; gain: simplest implementation and validation of the actual merged bytes; cost: the change has already moved and the developer repairs an archive candidate; in practice: the same delta archives, merges, and is then refused under `givn/archive/<id>/`.
  3. **CI only** — no new review or archive gate; CI lints the permanent corpus after landing; gain: no new gate or recovery path; cost: a red corpus can still be archived and downstream projects without CI get no protection; in practice: the same delta archives successfully and the next CI run fails.
- Disposition: answered — the user chose option 1 (project before mutation). Rejected: option 2 because it moves the failure after the mutation the archive gates are meant to protect; option 3 because it does not satisfy the documented hard gate.

### D2 — Which lint exit code is authoritative for use-case and interaction findings?

- Question: Should `givn lint` return `1` or `3` for an invalid use-case document or an interaction mismatch?
- Options:
  1. **Keep `1`** — the current binary and the executable scenarios already use `1`; gain: no contract change and no existing scenario rewrite; cost: the reference and runtime chapter must be corrected; in practice: an interaction mismatch exits `1` and the documentation says `1`.
  2. **Change to `3`** — make the reference authoritative; gain: use-case errors join duplicate titles under `3`; cost: the existing executable scenarios and any caller branching on `1` change behavior; in practice: an interaction mismatch exits `3` and the scenarios are updated.
- Disposition: answered — option 1. The executable specification and the current binary use `1`; the reference is corrected. Rejected: option 2 because it changes an existing executable contract to match stale prose.

### D3 — Should `givn missing-e2e-runner` become an executable command?

- Question: Should the documented E2E sentinel be added as a hidden command, or should the documentation describe the current comparison value?
- Options:
  1. **Correct the documentation** — describe `givn missing-e2e-runner` as the unconfigured comparison value intercepted by the E2E gate; gain: no unused command and no new CLI surface; cost: the three sentinels are not symmetric; in practice: running the string returns clap's unknown-subcommand error, and the reference explains why.
  2. **Add the hidden command** — make all three sentinels executable and print setup guidance; gain: symmetric behavior and a diagnosable command; cost: a new hidden CLI surface that no gate path invokes; in practice: `givn missing-e2e-runner` prints guidance and exits non-zero.
- Disposition: answered — option 1. The value is a comparison sentinel handled by the E2E gate, not a command. Rejected: option 2 because it adds an unexercised public surface to satisfy one sentence.

### D4 — Does the gate cover features the inventory does not declare?

- Question: Should the projected interaction-inventory gate also check permanent feature files that the owning use case does not declare in its capability list?
- Options:
  1. **Extend the gate** — every feature file in a use-case or fragment directory must be declared, and every `@e2e` scenario in it must be listed; gain: matches the proposal's stated guarantee, closes the new-capability false green, and is symmetric with the existing dangling-capability check; cost: a new blocking finding kind, stray or experimental feature files are flagged, existing corpora may need a one-time repair, and the fixture suite grows; in practice: `orders/new-cap.feature` with an `@e2e` scenario is refused with "capability 'new-cap' is not declared in use case 'orders'".
  2. **Narrow the claim** — change the proposal, design, and docs to check only declared capabilities; gain: a smaller change with no new finding kind; cost: the stated guarantee weakens, the new-capability false green remains, and a future change must close it; in practice: the same delta archives with the inventory silent.
  3. **Defer with a tracked risk** — keep this change focused and open a follow-up; gain: less scope now; cost: the current guarantee is partial and the gap is live until the follow-up; in practice: the delta passes now with a recorded risk.
- Disposition: answered — the user chose option 1 (extend the gate). Rejected: option 2 because it contradicts the proposal's stated guarantee; option 3 because it leaves the same false-green class open.
