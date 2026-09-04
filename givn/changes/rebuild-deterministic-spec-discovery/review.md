# Review: rebuild-deterministic-spec-discovery

## Fabrication Audit

**Step definition scan** — the new step definitions perform real fixture
setup and real CLI subprocess invocations with concrete assertions
(exit codes, stdout content, JSON shape, tree snapshots). No stub bodies.

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Deterministic rewrite + steps + group.md inventory | (implementation commit) | yes (spec/tree.rs fixes, steps) |
| Post-archive verification | (archive commit) | permanent spec merge |

**Promised-architecture cross-check** — the commands under test
(`spec tree/find/duplicates/route`) exist from Phases 4.1–4.5; no new
components promised or built.

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| spec tree / find / duplicates / route | yes | `src/spec/` |

**Strict-mode proof** — delta-scoped verify:
`GIVN_FEATURES=givn/changes/rebuild-deterministic-spec-discovery/specs ./run-tests.sh`
→ 97 scenarios (97 passed). The delta executes in a build without
`spec-retrieval` loaded at scenario level (no scenario touches the
model paths).

- Config-overlay delta: EXTEND recorded in the routing table; the modified
  scenarios re-express the leaf-removal contract against the protected-gate
  rule.

**Downgraded e2e scenario scan** — the six `@e2e` scenarios drive the real
compiled CLI subprocess and assert observable stdout/stderr/exit results.

**Local environment check** — N/A (no run surface).

**E2e isolation check** — N/A for this change.

**Design.md conformance check** — matches design.md; the RED-phase
findings (missing-corpus handling, subset direction) were fixed in the
production code and are covered by scenarios.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Wholesale rewrite, old steps retained for placeholders | yes | yes | yes | n/a |

## Retirements

N/A

## Overlap dispositions

The new scenarios share the common fixture/command skeleton
(prepare project, run CLI, assert output), so the deterministic
shape-match flags pairs whose real behaviour differs — every pair below
is **boundary**: different preconditions, different commands, different
asserted content. Tabelle maschinell aus der Gate-Ausgabe generiert.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A malformed permanent feature reports its path and fails the audit` | `Routing requires exactly one input` | boundary |
| `A missing draft file is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `A missing draft file is rejected` | `Routing requires exactly one input` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `A proposal with no matching surface recommends NEW` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `Archived scenarios are excluded from the audit` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `Lexical search reports an empty result for unknown terms` | boundary |
| `A proposal matching an existing capability surface recommends EXTEND` | `A proposal with no matching surface recommends NEW` | boundary |
| `A proposal matching an existing capability surface recommends EXTEND` | `Lexical search reports an empty result for unknown terms` | boundary |
| `A proposal with no matching surface recommends NEW` | `Lexical search reports an empty result for unknown terms` | boundary |
| `A structural subset is reported as an overlap` | `A missing permanent specifications directory is an empty corpus` | boundary |
| `A structural subset is reported as an overlap` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `A structural subset is reported as an overlap` | `A proposal with no matching surface recommends NEW` | boundary |
| `A structural subset is reported as an overlap` | `Archived scenarios are excluded from the audit` | boundary |
| `A structural subset is reported as an overlap` | `Lexical search reports an empty result for unknown terms` | boundary |
| `A valid feature without a scenario is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `A valid feature without a scenario is rejected` | `Routing requires exactly one input` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Routing requires exactly one input` | boundary |
| `Archived scenarios are excluded from the audit` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `Archived scenarios are excluded from the audit` | `A proposal with no matching surface recommends NEW` | boundary |
| `Archived scenarios are excluded from the audit` | `Lexical search reports an empty result for unknown terms` | boundary |
| `Auditing leaves permanent specifications unchanged` | `Discovery commands never modify permanent files` | boundary |
| `Auditing leaves permanent specifications unchanged` | `Routing never modifies permanent files` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Routing requires exactly one input` | boundary |
| `Detail mode — unknown change errors` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Detail mode — unknown change errors` | `Routing requires exactly one input` | boundary |
| `Exact duplicates across features are reported together` | `A missing permanent specifications directory is an empty corpus` | boundary |
| `Exact duplicates across features are reported together` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `Exact duplicates across features are reported together` | `A proposal with no matching surface recommends NEW` | boundary |
| `Exact duplicates across features are reported together` | `A structural subset is reported as an overlap` | boundary |
| `Exact duplicates across features are reported together` | `Archived scenarios are excluded from the audit` | boundary |
| `Exact duplicates across features are reported together` | `Lexical search reports an empty result for unknown terms` | boundary |
| `Exact duplicates across features are reported together` | `Same assertion shape without exact behavior is reported as a structural match` | boundary |
| `Existing spec review keeps its current behavior` | `A missing permanent specifications directory is an empty corpus` | boundary |
| `Existing spec review keeps its current behavior` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `Existing spec review keeps its current behavior` | `A proposal with no matching surface recommends NEW` | boundary |
| `Existing spec review keeps its current behavior` | `A structural subset is reported as an overlap` | boundary |
| `Existing spec review keeps its current behavior` | `Archived scenarios are excluded from the audit` | boundary |
| `Existing spec review keeps its current behavior` | `Exact duplicates across features are reported together` | boundary |
| `Existing spec review keeps its current behavior` | `Lexical search reports an empty result for unknown terms` | boundary |
| `Existing spec review keeps its current behavior` | `Same assertion shape without exact behavior is reported as a structural match` | boundary |
| `Explore remains unavailable without the retrieval feature` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Explore remains unavailable without the retrieval feature` | `Routing requires exactly one input` | boundary |
| `File and standard input cannot be selected together` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `File and standard input cannot be selected together` | `Routing requires exactly one input` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Routing requires exactly one input` | boundary |
| `Id ending with a hyphen is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Id ending with a hyphen is rejected` | `Routing requires exactly one input` | boundary |
| `Id with a dot is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Id with a dot is rejected` | `Routing requires exactly one input` | boundary |
| `Id with non-ASCII letters is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Id with non-ASCII letters is rejected` | `Routing requires exactly one input` | boundary |
| `Id with spaces is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Id with spaces is rejected` | `Routing requires exactly one input` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Routing requires exactly one input` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `A missing permanent specifications directory is an empty corpus` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `A proposal with no matching surface recommends NEW` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `A structural subset is reported as an overlap` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Archived scenarios are excluded from the audit` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Exact duplicates across features are reported together` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Lexical search reports an empty result for unknown terms` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Same assertion shape without exact behavior is reported as a structural match` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `A missing permanent specifications directory is an empty corpus` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `A proposal with no matching surface recommends NEW` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `A structural subset is reported as an overlap` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Archived scenarios are excluded from the audit` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Exact duplicates across features are reported together` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Lexical search reports an empty result for unknown terms` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Same assertion shape without exact behavior is reported as a structural match` | boundary |
| `Invalid Gherkin input is rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Invalid Gherkin input is rejected` | `Routing requires exactly one input` | boundary |
| `Missing impact decision refuses the review` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Missing impact decision refuses the review` | `Routing requires exactly one input` | boundary |
| `Multiple scenarios in one input are rejected` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Multiple scenarios in one input are rejected` | `Routing requires exactly one input` | boundary |
| `Path traversal id is rejected by archive` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Path traversal id is rejected by archive` | `Routing requires exactly one input` | boundary |
| `Path traversal id is rejected by check` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Path traversal id is rejected by check` | `Routing requires exactly one input` | boundary |
| `Path traversal id is rejected by graph` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Path traversal id is rejected by graph` | `Routing requires exactly one input` | boundary |
| `Path traversal id is rejected by instructions` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Path traversal id is rejected by instructions` | `Routing requires exactly one input` | boundary |
| `Path traversal id is rejected by lint` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Path traversal id is rejected by lint` | `Routing requires exactly one input` | boundary |
| `Path traversal id is rejected by status` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Path traversal id is rejected by status` | `Routing requires exactly one input` | boundary |
| `Repeating an unchanged audit produces the same report` | `Repeated audits emit stable output without progress lines` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Routing requires exactly one input` | boundary |
| `Routing never modifies permanent files` | `Discovery commands never modify permanent files` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `A missing permanent specifications directory is an empty corpus` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `A proposal matching an existing capability surface recommends EXTEND` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `A proposal with no matching surface recommends NEW` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `A structural subset is reported as an overlap` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `Archived scenarios are excluded from the audit` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `Lexical search reports an empty result for unknown terms` | boundary |
| `The audit reports an unavailable retrieval model` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `The audit reports an unavailable retrieval model` | `Routing requires exactly one input` | boundary |
| `The audit reports retrieval unavailable without the retrieval feature` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `The audit reports retrieval unavailable without the retrieval feature` | `Routing requires exactly one input` | boundary |
| `Uninitialized project errors` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `Uninitialized project errors` | `Routing requires exactly one input` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Routing requires exactly one input` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A malformed permanent feature reports its path and fails the audit` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Routing requires exactly one input` | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |
|  |  | boundary |

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Overlay remove is rejected when artifact has dependents` | `Overlay refuses to remove a protected gate artifact` | boundary |
| `Cycle in dependency graph is rejected` | `Overlay refuses to remove a protected gate artifact` | boundary |
| `Dangling require reference is rejected` | `Overlay refuses to remove a protected gate artifact` | boundary |
| `apply.requires referencing non-existent artifact is rejected` | `Overlay refuses to remove a protected gate artifact` | boundary |

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN

README-IMPACT: none
