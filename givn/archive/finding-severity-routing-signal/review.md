# Review: finding-severity-routing-signal

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — the delta has no `@e2e` scenario and none was removed. |
| Empty/no-op step bodies | 0 found in both touched step files. |
| Checked tasks have commits touching production | Clean — setup `e29e9d3e` plus seven scenario commits. |
| Promised components exist | Clean — the severity registry and rendering, the route confidence and ownership weighting, the two new route unit tests, the fixture Givens, and the three guidance edits. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario. |
| `verify.e2e_command` isolation | `./run-tests.sh` vs `./run-tests.sh --e2e`; 533 vs 88 — strictly fewer. |
| Implementation vs design.md | Matches; the modified permanent route scenario is replaced by a removed/added pair, and the permanent corpus is updated by the archive merge. |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

Note: the permanent corpus still asserts the pre-change route behavior for the
replaced scenario until the archive merge; the archive verification runs the
merged corpus and is the gate for the replacement.

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| spec-overlap-gates: detect a duplicate title (existing) | yes | `A duplicate title across two permanent spec files is a hard error` | Real `givn lint` subprocess |
| spec-explore-change: route a proposal (existing) | yes | `Routing a proposal prints ranked candidates with evidence` | Real `givn spec route --change` subprocess |
| instruction-integrity: request artifact instructions (fragment; no inventory) | yes | `An agent requests machine-readable artifact guidance` | Real `givn instructions specs --json` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end and
the new scenarios are output variants. No browser UI exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Severity classification | 06 §6.4, 08 §8.24 | Yes | Severity Classification | lint tasks | `src/cli/lint.rs` | Yes |
| Route confidence | 06 §6.17, 08 §8.24 | Yes | Route Confidence | route tasks | `src/spec/route.rs` | Yes |
| Ownership weighting | 06 §6.17 | Yes | Ownership-Weighted Scoring | route tasks | `src/spec/route.rs` + unit tests | Yes |
| Building-block rows | 05 table | Yes | (named in design) | (covered) | `cli::lint`, `spec::route` rows | Yes |
| R-71 mitigation | 11 table | Yes | Risks | (covered) | fixtures + unit tests | Yes |
| Glossary terms | 12 glossary | Yes | (named in design) | (covered) | two rows | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Finding severity` and `Route confidence` plus the
existing `Capability` and `Use case` terms. Specs, design, and code reuse
those terms: the delta scenarios say "severity", "advisory", "blocking",
"no signal", and "confidence: signal"; the code uses the same words. No
glossary term is reused inconsistently and no undefined term appears in the
spec. Clean.

## Visual review

The change declares no visual interface: it alters CLI text and JSON output,
not a rendered screen. Opposite-classification test: the output is terminal
text; nothing renders HTML, CSS, or JavaScript in a browser, so no browser
driver applies. No `@e2e` scenario in this delta requires a screenshot or
transcript. The visual block is N/A.

## Overlap dispositions

Every non-contradicting shape match involving this change. The pairs share
generic fixture steps but assert different behaviors, so each sits on a
behavior boundary.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Exact duplicates across features are reported together` | `A proposal with no matching surface reports no signal` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `A proposal with no matching surface reports no signal` | boundary |
| `A structural subset is reported as an overlap` | `A proposal with no matching surface reports no signal` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `A proposal with no matching surface reports no signal` | boundary |
| `Archived scenarios are excluded from the audit` | `A proposal with no matching surface reports no signal` | boundary |
| `A proposal matching an existing capability surface recommends EXTEND` | `A proposal with no matching surface reports no signal` | boundary |
| `A proposal with no matching surface recommends NEW` | `A proposal with no matching surface reports no signal` | boundary |
| `Lexical search reports an empty result for unknown terms` | `A proposal with no matching surface reports no signal` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `A proposal with no matching surface reports no signal` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `A proposal with no matching surface reports no signal` | boundary |
| `Explicit retirement documents a changed behavior` | `A proposal with no matching surface reports no signal` | boundary |
| `Consumer runs migration coverage check and sees passed evidence` | `A proposal with no matching surface reports no signal` | boundary |
| `An opaque revision identifier passes the integrity check` | `A proposal with no matching surface reports no signal` | boundary |
| `A non-existent change id is not surfaced` | `A blocking finding renders its severity` | boundary |
| `Review passes when every shape match is disposed and reports the net-delta receipt` | `A goal-aligned capability ranks above a lexically similar candidate` | boundary |
| `An explicit archive path still lints its file` | `The JSON output exposes the finding severity` | boundary |

The deterministic scan additionally reports 297 assertion-subset warnings
between delta and permanent scenarios; all are structural overlaps on generic
shared assertions.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`The JSON output exposes the finding severity` at 8 steps). No split-or-keep
decisions are required.

## Removed+added pairs

One pair: `A proposal with no matching surface recommends NEW` is removed and
`A proposal with no matching surface reports no signal` is added. The new
scenario supersedes the removed one: the empty-overlap case now reports no
signal instead of a NEW recommendation, which is the decided behavior change.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- `severity_for` and the rendering/JSON enrichment: covered by the three lint
  scenarios and the enumeration unit test.
- The route confidence rules and ownership weighting: covered by the three
  route scenarios and the two route unit tests.
- The guidance edits: covered by the routing-rule scenario.

No dead code introduced; no missing coverage; no hard-to-test gap.

## Use-case and Persona conformance

- The lint exit-code contract is preserved; route stays advisory and the
  recorded decision stays mandatory.
- `lint-command` and `spec-explore-change` extend their owners;
  `instruction-integrity` keeps fragment ownership.
- Confirmed Personas: `agent--technical-role` (CLI output) and
  `givn-maintainer--owner` (tooling). No Persona was invented or promoted;
  Gherkin actors remain the user and the agent.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (delta 8 passed, e2e 88
      passed); the archive verification carries the merged-corpus result.
- [x] Coverage classified from inspection; no gap remains.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No redundant unit tests added.
- [x] No `@wip` tags remain in the delta; no implementation-layer detail in
      the spec.
- [x] Canonical E2E policy read; the new scenarios are regular variants.
- [x] Every E2E scenario keeps its real-interface primary assertion.
- [x] Local run command starts the full stack: `./run-tests.sh`; no external
      dependency or digital twin exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; e2e scope is
      strictly smaller (88 vs 533).
- [x] Implementation matches design.md; the removed/added replacement is
      recorded.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
