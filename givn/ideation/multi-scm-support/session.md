# Ideation session: multi-scm-support

## Phase

Handoff complete 2026-09-14

## Progress

- 2026-09-14: Frame research complete. Read `givn status`, `givn spec tree`,
  `README.md`, Arc42 chapters 02/03/12, the ADR register, the archive-change
  and maintain-project use cases, `givn/config.yaml`, and the archive-proof
  scenario list. Ran a local Fossil 2.26 probe (status, partial commit, add,
  ignore, hooks).
- 2026-09-14: Frame confirmed by the user. Attribution feasibility probe
  done: no transparent Fossil hook; feasible only through a givn-controlled
  commit path. Recorded in `questions.md` Q2.
- 2026-09-14: Persona structure consolidated by user decision: three
  personas (primary user, technical role, givn maintainer); the Fossil
  audience persona is dropped. Persona contents confirmed (lgtm).
- 2026-09-14: Frame boundary quality review run (fresh context). Findings:
  F1 corrected (run-command ownership), F2 recorded as gap G11 and Q14, F5
  wording corrected, F6 recorded as enumerated gap G9 and clarified
  detection rule, F7 resolved (detection rule marked confirmed; residual
  cases parked as Q6/Q14).
- 2026-09-14: Event Storming draft written to `eventstorming.md`.
- 2026-09-14: Event Storming confirmed by the user. Events numbered A1-E7
  for cross-reference.
- 2026-09-14: Use Case Definition written to `use-case-definitions.md`.
  Boundary review run (fresh context); findings applied: event IDs (U1),
  `verify-change` and `operate-product` added to the cut (U2), E2
  misattribution removed from `inspect-change-state` (U3), root semantics
  assigned to the `scm-backend` fragment (U4), one owner per event with a
  Consumes column (U5), open questions Q5/Q7 no longer asserted (U6),
  fragment/capability title fixed (U7), stakeholder tags completed (U8),
  every capability classified (U9).

- 2026-09-14: Use Case Definition confirmed by the user.
- 2026-09-14: `archive-change` elaborated (`use-cases/archive-change.md`) and
  confirmed. Proposed resolutions confirmed: Q3 mirrored strictness, Q5 givn
  publishes the archive revisions, Q13 "unproven and repairable" guarantee.
- 2026-09-14: `scm-backend` fragment elaborated and confirmed. Proposed
  resolutions confirmed: Q1 detection only, Q4 checkout-root boundary, Q6
  root-local detection with ambiguity refusal, Q14 named refusal for a
  missing executable.
- 2026-09-14: `author-change` and `support`/`llm-attribution` confirmed.
  Proposed resolutions confirmed: Q2 `givn commit` path, Q7 init-without-SCM
  warning.
- 2026-09-14: `maintain-project`, `verify-change`, `inspect-change-state`,
  `operate-product` elaborated. All eight use case and fragment files exist.
- 2026-09-14: Elaboration boundary quality review run (fresh context).
  Findings applied: X1 clean-witness wording, X2 pending receipt unbound,
  X3 finalization example corrected, X4 migration-free upgrade has one
  revision, X5 ancestor detection wording, X6 refusal scope, X7 scm-backend
  included in verify-change and operate-product, X8 Fossil witness commands,
  X9 `--change` in examples, X10 real init output, X11 question statuses
  reconciled and proposed deltas marked, X12 capability registration note,
  X13 Q10 working-term note, X14 backend knowledge dropped from the
  integrity rule.

- 2026-09-14: Remaining elaborations and proposals confirmed (Q8, Q11,
  `inspect-change-state` backend line, `operate-product` wording).
- 2026-09-14: Handoff records confirmed by the user. Published the topic
  document at `docs/ideation/multi-scm-support.md`; merged the neutral terms
  into `docs/arc42/12-glossary.md`; promoted the three personas to
  `givn/personas/`; wrote seven change seeds under `changes/`.

## Next steps

No active change was started. Run `/givn-propose <id>` per seed, in the
suggested order: `scm-backend` (prerequisite), `archive-change`,
`maintain-project`, `author-change`, then the optional small seeds
`verify-change`, `inspect-change-state`, `operate-product`.

## Decisions already taken by the user

- givn natively supports both Git and Fossil.
- A Git project uses the Git backend; otherwise Fossil is used.
- The Frame (stakeholders, scope boundary, detection rule) is confirmed.
- Attribution exists on Fossil through `givn commit`; Git keeps the chained
  hook as a fallback.
- Three personas: primary user, technical role, givn maintainer; audience
  dropped.
