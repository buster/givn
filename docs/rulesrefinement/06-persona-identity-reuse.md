# 06 — persona-identity-reuse

Status: ready for `/givn-propose`
Source: persona fragmentation (T5 options 1, 2, 4)
Depends on: 04 (use-case model — persona references in use-case documents)

## Problem

The creation default is per-topic "create one or more personas" with no
inventory-existing-first step, no identity criteria, and a merge procedure
that is one sentence. The schema mixes stable fields (role, vocabulary,
evaluation capability) with topic-flavored fields (interests, goals, pain
points), so every topic stance looks like a new segment. Review mechanics
reward minting: a user-visible surface requires a confirmed persona, and the
only review lens mechanism is a full persona file. Nothing validates persona
references, so dangling IDs and silent redefinition have zero cost.

## Evidence

kiekste side:

- `givn/personas/adapter-implementer--channel.md`, `--repository.md`,
  `--runtime.md`: one human role sliced by which port is implemented. File
  bodies are largely port requirements (R-033/R-035/R-052, TD-011, Q-codes),
  not stable identity. This is the "cook who uses mobile vs cook who uses
  laptop" shape.
- `givn/ideation/fossil-scm-integration/personas/end-user--change-supervisor.md`:
  a full biography rewrite with a topic stance. Handoff merged running-record
  lines only; the topic biography deltas were silently dropped. Three
  divergent copies of one persona now exist.
- Interchangeable-building-blocks copied five reused personas in full and
  minted five new ones, hitting the cap exactly; topic copies stay frozen
  "as historical record" with no canonical pointer.
- `changes/evolve-a-port-contract.md` lists "Engine maintainer — confirmed
  stakeholder; no persona file yet" — a missing file treated as a gap.
- Agents invented three header dialects ("Reused segment: …", "Promoted
  segment …", "New segment. No project-level persona exists…") with no
  template.
- `givn/ideation/run-authority/frame.md` had to invent the identity rule
  locally ("persona segments name roles and their authority boundaries, not
  distinct individuals").
- All permanent `usecase.md` files record `Personas: - none`; zero
  `PERSONAS:` lines exist in active changes. Personas are write-heavy,
  read-light.

givn side:

- `assets/skills/givn-ideation/SKILL.md.tmpl` lines 57–76: creation guidance
  and the one-sentence promotion rule; line 92: topic-state file convention.
- `assets/commands/givn-ideation.md`: confirmation guardrails only.
- No persona file template exists in `assets/templates/`.
- `src/integrity_hook.rs::check_persona_disposition` (line 244): checks the
  `PERSONAS:` line exists and is not `none`; does not validate IDs.
- `docs/usecase-migration-release-plan.md` lines 543–550 list "Validate
  referenced persona IDs" — never implemented.
- `givn/specs/fragments/ideation-skill.feature` lines 18–24 assert the
  current phrases ("stakeholder type", "persona segment", "persona
  promotion", "every artifact that affects").

## Decided scope

1. Identity criteria and a stable-core/topic-stance schema. (T5 option 1)
2. Reuse-first creation protocol with an explicit merge procedure. (T5
   option 2)
3. Persona reference validation at lint and review time. (T5 option 4)

## Required changes

### 1. Identity criteria

A new persona exists only when it differs from an existing one in at least
one of:

- **authority boundary** — what it may do or decide;
- **evaluation capability** — what it can judge;
- **own vocabulary** — terms it uses or refuses.

The criteria are disjunctive, not conjunctive. Topic-specific demands go into
a dated `## Topic stances` appendix, never into Role, Interests, or Goals.
The three adapter-implementer files collapse to one persona (same authority:
ships an adapter against a port; same evaluation: conformance suite) with
three stance blocks.

### 2. Persona schema and template

Add `assets/templates/persona.md` with two parts:

- **Stable core**: role, authority boundary, evaluation capability, own
  vocabulary, success criterion.
- **Running record**: dated entries per review or topic stance, append-only.

Rewrite the Personas section of `givn-ideation/SKILL.md.tmpl` to use this
schema while preserving: user confirmation before use, the review duty, the
cap (at most three segments per stakeholder type, about ten total), advisory
findings with user arbitration, and handoff promotion with no auto-promotion.

### 3. Reuse-first protocol

Replace "for every stakeholder type, create one or more personas" with a
four-step protocol in the ideation skill (Personas, Topic state, and Handoff
sections) and in `assets/commands/givn-ideation.md` guardrails:

1. Inventory `givn/personas/` and every sibling topic's `personas/` into a
   session-state table against the topic's stakeholder types.
2. Match by ID, or record a one-sentence new-segment justification naming
   the distinguishing criterion from (1).
3. A reused persona's topic-local file contains a header line plus new
   running-record entries only — never a biography copy.
4. Handoff merge appends record entries to the project persona; the user
   confirms the merged diff; the topic file stays as historical record.

Standardize one header line:

```text
Reuse: <persona-id>, base <short-hash or date>; this file adds record entries only
New: <persona-id> — <distinguishing criterion>
```

Update the promotion sentence in the skill and the glossary definition so
merge means append-record, not replace-biography.

### 4. Reference validation

- `givn lint`: advisory warning when a `## Personas` entry in a change
  `usecase.md` or an ideation use case matches no file in `givn/personas/`
  or the topic's `personas/`. Forward references during an active ideation
  topic must not break; lint-time is warning-only.
- `givn check review`: the `PERSONAS:` line value must match a file in
  `givn/personas/`; the `none — <reason + operator confirmation>` path stays
  valid and untouched.
- Implement in `src/cli/lint.rs` and
  `src/integrity_hook.rs::check_persona_disposition`; add tests alongside the
  existing visual-review-gate tests.

### 5. givn self-application

Update givn's own glossary rows for Persona segment and Promotion
(`docs/arc42/12-glossary.md`) to the identity criteria and append-merge
semantics. This is part of the change, not a follow-up.

## Files to change

| Path | Change |
|---|---|
| `assets/templates/persona.md` | New template (stable core + stances) |
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Identity criteria; reuse protocol; schema; header line |
| `assets/commands/givn-ideation.md` | Guardrails: inventory, match-or-justify |
| `src/cli/lint.rs` | Advisory dangling-persona-ID warning |
| `src/integrity_hook.rs` | Validate `PERSONAS:` value against `givn/personas/` |
| `tests/steps/visual_review_gate_steps.rs` | Tests for the new validation |
| `docs/arc42/12-glossary.md` | Persona segment and Promotion rows |

## Executable-spec impact

- `givn/specs/fragments/ideation-skill.feature`: keep the asserted phrases
  "stakeholder type", "persona segment", "persona promotion", "every
  artifact that affects"; add assertions for "authority boundary", "topic
  stance", and "reuse".
- `givn/specs/verify-change/review-disposition-gates.feature`: scenario for
  a `PERSONAS:` line naming an unknown ID (fails) and `none — reason` (stays
  valid).
- `givn/specs/verify-change/lint-command.feature`: scenario for the advisory
  dangling-ID warning.
- Check `givn/specs/fragments/visual-review-gate` coverage for the
  disposition check.

## Acceptance criteria

- [ ] The three adapter-implementer-shaped files are refused by the identity
      criteria; the criteria name the distinguishing field explicitly.
- [ ] A reused persona's topic file is record-only; the biography lives in
      one place.
- [ ] Handoff merge is append-only and user-confirmed.
- [ ] A dangling `## Personas` ID produces a lint warning and fails
      `givn check review` at review time.
- [ ] `none — reason + operator confirmation` remains valid.
- [ ] The preserved behaviors are unchanged: user confirmation, review duty,
      cap, advisory findings, actor/persona distinction.

## Out of scope

- T5 option 3 (lightweight review lens). The mint-or-nothing pressure is
  addressed by identity criteria and reuse-first; do not add a second
  persona-like artifact.
- Auto-promotion or automatic merging.
- Rewriting kiekste's existing persona files.

## Risks and notes

- Authority and evaluation capability may be thin for non-workflow domains
  (for example a cooking project). Keep the criteria disjunctive and the
  stance appendix open-ended.
- Record-only topic files are less self-contained; this is accepted because
  IDs are stable and the project file is the single biography home.
- Parallel topics appending to one project persona can collide; keep
  entries topic-tagged and append-only, and surface conflicts in the
  user-confirmed merge diff.
- The validation is deliberately asymmetric: warning at lint time,
  enforcement at review time, so active ideation forward references do not
  break.
