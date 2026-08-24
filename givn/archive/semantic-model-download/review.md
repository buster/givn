# Review: semantic-model-download

## Fabrication Audit

In progress. The fabrication, E2E, Arc42, coverage, overlap, and semantic
evidence sections will be filled from executed commands before sign-off.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A binary without retrieval reports the explicit unavailable mode` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Detail mode — unknown change errors` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Uninitialized project errors` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Id with spaces is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Id with non-ASCII letters is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Id with a dot is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Id ending with a hyphen is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Every resolved semantic classification requires a rationale` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `A VARIANTE score below the boundary without consolidation remains blocked` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Changing a classified endpoint before remediation makes the decision stale` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `The audit reports retrieval unavailable without the retrieval feature` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `The audit reports an unavailable retrieval model` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Invalid Gherkin input is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Multiple scenarios in one input are rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `A valid feature without a scenario is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `A missing draft file is rejected` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `File and standard input cannot be selected together` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Explore remains unavailable without the retrieval feature` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Path traversal id is rejected by status` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Path traversal id is rejected by archive` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Path traversal id is rejected by check` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Path traversal id is rejected by lint` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Path traversal id is rejected by graph` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |
| `Path traversal id is rejected by instructions` | `A feature-free build keeps retrieval explicitly unavailable` | boundary |

## Fabrication Audit

**@e2e tag integrity:** Six inventory entries have six matching `@e2e`
scenarios. No planned E2E tag was removed. The combined BGE/NLI scenario keeps
both `@nli` and `@nli-combined` and is executed only by the combined feature
phase.

**Step definition scan:**

```text
tests/steps/semantic_model_download_steps.rs: 0 empty/trivial/unimplemented bodies
tests/steps/semantic_model_download_e2e_steps.rs: 0 empty/trivial/unimplemented bodies
0 fabricated step bodies found across 2 capability files.
```

Regular steps create isolated projects, invoke the compiled CLI, parse returned
JSON, inspect stderr, and assert cache/index state. E2E steps invoke the
compiled CLI subprocess and assert CLI stdout/stderr and exit status first;
filesystem checks are secondary evidence.

**Commit cross-check:**

| Scenario | Commit | Production/runtime evidence |
|---|---|---|
| A successful E5 setup is reused on the next indexing run | `dd3079e` | XDG E5 resolver and CLI cache-reuse output |
| A failed model download explains how to provide the missing artifacts | `c41b655` | E5 failure diagnostic with cache and override guidance |
| A failed model download leaves an existing permanent index unchanged | `f98bbd2` | Failure transaction preserves index bytes |
| Malformed or incompatible indexes are not reported as absent | `94fde14` | Search distinguishes absent from parse failure |
| Duplicate audit identifies an absent persistent index without changing its fallback contract | `b686cd6` | Duplicate fallback retains read-only transient behavior and emits hint |
| Failed second-stage downloads remain unavailable and actionable | `f8541ff` | BGE/NLI unavailable statuses and diagnostics |
| A corrupt second-stage cache is repaired before evidence scoring | `7c87c6d` | HF Hub resolver, validation, cache repair, manifest, and local fixture server |
| Explicitly disabled evidence layers do not download models | `02b61b0` | Regression scenario reuses the production disabled-selector path; no downloader is invoked |
| Archive maintenance does not download models implicitly | `0a4c4f9` | Explicit no-download archive retrieval APIs and archive path |
| NLI uses the bounded E5 neighbour union for the permanent duplicate audit | `532d569` | Regression scenario locks the existing independent full-E5 NLI pool contract |
| A feature-free build keeps retrieval explicitly unavailable | `78bd375` | Regression scenario locks the production feature-free error path |
| First semantic indexing downloads E5 artifacts before applying the token guard | `e16fe88` | E5 resolver, event ordering, XDG cache, and compiled CLI fixture |
| Search identifies the missing-index repair command | `8e875f7` | Compiled CLI search and shared diagnostic |
| Explore identifies the missing-index repair command | `a2f14ce` | Compiled CLI Explore and shared diagnostic |
| Advisory review identifies the missing-index repair command | `f7df3f3` | Compiled CLI advisory review and shared diagnostic |
| Blocking review identifies the missing-index repair command | `f15ed15` | Compiled CLI blocking review and shared diagnostic |
| Duplicate audit identifies the second-stage model caches | `30ae663` | Combined BGE/NLI compiled CLI, HF fixture, and XDG caches |

The three explicit regression scenarios marked as reuses (`disabled` evidence,
NLI pool wording, and feature-free fallback) intentionally protect existing
production contracts rather than introduce a new production branch. Their
steps execute the public binary and assert those contracts; no spec-only task
is being used to claim an unimplemented behavior.

**Promised component cross-check:**

| Design component | Exists | Evidence |
|---|---|---|
| XDG role-cache resolver | Yes | `src/spec/retrieval.rs:model_cache_dir`, `e5_model_cache_dir` |
| HF Hub artifact resolver | Yes | `src/spec/retrieval.rs:fetch_hf_artifacts`, `hf_api` |
| Artifact manifest and digest validation | Yes | `ArtifactManifest`, `artifact_digest`, `write_artifact_manifest` |
| E5 setup-before-token measurement | Yes | `auto_e5_artifacts`, `embed_texts_with_policy`, event fixture |
| BGE automatic snapshot path | Yes | `auto_bge_artifacts`, `BgeBackendKind::Auto` |
| NLI snapshot/flat path resolution | Yes | `NliArtifacts`, `auto_nli_artifacts`, `NliClassifier::load` |
| Explicit archive no-download path | Yes | `index_without_download`, `review_without_download`, `cli/archive.rs` |
| Regular capability bindings | Yes | `tests/steps/semantic_model_download_steps.rs` |
| Separate CLI E2E bindings | Yes | `tests/steps/semantic_model_download_e2e_steps.rs` |
| Local artifact-source digital twin | Yes | `tests/common/mod.rs:ArtifactFixtureServer` |
| User-facing setup guide | Yes | `docs/retrieval-models.md` |

**Strict-mode proof:** `tasks.md` records the setup run against an
`unimplemented!()` step. It exited non-zero with one failed scenario and one
failed step. The runner uses `GivnWorld::cucumber().fail_on_skipped()`.

**E2E scope and primary assertion check:**

| Scenario | Real action | CLI assertion | Downgraded? |
|---|---|---|---|
| First semantic indexing downloads E5 artifacts before applying the token guard | `givn spec index` | exit 0 and completed-index stdout | No |
| Search identifies the missing-index repair command | `givn spec search` | missing-index stderr | No |
| Explore identifies the missing-index repair command | `givn spec explore` | missing-index stderr | No |
| Advisory review identifies the missing-index repair command | `givn spec review` | missing-index stderr | No |
| Blocking review identifies the missing-index repair command | `givn check review` | missing-index stderr | No |
| Duplicate audit identifies the second-stage model caches | `givn spec duplicates --format json` | valid JSON, participating BGE/NLI evidence | No |

`verify.e2e_command` is `./run-tests.sh --e2e`, which invokes
`tests/features_runner.rs`; the capability E2E bindings are registered from
`tests/steps/semantic_model_download_e2e_steps.rs`. No parallel or stronger
implementation exists for this capability.

**E2E isolation evidence:** `./run-tests.sh` with the active feature path
passed `13` E5 scenarios, `1` NLI scenario, and `2` combined scenarios. The
E2E command passed `5` E5 scenarios and `1` combined scenario. The E2E subset
therefore contains `6` scenarios versus `16` active full-matrix phase
scenarios. The configuration values are distinct:

```text
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
```

**Interaction coverage verification:**

| Inventory entry | Matrix row | Matching scenario | Actual driver | Match |
|---|---|---|---|---|
| build or refresh the permanent semantic index through `givn spec index` | Yes | First semantic indexing downloads E5 artifacts before applying the token guard | compiled CLI subprocess | Yes |
| search permanent semantic scenarios through `givn spec search` | Yes | Search identifies the missing-index repair command | compiled CLI subprocess | Yes |
| explore a permanent or change-aware semantic scenario through `givn spec explore` | Yes | Explore identifies the missing-index repair command | compiled CLI subprocess | Yes |
| request advisory semantic review through `givn spec review` | Yes | Advisory review identifies the missing-index repair command | compiled CLI subprocess | Yes |
| request blocking semantic review through `givn check review` | Yes | Blocking review identifies the missing-index repair command | compiled CLI subprocess | Yes |
| run a second-stage semantic audit through `givn spec duplicates --format json` | Yes | Duplicate audit identifies the second-stage model caches | compiled CLI subprocess | Yes |

## Arc42 Implementation Conformance

The Arc42 addon is enabled. All twelve affected rows in `arc42.md` were
cross-checked against the durable chapters, design, tasks, and implementation:

| Chapter/fact | Durable documentation | Design/tasks | Implementation evidence | Match |
|---|---|---|---|---|
| Goals and quality requirements | Chapters 1 and 10 | FR-38/39, NFR-7/8, Q-58..Q-61 | XDG acquisition and recovery scenarios | Yes |
| Constraints | Chapter 2 | XDG/override/no-implicit-download contract | `model_cache_dir`, archive policy | Yes |
| Context and scope | Chapter 3 | Artifact source and cache boundary | HF endpoint and local fixture | Yes |
| Strategy | Chapter 4 | Resolver before tokenizer/model use | `embed_texts_with_policy` | Yes |
| Building blocks | Chapter 5 | Resolver, cache, and loader responsibilities | `retrieval.rs`, `hf-hub`, `sha2` | Yes |
| Runtime flows | Chapter 6 | Cold acquisition, validation, diagnostics, archive policy | CLI paths and fixture requests | Yes |
| Deployment | Chapter 7 | XDG role caches and flat/snapshot layouts | role roots and NLI model paths | Yes |
| Cross-cutting concepts | Chapter 8 | Ownership, error handling, index hint | shared diagnostic and manifests | Yes |
| Architecture decisions | Chapter 9, ADR-039, ADR-057 | Separate `AMEND_ADR` routes | amendment notes and indexes | Yes |
| Risks/debt | Chapter 11 | Shared-cache, disk/network, XDG risks | validation and explicit selectors | Yes |
| Glossary | Chapter 12 | XDG cache, role cache, resolver, diagnostic | documented terms and outputs | Yes |
| User-facing operations | `README.md`, `docs/retrieval-models.md` | sources, overrides, offline recovery | operational guide | Yes |

- [x] All twelve chapter files exist and contain decision-specific content.
- [x] No prohibited ASCII-art diagrams were introduced.
- [x] ADR-039 and ADR-057 are amended separately; ADR-056 remains the approval
      authority.
- [x] Durable Arc42 chapters, `arc42.md`, `design.md`, and implementation
      agree on XDG ownership and model acquisition.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage was measured with the Gherkin runner binary included:

```text
./measure-coverage.sh
  non-E2E: 335 scenarios passed
  E2E:      61 scenarios passed
./merge-coverages.sh
  merged: 13346 / 20786 lines = 64.2067%
  branches: 0 / 0 (no Cobertura branch nodes)
```

Change-scoped coverage was also measured with
`GIVN_FEATURES=givn/changes/semantic-model-download/specs`:

```text
non-E2E: 3 scenarios passed
E2E:     6 scenarios passed
merged: 5885 / 20786 lines = 28.3123%
branches: 0 / 0
```

The low change-scoped percentage is a scope report, not a quality threshold;
the full repository report is the coverage fact used for the change.

## Coverage Gap Classification

| Gap | Classification | Evidence |
|---|---|---|
| Large real ONNX construction is not repeated in every Gherkin scenario | Legitimately hard to test | Real E5, BGE, and NLI probes ran separately; the executable suite uses a local HF-compatible twin to avoid multi-gigabyte duplicate downloads. |
| Branch metric has no valid denominator | Legitimately hard to test | Cobertura emitted `0 / 0` branch nodes. |
| Existing retrieval/lifecycle paths outside this change | Legitimately hard to test | They are exercised by the permanent suite and prior capability scenarios; adding duplicate model scenarios would violate the black-box-first boundary. |

No dead model resolver code remains, and no redundant parallel unit test was
added for a behavior already asserted through the CLI feature scenarios. The
two cache-resolution unit tests cover pure precedence/error boundaries that are
not distinct user actions.

## Semantic Review

The final blocking command was:

```text
env XDG_CACHE_HOME=/tmp/opencode/xdg-manual
    GIVN_SPEC_EMBEDDER=stub
    GIVN_SPEC_BGE=disabled
    GIVN_SPEC_RERANKER=disabled
    givn check review --change semantic-model-download
```

It passed `verify`, `verify-e2e`, integrity, overlap dispositions, and the
retrieval gate. The retrieval output reported `Indexed 394 permanent
scenarios`, `Reused 394 scenario vectors`, zero E5 token-cap candidates, zero
similarity candidates, and a passing similarity gate. No unresolved semantic
candidate or fabricated classification was introduced.

## Result

REVIEW: PASS
