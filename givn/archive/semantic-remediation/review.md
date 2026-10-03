# Review: semantic-remediation

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A DUPLIKAT classification without removal remains blocked` | `A VARIANTE classification without consolidation remains blocked` | boundary |
| `A DUPLIKAT classification without removal remains blocked` | `A VALID-BOUNDARY classification above the boundary remains blocked` | boundary |
| `A VARIANTE classification without consolidation remains blocked` | `A VALID-BOUNDARY classification above the boundary remains blocked` | boundary |
| `Consolidating a VARIANTE candidate verifies the projected remediation` | `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | boundary |
| `Consolidating a VARIANTE candidate verifies the projected remediation` | `A complete classification allows change review and survives regeneration` | boundary |
| `Consolidating a VARIANTE candidate verifies the projected remediation` | `Real CLI review reports successful duplicate remediation` | boundary |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `A complete classification allows change review and survives regeneration` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Real CLI review reports successful duplicate remediation` | boundary |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `A complete classification allows change review and survives regeneration` | `Real CLI review reports successful duplicate remediation` | boundary |
| `Detail mode — unknown change errors` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Detail mode — unknown change errors` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Detail mode — unknown change errors` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Detail mode — unknown change errors` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Uninitialized project errors` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Uninitialized project errors` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Uninitialized project errors` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Uninitialized project errors` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Id with spaces is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Id with spaces is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Id with spaces is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Id with spaces is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Id with non-ASCII letters is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Id with non-ASCII letters is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Id with non-ASCII letters is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Id with non-ASCII letters is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Id with a dot is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Id with a dot is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Id with a dot is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Id with a dot is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Id ending with a hyphen is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Id ending with a hyphen is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Id ending with a hyphen is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Id ending with a hyphen is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Every resolved semantic classification requires a rationale` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Every resolved semantic classification requires a rationale` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Every resolved semantic classification requires a rationale` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Every resolved semantic classification requires a rationale` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Invalid Gherkin input is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Invalid Gherkin input is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Invalid Gherkin input is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Invalid Gherkin input is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Multiple scenarios in one input are rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Multiple scenarios in one input are rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Multiple scenarios in one input are rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Multiple scenarios in one input are rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `A valid feature without a scenario is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `A valid feature without a scenario is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `A valid feature without a scenario is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `A valid feature without a scenario is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `A missing draft file is rejected` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `A missing draft file is rejected` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `A missing draft file is rejected` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `A missing draft file is rejected` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `File and standard input cannot be selected together` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `File and standard input cannot be selected together` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `File and standard input cannot be selected together` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `File and standard input cannot be selected together` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Explore remains unavailable without the retrieval feature` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Explore remains unavailable without the retrieval feature` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Explore remains unavailable without the retrieval feature` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Explore remains unavailable without the retrieval feature` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Path traversal id is rejected by status` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Path traversal id is rejected by status` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Path traversal id is rejected by status` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Path traversal id is rejected by status` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Path traversal id is rejected by archive` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Path traversal id is rejected by archive` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Path traversal id is rejected by archive` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Path traversal id is rejected by archive` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Path traversal id is rejected by check` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Path traversal id is rejected by check` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Path traversal id is rejected by check` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Path traversal id is rejected by check` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Path traversal id is rejected by lint` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Path traversal id is rejected by lint` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Path traversal id is rejected by lint` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Path traversal id is rejected by lint` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Path traversal id is rejected by graph` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Path traversal id is rejected by graph` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Path traversal id is rejected by graph` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Path traversal id is rejected by graph` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Path traversal id is rejected by instructions` | `A VARIANTE score below the boundary without consolidation remains blocked` | boundary |
| `Path traversal id is rejected by instructions` | `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | boundary |
| `Path traversal id is rejected by instructions` | `A VALID-BOUNDARY score exactly at the boundary remains blocked` | boundary |
| `Path traversal id is rejected by instructions` | `Changing a classified endpoint before remediation makes the decision stale` | boundary |
| `Consolidating a VARIANTE candidate verifies the projected remediation` | `A regenerated classified report is byte-for-byte deterministic` | boundary |
| `Consolidating a VARIANTE candidate verifies the projected remediation` | `Removed candidates disappear after classification` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `A regenerated classified report is byte-for-byte deterministic` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Removed candidates disappear after classification` | boundary |
| `A complete classification allows change review and survives regeneration` | `A regenerated classified report is byte-for-byte deterministic` | boundary |
| `A complete classification allows change review and survives regeneration` | `Removed candidates disappear after classification` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Removed candidates disappear after classification` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Real CLI review reports successful duplicate remediation` | boundary |
| `Changing a candidate scenario invalidates its previous classification` | `Changing a permanent endpoint invalidates its prior classification` | boundary |
| `Changing a candidate scenario invalidates its previous classification` | `Any changed endpoint evidence or prior-report metadata invalidates carry-forward` | boundary |
| `Changing a permanent endpoint invalidates its previous classification` | `Any changed endpoint evidence or prior-report metadata invalidates carry-forward` | boundary |
| `Changing a permanent endpoint invalidates its prior classification` | `Any changed endpoint evidence or prior-report metadata invalidates carry-forward` | boundary |
| `Removed candidates disappear after classification` | `Real CLI review reports successful duplicate remediation` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Changing a candidate scenario invalidates its previous classification` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Changing a permanent endpoint invalidates its prior classification` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Any changed endpoint evidence or prior-report metadata invalidates carry-forward` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Changing a candidate scenario invalidates its previous classification` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Changing a permanent endpoint invalidates its prior classification` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Any changed endpoint evidence or prior-report metadata invalidates carry-forward` | boundary |
| `Existing spec review keeps its current behavior` | `Changing a candidate scenario invalidates its previous classification` | boundary |
| `Existing spec review keeps its current behavior` | `Changing a permanent endpoint invalidates its prior classification` | boundary |
| `Existing spec review keeps its current behavior` | `Any changed endpoint evidence or prior-report metadata invalidates carry-forward` | boundary |

## Fabrication Audit

- Step scan: `tests/steps/semantic_remediation_steps.rs` and
  `tests/steps/semantic_remediation_e2e_steps.rs` contain no empty bodies,
  `unimplemented!()`, or silent-pass bodies after GREEN. The initial strict RED
  proof captured `1 scenario (1 failed)`, `1 step (1 failed)`.
- Checked tasks have scenario commits: `1159f7b`, `48a4fb8`, `0b8fa81`,
  `b85cc5f`, `ccf3905`, `58718aa`, `f08b184`, `5bf4bef`, `4ac8cdc`,
  `6ac10ea`, `952293b`, `f02f95d`, and E2E `650987e`. Production verification
  is implemented in `src/spec/retrieval.rs`; the later scenario commits record
  executable black-box evidence and tags.
- Promised files exist: `src/spec/retrieval.rs`,
  `tests/steps/semantic_remediation_steps.rs`,
  `tests/steps/semantic_remediation_e2e_steps.rs`, and the registered Cucumber
  runner modules.
- One `@e2e` interaction exists for one CLI action. The E2E steps invoke the
  compiled `CARGO_BIN_EXE_givn` subprocess and assert exit status, stdout, and
  report status. No browser or external service is involved.
- `givn/commands.yaml` and `run-tests.sh` use distinct full/E2E commands. The
  change-specific counts are 13/112 and 1/5; the E2E count is strictly smaller.
- `givn lint --change semantic-remediation` is clean and no `@wip` tags remain.

## Arc42 Implementation Conformance

Arc42 is enabled. All twelve durable chapter files exist and were checked for
the projected-tree decision, class-specific remediation contracts, stale-entry
risk, and receipt. No new deployment or external dependency is introduced and
all diagrams remain Mermaid.

| Chapter/fact | Durable source / change claim | Design/tasks | Implementation evidence | Match? |
|---|---|---|---|---|
| FR-29 projected remediation | Ch.01 and `arc42.md` | Projected duplicate/variant/boundary contracts | `src/spec/retrieval.rs`, change scenarios | Yes |
| TC-17 retrieval-enabled gate | Ch.02 | Existing feature/no-feature boundary retained | Feature/no-feature path from PR 3 | Yes |
| Check-review interface | Ch.03 | Remediation findings and receipt | `src/cli/check.rs`, report PASS/blocked output | Yes |
| Projected-tree strategy | Ch.04/05 | Prior entry and projected endpoint primitives | `verify_semantic_remediation`, `recompute_prior_pair_score` | Yes |
| Runtime matching/order | Ch.06/08 | Exact/hash, changed evidence, disappearance, stale matching | Candidate disappearance and stale scenarios | Yes |
| ADR-047 | Ch.09 | Decision explicitly selects projected verification | `src/spec/retrieval.rs` and immutable fixtures | Yes |
| Q-67..Q-70 | Ch.10 | Negative and positive class contracts | 12 regular scenarios and E2E | Yes |
| R-44/R-45 | Ch.11 | Disappearance and score-only risks mitigated | Failure findings and below-boundary falsification scenarios | Yes |
| New vocabulary | Ch.12 | Projected remediation, prior entry, variant contract, invariant, receipt | Feature/design/report wording | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage was measured with the configured scripts against the change-specific
runner and both source reports were merged. The archive gate will repeat the
same measurement after the delta is merged into the permanent tree.

```text
Change full runner: 13 scenarios, 112 steps: PASS
Change E2E runner: 1 scenario, 5 steps: PASS
Coverage measurement: PASS
Coverage merge: PASS
Merged Cobertura lines: 3695/11896 = 31.06%
Merged Cobertura branches: 0/0 (no branch counters emitted)
Production remediation path exercised by blocking and successful CLI checks.
```

Coverage classification:

| Region | Classification | Evidence |
|---|---|---|
| Prior-entry parsing, projected duplicate removal, variant outline, boundary invariant, stale and atomic failure paths | Missing test coverage resolved | Regular executable scenarios cover each branch and non-mutation. |
| Real CLI remediation receipt | Missing test coverage resolved | Separate compiled-CLI E2E scenario passes. |
| Optional real-model inference/network failure | Legitimately hard to test | Deterministic local stub is used; unavailable-model behavior is covered by PR 3 and no network service is required. |
| Dead code | None found | Clippy and feature builds pass. |

## Sign-off

- [x] Fabrication audit clean.
- [x] All change scenarios and the E2E scenario pass.
- [x] Coverage measured and classified under the three allowed buckets.
- [x] No `@wip` tags remain.
- [x] Feature spec contains observable domain behavior only.
- [x] Local CLI environment requires no external service or digital twin.
- [x] E2E command isolation and interaction inventory are verified.
- [x] Implementation matches design, including prior-entry matching and projected contracts.
- [x] Arc42 conformance is clean.
- [x] No unresolved finding is excused outside the allowed coverage classifications.

REVIEW: PASS
