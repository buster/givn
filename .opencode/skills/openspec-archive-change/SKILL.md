---
name: openspec-archive-change
description: Archive a completed change in the experimental workflow. Use when the user wants to finalize and archive a change after implementation is complete.
license: MIT
compatibility: Requires openspec CLI.
metadata:
  author: openspec
  version: "1.0"
  generatedBy: "1.4.1"
---

Archive a completed change in the experimental workflow.

**Input**: Optionally specify a change name. If omitted, check if it can be inferred from conversation context. If vague or ambiguous you MUST prompt for available changes.

**Steps**

1. **If no change name provided, prompt for selection**

   Run `openspec list --json` to get available changes. Use the **AskUserQuestion tool** to let the user select.

   Show only active changes (not already archived).
   Include the schema used for each change if available.

   **IMPORTANT**: Do NOT guess or auto-select a change. Always let the user choose.

2. **Check artifact and task completion status**

   Run `openspec status --change "<name>" --json` to check artifact completion.

   Parse the JSON to understand:
   - `schemaName`: The workflow being used — determines archive strategy (see step 4)
   - `planningHome`, `changeRoot`, `artifactPaths`, and `actionContext`: path and scope context
   - `artifacts`: List of artifacts with their status (`done` or other)

   If status reports `actionContext.mode: "workspace-planning"`, explain that workspace archive is not supported in this slice and STOP. Do not move workspace changes into repo-local archives or edit linked repos.

   **If any artifacts are not `done`:**
   - Display warning listing incomplete artifacts
   - Use **AskUserQuestion tool** to confirm user wants to proceed
   - Proceed if user confirms

   Read the tasks file (typically `tasks.md`) to count incomplete tasks (`- [ ]`).

   **If incomplete tasks found:**
   - Display warning showing count of incomplete tasks
   - Use **AskUserQuestion tool** to confirm user wants to proceed
   - Proceed if user confirms

   **If no tasks file exists:** Proceed without task-related warning.

3. **Pre-archive GREEN gate** (schema: `spec-driven-gherkin` only)

   If `schemaName` is `spec-driven-gherkin`:

   Run:
   ```bash
   openspec verify "<changeRoot>/specs/"
   ```

   - If exit code is **1** (parse error): STOP. Show error output. Tell the user to fix syntax errors before archiving.
   - If exit code is **2** (PENDING — @wip scenarios remain): STOP. Tell the user all @wip tags must be removed before archiving. Show which scenarios are still PENDING.
   - If exit code is **0** (GREEN): proceed.

   For all other schemas: skip this step and proceed.

4. **Perform the archive**

   **If `schemaName` is `spec-driven-gherkin`:**

   Run the OpenSpec archiver — it merges `.feature` delta files into `openspec/specs/` automatically:
   ```bash
   openspec archive "<name>" --yes
   ```

   Parse the output. If the command fails, STOP and show the error. Do not proceed to step 5.

   **For all other schemas:**

   Use `artifactPaths.specs.existingOutputPaths` from status JSON to check for delta specs. If none exist, proceed without sync prompt.

   If delta specs exist:
   - Compare each delta spec with its corresponding main spec at `openspec/specs/<capability>/spec.md`
   - Determine what changes would be applied (adds, modifications, removals, renames)
   - Show a combined summary before prompting
   - Prompt options: "Sync now (recommended)", "Archive without syncing"
   - If user chooses sync, use Task tool (subagent_type: "general-purpose", prompt: "Use Skill tool to invoke openspec-sync-specs for change '<name>'. Delta spec analysis: <summary>"). Proceed to archive regardless of choice.

   Then move the change directory:
   ```bash
   mkdir -p "<planningHome.changesDir>/archive"
   mv "<changeRoot>" "<planningHome.changesDir>/archive/YYYY-MM-DD-<name>"
   ```

5. **Post-archive regression check** (schema: `spec-driven-gherkin` only)

   If `schemaName` is `spec-driven-gherkin`:

   Run:
   ```bash
   openspec verify "<planningHome.root>/openspec/specs/"
   ```

   - If exit code is **0**: all good — report GREEN.
   - If exit code is **1** or **2**: report the failure. The archive already completed, but warn the user that the permanent specs have a problem that needs attention. Show the full verify output.

   For all other schemas: skip this step.

6. **Display summary**

   Show archive completion summary including:
   - Change name
   - Schema that was used
   - Archive location
   - Specs merged (spec-driven-gherkin) or sync status (other schemas)
   - Regression check result (spec-driven-gherkin only)
   - Note about any warnings (incomplete artifacts/tasks)

**Output On Success**

```
## Archive Complete

**Change:** <change-name>
**Schema:** <schema-name>
**Archived to:** <planningHome.changesDir>/archive/YYYY-MM-DD-<name>/

### Specs merged
  + <capability>: N added  (or ~ modified / - removed)

### Regression check
✓ openspec/specs/ — N scenarios GREEN

All artifacts complete. All tasks complete.
```

**Guardrails**
- Always prompt for change selection if not provided
- For `spec-driven-gherkin`: NEVER use `mv` directly — always use `openspec archive` so the `.feature` merge pipeline runs
- For `spec-driven-gherkin`: NEVER archive a change with @wip scenarios — enforce the GREEN gate
- For `spec-driven-gherkin`: always run the post-archive regression check and report the result
- Don't block archive on artifact/task warnings — just inform and confirm
- Preserve .openspec.yaml when moving to archive (it moves with the directory)
- Show clear summary of what happened
