# Handoff: event-record-quality

Planning input for the givn agent. This is not a givn artifact. Use it to write
the specs, design and tasks.

## Evidence

Real local record, 13,795 events:

- 2,653 linked usage events duplicate inline turn usage;
- 8 question events contain text/wait but no answer;
- failed commands often carry only `exit_code`, no error text;
- some usage/subagent records omit model identity or report usage unavailable;
- analyzer currently counts one usage representation and refuses to infer
  answers, which is correct but leaves raw-record ambiguity.

## Source references

- `src/history.rs` — event envelope, usage linkage, bounded content.
- `src/cli/history.rs` — intake and question records.
- Existing event schema in `givn/history/events.jsonl`.

## Proposed design shape

- Emit one authoritative usage object per request. If a linked usage event is
  retained, mark it `derived: true` or `usage_of: <turn-id>`.
- On answer, add selected option, source (human/agent/policy), and timestamp.
  Do not infer approval from silence.
- On a failed command, record a bounded first error line or stderr excerpt with
  provenance. Keep full output opt-in.
- Include provider/model/variant on usage and subagent records when known.
  Keep `usage: unavailable` explicit.
- Keep changes additive; old records and the analyzer must remain compatible.

## Acceptance criteria

- The analyzer can count usage without a duplicate-usage rule for new records.
- Question answers are attributable and silence remains unrecorded.
- Failed commands have bounded error evidence.
- Model identity is recorded where known; unavailable stays unavailable.
- No breaking event-schema version change is required.
- Existing logs remain readable.

## Risks

- Larger records if full stderr is captured; default bounded.
- Secret leakage in error text; redact or bound.
- Linked usage compatibility for existing consumers.
- Do not turn derived usage into double counting.

## Suggested scenario areas

- a turn emits one usage representation;
- a retained linked usage is marked derived and not counted twice;
- an answered question records selection, source and time;
- an unanswered question stays unanswered;
- a failed command records a bounded error line;
- unavailable usage/model stays explicit.
