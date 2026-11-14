# Proposal: addon-artifact-report

## Release note

> `givn addons list` reports for each addon and each agent target whether its materialized files are present or missing, and prints the literal command that resolves the difference.

## Use-Case Context

- Use-case ID: initialize-project
- Ideation topic: none
- Seed path: none
- Confirmed Personas: none — the change reports on existing commands and adds no new visual or interactive surface; the developer who runs `givn addons list` is the existing Actor of `initialize-project`, confirmed by the operator in this session.

## Review Context

- Review kind: self-review
- Reason: the change extends one reporting command; it adds no new artifact, no new command, and no lifecycle gate.

## Problem / Opportunity

The project records two independent facts about each addon, and only one is reported.

**The stated setting.** `givn/config.yaml` says whether an addon is enabled. `givn addons list` prints this.

**The materialized files.** Enabling an addon also writes files once, and never overwrites them: `docs/glossary.md`, `docs/domain-model.md`, `docs/arc42/`, `README.md`, `scripts/coverage.sh`, and the generated skill directory in each selected agent target. Nothing compares these files against the stated setting.

The result is states the developer cannot see:

- An addon is enabled but its files are gone. A clean checkout, a stray `rm`, or a `.gitignore` that drops `docs/` leaves the setting true and the files absent. The next command that needs the file fails with a path error unrelated to addons.
- The files are present but the addon is disabled. The remaining files look authoritative and are never refreshed.
- A skill addon is enabled, one agent target has its skill directory, and another selected target does not. The developer sees one harness behave differently from another with no explanation.
- `givn/config.yaml` still carries a retired key (`absolute_mode`, `grill_me`, `grill_with_docs`). These are silently ignored. The developer believes the behavior is on.

`givn addons list` today prints all enabled addons identically, so a broken project and a clean project produce the same output.

## Proposed Solution

`givn addons list` reports, for every registered addon, the stated setting and the materialized files, and prints one literal command for every difference it finds.

**Per addon, three states are named separately and never merged into one word:**

| Reported fact | Values |
|---|---|
| Stated setting | `enabled` or `disabled` |
| Materialized files | `present`, `missing`, or `partial` for addons that write several files or several agent targets |
| Suggestion | one literal command, or none when the two agree |

**The four differences and their suggestions:**

| Difference | Suggestion printed |
|---|---|
| Enabled, files missing | `givn addons enable <name>` (re-materializes without touching existing files) |
| Enabled, skill missing from some selected agent targets | `givn skills sync` |
| Disabled, files present | `givn addons disable <name>` names the files to remove, or re-enabling keeps them |
| Retired key in `givn/config.yaml` | replace it with the named current addon, naming the config key |

**Addons that write no files** (for example `communication_policy`, whose effect is a managed block inside `AGENTS.md`) report that their managed block is intact or damaged, mirroring the existing README managed-block check.

**Suggestions are never executed.** The report writes nothing and changes no configuration. Every suggestion is a command the developer runs.

**Exit status:**

| Outcome | Exit |
|---|---|
| Every addon's stated setting matches its materialized files | 0 |
| An enabled addon is missing materialized files | 1 |
| Files present for a disabled addon, or a retired key in config | 0 (informational, names the command) |
| `givn/config.yaml` is absent | 0, with the existing advice to run `givn init` |

## Capability Routing

`givn spec route` reported signal for `givn-workflow-enforcement` and `readme-enforcement`. Both matches come from the widespread `givn check` invocation shared by the corpus; neither capability owns addon state resolution, and the existing permanent `initialize-project/enable-optional-addons` already carries the `givn addons list` output scenarios.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| enable-optional-addons | EXTEND givn-workflow-enforcement | `EXTEND enable-optional-addons` | Deviation. Route matched on the `givn check` token, not on the subject. The scenarios extend `givn addons list`, whose existing scenarios live in `initialize-project/enable-optional-addons`, with `initialize-project` as the owning use case. |

## Out of Scope

- A new top-level `givn diagnose` command, and any aggregation of `readme check`, `lint`, `status`, and stale receipts.
- A `--fix` flag or any automatic repair.
- Repairing damaged managed blocks. The report names them; the developer edits.
- Reporting the installed binary's version against `givn_config_version`. `givn upgrade` owns that.
- Detecting files left behind by a removed agent target selection, beyond naming the targets that are selected but lack the skill.

## Open Questions

### D1 — Where do the materialized paths live?

- Question: The paths an addon writes are currently spelled inside each scaffold routine and are not declared; the skill directory name is a separate table again. How is the report given the paths without a second, drifting copy?
- Options:
  1. **Declare them on the addon registry entry** — each registered addon carries its materialized paths and, for skill addons, the generated skill name; the scaffold routines read the same declaration. Gain: one source of truth, and a test can assert every registered addon declares its paths. Cost: the registry grows a field for all nine addons, four of which write nothing and must say so explicitly. In practice: adding an addon without declaring its paths fails to compile.
  2. **Keep a check table beside the registry** — one function in the reporting path maps addon to paths. Gain: no change to the registry or scaffold routines. Cost: a second copy that drifts silently; nothing fails when an addon writes a new file the table does not know about. In practice: `scaffold_arc42` gains a chapter directory and the report keeps saying `present`.
- Disposition: carried — owner: the maintainer of `src/cli/addons.rs`; decided in `design.md`.
