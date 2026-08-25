---
status: accepted
date: 2026-06-19
decision-makers:
  - givn maintainers
consulted:
  - Gherkin workflow design record
informed:
  - contributors and users
---

# Make the `.feature` file the specification

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - introducing a second durable format would require migration and reconciliation of existing specifications.
- [x] **Shapes quality attributes** - one source improves correctness and reduces specification drift.
- [x] **Structural** - the feature file is simultaneously the delta, executable contract, and archived permanent record.
- [x] **Must be shared** - authors, runners, archive logic, and agents must treat the same file as authoritative.

## Context and Problem Statement

Parallel Markdown specifications duplicate the behavior that a Gherkin runner executes. givn needs a single durable representation that can be edited during a change, tested, and merged into the permanent corpus.

## Decision Drivers

- one source of truth
- executable Gherkin
- explicit add/modify/remove delta operations
- clean permanent output after archive

## Considered Options

- `.feature` as the only specification format.
- A Markdown spec plus a separately maintained executable test.
- A generated Markdown view with a separate source model.

## Decision Outcome

Chosen option: **The `.feature` file is the specification**. Active files use `@givn.delta` and scenario delta tags; archive strips `@givn.*` tags into runnable permanent Gherkin. Specification content remains implementation-independent; implementation and runner details belong in the design and task artifacts unless they are observable behavior.

### Consequences

- Proposals and design documents are supporting lifecycle artifacts, not parallel behavior specifications.
- A configured test runner must execute the feature files through a Gherkin/Cucumber-compatible path.
- The archive merge must preserve enough text to keep permanent files usable; replaced scenarios are rendered from the parsed AST.
- A feature must not encode private module, framework, or step-definition mechanics
  merely to make implementation convenient.

### Confirmation

Merge tests cover added, modified, and removed scenarios, tag stripping, missing-target failures, and empty-baseline creation; lint and runner tests exercise the feature boundary.

## Pros and Cons of the Options

### `.feature` only

- Good, because behavior is written and executed in one representation.
- Bad, because authors must keep Gherkin readable for both humans and runners.

### Markdown plus separate tests

- Good, because prose can be unconstrained.
- Bad, because the two sources can diverge.

### Generated view plus source model

- Good, because tooling could generate multiple views.
- Bad, because it adds a second synchronization boundary to a small CLI.

## More Information

Delta and merge mechanics are described in [chapter 8.3](../08-crosscutting-concepts.md#83-the-gherkin-delta-model).
