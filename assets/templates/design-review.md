# Design Review: {{CHANGE_ID}}

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | | | | |

## Resolved by codebase exploration

Items answered without user input by exploring the existing codebase.

| Branch | Finding |
|---|---|
| | |

## Open questions

Questions that could not be resolved and were flagged explicitly.

| # | Question | Blocking? |
|---|---|---|
| | | |

## Architecture documentation (arc42) check

N/A if the arc42 addon is not enabled (`addons.arc42` in `givn/config.yaml`).
Otherwise, fill in — do not leave blank if arc42 is enabled:

| Chapter | Claimed touched by arc42.md? | Content matches design.md? | Still placeholder text? |
|---|---|---|---|
| | | | |

- [ ] `arc42.md`'s "chapters affected" claim (including a "no chapters
      affected" claim) was checked against the actual scope of
      proposal.md/design.md and found plausible.
- [ ] Every non-trivial architecture/technology decision in design.md has a
      corresponding MADR entry in chapter 09.
- [ ] No chapter claimed as "updated" is still scaffolded placeholder text.

## Changes made during hardening

Edits applied to existing artifacts as a result of grilling.

| Artifact | Change summary |
|---|---|
| | |

## Sign-off

- [ ] Every branch of the design tree has been walked.
- [ ] All blocking questions are resolved.
- [ ] `design.md` reflects any decisions reached during grilling.
- [ ] `specs/**/*.feature` updated if any missing scenarios were identified.
- [ ] `givn lint --change {{CHANGE_ID}}` exits 0 or 2 (no parse errors).
- [ ] Architecture documentation (arc42) check completed above (or marked
      N/A because the addon is not enabled) — never left blank.

**DESIGN-REVIEW: PASS** <!-- change to FAIL if any blocker remains -->
