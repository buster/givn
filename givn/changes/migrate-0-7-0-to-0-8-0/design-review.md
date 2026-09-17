# Design Review: migrate-0-7-0-to-0-8-0

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Bundle content fidelity | Is `docs/use-case-model.md` reachable in a consumer project? | agreed — rules inlined; live sources named |
| 2 | Bundle content fidelity | Where does a failed ADR's historical-scope note live? | agreed — migration ledger, never the ADR archive |
| 3 | Risk | Is the enforcement boundary stated? | agreed — "what the gates catch" paragraph |
| 4 | Missing scenarios | Is the new catalog route machine-checked? | agreed — one unit test |
| 5 | Scope | Is the unreleased multi-SCM work part of 0.8.0? | agreed — folded into the 0.8.0 section |
| 6 | Bundle content fidelity | How does the consumer obtain the blank ADR template? | agreed — delete, re-enable arc42, verify |
| 7 | Bundle content fidelity | Are ejected facets re-synchronized? | agreed — overrides phase added |
| 8 | Bundle content fidelity | Are seed statuses and term records fully specified? | agreed — five statuses, collision evidence |
| 9 | Testability / verification | Is the smoke-test recipe pinned? | agreed — exact commands and assertions |
| 10 | Tech choices | Manual bump or release workflow? | agreed — manual; tag is post-merge |
| 11 | Architecture documentation | Are the arc42 row reasons accurate? | agreed — rows 9 and 11 corrected |

### Q1 — Use-case model reachability

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the use-case phase is performable.
- Context: `docs/use-case-model.md` exists only in the Givn repository; no
  command writes it into a project.
- Question: what does the bundle point at?
- Options:
  1. **Inline the rules and name the live sources.**
     Meaning: the eight rules and R1-R10 are in the bundle; the generated
     ideation guidance and `givn instructions review` are named.
     In practice: the phase works without the repository file.
     Gain: self-contained.
     Cost: longer bundle.
  2. **Keep the repository reference.**
     Meaning: the instruction resolves to a missing file.
     In practice: a dead reference.
     Gain: shorter bundle.
     Cost: an unperformable phase.
- Resolution: **Option 1**. Rejected: Option 2.

### Q2 — Failed ADR historical-scope note

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: where the note lives.
- Context: the shipped guidance says a non-qualifying record must not be
  copied into the ADR archive.
- Question: archive or ledger?
- Options:
  1. **Migration ledger.**
     Meaning: rationale moves to one canonical home; the note records ID,
     failing test, and destination in the maintenance change.
     In practice: no archive pollution.
     Gain: consistent with the shipped rule.
     Cost: none.
  2. **ADR archive.**
     Meaning: copy the record into `docs/arc42/adr/archive/`.
     In practice: contradicts the shipped instruction.
     Gain: none.
     Cost: a rule violation.
- Resolution: **Option 1**. Rejected: Option 2.

### Q3 — Enforcement boundary

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether false completion is visible.
- Context: only two migrated rules are machine-checked.
- Question: state it?
- Options:
  1. **Add the paragraph.**
     Meaning: name the `[PERS ]` advisory and the visual `PERSONAS:` gate;
     the rest is authoring/review guidance.
     In practice: a green suite cannot be mistaken for migration proof.
     Gain: honest completion criteria.
     Cost: one paragraph.
  2. **Leave it implicit.**
     Meaning: green suite equals done.
     In practice: mega-leaves survive unnoticed.
     Gain: none.
     Cost: the likely false-completion path.
- Resolution: **Option 1**. Rejected: Option 2.

### Q4 — Route assertion

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the new edge is machine-checked.
- Context: asset existence is checked by the catalog loader; the route is
  not.
- Question: add a unit test?
- Options:
  1. **Assert the 0.7.0 route in `migration.rs`.**
     Meaning: `migration_path("0.7.0", env!("CARGO_PKG_VERSION"))` returns the
     new bundle id.
     In practice: a broken edge fails the lib suite.
     Gain: cheap guarantee.
     Cost: one test.
  2. **Rely on the smoke test.**
     Meaning: manual verification only.
     In practice: regressions slip.
     Gain: none.
     Cost: weaker.
- Resolution: **Option 1**, matching the 0.7.0 release precedent. Rejected:
  Option 2.

### Q5 — Unreleased changelog content

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the release section describes the whole binary.
- Context: the unreleased section holds the multi-SCM work.
- Question: fold it into 0.8.0?
- Options:
  1. **Fold and name it.**
     Meaning: `## [0.8.0] - <date>` includes the multi-SCM work and the
     model-hardening entries.
     In practice: the release notes match the binary.
     Gain: truthful changelog.
     Cost: none.
  2. **Leave it unreleased.**
     Meaning: 0.8.0 omits merged work.
     In practice: misleading notes.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**, matching the 0.7.0 release. Rejected: Option 2.

### Q6 — Blank ADR template delivery

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: how the consumer replaces a stale template copy.
- Context: no command delivers the template; the addon writes it only when
  missing.
- Question: what procedure does the bundle record?
- Options:
  1. **Delete, re-enable, verify.**
     Meaning: delete `docs/arc42/adr/adr-template.md`, run
     `givn addons enable arc42`, confirm the prefill strings are gone.
     In practice: the shipped blank template lands.
     Gain: concrete and idempotent.
     Cost: none.
  2. **Hand-edit the old copy.**
     Meaning: the consumer patches the prefill by hand.
     In practice: error-prone.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

### Q7 — Ejected facet synchronization

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether project overrides keep the old rules.
- Context: `givn upgrade` regenerates skills and commands but never replaces
  ejected facets.
- Question: add an overrides phase?
- Options:
  1. **Add it.**
     Meaning: inventory overrides, re-eject from 0.8.0, re-apply genuine
     edits.
     In practice: no stale rule text survives.
     Gain: complete migration.
     Cost: one phase.
  2. **Skip it.**
     Meaning: stale overrides keep pre-0.8.0 text.
     In practice: silent drift.
     Gain: shorter bundle.
     Cost: incomplete migration.
- Resolution: **Option 1**. Rejected: Option 2.

### Q8 — Seed statuses and term records

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the smaller adoptions are fully specified.
- Context: the phase mentioned headers and schemas but not the five statuses
  or the collision evidence.
- Question: spell them out?
- Options:
  1. **Name the statuses and evidence.**
     Meaning: five seed statuses plus `[SEED ]`; term fields with
     `candidate`/`confirmed`/`rejected` and pasted commands.
     In practice: consumed seeds and unverified terms are caught.
     Gain: complete.
     Cost: a paragraph.
  2. **Keep it general.**
     Meaning: consumers guess.
     In practice: `ready` on consumed seeds.
     Gain: shorter.
     Cost: incorrect migration.
- Resolution: **Option 1**. Rejected: Option 2.

### Q9 — Smoke-test recipe

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the verification is reproducible.
- Context: the design said "a scratch project whose marker is 0.7.0".
- Question: pin the commands and assertions?
- Options:
  1. **Pin the fixture recipe and assertions.**
     Meaning: init --no-addons, rewrite marker, commit, upgrade; assert the
     aggregate files, five phase headings, no `{{`, `migration.yaml` id, the
     second-run message, and `givn --version` 0.8.0.
     In practice: exact evidence in tasks.
     Gain: reproducible.
     Cost: none.
  2. **Describe loosely.**
     Meaning: ad-hoc verification.
     In practice: unrepeatable.
     Gain: none.
     Cost: weak evidence.
- Resolution: **Option 1**. Rejected: Option 2.

### Q10 — Release mechanics

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the prepare-release workflow is in scope.
- Context: the workflow exists but the last tag is `v0.4.0`.
- Question: manual bump or workflow?
- Options:
  1. **Manual bump; tag is post-merge.**
     Meaning: match the 0.6.0/0.7.0 precedent; record the tag as a maintainer
     action.
     In practice: no workflow edit.
     Gain: consistent.
     Cost: none.
  2. **Align the workflow.**
     Meaning: rework the release process in this change.
     In practice: scope creep.
     Gain: none.
     Cost: a process change in a migration release.
- Resolution: **Option 1**. Rejected: Option 2.

### Q11 — arc42 row reasons

- Status: closed 2026-09-17 (operator, bulk recommendation)
- Blocks: whether the assessment is accurate.
- Context: row 9 named the wrong chapter set and amendments; row 11 named
  four risks instead of seven.
- Question: correct them?
- Options:
  1. **Correct the reasons.**
     Meaning: chapters 8.15 and 8.19-8.24 plus the one ADR-028 amendment;
     R-23, R-46, R-67, R-68, R-69, R-70, R-71.
     In practice: the audit matches the register.
     Gain: accurate.
     Cost: none.
  2. **Leave them.**
     Meaning: inaccurate reasons.
     In practice: a design-review finding.
     Gain: none.
     Cost: drift.
- Resolution: **Option 1**. Rejected: Option 2.

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Missing scenarios | No existing scenario breaks with the new catalog entry; the aggregate fixtures scan for one `migrate-` change and never assert the bundle count. |
| Tech choices | Order 60, source 0.7.0, target 0.8.0 are correct; the catalog loader validates asset existence; version carriers are Cargo.toml, Cargo.lock, and the config marker only. |
| E2E fidelity | The skipped-specs maintenance path matches the archived 0.7.0 release change and the maintain-project use case. |
| Bundle content fidelity | The persona, use-case, ADR, and smaller-adoption facts match the shipped 0.8.0 guidance and templates. |
| Bundle content fidelity | `docs/migrations/README.md` is stale (names the 0.3.0-to-0.4.0 bundle); the design's task to update it is valid. |
| Testability | The scratch-project fixture mirrors `release_0_3_0_e2e_steps::legacy_git_project` and is achievable. |
| Architecture documentation | The independent 12-row walk matches arc42.md after the row 9/11 corrections; no ADR qualifies; no placeholder text. |

## Open questions

None.

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Release-owned maintenance change; `specs` is skipped. | accepted |
| Typed Includes / Extends relationships | No relationship changes. | accepted |
| Confirmed Persona coverage | `none` — no user-visible product surface; the change ships assets and documentation. | accepted |
| Actor and Persona distinction | No Gherkin actors introduced. | accepted |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| none | No user-visible product surface; the migration bundle targets a consumer project's own artifacts. | accepted |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | — |
| 2 | 02 architecture-constraints | No | No | Yes | — |
| 3 | 03 context-and-scope | No | No | Yes | — |
| 4 | 04 solution-strategy | No | No | Yes | — |
| 5 | 05 building-block-view | No | No | Yes | — |
| 6 | 06 runtime-view | No | No | Yes | — |
| 7 | 07 deployment-view | No | No | Yes | — |
| 8 | 08 crosscutting-concepts | No | No | Yes | — |
| 9 | 09 architecture-decisions | No | No | Yes | Yes — row reason corrected |
| 10 | 10 quality-requirements | No | No | Yes | — |
| 11 | 11 risks-and-technical-debt | No | No | Yes | Yes — row reason corrected |
| 12 | 12 glossary | No | No | Yes | — |

- [x] All 12 rows independently assessed against proposal.md/design.md
      before reading arc42.md's own claims.
- [x] No row where the reviewer said "Yes" and arc42.md said "No"/omitted it.
- [x] No qualified ADR candidate; the release adds no decision boundary.
- [x] No MADR "Bad, because..." consequence requires a chapter 11
      counterpart (no new MADR).
- [x] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/migrate-0-7-0-to-0-8-0/design.md` | Rules inlined for the bundle; ADR note moved to the migration ledger; enforcement boundary; overrides phase; seed/term specifics; route unit test; smoke-test recipe; changelog folding; tag note. |
| `givn/changes/migrate-0-7-0-to-0-8-0/arc42.md` | Rows 9 and 11 reasons corrected. |
| `givn/changes/migrate-0-7-0-to-0-8-0/proposal.md` | Unreleased multi-SCM work named as part of 0.8.0. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs` is intentionally skipped; no scenario updates were identified.
- [x] `givn lint --change migrate-0-7-0-to-0-8-0` has no parse errors (no delta specs).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
