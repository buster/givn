# Instruction: dialog

The canonical dialog policy. It applies to every user-facing question an
agent asks in a Givn project — with or without a named workflow, while
clarifying, confirming, reviewing, or delegating. Ideation, explore,
`grill-me`, `grill-with-docs`, proposal ambiguity flagging, design-review
grilling, review dispositions, and archive confirmations are covered; the
obligation is not limited to them.

A question that cannot be answered from what is written is a defect, not a
style choice.

Write for a reader who is familiar with the project: assume its purpose,
vocabulary, and ordinary rules are known, and do not assume the reader holds
every referenced artifact in mind. Explain the decision-relevant contents,
not the whole project.

This policy is an authoring obligation, not runtime enforcement: check each
question against it before presenting it. A question tool's labels,
recommendation chip, or file links do not replace the context and
consequences the person needs before answering.

## Rule 1 — Ask only necessary, independent questions

Settle discoverable facts and confirmed constraints before asking. Ask only
unresolved human goals, priorities, authority, or trade-offs. Never re-ask a
confirmed direction without a new reason. Separate independent decisions,
avoid false dilemmas, and exclude alternatives that silently violate a
confirmed constraint. When the research already settles the action, report
the finding instead of asking a preference question.

## Rule 2 — Show the decision subject

Display the actual goals, relevant changes, boundaries, findings, or actions
being approved. Counts, ID ranges, document existence, and generic
directories do not stand in for those contents.

## Rule 3 — State the context briefly and point to a source

State why the choice matters now and what it blocks. Point to a source: the
relevant existing document and section, rule, scenario, or finding. The
source supplements the decision-critical facts shown in the question. If no
recorded source exists, say so honestly and show the assumptions; never
fabricate a citation.

## Rule 4 — Make every answer consequential

For each genuine alternative, state four things:
what it means, what it gains, what it costs, and one concrete example of
what choosing it causes. Confirmations show both accept and revise outcomes,
including what work or authority is and is not authorized: consent covers the
stated authorization only and does not waive the policy or authorize work
outside the question. Never present a bare letter or an abstract label: "A
now, C later, B possibly in between" is not an option; it is a code whose
meaning exists nowhere.

## Rule 5 — Disclose material later-change costs

Prefer revisable choices where suitable. If a decision changes stored
information, public expectations, access, or substantial downstream work,
explain the constraints and the work involved in changing it later, and name
the benefit purchased by the hard-to-change commitment. Establish unknown
recovery or compatibility facts before consent; never invent rollback
guarantees. Do not add ceremonial reversibility prose to an ordinary local
choice with no material commitment.

## Rule 6 — Match the treatment to the question

A short information request explains how the answer will be used and its
immediate effect; it does not invent alternatives or trade-offs. A
substantive choice receives the full treatment above. A question is
answerable standalone: it names the decision it blocks, the context that
forces it, the options with their gains and costs, and a concrete example of
each, so the reader can decide without opening another file and without
re-deriving the trade-offs.

## Rule 7 — Name artifacts in domain language

Name every artifact in domain language with its identifier in parentheses.
Never present a bare identifier or an abstract label.

<bad_example>
**Q1 (Scope):** Should UC1 use D2 or D3? ADR12 says no.
</bad_example>

<good_example>
**Q1 (Scope):** Should the password-reset use case (UC1) store reset tokens in
the database (D2) or derive them from a signed secret (D3)? The
stateless-sessions decision (ADR-012) argues against server-side state.
</good_example>

## Rule 8 — Gloss every cross-reference

Every cross-reference (use case, question, ADR, invariant, risk, event number)
carries a one-line gloss on first use per file or per turn. The reader must be
able to answer without opening another file.

## Rule 9 — One question at a time

Ask one question, wait for the answer, then continue. Never batch questions.

## Rule 10 — Record the rejected alternatives

The resolution records what was rejected and why, not only the chosen
outcome. "Rule settled: B" is not a resolution.

## The record shape

A question record keeps the full text, options, gains, costs, examples, and
rejected alternatives after it closes. An outcome-only resolution is not a
valid record.

Bad:

```text
Closed via Q12: A now, C later, B possibly in between.
```

Good:

```markdown
## Q12 — Remote capabilities vs the v1 posture

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
  1. **Posture A — locally reachable bindings only.**
     Meaning: every bound service runs on the same host as kiekste; no
     credentials are stored.
     In practice: a Fossil tracker binding points at a local repository; a
     hosted GitHub tracker is not selectable.
     Gain: no credential store, no inbound network surface, the simplest
     security model; the binding implementation stays small.
     Cost: remote trackers and chats cannot be used at all; bridging is
     manual.
  2. **Posture B — outbound-only remote access.**
     Meaning: kiekste may call remote services over outbound HTTPS with
     stored credentials; remote services cannot call kiekste.
     In practice: a GitHub tracker binding polls issues and posts comments;
     incoming answers arrive on the next poll.
     Gain: hosted services usable without inbound reachability; covers
     most tracker and chat needs.
     Cost: credential storage and rotation; answer latency equals the poll
     interval; no webhooks.
  3. **Posture C — full remote integration.**
     Meaning: remote services can call kiekste (webhooks, OAuth), enabling
     push delivery.
     In practice: a tracker comment reaches the engine immediately through
     a public endpoint.
     Gain: lowest latency, native integration.
     Cost: public endpoint, OAuth flows, secret management; reopens the
     v1 non-goal of server operation.
- Resolution: Posture A now, Posture C later, Posture B possibly in
  between. Chosen: Posture A — v1 is local-only; the binding model must
  not be shaped around infrastructure that does not exist yet. Rejected:
  Posture C reopens the v1 non-goal and requires server operation;
  Posture B adds credential handling and polling semantics to the first
  binding implementation without a user need that justifies them.
```

Context required to answer a question is not filler; option codes without
meanings are.
