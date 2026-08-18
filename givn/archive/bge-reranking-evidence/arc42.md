# Arc42 Impact Assessment: BGE Reranking Evidence

This change promotes the existing local BGE experiment into an opt-in,
advisory evidence layer across every E5-backed search surface. `BGE_FILTERED`
is intentionally non-blocking and is not a Givn classification.

## Chapter Assessment

| # | Chapter | Affected | Change summary |
|---:|---|---|---|
| 1 | Introduction and goals | Yes | Adds false-positive reduction, top-K recall, runtime, and concise-authoring goals. |
| 2 | Architecture constraints | Yes | Adds feature independence, local-artifact, no-network, top-K, threshold, and pair-cap constraints. |
| 3 | Context and scope | Yes | Extends five CLI search surfaces with BGE raw/recommended/filtered evidence. |
| 4 | Solution strategy | Yes | Adds bounded E5 union, directional BGE scoring, and non-blocking admission. |
| 5 | Building block view | Yes | Adds shared BGE pool/evidence types, renderers, benchmark assets, and capability steps. |
| 6 | Runtime view | Yes | Adds explore, advisory review, blocking report, duplicate, unavailable, and overflow flows. |
| 7 | Deployment view | Yes | Adds local BGE cache/artifact behavior and explicit no-download test mode. |
| 8 | Cross-cutting concepts | Yes | Defines mode selection, identity exclusions, wire states, bounds, and authoring guidance. |
| 9 | Architecture decisions | Yes | Records advisory admission, independent feature flags, bounded refusal, output versioning, and NLI isolation. |
| 10 | Quality requirements | Yes | Adds admission precision/recall, top-K recall, runtime, candidate accounting, and verbosity measurements. |
| 11 | Risks and technical debt | Yes | Adds false filtering, recall loss, overflow, stale output, selector collision, and artifact-availability risks. |
| 12 | Glossary | Yes | Adds E5 raw candidate, BGE evidence, BGE_RECOMMENDED, BGE_FILTERED, bounded pool, and unavailable mode. |

## Durable Updates Required

- Chapters 1, 3, 4, 6, and 8 must no longer describe BGE/NLI as outside the
  permanent audit or blocking report once this change is implemented.
- Chapter 9 must contain MADR entries for the non-blocking BGE admission
  contract, independent feature graph, bounded-pool refusal, output/schema
  compatibility, and NLI isolation.
- Chapter 11 must contain a consequence for each of those decisions.
- Chapter 12 must define all new evidence and admission terms.

## Status

STATUS: DONE
