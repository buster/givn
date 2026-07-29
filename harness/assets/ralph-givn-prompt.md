# Iteration {{iteration}} / {{max_iterations}}

## Project task

{{prompt}}

## Your ONE job this iteration

You have EXACTLY ONE action this iteration. Read everything below
before doing anything.

### 1. Discover what to do

Run this command and read its output carefully:

```
givn instructions
```

It will tell you exactly ONE command to run — for example
`givn propose`, `givn spec --change <X>`, `givn implement --change <X>`,
or `givn archive --change <X>`.

### 2. Execute THAT ONE command

Run the exact command `givn instructions` told you. Follow the skill's
workflow. But remember:

- **propose / spec / design / design-review / tasks / review**:
  Complete the artifact and commit. That is ONE step.

- **givn implement**:
  Work through ONE scenario from `tasks.md` — RED first (watch it fail),
  then minimal GREEN, then REFACTOR. Commit after that ONE scenario.

- **givn archive**:
  Only run when all tasks are checked off and all tests pass.

### 3. STOP and signal

After completing that ONE step (one artifact or one scenario):

- If `givn instructions` would tell you there is more work to do,
  output this EXACT text and STOP:

  <promise>{{task_promise}}</promise>

- If all changes are archived and done, output:

  <promise>{{completion_promise}}</promise>

Do NOT run `givn instructions` again in this iteration.
Do NOT start the next lifecycle step.
Do NOT peek ahead at what comes next.
The NEXT iteration will run `givn instructions` fresh.

## Rules

- ONE givn lifecycle step per iteration. Never batch.
- ONE scenario per iteration during `givn implement`.
- Never write production code before a failing RED test.
- Always commit after each scenario with Conventional Commits.
- Do NOT edit givn specs, design, or task files unless a givn command
  tells you to.

## Blocked — when you need human input

If a givn command asks a question that cannot be answered from the task alone
(design-review grilling, missing spec decisions, ambiguous requirements):

1. Write the question to `BLOCKED.md` in this workspace:

```
## Question: <short topic>

<exact question text>

---
```

2. Then output exactly: COMPLETE

When you restart and `BLOCKED.md` contains answers below the `---` separator
(see the blocked context below), use those answers and continue.

{{blocked}}{{context}}{{tasks}}