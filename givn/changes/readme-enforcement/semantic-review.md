# Semantic Review
Report schema version: 3
Change: "readme-enforcement"
Status: PASS
Candidate count: 0
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 103
BGE recommended count: 0
BGE filtered count: 103
BGE unscored count: 0
BGE unique pair count: 103
BGE directional call count: 206
BGE model load attempts: 1
BGE runtime ms: 90480
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 103
NLI recommended count: 5
NLI filtered count: 98
NLI unscored count: 0
NLI unique pair count: 103
NLI directional call count: 206
NLI model load attempts: 1
NLI runtime ms: 53690

## BGE Filtered Candidate
- candidate_id: 5b1f2336f182a634
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unmanaged README blocks the archive"
- embedding_score: 0.9939
- bge_forward_raw: 1.2417
- bge_reverse_raw: 0.9746
- bge_symmetric_raw: 1.1082
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ea75a0e2593a7946
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9932
- bge_forward_raw: 1.5196
- bge_reverse_raw: 2.3934
- bge_symmetric_raw: 1.9565
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 08ae9c2cf276909f
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9875
- bge_forward_raw: 1.4784
- bge_reverse_raw: 1.4825
- bge_symmetric_raw: 1.4804
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e66cef1e1d8f0c79
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unknown impact value is refused"
- embedding_score: 0.9874
- bge_forward_raw: 1.0552
- bge_reverse_raw: 1.1240
- bge_symmetric_raw: 1.0896
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 451f2a21cca05b34
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9855
- bge_forward_raw: 0.8105
- bge_reverse_raw: 2.0350
- bge_symmetric_raw: 1.4227
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2dbf381a0c59fc4f
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9855
- bge_forward_raw: 0.8344
- bge_reverse_raw: 1.3269
- bge_symmetric_raw: 1.0806
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 718de8105bc8c221
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A missing README blocks the archive"
- embedding_score: 0.9852
- bge_forward_raw: 0.0784
- bge_reverse_raw: 0.4297
- bge_symmetric_raw: 0.2540
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 67c8c404e85e039d
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Missing impact decision refuses the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9846
- bge_forward_raw: 0.8687
- bge_reverse_raw: 0.4904
- bge_symmetric_raw: 0.6795
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d6161db52bf2c0d8
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9840
- bge_forward_raw: 1.5688
- bge_reverse_raw: 1.1350
- bge_symmetric_raw: 1.3519
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b7803e21f154ef83
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unmanaged README blocks the archive"
- embedding_score: 0.9820
- bge_forward_raw: 0.3910
- bge_reverse_raw: 0.3097
- bge_symmetric_raw: 0.3503
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7b9cff6d4173d01a
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unmanaged README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9820
- bge_forward_raw: 1.0356
- bge_reverse_raw: 1.4516
- bge_symmetric_raw: 1.2436
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b72943d9712d0821
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9767
- bge_forward_raw: 0.4715
- bge_reverse_raw: 0.3064
- bge_symmetric_raw: 0.3890
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 272ddc703d8f4223
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Archiving refreshes the development block"
- embedding_score: 0.9754
- bge_forward_raw: 0.5266
- bge_reverse_raw: 0.6314
- bge_symmetric_raw: 0.5790
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 690477042be51225
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unmanaged README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Archiving refreshes the development block"
- embedding_score: 0.9744
- bge_forward_raw: 0.4637
- bge_reverse_raw: 0.6717
- bge_symmetric_raw: 0.5677
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3887b7d23e78b672
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9738
- bge_forward_raw: 0.2480
- bge_reverse_raw: 1.0747
- bge_symmetric_raw: 0.6614
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 882e551bc5a2363d
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions stay silent on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9734
- bge_forward_raw: 1.6366
- bge_reverse_raw: 0.0844
- bge_symmetric_raw: 0.8605
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1b19fa61f35cf6e2
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check passes on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9733
- bge_forward_raw: 1.1446
- bge_reverse_raw: 1.4329
- bge_symmetric_raw: 1.2888
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 30dc2e901c444887
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Open placeholders block the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9727
- bge_forward_raw: 0.3450
- bge_reverse_raw: 1.0422
- bge_symmetric_raw: 0.6936
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7dd9137a027c7ed0
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check names a damaged managed block"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9714
- bge_forward_raw: 1.1267
- bge_reverse_raw: 0.7316
- bge_symmetric_raw: 0.9292
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b83a6e605323fa81
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Archiving refreshes the development block"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9711
- bge_forward_raw: 0.0876
- bge_reverse_raw: 0.2306
- bge_symmetric_raw: 0.1591
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 020cbd61ac6a74f6
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Archiving refreshes the development block"
- embedding_score: 0.9708
- bge_forward_raw: 0.5385
- bge_reverse_raw: 0.8213
- bge_symmetric_raw: 0.6799
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d8b0bdb53fb7c606
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A recorded impact decision passes the review"
- embedding_score: 0.9696
- bge_forward_raw: 0.0879
- bge_reverse_raw: 0.4544
- bge_symmetric_raw: 0.2711
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: cc5fcce2ed375ae6
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9686
- bge_forward_raw: 1.0385
- bge_reverse_raw: 1.5868
- bge_symmetric_raw: 1.3126
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 26720840cd2b3fa5
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions surface an unfinished README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9678
- bge_forward_raw: 2.2708
- bge_reverse_raw: 0.8566
- bge_symmetric_raw: 1.5637
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bdd5d75e50839026
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions surface an unfinished README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Scaffolding marks the unfilled sections"
- embedding_score: 0.9666
- bge_forward_raw: 0.5849
- bge_reverse_raw: -0.1591
- bge_symmetric_raw: 0.2129
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4bf3af0ad69fe073
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions stay silent on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9664
- bge_forward_raw: 1.7876
- bge_reverse_raw: 1.0801
- bge_symmetric_raw: 1.4339
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 958bc0534daa830c
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9655
- bge_forward_raw: -0.2928
- bge_reverse_raw: 0.6763
- bge_symmetric_raw: 0.1917
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 73ef4982d4c69cbf
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9654
- bge_forward_raw: 1.7500
- bge_reverse_raw: 2.0418
- bge_symmetric_raw: 1.8959
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fff48ac1a16fc107
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Scaffolding marks the unfilled sections"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9648
- bge_forward_raw: 0.6437
- bge_reverse_raw: 0.5435
- bge_symmetric_raw: 0.5936
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7c761dc7b9ac5799
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unknown impact value is refused"
- embedding_score: 0.9645
- bge_forward_raw: 0.4038
- bge_reverse_raw: 0.3568
- bge_symmetric_raw: 0.3803
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c11d72b83ddffb1f
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Missing impact decision refuses the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9640
- bge_forward_raw: 1.5127
- bge_reverse_raw: 1.1941
- bge_symmetric_raw: 1.3534
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2c51808392581eee
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9635
- bge_forward_raw: 0.8158
- bge_reverse_raw: 0.6191
- bge_symmetric_raw: 0.7175
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 86f00b6e0298404e
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A recorded impact decision passes the review"
- embedding_score: 0.9633
- bge_forward_raw: -0.9635
- bge_reverse_raw: -1.0480
- bge_symmetric_raw: -1.0058
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d92c4d881600f7ae
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9632
- bge_forward_raw: -0.2217
- bge_reverse_raw: 1.2232
- bge_symmetric_raw: 0.5007
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8d6a20cef761faf9
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A recorded impact decision passes the review"
- embedding_score: 0.9632
- bge_forward_raw: -1.0344
- bge_reverse_raw: -0.7773
- bge_symmetric_raw: -0.9059
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7f3993fb8f4f8976
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unknown impact value is refused"
- embedding_score: 0.9628
- bge_forward_raw: 0.5535
- bge_reverse_raw: 0.1947
- bge_symmetric_raw: 0.3741
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e9626df444862096
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9626
- bge_forward_raw: 1.6945
- bge_reverse_raw: 0.8965
- bge_symmetric_raw: 1.2955
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 63da93a585e757dc
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check names every open placeholder"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9625
- bge_forward_raw: -0.2071
- bge_reverse_raw: 0.3362
- bge_symmetric_raw: 0.0645
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 888235a27d0f74d4
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unmanaged README blocks the archive"
- embedding_score: 0.9621
- bge_forward_raw: 0.2023
- bge_reverse_raw: -0.0818
- bge_symmetric_raw: 0.0602
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: da6282afc583222f
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9619
- bge_forward_raw: 0.3513
- bge_reverse_raw: 0.8555
- bge_symmetric_raw: 0.6034
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e845b86d7bf61343
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Instructions stay silent on a complete README"
- embedding_score: 0.9618
- bge_forward_raw: 0.8706
- bge_reverse_raw: 0.6873
- bge_symmetric_raw: 0.7789
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 58c7c05090949ab6
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A disabled addon needs no impact decision"
- embedding_score: 0.9617
- bge_forward_raw: -0.8525
- bge_reverse_raw: -0.5846
- bge_symmetric_raw: -0.7186
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f9099945b6066bb4
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check names a damaged managed block"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9615
- bge_forward_raw: 1.2840
- bge_reverse_raw: 0.0483
- bge_symmetric_raw: 0.6662
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 38f9ec3fad05f294
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9612
- bge_forward_raw: 0.7885
- bge_reverse_raw: 1.4176
- bge_symmetric_raw: 1.1030
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1d79ae4bae0646e9
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A missing README blocks the archive"
- embedding_score: 0.9610
- bge_forward_raw: -0.4380
- bge_reverse_raw: -0.5756
- bge_symmetric_raw: -0.5068
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 931ccfa9f4060f5b
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unmanaged README blocks the archive"
- embedding_score: 0.9610
- bge_forward_raw: 0.2323
- bge_reverse_raw: -0.4323
- bge_symmetric_raw: -0.1000
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: aaa2d5dc440f827c
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions stay silent on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Instructions surface an unfinished README"
- embedding_score: 0.9609
- bge_forward_raw: 0.5088
- bge_reverse_raw: 0.5519
- bge_symmetric_raw: 0.5304
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3c3e57914cc842be
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check passes on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9608
- bge_forward_raw: 0.6009
- bge_reverse_raw: 1.6185
- bge_symmetric_raw: 1.1097
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 289807995d678739
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A legacy README is reported for migration"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A marker-less README is unmanaged"
- embedding_score: 0.9607
- bge_forward_raw: 0.5167
- bge_reverse_raw: -0.0914
- bge_symmetric_raw: 0.2127
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3882bf5a0c5ea643
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A legacy README is reported for migration"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9605
- bge_forward_raw: 0.9219
- bge_reverse_raw: 1.0321
- bge_symmetric_raw: 0.9770
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: af6466ba43a834fb
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A legacy README is reported for migration"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9600
- bge_forward_raw: 0.5310
- bge_reverse_raw: 0.5640
- bge_symmetric_raw: 0.5475
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 37d60ff6e706fa71
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unknown impact value is refused"
- embedding_score: 0.9599
- bge_forward_raw: -0.8768
- bge_reverse_raw: 0.0904
- bge_symmetric_raw: -0.3932
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 83587488e00a974b
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions surface an unfinished README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9599
- bge_forward_raw: 1.6885
- bge_reverse_raw: 1.7543
- bge_symmetric_raw: 1.7214
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 78672b23ab1a01c1
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unmanaged README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9598
- bge_forward_raw: -0.2096
- bge_reverse_raw: 1.0905
- bge_symmetric_raw: 0.4405
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 97f5a3f5ac22bb43
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Instructions surface an unfinished README"
- embedding_score: 0.9597
- bge_forward_raw: 0.6223
- bge_reverse_raw: 0.8358
- bge_symmetric_raw: 0.7290
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5537aa0d14d5cd0d
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A legacy README is reported for migration"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9585
- bge_forward_raw: 1.2871
- bge_reverse_raw: 1.1385
- bge_symmetric_raw: 1.2128
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4a30c2929e48291c
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Open placeholders block the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Scaffolding marks the unfilled sections"
- embedding_score: 0.9585
- bge_forward_raw: 0.0339
- bge_reverse_raw: 0.0928
- bge_symmetric_raw: 0.0634
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fbced72950749317
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9584
- bge_forward_raw: 0.3147
- bge_reverse_raw: 0.8220
- bge_symmetric_raw: 0.5683
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1dc18e391aa1ba59
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9584
- bge_forward_raw: 1.1294
- bge_reverse_raw: 0.1039
- bge_symmetric_raw: 0.6167
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1156914d3473a5cc
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A legacy README is reported for migration"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Instructions surface an unfinished README"
- embedding_score: 0.9578
- bge_forward_raw: 0.7845
- bge_reverse_raw: 1.1893
- bge_symmetric_raw: 0.9869
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 94a2e62e30b49589
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Missing impact decision refuses the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9576
- bge_forward_raw: 1.0805
- bge_reverse_raw: 0.2278
- bge_symmetric_raw: 0.6542
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4aee58fccb2f3521
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Scaffolding marks the unfilled sections"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9576
- bge_forward_raw: 0.8179
- bge_reverse_raw: 1.4575
- bge_symmetric_raw: 1.1377
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2ae7372a23c7e8f7
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9575
- bge_forward_raw: -0.3026
- bge_reverse_raw: 1.1254
- bge_symmetric_raw: 0.4114
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1af77f9c925d1078
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9575
- bge_forward_raw: 0.9101
- bge_reverse_raw: 0.1724
- bge_symmetric_raw: 0.5413
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 428af3df77a82dff
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Open placeholders block the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9575
- bge_forward_raw: -0.3062
- bge_reverse_raw: 0.6447
- bge_symmetric_raw: 0.1693
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ecfc339c598e30fb
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unmanaged README blocks the archive"
- embedding_score: 0.9564
- bge_forward_raw: 0.3070
- bge_reverse_raw: 0.0003
- bge_symmetric_raw: 0.1536
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b5b215c8912154b6
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unmanaged README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9560
- bge_forward_raw: 0.2949
- bge_reverse_raw: 1.1297
- bge_symmetric_raw: 0.7123
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a39353fed3db47f3
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9555
- bge_forward_raw: 0.4261
- bge_reverse_raw: 0.6816
- bge_symmetric_raw: 0.5539
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d86ad90900351d28
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check names every open placeholder"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9554
- bge_forward_raw: -0.3370
- bge_reverse_raw: 0.5377
- bge_symmetric_raw: 0.1004
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ecba99934934ebcc
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9544
- bge_forward_raw: 0.2559
- bge_reverse_raw: 0.6164
- bge_symmetric_raw: 0.4362
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 28da8f232bcb1774
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9543
- bge_forward_raw: 0.7280
- bge_reverse_raw: -0.0248
- bge_symmetric_raw: 0.3516
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: dd81007e86786836
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A marker-less README is unmanaged"
- embedding_score: 0.9540
- bge_forward_raw: -0.2501
- bge_reverse_raw: -0.1779
- bge_symmetric_raw: -0.2140
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 35d2e0bc566260f9
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A disabled addon needs no impact decision"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9539
- bge_forward_raw: 0.9035
- bge_reverse_raw: 0.3056
- bge_symmetric_raw: 0.6046
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 98113973d384e397
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9538
- bge_forward_raw: 0.6833
- bge_reverse_raw: -0.3862
- bge_symmetric_raw: 0.1486
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 77a502b01d0e7a8e
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Archiving refreshes the development block"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9538
- bge_forward_raw: -1.4265
- bge_reverse_raw: -1.0652
- bge_symmetric_raw: -1.2458
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 026ca734fedb7ea3
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unmanaged README blocks the archive"
- embedding_score: 0.9537
- bge_forward_raw: 0.4192
- bge_reverse_raw: 0.7045
- bge_symmetric_raw: 0.5619
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1a842bb44ff5a191
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "An unknown impact value is refused"
- embedding_score: 0.9535
- bge_forward_raw: -0.4996
- bge_reverse_raw: -0.2368
- bge_symmetric_raw: -0.3682
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bcdc58555881316e
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A recorded impact decision passes the review"
- embedding_score: 0.9534
- bge_forward_raw: -0.5875
- bge_reverse_raw: -0.5700
- bge_symmetric_raw: -0.5787
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: fc442223ade4a119
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions surface an unfinished README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9532
- bge_forward_raw: 0.9208
- bge_reverse_raw: 0.8956
- bge_symmetric_raw: 0.9082
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6247dfc13169dace
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Missing impact decision refuses the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Open placeholders block the archive"
- embedding_score: 0.9531
- bge_forward_raw: 0.9303
- bge_reverse_raw: -0.3762
- bge_symmetric_raw: 0.2770
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bda900cbd37d04b7
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9530
- bge_forward_raw: -0.1057
- bge_reverse_raw: 0.4565
- bge_symmetric_raw: 0.1754
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bcc03db83b57cee1
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Scaffolding marks the unfilled sections"
- embedding_score: 0.9529
- bge_forward_raw: -0.1801
- bge_reverse_raw: 0.3589
- bge_symmetric_raw: 0.0894
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d94f8f7c30d07117
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Missing impact decision refuses the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9529
- bge_forward_raw: 0.1933
- bge_reverse_raw: 0.4543
- bge_symmetric_raw: 0.3238
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c7d84df02cf6f0ca
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9528
- bge_forward_raw: 0.7339
- bge_reverse_raw: 0.6428
- bge_symmetric_raw: 0.6884
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9c347a96afd4f44e
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9524
- bge_forward_raw: 0.7587
- bge_reverse_raw: 0.9635
- bge_symmetric_raw: 0.8611
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8b0343b316ec6aec
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9518
- bge_forward_raw: 0.5211
- bge_reverse_raw: 1.1116
- bge_symmetric_raw: 0.8164
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bacdffb8d1a2b9d3
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9518
- bge_forward_raw: 0.8196
- bge_reverse_raw: 0.9799
- bge_symmetric_raw: 0.8997
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 19bd6968b60aa1dc
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "The check names every open placeholder"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9515
- bge_forward_raw: 0.3165
- bge_reverse_raw: 0.8468
- bge_symmetric_raw: 0.5817
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 37e40e01cecb0301
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A legacy README is reported for migration"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9514
- bge_forward_raw: 0.5682
- bge_reverse_raw: 0.5946
- bge_symmetric_raw: 0.5814
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3d7eb70ba5030e5e
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Archiving refreshes the development block"
- embedding_score: 0.9513
- bge_forward_raw: -0.4939
- bge_reverse_raw: -0.2585
- bge_symmetric_raw: -0.3762
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0b62cb859caed711
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions stay silent on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9512
- bge_forward_raw: 0.8793
- bge_reverse_raw: 0.6733
- bge_symmetric_raw: 0.7763
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 315f68c1ee436993
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions surface an unfinished README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9512
- bge_forward_raw: 0.9460
- bge_reverse_raw: 1.1521
- bge_symmetric_raw: 1.0491
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0b44b572be551a4c
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Scaffolding marks the unfilled sections"
- embedding_score: 0.9510
- bge_forward_raw: 1.1091
- bge_reverse_raw: 1.2607
- bge_symmetric_raw: 1.1849
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 56ab094402b034d6
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Instructions stay silent on a complete README"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Missing impact decision refuses the review"
- embedding_score: 0.9508
- bge_forward_raw: -0.1343
- bge_reverse_raw: 0.8349
- bge_symmetric_raw: 0.3503
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 5f4eeffcf128b594
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A recorded impact decision passes the review"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9507
- bge_forward_raw: -0.0539
- bge_reverse_raw: 0.5603
- bge_symmetric_raw: 0.2532
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8236bdb49ed19eb5
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Scaffolding marks the unfilled sections"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names a damaged managed block"
- embedding_score: 0.9507
- bge_forward_raw: 1.6313
- bge_reverse_raw: 1.3688
- bge_symmetric_raw: 1.5000
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7f10c9eafdfecbec
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check names every open placeholder"
- embedding_score: 0.9506
- bge_forward_raw: -0.0718
- bge_reverse_raw: 0.2205
- bge_symmetric_raw: 0.0744
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e3e9bb35558c7331
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "An unknown impact value is refused"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9506
- bge_forward_raw: 0.1291
- bge_reverse_raw: 0.8451
- bge_symmetric_raw: 0.4871
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d68bf5d562f223c1
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "A missing README blocks the archive"
- embedding_score: 0.9505
- bge_forward_raw: -0.0228
- bge_reverse_raw: 0.0947
- bge_symmetric_raw: 0.0359
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: d8b6b6aff9210cce
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A marker-less README is unmanaged"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Updated impact decision passes the review"
- embedding_score: 0.9504
- bge_forward_raw: 0.1470
- bge_reverse_raw: 0.9477
- bge_symmetric_raw: 0.5474
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: da9a6fee9eb24341
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A deleted development block is not re-inserted"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "Scaffolding marks the unfilled sections"
- embedding_score: 0.9503
- bge_forward_raw: 0.8729
- bge_reverse_raw: 2.1142
- bge_symmetric_raw: 1.4935
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0652b9dd2ce8d991
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "A missing README blocks the archive"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check reports a missing README"
- embedding_score: 0.9502
- bge_forward_raw: 1.2440
- bge_reverse_raw: 2.1873
- bge_symmetric_raw: 1.7156
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 4c88b704cf859823
- left_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- left_title: "Scaffolding marks the unfilled sections"
- right_file: "givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature"
- right_title: "The check passes on a complete README"
- embedding_score: 0.9502
- bge_forward_raw: 1.2320
- bge_reverse_raw: 0.4936
- bge_symmetric_raw: 0.8628
- bge_admission: BGE_FILTERED
