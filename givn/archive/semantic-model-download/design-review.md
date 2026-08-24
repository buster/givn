# Design Review: semantic-model-download

## Review Scope

The review covered `proposal.md`, the complete delta feature, `design.md`,
`arc42.md`, all twelve durable Arc42 chapters, ADR-039, ADR-056, ADR-057,
ADR-058, ADR-061, the retrieval implementation, the Cucumber runner, and
`run-tests.sh`. The review was performed in a fresh context before task
generation. No production implementation is treated as evidence at this
stage.

## Findings And Resolutions

| Severity | Finding | Resolution |
|---|---|---|
| P0 | The combined BGE/NLI E2E scenario could run in the E5-only phase. | The duplicate-audit E2E scenario is tagged `@nli @nli-combined @e2e`; E5 filters exclude the combined tag and the combined-feature phase is its sole executor. |
| P0 | A single E2E scenario collapsed distinct CLI actions. | The interaction inventory now has six entries and six matching E2E scenarios: index, search, explore, advisory review, blocking review, and duplicate audit. |
| P1 | Absent, malformed, and incompatible indexes were conflated. | The design requires an explicit absent-index result, validation-specific errors, one top-level diagnostic context, and regular coverage for absent, malformed, and incompatible paths. |
| P1 | XDG, `HF_HOME`, role overrides, and flat paths had no complete precedence. | The design defines the complete E5/BGE/NLI table, explicit flat-directory precedence, `HF_HOME` compatibility behavior, relative-path semantics, and `ApiBuilder::from_cache`. |
| P1 | E5 tokenizer discovery could select another model's tokenizer. | The design fixes the FastEmbed repository/files and requires exact repository resolution or `GIVN_SPEC_E5_TOKENIZER`; recursive shared-directory discovery is prohibited. |
| P1 | Corrupt, partial, or mixed cache snapshots could be reused. | Every automatic role cache has a revision/file/size/SHA-256 manifest, one-snapshot validation, atomic manifest publication, and affected-snapshot reacquisition. |
| P1 | BGE `local` and NLI `nli` could silently change from offline selectors to download selectors. | Existing `local`, `nli`, `stub`, `nli-stub`, `missing`, and `nli-missing` semantics remain unchanged. New `auto` and `nli-auto` selectors own acquisition; availability remains separate from BGE approval. |
| P1 | Automatic NLI snapshots use `onnx/model.onnx` while flat overrides use `model.onnx`. | The design defines a resolver result containing snapshot directory, model path, metadata paths, and optional external data; both deployment and glossary documentation describe both layouts. |
| P1 | Archive could acquire models through pre- or post-archive refresh. | Archive retrieval receives an explicit no-download policy; only explicit semantic commands use acquisition. The design requires separate pre-merge and post-success verification. |
| P1 | Download errors lost role, model, cache, file, and override context. | Structured acquisition diagnostics retain all of those fields, map selected BGE/NLI failures to existing unavailable statuses, and keep JSON stdout valid. |
| P1 | Existing `nli-stub` semantics conflicted with resolver testing. | `nli-stub` remains fully offline; acquisition validation uses a separate test seam and local HF-compatible fixture. |
| P1 | The E5 regression could not observe production ordering under the global stub runner. | The fixture endpoint is subprocess-visible through `HF_ENDPOINT`, records `artifact-resolved` before `token-budget-measured`, and is distinct from `GIVN_SPEC_EMBEDDER=stub`. Real multi-gigabyte construction remains a manual probe. |
| P1 | A permanent NLI scenario contradicted ADR-058/ADR-061 and the design. | The change carries a `@givn.modified` delta for the existing scenario, changing it to the independent full-E5-pool contract. |
| P1 | A failed first download did not specify existing-index preservation. | The feature has separate no-index and existing-index failure scenarios; the latter requires byte-for-byte preservation. |
| P1 | The E2E runner could silently skip NLI phases. | The design requires `run-tests.sh` to fail when the active feature path cannot be resolved and to route the combined tag only through the combined feature command. |
| P2 | User-facing setup/recovery documentation was unnamed. | The design names `README.md` and `docs/retrieval-models.md` as the operational documentation owners. |

## Scope

The proposal, feature delta, and design all cover the same behavior: explicit
semantic use acquires validated E5/BGE/NLI artifacts in shared XDG caches,
keeps project-local index state separate, preserves selector and approval
boundaries, and emits actionable diagnostics. No scoring, threshold, token cap,
index schema, or automatic classification change is included.

## Technical Choices

Synchronous `hf-hub` is appropriate because the retrieval pipeline and existing
FastEmbed/ORT path are synchronous. Direct cache construction avoids the
dependency's implicit home-directory lookup. FastEmbed remains the E5 inference
loader, while BGE retains its user-defined local loader and NLI receives an
explicit resolved model path. A local HF-compatible fixture is the smallest
subprocess-visible digital twin; live model traffic is excluded from the suite.

## Testability

Every scenario has concrete CLI, filesystem, cache, index, or stderr assertions.
The six E2E scenarios each drive one compiled CLI subprocess for one inventory
action and assert CLI output first. Regular scenarios cover selector variants,
invalid artifacts, malformed indexes, old-index preservation, and archive
no-download behavior. New Rust steps begin with `unimplemented!()` and the
runner uses `.fail_on_skipped()`.

## E2E Fidelity And Coverage

The real interface is the compiled `givn` CLI. The inventory in the feature file
and the six-row matrix in `design.md` match exactly. The combined BGE/NLI
scenario is excluded from E5-only phases by its `@nli`/`@nli-combined` tags and
runs through the feature set that compiles both evidence layers. The primary
assertion for each E2E scenario is stdout/stderr and exit status; cache/index
inspection is secondary evidence.

## Risk

The highest implementation risk is cache-path divergence between FastEmbed,
HF Hub, and explicit overrides. The design addresses it with one resolver,
explicit precedence, `Cache::new`, exact repository paths, manifests, and
subprocess fixture assertions. The second risk is accidental archive download;
the explicit no-download policy is passed through retrieval entrypoints rather
than encoded in mutable process-global state.

## Architecture Documentation

All twelve Arc42 rows are independently assessed as affected in `arc42.md`.
The durable chapters contain decision-specific content and no prohibited ASCII
diagrams. E5 cache ownership is routed as `AMEND_ADR` to ADR-039. BGE artifact
acquisition is routed as `AMEND_ADR` to ADR-057. NLI path mechanics, the shared
diagnostic, and explicit-operation behavior are owned by the feature/design and
Arc42 chapters rather than duplicated in a new ADR. ADR-056 remains the BGE
approval authority.

## Static Evidence

Command:

```sh
givn lint --change semantic-model-download
```

Result: one feature file parsed; seventeen expected `@wip` findings; no parse,
duplicate-title, or structural errors; exit status `2`.

## Status

DESIGN-REVIEW: PASS

## Post-Review Revalidation

The implementation phase made only contract-preserving clarifications after
the fresh design review: the archive scenario uses an existing removed target so
the no-download path cannot require new E5 vectors, the feature-free assertion
uses the existing CLI diagnostic form, and the duplicate-audit E2E wording is
unique from older capability bindings. The six inventory entries, six E2E
scenarios, combined-feature routing, XDG precedence, resolver snapshot model
paths, archive no-download policy, and ADR-039/ADR-057 amendment routing remain
unchanged. `givn lint --change semantic-model-download` is clean and the final
active full/E2E matrices pass.

DESIGN-REVIEW: PASS
