# Semantic Review
Report schema version: 3
Change: "rebuild-deterministic-spec-discovery"
Status: PASS
Candidate count: 0
Pair cache: 19 reused, 1 computed
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 20
BGE recommended count: 2
BGE filtered count: 18
BGE unscored count: 0
BGE unique pair count: 20
BGE directional call count: 40
BGE model load attempts: 1
BGE runtime ms: 5734
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 20
NLI recommended count: 0
NLI filtered count: 20
NLI unscored count: 0
NLI unique pair count: 20
NLI directional call count: 40
NLI model load attempts: 1
NLI runtime ms: 4037

## BGE Filtered Candidate
- candidate_id: c81dbfdec3abffa0
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/config-overlay/config-overlay.feature"
- left_title: "Overlay removes a leaf artifact via remove true"
- right_file: "givn/specs/support/config-overlay.feature"
- right_title: "Overlay removes a leaf artifact via remove true"
- embedding_score: 0.9847
- bge_forward_raw: 2.1862
- bge_reverse_raw: 2.9524
- bge_symmetric_raw: 2.5693
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fc4b1258bbeecf19
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Exact duplicates across features are reported together"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Exact duplicates in one feature are reported together"
- embedding_score: 0.9843
- bge_forward_raw: 2.8152
- bge_reverse_raw: 1.3854
- bge_symmetric_raw: 2.1003
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: 9ae982496b012fac
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Repeating an unchanged audit produces the same report"
- right_file: "givn/specs/spec-discovery/spec-duplicates.feature"
- right_title: "Repeating an unchanged audit produces the same report"
- embedding_score: 0.9837
- nli_forward_entailment: 0.2800
- nli_forward_neutral: 0.4898
- nli_forward_contradiction: 0.2302
- nli_reverse_entailment: 0.4548
- nli_reverse_neutral: 0.3020
- nli_reverse_contradiction: 0.2432
- nli_symmetric_entailment: 0.3649
- nli_symmetric_neutral: 0.3932
- nli_symmetric_contradiction: 0.2419
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 625e8de50da565b6
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Auditing leaves permanent specifications unchanged"
- right_file: "givn/specs/spec-discovery/spec-duplicates.feature"
- right_title: "Auditing leaves permanent specifications unchanged"
- embedding_score: 0.9816
- bge_forward_raw: 2.6487
- bge_reverse_raw: 3.1734
- bge_symmetric_raw: 2.9110
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 217333eac2667c64
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "A missing permanent specifications directory is an empty corpus"
- right_file: "givn/specs/spec-discovery/spec-duplicates.feature"
- right_title: "A missing permanent specifications directory is an empty corpus"
- embedding_score: 0.9805
- bge_forward_raw: 0.9792
- bge_reverse_raw: 0.0839
- bge_symmetric_raw: 0.5315
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bfc12b816ca9a7d8
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Exact duplicates across features are reported together"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Repeating an unchanged audit produces the same report"
- embedding_score: 0.9783
- bge_forward_raw: 2.4647
- bge_reverse_raw: 2.6482
- bge_symmetric_raw: 2.5565
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: eb91d84e3818da22
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Auditing leaves permanent specifications unchanged"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Repeating an unchanged audit produces the same report"
- embedding_score: 0.9776
- nli_forward_entailment: 0.4817
- nli_forward_neutral: 0.2901
- nli_forward_contradiction: 0.2282
- nli_reverse_entailment: 0.4011
- nli_reverse_neutral: 0.3392
- nli_reverse_contradiction: 0.2597
- nli_symmetric_entailment: 0.4410
- nli_symmetric_neutral: 0.3147
- nli_symmetric_contradiction: 0.2442
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 05ff00acaeaf13a3
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Exact duplicates in one feature are reported together"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Repeating an unchanged audit produces the same report"
- embedding_score: 0.9742
- bge_forward_raw: 1.5082
- bge_reverse_raw: 2.6518
- bge_symmetric_raw: 2.0800
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3ef22aade8651d7d
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "A missing permanent specifications directory is an empty corpus"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Archived scenarios are excluded from the audit"
- embedding_score: 0.9737
- bge_forward_raw: 3.1448
- bge_reverse_raw: 2.8471
- bge_symmetric_raw: 2.9959
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2bc9468551e635b6
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Repeating an unchanged audit produces the same report"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Same assertion shape without exact behavior is reported as a structural match"
- embedding_score: 0.9737
- bge_forward_raw: 2.6570
- bge_reverse_raw: 2.7832
- bge_symmetric_raw: 2.7201
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: df21b9147548464a
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Auditing leaves permanent specifications unchanged"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "The deterministic audit works without model downloads"
- embedding_score: 0.9733
- bge_forward_raw: 1.3740
- bge_reverse_raw: 0.5025
- bge_symmetric_raw: 0.9383
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 76a07d9e8e70c00f
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Auditing leaves permanent specifications unchanged"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Exact duplicates in one feature are reported together"
- embedding_score: 0.9725
- bge_forward_raw: 2.5165
- bge_reverse_raw: 1.2894
- bge_symmetric_raw: 1.9030
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6c4910eb76164e7d
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-explore-change/spec-explore-change.feature"
- left_title: "A proposal matching an existing capability surface recommends EXTEND"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-explore-change/spec-explore-change.feature"
- right_title: "A proposal with no matching surface recommends NEW"
- embedding_score: 0.9722
- bge_forward_raw: 0.8639
- bge_reverse_raw: 0.6130
- bge_symmetric_raw: 0.7385
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e9041c9121b326bd
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- left_title: "The JSON duplicate audit is valid JSON without progress wording"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- right_title: "The text audit keeps model wording off the report channel"
- embedding_score: 0.9713
- bge_forward_raw: 2.2345
- bge_reverse_raw: 1.7650
- bge_symmetric_raw: 1.9997
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bf486111ef309ac6
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- left_title: "The JSON duplicate audit is valid JSON without progress wording"
- right_file: "givn/specs/spec-discovery/spec-progress-output.feature"
- right_title: "The JSON duplicate audit keeps the report channel free of progress wording"
- embedding_score: 0.9708
- bge_forward_raw: 1.4590
- bge_reverse_raw: 3.0846
- bge_symmetric_raw: 2.2718
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9d8f8910eaea1a10
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- left_title: "Repeated audits emit stable output without progress lines"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- right_title: "The text audit keeps model wording off the report channel"
- embedding_score: 0.9707
- bge_forward_raw: 2.8305
- bge_reverse_raw: 2.8005
- bge_symmetric_raw: 2.8155
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 124afd09fc49aba6
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Auditing leaves permanent specifications unchanged"
- right_file: "givn/specs/spec-discovery/spec-duplicates.feature"
- right_title: "Auditing leaves the persistent retrieval index unchanged"
- embedding_score: 0.9699
- bge_forward_raw: 1.6714
- bge_reverse_raw: 1.8707
- bge_symmetric_raw: 1.7711
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 12784ec20c1f470b
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "A structural subset is reported as an overlap"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Same assertion shape without exact behavior is reported as a structural match"
- embedding_score: 0.9693
- bge_forward_raw: 2.9851
- bge_reverse_raw: 2.7420
- bge_symmetric_raw: 2.8636
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 79b80e79df3120a0
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- left_title: "Auditing leaves permanent specifications unchanged"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- right_title: "Exact duplicates across features are reported together"
- embedding_score: 0.9692
- bge_forward_raw: 2.6007
- bge_reverse_raw: 2.4085
- bge_symmetric_raw: 2.5046
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 999ced0337d7d831
- left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- left_title: "Repeated audits emit stable output without progress lines"
- right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-progress-output/spec-progress-output.feature"
- right_title: "The JSON duplicate audit is valid JSON without progress wording"
- embedding_score: 0.9692
- bge_forward_raw: 2.3848
- bge_reverse_raw: 2.3696
- bge_symmetric_raw: 2.3772
- bge_admission: BGE_FILTERED
