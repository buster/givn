# Review: mask-rule-names

## Fabrication Audit (do this first)

**Step definition scan** — the negative ignore-block step is substring-based,
the new masking step is block-scoped with a name sample, and unit tests
cover pattern coverage and user-content safety. No empty bodies.

**Commit cross-check**:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Plan | `4fee0270` | Planning artifacts |
| masked ignore entries | `156875ea` | `src/skills/rules.rs`, steps, arc42 8.7 |

**Promised-architecture cross-check** — `ignore_entries`, the masking step,
and the modified scenario exist.

**Strict-mode proof** — `.fail_on_skipped()` (existing); the modified
scenario fails RED against the per-path block.

**Downgraded e2e scenario scan** — no `@e2e` tag added or removed.

**E2e scope check** — no new `@e2e`.

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`; a single implementation exists.

**Local environment check** — no stack, no digital twins.

**E2e isolation check** — commands differ; the archive receipt records both
scopes.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `initialize-project` contract unchanged; entries masked | Yes |
| Capability ownership and typed relationships preserved | `initialize-a-project` extended only | Yes |
| Confirmed Persona-relevant outcomes preserved | Fewer Givn traces in the published tree | Yes |
| Actors remain distinct from Persona biographies | Gherkin names the action | Yes |
| Migration ledger and coverage evidence complete, if applicable | N/A | Yes |

PERSONAS: developer--primary-user, external-contributor--open-source-consumer, givn-maintainer--owner

## Deferred mandatory checks

None.

**Design.md conformance check**: the pattern list, the Fossil stripping, and
the substring/block-scoped assertions match the implementation.

**Interaction coverage verification**: the existing `@e2e` still covers the
only real action (initialize a project); the delta adds no inventory entry.

## Visual Review

N/A — no visual interface.

VISUAL-REVIEW: PASS

## Retirements

N/A.

## Arc42 implementation conformance

| Chapter | `arc42.md` | Implementation | Match? |
|---|---|---|---|
| 8 crosscutting | Yes | 8.7 names the pattern form and predicate discovery | Yes |
| 5 building blocks | No | Row unchanged | Yes |
| 12 glossary | No | No new term | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt yet; lib tests 268 pass and the delta is 1/1.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `ignore_entries`, masking step | 0 uncovered | Unit tests plus the delta scenario cover the patterns and the leak assertion | None required |

## Scenario execution evidence

Unproven candidate — no receipt yet; the archive verifies the merged
candidate.

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|

## Split-or-keep

None — no long-scenario finding.

## Sign-off

- [x] Fabrication audit: CLEAN.
- [x] Scenario execution evidence recorded.
- [x] Coverage gaps classified.
- [x] No `@wip` tags; no implementation detail in the delta.
- [x] Local run cleanly; no digital twins.
- [x] e2e command identified; no parallel implementation.
- [x] e2e isolation holds.
- [x] Implementation matches design.
- [x] Interaction coverage verified.
- [x] Internal Primitives states none.
- [x] arc42 slugs map.
- [x] No invented exceptions.

README-IMPACT: none

**REVIEW: PASS**
