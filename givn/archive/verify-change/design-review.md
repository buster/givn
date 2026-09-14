# Design Review: verify-change

## Grilling

In-session review per the standing assumption directive (A3). The mechanics
are unchanged from the archived seeds: the integrity check's only identifier
rules are emptiness, known placeholders, and duplicates — verified against
`check_revision_identifier_presence` — so the opaque-identifier scenario is
an acceptance pin, not new behavior.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| F1 | Blocker | Rewording the messages breaks the permanent placeholder scenario's `stderr contains "commit hash"`. | The permanent feature is aligned in the implementation commit; the delta carries the same modification for the merge. |
| N2 | Note | The permanent scenario titles keep the words "commit hash". | Titles are scenario identity; renaming them would need removed+added pairs for wording only. Recorded as acceptable (A17): the assertions and messages are neutral. |

## Branch dispositions

- Scope: wording plus the opacity pin; nothing else.
- Testability: the modified scenario fails RED against the old message.
- Personas: agent--technical-role, givn-maintainer--owner — review lenses.
- ADR: none (wording). Arc42: no chapter states the integrity message text.
- Ubiquitous language: "revision identifier" is the glossary term.

DESIGN-REVIEW: PASS
