# Proposal: Complete Semantic Token-Cap Evidence

## Problem / Opportunity

Semantic review currently sends a pair of scenarios to BGE or NLI even when the
complete pair is longer than that model's input window. The model can therefore
score an incomplete pair while the report presents the result as ordinary
second-stage evidence.

The semantic input also does not yet include all of the Gherkin context that can
distinguish two scenarios. Expanding that input without a consistent index hash
would reuse E5 vectors made from a different representation and produce a stale
candidate pool.

These behaviours hide the facts that make scenarios distinct and give an agent no
reliable signal that the pair must be shortened or split before review can be
trusted.

## Proposed Solution

Use one complete, deterministic semantic serialization for E5 indexing and for
BGE/NLI pair inputs. It includes Feature and Rule context, inherited
Backgrounds, descriptions, resolved step roles and values, tables, docstrings,
Scenario Outline templates, and concrete example rows. Paths, tags, scores, and
Givn classifications remain metadata; deterministic fingerprints remain a
separate evidence source.

Keep the existing E5 candidate-generation algorithm and candidate scopes. The
complete serialization becomes the E5 index input and its content hash. An old
index entry is reusable only when its endpoint identity and stored content hash
match the hash of the newly complete serialization. A changed hash is
re-embedded; no separate serializer-version requirement is introduced.

Before any model construction or inference, measure the complete inputs with the
actual local model tokenizer. Do not truncate or pad for this measurement. E5
passages and BGE/NLI pairs use their model limit of 512; BGE/NLI pair counts
include pair special tokens. A count of exactly 512 is valid and 513 is over
the cap.

For BGE and NLI, cap handling is recorded per candidate. `BGE_TOKEN_CAP` and
`NLI_TOKEN_CAP` candidates remain visible, unresolved, outside the filtered
collection, and blocking wherever the layer participates in blocking review.
Run-level `BGE_UNAVAILABLE` and `NLI_UNAVAILABLE` are used only when no pair in
that layer can be scored. A partial run remains enabled and reports fitting
scores alongside per-candidate cap evidence.

BGE and NLI independently score the same E5 candidate pool. Combined
recommendations are the intersection of the participating layers' recommended
sets; NLI never receives only the BGE survivors. A candidate that is capped,
unscored, or unavailable in one layer is not silently filtered by that layer and
cannot become resolved from the other layer alone.

The duplicate audit and blocking review expose the measured left, right, pair,
and limit counts, the affected model, the token-cap reason, and an explicit
instruction to shorten or split the scenarios and rerun review. Advisory
exploration and advisory review use the same shared guard, but remain advisory.
No model evidence assigns a Givn classification.

Canonical retrieval/spec guidance and its derived guidance must define these
statuses, preserve the distinction between token-cap and filtered evidence, and
instruct agents to sharpen, shorten, or split scenarios rather than inflate them
or add automatic edits.

## Out of Scope

Automatic chunking, automatic shortening, automatic splitting, and automatic
scenario edits are out of scope. The system reports the complete-pair problem;
the agent refactors the feature and reruns the review.

Changing the E5 candidate-generation algorithm, BGE/NLI thresholds,
deterministic exact or structural findings, or agent-owned Givn classification
is out of scope.

## Acceptance Boundary

Regular and E2E tests use a deterministic test-only token-budget seam. They do
not download models, use a network service, or construct multi-gigabyte local
inference artifacts. Production uses the real model tokenizers and local
artifacts.

The shared guard applies to permanent duplicate audit, permanent and
change-aware Explore, advisory review, and blocking review. The change-level
scenarios concentrate on duplicate-audit JSON and blocking-review evidence
without multiplying every surface and layer combination.

No product decision remains open. Arc42 documentation is a separate enabled
lifecycle artifact and remains required before this change can advance past the
planning gate.
