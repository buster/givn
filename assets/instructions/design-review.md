# Instruction: design-review

Stress-test the plan before the task list is written.

This review runs after design is complete — the full planning layer exists
(proposal, spec, design) but tasks have not been written and implementation has
not started. Its purpose is to catch scope, architecture, and testability
problems now, when changing a decision costs a markdown edit, not a rewrite.

## Mandatory, non-skippable, fresh context

Never skip this step. Never run it as a continuation of the session that wrote
the design, and never spawn it from the designing context — a model grilling
its own just-written plan confirms its own assumptions instead of finding
blind spots. Spawn both subagents via the Task tool into a genuinely fresh
context. If the harness lets you choose a model or reasoning-effort tier for
the grilling subagent, choose the most capable one available and set
reasoning/thinking effort to high — a fast tier that rubber-stamps is worse
than no review, because it creates false confidence.

**Two phases, both required:**

### 1. Grilling — relentless interrogation of the plan

A grilling subagent reads the full plan and produces a ranked list of questions
with recommended answers. The orchestrator then presents the questions to the
user one at a time, waits for each answer, and records the outcome.

Required branches — cover every one (or explicitly record "no issue found"):

- **Scope** — does the spec contain exactly what the proposal asked for? Nothing
  extra, nothing missing? Is "done" unambiguous?
- **Tech choices** — is the stack in design.md the right fit for the scenarios?
  Are there simpler alternatives that still satisfy all observable behaviours?
- **Missing scenarios** — are there observable behaviours implied by the proposal
  but absent from the spec? Error paths? Boundary conditions?
- **Testability** — can every scenario actually fail in RED with no production
  code? Do the Then-steps assert concrete observable values, not just "it ran"?
  Does the design name a concrete strict-mode mechanism, a not-implemented stub
  pattern, and a single-scenario run command? Missing or vague on any of the
  three is a finding — it is the root cause of fabricated changes.
- **E2E fidelity** — read the spec instruction served by
  `givn instructions specs --change <id>` and verify its normalized action
  scope, interface type, real driver, and primary-assertion rules are applied
  without redefinition. A silent redefinition, an HTTP/fetch plan for a
  browser UI, or duplicate coverage of one action is a finding.
- **Interaction Coverage** — the interaction inventory lives in the owning
  use-case document (`usecase.md` §Interactions), never in a `.feature` comment
  block. Every inventory entry must appear as a row in the design's
  Interaction Coverage Matrix; an entry without a matching row is a finding.
- **Risk** — what is the single most likely way this plan fails during
  implementation? What is the mitigation?
- **Use-case and Persona context** — confirm use-case identity, contract,
  capability ownership, typed relationships, confirmed Persona coverage, and
  explicit dispositions. Run the use-case scope rejection criteria from the
  review instruction (`givn instructions review`) against the use-case
  contract. Reject invented or silently promoted Personas and any use of
  Persona biographies as Gherkin actors.
- **Decision ledger** — verify that every proposal question ID (`D<n>`)
  resolves in the design's `## Decisions` ledger or is explicitly carried
  with a named owner, and that a seed-sourced question keeps its seed
  reference (`D2 (seed Q5)`). An unresolved, unrecorded item is a finding.
- **Visual design contract** — for a change that introduces or visibly changes
  a screen, `design.md` must contain the `## Visual Design Contract` section
  with its `Interface:` line and viewports. When no approved design exists,
  the agent must propose two or three rendered options (style tiles or an
  equivalent rendered reference) and the user approves one; the approved
  direction is recorded in `design.md`. Without an approved contract,
  design-review does not sign off.
- **Deferrals** — a mandatory check may be deferred only after an operator question; record the operator's decision, otherwise do not sign off.
- **Persona disposition** — record, per persona, the relevance and evidence; when no persona is confirmed, record the reason and the operator confirmation. A user-visible surface requires at least one confirmed Persona.
- **Interface classification** — test the opposite classification before accepting a non-browser interface: a screen rendered in a browser is a Web UI, so a design that declares no browser driver must say why the opposite does not hold.
- **ADR qualification** — delegate the complete qualification procedure to the
  canonical Arc42 addon instruction by running `givn instructions arc42-docs
  --change <id>`. Check only candidates whose structured verdict has all
  mandatory gates `PASS`; do not turn every non-trivial design choice into an
  ADR. The verdict must include a completed existing-ADR or duplicate check. A
  non-qualifying choice must name exactly one canonical feature, design,
  Arc42, process, project-documentation, or code destination. A refinement must
  amend an existing ADR, and a replacement must supersede it. For every
  proposed `NEW_ADR`, require the explicit cheaper home: name the specific
  canonical artifact (feature specification, `design.md`, Arc42 chapter,
  process documentation, project documentation, or code) that would own the
  rationale instead. A missing cheaper home for a proposed `NEW_ADR` is a
  finding. `Must be shared`
  is supporting evidence only. This policy is authoring guidance, not runtime
  enforcement, and this artifact must not duplicate the complete procedure or
  verdict schema.
{{#if addons.arc42}}
- **Architecture documentation (arc42)** — do not just
  audit what `arc42.md` claims — independently re-derive the expected
  chapter set first, then diff against the claim:
  1. Read proposal.md/specs/design.md yourself and walk all 12 rows of the
     arc42-docs instruction's chapter-selection table, forming your own
     Yes/No per row, before opening `arc42.md`.
  2. Open `arc42.md` and compare its per-row Yes/No against your own. Any
row where you say "Yes" and `arc42.md` says "No" (or omits the row) is
      a finding — an omission, not just a contradiction, is what
     slips through unnoticed and must block sign-off.
  3. For every chapter marked "Yes" (by either party): open the chapter file
     and confirm the content actually matches design.md, not placeholder
     text or a stale contradiction.
  4. **Chapter file integrity check (all 12 files):** For every one of the 12
     chapter files under `docs/arc42/`:
     - Confirm the file exists (not absent after a scaffold).
     - Confirm the file contains more than just the scaffolded placeholder
       template — at least some actual content or decision-specific text.
     - **Scan the file for ASCII art diagrams:** search for Unicode
       box-drawing characters (`┌┐└┘│─├┤┬┴┼╔╗╚╝║═╠╣╦╩╬╭╮╰╯`) and for
       multi-line blocks where lines begin with `+` or `|` and contain
       `---` or `===` (non-table pseudo-diagrams). If any are found, the
content violates the diagram policy — treat this as a
        finding and require the diagram to be converted to Mermaid.
   5. Confirm every qualified ADR candidate in design.md has a MADR entry in
      chapter 09, and that any consequence of that decision (e.g. "Bad,
      because...") is reflected in chapter 11. Absence of a MADR for a
      non-qualifying design choice is correct when its canonical destination is
      recorded. Do not copy the canonical qualification procedure or verdict
      schema into this review instruction.
  `arc42.md`'s `STATUS: DONE` is a self-report from the same agent that
  wrote the docs — not verification; treat any mismatch, including a
  missing row, as a finding.
{{/if}}
{{#if addons.domain_modeling}}
{{#if addons.arc42}}
- **Ubiquitous language** — confirm that the ubiquitous language is used consistently and recorded in the project's arc42 glossary before sign-off is mandatory: `docs/arc42/12-glossary.md` must already contain the design's agreed terms. If the design introduces a term the glossary does not hold, or reuses an existing term inconsistently, raise it as a question — do not sign off.
{{/if}}
{{/if}}

If a question can be answered by exploring the existing codebase, explore it
and record the finding — do not burden the user with it.

### 2. Hardening — fix what grilling found

After all questions are answered, a hardening subagent applies decisions:

- Update `design.md` if any tech decisions changed.
- Add missing scenarios to `specs/**/*.feature` with `@givn.added @wip` if any
  were identified; run `givn lint --change <id>` to confirm exit 0 or 2.
- **Never remove `@e2e` tags** from any scenario. If the e2e infrastructure is
  not buildable yet, record a tracked open question in `design-review.md` with
  a concrete date or condition for resolving it — never a tag removal. An
  `@e2e` scenario with an unconfigured runner is an unenforced gap; tag
  removal only hides it.
- Do not write `tasks.md` — that is the next artifact.
- Do not change the proposal unless grilling revealed a fundamental scope
  mismatch and the user decided to fix it — flag that as a blocker instead.

**Rules:**
- Do not rubber-stamp. If the plan looks clean on first read, look harder.
- Grilling is complete only when every branch has been walked and every question
  answered or flagged as a blocker.
- Sign off by writing `DESIGN-REVIEW: PASS` at the end of `design-review.md`
  only when all questions are resolved and all hardening edits are applied.
  Write `DESIGN-REVIEW: FAIL` if any question remains unresolved.
