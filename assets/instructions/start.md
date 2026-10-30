# Instruction: start

Investigate the problem, question, or improvement and recommend a proportionate
next step. The default is to investigate: do not assume every modification requires a Givn change.

**This is a routing conversation, not a workflow.** It creates no workflow artifacts: no change directory, topic state, use case, proposal, specification, design, task list, or archive record. It makes no application changes. Once the
user authorizes a destination, hand off; work performed there is not an
artifact-producing phase inside start.

**Dialog policy:** for every user-facing question, follow the effective policy
from `givn instructions dialog`; load it before this context's first question.

## The four outcomes

Recommend one of these outcomes. None is a failure to adopt Givn.

| Outcome | Appropriate when | Concrete example |
|---|---|---|
| Answer and stop | Investigation answers the question, or establishes that no worthwhile modification is justified. | Explain an existing configuration option; reject a proposed change whose complexity is not justified by its benefit. |
| Direct work without a Givn change | The work is sufficiently understood and bounded that the formal lifecycle costs more than it contributes. | Correct a documentation typo; restore an existing specified behavior with a focused regression test. |
| `givn-ideation` | Domain understanding needs durable, structured development before selecting changes. | Establish actors, rules, terminology, and boundaries for team approvals. |
| `givn-propose` | A concrete improvement warrants the specification-driven lifecycle. | Introduce a new archive recovery behavior with observable guarantees. |

Answering without modifying anything and modifying the project without a formal
change are distinct outcomes. Both are explicit.

## Proportionality, not a size threshold

Judge what the formal workflow buys for the actual work and whether that benefit
justifies its cost. This is proportionality.

Possible benefits: clarifying disputed behavior, establishing a durable
contract, coordinating dependent work, exposing consequential decisions to
review, and preserving useful implementation evidence.

Costs: artifact production and maintenance, review effort, latency, and agent
usage, time, and money.

Vagueness alone does not require ideation. A vague request can become a clear,
bounded correction after inspecting the code. Ideation is warranted by domain
work that needs to persist, not by how polished the initial request is.

Propose does not require every requirement or design decision to be settled
beforehand; developing these is part of its responsibility.

A label is not an exemption. A one-line permissions fix may warrant
substantially more scrutiny than a larger documentation correction. Do not route
solely by line count, filename extension, or the user's use of the word
"bugfix".

## Direct work retains engineering obligations

Direct work skips formal change administration, not correctness or applicable
project rules. For direct work:

- Read the relevant code, existing specifications, documentation, and the
  applicable contract.
- Make a focused correction within the authorized scope.
- Add or adjust regression coverage where appropriate and run proportionate
  checks.
- Keep existing specifications and documentation accurate; skipping a new
  change directory does not permit contract drift.
- Report the modification and verification evidence, including material limits.
- Do not create a proposal, design, task list, or archive ceremony solely to
  justify taking this path.
- Remember that authorization to edit is not authorization to commit, push,
  archive, or perform unrelated work.

If investigation exposes disputed behavior, consequential scope, or domain
uncertainty, report that evidence and recommend reconsidering the path. Do not
silently promote a direct fix into a formal change.

## Recommendation and authorization

Recommend the destination and wait unless continuation is already authorized.

Example recommendation:

> The documented behavior is clear, and the defect is isolated to argument
> handling. I recommend a direct fix with a regression test, without creating a
> Givn change.

An ordinary request to discuss is not authorization to create a formal change,
persist ideation state, or implement a direct fix. An explicit request such as
"discuss this, then create the proposal" can authorize the stated continuation
without a redundant handoff confirmation. It does not authorize unrelated
actions or waive questions about genuinely unresolved consequential decisions.

Do not automatically enter whichever workflow you select after an ordinary
discussion. That can turn conversation into durable work without consent.

## Direct entry remains available

Retain direct access to `givn-ideation` and `givn-propose`, as well as existing
change continuation. Start is a default for new or unclassified work, not a gate
placed in front of every skill.

A clear, applicable request to perform a bounded direct correction can also
proceed without another start conversation. If the request is not sufficiently
understood or its path is uncertain, investigate and recommend rather than
guessing.

## Session behavior

1. Read the project state from `givn instructions`.
2. Relate it to the request:
   - Explicit continuation of an existing change: follow that change's next step.
   - New or unclassified work: use start.
   - Explicit, applicable ideation or proposal request: enter that skill directly.
   - Clearly authorized and sufficiently understood direct work: proceed without
     creating a Givn change; investigate or use start when the appropriate
     scope or path is uncertain.
3. Respect the authorization boundary of the selected path.

A reported lifecycle step does not authorize taking it. An active change does not
imply that every subsequent request belongs to that change. No active change
does not imply that a new one must be created.

## Investigation safeguards

- **Grounded** — inspect the actual codebase; do not just theorize.
- **Open threads** — surface multiple directions; let the user follow what
  resonates.
- **Visual** — use Mermaid diagrams when they clarify the problem (never ASCII
  art).
- **Failure budget** — after two identical failures from a tool or external
  source, stop and change strategy. Do not retry a cancelled search, a 404, or
  an unchanged command.
- **Minimize before retry** — when a parser or gate rejects an artifact, reduce
  it to a minimal reproducer. Change one dimension at a time.
- **Tool contract first** — read the supported dialect and capabilities before
  use. Use the simplest tool that can do the job.
- **Capture output** — write stdout/stderr to files, check the exit status, then
  inspect. Do not let `head` or `tail` mask or kill the producer.

## Handoff

When the discussion crystallises, state the recommended destination and wait for
authorization when it is still required. Derive a kebab-case change id when the
destination is `givn-propose`. If an active change exists and the discussion
revealed updates rather than new work, recommend the matching existing step
instead of creating a new change.
