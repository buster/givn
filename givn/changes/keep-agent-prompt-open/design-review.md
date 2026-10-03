# Design Review: keep-agent-prompt-open

## Grilling log

Fresh-context grilling subagent (`GrillPromptLoop`, read-only) read proposal,
delta, design, arc42 assessment, permanent contract, the pinned `dialoguer`
source, and the test harness, and empirically probed the planned key
sequences with a throwaway binary against `dialoguer 0.11.0`. All ten branches
were walked; nine findings were raised and every one is resolved below. No
question needed operator input: the operator had already fixed the desired
behaviour, and each remaining point was settled by codebase or dependency
evidence.

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Docs / code plan | Stale doc comment still promises an unmarked prompt and a refusal | Closed — added to the impact list, updated with the loop |
| 2 | Scope | The plan promised Esc as a cancel key; Esc is ignored by `MultiSelect::interact()` | Closed — prose corrected to Ctrl-C/SIGINT only |
| 3 | Testability | `"\x20"` unmarks only the highlighted item; the invariant was unstated | Closed — invariant recorded in design.md and the step |
| 4 | Assertions | PTY transcript is copied to both stdout and stderr, so the channel assertion is non-discriminating | Closed — recorded as a known harness limitation, channel fixed by design |
| 5 | Assertions | The error's naming of the three targets was under-asserted | Closed — scenario asserts the contiguous phrase; absence checks extended |
| 6 | Gate mechanics | `@wip` tags block the tasks/review/archive gates | Closed — explicit task step removes them |
| 7 | Gate mechanics | SHAPE/SUBST advisories need review dispositions | Closed — planned in tasks/review |
| 8 | Code plan | The builder must be rebuilt inside the loop for `defaults` to re-apply | Closed — design states the rebuild and why |
| 9 | Merge | The delta kept the base's own base-sha marker, producing two markers after merge | Closed — stale marker dropped from the delta |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Scope | Delta covers exactly the two proposal behaviours; non-terminal refusal scenarios and fixtures stay byte-identical; the code change is confined to the `stdin.is_terminal()` branch. |
| Merge semantics | The removed title exists verbatim in the permanent spec (`givn/specs/initialize-project/initialize-a-project.feature:202-207`); both added titles are absent from `givn/specs`, so `@givn.added` cannot collide; merge is delete-by-title + append (31 → 32 scenarios). |
| Testability | Empirically proven with a throwaway `dialoguer 0.11.0` binary through `script -qec`: `"\n"` selects item 0; `"\x20\n\n"` unmarks opencode, triggers the error line, re-renders with the default and selects it; `"\x20\n"` blocks, proving the loop. `console` uses TCSADRAIN, so queued bytes survive across the two `interact()` calls. |
| Code plan | `require_agent_target` stays reachable for piped/flag paths and unreachable for the TTY branch; no dead code; the constant is used. |
| Scenario assertions | Every planned step binds to an existing definition; the error phrase is an exact substring of the planned message. |
| Risk | RED is provable both ways: today `"\n"` on an unmarked list exits non-zero (scenario 1), and `"\x20\n\n"` marks opencode on the first space so the error line never appears (scenario 2). Residual risk is PTY flake — mitigated by the proven determinism and the 60-second deadline with a loud panic. |
| Docs consistency | All durable docs carry the new wording; the only contradiction was the function doc comment (finding 1). |
| Arc42 | Independent 12-row walk matches `arc42.md` (Yes: 06, 08, 09, 10; No elsewhere); chapters contain the updated wording; no ASCII-art diagrams. |
| Domain language | Terms match the glossary; no new domain noun; actors stay `user`/`consumer`. |
| Use-case delta | Base marker equals `sha256sum givn/specs/initialize-project/usecase.md` (`fdbebb1a…e528fa`); the document parses structurally; extensions stay step-referenced. |

## Open questions

| # | Question |
|---|---|
| — | none |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | `initialize-project` owns the capability; the change extends `initialize-a-project` (route reported no signal; proposal records the decision). | none needed — evidence-derived |
| Typed Includes / Extends relationships | `Includes: fragment: support`; `Extends: none` — unchanged. | none needed — evidence-derived |
| Confirmed Persona coverage | `developer--primary-user` confirmed; the CLI text surface has a named persona. | none needed — evidence-derived |
| Actor and Persona distinction | Gherkin actors stay `the user`; personas remain review lenses. | none needed — evidence-derived |

### Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| developer--primary-user | Primary — keeps the dialog open and reads the error | Two PTY scenarios assert exit 0, the error phrase, and the installed target. |
| agent--technical-role | Indirect — consumes installed skills | Generated guidance unchanged; pinned by existing e2e scenarios. |
| givn-maintainer--owner | Governance — the interactive retry no longer forces a re-run while scripted misuse still fails fast | Delta extension split + ADR-013 amendment wording. |

## Architecture documentation (arc42) check

Independent walk of the chapter-selection table before reading `arc42.md`.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | Yes | n/a |
| 2 | 02 architecture-constraints | No | No | Yes | n/a |
| 3 | 03 context-and-scope | No | No | Yes | n/a |
| 4 | 04 solution-strategy | No | No | Yes | n/a |
| 5 | 05 building-block-view | No | No | Yes | n/a |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes — §6.1 step 2 records the pre-marked default and the re-ask |
| 7 | 07 deployment-view | No | No | Yes | n/a |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes — §8.7 separates the interactive re-ask from the non-terminal refusal |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes — ADR-013 amendment text and both register rows updated |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes — Q-5 records the re-ask |
| 11 | 11 risks-and-technical-debt | No | No | Yes | n/a — R-72 already carries the scripted refusal, which is unchanged |
| 12 | 12 glossary | No | No | Yes | n/a — no new term |

- [x] All 12 rows assessed independently before reading `arc42.md`.
- [x] No row where the reviewer says "Yes" and `arc42.md` says "No" or omits it.
- [x] The single decision is an `AMEND_ADR`; its consequence (scripted init
      requires `--agents`) already has R-72 in chapter 11.
- [x] The verdict records the structured qualification result and the
      route-specific target; `Must be shared` is supporting evidence only.
- [x] No chapter claimed as updated is still placeholder text.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `proposal.md` | Cancel semantics corrected to Ctrl-C/SIGINT; Esc documented as ignored. |
| `design.md` | Fresh-builder-in-loop requirement, the key-sequence invariant, the transcript/channel limitation, the doc-comment update, and the corrected cancel semantics recorded. |
| `specs/…/initialize-a-project.feature` | Scenario 1 asserts `.agents` absence too; scenario 2 asserts the contiguous error phrase `mark at least one agent target`. |
| `specs/initialize-project/usecase.md` | Stale base-sha marker dropped so the merged permanent keeps exactly one marker. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] Every proposal question ID (`D<n>`) resolves in the design's `## Decisions` ledger (`Decides D1`).
- [x] All questions are resolved.
- [x] `design.md` reflects any decisions reached during grilling.
- [x] `specs/**/*.feature` updated with the missing assertions.
- [x] `givn lint --change keep-agent-prompt-open` exits 0 or 2 (only `@wip` and advisory findings).
- [x] Architecture documentation (arc42) check completed above.

**DESIGN-REVIEW: PASS**
