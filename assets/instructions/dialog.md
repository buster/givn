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

## Worked contrasts

The five contrasts below illustrate different question shapes; they are not a
mandatory verbose template. Every record, link, file and setting in them is
hypothetical: the in-file links point at the example's own record below, not
at a real project file, implemented recovery mechanism, or current product
behavior. Nothing here adopts the examples' hypothetical decisions.

### Example 1 decision record

Hypothetical neighborhood-library planning record: three proposed member goals
are reserve an available copy, cancel one's own reservation, and view one's
queue position. Staff acquisition and catalog editing are excluded.
Confirmation authorizes elaborating these goals, not implementation, change
count, or delivery order. Changing a goal later requires revisiting its
affected elaboration, not treating the original confirmation as irrevocable.

#### Bad — approving a count instead of the goals

> The cut contains two changing goals and one observing goal. It is not three
> changes or a delivery order. Adopt this cut for elaboration?

#### Good — show the whole approval subject

> Should these three member goals become the basis for use-case elaboration?
>
> We need their boundaries before drafting flows. Details:
> [the planning record](#example-1-decision-record).
>
> | Proposed goal | Member outcome | Boundary |
> |---|---|---|
> | Reserve an available copy | A copy is reserved for the member. | Does not acquire new books. |
> | Cancel one's own reservation | The member's reservation is removed. | Does not cancel another member's reservation. |
> | View one's queue position | The member can see their position. | Read-only; does not edit the catalog. |
>
> **Adopt this cut:** elaborate these three goals with those boundaries. For
> example, canceling another member's reservation is not added to the cancel
> flow. Benefit: a bounded basis for elaboration; cost: omitted goals need
> separate consideration. This does not approve implementation or delivery
> order.
>
> **Revise the cut first:** identify a goal or boundary to change before
> elaborating it. For example, if staff cancellation is needed, settle its
> actor and outcome first. Benefit: avoids elaborating the wrong scope; cost:
> delays the affected flow.
>
> Goals can be amended later, but affected rules and flows must be reviewed
> again. Adopt these goals or revise a named boundary first?

Contrast: all approved goals are visible; the count is not the evidence, and
both answers have bounded consequences.

### Example 2 decision record

Hypothetical target-selection record: both `/work/catalog` and
`/work/catalog-tools` are initialized projects the operator owns. The
repository inspection identifies both but cannot establish which project the
operator intends to work on. Selecting one sets the scope for subsequent
commands; selecting it alone writes nothing. Neither current-directory
inference nor another artifact settles the intent.

#### Bad — an unexplained information request

> Which directory?

#### Good — short clarification, no artificial option ceremony

> Both `/work/catalog` and `/work/catalog-tools` are initialized projects;
> inspection cannot tell which one you intend to change. Details:
> [the inspected target record](#example-2-decision-record).
>
> Which project root should subsequent commands use? For example, answering
> `/work/catalog` limits the work to the catalog project, not catalog-tools.
> Selecting a root alone writes nothing; we can switch before any write.

Contrast: the agent establishes discoverable facts first, asks only the
missing human intent, and explains how the answer will be used without
invented gains or costs for arbitrary paths.

### Example 3 decision record

Hypothetical rule-reader project: general collaboration guidance can be read
through `tool instructions dialog`; task-specific guidance is read through
`tool instructions task`. No decision yet settles whether general guidance
requires an active task. Task-specific guidance does require one. The draft
rejects all instruction requests when no task exists. No data migration or
permission change is involved. Altering an established command expectation
later may require updating callers and guidance.

#### Bad — a preference poll about implementation labels

> D1 is open. Fallback first or task resolution first? Recommend fallback.

#### Good — decide observable behavior

> Should general collaboration rules be readable before an active task exists?
>
> The draft rejects `tool instructions dialog` with `No active task`, which
> prevents an agent from reading the collaboration rules before planning.
> Details: [the draft behavior and boundary](#example-3-decision-record).
>
> **Allow general guidance without a task — recommended.** The command prints
> the collaboration rules; task-specific instructions still require a task.
> Example: in a fresh project, `tool instructions dialog` prints the rules
> while `tool instructions task` returns `No active task`. Benefit: rules are
> available before planning. Cost: lookup must distinguish general from
> task-specific guidance.
>
> **Require a task for both.** Example: both commands return `No active task`
> until the operator creates or selects a task. Benefit: one prerequisite for
> all lookup. Cost: the agent cannot retrieve the rules through this command
> before planning.
>
> No stored information changes. The behavior is revisable, but callers and
> guidance relying on it may need updates. Which availability rule should the
> project establish?

Contrast: the reader decides a real product expectation, not an internal
ordering detail. The source is specific, both results are shown, and the
local choice does not receive migration-style rollback ceremony. If the
project already confirms availability, report and apply that decision instead
of asking again.

### Example 4 decision record

Hypothetical saved-project upgrade: the old format records a preview setting
called Backend Port; the proposed format calls the same setting Candidate
Port. The port value is preserved. If approval includes upgrading saved
projects, the conversion work becomes part of the approved change rather than
a later follow-up. No implemented recovery support is assumed. Under the
stated assumptions, older releases cannot open upgraded projects; downgrading
therefore needs a separately verified reverse conversion, and merely
reinstalling an old release is insufficient. That recovery is
required work, not demonstrated support.

#### Bad — hiding a lasting commitment in a label

> Include the saved-format rename now or later? Recommend now for consistency.

#### Good — show the hard-to-change cost and its purchased benefit

> Should this terminology change include upgrading existing saved projects?
>
> Documentation will call the preview setting Candidate Port. We must choose
> whether saved projects change with it. Details:
> [the upgrade and compatibility record](#example-4-decision-record).
>
> **Upgrade saved projects in this change.** Example: a project saved as
> Backend Port = 8080 becomes Candidate Port = 8080; the number stays the
> same. Benefit: documentation and saved projects use one name, so future work
> does not translate between them. Cost: implement and verify conversion
> before rollout. Changing course after rollout requires verified reverse
> conversion, not just reinstalling the old release.
>
> **Keep saved projects unchanged for now.** Example: documentation says
> Candidate Port = 8080 while the saved project still calls it Backend Port.
> Benefit: this change does not introduce that format-related downgrade
> restriction. Cost: the project must maintain the documented distinction;
> later alignment still needs a conversion.
>
> Both options preserve the configured port. The second leaves conversion open;
> the first purchases immediate consistency with a harder-to-reverse format
> commitment. Include the conversion, or leave saved projects out of this
> change?

Contrast: the difficult option states its benefit and compatibility cost; the
safer option is concrete. The example does not promise an unimplemented
rollback. If real compatibility or recovery constraints are unknown,
establish them or narrow approval to investigation before asking for rollout
consent.

### Example 5 decision record

Hypothetical review record: a status scenario succeeds whenever the output
contains the word active. Its declared requirement is that the project status
is active. A response saying the project is inactive while an active-profile
label exists still passes the substring check. The recorded contract already
requires checking the actual project status. The assertion weakness and remedy
are discoverable; no policy choice remains. A separate confirmed rule also
prohibits processing one repeated write as two answers.

#### Bad — delegating diagnosis or offering an invalid option

> Which of the 22 status scenarios proves nothing? Should we keep the broad
> substring check or choose exact whole-output matching?
>
> Alternatively, should a repeated answer become a second answer so a human
> can decide? The run might continue twice.

#### Good — do the research and report; do not ask a preference question

> The status check accepts an inactive project when an unrelated label contains active.
> Details: [the assertion counterexample and status contract](#example-5-decision-record).
>
> I will make the assertion distinguish the actual project status while
> preserving the existing output contract. The inactive-project counterexample
> will fail; an active project with the required status will pass. This
> changes verification, not product behavior. Whole-output formatting is not
> required by the supplied contract.
>
> I will not offer processing a repeated write as a second answer: that would
> violate the confirmed one-answer rule. If a future request intentionally
> changes that rule, it needs a separate explicit requirement decision and its
> consequences, not a silent exception.

Contrast: investigate unseen assertions instead of asking the human to
memorize them; reject a false dilemma and an invariant-violating alternative.
The policy's good outcome is sometimes a report with no user-facing question.
