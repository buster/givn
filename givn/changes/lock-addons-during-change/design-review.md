# Design Review: lock-addons-during-change

Review kind: self-review
Author: Maki
Reviewer context: Maki

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| Q1 | Scope | Does the lock cover add-on policy only? | Closed — `config-lock.yaml` stores the nine add-on values; rules, targets, and commands are untouched. |
| Q2 | Scope | Are old changes unaffected? | Closed — a missing lock is skipped; a unit test asserts the optional lock. |
| Q3 | Tech choices | Is a new dependency required? | Closed — serde_yaml_ng and the existing SCM/CLI stack suffice. |
| Q4 | Missing scenarios | Are refusal, drift, forced init, override, and no-active-change cases covered? | Closed — eight delta scenarios cover them. |
| Q5 | Testability | Can the TTY override be tested? | Closed — a real `script` PTY runs the command and the lock assertion checks the recorded reason. |
| Q6 | Testability | Can non-TTY and empty-reason overrides fail? | Closed — the non-interactive scenario is covered; the empty-reason guard is in the design and unit-level path. |
| Q7 | E2E fidelity | Does the change add a new E2E action? | Closed — the lock scenarios are regular variants of existing add-on and review actions. |
| Q8 | Risk | What is the main residual bypass? | Closed — deleting the lock is deliberate tampering. D4 records it as a documented limit; no extra marker in 0.9.16. |
| Q9 | Risk | Can `givn init --force` bypass the lock? | Closed — a forced init that changes the add-on set refuses before writes; a preserving init is allowed. |
| Q10 | Risk | Can an agent use `--operator`? | Closed — it requires a TTY and a non-empty reason; the test runs it through a real PTY. |
| Q11 | Use-case and Persona context | Is the owning use case preserved? | Closed — initialize-project owns the lock; verify-change owns the gates. |
| Q12 | Decision ledger | Do D1–D5 resolve? | Closed — all recorded in the proposal and design. |
| Q13 | Contracts and components | Does one component implement each contract? | Closed — `config::lock`, the add-on guard, and the review/archive gate each have one implementing component. |
| Q14 | Visual design contract | What is the interface? | Closed — CLI text; PTY transcript. |
| Q15 | ADR qualification | Does this need an ADR? | Closed — no; the use cases and design own it. |
| Q16 | Architecture documentation | Are affected chapters current? | Closed — 3, 5, 6, 8, 11, 12 updated. |
| Q17 | Ubiquitous language | Are the terms recorded? | Closed — Add-on lock and Operator override. |
| Q18 | Deferrals | Is anything deferred without authority? | Closed — lock deletion hardening is explicitly out of scope. |

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `docs/reference.md` | Documented the lock, refusal, override, and drift checks. |
| `docs/arc42/03`, `05`, `06`, `08`, `11` | Updated command/interface, component, runtime, configuration, and risk sections. |
| `docs/glossary.md` | Added Add-on lock and Operator override. |
| `specs/initialize-project/enable-optional-addons.feature` | Added the PTY override, forced-init, and no-active-change scenarios. |
| `specs/verify-change/givn-workflow-enforcement.feature` | Added the review drift scenario. |
| `tests/common/mod.rs` | Added the generic PTY command helper. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger.
- [x] All questions are resolved.
- [x] `design.md` reflects the decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios.
- [x] `givn lint --change lock-addons-during-change` has no parse errors.
- [x] Architecture documentation (arc42) check completed.

**DESIGN-REVIEW: PASS**
