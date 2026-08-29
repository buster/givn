# Semantic Review
Report schema version: 3
Change: "template-engine-conditional-rendering"
Status: PASS
Candidate count: 0
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 10
BGE recommended count: 2
BGE filtered count: 8
BGE unscored count: 0
BGE unique pair count: 10
BGE directional call count: 20
BGE model load attempts: 1
BGE runtime ms: 14330
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 10
NLI recommended count: 0
NLI filtered count: 10
NLI unscored count: 0
NLI unique pair count: 10
NLI directional call count: 20
NLI model load attempts: 1
NLI runtime ms: 9283

## BGE Filtered Candidate
- candidate_id: 0d6b481e47b5c6c4
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer enables the addons a generated section needs and the guidance gains it"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "A consumer synchronizes skills and the generated guidance omits disabled-addon sections"
- embedding_score: 0.9807
- bge_forward_raw: 2.2692
- bge_reverse_raw: 2.2176
- bge_symmetric_raw: 2.2434
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: b1b3cc32f21d82ef
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "An unbalanced conditional in a command template fails the synchronization"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "An unknown addon name in a conditional fails the synchronization and names the template"
- embedding_score: 0.9793
- nli_forward_entailment: 0.0787
- nli_forward_neutral: 0.6962
- nli_forward_contradiction: 0.2251
- nli_reverse_entailment: 0.0771
- nli_reverse_neutral: 0.6393
- nli_reverse_contradiction: 0.2836
- nli_symmetric_entailment: 0.0781
- nli_symmetric_neutral: 0.6687
- nli_symmetric_contradiction: 0.2533
- nli_admission: NLI_FILTERED

## NLI Filtered Candidate
- candidate_id: b82e085e771edd51
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "Review guidance omits the Arc42 section when the Arc42 addon is disabled"
- embedding_score: 0.9792
- nli_forward_entailment: 0.1643
- nli_forward_neutral: 0.6008
- nli_forward_contradiction: 0.2350
- nli_reverse_entailment: 0.1253
- nli_reverse_neutral: 0.7592
- nli_reverse_contradiction: 0.1155
- nli_symmetric_entailment: 0.1459
- nli_symmetric_neutral: 0.6866
- nli_symmetric_contradiction: 0.1675
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 8076a7aa4a072ad8
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "A consumer enables the addons a generated section needs and the guidance gains it"
- embedding_score: 0.9705
- bge_forward_raw: 2.9342
- bge_reverse_raw: 1.4721
- bge_symmetric_raw: 2.2031
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 77bc797563cd6f12
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "A consumer synchronizes skills and the generated guidance omits disabled-addon sections"
- embedding_score: 0.9686
- bge_forward_raw: 3.1683
- bge_reverse_raw: 1.7762
- bge_symmetric_raw: 2.4723
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d5d5f2948581dd49
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer synchronizes skills and the generated guidance omits disabled-addon sections"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "Review guidance omits the Arc42 section when the Arc42 addon is disabled"
- embedding_score: 0.9660
- bge_forward_raw: 1.6920
- bge_reverse_raw: 4.2196
- bge_symmetric_raw: 2.9558
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3b5dae4c323aaadc
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer synchronizes skills and the generated guidance omits disabled-addon sections"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled"
- embedding_score: 0.9593
- bge_forward_raw: 0.6761
- bge_reverse_raw: 3.3159
- bge_symmetric_raw: 1.9960
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 46c0f0675cb67a3c
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer synchronizes skills and the generated guidance omits disabled-addon sections"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "An unknown addon name in a conditional fails the synchronization and names the template"
- embedding_score: 0.9533
- bge_forward_raw: 1.7276
- bge_reverse_raw: 2.7122
- bge_symmetric_raw: 2.2199
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1c6c04f48c37ee0f
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer enables the addons a generated section needs and the guidance gains it"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "Review guidance omits the Arc42 section when the Arc42 addon is disabled"
- embedding_score: 0.9523
- bge_forward_raw: 1.0743
- bge_reverse_raw: 4.7952
- bge_symmetric_raw: 2.9347
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 64144b3319ffc142
- left_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- left_title: "A consumer enables the addons a generated section needs and the guidance gains it"
- right_file: "givn/changes/template-engine-conditional-rendering/specs/template-conditional-rendering/template-conditional-rendering.feature"
- right_title: "Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled"
- embedding_score: 0.9505
- bge_forward_raw: 0.9804
- bge_reverse_raw: 2.8448
- bge_symmetric_raw: 1.9126
- bge_admission: BGE_FILTERED
