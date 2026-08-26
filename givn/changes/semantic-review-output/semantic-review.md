# Semantic Review
Report schema version: 3
Change: "semantic-review-output"
Status: PASS
Candidate count: 0
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 240
BGE recommended count: 1
BGE filtered count: 239
BGE unscored count: 0
BGE unique pair count: 240
BGE directional call count: 480
BGE model load attempts: 1
BGE runtime ms: 284483
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 240
NLI recommended count: 1
NLI filtered count: 239
NLI unscored count: 0
NLI unique pair count: 240
NLI directional call count: 480
NLI model load attempts: 1
NLI runtime ms: 166608

## NLI Filtered Candidate
- candidate_id: 973b3c7772393172
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "An NLI-only recommendation flags the review without BGE participation"
- embedding_score: 0.9992
- nli_forward_entailment: 0.0056
- nli_forward_neutral: 0.0297
- nli_forward_contradiction: 0.9646
- nli_reverse_entailment: 0.0026
- nli_reverse_neutral: 0.0180
- nli_reverse_contradiction: 0.9794
- nli_symmetric_entailment: 0.0038
- nli_symmetric_neutral: 0.0232
- nli_symmetric_contradiction: 0.9730
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: c1c93076a7f09cf6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Flagged recommendations persist their findings beside the change"
- embedding_score: 0.9847
- bge_forward_raw: -0.4182
- bge_reverse_raw: 0.3203
- bge_symmetric_raw: -0.0490
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9e0315453720d778
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9815
- bge_forward_raw: 0.6633
- bge_reverse_raw: -0.1276
- bge_symmetric_raw: 0.2678
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4df80f5f30abb605
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9809
- bge_forward_raw: 0.4852
- bge_reverse_raw: -0.9913
- bge_symmetric_raw: -0.2531
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c89028f680b05331
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9803
- bge_forward_raw: 1.4061
- bge_reverse_raw: 1.3268
- bge_symmetric_raw: 1.3664
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e8c0d4e08effd411
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9802
- bge_forward_raw: -0.1356
- bge_reverse_raw: 0.2682
- bge_symmetric_raw: 0.0663
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a78b396cb54889ed
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9795
- bge_forward_raw: -0.5036
- bge_reverse_raw: 0.8815
- bge_symmetric_raw: 0.1889
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 40ce8f0a5ee73a71
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9778
- bge_forward_raw: 0.2989
- bge_reverse_raw: 0.6002
- bge_symmetric_raw: 0.4495
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 265da672d623ab96
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9777
- bge_forward_raw: 0.7508
- bge_reverse_raw: 0.5397
- bge_symmetric_raw: 0.6452
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fd170472da907aec
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9775
- bge_forward_raw: 0.4509
- bge_reverse_raw: 0.7904
- bge_symmetric_raw: 0.6206
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ea8dc8e3f789bf6e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9766
- bge_forward_raw: 0.5222
- bge_reverse_raw: 0.4319
- bge_symmetric_raw: 0.4771
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fab9a83e3fb9720f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9762
- bge_forward_raw: 1.9770
- bge_reverse_raw: 2.1754
- bge_symmetric_raw: 2.0762
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e9f1efe8b5b3f425
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9762
- bge_forward_raw: 0.1482
- bge_reverse_raw: 0.0390
- bge_symmetric_raw: 0.0936
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 54c49fae4dfc4799
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9760
- bge_forward_raw: 0.4194
- bge_reverse_raw: -0.9910
- bge_symmetric_raw: -0.2858
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fe7b831cd78997a6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9749
- bge_forward_raw: 2.1120
- bge_reverse_raw: 1.4511
- bge_symmetric_raw: 1.7815
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 65d257e413893077
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9745
- bge_forward_raw: -0.3100
- bge_reverse_raw: 0.1725
- bge_symmetric_raw: -0.0687
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2ca76c01f8f753b2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9737
- bge_forward_raw: -0.0838
- bge_reverse_raw: 0.0008
- bge_symmetric_raw: -0.0415
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bed386d483ab359e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9736
- bge_forward_raw: -0.6595
- bge_reverse_raw: -0.0212
- bge_symmetric_raw: -0.3403
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b4da2a4bbfe7b6f1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9733
- bge_forward_raw: 0.1852
- bge_reverse_raw: -0.3422
- bge_symmetric_raw: -0.0785
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4231004b1dd1eafc
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9733
- bge_forward_raw: -1.2277
- bge_reverse_raw: -0.0766
- bge_symmetric_raw: -0.6522
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ff5ea8faad67860c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9731
- bge_forward_raw: -0.0282
- bge_reverse_raw: -0.5530
- bge_symmetric_raw: -0.2906
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 75348cb7a7d8a836
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9723
- bge_forward_raw: -0.0103
- bge_reverse_raw: 0.4691
- bge_symmetric_raw: 0.2294
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 19468d6591e5918c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9719
- bge_forward_raw: 0.4195
- bge_reverse_raw: -0.7620
- bge_symmetric_raw: -0.1712
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4330bb9b3346a992
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9716
- bge_forward_raw: -0.2973
- bge_reverse_raw: 0.1954
- bge_symmetric_raw: -0.0510
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d9fdc4af0075af21
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9715
- bge_forward_raw: 0.4668
- bge_reverse_raw: 0.3639
- bge_symmetric_raw: 0.4154
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1b23d26fa35b284e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9713
- bge_forward_raw: -0.3086
- bge_reverse_raw: 1.2604
- bge_symmetric_raw: 0.4759
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8eb1a272d6210d02
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "An unwritable findings location warns without masking the flagged failure"
- embedding_score: 0.9711
- bge_forward_raw: 0.5370
- bge_reverse_raw: 0.1280
- bge_symmetric_raw: 0.3325
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c1db95d5a1e4bb9c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI blocking review flags recommendations through the workflow gate"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9709
- bge_forward_raw: 0.3530
- bge_reverse_raw: 0.9132
- bge_symmetric_raw: 0.6331
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4b38c97253eeca08
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9707
- bge_forward_raw: 0.0219
- bge_reverse_raw: -0.5229
- bge_symmetric_raw: -0.2505
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b4e4c24d79fce7c7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9704
- bge_forward_raw: 2.2424
- bge_reverse_raw: 0.1513
- bge_symmetric_raw: 1.1968
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 63d18e875b5005be
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9697
- bge_forward_raw: -0.7513
- bge_reverse_raw: -1.5274
- bge_symmetric_raw: -1.1394
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: addc03ac4844eb43
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9685
- bge_forward_raw: -0.7309
- bge_reverse_raw: 0.5832
- bge_symmetric_raw: -0.0739
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c419f30cf3be9189
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9683
- bge_forward_raw: 1.3837
- bge_reverse_raw: 1.5889
- bge_symmetric_raw: 1.4863
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f7d22224c055d786
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Flagged recommendations persist their findings beside the change"
- embedding_score: 0.9681
- bge_forward_raw: 1.4618
- bge_reverse_raw: 0.0889
- bge_symmetric_raw: 0.7753
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c344699047a6e2ab
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9681
- bge_forward_raw: 0.6547
- bge_reverse_raw: 0.2656
- bge_symmetric_raw: 0.4602
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 984e26efe805ee98
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Verbose output lists every scored second-stage pair"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9680
- bge_forward_raw: 0.4158
- bge_reverse_raw: 0.2185
- bge_symmetric_raw: 0.3171
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 83a50a370284fa00
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9677
- bge_forward_raw: -1.2072
- bge_reverse_raw: -0.1523
- bge_symmetric_raw: -0.6797
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 57cc28a379992ec1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9677
- bge_forward_raw: -0.4607
- bge_reverse_raw: -1.3388
- bge_symmetric_raw: -0.8998
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 90154eb4d5b4371f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9676
- bge_forward_raw: 0.1187
- bge_reverse_raw: 0.2225
- bge_symmetric_raw: 0.1706
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 88849a336ab60bc7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9675
- bge_forward_raw: -0.8544
- bge_reverse_raw: 0.4766
- bge_symmetric_raw: -0.1889
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c16b31ebb2b0390f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9673
- bge_forward_raw: -0.4279
- bge_reverse_raw: -0.6944
- bge_symmetric_raw: -0.5611
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 71900cca327e5c62
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9670
- bge_forward_raw: 0.9136
- bge_reverse_raw: 1.2675
- bge_symmetric_raw: 1.0905
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 91bed6748b9ed8a2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9670
- bge_forward_raw: 0.1713
- bge_reverse_raw: 0.0034
- bge_symmetric_raw: 0.0873
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1197d980bef09af7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "The raw E5 list is verbose-only detail"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9669
- bge_forward_raw: -0.2115
- bge_reverse_raw: 1.0192
- bge_symmetric_raw: 0.4039
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0e1ea8897fde430a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9668
- bge_forward_raw: 0.3628
- bge_reverse_raw: 0.4044
- bge_symmetric_raw: 0.3836
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 80a25208a4957ea5
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "The raw E5 list is verbose-only detail"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9668
- bge_forward_raw: -0.7955
- bge_reverse_raw: 0.9611
- bge_symmetric_raw: 0.0828
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bbff3c2afb50b72b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "JSON counters match the text counters of the same run"
- embedding_score: 0.9667
- bge_forward_raw: -1.7385
- bge_reverse_raw: 0.2577
- bge_symmetric_raw: -0.7404
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 83d2f3d5e62fb6e5
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9666
- bge_forward_raw: 1.8946
- bge_reverse_raw: 2.0012
- bge_symmetric_raw: 1.9479
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a7aec191aa662e43
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI blocking review flags recommendations through the workflow gate"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9666
- bge_forward_raw: 0.6539
- bge_reverse_raw: 1.1206
- bge_symmetric_raw: 0.8872
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8d2c4343f940aedf
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9665
- bge_forward_raw: -0.4991
- bge_reverse_raw: 0.3906
- bge_symmetric_raw: -0.0542
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4af83b951b9c2bd3
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9662
- bge_forward_raw: -0.7355
- bge_reverse_raw: -0.0902
- bge_symmetric_raw: -0.4128
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c3f07bc0ab84bf3c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9662
- bge_forward_raw: 1.7069
- bge_reverse_raw: 0.8399
- bge_symmetric_raw: 1.2734
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3ac329a5ba97bd0a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9662
- bge_forward_raw: -0.5543
- bge_reverse_raw: 0.9221
- bge_symmetric_raw: 0.1839
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: daa7fa23a1850001
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output reports only recommended second-stage pairs"
- embedding_score: 0.9661
- bge_forward_raw: -0.6310
- bge_reverse_raw: 0.4847
- bge_symmetric_raw: -0.0732
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3442330e2dec2ead
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9661
- bge_forward_raw: 0.3930
- bge_reverse_raw: 1.2625
- bge_symmetric_raw: 0.8278
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0e0d883257563e2b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9657
- bge_forward_raw: 2.2965
- bge_reverse_raw: 2.1759
- bge_symmetric_raw: 2.2362
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 16655681f0a12447
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9652
- bge_forward_raw: 0.1536
- bge_reverse_raw: 1.0518
- bge_symmetric_raw: 0.6027
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fde92b6027a822d4
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9650
- bge_forward_raw: 0.4454
- bge_reverse_raw: 0.2238
- bge_symmetric_raw: 0.3346
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 884dd90e3d13745d
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9649
- bge_forward_raw: 0.2935
- bge_reverse_raw: 2.0803
- bge_symmetric_raw: 1.1869
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 689a876f9e1fd0a7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9645
- bge_forward_raw: -1.8158
- bge_reverse_raw: -0.4961
- bge_symmetric_raw: -1.1559
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5ef3a6ca8ddc7893
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9641
- bge_forward_raw: -0.7113
- bge_reverse_raw: 0.2876
- bge_symmetric_raw: -0.2118
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6a33cb938da644b3
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9641
- bge_forward_raw: 2.2545
- bge_reverse_raw: 2.5402
- bge_symmetric_raw: 2.3974
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ed1ab9f3c60dec3f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9639
- bge_forward_raw: 0.6057
- bge_reverse_raw: 2.3012
- bge_symmetric_raw: 1.4534
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 395f680a3b2b34fb
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9637
- bge_forward_raw: 1.6572
- bge_reverse_raw: 2.4424
- bge_symmetric_raw: 2.0498
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a2cdade3390867dc
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9636
- bge_forward_raw: 1.4266
- bge_reverse_raw: 1.3565
- bge_symmetric_raw: 1.3915
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a9cdf64af48cd8de
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9635
- bge_forward_raw: -0.0159
- bge_reverse_raw: 0.4445
- bge_symmetric_raw: 0.2143
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 79b3a02ed86ce6b2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9634
- bge_forward_raw: 2.2051
- bge_reverse_raw: 1.3405
- bge_symmetric_raw: 1.7728
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c1442fc9c37d72b1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output reports only recommended second-stage pairs"
- embedding_score: 0.9634
- bge_forward_raw: 1.0906
- bge_reverse_raw: 0.9492
- bge_symmetric_raw: 1.0199
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f2b5741ad03602f5
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output reports only recommended second-stage pairs"
- embedding_score: 0.9634
- bge_forward_raw: -0.8954
- bge_reverse_raw: -0.9391
- bge_symmetric_raw: -0.9173
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 29579e318e39f402
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9634
- bge_forward_raw: 0.3912
- bge_reverse_raw: 1.0694
- bge_symmetric_raw: 0.7303
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 66f25b576528ea4c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON verdict and stage summaries record run accounting"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9633
- bge_forward_raw: -0.5785
- bge_reverse_raw: -1.0263
- bge_symmetric_raw: -0.8024
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 09e8d6887e86023e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Successful reviews write no findings document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9632
- bge_forward_raw: 1.3667
- bge_reverse_raw: 1.1206
- bge_symmetric_raw: 1.2436
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5626819d0780db37
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Recommended pairs are ordered by model confidence before embedding score"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9631
- bge_forward_raw: 0.3633
- bge_reverse_raw: 0.6174
- bge_symmetric_raw: 0.4904
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7490e2dbae77473d
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI JSON review emits one structured document"
- embedding_score: 0.9630
- bge_forward_raw: 0.0663
- bge_reverse_raw: 0.3992
- bge_symmetric_raw: 0.2327
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e3b03241844edc99
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9629
- bge_forward_raw: 0.8057
- bge_reverse_raw: 0.7045
- bge_symmetric_raw: 0.7551
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d53c8951ecb162ee
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "An unwritable findings location warns without masking the flagged failure"
- embedding_score: 0.9628
- bge_forward_raw: 2.6960
- bge_reverse_raw: 2.0480
- bge_symmetric_raw: 2.3720
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: eb5edfa4f6e1f1a2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9627
- bge_forward_raw: 0.1244
- bge_reverse_raw: 0.3563
- bge_symmetric_raw: 0.2404
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 148baaccfc3f5c2c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9627
- bge_forward_raw: 0.8269
- bge_reverse_raw: -0.1695
- bge_symmetric_raw: 0.3287
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: dab04b4d34088e6e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9626
- bge_forward_raw: 2.0663
- bge_reverse_raw: 1.1401
- bge_symmetric_raw: 1.6032
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c18d9261e3904eec
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9626
- bge_forward_raw: 1.0530
- bge_reverse_raw: 0.3767
- bge_symmetric_raw: 0.7149
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d94f71e8d120f685
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Flagged recommendations persist their findings beside the change"
- embedding_score: 0.9625
- bge_forward_raw: -0.1487
- bge_reverse_raw: 0.3158
- bge_symmetric_raw: 0.0835
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b42652f9c7382677
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9625
- bge_forward_raw: -0.8504
- bge_reverse_raw: 0.5639
- bge_symmetric_raw: -0.1433
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8426276921b0ef8b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9622
- bge_forward_raw: -0.2490
- bge_reverse_raw: 0.8538
- bge_symmetric_raw: 0.3024
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 027fbbf0fa301286
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9622
- bge_forward_raw: 1.1372
- bge_reverse_raw: 0.2017
- bge_symmetric_raw: 0.6695
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c0772ec2a5e72fcb
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9622
- bge_forward_raw: 0.3592
- bge_reverse_raw: 0.2614
- bge_symmetric_raw: 0.3103
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d399d00ed09aaff0
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9620
- bge_forward_raw: -1.6991
- bge_reverse_raw: -0.2302
- bge_symmetric_raw: -0.9646
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 521e2a08500a02d6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9620
- bge_forward_raw: 0.0971
- bge_reverse_raw: 0.3355
- bge_symmetric_raw: 0.2163
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 231bffa6074e7862
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9617
- bge_forward_raw: -0.4880
- bge_reverse_raw: 1.1033
- bge_symmetric_raw: 0.3076
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 17c5f09a7d95132a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "An unwritable findings location warns without masking the flagged failure"
- embedding_score: 0.9616
- bge_forward_raw: 2.4241
- bge_reverse_raw: 1.7723
- bge_symmetric_raw: 2.0982
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 194494fe937173f4
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9616
- bge_forward_raw: -2.5457
- bge_reverse_raw: -1.5899
- bge_symmetric_raw: -2.0678
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f7496e08cc99831c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9613
- bge_forward_raw: -0.5122
- bge_reverse_raw: -0.0175
- bge_symmetric_raw: -0.2648
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7a2e3c8f5b5f6394
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI JSON review emits one structured document"
- embedding_score: 0.9610
- bge_forward_raw: -0.2888
- bge_reverse_raw: 0.0282
- bge_symmetric_raw: -0.1303
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b5d44878c00e6769
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9607
- bge_forward_raw: -0.2595
- bge_reverse_raw: -0.7258
- bge_symmetric_raw: -0.4926
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: df478819c5b9bcb3
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9606
- bge_forward_raw: 1.1820
- bge_reverse_raw: -0.5661
- bge_symmetric_raw: 0.3080
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a8fa684099cda9b7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9605
- bge_forward_raw: 0.7392
- bge_reverse_raw: 0.7850
- bge_symmetric_raw: 0.7621
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0071e7a6b1cd9517
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9605
- bge_forward_raw: -0.6294
- bge_reverse_raw: -0.2530
- bge_symmetric_raw: -0.4412
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1e197bb2f2e14826
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "A recommended pair from either stage is the problem signal"
- embedding_score: 0.9604
- bge_forward_raw: 1.2440
- bge_reverse_raw: -0.1230
- bge_symmetric_raw: 0.5605
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bce9ad9e5bf797b2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9603
- bge_forward_raw: 0.5903
- bge_reverse_raw: 1.5444
- bge_symmetric_raw: 1.0674
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 036b59dd5c993483
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI blocking review flags recommendations through the workflow gate"
- embedding_score: 0.9602
- bge_forward_raw: 0.8478
- bge_reverse_raw: 0.5597
- bge_symmetric_raw: 0.7038
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b087e5da16a93cfb
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9601
- bge_forward_raw: -0.2361
- bge_reverse_raw: 0.0391
- bge_symmetric_raw: -0.0985
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ce7af0f49edee459
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9601
- bge_forward_raw: 0.4753
- bge_reverse_raw: 0.2766
- bge_symmetric_raw: 0.3759
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: db2e0af5b7128d2d
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9600
- bge_forward_raw: 1.7244
- bge_reverse_raw: 2.3669
- bge_symmetric_raw: 2.0456
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 80154db077aef214
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9599
- bge_forward_raw: -0.4009
- bge_reverse_raw: -0.2917
- bge_symmetric_raw: -0.3463
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c61d2cc1e69d50e9
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI JSON review emits one structured document"
- embedding_score: 0.9598
- bge_forward_raw: -0.5322
- bge_reverse_raw: 1.5428
- bge_symmetric_raw: 0.5053
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9be0569ea75da04e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9598
- bge_forward_raw: -0.2338
- bge_reverse_raw: -0.2118
- bge_symmetric_raw: -0.2228
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 242ab5adc5c26f19
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Flagged recommendations persist their findings beside the change"
- embedding_score: 0.9597
- bge_forward_raw: 1.3585
- bge_reverse_raw: -0.3545
- bge_symmetric_raw: 0.5020
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f57395be01934017
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI blocking review flags recommendations through the workflow gate"
- embedding_score: 0.9597
- bge_forward_raw: 1.0815
- bge_reverse_raw: 1.1657
- bge_symmetric_raw: 1.1236
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8e6f2613dab4c6c6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Verbose output lists every scored second-stage pair"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9597
- bge_forward_raw: -0.8422
- bge_reverse_raw: -0.3106
- bge_symmetric_raw: -0.5764
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 984384d257b9e964
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9596
- bge_forward_raw: -1.2170
- bge_reverse_raw: -0.8802
- bge_symmetric_raw: -1.0486
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 166f60e96f4f0374
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9594
- bge_forward_raw: 0.1870
- bge_reverse_raw: 0.6612
- bge_symmetric_raw: 0.4241
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ffc92b5e42eb5037
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9594
- bge_forward_raw: 0.3400
- bge_reverse_raw: 0.3064
- bge_symmetric_raw: 0.3232
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 469b2652dce4320b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9594
- bge_forward_raw: 0.3958
- bge_reverse_raw: 0.5368
- bge_symmetric_raw: 0.4663
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6825c256ef9a7d24
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9593
- bge_forward_raw: 0.3678
- bge_reverse_raw: -1.5514
- bge_symmetric_raw: -0.5918
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2c3c33b900f77045
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON verdict and stage summaries record run accounting"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9593
- bge_forward_raw: 0.9687
- bge_reverse_raw: -0.6233
- bge_symmetric_raw: 0.1727
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 466575bfde03b2dc
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Successful reviews write no findings document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9593
- bge_forward_raw: 1.8935
- bge_reverse_raw: 1.6941
- bge_symmetric_raw: 1.7938
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8967493bfe8e3d62
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9592
- bge_forward_raw: 1.1564
- bge_reverse_raw: 1.2251
- bge_symmetric_raw: 1.1907
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1fca656f7ca74e96
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9592
- bge_forward_raw: -0.9678
- bge_reverse_raw: -0.9453
- bge_symmetric_raw: -0.9565
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c735ecc97ac52a27
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "JSON counters match the text counters of the same run"
- embedding_score: 0.9591
- bge_forward_raw: -2.1993
- bge_reverse_raw: -0.3500
- bge_symmetric_raw: -1.2747
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6871b188c3cec1e1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9590
- bge_forward_raw: -0.4472
- bge_reverse_raw: 1.1793
- bge_symmetric_raw: 0.3661
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 33153b723b0435bd
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9588
- bge_forward_raw: -0.7782
- bge_reverse_raw: 0.0850
- bge_symmetric_raw: -0.3466
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 84b28d258eb7eaaa
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9588
- bge_forward_raw: 1.6586
- bge_reverse_raw: 1.0541
- bge_symmetric_raw: 1.3563
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f2a32fdd28333df6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output reports only recommended second-stage pairs"
- embedding_score: 0.9584
- bge_forward_raw: 0.8927
- bge_reverse_raw: 1.5193
- bge_symmetric_raw: 1.2060
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5bc88f17e3d05fde
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9584
- bge_forward_raw: 1.6873
- bge_reverse_raw: 1.6181
- bge_symmetric_raw: 1.6527
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2ec89a6726eaf166
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "An NLI-only recommendation flags the review without BGE participation"
- embedding_score: 0.9583
- bge_forward_raw: 0.4367
- bge_reverse_raw: 1.7292
- bge_symmetric_raw: 1.0830
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1905fb7c28dc96f0
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9583
- bge_forward_raw: 1.9265
- bge_reverse_raw: 0.2708
- bge_symmetric_raw: 1.0986
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 458371b4bf81eb3f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9582
- bge_forward_raw: 2.6722
- bge_reverse_raw: 1.8503
- bge_symmetric_raw: 2.2612
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 41653a483a315703
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9581
- bge_forward_raw: -0.7828
- bge_reverse_raw: 0.2300
- bge_symmetric_raw: -0.2764
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5c738be469d5df5b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9581
- bge_forward_raw: 1.1486
- bge_reverse_raw: -0.9814
- bge_symmetric_raw: 0.0836
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4856c4680615e216
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9581
- bge_forward_raw: 0.5761
- bge_reverse_raw: 0.4826
- bge_symmetric_raw: 0.5293
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 14655dc0684911ad
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9577
- bge_forward_raw: 0.4452
- bge_reverse_raw: 0.4223
- bge_symmetric_raw: 0.4337
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5257434e0a78e24a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9576
- bge_forward_raw: 0.0957
- bge_reverse_raw: 0.1875
- bge_symmetric_raw: 0.1416
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6378d30841ddfdde
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI blocking review flags recommendations through the workflow gate"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9575
- bge_forward_raw: 0.0942
- bge_reverse_raw: 1.1895
- bge_symmetric_raw: 0.6418
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d9066f6d9795e324
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9575
- bge_forward_raw: 1.3490
- bge_reverse_raw: 1.8223
- bge_symmetric_raw: 1.5857
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c30b939bb5648d98
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9574
- bge_forward_raw: 1.5189
- bge_reverse_raw: 0.4173
- bge_symmetric_raw: 0.9681
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: cd0ec886706120d5
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "The truncation disclaimer appears only for a truncated listing"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9574
- bge_forward_raw: 1.0555
- bge_reverse_raw: 0.7503
- bge_symmetric_raw: 0.9029
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8b415575cd8919d7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9573
- bge_forward_raw: -0.5933
- bge_reverse_raw: 0.9106
- bge_symmetric_raw: 0.1587
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 45b4bd66a36772e1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9573
- bge_forward_raw: 0.7054
- bge_reverse_raw: -0.0272
- bge_symmetric_raw: 0.3391
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: db7e2ffd3824d860
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9573
- bge_forward_raw: 0.5456
- bge_reverse_raw: 0.3475
- bge_symmetric_raw: 0.4465
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a0f139afe6417147
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9573
- bge_forward_raw: 2.1992
- bge_reverse_raw: 1.5629
- bge_symmetric_raw: 1.8811
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a72115bf44f8666b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9572
- bge_forward_raw: 1.6834
- bge_reverse_raw: 1.6895
- bge_symmetric_raw: 1.6864
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 539ae080f5b40379
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9570
- bge_forward_raw: 1.8003
- bge_reverse_raw: 0.0877
- bge_symmetric_raw: 0.9440
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0011a963ab795d29
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9569
- bge_forward_raw: 0.3620
- bge_reverse_raw: 1.2365
- bge_symmetric_raw: 0.7993
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 17da878f1b23e02e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON verdict and stage summaries record run accounting"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9569
- bge_forward_raw: 0.0470
- bge_reverse_raw: -0.6720
- bge_symmetric_raw: -0.3125
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bd4e683deb03ee2f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9568
- bge_forward_raw: 0.4178
- bge_reverse_raw: 0.7996
- bge_symmetric_raw: 0.6087
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a484d7c046b8d79d
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output reports only recommended second-stage pairs"
- embedding_score: 0.9565
- bge_forward_raw: 1.2509
- bge_reverse_raw: -0.3937
- bge_symmetric_raw: 0.4286
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7ce6191a80cd6a73
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9565
- bge_forward_raw: 2.2552
- bge_reverse_raw: 1.7192
- bge_symmetric_raw: 1.9872
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 07290179417f7004
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9565
- bge_forward_raw: 0.9137
- bge_reverse_raw: 0.6972
- bge_symmetric_raw: 0.8054
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fc5f1d25225106b6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9564
- bge_forward_raw: 0.6732
- bge_reverse_raw: -0.0847
- bge_symmetric_raw: 0.2942
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4b9351030c23da55
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9561
- bge_forward_raw: -0.4952
- bge_reverse_raw: 0.1372
- bge_symmetric_raw: -0.1790
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: dfae6e3c6c7a9832
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9560
- bge_forward_raw: 1.0163
- bge_reverse_raw: 1.5965
- bge_symmetric_raw: 1.3064
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9f4970d66c3db1b2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9560
- bge_forward_raw: -0.3781
- bge_reverse_raw: -0.5765
- bge_symmetric_raw: -0.4773
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 684a44dfc1a4abd1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9560
- bge_forward_raw: 0.8539
- bge_reverse_raw: 0.3667
- bge_symmetric_raw: 0.6103
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 27bc813561081733
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9559
- bge_forward_raw: 0.4933
- bge_reverse_raw: 2.1368
- bge_symmetric_raw: 1.3151
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 839139576474ee3a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9558
- bge_forward_raw: 1.9433
- bge_reverse_raw: 0.9903
- bge_symmetric_raw: 1.4668
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 864b827b27dc54b4
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9558
- bge_forward_raw: 0.9691
- bge_reverse_raw: 0.5941
- bge_symmetric_raw: 0.7816
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 73566a4ed96e1b13
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9557
- bge_forward_raw: -0.2437
- bge_reverse_raw: 0.2221
- bge_symmetric_raw: -0.0108
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 083febe03135ce14
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9557
- bge_forward_raw: 1.0423
- bge_reverse_raw: 2.0364
- bge_symmetric_raw: 1.5393
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: acedf49b10665024
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9556
- bge_forward_raw: 0.4913
- bge_reverse_raw: 0.3122
- bge_symmetric_raw: 0.4018
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f54ba41fc8d1b3f9
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output reports only recommended second-stage pairs"
- embedding_score: 0.9555
- bge_forward_raw: 1.7236
- bge_reverse_raw: -0.0084
- bge_symmetric_raw: 0.8576
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a89b789d8aa087bd
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9555
- bge_forward_raw: 0.5275
- bge_reverse_raw: 0.4691
- bge_symmetric_raw: 0.4983
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ba7978c5968f6a26
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9554
- bge_forward_raw: 0.2102
- bge_reverse_raw: 0.4427
- bge_symmetric_raw: 0.3265
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c39ca0713ca75d11
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9551
- bge_forward_raw: 1.1599
- bge_reverse_raw: 0.0781
- bge_symmetric_raw: 0.6190
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5b6552dc8cda97a7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9551
- bge_forward_raw: 1.5054
- bge_reverse_raw: -0.4559
- bge_symmetric_raw: 0.5248
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ba43ae2db037b2ca
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Flagged recommendations persist their findings beside the change"
- embedding_score: 0.9550
- bge_forward_raw: 2.1184
- bge_reverse_raw: 0.2096
- bge_symmetric_raw: 1.1640
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 04b46c758b3f1e03
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9550
- bge_forward_raw: 0.6070
- bge_reverse_raw: 0.9522
- bge_symmetric_raw: 0.7796
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: daab9b50a79bb581
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "A recommended pair from either stage is the problem signal"
- embedding_score: 0.9550
- bge_forward_raw: -1.0114
- bge_reverse_raw: 0.2024
- bge_symmetric_raw: -0.4045
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c07b8e1947630e63
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Recommended pairs are ordered by model confidence before embedding score"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9550
- bge_forward_raw: 0.4609
- bge_reverse_raw: 0.7068
- bge_symmetric_raw: 0.5839
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d9b4f86aae56774e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9549
- bge_forward_raw: 2.2180
- bge_reverse_raw: 1.0992
- bge_symmetric_raw: 1.6586
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 60368f9e3381598b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9549
- bge_forward_raw: -0.1192
- bge_reverse_raw: 0.1450
- bge_symmetric_raw: 0.0129
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bf57c1bd791f391e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "JSON counters match the text counters of the same run"
- embedding_score: 0.9549
- bge_forward_raw: -0.7334
- bge_reverse_raw: -0.2162
- bge_symmetric_raw: -0.4748
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e1abfc00a9a0ef26
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9547
- bge_forward_raw: 1.6564
- bge_reverse_raw: 1.7043
- bge_symmetric_raw: 1.6804
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b5f8d99fdc99b8fb
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9547
- bge_forward_raw: -0.0789
- bge_reverse_raw: 0.1227
- bge_symmetric_raw: 0.0219
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8c394930ac28c4fd
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9547
- bge_forward_raw: 0.8625
- bge_reverse_raw: -0.0977
- bge_symmetric_raw: 0.3824
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d341bb19ccf5b632
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9546
- bge_forward_raw: 0.8056
- bge_reverse_raw: 1.1922
- bge_symmetric_raw: 0.9989
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d14cd325b0494758
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9545
- bge_forward_raw: 0.1831
- bge_reverse_raw: 0.2802
- bge_symmetric_raw: 0.2317
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ea9f69b839f101a3
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9543
- bge_forward_raw: 0.6078
- bge_reverse_raw: 1.8415
- bge_symmetric_raw: 1.2246
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 11437477fe44fedd
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9543
- bge_forward_raw: 1.0097
- bge_reverse_raw: -1.0862
- bge_symmetric_raw: -0.0382
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 65b58f8abd56e6f6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9543
- bge_forward_raw: 0.3685
- bge_reverse_raw: -0.6958
- bge_symmetric_raw: -0.1636
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d76f1440d78039d6
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9543
- bge_forward_raw: 0.0764
- bge_reverse_raw: 0.2690
- bge_symmetric_raw: 0.1727
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b03572043e4b6155
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9541
- bge_forward_raw: 2.2430
- bge_reverse_raw: 0.9651
- bge_symmetric_raw: 1.6040
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 108375e02bafb1b2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9541
- bge_forward_raw: -0.4662
- bge_reverse_raw: -0.5849
- bge_symmetric_raw: -0.5255
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 31941a3ff0c134af
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9541
- bge_forward_raw: 0.5360
- bge_reverse_raw: 0.6106
- bge_symmetric_raw: 0.5733
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 00c24c21ccbd779a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Recommended pairs are ordered by model confidence before embedding score"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9541
- bge_forward_raw: 0.4792
- bge_reverse_raw: 0.5277
- bge_symmetric_raw: 0.5034
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d1ef380fe9e9e04a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9540
- bge_forward_raw: 2.3274
- bge_reverse_raw: 1.2399
- bge_symmetric_raw: 1.7837
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: eed6029c35b3dd0a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "JSON counters match the text counters of the same run"
- embedding_score: 0.9540
- bge_forward_raw: 0.5172
- bge_reverse_raw: 0.4112
- bge_symmetric_raw: 0.4642
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4cd0f43b4a776e19
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9539
- bge_forward_raw: 1.3739
- bge_reverse_raw: 0.4339
- bge_symmetric_raw: 0.9039
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ea90f7cf69b6f26d
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9538
- bge_forward_raw: -0.1173
- bge_reverse_raw: 0.4876
- bge_symmetric_raw: 0.1851
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1caddc7deeb71c48
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9538
- bge_forward_raw: 1.5625
- bge_reverse_raw: 1.4174
- bge_symmetric_raw: 1.4900
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7021ed8c3936463e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "E5-only hits without a recommendation are not problems"
- embedding_score: 0.9537
- bge_forward_raw: 0.7568
- bge_reverse_raw: 2.3469
- bge_symmetric_raw: 1.5518
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8106d6a937f21efa
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output collapses artifact resolution chatter to one line per stage"
- embedding_score: 0.9537
- bge_forward_raw: -0.7493
- bge_reverse_raw: -0.2477
- bge_symmetric_raw: -0.4985
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: edf3c85f7d3b1863
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Retained text lines keep their established wording"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9537
- bge_forward_raw: 0.1484
- bge_reverse_raw: 1.9942
- bge_symmetric_raw: 1.0713
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8bf2109d4432f36e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "The raw E5 list is verbose-only detail"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9537
- bge_forward_raw: 0.0019
- bge_reverse_raw: 0.6642
- bge_symmetric_raw: 0.3330
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b16d68757f3d5248
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI blocking review flags recommendations through the workflow gate"
- embedding_score: 0.9536
- bge_forward_raw: 0.9701
- bge_reverse_raw: 1.2808
- bge_symmetric_raw: 1.1254
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6a8946ed3b9496c0
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI blocking review flags recommendations through the workflow gate"
- embedding_score: 0.9536
- bge_forward_raw: 0.5611
- bge_reverse_raw: -0.3221
- bge_symmetric_raw: 0.1195
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 68672536c07da2c2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Recommended pairs are ordered by model confidence before embedding score"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9535
- bge_forward_raw: -0.2070
- bge_reverse_raw: -0.1555
- bge_symmetric_raw: -0.1813
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9ebbde9cb7baeb0e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Flagged recommendations persist their findings beside the change"
- embedding_score: 0.9533
- bge_forward_raw: 1.8371
- bge_reverse_raw: 0.2579
- bge_symmetric_raw: 1.0475
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e83c9e5dbd5668a2
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI JSON review emits one structured document"
- embedding_score: 0.9533
- bge_forward_raw: -1.1920
- bge_reverse_raw: -0.5456
- bge_symmetric_raw: -0.8688
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5c1e123b2aae545f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9532
- bge_forward_raw: 0.7314
- bge_reverse_raw: 0.9081
- bge_symmetric_raw: 0.8197
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1de9a9e3bf6bf6de
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9532
- bge_forward_raw: 1.1682
- bge_reverse_raw: 0.9570
- bge_symmetric_raw: 1.0626
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d3c897b1610fdd24
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9532
- bge_forward_raw: -0.0471
- bge_reverse_raw: 0.4161
- bge_symmetric_raw: 0.1845
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1577b296d0d6b1ab
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9532
- bge_forward_raw: 0.6700
- bge_reverse_raw: 1.6245
- bge_symmetric_raw: 1.1473
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2dc43195f4af7c0e
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9531
- bge_forward_raw: 1.3151
- bge_reverse_raw: 1.5242
- bge_symmetric_raw: 1.4197
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 097d8a277fa5dbcd
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9530
- bge_forward_raw: 0.6451
- bge_reverse_raw: -1.4402
- bge_symmetric_raw: -0.3976
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0292d25bc52dd78a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Default output collapses artifact resolution chatter to one line per stage"
- embedding_score: 0.9528
- bge_forward_raw: 1.6230
- bge_reverse_raw: 1.4275
- bge_symmetric_raw: 1.5253
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: cf57fa7307e1c366
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9528
- bge_forward_raw: 1.1133
- bge_reverse_raw: 0.8142
- bge_symmetric_raw: 0.9637
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 00f4c6ec3889acf7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9527
- bge_forward_raw: 2.1226
- bge_reverse_raw: 1.9650
- bge_symmetric_raw: 2.0438
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2e45e7d81eb23da1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An NLI-only recommendation flags the review without BGE participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9526
- bge_forward_raw: 2.1411
- bge_reverse_raw: 1.1059
- bge_symmetric_raw: 1.6235
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f884330a0392fbdf
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON output is one complete review document"
- embedding_score: 0.9526
- bge_forward_raw: 0.5889
- bge_reverse_raw: 1.1156
- bge_symmetric_raw: 0.8523
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 04479fa1729744b7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9526
- bge_forward_raw: 0.0516
- bge_reverse_raw: 1.0361
- bge_symmetric_raw: 0.5438
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7f31dae5c7bbc9b4
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9526
- bge_forward_raw: 1.4153
- bge_reverse_raw: 0.8293
- bge_symmetric_raw: 1.1223
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 92b1d80cf37d2f93
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "E5-only hits without a recommendation are not problems"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9525
- bge_forward_raw: 2.1974
- bge_reverse_raw: 2.0966
- bge_symmetric_raw: 2.1470
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e0e3230cbc3d620a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9524
- bge_forward_raw: -0.0088
- bge_reverse_raw: -0.4514
- bge_symmetric_raw: -0.2301
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 39bf430ecc880074
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Recommended pairs are ordered by model confidence before embedding score"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9523
- bge_forward_raw: 0.2660
- bge_reverse_raw: -0.0240
- bge_symmetric_raw: 0.1210
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 65c5a359e5112960
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Successful reviews write no findings document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9523
- bge_forward_raw: 1.9927
- bge_reverse_raw: 2.1733
- bge_symmetric_raw: 2.0830
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: cfe2731fef0bf458
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A BGE-only recommendation flags the review without NLI participation"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9520
- bge_forward_raw: 1.4959
- bge_reverse_raw: -0.7475
- bge_symmetric_raw: 0.3742
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4f3a02e7c0947919
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON verdict and stage summaries record run accounting"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9520
- bge_forward_raw: 0.5194
- bge_reverse_raw: -0.0765
- bge_symmetric_raw: 0.2214
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 948781ae22808818
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9519
- bge_forward_raw: -0.1159
- bge_reverse_raw: -1.5164
- bge_symmetric_raw: -0.8161
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d6e9f6ad8842723d
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI persists findings beside the change after flagged recommendations"
- embedding_score: 0.9518
- bge_forward_raw: 1.0398
- bge_reverse_raw: 0.8091
- bge_symmetric_raw: 0.9245
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d1f33f2e5b4be614
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON preserves exit codes and failure messages"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9518
- bge_forward_raw: 0.8654
- bge_reverse_raw: -0.6605
- bge_symmetric_raw: 0.1025
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 471962da4e2bcaf4
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The truncation disclaimer appears only for a truncated listing"
- embedding_score: 0.9517
- bge_forward_raw: 1.0291
- bge_reverse_raw: 1.4306
- bge_symmetric_raw: 1.2299
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 393bf8ca0e63a016
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output reports only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9517
- bge_forward_raw: -0.1130
- bge_reverse_raw: 0.5566
- bge_symmetric_raw: 0.2218
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: da4a33b000128126
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9516
- bge_forward_raw: 0.2840
- bge_reverse_raw: -0.1520
- bge_symmetric_raw: 0.0660
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e695a3e4b2d3ec6c
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON verdict and stage summaries record run accounting"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9516
- bge_forward_raw: 1.4274
- bge_reverse_raw: 1.8562
- bge_symmetric_raw: 1.6418
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2307db793c2ad103
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "An unwritable findings location warns without masking the flagged failure"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON candidates record endpoints, labels, and evidence totals"
- embedding_score: 0.9515
- bge_forward_raw: 0.6388
- bge_reverse_raw: 0.9346
- bge_symmetric_raw: 0.7867
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4e09a9c9af8415dd
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Retained text lines keep their established wording"
- embedding_score: 0.9514
- bge_forward_raw: 0.7479
- bge_reverse_raw: -0.4461
- bge_symmetric_raw: 0.1509
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 89a86c89f5401c6a
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9514
- bge_forward_raw: -0.4091
- bge_reverse_raw: -0.6638
- bge_symmetric_raw: -0.5364
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 235d4c47791c1183
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Default output collapses artifact resolution chatter to one line per stage"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON verdict and stage summaries record run accounting"
- embedding_score: 0.9513
- bge_forward_raw: -1.1395
- bge_reverse_raw: 0.0584
- bge_symmetric_raw: -0.5405
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5ef8ab0a12a0fe1b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A recommended pair from either stage is the problem signal"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9508
- bge_forward_raw: -0.7183
- bge_reverse_raw: 2.0155
- bge_symmetric_raw: 0.6486
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 93e022c7e39a5cec
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9507
- bge_forward_raw: -0.1736
- bge_reverse_raw: 0.7379
- bge_symmetric_raw: 0.2822
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b0d4f12fa23d3027
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "JSON counters match the text counters of the same run"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI review surfaces only recommended second-stage pairs"
- embedding_score: 0.9507
- bge_forward_raw: -0.4325
- bge_reverse_raw: -1.1453
- bge_symmetric_raw: -0.7889
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6d2ae318a186b1a7
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "The truncation disclaimer appears only for a truncated listing"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Without participating second-stage layers the review stays advisory"
- embedding_score: 0.9507
- bge_forward_raw: 0.0646
- bge_reverse_raw: 0.3090
- bge_symmetric_raw: 0.1868
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4c80f1483e628ece
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI JSON review emits one structured document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output lists every scored second-stage pair"
- embedding_score: 0.9506
- bge_forward_raw: 0.1413
- bge_reverse_raw: -0.5493
- bge_symmetric_raw: -0.2040
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 527d2019636b9abf
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI persists findings beside the change after flagged recommendations"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "The raw E5 list is verbose-only detail"
- embedding_score: 0.9505
- bge_forward_raw: 0.6212
- bge_reverse_raw: -0.4760
- bge_symmetric_raw: 0.0726
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 88fb3775f7b82aba
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI review surfaces only recommended second-stage pairs"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Recommended pairs are ordered by model confidence before embedding score"
- embedding_score: 0.9504
- bge_forward_raw: 0.7748
- bge_reverse_raw: 0.8428
- bge_symmetric_raw: 0.8088
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2568e95060f56109
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Flagged recommendations persist their findings beside the change"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Real CLI verbose review lists all second-stage evidence"
- embedding_score: 0.9503
- bge_forward_raw: -0.5275
- bge_reverse_raw: -0.2945
- bge_symmetric_raw: -0.4110
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8682e743e0afadb3
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "A deprecated report-limit setting is accepted silently without effect"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "An unwritable findings location warns without masking the flagged failure"
- embedding_score: 0.9501
- bge_forward_raw: 0.5893
- bge_reverse_raw: 1.6446
- bge_symmetric_raw: 1.1169
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8e9717bb30151a0b
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Recommended pairs are ordered by model confidence before embedding score"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Successful reviews write no findings document"
- embedding_score: 0.9501
- bge_forward_raw: 1.5928
- bge_reverse_raw: 2.0474
- bge_symmetric_raw: 1.8201
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0ee21f6d34fd0a4f
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Real CLI verbose review lists all second-stage evidence"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Structured JSON preserves exit codes and failure messages"
- embedding_score: 0.9500
- bge_forward_raw: 0.3660
- bge_reverse_raw: 0.6814
- bge_symmetric_raw: 0.5237
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7e2564cbcbd94df8
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Structured JSON output is one complete review document"
- right_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- right_title: "Verbose output restores full artifact resolution detail"
- embedding_score: 0.9500
- bge_forward_raw: 0.5552
- bge_reverse_raw: 1.4465
- bge_symmetric_raw: 1.0009
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ade9532352f2f7f1
- left_file: "givn/changes/semantic-review-output/specs/semantic-review-output/semantic-review-output.feature"
- left_title: "Successful reviews write no findings document"
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "Advisory spec review does not create the semantic report"
- embedding_score: 0.9545
- bge_forward_raw: 0.3028
- bge_reverse_raw: -0.0554
- bge_symmetric_raw: 0.1237
- bge_admission: BGE_FILTERED
