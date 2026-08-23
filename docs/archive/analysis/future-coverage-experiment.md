# Future Experiment: Scenario-Level Branch Coverage

> **Status: reference archive.** This is an unimplemented optional experiment
> proposal retained as historical planning context. It contains no current
> coverage measurement and is not part of the active retrieval evidence.

**Status:** Future optional experiment. Not part of the current Givn production
extension and not part of the Cross-Encoder, NLI, or combined-corpus experiment
decision.

## Idea

Use branch coverage as an additional signal when reviewing new scenarios:

1. Measure coverage for the permanent specification set before a change.
2. Measure coverage for the projected specification set after the change.
3. Calculate newly covered branches.
4. Attribute those branches to individual new scenarios where the test runner
   supports isolated scenario execution.
5. Flag scenarios that add no marginal branch coverage for manual review.

The intended output is evidence, not an automatic deletion rule.

## Proposed Measurements

Let:

```text
B_permanent = branches covered by permanent scenarios
B_projected = branches covered by permanent scenarios plus the change
```

The suite-level coverage delta is:

```text
B_projected - B_permanent
```

For an individual scenario `S`, a leave-one-out measurement would compare:

```text
B_all          = coverage with all projected scenarios
B_without_S    = coverage with S removed
M_S            = B_all - B_without_S
```

`M_S` is the scenario's marginal branch contribution under the measured test
conditions.

## Why This Is Uncertain

Aggregate Cobertura XML often reports branch counts per line but not stable,
scenario-attributable branch identifiers. A reliable experiment would therefore
need a project-specific coverage contract with:

- stable branch identity;
- a command that can run one scenario or an isolated scenario subset;
- deterministic test setup and ordering;
- valid branch coverage, not only line coverage;
- a reproducible raw coverage artifact in addition to the display XML.

Running every scenario independently can be expensive or impossible for tests
that share state, use external services, depend on ordering, or require a full
application lifecycle.

## Interpretation Rules

No new branch coverage is a warning, not proof of redundancy. A scenario may
still protect an independent:

- JSON or text output contract;
- shell or transport boundary;
- security input boundary;
- lifecycle transition;
- error versus success contract;
- persistence or event contract.

Conversely, new branch coverage does not prove a scenario is valuable. A weak
scenario can execute a new branch without asserting the intended behavior.

The experiment should therefore combine coverage with deterministic facts,
semantic candidates, assertion analysis, and, if practical, mutation testing.

## Possible Givn Integration

This experiment could later add an opt-in coverage review contract to the project
manifest. Candidate project-owned commands would provide:

- a permanent baseline coverage run;
- a projected coverage run;
- an isolated scenario coverage run;
- a declared Cobertura/raw-coverage location;
- a declared branch identity scheme.

Projects without this contract would continue to use normal semantic and
deterministic review. Coverage would not be silently assumed for every Givn
project because Givn is test-runner-agnostic.

## Decision Boundary

This experiment must not automatically delete a scenario merely because
`M_S` is empty. It may produce findings such as:

```text
NO_INCREMENTAL_BRANCH_COVERAGE
```

The agent must then decide whether the scenario is redundant or protects a
distinct observable contract. Automatic enforcement would require a later
calibration study, scenario-level attribution, and mutation-quality evidence.
