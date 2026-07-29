# Iteration {{iteration}} / {{max_iterations}}

## Project task

{{prompt}}

## Givn workflow — one step at a time

You are executing the **givn** spec-driven workflow. Each Ralph iteration does
EXACTLY ONE givn lifecycle step. Do NOT jump ahead. Do NOT write production code
before the design says you should.

### Step 1 — Start

Run this command and do exactly what it tells you:

```
givn instructions
```

If the output tells you to run `givn propose`, do that. If it tells you to run
`givn design`, do that. If it tells you to run `givn implement --change <X>`,
do that. Only run the EXACT command givn instructions gives you.

### Step 2 — Follow the command's skill

Each givn command loads a skill with detailed instructions. Follow the skill's
workflow precisely. In particular:

- **givn implement**: Work through ONE scenario from `tasks.md` at a time.
  Write the RED test first, watch it fail, then write the minimal GREEN
  implementation, then REFACTOR. Commit after each scenario.
- **givn archive**: Only run when all tasks are checked off and all tests pass.
  The archive command runs gate hooks — do NOT skip them.

### Step 3 — Verify and signal

After completing one lifecycle step or one scenario (for implement):

```
givn instructions
```

If `givn instructions` tells you the work is NOT done, output this promise and
let the next iteration continue:

<promise>{{task_promise}}</promise>

If `givn instructions` tells you all changes are archived, output the final
promise:

<promise>{{completion_promise}}</promise>

## Rules

- One scenario per iteration during `givn implement`. Never batch scenarios.
- Always run `givn instructions` at the start of every iteration.
- Never write production code before a failing RED test.
- Always commit after each scenario with Conventional Commits.
- Do NOT edit givn specs, design, or task files unless a givn command tells
  you to.
- If a command or skill asks you a question, answer it. Answer ONE question
  at a time.

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