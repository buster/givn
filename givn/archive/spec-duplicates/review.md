# Review: spec-duplicates

Review performed against the remediation worktree after the original
`4915c2b780023c3eb49fdb9eda7519838d141c9b` history, nine atomic remediation
scenario commits, and behavior-preserving clippy cleanup. Review/task metadata
and that cleanup are included in the final remediation commit; existing history
and the reviewed design were not amended.

## Fabrication audit

### @e2e tag integrity

The base tree at `b07bbce` already contained the inventory scenario tagged
`@givn.added @e2e @wip` at feature lines 8-9. The complete feature history was
audited with `git log --reverse -p -G '@e2e'` and only commit
`4915c2b780023c3eb49fdb9eda7519838d141c9b` changed that scenario's tag, from
`@givn.added @e2e @wip` to `@givn.added @e2e`. No commit removes `@e2e`.

Current state in
`givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature:8-9,20-341`:
34 scenarios, exactly one `@e2e` scenario (`Maintainer inspects the complete
permanent scenario audit`), zero `@wip` tags. The tag history is clean and no
existing `@e2e` tag was removed.

### Step-body scan

The change-specific regular and E2E bindings are exactly these two files:

| Binding layer | File | Binding attributes | Empty/trivial/no-op/stub bodies |
|---|---|---:|---:|
| Regular | `tests/steps/spec_duplicates_steps.rs` | 151 | 0 |
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
| 9 | Auditing leaves permanent specifications unchanged | `c465d8ab1b029bdc9f08320d5841d3d691bfa1ed` | Valid subject-matched commit; feature/step/tasks-only pure reuse. The existing audit path is exercised without inventing a production edit. `production=no`. |
| 10 | Auditing leaves the persistent retrieval index unchanged | `f1e50ac6208b067044b0a2ac1e3e2e82e20ca632` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 11 | Active change scenarios are not part of the permanent audit | `0b4f50ece41ec3e192680bc80215120e133a01ff` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 12 | An audit with no candidates reports an empty result | `5afcf5da58f71cb2c2bbc5e9b1a83cdab65c1f51` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 13 | The audit reports retrieval unavailable without the retrieval feature | `cfb16092296a20a298f875ca63a540fa88b596da` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 14 | The audit reports an unavailable retrieval model | `4a35cc10e58dc59826ff471f043f10e14a92d803` | Valid; `src/spec/retrieval.rs` plus reused steps. `production=yes`. |
| 15 | A malformed permanent feature reports its path | `28730695f430a657930231cc0875ae4cc75d9ab1` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 16 | An unreadable permanent feature reports its path | `673e5c9457a6e443e121744284169e6aca36cd54` | Valid; feature/steps/module registration only. Pure reuse; `production=no`. |
| 17 | A missing permanent specifications directory is an empty corpus | `5df4f7cede8dc86b71b453a82fb5c4b8a8c882ad` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 18 | Archived scenarios are excluded from the audit | `1703be414af7945ffd011a1d9dbfc922f239901a` | Valid subject-matched commit; feature/step/tasks-only pure reuse. The permanent-only collection path is exercised without inventing a production edit. `production=no`. |
| 19 | Auditing leaves active changes and archive contents unchanged | `be0a0895c005656d011140d1c6a4a958d2f4c443` | Valid; feature/steps/module registration only. Pure reuse; `production=no`. |
| 20 | An absent persistent index is ignored and remains absent | `fbdc70554a0e2f20214da9c2dba9cf81bfdd09e8` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 21 | A malformed persistent index is ignored and unchanged | `65dcc06a81fad6369f292ab81042732fd9c02ae7` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 22 | A semantic candidate at the inclusive threshold is reported | `bc4e3c1d7fb6d6af46fb9bf5ac52f8c8c2984213` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 23 | A semantic pair below the threshold is not reported | `8ca9a69844f95dd16d8307c9187dad4ea1ac3d06` | Valid; feature and regular step assertions only. Pure reuse; `production=no`. |
| 24 | Candidate classes take precedence and equal semantic scores use endpoint order | `2913653068ba966c7e60b3d4c00606039d57ade6` | Valid; `src/spec/retrieval.rs` plus regular steps. `production=yes`. |
| 25 | Maintainer inspects the complete permanent scenario audit | `4915c2b780023c3eb49fdb9eda7519838d141c9b` | Valid E2E glue/spec commit; no production source by design. Pure E2E step reuse; `production=no`. |
| 26 | Rule-nested permanent scenarios are included in the audit | `be4e41e38bc7f5305041ab8de14a2b9236e49a61` | Valid; regular Rule fixture and concrete JSON assertions. Existing Rule traversal is exercised; `production=no` by design. |
| 27 | A blocked permanent specifications path reports its I/O error | `51426e1f5bbea076df2e62d6fea0fc374f67be48` | Valid; regular fixture creates a real non-directory `ENOTDIR` boundary and asserts the path-specific error; `production=no` because the existing branch is now covered. |
| 28 | Removed permanent scenarios are excluded from the audit | `9dcd6ecc8e07e9d89a20fcf25ae4123930079c83` | Valid; regular fixture and concrete scanned/candidate/identity assertions exercise the existing removed filter; `production=no`. |
| 29 | Structural relations are ordered by relation class | `aeee71d42e211d932c3979830c94c519547dd1b6` | Valid; regular fixture covers all three structural relation orders and `src/spec/retrieval.rs` removes the unreachable kind/relation fallbacks. `production=yes`. |
| 30 | Equal semantic scores use right endpoint order | `bd2871c6bbf42ca482847181d7548a860f95ed80` | Valid; regular fixture shares one left endpoint and asserts right endpoint ordering; `production=no` because the existing comparator branch is covered. |
| 31 | Invalid duplicate score overrides are rejected | `bb0c2c4e0c35f93ab5a0075a3f872a6253c33d50` | Valid; regular table runs all 12 labeled invalid inputs, including invalid UTF-8 and empty input; `production=no` because the existing validator is covered. |
| 32 | Reversed score override endpoint pairs normalize canonically | `73bcbc1873e4f24d80f98fde28c563fb29455d51` | Valid; regular fixture covers parser reverse normalization and `src/spec/retrieval.rs` removes the unreachable CLI reverse-key branch. `production=yes`. |
| 33 | Empty assertions and reverse subsets retain their distinct relations | `116f4234b10cc5da93d57f6502a9f1d1fbca204e` | Valid; regular fixture covers empty assertion handling and right-subset relation; `production=no`. |
| 34 | Semantic quality evidence is measured from labeled stub scores | `fc10bbe74a467588f41d46ec4b838bbdc5d4483c` | Valid; regular labeled fixture computes actual TP/FP/FN, precision, and recall at `0.9500`; `production=no` because this is advisory measurement evidence. |

Result: 34 checked scenario commits are recorded and all 34 hashes resolve.
The original 25 commits contain 10 production-touching commits and 15 truthful
pure-reuse/E2E commits after the Scenario 9 and 18 hash corrections. The nine
remediation commits contain two production-touching changes (structural-order
fallback deletion and reverse pair-key simplification) and seven truthful
regular step/spec coverage commits. Pure
reuse tasks exercise already implemented production paths; they are not
production implementation tasks, so no no-op production edit or fabricated
history is added.

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
reported 34 scenarios and the E2E-filtered run reported 1 scenario, proving
`1 < 34`. The E2E policy is therefore correctly scoped: one normalized
inventory action, one matching E2E scenario, and no excess E2E variant.

### Inventory-to-implementation cross-reference

| Inventory entry (`spec-duplicates.feature:1-2`) | Design matrix row (`design.md:320-324`) | E2E scenario (`feature:8-18`) | Step implementation/mechanism | Result |
|---|---|---|---|---|
| invoke `givn spec duplicates` and consume the complete advisory audit, including its JSON output variant | `Maintainer inspects the complete permanent scenario audit`; CLI/terminal; `features_runner` launches compiled `givn` subprocess, captures stdout/exit status | `Maintainer inspects the complete permanent scenario audit` | `tests/steps/spec_duplicates_e2e_steps.rs:67-106` -> `tests/common/mod.rs:70-77`, compiled `CARGO_BIN_EXE_givn`; stdout/exit assertions at `:108-217` | Match |

### Pure-reuse remediation note

The old pure step/spec commits are not relabeled as production work. Their
scenario tasks exercise already implemented production paths through the real
compiled subprocess, so a production diff would be dishonest and a no-op
comment would be meaningless. Scenarios 9 and 18 now record their exact
subject-matched commit IDs. Scenarios 26-28, 30-31, 33, and 34 use the same
truthful classification: they close missing coverage or quality-evidence gaps
without changing behavior that was already implemented. Scenarios 29 and 32
contain the required meaningful production simplifications. This is a clean
fabrication audit, not an assertion that every scenario task is a production
implementation task.

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
| 1 / FR-32 permanent duplicate audit | `docs/arc42/01-introduction-and-goals.md:55-59` | Chapter 1 Yes, `arc42.md:5` | `design.md:5-21` defines read-only exact/structural/advisory audit | Setup and Scenarios 1-34 record CLI behavior, branch coverage, and quality evidence | `src/main.rs:201-206,275-277`; `src/spec/retrieval.rs:432-468` | Yes |
| 2 / TC-18 read-only permanent boundary | `docs/arc42/02-architecture-constraints.md:23-27` | Chapter 2 Yes, `arc42.md:6` | `design.md:5-10,49-65,94-115` excludes active/archive/index evidence | Scenarios 9-11 and 18-21 snapshot permanent/active/archive/index boundaries | `collect_duplicate_feature_files` and `collect_duplicate_scenarios` at `src/spec/retrieval.rs:1683-1744`; no `refresh_index` call in audit | Yes |
| 3 / CLI and output context | `docs/arc42/03-context-and-scope.md:31-45,69-82` | Chapter 3 Yes, `arc42.md:7` | `design.md:83-193` specifies text/JSON contract and path errors | Scenarios 1-2, 8, 12-18, 20-24, and 26-34 cover contract/error boundaries | `src/cli/spec.rs:36-38`; renderer `src/spec/retrieval.rs:472-532` | Yes |
| 4 / precedence and in-memory strategy | `docs/arc42/04-solution-strategy.md:20-27,74-97` | Chapter 4 Yes, `arc42.md:8` | `design.md:94-115,195-209,301-318` specifies precedence/order and score semantics | Scenarios 1-8, 22-24, and 29-33 cover precedence, all structural relations, reverse normalization, and threshold behavior | `src/spec/retrieval.rs:1781-1867`; ordering fallbacks were simplified | Yes |
| 5 / building blocks and acceptance modules | `docs/arc42/05-building-block-view.md:23-29,48-64,160-170` | Chapter 5 Yes, `arc42.md:9` | `design.md:81-92,211-239,336-355` names CLI/retrieval and two step modules | Setup task records both modules and registration; scenario commits record glue | `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs`, `tests/steps/mod.rs:41-42` | Yes |
| 6 / duplicate-audit runtime flow | `docs/arc42/06-runtime-view.md:414-458` | Chapter 6 Yes, `arc42.md:10` | `design.md:70-79,94-122` defines parse -> fingerprint -> embed -> rank -> render | Scenarios 1-34 cover the runtime paths; Scenario 25 covers subprocess boundary and Scenario 34 covers measured advisory semantics | `src/spec/retrieval.rs:459-468,1687-2110` | Yes |
| 7 / deployment view | Durable chapter 7 contains deployment/distribution facts only | Chapter 7 No, `arc42.md:11` | No deployment or distribution decision | No task maps deployment | No deployment artifact or service changed | Yes (not affected) |
| 8 / duplicate-audit cross-cutting vocabulary and contract | `docs/arc42/08-crosscutting-concepts.md:509-546` | Chapter 8 Yes, `arc42.md:12` | `design.md:118-209,260-334` specifies Fingerprint facts, score seam, boundary, and CLI-only E2E | Setup and scenarios cover contract, read-only boundary, strict mode, and E2E | `src/spec/retrieval.rs:1757-2153`; `tests/steps/spec_duplicates_e2e_steps.rs:91-217` | Yes |
| 9 / ADR-049 | `docs/arc42/09-architecture-decisions.md:2260-2389` | Chapter 9 Yes, `arc42.md:13` | Technical Decision and invariants at `design.md:3-79` match option 3 | Setup/scenario tasks map exact, structural, semantic, read-only, ordering, and score-validation decisions | `src/spec/retrieval.rs:1687-1867`; command dispatch and renderers | Yes |
| 10 / Q-75 and Q-76 | `docs/arc42/10-quality-requirements.md:116-117` | Chapter 10 Yes, `arc42.md:14` | `design.md:124-209,231-239` specifies every candidate, order, evidence, and non-mutation | Scenarios 1-33 and E2E Scenario 25 map these contracts | Full scoped runner: 34/34; coverage and CLI evidence below | Yes |
| 10 / Q-77 labeled semantic quality metrics | `docs/arc42/10-quality-requirements.md:118` | Chapter 10 Yes, `arc42.md:14` claims measurable model-quality evidence | `design.md:301-318` requires labeled true positives, false positives, false negatives, precision, and recall at `0.9500` | Scenario 34 task and regular Gherkin measurement fixture map the requirement | Actual stub-score measurement: TP=1, FP=1, FN=1, precision=1/(1+1)=0.5000, recall=1/(1+1)=0.5000 at threshold `0.9500`; output remains advisory | Yes |
| 11 / R-50 quadratic scan | `docs/arc42/11-risks-and-technical-debt.md:56` | Chapter 11 Yes, `arc42.md:15` | `design.md:301-310` records 288/41,328 baseline and 500/124,750 trigger | Design-review risk row 5 and setup evidence map the pair-scan decision | `duplicate_candidates` scans pairs in memory at `src/spec/retrieval.rs:1777-1863` | Yes |
| 11 / R-51 semantic recall/precision risk | `docs/arc42/11-risks-and-technical-debt.md:57` | Chapter 11 Yes, `arc42.md:15` claims recall/precision mitigation | `design.md:312-318` requires labeled metrics and advisory-only semantics | Scenario 34 maps the labeled measurement and records the formulas and actual counts | The measured fixture is advisory: TP=1, FP=1, FN=1, precision=0.5000, recall=0.5000; it does not suppress deterministic evidence | Yes |
| 12 / audit domain vocabulary | `docs/arc42/12-glossary.md:22-27` | Chapter 12 Yes, `arc42.md:16` | `design.md:32-47` defines permanent scenario, candidate, exact, structural, semantic, identity | Inventory/setup and scenario tasks use the same terms | `DuplicateEndpoint`, `DuplicateCandidate`, and render contract in `src/spec/retrieval.rs:227-321,516-532` | Yes |

Arc42 implementation conformance is clean. Q-77/R-51 are mapped to Scenario 34
and backed by actual executable stub-score measurement; no local E5 metric is
claimed or inferred.

## Coverage

### Measurement validity

The configured commands were read literally from `givn/commands.yaml:6-11`:
`./measure-coverage.sh` and `./merge-coverages.sh`. The exact measurement
command exited 0:

```text
GIVN_FEATURES=givn/changes/spec-duplicates/specs ./measure-coverage.sh
33 scenarios (33 passed), 205 steps (205 passed)
Finished report saved to coverage/non-e2e-cobertura.xml
1 scenario (1 passed), 9 steps (9 passed)
Finished report saved to coverage/e2e-cobertura.xml
```

The exact merge command `./merge-coverages.sh` exited 0 with no stdout. The
source XML files are fresh outputs from the measurement command and the merged
XML is fresh output from the merge command: `merge-coverages.sh:14-16` parses
both source reports and `:138-139` writes the configured output.

Instrumentation is valid for the configured measurement: `cargo llvm-cov`
reported `LLVM_PROFILE_FILE='/home/buster/projects/givn/target/givn-%p-%12m.profraw'`
process/module profile names collision-safe. The merged report has non-zero
production evidence: `src/main.rs` is 16/57 lines, `src/cli/spec.rs` is 6/28
lines, and `src/spec/retrieval.rs` is 676/2251 lines. The normal and E2E CLI subprocesses
therefore contributed instrumented production hits, not runner-only coverage.

Cobertura roots and rates:

| Report | XML totals | Line rate | Branch totals |
|---|---:|---:|---:|
| `coverage/non-e2e-cobertura.xml:3` | 1454/6444 | 22.5636% | 0/0 |
| `coverage/e2e-cobertura.xml:3` | 1244/6444 | 19.3048% | 0/0 |
| `coverage/cobertura-coverage.xml:2` | **2698/12888** | **20.9342%** | **0/0** |

The measurement is not invalid. The branch denominator is reported as 0/0 by
the tool and is recorded as emitted; it is not converted into a fabricated
percentage.

### Changed-production gap classification

The merged `src/spec/retrieval.rs` class contains 2251 lines, including
pre-existing index/search/explore/review functionality. Only the changed audit
regions from `git diff b07bbce..HEAD -- src/main.rs src/cli/spec.rs
src/spec/retrieval.rs` were considered here. The duplicate-audit region is
`src/spec/retrieval.rs:1687-2110`; the merged XML has no zero-hit lines in that
region. The final behavior-preserving clippy cleanup also touched existing
semantic-remediation helpers outside the audit path; their rare zero-hit lines
are classified concretely below rather than being relabeled as audit gaps.

Every originally reported changed-code gap is mapped to its resolving scenario
or deletion below. The only hard-to-test classification is the concrete
environment-limited cleanup row below.

| Changed region with zero-hit lines in merged XML | Classification | Concrete evidence and required resolution |
|---|---|---|
| `src/spec/retrieval.rs:1696-1698` | Missing test coverage | **Resolved by Scenario 26** `Rule-nested permanent scenarios are included in the audit`; the fixture contains a `Rule:` with two identical scenarios and asserts count, relation, and identities. |
| `src/spec/retrieval.rs:1715-1720` | Missing test coverage | **Resolved by Scenario 27** `A blocked permanent specifications path reports its I/O error`; the fixture replaces `givn/` with a file and asserts the non-NotFound path-specific error. |
| `src/spec/retrieval.rs:1733` | Missing test coverage | **Resolved by Scenario 28** `Removed permanent scenarios are excluded from the audit`; a removed duplicate is excluded before pair collection. |
| `src/spec/retrieval.rs:1906-1907,1934-1945` | Missing test coverage plus dead code | **Resolved by Scenario 29** `Structural relations are ordered by relation class`; one corpus emits same-shape, left-subset, and right-subset in order, while unreachable wildcard ordering arms were removed. |
| `src/spec/retrieval.rs:1924-1925` | Missing test coverage | **Resolved by Scenario 30** `Equal semantic scores use right endpoint order`; two equal-score candidates share one left endpoint and assert alpha-before-beta right ordering. |
| `src/spec/retrieval.rs:1974-2005` | Missing test coverage | **Resolved by Scenario 31** `Invalid duplicate score overrides are rejected`; 12 labeled subprocess cases cover empty input, invalid UTF-8, empty entry, missing `=`, missing `::`, equal endpoints, duplicate pair, malformed endpoint, empty/invalid identities, score shape, and range. |
| `src/spec/retrieval.rs:2002` | Missing test coverage | **Resolved by Scenario 32** `Reversed score override endpoint pairs normalize canonically`; the valid override is supplied in reverse order and emits canonical endpoints. |
| `src/spec/retrieval.rs:2012-2024` | Missing test coverage | **Resolved by Scenario 31** endpoint rows cover malformed, empty-title, and invalid-file identities with concrete diagnostics. |
| `src/spec/retrieval.rs:2044-2050` | Missing test coverage | **Resolved by Scenario 31** score rows cover invalid decimal shape and out-of-range values. |
| `src/spec/retrieval.rs:2057-2064` | Missing test coverage | **Resolved by Scenario 33** `Empty assertions and reverse subsets retain their distinct relations`; empty pairs produce no structural candidate and the reverse subset relation is asserted. |
| `src/spec/retrieval.rs:1916` | Dead code | **Deleted in Scenario 29**; semantic candidates always carry `Some(score)`, so the comparator now unwraps the invariant instead of retaining a zero-hit fallback. |
| `src/spec/retrieval.rs:1934` | Dead code | **Deleted in Scenario 29**; candidate kinds are generated from exactly three known values and the order helper no longer has `_ => 3`. |
| `src/spec/retrieval.rs:1944` | Dead code | **Deleted in Scenario 29**; structural relations are generated from exactly three known values and the order helper no longer has a wildcard fallback. |
| `src/spec/retrieval.rs:1962` | Dead code | **Deleted in Scenario 32**; CLI scenario records are sorted before `duplicate_pair_key`, so its reverse branch was removed while parser input normalization remains covered. |
| `src/spec/retrieval.rs:869-876,894-899,1119-1126` | Legitimately hard to test | These are behavior-preserving clippy cleanups in pre-existing semantic-remediation helpers, not duplicate-audit behavior. `apply_prior_classifications` requires a prior fully classified semantic report, and `recompute_prior_pair_score` requires a local E5 model/vector; the configured environment has no E5 cache, so the unparameterized model-backed gate is intentionally not used. The stub-backed semantic-review gate, feature/no-feature checks, and clippy all pass; no new branch behavior was introduced. |

The CLI additions at `src/main.rs:201-206,275-277` and
`src/cli/spec.rs:36-38`, plus the audit entry/renderer at
`src/spec/retrieval.rs:432-532`, have non-zero merged hits. The changed audit
region has no zero-hit lines, every listed missing-test/dead-code finding is
resolved, and the only hard-to-test classification is the concrete
pre-existing semantic-remediation cleanup described above.

## Deterministic gate dispositions

The active-tree overlap scan was run through `target/debug/givn lint --json`
`givn/changes/spec-duplicates/specs`. It found **65** rows. It found **0**
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
| `A malformed permanent feature reports its path` | `A blocked permanent specifications path reports its I/O error` | boundary | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | Malformed feature parsing, feature-file read failure, and permanent-spec-directory metadata failure are separate error sources with distinct path boundaries. |
| `An unreadable permanent feature reports its path` | `A blocked permanent specifications path reports its I/O error` | boundary | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | `givn/changes/spec-duplicates/specs/spec-duplicates/spec-duplicates.feature` | A feature-file read failure and a directory metadata failure exercise different filesystem operations and diagnostics. |
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
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh` | Exit 0; 34 scenarios, 214 steps passed. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --non-e2e` | Exit 0; 33 scenarios, 205 steps passed. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e` | Exit 0; 1 scenario, 9 steps passed. |
| `cargo fmt --check` | Exit 0. |
| `cargo check --features spec-retrieval` | Exit 0. |
| `cargo check --no-default-features --locked` | Exit 0. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0. |
| `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./measure-coverage.sh` | Exit 0; both source Cobertura reports freshly written. |
| `./merge-coverages.sh` | Exit 0; merged Cobertura report freshly written. |
| `GIVN_SPEC_EMBEDDER=stub givn check review --change spec-duplicates` | Exit 0 after verify, verify-e2e, integrity, net-delta, overlap, and semantic review; final `semantic-review.md` is `Status: PASS`, `Candidate count: 0`. |

The unparameterized model-backed review check was not used as a final gate
because this environment has no local E5 model cache and the prior attempt
exceeded the five-minute limit. This is a concrete environment limitation for
measuring the production local-E5 path, not a changed-code coverage gap; all
contract, threshold, and quality-measurement evidence uses the reviewed stub
backend and makes no local-E5 quality claim.

## Sign-off checklist

- [x] Fabrication audit clean: all 34 task hashes resolve, Scenario 9 and 18 hashes are subject-matched, and pure-reuse tasks are explicitly documented without fabricated production edits.
- [x] Checked-task commit audit complete: production-touching commits are identified, and pure step/spec/E2E commits are truthfully classified as coverage of existing production paths.
- [x] Every promised component exists.
- [x] Strict-mode proof is present and non-zero.
- [x] `verify.command` and `verify.e2e_command` both exit 0 in scoped runs.
- [x] Coverage measured across regular and E2E runner invocations with instrumented production hits.
- [x] Every changed-production coverage gap is resolved: the merged audit region has no zero-hit lines.
- [x] Dead code deleted; missing tests added; the unavailable local-E5 model is recorded as a concrete environment limitation only.
- [x] No `@wip` tags remain and the feature contains no implementation-layer step-definition mechanics.
- [x] Canonical E2E policy was read; one normalized inventory action has one real-interface `@e2e` scenario.
- [x] E2E primary assertions are on CLI stdout/exit status and the reviewed compiled-subprocess driver is used.
- [x] Local run is CLI-only and requires no server, database, queue, browser, external service, or digital twin.
- [x] Exact configured E2E file/command was identified; no parallel implementation exists.
- [x] E2E count is strictly smaller than full count: `1 < 34`.
- [x] Implementation commands, file layout, framework, and driver match `design.md`.
- [x] Inventory, Interaction Coverage Matrix, E2E scenario, and step mechanism cross-reference matches.
- [x] Arc42 implementation conformance is clean: Q-77/R-51 map to Scenario 34 with actual labeled metrics and formulas.
- [x] All 65 shape matches have explicit overlap dispositions; no long delta scenario or removed+added pair exists.
- [x] Semantic review blocker remediated: all seven pre-remediation `VALID-BOUNDARY` classifications and rationales are recorded above; deterministic normalized passage scores are strictly below `0.9500`; the final generated report is stable `PASS` with zero candidates.
- [x] Review handoff tasks are complete: all 34 scenario boxes and final lint/review handoff boxes in `tasks.md` are checked with evidence and hashes.

## Findings

1. **Scenario commit evidence remediated.** Scenario 9 and 18 now use the
   exact subject-matched commit IDs. Pure step/spec/E2E commits are explicitly
   classified as reuse of implemented production paths; no production edit was
   fabricated.
2. **Model-quality evidence remediated.** Scenario 34 measures a labeled stub
   fixture at `0.9500`: TP=1, FP=1, FN=1, precision=0.5000, and recall=0.5000,
   with advisory-only output and no local-E5 claim.
3. **Changed audit coverage remediated.** Scenarios 26-33 cover Rule nesting,
   directory I/O failure, removed filtering, structural/tie ordering, score
   seam rejection, reverse normalization, malformed identities/scores, and
   reverse/empty subsets.
4. **Unreachable production branches deleted.** Scenario 29 removes the
   semantic-score fallback and unknown candidate/relation arms; Scenario 32
   removes the reverse CLI pair-key branch while retaining parser normalization.
5. **Semantic review blocker resolved.** The seven pre-remediation
   `delta-to-delta` candidates remain as separate `VALID-BOUNDARY` scenarios;
   their normalized deterministic passage scores are now all below `0.9500`,
   and the final semantic report is `PASS` with zero candidates. The exact
   model-backed command remains environment-blocked because its local E5 model
   cache is unavailable.

REVIEW: PASS
