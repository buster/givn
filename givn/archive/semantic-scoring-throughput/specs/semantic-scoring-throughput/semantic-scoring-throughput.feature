# This change is intentionally guarded by no scenario tests.
#
# The behaviour it originally defined — reporting the resolved thread count
# on each model stage's status line — was dropped during implementation as
# untestable: thread counts are not observable through the stub backends,
# and probing the real ONNX models for thread counts is not viable.
#
# Retained (verified by the code-level resolver and the full-suite smoke
# run, not by Gherkin scenarios): each model stage (E5 fastembed, BGE
# reranker, NLI classifier) constructs with the machine's available
# parallelism as its intra-op thread budget by default, with an optional
# GIVN_SPEC_THREADS override, applied only in the real-model paths.

@givn.delta @semantic-scoring-throughput

Feature: Semantic model scoring uses the available parallelism
