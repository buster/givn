# Proposal: bound-review-context

## Release note

Givn runs each change in one context by default, permits at most one fresh independent review context for major or user-requested changes, and removes prescriptive subagent orchestration from its workflow assets.

## Use-Case Context

- Use-case ID: `verify-change`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `agent--technical-role`, `developer--primary-user` — CLI text output and workflow guidance are user-visible surfaces

## Review Context

- Review kind: self-review
- Reason: the change is a cross-cutting workflow policy; the default single-context review applies, and no independent reviewer is requested.

## Problem / Opportunity

Givn currently tells the workflow to spawn planning, design-review, review and implementation subagents. That prescription couples givn to a particular execution topology and produces the dominant cost in controlled runs.

Measured on givn 0.9.8, all ten add-ons, full Catopia:

| Policy | Time | Spend | Review children | Grade | Tests | Coverage L/B |
|---|---:|---:|---:|---:|---:|---:|
| One context, no subagents | **1h05m** | **$0.6919** | 0 | 24/25 | 135 | 84.9 / 70.4 |
| One review context per change | 2h13m | $1.4697 | 3 | 24/25 | 133 | 90.9 / 84.0 |
| Full delegation | 4h02m | $2.2973 | 13 | 25/25 | 278 | 97.0 / 95.0 |

One context is the cheapest and completes 24/25. One review per change adds coverage and requirements for roughly double the time. Full delegation is the quality outlier, not the default.

The same artifacts and gates already work across stop/restart and across separate sessions. Givn does not need to prescribe how many contexts execute each step.

## Proposed Solution

### Default execution

Givn requires no subagents. The main context performs proposal, design, design review, tasks, implementation and review.

The user or harness may still run steps in separate sessions or contexts. Givn does not forbid that and does not need to know. The artifacts and state remain the handoff mechanism.

### Default review

Review is a **self-review** in the current context.

- Record it as `Review kind: self-review`.
- Do not label it independent.
- A self-review may satisfy review and design-review artifacts when the project does not require independence.
- Do not claim an independent reviewer exists.

### Optional independent review

For a major change, or when the user requests it at the start, the change may use **exactly one fresh review context**.

- The context may be one review subagent, one separate reviewer session, or a human reviewer.
- The reviewer did not author the change, does not receive the author's conversation history, and does not edit.
- One context covers both the design-review obligation and the final review obligation for that change.
- Never start a second review context.
- Never start planning, design, hardening, coverage, implementation or additional review subagents.

### Major changes

A change is major when any of these is true, and the change records the reason:

- public contract, CLI or API change;
- persistent data, schema or migration;
- security, privacy or authorization;
- concurrency or distributed behavior;
- architecture boundary or external adapter;
- irreversible or destructive operation;
- multiple capabilities or high blast radius;
- the user explicitly requests an independent review.

If classification is ambiguous, ask the user one question. Do not infer "major" silently.

### Review record and gates

- `Review kind: self-review` or `Review kind: independent`.
- `Author` and `Reviewer context` are recorded for both.
- For independent review, the reviewer context must differ from the author.
- A gate that requires independence accepts only `Review kind: independent`.
- A gate that does not require independence accepts self-review.
- A self-review must never be turned into an independent claim by gate inference.
- Existing design-review and review markers remain required; only the context policy changes.

### Persistence

- At proposal time, record the requested review kind: `self-review` or `independent`.
- The default is `self-review`.
- A user request at the start must be written into the change, not only into session memory.
- After restart, the recorded review kind is authoritative.
- If an independent review is required but no fresh context is available, ask the user; do not silently fall back to self-review.

## Asset changes

Remove prescriptive subagent and execution-topology language from every guidance asset except `review.md`.

### Allowed exception

`assets/instructions/review.md` is the only asset allowed to mention a review subagent. It owns:

- default self-review;
- optional one independent review context;
- the major-change criteria;
- the one-context maximum;
- the rule that the reviewer must not be the author;
- the rule that no other subagents may be started.

### Remove or rewrite

Audit at least:

- `assets/instructions/proposal.md` — remove architect-subagent orchestration and fresh-context design-review instructions; write proposal/design in the current context.
- `assets/instructions/design-review.md` — remove mandatory fresh-context, grilling-subagent and hardening-subagent instructions; review in the current context by default. If an independent review is selected, refer to `givn instructions review`; do not start a separate design-review context.
- `assets/instructions/characterize.md` — remove the review-subagent step; use the current-context review policy.
- `assets/instructions/ideation.md` — remove fresh-context quality-review instructions; use the current-context review policy.
- `assets/instructions/reverse-engineer.md` — remove fresh-delegated-context instructions.
- `assets/instructions/tasks.md` — remove batch coding-subagent delegation and subagent self-report language; work in the current context unless the user/harness chooses otherwise.
- `assets/commands/givn-review.md` and `assets/skills/givn-review/SKILL.md.tmpl` — remove "start one fresh review subagent"; load the review instruction.
- Other command/skill templates — remove "delegate to a fresh context" / "fresh-context agent" boilerplate that prescribes execution topology.
- `assets/instructions/dialog.md` — keep business-language "delegating" where it means handing a task to a person or service; remove only execution-topology prescriptions.

Do not remove "architecture," "delegation" in business language, or domain terms. The removal target is execution mechanism, not domain vocabulary.

## Review instruction text

Use this exact behavior in `assets/instructions/review.md`:

> Default: perform the review in the current context and record `Review kind: self-review`. A self-review is not an independent review; do not label it independent.
>
> For a major change, or when the user requests it, perform the review in exactly one fresh context: one review subagent, one separate reviewer session, or one human reviewer. One context covers both the design-review and final-review obligations for the change. The reviewer must not be the author, must not receive the author's conversation history, and must review only.
>
> Do not start planning, design, hardening, coverage, implementation, or additional review subagents. Never start more than one review context.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` | The existing verification use case owns the review gate and workflow enforcement. The new review-kind record and independence check extend that contract; the guidance changes carry the same policy. |

## Out of Scope

- No full-delegation policy.
- No harness, driver, model or provider changes.
- No add-on default changes.
- No benchmark-specific laws, names or data in generated guidance.
- No removal of design-review, review, archive or verification gates.
- No automatic major-change classifier without a recorded reason.
- No requirement that every step run in one context; the user/harness may split steps.
- No removal of descriptive subagent vocabulary from the development report or the harness reporter integrations; those record measured activity and do not prescribe topology.

## Open Questions

### D1 — Which assets does the subagent-vocabulary acceptance check cover?

- Question: does the acceptance check `rg -i 'subagent|sub-agent' assets/` cover the development report and harness reporter integrations, which record subagent spans but do not prescribe workflow topology?
- Options:
  1. **Guidance assets only** — run the check over `assets/instructions`, `assets/commands`, `assets/skills`, and `assets/templates`; the development report and integrations keep recording measured subagent activity; gain: the policy target is enforced without changing the recorded activity contract; cost: the literal whole-`assets/` command is not the acceptance check.
  2. **Every asset** — rename the timeline KPI and span labels and stop parsing the reporters' `subagent` events; gain: the literal whole-`assets/` command passes; cost: the development report loses measured subagent spans and the harness integration ignores reported activity, which the out-of-scope list forbids.
- Disposition: answered 2026-10-29 (user) — option 1. Measuring eventual subagents is required; the check targets guidance assets only.
