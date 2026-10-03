# Proposal: require-agent-target

## Release note

> Required. One complete, user-facing sentence naming what this change does.

`givn init` refuses to initialize a project unless at least one agent target
is selected, instead of silently installing OpenCode skills.

## Use-Case Context

- Use-case ID: initialize-project
- Ideation topic: none
- Seed path: none
- Confirmed Personas: developer--primary-user

## Problem / Opportunity

`givn init` treats an empty agent-target selection as "install OpenCode": an
interactive confirmation with nothing marked, an empty or unknown-only answer
on piped stdin, and EOF without the `--agents` flag all resolve to OpenCode.
The developer sees a successful initialization whose only workflow guidance
sits in `.opencode/` — worthless for a project driven from Claude Code, and,
for the scripted case, never asked for at all.

The failure is silent in both directions: `givn init` reports success, and the
harness actually in use has no skills to run the Givn workflow from. A
project initialized without any agent target is not a usable Givn project —
the workflow lives in generated harness guidance — yet the command claims one.

## Proposed Solution

- `givn init` requires at least one agent target. When the interactive
  confirmation, the piped answer, or the `--agents` value yields no known
  target, the command exits non-zero, names the valid targets
  (`.opencode`, `.agents`, `.claude`), names `--agents` as the way to state
  the selection, and leaves the project unchanged.
- The interactive prompt starts with no target marked; confirming nothing is
  not a selection and is refused like any other empty answer.
- `--agents` remains the non-interactive way to state the selection
  (for example `givn init --agents opencode`); `all` still expands to every
  target. A mixed answer still drops unknown names with a hint and keeps the
  known ones.
- Re-running `givn init` on an initialized project keeps the persisted
  selection, and `givn skills sync` keeps its OpenCode fallback for a config
  without a skills block — existing projects stay upgradeable.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| initialize-a-project | no signal — the top candidates tie at 16.00 | `EXTEND initialize-a-project` | The change alters initialization behaviour inside the existing capability; no new capability is introduced. |
| enable-optional-addons | EXTEND initialize-a-project (16.50) | `EXTEND enable-optional-addons` | Route matches on the shared `givn init` invocation; the delta modifies two scenarios owned by this capability's feature, so it is the merge target. |
| declare-the-product-start | EXTEND initialize-a-project (15.40) | `EXTEND declare-the-product-start` | Same route heuristic; the delta modifies this capability's scaffolding scenario, which stays its own contract. |
| measure-coverage-per-project | EXTEND initialize-a-project (15.60) | `NEW in fragments` | The capability lives under the support fragment; the routing check resolves only use-case-owned capabilities, so fragment deltas record `NEW in fragments` (same rule as `archive-change` and `author-change`). |
| rules-policy | EXTEND initialize-a-project (16.00) | `NEW in fragments` | Same fragment rule; the delta modifies the existing fragment scenario, and its permanent feature already exists. |
| readme-enforcement | EXTEND readme-enforcement (15.00) | `EXTEND readme-enforcement` | Route and decision agree; the delta modifies this capability's scaffolding scenario. |

## Out of Scope

- `givn skills targets add` unknown-name handling.
- `givn skills sync`'s fallback for a config without a skills block (legacy
  projects and repositories upgraded from older Givn versions keep working).
- Adding or removing agent targets.
- Re-running `givn init` on an already-initialized project, which stays
  idempotent and does not re-ask for a selection.

## Open Questions

### D1 — Empty interactive selection: refuse or keep asking?

- Question: when the interactive confirmation arrives with no target marked,
  does initialization fail, or does the prompt stay open until a target is
  marked?
- Options:
  1. **Refuse** — the command exits non-zero with the same error as the
     non-interactive path; gain: one rule for every input path, and the
     developer can script against it; cost: the developer re-runs `givn init`
     after the refusal; in practice: the confirmation returns empty, `givn`
     prints the error and exits 1, nothing is written.
  2. **Keep asking** — the prompt is shown again until something is marked;
     gain: no re-run; cost: interactive and non-interactive behaviour diverge,
     and an empty piped answer still has no loop to repeat; in practice: the
     checkbox list re-appears with a "select at least one target" hint.
- Disposition: answered — the operator report (developer, 2026-10-03) states
  initializing without a harness makes no sense; refusing treats every empty
  selection identically and protects the minimal guarantee (nothing written).

### D2 — Does the prompt keep OpenCode pre-marked?

- Question: does the interactive prompt start with `.opencode` marked, so a
  bare confirmation selects it?
- Options:
  1. **Keep `.opencode` pre-marked** — gain: a fast default path; cost: a
     developer who never made a selection still gets OpenCode, the exact
     outcome the report rejects; in practice: pressing Enter installs
     `.opencode` skills without a decision.
  2. **Start unmarked** — gain: the installed targets are always the ones the
     developer marked; cost: one deliberate selection step; in practice: the
     list opens with nothing marked and a bare confirmation is refused.
- Disposition: answered — the report treats an unselected prompt as no
  selection; only a target the developer marks is a selection.
