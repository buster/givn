# Rules Refinement Handover

This directory is a staging area for eight planned givn rule improvements.
It is not a givn artifact and is not governed by the givn flow.

The package originates from a large kiekste development session that used givn
end to end. Six read-only investigations examined givn's rules and skills plus
kiekste's produced artifacts. Each numbered document below is written to be the
input for exactly one `/givn-propose` run in the givn repository. The user
decided the scope of each item; rejected options are recorded per document so
the decision is not re-opened.

## How to use this package

1. Work the changes in the order below. Dependencies are real; later documents
   assume the vocabulary and rules introduced by earlier ones.
2. For each change, run the normal givn flow in the givn repository:
   `/givn-propose` with the handover document as the change input, then the
   standard artifacts (proposal, specs, design, design-review, tasks, review).
3. Treat each document's "Required changes" and "Acceptance criteria" sections
   as the change contract. The "Out of scope" section is binding: it records
   options the user explicitly rejected.
4. givn self-hosts. Every phrase that generated guidance asserts is covered by
   givn's own executable specs under `givn/specs/`. Guidance edits and their
   spec updates belong to the same change; the "Executable-spec impact"
   section lists the affected specs.
5. Update this README's status table when a change is archived.

## Ordered changes

| # | Change | Source findings | Depends on | Status |
|---|---|---|---|---|
| 1 | [dialog-question-quality](01-dialog-question-quality.md) | Terse unanswerable questions (T1 A, T1 B questions, T1 C) | — | ready |
| 2 | [seed-artifact-lifecycle](02-seed-artifact-lifecycle.md) | T1 B seeds + seed lifecycle (M2 A) | 1 | ready |
| 3 | [term-introduction-discipline](03-term-introduction-discipline.md) | Term proliferation (T2 options 2 and 4) | 1 | ready |
| 4 | [use-case-model-scoping](04-use-case-model-scoping.md) | Use-case scoping (T3 option A) | 1 | ready |
| 5 | [adr-qualification-hardening](05-adr-qualification-hardening.md) | ADR overuse (T4 options A and C) | 4 | ready |
| 6 | [persona-identity-reuse](06-persona-identity-reuse.md) | Persona fragmentation (T5 options 1, 2, 4) | 4 | ready |
| 7 | [guarantee-question-traceability](07-guarantee-question-traceability.md) | Guarantees over open prerequisites (M5 options A and B) | 2, 4 | ready |
| 8 | [finding-severity-routing-signal](08-finding-severity-routing-signal.md) | Ignored advisory output (M6 options A and B) | — | ready |

Ordering rationale:

- The dialog policy (1) is foundational: it defines how every later question,
  decision, and term discussion is phrased and recorded.
- The seed artifact (2) and the term record (3) both consume the dialog policy.
- The use-case model (4) is the contract that the ADR gate (5), the persona
  work (6), and the guarantee traceability (7) all reference.
- The finding severity and routing work (8) is tooling and can land last.

## Explicitly rejected options (no action)

Recorded so they are not re-proposed:

- T1 option D (answerability checklist at phase reviews) — covered by 1's
  review-facing references; no separate checklist gate.
- T2 option 1 (machine-checkable `TERMS:` declarations) and option 3
  (anti-term lint) — rejected; term discipline stays authoring guidance.
- T3 options B (parser oversize warnings) and C (router anti-absorption,
  `SPLIT`/`PROMOTE` verbs, archive-time capability moves) — rejected for now.
- T4 options B (decision log) and D (one-ADR-per-change budget) — rejected.
  The falsification tests and the cheaper-home challenge must therefore name
  an existing canonical home, never a new decision log.
- T5 option 3 (lightweight review lens) — rejected; identity criteria and
  reuse-first cover the minting pressure.
- M1 (archive gate enforcement gap), M3 (atomic-commit format check), M4
  (temporal state marking) — classified as bulk-archiving artifacts, not
  systemic problems. No change.
- M2 option B (`givn new --from-seed` flow) — rejected; status headers and
  stamping only.

## Provenance

The investigations covered six topics: question answerability, term
proliferation, use-case scoping, ADR overuse, persona fragmentation, and a
meta-review of systemic findings. Evidence came from the givn repository
(`$HOME/projects/givn`, v0.7.0) and from the kiekste worktree that was
developed with givn (43 changes, 23 ADRs, 3 ideation topics, 29 permanent
feature files). Each handover document cites its specific evidence; no claim
in these documents is based on a run of givn's own test suite.
