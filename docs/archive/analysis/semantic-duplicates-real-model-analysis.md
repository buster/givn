# Real-Model Semantic Duplicate Analysis

> **Status: historical.** Measurement basis: a 2026-08-19 real local-model run
> on 385 permanent scenarios and 100 E5 candidates, before PR #27. The 385/100
> figures and the resulting sequential BGE/NLI measurements are retained for
> provenance only and are not the current operating-point truth. See the current
> [`semantic-duplicates-real-model-filter-report.md`](../../semantic-duplicates-real-model-filter-report.md),
> [`semantic-duplicates-e5-threshold-vs-serialization-analysis.md`](../../semantic-duplicates-e5-threshold-vs-serialization-analysis.md),
> and [`semantic-duplicates-manual-evaluation.md`](../../semantic-duplicates-manual-evaluation.md).

## Executive Summary

This report records a real-model comparison of the permanent `givn spec
duplicates` corpus. No embedding, reranker, or NLI stub was used.

The corpus contains `385` permanent scenarios. Deterministic analysis produces
`4161` structural overlaps. The semantic E5 stage produces `100` model-scored
candidates. Those `100` candidates are the exhaustive candidate matrix below.

The second-stage results are:

| Pipeline | Candidate set | Recommended | Filtered | Unscored | Final candidates |
|---|---:|---:|---:|---:|---:|
| E5-only | 100 | 100 visible | 0 | 0 | 4261 total findings |
| E5 + BGE | 100 BGE pairs | 14 | 86 | 0 | 4175 total findings |
| E5 + NLI | 100 NLI pairs | 7 | 93 | 0 | 4168 total findings |
| E5 + BGE + NLI | 100 E5, 14 NLI after BGE | 1 | 13 NLI | 0 | 4162 total findings |

The only candidate recommended by both real second-stage models is the pair
about rejecting a path-traversal identifier in `status`.

The 20 highest-impact disagreement and consensus cases are prepared for manual
classification in `semantic-duplicates-manual-classification-questionnaire.md`.

The token-budget measurement for the planned complete serialization is recorded
in `semantic-serialization-token-budget-analysis.md`.

## Reproduction

The index was rebuilt before the comparison:

```sh
env -u GIVN_SPEC_EMBEDDER \
  cargo run --quiet \
  --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  -- spec index
```

Observed result:

```text
Indexed 385 permanent scenarios
Reused 0 scenario vectors
```

The resulting index reports model `intfloat/multilingual-e5-large`, `385`
entries, and `1024` dimensions per embedding. The duplicate runs used the
same index and the same permanent corpus:

### E5-only

```sh
env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER \
  cargo run --quiet \
  --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  -- spec duplicates --format json
```

### BGE

```sh
env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_RERANKER \
  GIVN_SPEC_BGE=local \
  cargo run --quiet \
  --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  -- spec duplicates --format json
```

### NLI

```sh
env -u GIVN_SPEC_EMBEDDER -u GIVN_SPEC_BGE \
  GIVN_SPEC_RERANKER=nli \
  GIVN_SPEC_NLI_MODEL_DIR=/tmp/opencode/nli-model \
  cargo run --quiet \
  --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  -- spec duplicates --format json
```

### BGE + NLI

```sh
env -u GIVN_SPEC_EMBEDDER \
  GIVN_SPEC_BGE=local \
  GIVN_SPEC_RERANKER=nli \
  GIVN_SPEC_NLI_MODEL_DIR=/tmp/opencode/nli-model \
  cargo run --quiet \
  --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  -- spec duplicates --format json
```

The report was generated on `2026-08-19` from the semantic stack tip. The
JSON output was parsed directly; no scores were manually transcribed into the
matrix.

## Model Contracts

| Stage | Model | Threshold / rule | Directional evidence |
|---|---|---|---|
| E5 | `intfloat/multilingual-e5-large` | cosine `>= 0.9500` | one embedding similarity |
| BGE | `rozgo/bge-reranker-v2-m3` | symmetric raw logit `>= 3.0000` | forward, reverse, symmetric average |
| NLI | `MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7` | symmetric entailment `>= 0.5000` | forward, reverse, symmetric entailment/neutral/contradiction probabilities |

Deterministic exact and structural relationships are not sent to BGE or NLI.
They remain evidence findings and are included in the `4161` structural-overlap
aggregate, not in the `100` model-scored rows.

## Artifact Provenance

### E5

| Property | Value |
|---|---|
| Index | `givn/.spec-index.json` |
| Model | `intfloat/multilingual-e5-large` |
| Entries | `385` |
| Embedding dimensions | `1024` |

### BGE

| Property | Value |
|---|---|
| Repository | `rozgo/bge-reranker-v2-m3` |
| Revision | `fbd57b17b4db111a9d16813bb08b4c804fac18e9` |
| Cache | `givn/.spec-reranker-cache/` |
| `model.onnx` SHA-256 | `3af844cd2de818a95d2b5de5893a336836312c8ade03f53b286ac6beae080321` |
| `model.onnx.data` SHA-256 | `84b66c787b9b98977a16d5c993a3959210a214c98fc3466da263c949c2068945` |
| `tokenizer.json` SHA-256 | `8bf8afbfd11306bd872018c53bfdf2e160a56f8edbcf49933324404791c148d3` |
| `config.json` SHA-256 | `835fe05211a0d1535b5cd2accb6d788ad142bdc507f8a8bd7287727915d9d61f` |
| `special_tokens_map.json` SHA-256 | `8c785abebea9ae3257b61681b4e6fd8365ceafde980c21970d001e834cf10835` |
| `tokenizer_config.json` SHA-256 | `7e4c1cc848840aeccdd763458c18dd525eb0f795c992e00ebe9c28554e7db2d4` |

The BGE cache is project-local and ignored by Git. The relevant downloaded
files occupy approximately `2.2 GB`.

### NLI

| Property | Value |
|---|---|
| Model directory | `/tmp/opencode/nli-model` |
| Model | `MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7` |
| `model.onnx` SHA-256 | `141998dbc1f3f282a9432d2f6678a9b8116c9e101b2493671dd7c0fd48016626` |
| `tokenizer.json` SHA-256 | `ac9bd80d04d21b4df95917bdf7c750cbea28acc30a4462c76ca2b9c86d371863` |
| `config.json` SHA-256 | `14877ff8e4699989ef08bed8fa570a803e1caef0cc0a35cb4372856deabba958` |
| `tokenizer_config.json` SHA-256 | `fc66a38a04a7e271944043a7d09ca0d6e30be0ed3c31c24b1d8bef842fcc1ea6` |
| `special_tokens_map.json` SHA-256 | `63cd5b53789b3b5ae2c3d7aa377bcba1557ff0b41dc69ce07b94350ea32291c2` |

## Aggregate Runtime Evidence

| Run | Schema | E5 semantic | Pairs | Directional calls | Recommended | Filtered | Model runtime |
|---|---:|---:|---:|---:|---:|---:|---:|
| E5-only | 1 | 100 | - | - | 100 visible | - | - |
| BGE-only | 2 | 100 | 100 | 200 | 14 | 86 | 88.8 s |
| NLI-only | 3 | 100 | 100 | 200 | 7 | 93 | 108.3 s |
| BGE+NLI | 3 | 100 | BGE 100, NLI 14 | BGE 200, NLI 28 | 1 | BGE 86, NLI 13 | BGE 90.4 s, NLI 12.3 s |

## Pipeline Interpretation

The combined implementation is sequential:

1. E5 produces the raw semantic candidate pool.
2. BGE evaluates all `100` candidates and keeps `14`.
3. NLI evaluates those `14` BGE survivors and keeps `1`.

The standalone NLI run evaluates all `100` E5 candidates independently. Six
of its seven recommendations are rejected by BGE. Conversely, BGE recommends
thirteen candidates that NLI rejects. The final intersection is therefore one
candidate.

The matrix distinguishes these states explicitly:

- `BGE_FILTERED; NLI_NOT_SCORED`: BGE rejected the candidate before the combined NLI stage.
- `BGE_RECOMMENDED + NLI_FILTERED`: both models evaluated the candidate; NLI rejected it.
- `BGE_RECOMMENDED + NLI_RECOMMENDED`: both models accepted it.

The Combined column below is the actual sequential pipeline state. The
standalone NLI column is independent evidence for all `100` E5 candidates,
which makes the model intersection auditable even where the sequential
pipeline did not invoke NLI.

## Complete Candidate Matrix

`BGE F/R/S` contains forward raw logit, reverse raw logit, and symmetric raw
logit. `NLI F/R/S` contains the three probability values in the order
`entailment/neutral/contradiction` for forward, reverse, and symmetric scoring.
The rank is the E5 semantic rank. Every row below is a model-scored E5
candidate; the `4161` deterministic structural findings are intentionally not
model-scored.

| Nr. | E5 | BGE F/R/S | BGE | NLI F(E/N/C) / R(E/N/C) / S(E/N/C) | NLI | Combined | Pair |
|---:|---:|---|---|---|---|---|---|
|1|0.9878|1.6625/ 1.921/ 1.7917|BGE_FILTERED|0.0374/0.0378/0.9248 / 0.3209/0.3673/0.3119 / 0.1434/0.1541/0.7025|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-duplicates/spec-duplicates.feature::Exact duplicates across features are reported together <-> givn/specs/spec-duplicates/spec-duplicates.feature::Exact duplicates in one feature are reported together|
|2|0.9850|1.1661/ -1.1811/ -0.0075|BGE_FILTERED|0.0027/0.0081/0.9892 / 0.0036/0.0116/0.9849 / 0.0031/0.0097/0.9872|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature::Integrity hook fails on unedited arc42 chapter <-> givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature::Integrity hook passes on edited arc42 chapter|
|3|0.9848|1.3326/ 3.9012/ 2.6169|BGE_FILTERED|0.015/0.4584/0.5266 / 0.118/0.2024/0.6796 / 0.0445/0.3223/0.6331|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init accepts the default selection with an empty answer <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init installs only the agents the user selects|
|4|0.9845|4.5406/ 2.7871/ 3.6639|BGE_RECOMMENDED|0.3138/0.3681/0.3181 / 0.1306/0.2854/0.5841 / 0.2114/0.3385/0.4502|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Existing spec review keeps its current behavior <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review reports an advisory retrieval candidate|
|5|0.9841|2.4093/ 2.8826/ 2.6459|BGE_FILTERED|0.0525/0.4111/0.5364 / 0.0229/0.0806/0.8965 / 0.0381/0.2/0.7619|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Active and archived changes — single active auto-defaults to detail (text) <-> givn/specs/givn-status-command/givn-status-command.feature::Active and archived changes — single active auto-defaults to detail JSON|
|6|0.9840|2.1674/ 2.6184/ 2.3929|BGE_FILTERED|0.1552/0.1486/0.6962 / 0.0105/0.2088/0.7807 / 0.0423/0.1846/0.773|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::addon skills are written to every configured agent target after sync <-> givn/specs/install-extras/install-extras.feature::addon skills are written to the default agent target after sync|
|7|0.9801|2.6123/ 0.8022/ 1.7073|BGE_FILTERED|0.0224/0.0349/0.9426 / 0.021/0.1761/0.8029 / 0.0223/0.0809/0.8968|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects a missing Rust parity attestation <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer requires enforcement for any approval output|
|8|0.9788|3.7857/ 3.7856/ 3.7857|BGE_RECOMMENDED|0.13/0.8345/0.0355 / 0.2925/0.645/0.0625 / 0.1999/0.7519/0.0483|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/remove-dead-skip-list-code/remove-dead-skip-list-code.feature::givn status JSON still reports skipped artifacts after dead code removal <-> givn/specs/remove-dead-skip-list-code/remove-dead-skip-list-code.feature::givn status still reports skipped artifacts with the skip marker after dead code removal|
|9|0.9745|3.3213/ 1.8849/ 2.6031|BGE_FILTERED|0.0006/0.0032/0.9962 / 0.1201/0.3563/0.5236 / 0.0109/0.0442/0.945|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::givn init --no-addons produces no skill or command addon files <-> givn/specs/install-extras/install-extras.feature::givn init defaults to all addons enabled and generates their files|
|10|0.9742|2.0488/ 2.3909/ 2.2199|BGE_FILTERED|0.0052/0.0325/0.9623 / 0.0299/0.176/0.7941 / 0.0129/0.0786/0.9085|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Detail mode — archived change (JSON) <-> givn/specs/givn-status-command/givn-status-command.feature::Detail mode — archived change (text)|
|11|0.9737|3.5805/ 0.3138/ 1.9471|BGE_FILTERED|0.0141/0.2866/0.6993 / 0.1525/0.3567/0.4909 / 0.0488/0.3358/0.6154|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::givn addons enable generates skill and command files <-> givn/specs/install-extras/install-extras.feature::givn init defaults to all addons enabled and generates their files|
|12|0.9726|1.6105/ 1.1163/ 1.3634|BGE_FILTERED|0.0179/0.0748/0.9074 / 0.0321/0.8283/0.1396 / 0.0381/0.3958/0.5661|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks when a long scenario in the delta has no split-or-keep entry <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when a long scenario has a split-or-keep entry|
|13|0.9717|3.0633/ 2.2514/ 2.6573|BGE_FILTERED|0.0168/0.3857/0.5975 / 0.0215/0.7561/0.2223 / 0.0206/0.5848/0.3946|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/new-command/new-command.feature::Id with underscores is rejected <-> givn/specs/new-command/new-command.feature::Id with uppercase letters is rejected|
|14|0.9716|2.4164/ 0.809/ 1.6127|BGE_FILTERED|0.0855/0.267/0.6474 / 0.1156/0.4828/0.4016 / 0.1027/0.3708/0.5266|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::A change without a specs directory is rejected without changing the permanent index <-> givn/specs/spec-explore-change/spec-explore-change.feature::A missing change is rejected without changing the permanent index|
|15|0.9709|3.9668/ 3.415/ 3.6909|BGE_RECOMMENDED|0.2632/0.553/0.1838 / 0.6107/0.3222/0.0671 / 0.4292/0.4519/0.1189|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Existing spec review keeps its current behavior <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review gate keeps retrieval advisory|
|16|0.9701|2.6963/ 1.9261/ 2.3112|BGE_FILTERED|0.0688/0.2261/0.705 / 0.0115/0.128/0.8605 / 0.0288/0.1741/0.7971|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects a forged parity document <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects a missing Rust parity attestation|
|17|0.9699|1.6474/ 2.8908/ 2.2691|BGE_FILTERED|0.042/0.5577/0.4003 / 0.6394/0.1973/0.1633 / 0.2181/0.4415/0.3404|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init can select the .agents target <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init installs only the agents the user selects|
|18|0.9695|1.1625/ 2.649/ 1.9057|BGE_FILTERED|0.1453/0.4013/0.4534 / 0.9427/0.0358/0.0215 / 0.6288/0.2036/0.1676|NLI_RECOMMENDED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Skills and commands are not installed to an agent excluded from the selection <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::givn skills sync installs to the persisted selection only|
|19|0.9689|-0.3173/ -0.1814/ -0.2493|BGE_FILTERED|0.004/0.034/0.962 / 0.0052/0.0264/0.9684 / 0.0046/0.0299/0.9655|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks when a shape match involving the change's delta is undisposed <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when every shape match is disposed and reports the net-delta receipt|
|20|0.9685|1.7667/ 2.4204/ 2.0936|BGE_FILTERED|0.008/0.016/0.9759 / 0.012/0.0143/0.9737 / 0.0098/0.0151/0.975|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/enforce-post-merge-e2e-gate/enforce-post-merge-e2e-gate.feature::Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured <-> givn/specs/enforce-post-merge-e2e-gate/enforce-post-merge-e2e-gate.feature::Archive runs e2e_command when merged permanent specs contain @e2e and e2e_command is configured|
|21|0.9677|2.8532/ 3.4471/ 3.1502|BGE_RECOMMENDED|0.4772/0.4284/0.0944 / 0.0913/0.2306/0.678 / 0.269/0.405/0.326|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init accepts the default selection with an empty answer <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init can select the .agents target|
|22|0.9671|0.1667/ 0.5082/ 0.3374|BGE_FILTERED|0.0772/0.3185/0.6043 / 0.0124/0.5659/0.4217 / 0.0322/0.4421/0.5257|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::givn addons disable removes generated skill files <-> givn/specs/install-extras/install-extras.feature::givn addons enable generates skill and command files|
|23|0.9665|2.2156/ 0.6174/ 1.4165|BGE_FILTERED|0.1756/0.7393/0.0851 / 0.0819/0.7804/0.1377 / 0.1214/0.769/0.1096|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/nli-evidence/nli-evidence.feature::NLI reduces change-aware exploration candidates without changing E5 raw results <-> givn/specs/nli-evidence/nli-evidence.feature::NLI reduces permanent exploration candidates without changing E5 raw results|
|24|0.9664|3.9486/ 2.9069/ 3.4278|BGE_RECOMMENDED|0.0395/0.0278/0.9327 / 0.2314/0.138/0.6306 / 0.1034/0.067/0.8296|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/deduplicate-change-dir-detection/deduplicate-change-dir-detection.feature::Integrity check detects a placeholder COMMIT hash in tasks.md and fails <-> givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature::Integrity hook fails on placeholder commit hash|
|25|0.9664|2.7817/ 2.601/ 2.6914|BGE_FILTERED|0.0014/0.0045/0.9941 / 0.0014/0.0029/0.9957 / 0.0014/0.0036/0.995|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/enforce-post-merge-e2e-gate/enforce-post-merge-e2e-gate.feature::Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured <-> givn/specs/enforce-post-merge-e2e-gate/enforce-post-merge-e2e-gate.feature::Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured|
|26|0.9662|0.3718/ 1.2777/ 0.8248|BGE_FILTERED|0.0023/0.022/0.9757 / 0.0088/0.0247/0.9665 / 0.0045/0.0233/0.9721|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Detail mode — active change (JSON) <-> givn/specs/givn-status-command/givn-status-command.feature::Detail mode — active change with proposal only (text)|
|27|0.9657|2.1749/ 1.3315/ 1.7532|BGE_FILTERED|0.1008/0.1592/0.7401 / 0.0163/0.0441/0.9397 / 0.0422/0.0874/0.8703|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Consumer selects agent targets during interactive init <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init accepts the default selection with an empty answer|
|28|0.9648|2.8062/ 0.7177/ 1.7619|BGE_FILTERED|0.0952/0.167/0.7378 / 0.0109/0.0195/0.9696 / 0.0344/0.061/0.9045|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::givn addons enable generates skill and command files <-> givn/specs/install-extras/install-extras.feature::givn init --no-addons produces no skill or command addon files|
|29|0.9646|1.8571/ 2.392/ 2.1246|BGE_FILTERED|0.0728/0.4054/0.5218 / 0.0311/0.3008/0.6681 / 0.0482/0.3537/0.5981|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Unavailable BGE JSON preserves raw candidates and names unscored pairs <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Unavailable BGE duplicate JSON preserves the E5 fallback accounting|
|30|0.9634|0.7672/ 1.9427/ 1.3549|BGE_FILTERED|0.0056/0.9922/0.0022 / 0.0287/0.232/0.7394 / 0.0238/0.9009/0.0753|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/lint-command/lint-command.feature::A parse error is reported on stderr and exits 1 <-> givn/specs/spec-overlap-gates/spec-overlap-gates.feature::The json output reports a parse error as a machine-readable finding|
|31|0.9629|2.7013/ 0.3536/ 1.5275|BGE_FILTERED|0.0317/0.099/0.8693 / 0.0101/0.1017/0.8882 / 0.018/0.1007/0.8814|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Pair-cap unavailable blocking review retains E5 candidates <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Unavailable BGE blocking review keeps E5 candidates blocking|
|32|0.9627|-1.3881/ -0.1607/ -0.7744|BGE_FILTERED|0.0121/0.1635/0.8244 / 0.0049/0.177/0.8181 / 0.0077/0.1703/0.822|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::A DUPLIKAT classification without removal remains blocked <-> givn/specs/semantic-review/semantic-review.feature::Removing a DUPLIKAT candidate verifies the projected remediation|
|33|0.9626|4.3659/ 4.4078/ 4.3869|BGE_RECOMMENDED|0.0806/0.8402/0.0792 / 0.0453/0.081/0.8737 / 0.1034/0.4464/0.4501|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/validate-artifact-id-input/validate-artifact-id-input.feature::givn check with unknown artifact validates against manifest and provides helpful error <-> givn/specs/validate-artifact-id-input/validate-artifact-id-input.feature::givn eject instruction with unknown artifact validates against manifest and provides helpful error|
|34|0.9625|1.5757/ -0.4605/ 0.5576|BGE_FILTERED|0.04/0.1649/0.7951 / 0.076/0.2733/0.6507 / 0.0559/0.2152/0.7289|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks when a delta scenario shape-matches a permanent scenario without disposition <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when a shape match exists only between permanent scenarios|
|35|0.9615|1.0869/ 1.1377/ 1.1123|BGE_FILTERED|0.5453/0.3494/0.1053 / 0.4048/0.2904/0.3048 / 0.4856/0.3292/0.1852|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Exploration leaves permanent feature files unchanged <-> givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Unchanged permanent scenarios reuse the existing index|
|36|0.9614|-2.2565/ -0.5299/ -1.3932|BGE_FILTERED|0.0359/0.1104/0.8536 / 0.0423/0.6865/0.2713 / 0.049/0.3461/0.6049|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks with a clear message when review.md is missing <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when a shape match exists only between permanent scenarios|
|37|0.9610|2.001/ 1.2044/ 1.6027|BGE_FILTERED|0.2286/0.1411/0.6304 / 0.0013/0.0444/0.9543 / 0.0201/0.0907/0.8892|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Active and archived changes — single active auto-defaults to detail (text) <-> givn/specs/givn-status-command/givn-status-command.feature::Single active change — auto-defaults to detail view (text)|
|38|0.9606|1.987/ 0.848/ 1.4175|BGE_FILTERED|0.0633/0.2704/0.6663 / 0.0029/0.0213/0.9758 / 0.0151/0.0847/0.9001|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::A VALID-BOUNDARY score below the boundary without distinction remains blocked <-> givn/specs/semantic-review/semantic-review.feature::Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation|
|39|0.9606|0.8962/ 1.109/ 1.0026|BGE_FILTERED|0.0141/0.0287/0.9572 / 0.0672/0.166/0.7668 / 0.0322/0.0721/0.8957|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-retrieval/spec-retrieval.feature::Review reports an advisory retrieval candidate <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review suppresses a disposed retrieval candidate|
|40|0.9604|1.4025/ 0.7995/ 1.101|BGE_FILTERED|0.1463/0.7198/0.1339 / 0.0899/0.594/0.3161 / 0.1177/0.6711/0.2112|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Existing spec review keeps its current behavior <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review ignores a removed delta scenario|
|41|0.9603|4.8774/ 3.6359/ 4.2567|BGE_RECOMMENDED|0.059/0.0972/0.8438 / 0.0956/0.247/0.6574 / 0.077/0.1589/0.764|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/new-command/new-command.feature::Id beginning with a hyphen is rejected by argument parsing <-> givn/specs/new-command/new-command.feature::Id beginning with a hyphen passed after the argument separator is rejected by validation|
|42|0.9601|4.1281/ 2.0631/ 3.0956|BGE_RECOMMENDED|0.1936/0.0672/0.7392 / 0.023/0.0198/0.9572 / 0.0707/0.0386/0.8908|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Consumer selects agent targets during interactive init <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init can select the .agents target|
|43|0.9599|2.1638/ 2.2128/ 2.1883|BGE_FILTERED|0.9107/0.0861/0.0032 / 0.1273/0.8694/0.0033 / 0.5515/0.4432/0.0053|NLI_RECOMMENDED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/coverage-readme-example/coverage-readme-example.feature::Coverage-enabled project receives a complete README coverage example <-> givn/specs/readme-coverage-badge/readme-coverage-badge.feature::Coverage-enabled project is guided to place the coverage badge directly after the README title|
|44|0.9598|3.0469/ 3.4491/ 3.248|BGE_RECOMMENDED|0.0059/0.0475/0.9466 / 0.0287/0.084/0.8872 / 0.0131/0.0637/0.9232|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature::Archive blocked when merged permanent specs fail verify <-> givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature::Archive re-runs verify against merged permanent specs and passes|
|45|0.9596|0.8194/ 0.5363/ 0.6779|BGE_FILTERED|0.0972/0.3039/0.599 / 0.0195/0.0516/0.9289 / 0.0476/0.1369/0.8155|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::A VALID-BOUNDARY score exactly at the boundary remains blocked <-> givn/specs/semantic-review/semantic-review.feature::Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation|
|46|0.9595|1.2322/ -1.0887/ 0.0718|BGE_FILTERED|0.7064/0.2099/0.0837 / 0.4172/0.3271/0.2557 / 0.5707/0.2755/0.1538|NLI_RECOMMENDED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::A missing change is rejected without changing the permanent index <-> givn/specs/spec-explore-change/spec-explore-change.feature::Change exploration does not persist delta scenarios in the permanent index|
|47|0.9595|1.1409/ 2.5819/ 1.8614|BGE_FILTERED|0.1203/0.1325/0.7473 / 0.3775/0.147/0.4755 / 0.2246/0.1471/0.6283|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/preserve-docstrings-on-archive/preserve-docstrings-on-archive.feature::A modified scenario keeps its step docstring after archiving <-> givn/specs/preserve-docstrings-on-archive/preserve-docstrings-on-archive.feature::Consumer archives a modified scenario and the permanent spec keeps the docstring|
|48|0.9595|0.5318/ -1.3878/ -0.428|BGE_FILTERED|0.0109/0.095/0.8941 / 0.003/0.0915/0.9056 / 0.0057/0.0934/0.901|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::A VALID-BOUNDARY classification above the boundary remains blocked <-> givn/specs/semantic-review/semantic-review.feature::Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation|
|49|0.9591|1.5136/ 1.4124/ 1.463|BGE_FILTERED|0.1059/0.4463/0.4479 / 0.2563/0.2003/0.5434 / 0.1721/0.3124/0.5155|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::A missing change is rejected without changing the permanent index <-> givn/specs/spec-explore-change/spec-explore-change.feature::Unsafe change identifiers are rejected without changing the permanent index|
|50|0.9587|0.4776/ 1.5938/ 1.0357|BGE_FILTERED|0.0074/0.0706/0.922 / 0.0024/0.0237/0.9739 / 0.0042/0.0412/0.9546|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::E5-only exploration remains the default when second-stage evidence is disabled <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Permanent exploration exposes bounded second-stage evidence|
|51|0.9586|-0.2406/ 0.3176/ 0.0385|BGE_FILTERED|0.0028/0.41/0.5872 / 0.0043/0.0979/0.8978 / 0.0037/0.2154/0.7808|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/fix-uppercase-checkbox-parsing/fix-uppercase-checkbox-parsing.feature::Lowercase [x] checkboxes are counted as done in text mode <-> givn/specs/fix-uppercase-checkbox-parsing/fix-uppercase-checkbox-parsing.feature::Mixed-case checkboxes are both counted as done in JSON mode|
|52|0.9585|1.2652/ 0.471/ 0.8681|BGE_FILTERED|0.779/0.0803/0.1407 / 0.0109/0.0703/0.9188 / 0.1749/0.1426/0.6825|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/new-command/new-command.feature::Empty id is rejected <-> givn/specs/new-command/new-command.feature::Uninitialised project is rejected|
|53|0.9584|-1.1673/ 0.6616/ -0.2529|BGE_FILTERED|0.2014/0.7208/0.0778 / 0.0923/0.4955/0.4122 / 0.1494/0.6545/0.1961|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::addon skills are written to the default agent target after sync <-> givn/specs/install-extras/install-extras.feature::givn addons enable generates skill and command files|
|54|0.9581|1.862/ 4.7565/ 3.3092|BGE_RECOMMENDED|0.0036/0.8366/0.1598 / 0.0102/0.8429/0.147 / 0.0061/0.8406/0.1534|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/new-command/new-command.feature::Id beginning with a hyphen is rejected by argument parsing <-> givn/specs/new-command/new-command.feature::Id ending with a hyphen is rejected|
|55|0.9580|3.5355/ 4.6515/ 4.0935|BGE_RECOMMENDED|0.28/0.6339/0.0861 / 0.1413/0.54/0.3186 / 0.2095/0.6161/0.1744|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects a missing Rust parity attestation <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects an incomplete local BGE artifact set|
|56|0.9580|-1.984/ -0.5072/ -1.2456|BGE_FILTERED|0.8996/0.0795/0.021 / 0.9417/0.0554/0.0028 / 0.9255/0.0667/0.0078|NLI_RECOMMENDED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-permanent/spec-explore-permanent.feature::File exploration finds a semantically similar permanent scenario <-> givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Stdin exploration returns the same candidates as file exploration|
|57|0.9580|2.1165/ 3.3588/ 2.7377|BGE_FILTERED|0.8902/0.1017/0.008 / 0.934/0.0404/0.0256 / 0.9208/0.0647/0.0145|NLI_RECOMMENDED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/enforce-post-merge-e2e-gate/enforce-post-merge-e2e-gate.feature::Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured <-> givn/specs/enforce-post-merge-e2e-gate/enforce-post-merge-e2e-gate.feature::Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner|
|58|0.9579|-0.9797/ 2.2922/ 0.6563|BGE_FILTERED|0.0429/0.1922/0.7649 / 0.1174/0.2005/0.6821 / 0.0717/0.1984/0.7299|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::An unknown agent name in the answer is dropped with a hint listing the valid agents <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Consumer selects agent targets during interactive init|
|59|0.9578|1.952/ -0.0463/ 0.9528|BGE_FILTERED|0.0283/0.1996/0.7721 / 0.0402/0.868/0.0918 / 0.0471/0.5812/0.3717|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks when a shape match involving the change's delta is undisposed <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when a shape match exists only between permanent scenarios|
|60|0.9577|0.0007/ -0.0647/ -0.032|BGE_FILTERED|0.1099/0.485/0.4051 / 0.0324/0.1117/0.8559 / 0.0677/0.2641/0.6682|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::A VARIANTE score below the boundary without consolidation remains blocked <-> givn/specs/semantic-review/semantic-review.feature::Consolidating a VARIANTE candidate verifies the projected remediation|
|61|0.9575|2.2678/ 1.6086/ 1.9382|BGE_FILTERED|0.0146/0.0724/0.9131 / 0.0284/0.3638/0.6078 / 0.0219/0.1749/0.8031|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/extract-coverage-gate/extract-coverage-gate.feature::Archive with coverage addon enabled still rolls back specs and README on coverage failure <-> givn/specs/extract-coverage-gate/extract-coverage-gate.feature::Archive with coverage addon enabled still runs the coverage gate|
|62|0.9573|1.8134/ 0.7453/ 1.2793|BGE_FILTERED|0.1642/0.131/0.7048 / 0.6952/0.1097/0.1951 / 0.4078/0.1447/0.4475|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/fix-uppercase-checkbox-parsing/fix-uppercase-checkbox-parsing.feature::Consumer runs givn status on a change with uppercase checkboxes <-> givn/specs/fix-uppercase-checkbox-parsing/fix-uppercase-checkbox-parsing.feature::Uppercase [X] checkboxes are counted as done in text mode|
|63|0.9572|1.2709/ -0.6008/ 0.3351|BGE_FILTERED|0.374/0.3743/0.2517 / 0.2458/0.4727/0.2815 / 0.3063/0.4249/0.2689|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::A malformed delta feature is rejected without changing the permanent index <-> givn/specs/spec-explore-change/spec-explore-change.feature::Change exploration does not persist delta scenarios in the permanent index|
|64|0.9568|-1.7621/ -0.9104/ -1.3363|BGE_FILTERED|0.1161/0.3286/0.5553 / 0.0052/0.1433/0.8516 / 0.0263/0.2335/0.7402|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::Permanent exploration JSON adds source metadata while retaining legacy result fields <-> givn/specs/spec-explore-permanent/spec-explore-permanent.feature::JSON exploration reports query index and candidate fields|
|65|0.9564|0.3359/ 0.8717/ 0.6038|BGE_FILTERED|0.1231/0.1144/0.7625 / 0.0742/0.1622/0.7635 / 0.0961/0.137/0.767|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/preserve-docstrings-on-archive/preserve-docstrings-on-archive.feature::An added scenario keeps its step docstring after archiving <-> givn/specs/preserve-docstrings-on-archive/preserve-docstrings-on-archive.feature::Consumer archives a modified scenario and the permanent spec keeps the docstring|
|66|0.9563|1.6491/ 1.4862/ 1.5677|BGE_FILTERED|0.0093/0.0279/0.9628 / 0.0137/0.85/0.1364 / 0.0214/0.2918/0.6869|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Active and archived changes — single active auto-defaults to detail JSON <-> givn/specs/givn-status-command/givn-status-command.feature::Single active change — auto-defaults to detail view (text)|
|67|0.9562|-1.6061/ -1.4357/ -1.5209|BGE_FILTERED|0.0194/0.4961/0.4845 / 0.1115/0.0895/0.799 / 0.0529/0.2396/0.7075|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks when a delta scenario shape-matches a permanent scenario without disposition <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when every shape match is disposed and reports the net-delta receipt|
|68|0.9562|0.3304/ 0.7109/ 0.5206|BGE_FILTERED|0.0244/0.16/0.8156 / 0.0591/0.1773/0.7636 / 0.0381/0.1692/0.7927|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::Change-aware JSON exploration labels candidate sources <-> givn/specs/spec-explore-change/spec-explore-change.feature::Change-aware file exploration finds permanent and active change candidates|
|69|0.9561|2.06/ 1.0795/ 1.5697|BGE_FILTERED|0.1118/0.154/0.7342 / 0.0344/0.0573/0.9082 / 0.0638/0.0966/0.8396|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Non-interactive init installs skills to opencode only <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Skills and commands are not installed to an agent excluded from the selection|
|70|0.9556|1.1173/ 1.2866/ 1.202|BGE_FILTERED|0.0169/0.0563/0.9268 / 0.0769/0.1818/0.7414 / 0.0373/0.1047/0.858|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Existing spec review keeps its current behavior <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review suppresses a disposed retrieval candidate|
|71|0.9554|1.4632/ 0.5358/ 0.9995|BGE_FILTERED|0.0068/0.0116/0.9816 / 0.0064/0.025/0.9685 / 0.0066/0.017/0.9763|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects a forged parity document <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer requires enforcement for any approval output|
|72|0.9551|1.6108/ 0.3326/ 0.9717|BGE_FILTERED|0.1791/0.5795/0.2414 / 0.3861/0.4057/0.2081 / 0.2705/0.4989/0.2306|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::A change without a specs directory is rejected without changing the permanent index <-> givn/specs/spec-explore-change/spec-explore-change.feature::Unsafe change identifiers are rejected without changing the permanent index|
|73|0.9551|-0.5654/ 3.8449/ 1.6398|BGE_FILTERED|0.6456/0.2185/0.1359 / 0.2421/0.2369/0.521 / 0.4447/0.2559/0.2994|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::An unknown agent name in the answer is dropped with a hint listing the valid agents <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Skills and commands are not installed to an agent excluded from the selection|
|74|0.9549|1.7241/ 1.8023/ 1.7632|BGE_FILTERED|0.1545/0.3022/0.5433 / 0.0013/0.0169/0.9817 / 0.0176/0.0877/0.8947|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects a forged parity document <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Approval writer rejects an incomplete local BGE artifact set|
|75|0.9548|-1.937/ -0.6393/ -1.2881|BGE_FILTERED|0.0866/0.0533/0.8602 / 0.384/0.3482/0.2678 / 0.2283/0.1706/0.6011|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-explore-change/spec-explore-change.feature::Change exploration does not persist delta scenarios in the permanent index <-> givn/specs/spec-explore-permanent/spec-explore-permanent.feature::Exploration leaves permanent feature files unchanged|
|76|0.9547|-0.3178/ 1.26/ 0.4711|BGE_FILTERED|0.007/0.0135/0.9795 / 0.0157/0.0832/0.9011 / 0.0107/0.034/0.9553|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/config-overlay/config-overlay.feature::Arc42 addon expands to arc42-docs artifact when enabled <-> givn/specs/config-overlay/config-overlay.feature::Arc42 addon is idempotent when already enabled|
|77|0.9545|1.2367/ 1.3182/ 1.2775|BGE_FILTERED|0.0091/0.1905/0.8004 / 0.0158/0.0864/0.8978 / 0.0121/0.1299/0.858|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init can select the .agents target <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Non-interactive init installs skills to opencode only|
|78|0.9541|-2.7352/ -2.5955/ -2.6653|BGE_FILTERED|0.0021/0.0116/0.9863 / 0.0263/0.6301/0.3436 / 0.0111/0.1265/0.8624|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::Changing a classified endpoint before remediation makes the decision stale <-> givn/specs/semantic-review/semantic-review.feature::Changing a permanent endpoint invalidates its prior classification|
|79|0.9539|3.2733/ 3.1907/ 3.232|BGE_RECOMMENDED|0.2522/0.3421/0.4058 / 0.3072/0.1575/0.5353 / 0.285/0.2377/0.4773|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::The E5 similarity gate remains available without BGE <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::The independent E5 gate remains separate from BGE evidence|
|80|0.9538|-0.7819/ 0.7195/ -0.0312|BGE_FILTERED|0.0545/0.3397/0.6058 / 0.0145/0.0248/0.9608 / 0.0318/0.1039/0.8643|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Detail mode — active change (JSON) <-> givn/specs/givn-status-command/givn-status-command.feature::Single active change — auto-defaults to detail view (text)|
|81|0.9536|3.9635/ 3.3535/ 3.6585|BGE_RECOMMENDED|0.9944/0.004/0.0016 / 0.4038/0.4387/0.1575 / 0.9162/0.0609/0.0229|NLI_RECOMMENDED|BGE_RECOMMENDED + NLI_RECOMMENDED|givn/specs/validate-change-id-input/validate-change-id-input.feature::Consumer invokes givn status with a path traversal change id and gets a clear error <-> givn/specs/validate-change-id-input/validate-change-id-input.feature::Path traversal id is rejected by status|
|82|0.9535|-1.3491/ -1.0674/ -1.2083|BGE_FILTERED|0.004/0.0416/0.9544 / 0.0354/0.5657/0.3989 / 0.0152/0.1962/0.7886|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-retrieval/spec-retrieval.feature::Review ignores a removed delta scenario <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review reports an advisory retrieval candidate|
|83|0.9534|0.5973/ 0.5061/ 0.5517|BGE_FILTERED|0.0046/0.0069/0.9885 / 0.0462/0.3254/0.6284 / 0.0171/0.0558/0.9271|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Active and archived changes — single active auto-defaults to detail JSON <-> givn/specs/givn-status-command/givn-status-command.feature::Detail mode — archived change (JSON)|
|84|0.9533|-0.1718/ 0.9867/ 0.4075|BGE_FILTERED|0.1463/0.2752/0.5784 / 0.0215/0.1361/0.8424 / 0.0592/0.2042/0.7365|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-retrieval/spec-retrieval.feature::Review ignores a removed delta scenario <-> givn/specs/spec-retrieval/spec-retrieval.feature::Review rejects a missing change delta|
|85|0.9533|2.6462/ 1.8385/ 2.2423|BGE_FILTERED|0.2707/0.5901/0.1391 / 0.8316/0.1631/0.0053 / 0.5844/0.3821/0.0335|NLI_RECOMMENDED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::givn skills sync excludes addon skills and commands when disabled <-> givn/specs/install-extras/install-extras.feature::givn skills sync includes skill and command addons when enabled|
|86|0.9533|-2.5474/ -0.5853/ -1.5664|BGE_FILTERED|0.0255/0.216/0.7585 / 0.0407/0.3736/0.5857 / 0.0328/0.289/0.6782|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/review-disposition-gates/review-disposition-gates.feature::Review blocks with a clear message when review.md is missing <-> givn/specs/review-disposition-gates/review-disposition-gates.feature::Review passes when every shape match is disposed and reports the net-delta receipt|
|87|0.9533|-2.5074/ -0.8891/ -1.6983|BGE_FILTERED|0.009/0.0381/0.9529 / 0.2685/0.4266/0.3049 / 0.0688/0.1781/0.7531|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::addon skills are written to the default agent target after sync <-> givn/specs/install-extras/install-extras.feature::givn addons disable removes generated skill files|
|88|0.9531|-1.2501/ 0.2071/ -0.5215|BGE_FILTERED|0.4981/0.4082/0.0937 / 0.4501/0.5373/0.0126 / 0.485/0.4797/0.0352|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Missing second-stage model artifacts are explicitly unavailable <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Unavailable BGE JSON preserves raw candidates and names unscored pairs|
|89|0.9528|0.6206/ -1.4007/ -0.39|BGE_FILTERED|0.4002/0.5263/0.0736 / 0.1889/0.4648/0.3463 / 0.2959/0.5323/0.1718|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::A corrupt local BGE cache is rejected before model construction <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Missing second-stage model artifacts are explicitly unavailable|
|90|0.9526|0.3474/ -0.3661/ -0.0093|BGE_FILTERED|0.0053/0.2181/0.7766 / 0.0026/0.7287/0.2686 / 0.0043/0.464/0.5317|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/fix-uppercase-checkbox-parsing/fix-uppercase-checkbox-parsing.feature::Mixed-case checkboxes are both counted as done in JSON mode <-> givn/specs/fix-uppercase-checkbox-parsing/fix-uppercase-checkbox-parsing.feature::Uppercase [X] checkboxes are counted as done in text mode|
|91|0.9523|0.0291/ 0.4654/ 0.2473|BGE_FILTERED|0.0045/0.4373/0.5582 / 0.0551/0.6372/0.3077 / 0.0164/0.551/0.4326|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/spec-overlap-gates/spec-overlap-gates.feature::A duplicate title across two permanent spec files is a hard error <-> givn/specs/spec-overlap-gates/spec-overlap-gates.feature::The json output reports duplicate-title findings machine-readably|
|92|0.9523|1.8748/ 2.1838/ 2.0293|BGE_FILTERED|0.0261/0.1425/0.8314 / 0.0259/0.1754/0.7987 / 0.026/0.1582/0.8157|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/remove-dead-skip-list-code/remove-dead-skip-list-code.feature::Consumer runs givn status on a change with skipped artifacts and sees them correctly <-> givn/specs/remove-dead-skip-list-code/remove-dead-skip-list-code.feature::givn status still reports skipped artifacts with the skip marker after dead code removal|
|93|0.9520|4.0942/ 3.6835/ 3.8888|BGE_RECOMMENDED|0.2459/0.3089/0.4452 / 0.2466/0.1649/0.5885 / 0.2503/0.2294/0.5203|NLI_FILTERED|BGE_RECOMMENDED + NLI_FILTERED|givn/specs/validate-artifact-id-input/validate-artifact-id-input.feature::Consumer invokes givn check with an unknown artifact and gets a helpful error <-> givn/specs/validate-artifact-id-input/validate-artifact-id-input.feature::givn check with unknown artifact validates against manifest and provides helpful error|
|94|0.9518|-0.3079/ 0.3698/ 0.031|BGE_FILTERED|0.0011/0.0176/0.9813 / 0.6048/0.1711/0.2241 / 0.0476/0.0997/0.8527|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::addon skills are written to the default agent target after sync <-> givn/specs/install-extras/install-extras.feature::givn init defaults to all addons enabled and generates their files|
|95|0.9513|-1.0946/ 1.7536/ 0.3295|BGE_FILTERED|0.0432/0.8307/0.126 / 0.0801/0.1649/0.755 / 0.0798/0.5019/0.4183|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/init-agent-target-selection/init-agent-target-selection.feature::An unknown agent name in the answer is dropped with a hint listing the valid agents <-> givn/specs/init-agent-target-selection/init-agent-target-selection.feature::Interactive init installs only the agents the user selects|
|96|0.9512|0.0389/ -0.9301/ -0.4456|BGE_FILTERED|0.0138/0.1676/0.8187 / 0.006/0.7429/0.2512 / 0.0111/0.4327/0.5561|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/semantic-review/semantic-review.feature::A VARIANTE classification without consolidation remains blocked <-> givn/specs/semantic-review/semantic-review.feature::Consolidating a VARIANTE candidate verifies the projected remediation|
|97|0.9512|-0.576/ -0.1282/ -0.3521|BGE_FILTERED|0.1878/0.7217/0.0904 / 0.3873/0.5579/0.0548 / 0.2767/0.6511/0.0722|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Missing second-stage model artifacts are explicitly unavailable <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Unavailable BGE change exploration retains the E5 raw fallback|
|98|0.9511|2.223/ 2.6192/ 2.4211|BGE_FILTERED|0.1024/0.5774/0.3202 / 0.0119/0.0986/0.8895 / 0.0433/0.2955/0.6611|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Failed benchmark approval keeps advisory review on E5-only evidence <-> givn/specs/bge-reranking-evidence/bge-reranking-evidence.feature::Failed benchmark approval keeps blocking review on E5-only evidence|
|99|0.9508|-0.288/ 0.6179/ 0.1649|BGE_FILTERED|0.0998/0.7178/0.1823 / 0.1096/0.7919/0.0986 / 0.1054/0.7596/0.1351|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/givn-status-command/givn-status-command.feature::Detail mode — skipped artifacts shown with skip marker <-> givn/specs/remove-dead-skip-list-code/remove-dead-skip-list-code.feature::givn status JSON still reports skipped artifacts after dead code removal|
|100|0.9502|0.4245/ 1.2837/ 0.8541|BGE_FILTERED|0.2102/0.7083/0.0815 / 0.4029/0.4494/0.1477 / 0.3016/0.5847/0.1137|NLI_FILTERED|BGE_FILTERED; NLI_NOT_SCORED|givn/specs/install-extras/install-extras.feature::givn addons enable generates skill and command files <-> givn/specs/install-extras/install-extras.feature::givn skills sync includes skill and command addons when enabled|
