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
- Enforcement split: the guidance states the per-scenario intent; the gate
  proves per declared state and viewport that evidence exists at all, because
  the binary cannot know which scenario traverses which state. The review
  instruction names the state screenshots the review saw. This boundary is
  also recorded in design.md and chapter 11 (D-19).

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

## A10 — Positive/negative gate scenario pairing

- Assumption: the positive gate scenarios (complete state evidence, confirmed
  persona, operator decision) cannot fail RED on their own; each is paired
  with its negative counterpart in one task, where the negative drives the
  production change and the positive proves the complete artifact passes.
- Options: (a) pair them in one task; (b) mark them fixture-coupled and
  forbid a commit until the paired production code lands; (c) make positives
  assert a new observable only the new code produces.
- Chosen: (a), because the gate outcome is binary and the negative scenario
  is the honest RED driver.
- Impact: `tasks.md` structure, review evidence.

## A11 — Deferral and persona marker parsing

- Assumption: both marker checks ignore fenced code blocks, so the review
  template may scaffold examples inside a fence; a compliant change omits
  `DEFERRAL:`/`OPERATOR-DECISION:` entirely when nothing is deferred, and
  `PERSONAS:` must be a real line outside a fence.
- Options: (a) strip fences; (b) treat the raw text and require no scaffold;
  (c) add a `DEFERRAL: none` sentinel.
- Chosen: (a). It keeps the template instructive without making the scaffold
  itself pass or trigger a check.
- Impact: `check_deferrals`, `check_persona_disposition`,
  `assets/templates/review.md`, review instructions.

## A12 — Persona rule vs this change's own `none`

- Assumption: the change itself records `Confirmed Personas: none` because
  givn's owning use cases declare no personas and inventing one is forbidden;
  the operator's instruction to proceed is recorded here and the disposition
  must be revisited after the change.
- Options: (a) record `none` with the reason and flag it; (b) invent a
  persona; (c) stop and ask the operator now.
- Chosen: (a). Option (b) violates the no-invention rule; option (c)
  contradicts the operator's explicit instruction to assume and record.
- Impact: `proposal.md`, `design.md`, `review.md`, this file.

## A13 — Classification wording in the matrix

- Assumption: the matrix guidance links classification and driver with the
  phrases "is a Web UI row" and "needs a browser driver"; the existing
  "NOT a valid driving mechanism" sentence stays.
- Options: keep the existing matrix rule unchanged (no new RED); add the
  classification link (chosen); restructure the matrix section.
- Chosen: add the link, so the scenario has a fail-first assertion and the
  rule is stated in one place.
- Impact: `assets/instructions/design.md`, `assets/skills/givn-design/SKILL.md.tmpl`.

## A14 — Empty `- States:` line

- Assumption: an empty `- States:` value yields no declared states, so no
  state evidence is required; the unit tests cover the parser behavior.
- Options: (a) treat empty as an error; (b) treat empty as "no states".
- Chosen: (b). A contract that names no state has no state evidence to
  require; naming a state with an empty file still fails.
- Impact: `parse_visual_contract`, unit tests.

## A15 — Instruction line breaks are load-bearing

- Assumption: `givn instructions` embeds the asset text verbatim, so a phrase
  asserted by a scenario must sit on one line in the asset. New guidance
  sentences are written unwrapped (one sentence per line) where a scenario
  asserts a phrase that would otherwise wrap.
- Options: (a) unwrapped sentences in the asset; (b) assert shorter
  fragments; (c) change the assertions after the design review.
- Chosen: (a). It keeps the spec assertions intact and the guidance readable
  as single sentences.
- Impact: `assets/instructions/*.md`, the guidance scenarios.

## A16 — Embedded assets need a package rebuild

- Assumption: `rust_embed` does not track changes under `assets/` for
  incremental rebuilds, so every targeted run after an asset edit is preceded
  by `cargo clean -p givn`.
- Options: (a) clean the givn package before each asset-driven run; (b) add a
  build script that tracks `assets/`; (c) touch `src/config/embedded.rs`.
- Chosen: (a) during this change; (b) is a separate improvement.
- Impact: the RED/GREEN run commands in `tasks.md`.
