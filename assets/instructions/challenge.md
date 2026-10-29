# Instruction: challenge

Challenge the subject with evidence before asking any question. The subject is
the proposal, tests, specs, design, code, or the whole change.

Apply the team's engineering principles from `givn instructions dev-principles`.
Reference them; do not copy them into a checklist here.

## Investigate first

Establish every discoverable fact before involving the person:

- Read the subject and the sources it references.
- Reproduce the suspected defect where the environment permits.
- Cite the source of each finding: a specification scenario, proposal line,
  design decision, code path, test assertion, command output, or recorded
  measurement.
{{#if addons.arc42}}
- Cite the relevant `docs/arc42/` chapter for architecture findings.
{{/if}}
- Label anything not established by evidence as a hypothesis and state what
  would confirm or refute it.

Do not ask the person to inspect material the agent can inspect itself. A
question that research can answer is a defect in the challenge.

## Report findings

Report discoverable defects as bounded, actionable findings: the observed
behaviour, the expected behaviour, the discriminating assertion or
counterexample, and the root cause when established. Include compatibility
and failure-recovery evidence where the subject affects stored information,
public behaviour, or recovery. Performance claims require an actual
measurement with its population and method.

Accepted modularity and fix-forward choices are not overridden by blanket
compatibility or rollback rules. Challenge those choices only with concrete
evidence of the cost the choice actually creates.

## Ask only unresolved decisions

After the evidence is in place, ask only unresolved human goals, priorities,
authority, or trade-offs. Follow the effective dialog policy from
`givn instructions dialog` and read the companion example for the question
type before the first such question in this context.

## Output

- Findings with their evidence and citations.
- Hypotheses labelled as hypotheses, not reported as facts.
- Unresolved decisions that genuinely need the person, with the evidence that
  leaves them open.
