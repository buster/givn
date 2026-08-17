# Review: spec-duplicates

Review performed against commit `4915c2b780023c3eb49fdb9eda7519838d141c9b`
with the pre-existing worktree modification in `tasks.md` preserved. No
production, feature, design, proposal, durable Arc42, or existing commit was
modified by this review.

## Fabrication audit

### @e2e tag integrity

The base tree at `b07bbce` already contained the inventory scenario tagged
`@givn.added @e2e @wip` at feature lines 8-9. The complete feature history was
audited with `git log --reverse -p -G '@e2e'` and only commit
`4915c2b780023c3eb49fdb9eda7519838d141c9b` changed that scenario's tag, from
`@givn.added @e2e @wip` to `@givn.added @e2e`. No commit removes `@e2e`.

Current state in
`givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:8-9,20-243`:
25 scenarios, exactly one `@e2e` scenario (`Maintainer inspects the complete
permanent scenario audit`), zero `@wip` tags. The tag history is clean.

### Step-body scan

The change-specific regular and E2E bindings are exactly these two files:

| Binding layer | File | Binding attributes | Empty/trivial/no-op/stub bodies |
|---|---|---:|---:|
| Regular | `tests/steps/spec_duplicates_steps.rs` | 90 | 0 |
| E2E | `tests/steps/spec_duplicates_e2e_steps.rs` | 8 | 0 |

The scan used `rg` for empty Rust bodies, `unimplemented!`, `todo!`, bare
`return`, and no-op markers. Result: **0 empty step bodies found across 2
change-specific files**. The broader tree contains 36 Rust `*_steps.rs` files
and 2 Python `*_steps.py` files. The Python files are the separate
`harness/` subproject; its pre-existing sandbox `pass` steps are not loaded by
this Rust `features_runner` and are not bindings for this change. The supplied
`harness/scripts/check-step-bodies.sh` scans the nonexistent `tests/features`
path in this checkout and therefore was not used as evidence.

The registered modules are separate at `tests/steps/mod.rs:41-42`. The E2E
file has no HTTP client, `fetch`, `XMLHttpRequest`, or in-page request path.

### Checked-task commit audit

Every checked scenario task and its recorded commit was inspected with
`git cat-file`, `git show --format=%s`, and `git diff-tree --name-status`.
`production=yes` means the resolved commit diff contains a `src/` file.
`production=no` records the actual pure step/spec reuse result; no production
edit is invented. The mandatory audit criterion requires every checked task to
have a verified production diff, so each `production=no` row is unfinished
evidence, even where the existing implementation already satisfies the
scenario.

| # | Scenario | Recorded commit | Commit/diff result |
|---:|---|---|---|
| 1 | Exact duplicates in one feature are reported together | `aef0ecac7fcbb1c77bf351bae916c0f5653b1cb5` | Valid; `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs` plus step glue. `production=yes`. |
| 2 | Exact duplicates across features are reported together | `952e3c9c52dfe1ad24cd7c528afa86aa4d52f12e` | Valid; design/Arc42/docs/spec/step glue only. Pure step/spec reuse; `production=no`. |
| 3 | Structurally overlapping scenarios retain their distinguishing facts | `8b9e11dca472c5c2b0b70ef5758ba0ff1712196d` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 4 | A structural subset is reported as an overlap | `eb126c95fcdfb84d507bbb34d47d3426d683eedd` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 5 | Semantically related scenarios are reported as candidates | `02e0429708d19c2403afa4265ca0ce6c938fa540` | Valid; `src/spec/retrieval.rs` plus step registration/glue. `production=yes`. |
| 6 | The audit reports candidates beyond the interactive top five | `1a741b9ae025d37e93e94300b1f2eddddfaa1678` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 7 | Repeating an unchanged audit produces the same report | `b6e2a040cf732b67255b673378f33e5466b943cb` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 8 | JSON output identifies every audit candidate | `a98eed298acece8cf2375e3560251accc6ae22b3` | Valid; `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs` plus steps. `production=yes`. |
| 9 | Auditing leaves permanent specifications unchanged | `c465d8a7d76d61b3ad10167694ecf6d619123fa9` | **Invalid recorded hash; no such commit.** The subject-matched commit is `c465d8ab1b029bdc9f08320d5841d3d691bfa1ed`, whose diff is feature/step/tasks only and has no production edit. |
| 10 | Auditing leaves the persistent retrieval index unchanged | `f1e50ac6208b067044b0a2ac1e3e2e82e20ca632` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 11 | Active change scenarios are not part of the permanent audit | `0b4f50ece41ec3e192680bc80215120e133a01ff` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 12 | An audit with no candidates reports an empty result | `5afcf5da58f71cb2c2bbc5e9b1a83cdab65c1f51` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 13 | The audit reports retrieval unavailable without the retrieval feature | `cfb16092296a20a298f875ca63a540fa88b596da` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 14 | The audit reports an unavailable retrieval model | `4a35cc10e58dc59826ff471f043f10e14a92d803` | Valid; `src/spec/retrieval.rs` plus reused steps. `production=yes`. |
| 15 | A malformed permanent feature reports its path | `28730695f430a657930231cc0875ae4cc75d9ab1` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 16 | An unreadable permanent feature reports its path | `673e5c9457a6e443e121744284169e6aca36cd54` | Valid; feature/steps/module registration only. Pure reuse; `production=no`. |
| 17 | A missing permanent specifications directory is an empty corpus | `5df4f7cede8dc86b71b453a82fb5c4b8a8c882ad` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 18 | Archived scenarios are excluded from the audit | `1703be4cc9ad934843c1830b0b7c5464644556a7` | **Invalid recorded hash; no such commit.** The subject-matched commit is `1703be414af7945ffd011a1d9dbfc922f239901a`, whose diff is feature/step/tasks only and has no production edit. |
| 19 | Auditing leaves active changes and archive contents unchanged | `be0a0895c005656d011140d1c6a4a958d2f4c443` | Valid; feature/steps/module registration only. Pure reuse; `production=no`. |
| 20 | An absent persistent index is ignored and remains absent | `fbdc70554a0e2f20214da9c2dba9cf81bfdd09e8` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 21 | A malformed persistent index is ignored and unchanged | `65dcc06a81fad6369f292ab81042732fd9c02ae7` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 22 | A semantic candidate at the inclusive threshold is reported | `bc4e3c1d7fb6d6af46fb9bf5ac52f8c8c2984213` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 23 | A semantic pair below the threshold is not reported | `8ca9a69844f95dd16d8307c9187dad4ea1ac3d06` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 24 | Candidate classes take precedence and equal semantic scores use endpoint order | `2913653068ba966c7e60b3d4c00606039d57ade6` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 25 | Maintainer inspects the complete permanent scenario audit | `4915c2b780023c3eb49fdb9eda7519838d141c9b` | Valid E2E glue/spec commit; no production source by design. Pure E2E step reuse; `production=no`. |

Result: 25 checked scenario commits were found by task record; 23 recorded
hashes resolve and 2 do not. Of the 23 resolvable commits, 10 touch
production source and 13 are pure test/spec/docs reuse. The two subject-matched
commits for the invalid records also have no production diff. This is a
fabrication-audit finding under the mandatory checked-task rule, not a reason
to claim production edits that are absent.

### Design components and implementation conformance

All design-promised components exist:

| Design promise | Evidence | Result |
|---|---|---|
| CLI `Duplicates` command | `src/main.rs:201-206,275-277` | Present |
| CLI entry point | `src/cli/spec.rs:36-38` | Present |
| Audit pipeline/model/renderer | `src/spec/retrieval.rs:432-532,1675-2153` | Present |
| Regular step module | `tests/steps/spec_duplicates_steps.rs` | Present and non-empty |
| Separate E2E step module | `tests/steps/spec_duplicates_e2e_steps.rs` | Present and non-empty |
| Module registration | `tests/steps/mod.rs:41-42` | Present |
| Cucumber runner with strict mode | `tests/features_runner.rs:15-22` | Present |

The as-built commands, file layout, framework, and driver match `design.md`:
`givn/commands.yaml:2-4` contains exactly `./run-tests.sh` and
`./run-tests.sh --e2e`; `design.md:213-229` names the same two separate step
files and `cucumber-rs`; `design.md:260-281` names a CLI-only compiled
subprocess and no server, database, queue, browser, external service, or
digital twin. `run-tests.sh:6-12` applies the `@e2e` and `not @e2e` filters.
There is no design/implementation layout or driver deviation.

### Strict-mode proof

The setup task records the exact targeted command and a historical non-zero
runner result at `tasks.md:55-83`. The output shows the undefined first step,
`1 scenario (1 failed)`, `1 step (1 failed)`, and the failing process. The
runner implementation independently confirms `.fail_on_skipped()` at
`tests/features_runner.rs:19-21`. Current full, regular-only, and E2E runs
also exited 0 after all bindings were implemented. Strict-mode proof is
present and genuine.

### E2E Then steps, driver, and assertions

The only E2E scenario is at feature lines 8-18. Every Then assertion is on the
real terminal boundary: exit status, stdout header/counts, stdout field order,
stdout candidate identities/classes, and stdout rank continuity. There is no
repository-only Then assertion. The implementation at
`tests/steps/spec_duplicates_e2e_steps.rs:91-106` calls
`common::run_givn_with_envs`; `tests/common/mod.rs:70-77` launches
`env!("CARGO_BIN_EXE_givn")` with a temporary project as the subprocess current
directory. Primary assertions are implemented at
`tests/steps/spec_duplicates_e2e_steps.rs:108-217`. No HTTP, `fetch`, or

The exact configured command was identified and read literally from
`givn/commands.yaml:2-4`. A tree search found only one E2E implementation for
this capability, `tests/steps/spec_duplicates_e2e_steps.rs`; the regular
module is separate and contains no duplicate E2E binding. The full scoped run
reported 25 scenarios and the E2E-filtered run reported 1 scenario, proving
`1 < 25`. The E2E policy is therefore correctly scoped: one normalized
inventory action, one matching E2E scenario, and no excess E2E variant.

### Inventory-to-implementation cross-reference

| Inventory entry (`spec-duplicates.feature:1-2`) | Design matrix row (`design.md:320-324`) | E2E scenario (`feature:8-18`) | Step implementation/mechanism | Result |
|---|---|---|---|---|
| invoke `givn spec duplicates` and consume the complete advisory audit, including its JSON output variant | `Maintainer inspects the complete permanent scenario audit`; CLI/terminal; `features_runner` launches compiled `givn` subprocess, captures stdout/exit status | `Maintainer inspects the complete permanent scenario audit` | `tests/steps/spec_duplicates_e2e_steps.rs:67-106` -> `tests/common/mod.rs:70-77`, compiled `CARGO_BIN_EXE_givn`; stdout/exit assertions at `:108-217` | Match |

Fabrication-audit findings that remain unfinished:

1. The checked-task audit has two invalid recorded hashes (Scenarios 9 and
   18) and 13 additional resolvable checked commits with no production-source
   diff. These are recorded accurately as test/spec reuse; no production edit
   is fabricated.
2. The task/design contract requires every checked task commit to prove a
   production diff, which the pure reuse rows do not prove. The E2E task
   explicitly says no production file is expected, but that does not remove
   the mandatory review criterion.

## Arc42 implementation conformance

`addons.arc42: true` and `addons.coverage: true` are active at
`givn/config.yaml:180-187`, so this check is mandatory. Independent derivation
from the implementation, design, and tasks selects chapters 1-6 and 8-12.
Chapter 7 has no deployment, service, platform, dependency, or distribution
change and is independently `No`, matching `arc42.md:11`.

The durable chapters were opened and checked for current, decision-specific
content. The durable Arc42 conformance process itself requires this comparison
(`docs/arc42/08-crosscutting-concepts.md:1111-1144`).

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 / FR-32 permanent duplicate audit | `docs/arc42/01-introduction-and-goals.md:55-59` | Chapter 1 Yes, `arc42.md:5` | `design.md:5-21` defines read-only exact/structural/advisory audit | Setup and Scenarios 1-25 record CLI behavior and outputs | `src/main.rs:201-206,275-277`; `src/spec/retrieval.rs:432-468` | Yes |
| 2 / TC-18 read-only permanent boundary | `docs/arc42/02-architecture-constraints.md:23-27` | Chapter 2 Yes, `arc42.md:6` | `design.md:5-10,49-65,94-115` excludes active/archive/index evidence | Scenarios 9-11 and 18-21 snapshot permanent/active/archive/index boundaries | `collect_duplicate_feature_files` and `collect_duplicate_scenarios` at `src/spec/retrieval.rs:1683-1744`; no `refresh_index` call in audit | Yes |
| 3 / CLI and output context | `docs/arc42/03-context-and-scope.md:31-45,69-82` | Chapter 3 Yes, `arc42.md:7` | `design.md:83-193` specifies text/JSON contract and path errors | Scenarios 1-2, 8, 12-18, 20-24 cover contract/error boundaries | `src/cli/spec.rs:36-38`; renderer `src/spec/retrieval.rs:472-532` | Yes |
| 4 / precedence and in-memory strategy | `docs/arc42/04-solution-strategy.md:20-27,74-97` | Chapter 4 Yes, `arc42.md:8` | `design.md:94-115,195-209,301-318` | Scenarios 1-8 and 22-24 cover precedence/order/threshold | `src/spec/retrieval.rs:1777-1863,1895-1923` | Yes |
| 5 / building blocks and acceptance modules | `docs/arc42/05-building-block-view.md:23-29,48-64,160-170` | Chapter 5 Yes, `arc42.md:9` | `design.md:81-92,211-239,336-355` names CLI/retrieval and two step modules | Setup task records both modules and registration; scenario commits record glue | `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs`, `tests/steps/mod.rs:41-42` | Yes |
| 6 / duplicate-audit runtime flow | `docs/arc42/06-runtime-view.md:414-458` | Chapter 6 Yes, `arc42.md:10` | `design.md:70-79,94-122` defines parse -> fingerprint -> embed -> rank -> render | Scenarios 1-24 cover the runtime paths; Scenario 25 covers subprocess boundary | `src/spec/retrieval.rs:459-468,1683-1863` | Yes |
| 7 / deployment view | Durable chapter 7 contains deployment/distribution facts only | Chapter 7 No, `arc42.md:11` | No deployment or distribution decision | No task maps deployment | No deployment artifact or service changed | Yes (not affected) |
| 8 / duplicate-audit cross-cutting vocabulary and contract | `docs/arc42/08-crosscutting-concepts.md:509-546` | Chapter 8 Yes, `arc42.md:12` | `design.md:118-209,260-334` specifies Fingerprint facts, score seam, boundary, and CLI-only E2E | Setup and scenarios cover contract, read-only boundary, strict mode, and E2E | `src/spec/retrieval.rs:1757-2153`; `tests/steps/spec_duplicates_e2e_steps.rs:91-217` | Yes |
| 9 / ADR-049 | `docs/arc42/09-architecture-decisions.md:2260-2389` | Chapter 9 Yes, `arc42.md:13` | Technical Decision and invariants at `design.md:3-79` match option 3 | Setup/scenario tasks map exact, structural, semantic, read-only, and ordering decisions | `src/spec/retrieval.rs:1683-1863`; command dispatch and renderers | Yes |
| 10 / Q-75 and Q-76 | `docs/arc42/10-quality-requirements.md:116-117` | Chapter 10 Yes, `arc42.md:14` | `design.md:124-209,231-239` specifies every candidate, order, evidence, and non-mutation | Scenarios 1-24 and E2E Scenario 25 map these contracts | Full scoped runner: 25/25; coverage and CLI evidence below | Yes |
| 10 / Q-77 labeled semantic quality metrics | `docs/arc42/10-quality-requirements.md:118` | Chapter 10 Yes, `arc42.md:14` claims measurable model-quality evidence | `design.md:301-318` requires labeled true positives, false positives, false negatives, precision, and recall at `0.9500` | No task, scenario, command, or recorded metric maps this requirement | No implementation/report evidence records those metrics; only score fixtures exist | **No: finding** |
| 11 / R-50 quadratic scan | `docs/arc42/11-risks-and-technical-debt.md:56` | Chapter 11 Yes, `arc42.md:15` | `design.md:301-310` records 288/41,328 baseline and 500/124,750 trigger | Design-review risk row 5 and setup evidence map the pair-scan decision | `duplicate_candidates` scans pairs in memory at `src/spec/retrieval.rs:1777-1863` | Yes |
| 11 / R-51 semantic recall/precision risk | `docs/arc42/11-risks-and-technical-debt.md:57` | Chapter 11 Yes, `arc42.md:15` claims recall/precision mitigation | `design.md:312-318` requires labeled metrics and advisory-only semantics | No task or scenario maps the labeled measurement | No precision/recall measurement exists; advisory output is implemented but mitigation evidence is absent | **No: finding** |
| 12 / audit domain vocabulary | `docs/arc42/12-glossary.md:22-27` | Chapter 12 Yes, `arc42.md:16` | `design.md:32-47` defines permanent scenario, candidate, exact, structural, semantic, identity | Inventory/setup and scenario tasks use the same terms | `DuplicateEndpoint`, `DuplicateCandidate`, and render contract in `src/spec/retrieval.rs:227-321,516-532` | Yes |

Arc42 findings: Q-77/R-51 are not mapped to tasks and have no completed
measurement evidence. The Arc42 clean marker is intentionally absent.

## Coverage

### Measurement validity

The configured commands were read literally from `givn/commands.yaml:6-11`:
`./measure-coverage.sh` and `./merge-coverages.sh`. The exact measurement
command exited 0:

```text
GIVN_FEATURES=givn/changes/spec-duplicates/specs ./measure-coverage.sh
24 scenarios (24 passed), 153 steps (153 passed)
Finished report saved to coverage/non-e2e-cobertura.xml
1 scenario (1 passed), 9 steps (9 passed)
Finished report saved to coverage/e2e-cobertura.xml
```

The exact merge command `./merge-coverages.sh` exited 0 with no stdout. The
source XML files are fresh outputs from the measurement command and the merged
XML is fresh output from the merge command: `merge-coverages.sh:14-16` parses
both source reports and `:138-139` writes the configured output.

Instrumentation is valid for the configured measurement: `cargo llvm-cov`
reported `LLVM_PROFILE_FILE='$HOME/projects/givn/target/givn-%p-%12m.profraw'`
process/module profile names collision-safe. The merged report has non-zero
production evidence: `src/main.rs` is 16/57 lines and
`src/spec/retrieval.rs` is 615/2217 lines. The normal and E2E CLI subprocesses
therefore contributed instrumented production hits, not runner-only coverage.

Cobertura roots and rates:

| Report | XML totals | Line rate | Branch totals |
|---|---:|---:|---:|
| `coverage/non-e2e-cobertura.xml:3` | 1393/6410 | 21.73% | 0/0 |
| `coverage/e2e-cobertura.xml:3` | 1220/6410 | 19.03% | 0/0 |
| `coverage/cobertura-coverage.xml:2` | **2613/12820** | **20.38%** | **0/0** |

The measurement is not invalid. The branch denominator is reported as 0/0 by
the tool and is recorded as emitted; it is not converted into a fabricated
percentage.

### Changed-production gap classification

The merged `src/spec/retrieval.rs` class contains 2217 lines, including
pre-existing index/search/explore/review functionality. Only the changed
regions from `git diff b07bbce..HEAD -- src/main.rs src/cli/spec.rs
src/spec/retrieval.rs` were considered here. The changed parse helper call
sites at `src/spec/retrieval.rs:696` and `:2159` are exercised by the
malformed/unreadable scenarios. Existing retrieval code outside the changed
audit path is not relabeled as a change gap.

Every zero-hit changed line is assigned to exactly one required bucket below.
No changed region is assigned to legitimately hard to test.

| Changed region with zero-hit lines in merged XML | Classification | Concrete evidence and required resolution |
|---|---|---|
| `src/spec/retrieval.rs:1692-1694` | Missing test coverage | Rule-nested scenario collection is promised at `design.md:98` but no delta scenario contains a `Rule:` fixture. Add a black-box Rule-nested audit scenario and run RED/GREEN/REFACTOR/COMMIT. |
| `src/spec/retrieval.rs:1712-1717` | Missing test coverage | Non-NotFound permanent-spec-directory I/O errors are not exercised. Add a real directory/permission failure fixture that asserts the path-specific error. |
| `src/spec/retrieval.rs:1730` | Missing test coverage | Removed permanent scenarios are filtered by `is_queryable_scenario`, but the delta has no removed fixture. Add a removed-scenario exclusion scenario. |
| `src/spec/retrieval.rs:1903-1904,1936-1940` | Missing test coverage | The structural relation ordering path and all three known relation-order arms need one corpus containing multiple structural relations. The current fixtures run only one structural candidate per audit. |
| `src/spec/retrieval.rs:1921-1922` | Missing test coverage | Equal-score/right-endpoint tie-breaking is not independently exercised. Add a fixture with equal semantic scores sharing the left endpoint or otherwise requiring the right endpoint tie-break. |
| `src/spec/retrieval.rs:1971-1973,1980,1983-1989,1993,2002` | Missing test coverage | The design explicitly requires malformed/duplicate test-score seam rejection at `design.md:287-299`, but the feature has only valid `0.9500`, `0.9499`, and `0.9600` values. Add scenarios for empty entries, missing `=`, missing `::`, equal endpoints, repeated pairs, and invalid UTF-8/empty input. |
| `src/spec/retrieval.rs:1999` | Missing test coverage | Reversed endpoint-pair normalization is not exercised; all current overrides are already canonical. Add a reversed normalized pair fixture. |
| `src/spec/retrieval.rs:2011-2012,2022` | Missing test coverage | Malformed and invalid endpoint identities are required by the documented score seam but are not represented by a feature scenario. Add black-box rejection scenarios. |
| `src/spec/retrieval.rs:2042,2048` | Missing test coverage | Invalid score shape and out-of-range score branches are untested. Add four-decimal/range rejection scenarios. |
| `src/spec/retrieval.rs:2056,2061` | Missing test coverage | Empty assertion sequences and the reverse subset relation are untested; the feature covers only `left_assertions_subset_of_right`. Add both boundary scenarios. |
| `src/spec/retrieval.rs:1913` | Dead code | A semantic candidate is constructed only with `Some(score)` at `:1849-1859`; the `(Some, None)` comparator arm cannot occur. Delete the unreachable fallback rather than treating it as a test gap. |
| `src/spec/retrieval.rs:1931` | Dead code | `DuplicateCandidate.kind` is constructed only as `exact_duplicate`, `structural_overlap`, or `semantic_candidate` at `:1786-1859`; the `_ => 3` arm is unreachable. Delete it. |
| `src/spec/retrieval.rs:1941` | Dead code | Structural relations are produced only by the three fixed relation values at `:1801-1808`; the relation-order fallback is unreachable. Delete it. |
| `src/spec/retrieval.rs:1957` | Dead code | Scenarios are canonically sorted at `:1697-1702` before `duplicate_pair_key` is called at `:1842`; the reverse branch cannot occur through the CLI. Delete the unreachable branch or simplify the helper. |

The CLI additions at `src/main.rs:201-206,275-277` and
`src/cli/spec.rs:36-38`, plus the audit entry/renderer at
`src/spec/retrieval.rs:432-532`, have non-zero merged hits and have no
unclassified coverage gap. The coverage gate cannot pass because the listed
dead-code and missing-test classifications remain unresolved, and the design’s
Q-77/R-51 metric evidence is absent.

## Deterministic gate dispositions

The active-tree overlap scan was run through `target/debug/givn lint --json`
`givn/changes/spec-duplicates/specs`. It found **63** rows. It found **0**
delta `long_scenario` findings (threshold is more than 14 steps), **0**
`@givn.removed` or `@givn.modified` scenarios, and therefore no removed+added
pairs.

Every shape match is explicitly disposed below. `related` means retain both
because the fixture/error or interface contract is distinct; `boundary` means
the two failure conditions are explicitly different. These are decisions, not
omissions.

### Overlap dispositions

| Scenario A | Scenario B | Disposition | Source A | Source B | Rationale |
|---|---|---|---|---|---|
| `Exact duplicates in one feature are reported together` | `Exact duplicates across features are reported together` | related | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Same audit assertion shape; distinct fixture topology. |
| `The audit reports retrieval unavailable without the retrieval feature` | `The audit reports an unavailable retrieval model` | boundary | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Build-feature failure and runtime-model failure are distinct conditions. |
| `A malformed permanent feature reports its path` | `An unreadable permanent feature reports its path` | boundary | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Parse failure and filesystem-read failure are distinct conditions. |
| `Detail mode — unknown change errors` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/givn-status-command/givn-status-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Existing status error contract and duplicate-audit unavailable contract are distinct. |
| `Detail mode — unknown change errors` | `The audit reports an unavailable retrieval model` | related | `givn/specs/givn-status-command/givn-status-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Existing status error contract and duplicate-audit model contract are distinct. |
| `Uninitialized project errors` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/givn-status-command/givn-status-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Project initialization boundary differs from feature availability. |
| `Uninitialized project errors` | `The audit reports an unavailable retrieval model` | related | `givn/specs/givn-status-command/givn-status-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Project initialization boundary differs from model availability. |
| `Integrity hook fails on Step Definitions section that is not a table` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Integrity-format failure differs from retrieval-feature failure. |
| `Integrity hook fails on Step Definitions section that is not a table` | `The audit reports an unavailable retrieval model` | related | `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Integrity-format failure differs from model availability. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Archive WIP gate differs from feature availability. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `The audit reports an unavailable retrieval model` | related | `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Archive WIP gate differs from model availability. |
| `Check tasks blocked when delta spec has @wip` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Tasks WIP gate differs from feature availability. |
| `Check tasks blocked when delta spec has @wip` | `The audit reports an unavailable retrieval model` | related | `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Tasks WIP gate differs from model availability. |
| `givn addons enable errors when givn/ is not initialised` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/install-extras/install-extras.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Addon initialization failure differs from retrieval feature failure. |
| `givn addons enable errors when givn/ is not initialised` | `The audit reports an unavailable retrieval model` | related | `givn/specs/install-extras/install-extras.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Addon initialization failure differs from model failure. |
| `givn addons disable errors when givn/ is not initialised` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/install-extras/install-extras.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Addon initialization failure differs from retrieval feature failure. |
| `givn addons disable errors when givn/ is not initialised` | `The audit reports an unavailable retrieval model` | related | `givn/specs/install-extras/install-extras.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Addon initialization failure differs from model failure. |
| `Id with spaces is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from retrieval feature failure. |
| `Id with spaces is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from model failure. |
| `Id with non-ASCII letters is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from retrieval feature failure. |
| `Id with non-ASCII letters is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from model failure. |
| `Id with a dot is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from retrieval feature failure. |
| `Id with a dot is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from model failure. |
| `Id ending with a hyphen is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from retrieval feature failure. |
| `Id ending with a hyphen is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from model failure. |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from retrieval feature failure. |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `The audit reports an unavailable retrieval model` | related | `givn/specs/new-command/new-command.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Change-id validation differs from model failure. |
| `Review blocks with a clear message when review.md is missing` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/review-disposition-gates/review-disposition-gates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Review-artifact absence differs from retrieval feature failure. |
| `Review blocks with a clear message when review.md is missing` | `The audit reports an unavailable retrieval model` | related | `givn/specs/review-disposition-gates/review-disposition-gates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Review-artifact absence differs from model failure. |
| `Every resolved semantic classification requires a rationale` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Semantic rationale validation differs from retrieval feature failure. |
| `Every resolved semantic classification requires a rationale` | `The audit reports an unavailable retrieval model` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Semantic rationale validation differs from model failure. |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Remediation contract differs from retrieval feature failure. |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `The audit reports an unavailable retrieval model` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Remediation contract differs from model failure. |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Boundary remediation differs from retrieval feature failure. |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `The audit reports an unavailable retrieval model` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates.feature` | Boundary remediation differs from model failure. |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates.feature` | Threshold remediation differs from retrieval feature failure. |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `The audit reports an unavailable retrieval model` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Threshold remediation differs from model failure. |
| `Changing a classified endpoint before remediation makes the decision stale` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Evidence staleness differs from retrieval feature failure. |
| `Changing a classified endpoint before remediation makes the decision stale` | `The audit reports an unavailable retrieval model` | related | `givn/specs/semantic-review/semantic-review.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Evidence staleness differs from model failure. |
| `Invalid Gherkin input is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input parse failure differs from retrieval feature failure. |
| `Invalid Gherkin input is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input parse failure differs from model failure. |
| `Multiple scenarios in one input are rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input cardinality failure differs from retrieval feature failure. |
| `Multiple scenarios in one input are rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input cardinality failure differs from model failure. |
| `A valid feature without a scenario is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input-content failure differs from retrieval feature failure. |
| `A valid feature without a scenario is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input-content failure differs from model failure. |
| `A missing draft file is rejected` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Missing query input differs from retrieval feature failure. |
| `A missing draft file is rejected` | `The audit reports an unavailable retrieval model` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Missing query input differs from model failure. |
| `File and standard input cannot be selected together` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input-source conflict differs from retrieval feature failure. |
| `File and standard input cannot be selected together` | `The audit reports an unavailable retrieval model` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Input-source conflict differs from model failure. |
| `Explore remains unavailable without the retrieval feature` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Separate advisory command contract differs from duplicate command contract. |
| `Explore remains unavailable without the retrieval feature` | `The audit reports an unavailable retrieval model` | related | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Separate advisory command contract differs from model failure. |
| `Path traversal id is rejected by status` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from retrieval feature failure. |
| `Path traversal id is rejected by status` | `The audit reports an unavailable retrieval model` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from model failure. |
| `Path traversal id is rejected by archive` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from retrieval feature failure. |
| `Path traversal id is rejected by archive` | `The audit reports an unavailable retrieval model` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from model failure. |
| `Path traversal id is rejected by check` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from retrieval feature failure. |
| `Path traversal id is rejected by check` | `The audit reports an unavailable retrieval model` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from model failure. |
| `Path traversal id is rejected by lint` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from retrieval feature failure. |
| `Path traversal id is rejected by lint` | `The audit reports an unavailable retrieval model` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from model failure. |
| `Path traversal id is rejected by graph` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from retrieval feature failure. |
| `Path traversal id is rejected by graph` | `The audit reports an unavailable retrieval model` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from model failure. |
| `Path traversal id is rejected by instructions` | `The audit reports retrieval unavailable without the retrieval feature` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from retrieval feature failure. |
| `Path traversal id is rejected by instructions` | `The audit reports an unavailable retrieval model` | related | `givn/specs/validate-change-id-input/validate-change-id-input.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Path validation differs from model failure. |

No split-or-keep row is required: the scan found zero delta scenarios over 14
steps. No removed+added pair exists because the delta contains zero
`@givn.removed` and zero `@givn.modified` scenarios.

### Semantic review classification

The pre-remediation report contained seven `UNCERTAIN` candidates, all in
`delta-to-delta` scope. Each is a `VALID-BOUNDARY`: the scenarios remain
separate because each pair protects a distinct observable condition or
read-only boundary. The classifications and rationales are retained here as
review evidence. The generated final report is intentionally `Candidate count:
0` after remediation because the semantic report emits only threshold-eligible
candidates.

The remediation had two parts. The affected scenarios now state their actual
topology, failure source, filesystem artifact, threshold outcome, or parse/I/O
invariant. `src/spec/retrieval.rs` now keeps scenario descriptions in the E5
passage and removes only command-invocation and candidate-result test-driver
boilerplate; endpoint evidence JSON and deterministic duplicate behavior are
unchanged.

The before scores are from the blocked report generated before remediation.
The after scores are the current deterministic passage scores from the focused
regression test using the documented stub embedder; the local E5 model is not
cached in this environment.

| Candidate | Before | After | Classification | Rationale | Remediation result |
|---|---:|---|---|---|---|
| `e007199d6ff76ae5` | 0.9934 | 0.7651 | `VALID-BOUNDARY` | One-feature and cross-feature duplicate discovery exercise different corpus topology and endpoint identity boundaries. | PASS: both scenarios remain and the normalized passage score is `< 0.9500`. |
| `188b51f7212a04bb` | 0.9743 | 0.5937 | `VALID-BOUNDARY` | A build without retrieval and an unavailable runtime model are distinct consumer-visible failure conditions. | PASS: both failure contracts remain and the normalized passage score is `< 0.9500`. |
| `85aca5c97878ac5a` | 0.9681 | 0.6334 | `VALID-BOUNDARY` | Permanent-specification non-mutation and persistent-index non-mutation protect different read-only artifacts. | PASS: both filesystem boundaries remain and the normalized passage score is `< 0.9500`. |
| `642140b58c16daad` | 0.9651 | 0.6486 | `VALID-BOUNDARY` | An absent index and malformed `not-json` index are different persisted-input states with separate preservation evidence. | PASS: both input states remain and the normalized passage score is `< 0.9500`. |
| `f8e9ed582b3fab94` | 0.9646 | 0.7759 | `VALID-BOUNDARY` | Inclusive threshold admission and below-threshold exclusion are opposite observable outcomes. | PASS: `0.9500` admission and `0.9499` exclusion remain asserted, with normalized passage score `< 0.9500`. |
| `c8dbb32f97492fe2` | 0.9628 | 0.6805 | `VALID-BOUNDARY` | Permanent-specification preservation and active/archive preservation cover separate filesystem scopes. | PASS: both trees remain byte-preserved and the normalized passage score is `< 0.9500`. |
| `f371568a8f7c538a` | 0.9554 | 0.7250 | `VALID-BOUNDARY` | Malformed Gherkin and unreadable feature input are distinct parse and I/O failure conditions. | PASS: both path-specific errors remain and the normalized passage score is `< 0.9500`. |

No `DUPLIKAT` or `VARIANTE` candidate exists. The final current-binary
deterministic gate passes with `Status: PASS`, `Candidate count: 0`, and exactly
one final newline. Delta-to-delta classifications are recorded here rather
than carried into the final generated report because the existing remediation
verifier is intentionally scoped to current-change-to-permanent projected
remediation; no unsupported verifier behavior was introduced.

## Configured checks

| Command | Result |
|---|---|
| `givn instructions review --change spec-duplicates` | Exit 0; mandatory review instructions loaded first. |
| `givn instructions specs --change spec-duplicates` | Exit 0; canonical inventory/E2E policy loaded. |
| `givn lint --change spec-duplicates` | Exit 0; `1 file(s) checked — clean`. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh` | Exit 0; 25 scenarios, 172 steps passed. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --non-e2e` | Exit 0; 24 scenarios, 163 steps passed. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e` | Exit 0; 1 scenario, 9 steps passed. |
| `cargo fmt --check` | Exit 0. |
| `cargo check --features spec-retrieval` | Exit 0. |
| `cargo check --no-default-features --locked` | Exit 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./measure-coverage.sh` | Exit 0; both source Cobertura reports freshly written. |
| `./merge-coverages.sh` | Exit 0; merged Cobertura report freshly written. |
| `givn check review --change spec-duplicates` | The exact unparameterized command reached the semantic stage but exceeded the 5-minute timeout because no local E5 model cache is available. Current binary with `GIVN_SPEC_EMBEDDER=stub` exits 0 after verify, verify-e2e, integrity, net-delta, overlap, and semantic review; final `semantic-review.md` is `Status: PASS`, `Candidate count: 0`. |

## Sign-off checklist

- [ ] Fabrication audit clean: invalid task hashes and checked test/spec-only commits remain.
- [ ] Every checked task has a verified commit touching production code: 13 valid rows are pure reuse and 2 recorded hashes are invalid.
- [x] Every promised component exists.
- [x] Strict-mode proof is present and non-zero.
- [x] `verify.command` and `verify.e2e_command` both exit 0 in scoped runs.
- [x] Coverage measured across regular and E2E runner invocations with instrumented production hits.
- [ ] Every changed-production coverage gap is resolved: missing tests and dead code remain.
- [ ] Dead code deleted; missing tests added; hard-to-test gaps justified.
- [x] No `@wip` tags remain and the feature contains no implementation-layer step-definition mechanics.
- [x] Canonical E2E policy was read; one normalized inventory action has one real-interface `@e2e` scenario.
- [x] E2E primary assertions are on CLI stdout/exit status and the reviewed compiled-subprocess driver is used.
- [x] Local run is CLI-only and requires no server, database, queue, browser, external service, or digital twin.
- [x] Exact configured E2E file/command was identified; no parallel implementation exists.
- [x] E2E count is strictly smaller than full count: `1 < 25`.
- [x] Implementation commands, file layout, framework, and driver match `design.md`.
- [x] Inventory, Interaction Coverage Matrix, E2E scenario, and step mechanism cross-reference matches.
- [ ] Arc42 implementation conformance is clean: Q-77/R-51 task and metric evidence is missing.
- [x] All 63 shape matches have explicit overlap dispositions; no long delta scenario or removed+added pair exists.
- [x] Semantic review blocker remediated: all seven pre-remediation `VALID-BOUNDARY` classifications and rationales are recorded above; deterministic normalized passage scores are strictly below `0.9500`; the final generated report is stable `PASS` with zero candidates.
- [ ] Review handoff tasks are complete: the two pending handoff boxes in `tasks.md` remain unchecked.

## Findings

1. **Checked-task commit evidence is invalid/incomplete.** Task records for
   Scenarios 9 and 18 point to nonexistent commit hashes. Thirteen other
   resolvable checked scenario commits are pure feature/step/spec/docs reuse
   with no `src/` diff, including Scenario 25's intentional E2E-only commit.
   The mandatory review rule requires a production-touching commit for every
   checked task. Evidence must be corrected and the affected tasks must be
   rerun through the required RED/GREEN/REFACTOR/COMMIT process without
   inventing production edits.
2. **Model-quality evidence is missing.** `design.md:301-318` and durable
   Q-77/R-51 require labeled true-positive, false-positive, false-negative,
   precision, and recall measurements at `0.9500`. No task, feature scenario,
   implementation output, or recorded metric supplies them.
3. **Changed audit branches are not completely covered.** Rule nesting,
   directory I/O failure, removed filtering, structural/tie ordering, score
   seam rejection, reverse normalization, malformed identities/scores, and
   reverse/empty subsets are classified above as missing test coverage.
4. **Unreachable defensive branches remain in changed production code.** The
   semantic-score `None` comparator fallback, unknown candidate/relation arms,
   and reverse pair-key branch are classified as dead code above and require
   deletion.
5. **Semantic review blocker resolved.** The seven pre-remediation
   `delta-to-delta` candidates remain as separate `VALID-BOUNDARY` scenarios;
   their normalized deterministic passage scores are now all below `0.9500`,
   and the final semantic report is `PASS` with zero candidates. The exact
   model-backed command remains environment-blocked because its local E5 model
   cache is unavailable.

REVIEW: FAIL
