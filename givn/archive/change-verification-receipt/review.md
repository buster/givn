# Review: change-verification-receipt

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Exact duplicates across features are reported together | Review accepts a current verification receipt | boundary |
| Same assertion shape without exact behavior is reported as a structural match | Review accepts a current verification receipt | boundary |
| A structural subset is reported as an overlap | Review accepts a current verification receipt | boundary |
| A missing permanent specifications directory is an empty corpus | Review accepts a current verification receipt | boundary |
| Archived scenarios are excluded from the audit | Review accepts a current verification receipt | boundary |
| Lexical search reports an empty result for unknown terms | Review accepts a current verification receipt | boundary |
| The transition is idempotent after the removal is committed | Review accepts a current verification receipt | boundary |
| The dedicated check passes on a clean project | Review accepts a current verification receipt | boundary |
| A proposal matching an existing capability surface recommends EXTEND | Review accepts a current verification receipt | boundary |
| A proposal with no matching surface reports no signal | Review accepts a current verification receipt | boundary |
| The upgrade reports rules that survive in reachable history | Review accepts a current verification receipt | boundary |
| Integrity hook passes when tasks.md commit hashes are present | Review accepts a current verification receipt | boundary |
| Integrity hook skips arc42 check when slug cannot be resolved | Review accepts a current verification receipt | boundary |
| Explicit retirement documents a changed behavior | Review accepts a current verification receipt | boundary |
| Consumer runs migration coverage check and sees passed evidence | Review accepts a current verification receipt | boundary |
| An opaque revision identifier passes the integrity check | Review accepts a current verification receipt | boundary |

## Split-or-keep

| Scenario | Decision |
|---|---|

README-IMPACT: none

REVIEW: PASS
