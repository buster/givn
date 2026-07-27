# Proposal: givn-test-harness

## Problem / Opportunity

givn ships a suite of AI-agent-facing assets: skill templates, artifact
instructions, a CLI interface, and a configurable workflow. These assets are
the primary surface through which an agent interacts with givn. Today, the
only way to verify that these assets successfully guide an agent through the
full propose-to-archive lifecycle is manual dogfooding — a human runs each
workflow step, inspects outputs, and judges quality ad hoc.

Manual testing has three structural problems:

1. **Inconsistent.** Each test run is a unique interaction with an LLM,
   producing different quality assessments and different failure conditions.
   Without a repeatable benchmark, "this skill works" is an anecdote, not a
   measurement.

2. **Slow feedback.** A full end-to-end workflow takes hours of agent time.
   Waiting for a human to notice a regression after an asset change means
   defects live undetected until the next manual run.

3. **No quality signal.** Changes to skill templates, instructions, or CLI
   behaviour are committed without evidence that they improve or degrade
   workflow outcomes. There is no feedback loop between asset quality and
   workflow success.

The opportunity: build an automated test harness that treats givn's
agent-facing assets as the system under test, runs complete autoresearch
loops against realistic tasks, and produces objective quality measurements.

## Proposed Solution

A Python CLI — `givn-harness` — that runs automated end-to-end tests of the
givn workflow. Each test run executes a complete autoresearch loop: an AI
agent (opencode, driven by ralph) receives a task description and must carry
it from proposal through archive using givn's workflow.

### What the harness does, observably

**Isolated test runs.** Every run creates a fresh, temporary workspace. No
run touches the givn repository itself. The workspace is initialized with
`git init` and `givn init`, giving the agent a realistic starting state
where `verify.command` points at the sentinel and must be reconfigured.

**Agent-driven workflow.** The task prompt is delivered as a file in the
workspace. ralph invokes opencode with an LLM and drives it through
iterations until completion (or timeout/max-iterations). The agent must
interpret the task, configure the test runner, produce every givn artifact,
implement the solution, pass the review gate, and archive the change.

**Automated evaluation.** Every run produces a report with objective
pass/fail checks and subjective quality scores:

- **10 automated checks** verify that required artifacts exist, lint
  passes clean, task checkboxes are all done, valid commits were produced,
  archive succeeded, permanent specs are clean Gherkin, step definition
  files are non-empty, and the sign-off markers REVIEW: PASS and
  DESIGN-REVIEW: PASS are present. A final check confirms `verify.command`
  exits zero (the test suite is green).

- **LLM-as-a-judge scoring** rates five artifacts (proposal, specs, design,
  tasks, review) on a 0-4 rubric with written explanations. This catches
  quality problems that structural checks miss: a proposal that exists but
  is empty, specs that parse but describe implementation detail instead of
  behaviour, a design with no strict-mode proof.

**Per-run and aggregate reporting.** Each run writes a markdown report with
all checks, scores, agent logs, and workspace path. A cross-run index (JSON)
accumulates results across runs so that successive runs against different
tasks (or after asset changes) produce a comparable record.

**Sandboxed agent execution.** The agent runs inside a sandlock sandbox with
filesystem isolation (learn-then-confine: the first run learns which paths
the workload touches, subsequent runs enforce the learned profile) and
default-allow networking. Docker, Playwright, and typical E2E tooling must
work inside the sandbox.

### First task

"Create a website which can be used to manage a shopping list as a family.
There does not need to be a login. A shared password is fine. It shall be
easy to run, mobile friendly and beautiful. Use the givn workflow framework
until changes are archived."

## Out of Scope

- **Multi-project-type diversity.** The MVP runs one task. Running tasks
  against different project types (Python, Rust, Node.js) is deferred.

- **Self-improvement loop.** The harness does not auto-propose changes to
  givn's assets based on test results, re-test with the modified assets,
  and commit/revert.

- **CI/CD integration.** The harness runs as a local CLI. No CI integration
  is designed yet.

- **Benchmark history DB.** The cross-run index is a JSON file, not a
  queryable database with historical trend analysis.

## Open Questions

None. The requirements are specified to sufficient detail in the task
description.
