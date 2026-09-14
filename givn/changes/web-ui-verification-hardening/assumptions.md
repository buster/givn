# Assumptions — web-ui-verification-hardening

Recorded while working the change. Each entry names the assumption, the
options considered, and the chosen option. Revise after the fact by editing
the artifacts named under "Impact".

## A1 — Canonical edit targets

- Assumption: the handover's "Wo ändern" paths (`.opencode/...`) are generated
  copies. The canonical sources are `assets/instructions/*.md`,
  `assets/skills/*/SKILL.md.tmpl`, `assets/templates/*.md`, `assets/commands/*.md`.
- Options: (a) edit generated `.opencode/.claude/.agents` trees directly;
  (b) edit `assets/` and regenerate with `givn skills sync`; (c) edit both.
- Chosen: (b). The repository rule (`author-change`: generated guidance is
  synchronized from canonical source assets) and the just-archived
  `visual-ui-review` change both use `assets/` as the source of truth.
- Impact: all guidance edits; `givn skills sync` regenerates the agent trees.

## A2 — Change identity

- Assumption: change id `web-ui-verification-hardening`.
- Options: `web-ui-verification-hardening`, `sharpen-web-ui-rules`,
  `browser-verification-fail-closed`.
- Chosen: `web-ui-verification-hardening`.
- Impact: change directory, commit scopes.

## A3 — Mechanical enforcement depth

- Assumption: only rules that are decidable from artifacts get a runtime gate
  check; conversation-level acts (operator sign-off, operator question) are
  enforced through review/design-review guidance plus templates.
- Options: (a) guidance only; (b) guidance + gate for state evidence and
  persona/"none" markers; (c) build a full semantic checker for
  classification, deferrals, and sign-offs.
- Chosen: (b) where the artifact is structurally decidable (state evidence
  paths, `none` reason), guidance for the rest.
- Impact: `src/integrity_hook.rs`, `assets/instructions/review.md`,
  `assets/instructions/design-review.md`, templates.

## A4 — Browser driver default (F1)

- Assumption: Playwright is the default driver; the version is resolved per
  change at design time with a recorded source; givn pins no version.
- Options: central pin; no default; Playwright mandatory unless a named
  constraint forbids it.
- Chosen: operator decision, Playwright default + per-change version.
- Impact: `assets/instructions/design.md`, matrix wording.

## A5 — Screenshot scope (F2)

- Assumption: evidence is one screenshot per `@e2e` scenario, per viewport,
  plus one per UI state the scenario traverses. Final-state keeps
  `<viewport>.png`; additional states use `<viewport>--<state>.png`.
- Options: scenario × viewport only; + states; full matrix; desktop only.
- Chosen: operator decision, + states.
- Impact: `assets/instructions/specs.md`, `assets/instructions/design.md`,
  `assets/skills/givn-steps/SKILL.md.tmpl`, gate evidence check.

## A6 — Persona requirement boundary (F4)

- Assumption: a user-visible surface is a browser interface, a terminal
  interface, or CLI text output. Only changes with no user-visible surface may
  declare `none`.
- Options: only browser/terminal; never none; none anywhere with a reason.
- Chosen: operator decision, every user-visible surface.
- Impact: `assets/instructions/proposal.md`, templates, design-review, review.

## A7 — Persona disposition for this change itself

- Assumption: givn's own use cases (`author-change`, `verify-change`) declare
  `Personas: none`, confirmed by the operator through earlier archived
  changes; this change does not invent personas and records `none` with that
  reason.
- Options: (a) carry the existing `none`; (b) stop and ask the operator for
  personas; (c) invent personas.
- Chosen: (a). The operator instruction was to assume and record.
- Impact: `proposal.md`, `design.md` use-case traceability.

## A8 — `States:` contract line

- Assumption: the `## Visual Design Contract` gains an optional `- States:`
  line naming the UI states the screen has (e.g. `empty, loading, error`).
  When present, state evidence is required; when absent, only final-state
  evidence is required (backward compatible).
- Options: (a) optional line; (b) mandatory line for web contracts;
  (c) no line, guidance-only.
- Chosen: (a). Keeps existing contracts valid while giving the gate a
  decidable state list.
- Impact: `parse_visual_contract`, `check_visual_evidence`,
  `assets/instructions/design.md`, template.

## A9 — Commit granularity

- Assumption: one atomic commit per scenario (RED+GREEN+REFACTOR), commit
  hash recorded in `tasks.md`, per the project workflow.
- Chosen: project rule, no alternative.
- Impact: git history.
