# Instruction: dialog

The canonical dialog policy. It applies to every user-facing question an
agent asks in a Givn project — with or without a named workflow, while
clarifying, confirming, reviewing, or delegating. Ideation, explore, proposal ambiguity flagging, challenge findings,
design-review grilling, review dispositions, and archive confirmations are
covered; the obligation is not limited to them.

A question that cannot be answered from what is written is a defect, not a
style choice.

Write for a reader who is familiar with the project: assume its purpose,
vocabulary, and ordinary rules are known, and do not assume the reader holds
every referenced artifact in mind. Explain the decision-relevant contents,
not the whole project.

This policy is an authoring obligation, not runtime enforcement and not an
automatic autoload claim: load it with `givn instructions dialog` once per
context and reload it if lost, then check each question against it before
presenting it. A question tool's labels,
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

Prefer revisable choices where suitable.
{{#if addons.architecture_principles}}
For an architecture or design question, `givn instructions
architecture-principles` owns the contract and component rules and the
decision-timing classification; this policy owns how the question is presented.
{{/if}}
If a decision changes stored information, public expectations, access, or
substantial downstream work,
explain the constraints and the work involved in changing it later, and name
the benefit purchased by the hard-to-change commitment. Establish unknown
recovery or compatibility facts before consent; never invent rollback
guarantees. Do not add ceremonial reversibility prose to an ordinary local
choice with no material commitment.

## Rule 6 — Match the treatment to the question

A short information request explains how the answer will be used and its
immediate effect; it does not invent alternatives or trade-offs.
A substantive choice receives the full treatment above.
A question is answerable standalone: it names the decision it blocks, the
context that forces it, the options with their gains and costs, and a concrete
example of each, so the reader can decide without opening another file and
without re-deriving the trade-offs.

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
outcome. "Rule settled: B" is not a resolution. Record the chosen outcome as
`Resolution:` and the alternatives as `Rejected:` with their reasons; an
outcome-only resolution is not a valid record.

## Companion examples

`givn instructions dialog-examples` holds the record shape and the regression
examples. Before the first question of a type in each context, read that
type's example; questions of a type already read in this context need no
reread. The companion supplements this policy and is not loaded automatically.

| Question type | Companion example |
|---|---|
| Scope approval | Example 1 decision record |
| Target selection | Example 2 decision record |
| Behavior choice | Example 3 decision record |
| Persistent-format commitment | Example 4 decision record |
| Discoverable diagnosis | Example 5 decision record |
