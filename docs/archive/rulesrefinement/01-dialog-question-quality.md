# 01 — dialog-question-quality

Status: ready for `/givn-propose`
Source: terse unanswerable questions (T1 options A, B for `questions.md`, C)
Depends on: none

## Problem

givn's dialog content rule exists in exactly one place: the design-review
orchestrator. Ideation — where most user-facing questions actually happen —
plus explore, both grills, and propose's ambiguity step have no content rule
for questions. The templates that carry the records compress context away:
`questions.md` has no schema, the Grilling-log table has one `Question` cell,
and no change-seed template exists.

Agents fill the vacuum with token-optimal codes: lettered options whose
meanings live nowhere ("A now, C later, B possibly in between"), bare
cross-references ("(Q16)", "(R-033/R-035/R-052 family)", "(ADR-0013,
ADR-0016)"), and outcome-only resolutions ("Rule settled: B"). A reader must
open several other files and reconstruct the question from memory to answer
it. Some questions are not answerable at all.

## Evidence

givn side:

- The only content rule: `assets/commands/givn-design-review.md` lines
  191–195 and 237–253 (dialog discipline with a bad/good example), duplicated
  in `assets/skills/givn-design-review/SKILL.md.tmpl` lines 39–55.
- `assets/instructions/design-review.md` states the gate policy without the
  dialog-discipline wording.
- No content rule in: `assets/skills/givn-ideation/SKILL.md.tmpl`,
  `assets/skills/givn-explore/SKILL.md.tmpl`,
  `assets/commands/givn-ideation.md`, `assets/commands/givn-explore.md`,
  `assets/commands/grill-me.md`, `assets/commands/grill-with-docs.md`,
  `assets/commands/givn-propose.md` (ambiguity flagging).
- `assets/templates/design-review.md` line 5: `# | Branch | Question |
  Recommended answer | Outcome` — no context or rejected-alternative cell.
- `assets/templates/proposal.md` lines 39–41: Open Questions is a one-line
  prompt with no shape.
- `assets/addons/absolute-mode/policy.md` rewards compression and already
  carries a partial version of the rule (lines 14–22); it does not cover
  undefined letter options or unglossed cross-file citations.

kiekste side:

- `givn/ideation/interchangeable-building-blocks/questions.md` Q7: "Closed
  via Q12: **A now, C later, B possibly in between**" — options A/B/C are
  defined nowhere in the file. Same in Q2, Q12, Q16, `session.md`.
- `givn/ideation/run-authority/questions.md` Q3 references "astratesf's
  Repair", "the non-goal", "I1", "I4" without definition.
- `givn/ideation/interchangeable-building-blocks/changes/act-from-a-port.md`
  open questions are "(Q16)", "(Q16/Q5)".
- Counter-evidence that the rule works where it applies:
  `givn/changes/mobile-panel-order/design-review.md` grilling questions are
  full sentences with concrete values and named alternatives.

## Decided scope

1. Extract one canonical dialog policy; reference it from every interactive
   surface. (T1 A)
2. Give `questions.md` a fixed entry schema. (T1 B, questions part)
3. Fix the Grilling-log record shape and the design-review operator-decision
   convention. (T1 C)

## Required changes

### 1. New canonical policy `assets/instructions/dialog.md`

The whole `assets/` tree is embedded into the binary (`src/config/embedded.rs`,
`#[derive(Embed)] #[folder = "assets/"]`), and instruction resolution reads
`instructions/<id>.md` (`src/config/cascade.rs::resolve_instruction`).
Adding the file therefore makes `givn instructions dialog` work with no
manifest change. Verify this during design; if resolution rejects unknown
artifact ids, add the minimal wiring.

Required content, one page maximum:

- Applies to every interactive surface: ideation, explore, `grill-me`,
  `grill-with-docs`, propose ambiguity flagging, design-review grilling,
  review dispositions, archive confirmations.
- Rule 1: name every artifact in domain language with its identifier in
  parentheses. Keep the existing bad/good example from
  `givn-design-review.md` lines 241–253.
- Rule 2: every option states four things — what it means, what it gains,
  what it costs, and one concrete example of what the user sees or what the
  system does. Never a bare letter or abstract label. Worked example for the
  record shape:

  ```text
  Bad:  "Closed via Q12: A now, C later, B possibly in between."

  Good: "## Q12 — Remote capabilities vs the v1 posture

  - Status: closed 2026-09-15 (user)
  - Blocks: whether bindings may reach remote services in v1; shapes the
    binding model, credential handling, and the tracker write path.
  - Context: the composition plan allows six independently bound project
    services. Remote services need credentials and, for push delivery,
    inbound reachability. kiekste v1 runs locally with no server operation.
    The decision determines whether the binding model is built around
    local-only services or takes on remote infrastructure now.
  - Question: should v1 support bindings to remote services, and if so, in
    which form?
  - Options:
    1. Posture A — locally reachable bindings only.
       Meaning: every bound service runs on the same host as kiekste; no
       credentials are stored.
       In practice: a Fossil tracker binding points at a local repository; a
       hosted GitHub tracker is not selectable.
       Gain: no credential store, no inbound network surface, the simplest
       security model; the binding implementation stays small.
       Cost: remote trackers and chats cannot be used at all; bridging is
       manual.
    2. Posture B — outbound-only remote access.
       Meaning: kiekste may call remote services over outbound HTTPS with
       stored credentials; remote services cannot call kiekste.
       In practice: a GitHub tracker binding polls issues and posts comments;
       incoming answers arrive on the next poll.
       Gain: hosted services usable without inbound reachability; covers
       most tracker and chat needs.
       Cost: credential storage and rotation; answer latency equals the poll
       interval; no webhooks.
    3. Posture C — full remote integration.
       Meaning: remote services can call kiekste (webhooks, OAuth), enabling
       push delivery.
       In practice: a tracker comment reaches the engine immediately through
       a public endpoint.
       Gain: lowest latency, native integration.
       Cost: public endpoint, OAuth flows, secret management; reopens the
       v1 non-goal of server operation.
  - Resolution: Posture A now, Posture C later, Posture B possibly in
    between. Chosen: Posture A — v1 is local-only; the binding model must
    not be shaped around infrastructure that does not exist yet. Rejected
    now: Posture C reopens the v1 non-goal and requires server operation;
    Posture B adds credential handling and polling semantics to the first
    binding implementation without a user need that justifies them."
  ```

- Rule 3: every cross-reference (use case, question, ADR, invariant, risk,
  event number) carries a one-line gloss on first use per file or per turn.
  The reader must be able to answer without opening another file.
- Rule 4: a question is answerable standalone — it names the decision it
  blocks, the context that forces it, the options with their gains and
  costs, and a concrete example of each. The reader must be able to decide
  without opening another file and without re-deriving the trade-offs.
- Rule 5: record the rejected alternatives with the resolution, not only the
  chosen outcome.
- Rule 6: one question at a time; wait for the answer.
- A closing note: "Context required to answer a question is not filler;
  option codes without meanings are." This binds the absolute-mode addon.

### 2. Reference the policy from every interactive surface

Replace duplicated rule text with a reference; worked examples may stay.
Targets:

- `assets/commands/givn-design-review.md` — replace the inline dialog
  discipline (lines 237–253) with a reference; keep the bad/good example.
- `assets/skills/givn-design-review/SKILL.md.tmpl` — same.
- `assets/skills/givn-ideation/SKILL.md.tmpl` §"One question at a time" —
  reference plus the standalone-answerability rule.
- `assets/skills/givn-explore/SKILL.md.tmpl` and
  `assets/commands/givn-explore.md`.
- `assets/commands/grill-me.md`, `assets/commands/grill-with-docs.md`.
- `assets/commands/givn-propose.md` — the ambiguity/architecture step.
- `assets/addons/absolute-mode/policy.md` — add the "context is not filler"
  line; do not weaken the existing rules.

### 3. `questions.md` entry schema

Add the schema to the ideation skill next to the topic-state list
(`givn-ideation/SKILL.md.tmpl` lines 93–95), following the existing precedent
of the fenced use-case schema in the same file. Required shape:

```markdown
## Q<N> — <short title in domain language>

- Status: open | closed <date> (<decider>)
- Blocks: <the decision or work item this question blocks, one line>
- Context: <why the question exists — the problem, the constraints, and
  what changes depending on the answer, 2–5 sentences>
- Question: <full standalone question>
- Options:
  1. **<option name>** — <what it means>
     In practice: <concrete example of what the user sees or the system does>
     Gain: <what improves>
     Cost: <what gets harder or is given up>
- Resolution: **<chosen option>** because <reason>. Rejected: <option>
  because <reason>; <option> because <reason>.
```

A closed question keeps its full text, options, gains, costs, examples, and
rejected alternatives. Outcome-only resolutions are not a valid record.

### 4. Grilling log and operator decisions

- `assets/templates/design-review.md`: the Grilling-log table stays the
  scannable index (`# | Branch | Question | Outcome`). Every question's full
  record — context, options with meaning, gain, cost, and a concrete
  example, plus rejected alternatives — is written below it as a
  per-question block using the `questions.md` shape. The `Question` cell
  holds the short title, not the whole record.
- `assets/commands/givn-design-review.md` hardening prompt and the
  operator-decision convention: the presentation to the user carries the
  full option treatment (rule 2), and the record carries the rejected
  alternatives. The `authorize-build-start` record shape in kiekste (outcome
  only) is the non-example.

## Files to change

| Path | Change |
|---|---|
| `assets/instructions/dialog.md` | New canonical policy |
| `assets/commands/givn-design-review.md` | Reference policy; hardening prompt |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | Reference policy |
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Reference policy; questions schema |
| `assets/skills/givn-explore/SKILL.md.tmpl` | Reference policy |
| `assets/commands/givn-ideation.md` | Reference policy |
| `assets/commands/givn-explore.md` | Reference policy |
| `assets/commands/grill-me.md` | Reference policy |
| `assets/commands/grill-with-docs.md` | Reference policy |
| `assets/commands/givn-propose.md` | Reference policy at ambiguity step |
| `assets/addons/absolute-mode/policy.md` | "Context is not filler" line |
| `assets/templates/design-review.md` | Index table plus per-question detail blocks |

## Executable-spec impact

- `givn/specs/fragments/instruction-integrity.feature`: new scenario — the
  `dialog` instruction is retrievable through the CLI and its output contains
  the named-artifact rule, the option treatment (meaning, gain, cost,
  example), standalone answerability, and the rejected-alternative rule.
- `givn/specs/fragments/ideation-skill.feature`: extend the existing
  "one question at a time" scenario; assert the generated ideation guidance
  references the dialog policy and carries the question-entry schema.
- `givn/specs/fragments/skill-rewrites.feature`: update phrase assertions if
  the rewrites touch asserted guidance.
- Check whether `givn/specs/fragments/instruction-integrity.feature` or
  `src/cli/instructions.rs` tests enumerate known instruction ids; extend if
  the new `dialog` id must be listed.

## Acceptance criteria

- [ ] `givn instructions dialog` prints the canonical policy.
- [ ] Every listed surface references the policy; no surface keeps competing
      rule text.
- [ ] A question recorded as "Closed via Q12: A now, C later" violates the
      policy; the full record shape — context, per-option meaning, gain,
      cost, and a concrete example, plus rejected alternatives — is
      demonstrated in the policy.
- [ ] The `questions.md` schema is present in the generated ideation
      guidance.
- [ ] The design-review template records every question with its full record
      (context, options with meaning, gain, cost, example, rejected
      alternatives), not only a table row.
- [ ] Existing asserted phrases ("one question at a time") remain or their
      specs are updated in the same change.
- [ ] `givn lint` and `givn check review` behavior is unchanged.

## Out of scope

- T1 option D (separate answerability checklist at phase reviews).
- Any new runtime validation of question text; this remains authoring
  guidance plus the deterministic checks delivered in changes 2, 6, and 8.
- Rewriting existing kiekste artifacts.

## Risks and notes

- The rule text is loaded on many turns; keep it short. The worked example
  is the only long part — it may live in the `questions.md` schema section
  of the ideation skill and be referenced from the policy, if prompt size
  becomes a problem.
- The design-review surfaces currently own the only rule text. After
  extraction, confirm the generated command and skill still contain a
  pointer (their executable specs assert phrases such as "Overlap
  dispositions" — check before removing text).
- Do not touch `assets/instructions/design-review.md`'s gate policy beyond a
  pointer line; the gate content is owned by change 5.
