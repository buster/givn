# Semantic Review
Report schema version: 3
Change: "run-command"
Status: PASS
Candidate count: 0
Pair cache: 0 reused, 20 computed
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 20
BGE recommended count: 0
BGE filtered count: 20
BGE unscored count: 0
BGE unique pair count: 20
BGE directional call count: 40
BGE model load attempts: 1
BGE runtime ms: 19293
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
NLI runtime ms: 11663

## BGE Filtered Candidate
- candidate_id: 553c4acce751bd63
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "Archive is blocked while the run declaration is unconfigured"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "Archive passes once the run declaration is complete"
- embedding_score: 0.9860
- bge_forward_raw: 1.4152
- bge_reverse_raw: 1.4417
- bge_symmetric_raw: 1.4284
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0e90ca39008111cc
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "The upgrade commit includes the backfilled run declaration"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "Upgrade backfills the run declaration into an existing project"
- embedding_score: 0.9843
- bge_forward_raw: 2.4088
- bge_reverse_raw: 2.8262
- bge_symmetric_raw: 2.6175
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6d79d7cc3a74ea15
- left_file: "givn/changes/run-command/specs/run-execution.feature"
- left_title: "Print mode emits the start plan without starting"
- right_file: "givn/changes/run-command/specs/run-execution.feature"
- right_title: "Print mode omits the port hand-off for a fixed-port declaration"
- embedding_score: 0.9805
- bge_forward_raw: 1.2871
- bge_reverse_raw: 1.4599
- bge_symmetric_raw: 1.3735
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5e1591d8b12749d9
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A complete server declaration validates cleanly"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server needs exactly one port source"
- embedding_score: 0.9767
- bge_forward_raw: 1.2077
- bge_reverse_raw: 1.8134
- bge_symmetric_raw: 1.5106
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4a1d103f8a879dfa
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A server needs exactly one port source"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server rejects two port sources"
- embedding_score: 0.9762
- bge_forward_raw: 1.4476
- bge_reverse_raw: 1.6083
- bge_symmetric_raw: 1.5280
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 28330109849b1411
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "The check names a damaged run managed block"
- right_file: "givn/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9746
- bge_forward_raw: -1.1376
- bge_reverse_raw: -1.7062
- bge_symmetric_raw: -1.4219
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a45e9edb6a8397c4
- left_file: "givn/changes/run-command/specs/run-execution.feature"
- left_title: "A failing readiness response is retried until ready"
- right_file: "givn/changes/run-command/specs/run-execution.feature"
- right_title: "Any success response satisfies the readiness probe"
- embedding_score: 0.9743
- bge_forward_raw: 2.2455
- bge_reverse_raw: 3.0148
- bge_symmetric_raw: 2.6301
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 992afdc6a6e93053
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A repeated upgrade creates no additional commit"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "The upgrade commit includes the backfilled run declaration"
- embedding_score: 0.9742
- bge_forward_raw: 1.8896
- bge_reverse_raw: 1.2323
- bge_symmetric_raw: 1.5609
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 026d0b65fcc06d01
- left_file: "givn/changes/run-command/specs/run-execution.feature"
- left_title: "A failing readiness response is retried until ready"
- right_file: "givn/changes/run-command/specs/run-execution.feature"
- right_title: "A port that is not listening yet is retried until ready"
- embedding_score: 0.9736
- bge_forward_raw: 2.1142
- bge_reverse_raw: 2.9987
- bge_symmetric_raw: 2.5564
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: cce05e02fa152c72
- left_file: "givn/changes/run-command/specs/run-execution.feature"
- left_title: "Print mode emits the start plan without starting"
- right_file: "givn/changes/run-command/specs/run-execution.feature"
- right_title: "Print mode for a library resolves the declared start command"
- embedding_score: 0.9726
- bge_forward_raw: 1.5250
- bge_reverse_raw: 1.4660
- bge_symmetric_raw: 1.4955
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5ccc3302ae58af52
- left_file: "givn/changes/run-command/specs/run-execution.feature"
- left_title: "A caller port without a port hand-off is refused"
- right_file: "givn/changes/run-command/specs/run-execution.feature"
- right_title: "A fixed declared port is used without a port hand-off"
- embedding_score: 0.9725
- bge_forward_raw: 2.3198
- bge_reverse_raw: 1.1729
- bge_symmetric_raw: 1.7463
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2b4f2e189b86aae8
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A complete server declaration validates cleanly"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A zero readiness timeout is rejected"
- embedding_score: 0.9717
- bge_forward_raw: 1.0241
- bge_reverse_raw: 0.4902
- bge_symmetric_raw: 0.7571
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e4598d13299caf28
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A relative readiness path is rejected"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server without a readiness path is incomplete"
- embedding_score: 0.9715
- bge_forward_raw: 1.2219
- bge_reverse_raw: 2.5019
- bge_symmetric_raw: 1.8619
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6b51b34cd0c65ff9
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A complete server declaration validates cleanly"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "The README renders a complete server declaration"
- embedding_score: 0.9706
- bge_forward_raw: 1.9931
- bge_reverse_raw: 1.4002
- bge_symmetric_raw: 1.6966
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 12583a08241b451f
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A server needs exactly one port source"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server without a readiness path is incomplete"
- embedding_score: 0.9694
- bge_forward_raw: 1.4767
- bge_reverse_raw: 2.0076
- bge_symmetric_raw: 1.7422
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 981382114040b674
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "The README renders a library product as nothing to start"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "The README renders a library's declared start command"
- embedding_score: 0.9688
- bge_forward_raw: 3.0013
- bge_reverse_raw: 2.1123
- bge_symmetric_raw: 2.5568
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f7fa6799b484f94b
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A relative readiness path is rejected"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server needs exactly one port source"
- embedding_score: 0.9680
- bge_forward_raw: 1.3414
- bge_reverse_raw: 1.3956
- bge_symmetric_raw: 1.3685
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7684eff9f8dc97c3
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A server rejects two port sources"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server without a readiness path is incomplete"
- embedding_score: 0.9677
- bge_forward_raw: 0.9932
- bge_reverse_raw: 1.6487
- bge_symmetric_raw: 1.3209
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 443af7736578d8b9
- left_file: "givn/changes/run-command/specs/run-execution.feature"
- left_title: "A readiness timeout abandons the start"
- right_file: "givn/changes/run-command/specs/run-execution.feature"
- right_title: "An early product exit is forwarded before readiness"
- embedding_score: 0.9677
- bge_forward_raw: 1.1278
- bge_reverse_raw: 1.7138
- bge_symmetric_raw: 1.4208
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 40b0505c2b48a26e
- left_file: "givn/changes/run-command/specs/run-command.feature"
- left_title: "A complete server declaration validates cleanly"
- right_file: "givn/changes/run-command/specs/run-command.feature"
- right_title: "A server without a readiness path is incomplete"
- embedding_score: 0.9670
- bge_forward_raw: 0.5767
- bge_reverse_raw: 2.1762
- bge_symmetric_raw: 1.3765
- bge_admission: BGE_FILTERED
