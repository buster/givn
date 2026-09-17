# 03 — term-introduction-discipline

Status: ready for `/givn-propose`
Source: ambiguous term proliferation (T2 options 2 and 4)
Depends on: 01 (dialog policy)

## Problem

Agents coin new domain terms freely. The terms are often abstract and not
self-explanatory, so questions that use them cannot be answered. There is no
term-introduction gate, no collision check, no positive naming guidance, and
all vocabulary enforcement is prose asking the model to judge its own
compliance. The one deterministic hook checks only that the glossary file is
not byte-identical to its template.

## Evidence

kiekste side:

- Terms coined and rejected only after user challenge: Substrate, Provider,
  Seam, Ingress, Direct write, External content update
  (`interchangeable-building-blocks/domain-terms.md` lines 167–193). "Seam"
  collided with the codebase's test/crash-injection meaning.
- `Backend Port` → `Candidate Port` rename was proposed in a change seed and
  never applied; the glossary row is unchanged and no signal exists anywhere.
- Collisions discovered mid-ideation, not at introduction: Operator authority
  split (`run-authority/domain-terms.md` lines 9–35, needed ~15 code-comment
  fixes), Channel (question/answer channel vs human-surface channels,
  `eventstorming.md` line 137), Capability (three scopes,
  `domain-terms.md` lines 92–101), Delivery (mirror delivery vs Delivery
  tiers).
- Terms referenced everywhere but never defined: End User, External Partner,
  Requester, Observer, Preview Execution Environment.
- Topic state deliberately retains draft vocabulary as "historical record"
  while the whole 67-event chain was renamed at handoff.

givn side:

- `assets/skills/givn-domain-modeling/SKILL.md.tmpl` lines 34–51 and 78–95:
  "challenge generic terms", anti-term list, "keep it short"; no
  self-explanatory naming test, no collision procedure.
- `assets/skills/givn-ideation/SKILL.md.tmpl` line 95: `domain-terms.md` named
  with no schema; line 165–166: "Term collisions … go to the domain terms
  file" with no procedure; lines 192–202: quality review mentions term
  collisions with no verification steps.
- `src/integrity_hook.rs::check_arc42_conformance`: verifies only that a
  chapter file is not byte-identical to the template.

## Decided scope

1. Fixed `domain-terms.md` entry schema with a mandatory collision check.
   (T2 option 2)
2. A term introduction test for `confirmed` terms: genus + differentia,
   contrast pair, one concrete domain sentence. (T2 option 4)

## Required changes

### 1. `domain-terms.md` entry schema

Define in the ideation skill (fenced block, next to the topic-state list).
Every term entry has:

```text
Term | Status (candidate | confirmed | rejected) | Definition (one sentence) |
Contrast (X, not Y) | First use (file:line) | Collision check (commands run, result)
```

- `candidate`: definition and first use are sufficient.
- `confirmed`: requires the full term introduction test (2) and a recorded
  collision check.
- `rejected`: keeps the definition and the rejection reason; never deleted
  (the kiekste Substrate/Seam pattern shows the value of the record).

### 2. Search-before-coin procedure

Replace the one-line collision instruction with a three-step procedure that
must be executed and recorded before a term reaches `confirmed`:

1. Grep the durable glossary (`docs/arc42/12-glossary.md` when the arc42
   addon is enabled) for the term and its synonyms.
2. Grep every sibling topic's `domain-terms.md` and the repository for the
   term.
3. Record the commands run and the hit counts in the `Collision check`
   column. "No collision" without recorded commands is not a record.

### 3. Term introduction test

Add to `givn-domain-modeling/SKILL.md.tmpl` (new section) and reference it
from the ideation skill. A term may become `confirmed` only when all three
exist:

1. Genus + differentia definition: "a <Term> is a <kind> that <distinguishes
   it>". Example: "a Port is the interface contract at which one
   implementation can be replaced".
2. Contrast pair: "Term, not <near neighbour>" with the distinction.
   Example: "Port, not Boundary — a boundary is a limit that must not be
   crossed".
3. One concrete domain sentence using the term.

If the genus slot cannot be filled, the term is not ready. The kiekste
Substrate case fails immediately: "a Substrate is a kind of ___" has no
referent, which is why the three underlying concepts had to be named
separately.

### 4. Integrate into ideation flow

- Event Storming: the term-collision bullet points to the procedure and
  schema.
- Quality reviews, Event Storming branch: the reviewer verifies that every
  collision entry has a recorded check, instead of re-reading for tone.
- Handoff term merges: only `confirmed` terms are proposed for the durable
  term list; `rejected` terms are reported with their reason.

### 5. Human-facing pointer

Add a short paragraph to `docs/workflow.md` §3 (Optional upstream: ideation)
naming the term record and pointing at the skill as the canonical schema.
Do not duplicate the schema in two places.

## Files to change

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Term schema; collision procedure; review branch; handoff |
| `assets/skills/givn-domain-modeling/SKILL.md.tmpl` | Term introduction test; reference the schema |
| `docs/workflow.md` | Short pointer in §3 |

## Executable-spec impact

- `givn/specs/fragments/ideation-skill.feature`: new scenario asserting the
  generated guidance contains the term status values and the collision-check
  requirement.
- `givn/specs/fragments/domain-skill-trigger.feature`: inspect its assertions;
  extend if it asserts domain-modeling guidance phrases.
- `givn/specs/fragments/skill-rewrites.feature`: update if its phrase
  assertions cover the edited skill sections.

## Acceptance criteria

- [ ] The generated ideation guidance contains the term schema and the
      three-step collision procedure.
- [ ] The generated domain-modeling guidance contains the term introduction
      test.
- [ ] A Substrate-shaped coinage fails the test; the failure mode is
      demonstrated in the guidance.
- [ ] The Seam case (a term colliding with existing code meaning) is caught
      by the recorded grep step, not by user challenge.
- [ ] No deterministic term checker is introduced.

## Out of scope

- T2 option 1 (machine-checkable `TERMS:` declarations and parser).
- T2 option 3 (glossary scaffold upgrade and anti-term lint). The scaffold /
  instruction column mismatch remains as accepted debt.
- Migrating kiekste's existing term records.

## Risks and notes

- Pure authoring guidance: a lazy agent can write "no collision" without
  grepping. The pasted-command requirement is the mitigation, consistent with
  givn's existing paste-the-command culture in `review.md`.
- Require the full test only for `confirmed`; keep `candidate` cheap so early
  ideation is not slowed.
- Keep the anti-term default list; fold it into the new section as worked
  contrast examples rather than a separate rule set.
