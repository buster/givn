---
description: Stress-test the plan before tasks are written — grill the design, harden the artifacts, sign off
---

Stress-test the plan (proposal, spec, design) before the task list is written.
Catches scope, architecture, and testability problems when they cost a markdown
edit rather than a rewrite.

**Input**: Optionally specify a change ID after `/givn-design-review`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

---

## Mandatory, non-skippable, fresh context

This gate exists to catch a flawed plan before it produces flawed tasks and
flawed code. Two conditions make it work; violating either one defeats its
purpose:

- **Never skip it.** Do not let this step be bypassed because "the design
  looks fine" or "there's no time." A design that looks fine to the person
  who just wrote it is exactly what this gate exists to check.
- **Never run it as a continuation of the session that wrote the design.**
  Both subagents below MUST be spawned via the Task tool into a genuinely
  fresh context — not the same conversation, not a subagent that has already
  seen the architect's reasoning. A model that just produced the design
  cannot credibly grill its own blind spots; it will confirm its own
  assumptions. If invoked directly after `/givn-propose` in the same
  conversation, still spawn the grilling subagent fresh — do not let it
  inherit the orchestrator's or the architect's context.
- **Use a capable reasoning model with high thinking/reasoning effort for
  the grilling subagent.** This step depends on catching subtle scope gaps,
  wrong tech choices, and untestable scenarios — the same failure modes a
  fast, low-effort model is prone to missing or rubber-stamping. When the
  harness lets you choose a model or reasoning-effort tier for the spawned
  subagent, choose the most capable one available and set reasoning/thinking
  effort to high. Do not spawn the grilling subagent on a fast/cheap/
  low-reasoning-effort tier to save time — that produces a review that
  passes everything, which is worse than no review at all because it
  creates false confidence.

---

## Role: orchestrator

This command is an **orchestrator** with two subagents and an interactive loop
in between:

1. **Grilling subagent** — reads all planning artifacts and the codebase; returns
   a ranked list of questions with recommended answers.
2. **Interactive loop** — the orchestrator presents questions one at a time to
   the user, records answers.
3. **Hardening subagent** — applies all decisions reached to the artifacts and
   writes `design-review.md`.

The interactive loop runs in the orchestrator's own context so the
one-question-at-a-time dialogue works naturally.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".

Check the `artifacts` array:
- `design` must be `"status": "done"` before proceeding.
- If `design` is not done, stop: "Design must be complete before design-review."

### 2. Collect instructions

```sh
givn instructions design-review --change <id>
```

Capture `output` path, `template`, `instruction`, `dependencies`.

### 3. Spawn the grilling subagent

Use the **Task tool** to spawn a grilling subagent in a genuinely fresh
context — never a continuation of this conversation or of whatever session
wrote the design. If the harness exposes a model or reasoning-effort choice
for the spawned subagent, select the most capable model available with
reasoning/thinking effort set to high. This subagent's entire value is
catching what the design's author missed; a low-effort model in the same
context as the author defeats the purpose.

Pass a self-contained prompt:

```
You are a grilling subagent for givn change '<id>'.

Your job: read the full plan and return a prioritised list of questions that
must be answered before implementation starts. You do NOT ask the user anything
directly — you return the questions so the orchestrator can present them one at
a time.

## Read everything first

Read all of these in full before forming any questions:
- givn/changes/<id>/proposal.md
- givn/changes/<id>/specs/**/*.feature
- givn/changes/<id>/design.md
- Existing codebase files named in design.md (integration points, affected
  modules, patterns). If a question can be answered by exploring the codebase,
  explore it and record the finding as a resolved item — do not include it as
  a question for the user.
- If the arc42 addon is enabled (check `givn/config.yaml` for `addons.arc42:
  true`, or the presence of `givn/changes/<id>/arc42.md`): also read
  `givn/changes/<id>/arc42.md` and every chapter file under `docs/arc42/`
  (or the configured `arc42.output_dir`) that the marker file claims to have
  touched. This is required input for the Architecture documentation branch
  below — do not skip it because arc42.md says `STATUS: DONE`; that marker is
  a self-report from the same agent that wrote the docs, not verification.

## Required branches to cover

Walk the design tree and produce at least one question per branch (or record it
as "no issue found" if genuinely clean):

- **Scope** — does the spec match what the proposal asked for? Anything extra?
  Anything missing? Is "done" unambiguous?
- **Tech choices** — is the stack in design.md the right fit? Are there simpler
  alternatives that satisfy all the observable behaviours in the spec?
- **Missing scenarios** — are there observable behaviours implied by the proposal
  but absent from the spec? Error paths? Boundary conditions?
- **Testability** — can every scenario fail in RED with no production code?
  Do Then-steps assert concrete values, not "it ran"? Does design.md name a
  concrete strict-mode mechanism, not-implemented stub pattern, and
  single-scenario run command? Missing or vague on any of the three is a
  blocking finding — this is the root cause of past fabricated changes.
- **E2E fidelity** — for each capability, does design.md name the actual
  interface type (browser UI / CLI / API-only) and a matching real driving
  mechanism? For a browser-UI capability, is a real browser driver named —
  not an HTTP client or in-page `fetch()` standing in for it? Is there
  exactly one `@e2e` scenario per distinct happy-path action, not one per
  variant/error case? Vague interface type, an HTTP/fetch()-based plan for
  a browser UI, or more than one `@e2e` scenario per action is a blocking
  finding.
- **Risk** — what is the single most likely way this plan fails during
  implementation? What is the mitigation?
- **Architecture documentation (arc42)** — skip this branch entirely (report
  "N/A — arc42 addon not enabled") if `addons.arc42` is not `true` in
  `givn/config.yaml`. Otherwise:
  - Does `arc42.md`'s claim about which chapters are affected match the
    actual scope of `proposal.md`/`design.md`? Cross-check against the
    chapter-selection table in the arc42-docs instruction (new
    components/modules → building-block-view, new runtime flows →
    runtime-view, deployment changes → deployment-view, etc.). If
    `arc42.md` claims "no chapters affected" but design.md describes new
    modules, new runtime flows, or a non-trivial tech decision, that claim
    is implausible — treat it as a blocking finding, not a resolved item.
  - For every chapter `arc42.md` claims to have created/updated: open it and
    confirm the content actually reflects design.md — not placeholder text,
    not a stale description that contradicts design.md's Architecture
    Impact or Technology Decisions sections. A contradiction is a blocking
    finding.
  - For every non-trivial architecture/technology decision in design.md
    (a real tradeoff was made, not a default/obvious choice): confirm
    chapter 09 (architecture-decisions) has a corresponding MADR entry
    (status/date/decision-makers, considered options, decision outcome,
    consequences). A non-trivial decision with no MADR entry is a blocking
    finding.
  - A chapter file still containing only its scaffolded placeholder text,
    while `arc42.md` claims that chapter was "updated," is a blocking
    finding — the same fabrication pattern as an empty step body reported
    as passing.

## Return format

Return a JSON array of objects, ordered by priority (most critical first):

[
  {
    "branch": "<branch name>",
    "question": "<the question>",
    "recommended": "<your recommended answer>",
    "codebase_finding": null   // or a string if you resolved it by exploring
  },
  ...
]

Also return a separate array of items you resolved by codebase exploration
(so the orchestrator can report them):

{
  "questions": [ ... ],
  "resolved_by_codebase": [
    { "branch": "...", "finding": "..." }
  ]
}

Return only the JSON — no prose before or after it.
```

### 4. Interactive grilling loop

Wait for the grilling subagent to return.

If any `resolved_by_codebase` items were found, report them first:

```
## Design review: <id>

**Resolved by codebase exploration:**
- [Branch]: <finding>
```

Then work through `questions` one at a time:

For each question:
1. Present it to the user:
   ```
   **Q<N> (<branch>):** <question>
   **Recommended:** <recommended answer>
   ```
2. **Wait for the user's response.** Do not present the next question until
   the current one is answered.
3. Record the outcome: agreed / overridden (note what the user decided) /
   deferred (flag as open question).
4. Ask: "Blocking or non-blocking?" Record the answer. Blocking means this
   question must be resolved before implementation can proceed. Non-blocking
   means it can be deferred.

When all questions are answered, summarise:

```
**Grilling complete.** <N> questions answered.
Open questions (blocking): <list or "none">
Open questions (non-blocking): <list or "none">

Proceeding to hardening...
```

If any open question is marked as blocking by the user, stop and report the
blocker. Do not proceed to hardening until the user resolves it.

### 5. Spawn the hardening subagent

Use the **Task tool** to spawn a hardening subagent in a clean context.

Pass a self-contained prompt that includes the full grilling outcome:

```
You are a hardening subagent for givn change '<id>'.

The plan has been grilled. Your job: apply the decisions reached to the
existing artifacts and write design-review.md.

## Grilling outcome

### Resolved by codebase exploration
<paste resolved_by_codebase items>

### Questions and answers
<paste each question, recommended answer, and the user's decision>

### Open questions (non-blocking)
<list, or "none">

## Your tasks

### 1. Apply hardening edits

Based on the grilling outcome, edit the existing artifacts as needed:

- `givn/changes/<id>/design.md` — update if any tech decisions changed.
- `givn/changes/<id>/specs/**/*.feature` — add missing scenarios with
  `@givn.added @wip` if any were identified. Then run:
  ```sh
  givn lint --change <id>
  ```
  Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.
- Do NOT create tasks.md — tasks has not been written yet and is not your
  responsibility.
- Do NOT change proposal.md unless the grilling revealed a fundamental scope
  mismatch that the user explicitly decided to fix — flag that as a blocker.

### 2. Write design-review.md

Write `givn/changes/<id>/design-review.md` following this template structure:

## Grilling log
| # | Branch | Question | Recommended | Outcome |

## Resolved by codebase exploration
| Branch | Finding |

## Open questions
| # | Question | Blocking? |

## Architecture documentation (arc42) check
N/A if the arc42 addon is not enabled. Otherwise fill in — do not leave blank:
| Chapter | Claimed touched by arc42.md? | Content matches design.md? | Still placeholder text? |
- [ ] arc42.md's "chapters affected" claim checked against actual scope and found plausible.
- [ ] Every non-trivial architecture/technology decision in design.md has a MADR entry in chapter 09.
- [ ] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening
| Artifact | Change summary |

## Sign-off
- [ ] All branches walked.
- [ ] All open questions resolved or flagged as non-blocking.
- [ ] design.md reflects decisions reached.
- [ ] specs/*.feature updated for any missing scenarios.
- [ ] givn lint exits 0 or 2.
- [ ] Architecture documentation (arc42) check completed above, or marked
      N/A because the addon is not enabled.

DESIGN-REVIEW: PASS

Write `DESIGN-REVIEW: PASS` as the final line ONLY if no questions are marked
as blocking. If there are blocking open questions, write `DESIGN-REVIEW: FAIL`
instead and list the blockers.

### 3. Report back with
- Artifacts edited (list, or "none")
- Lint result (exit code)
- Whether DESIGN-REVIEW: PASS or FAIL was written, and why
- Path written: givn/changes/<id>/design-review.md
```

### 6. Wait for the hardening subagent to complete

If the subagent reports `DESIGN-REVIEW: FAIL`, surface the blockers to the user
and stop. Do not suggest next steps until blockers are resolved.

### 7. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-tasks` to write the task list."

---

## Output on completion

```
## Design review complete: <id>

**Grilling:** <N> questions across <M> branches
**Resolved by codebase:** <N> items
**Hardening:** <summary of edits, or "no changes needed">
**Open questions (non-blocking):** <list or "none">

**Review written:** givn/changes/<id>/design-review.md  (DESIGN-REVIEW: PASS)

**Next:** Run `/givn-tasks` to write the task list.
```

## Output on blocker

```
## Design review blocked: <id>

**Blocking question:** <question>
**Status:** DESIGN-REVIEW: FAIL

Resolve the blocker before proceeding.
```

---

## Guardrails

- The orchestrator presents exactly one question per turn. Never batch questions.
- Do not proceed to hardening while any question is unanswered.
- Do not proceed to hardening while any blocking open question is unresolved.
- The hardening subagent must not write tasks.md — that is the next artifact.
- Hardening edits do not reset design or specs status — they are ordinary edits.
- `DESIGN-REVIEW: PASS` requires no blocking open questions. If blockers remain,
  write `DESIGN-REVIEW: FAIL` and surface them to the user.
- `givn lint` must exit 0 or 2 after any .feature edits before signing off.
