# 12. Glossary

| Term | Definition |
| --- | --- |
| **Addon** | A pre-packaged artifact (or set) shipped embedded but off by default; enabled via the `addons:` overlay sugar (e.g. `arc42`). givn's extension model — no separate plugin runtime. |
| **Agent target** | One of the agent directories givn can install generated skills/commands to: `opencode`, `agents` (`.agents/`), `claude` (`.claude/`). The enabled set is stored in `skills.targets` in `givn/config.yaml`; default `[opencode]`. |
| **arc42-docs** | The bundled addon artifact that creates and maintains durable project-level arc42 documentation (default after `design`, output `docs/arc42/`). |
| **Artifact** | A produced document in the lifecycle (proposal, spec, design, tasks, review). Each has facets: a template, an instruction, and dependencies. May be `optional` and is fully configurable via overlay. |
| **Capability** | A named unit of behaviour, kebab-case (e.g. `user-auth`), corresponding to one `.feature` file. |
| **Cascade** | The CSS-style resolution where a project override file wins over the embedded default, per facet. |
| **Change** | A unit of work moving through the lifecycle, living in `givn/changes/<id>/` and ending in `givn/archive/<id>/`. |
| **Delta** | A `.feature` file tagged `@givn.delta` describing additions/modifications/removals to be merged into the permanent spec. |
| **Disposition table** | The review.md table in which every shape match involving the change's delta is classified (`merge`/`delete`/`boundary`/`related`); the review gate blocks until it is complete. See ADR-037. |
| **Fingerprint** | The deterministic, masked assertion sequence plus extracted facts (exit code, contains polarity, data-table contents, `[DONE]`, TTY) of a scenario; the basis of shape-match and subset detection. See ADR-037. |
| **Net-delta receipt** | The review output counting added/modified/removed scenarios of a change (prose + JSON line), acknowledging the net effect of removed+added pairs. |
| **Eject** | Materialise an embedded default into the project `givn/` dir so it can be edited (`givn eject`). |
| **Facet** | The smallest overridable unit of an artifact: template, instructions, or deps. |
| **Feature file** | A Gherkin `.feature` file. In givn it IS the specification (no `spec.md`). |
| **Manifest** | The YAML document defining the artifact graph, `apply`, and `verify`. Embedded default + project overlay. |
| **Overlay / patch** | The project's `givn/config.yaml`, merged onto the embedded default manifest (add/modify/remove). |
| **Permanent spec** | The merged, tag-stripped `.feature` under `givn/specs/` — the source of truth after archive. |
| **Permanent scenario audit** | The complete read-only inspection produced by `givn spec duplicates` over eligible scenarios under `givn/specs/`, using current permanent files and transient embeddings rather than `.spec-index.json`. |
| **Candidate pair** | Two distinct permanent scenarios for which the audit has exact, structural, or semantic evidence. |
| **Exact duplicate** | Two scenarios with the same canonical behavior body regardless of title or file location. |
| **Structural overlap** | Two scenarios with the same masked assertion shape or a strict masked assertion-subset relationship, reported with the existing `Fingerprint` fact model and distinguishing values; Given/When action structure is not part of the relation. |
| **Semantic candidate** | A permanent scenario pair outside deterministic findings whose local E5 passage similarity reaches the inclusive `0.9500` threshold; advisory evidence, not an automatic duplicate decision. |
| **Scenario identity** | The stable pair of project-relative feature file and scenario title identifying one audit endpoint. |
| **Scenario** | A Gherkin scenario (Given/When/Then). The unit of the TDD loop. |
| **Shape match** | Two scenarios whose masked assertion sequences are equal but whose titles differ; a lint warning that requires a disposition at review unless their facts contradict. See ADR-037. |
| **Semantic review report** | The change-local generated Markdown worklist at `givn/changes/<id>/semantic-review.md`, regenerated only by blocking `givn check review --change <id>` and containing candidate evidence plus agent-owned classifications. The existing archive move retains it under `givn/archive/<id>/` as historical evidence; it is not a permanent specification. |
| **Semantic report wire grammar** | The exact UTF-8/LF Markdown byte contract for `semantic-review.md`: fixed headers and candidate field order, compact JSON serialization, `null` as the only absent value, exact score/hash formats, one final newline, and no unknown, timestamp, or environment fields. |
| **Semantic review candidate** | A pair involving an active `Added` or `Modified` scenario from the current change with an existing E5 cosine score at or above the binary-fixed `0.9500` boundary: either delta-to-permanent or delta-to-delta. Removed and permanent-permanent pairs are excluded. |
| **Candidate identity** | The deterministic fixed-format hash of report schema, candidate scope, canonical left/right endpoint tuples, the fixed four-decimal score string, and both endpoint evidence hashes. |
| **Semantic classification** | The agent's exact decision for a semantic review candidate: `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN`; every resolved value requires a rationale, and missing/invalid/`UNCERTAIN` state blocks change review. A candidate that appears unrelated above the boundary remains unresolved until its vocabulary is sharpened or it receives an allowed classification. |
| **Classification rationale** | The agent-authored explanation for a semantic classification. It is carried forward only with unchanged candidate evidence and is required before a candidate can pass. |
| **Vocabulary sharpening** | Rewriting overloaded domain terms and scenario invariants so two different concepts are distinguishable to a human and no longer remain an unresolved high-similarity candidate. |
| **Classification carry-forward** | Reusing a prior semantic classification only when the candidate id and both complete endpoint evidence hashes are unchanged; any content, table, background, operation, target, model, index, fingerprint/evidence schema, or report schema change makes the regenerated candidate `UNCERTAIN`. |
| **Skill** | A `SKILL.md` (frontmatter + markdown) that an agent can load to perform a workflow action. |
| **TDD loop** | Scenario-by-scenario RED (failing assertions) → GREEN (minimum code) → REFACTOR. |
| **Hook** (`post`) | A command an artifact runs after completion: built-in `verify` (runs the test runner) or `lint`, or a shell command. Run by `givn check` and as the `givn archive` gate; failure blocks (unless `allow_failure`). See ADR-028, ch.8.12. |
| **Check** (`givn check`) | Built-in command that runs an artifact's `post` hooks (e.g. the test suite via `verify.command`). Exits non-zero if a required hook fails. |
| **Lint** (`givn lint`) | Built-in command that statically checks `.feature` syntax, `@wip`/PENDING, and delta tags. Exit 0/1/2. **Runs no tests.** (Renamed from `givn verify` — ADR-027.) |
| **Test runner** | The command that actually executes step definitions against code, configured in `verify.command`. Invoked by the agent/CI, not by givn. Confirms GREEN. |
| **`verify.command`** | The manifest config key holding the test-runner command (default sentinel `givn missing-testrunner`). Read after overlay resolution; written by `init`/`upgrade`; invoked by agent/CI. |
| **Canonical authoring policy** | The resolved `specs` instruction containing the normative rules for interaction inventories and E2E evidence. Other guidance references it instead of restating it. |
| **Effective instruction** | The instruction facet selected by the project override cascade or, when no override exists, the embedded asset in the running binary. |
| **Instruction JSON contract** | The single machine-readable object emitted by `givn instructions --json`: `id`, `generates`, `requires`, and `instruction`. |
| **Derived guidance** | Artifact-specific fields, audit procedures, or choreography that may reference the canonical policy but may not redefine it. |
| **Embedding** | A numeric vector representation of scenario or query text used for interactive ranking and strict scenario-pair comparison. |
| **Complete semantic serialization** | The deterministic model text for one parsed Gherkin scenario: Feature/Rule context and descriptions, inherited backgrounds, resolved step roles, tables, docstrings, outline templates, and concrete Examples rows in stable order. It feeds E5, BGE, and NLI; paths, tags, scores, classifications, and deterministic Fingerprints are not part of it. |
| **Semantic content hash** | The hash of the exact complete serialized E5 passage. An existing vector is reusable only when endpoint identity and this hash match; a mismatch re-embeds, with no separate serializer-version field. |
| **Actual-tokenizer preflight** | The guard that measures an input with the tokenizer of the model that will consume it, before model construction or inference, without truncation or padding. |
| **E5 token-cap candidate** | A complete scenario passage above the E5 tokenizer's inclusive 512-token budget. It has no E5 vector or semantic pair score, remains visible and blocking as `E5_TOKEN_CAP`, carries exact count/limit/model/tokenizer provenance, and receives shorten/split-and-rerun guidance without an automatic Givn classification. |
| **E5 token-cap metadata** | The additive permanent-index record for an over-cap endpoint. It is kept separately from vector entries, reused only with a matching complete content hash, and sorted by endpoint identity. |
| **Pair token count** | The actual tokenizer length of a complete left/right BGE or NLI encoding, including pair special tokens; it is not the sum of individual counts. |
| **Token budget** | The inclusive model input limit of 512 tokens for an E5 passage or a BGE/NLI pair. Exactly 512 fits and 513 is over. |
| **Shared E5 candidate pool** | The immutable candidate identity and pair set produced by E5 for one invocation and supplied independently to BGE and NLI. NLI never receives only BGE survivors. |
| **Combined recommendation** | The intersection of the participating BGE and NLI recommended sets. Filtered, capped, unscored, or unavailable evidence remains visible and cannot be resolved by another layer alone. |
| **Partial token-cap run** | A second-stage run in which at least one pair fits and is scored while other candidates receive per-candidate token-cap evidence; the layer remains enabled. |
| **All-over-cap run** | A second-stage run in which no pair fits the layer's token budget; the layer is run-level unavailable with reason `token-cap`, while affected candidates retain cap evidence. |
| **Retrieval index** | The local persisted collection of scenario titles, paths, content hashes, and embeddings used by `givn spec search`. |
| **Advisory match** | A result from interactive `givn spec search`; it informs an author but does not itself block the workflow. |
| **Explore query** | One complete Gherkin scenario supplied through `givn spec explore` for comparison with permanent scenarios and, optionally, active scenarios in a named current change. |
| **Exact scenario identity** | The pair `(project-relative forward-slash file identity, scenario title)` used to omit a file query's own candidate; modified scenarios also have a projected permanent target identity. |
| **Embedding score** | The cosine similarity between E5 passage vectors; used as advisory explore ranking evidence and as the semantic-report candidate signal. Semantic-report candidates are restricted to `0.9500` through `1.0000` and rendered with four decimals. |
| **Current-change candidate** | An active `@givn.added` or `@givn.modified` scenario held in request-local exploration or semantic-review evidence; it is not a permanent index entry. |
| **Request-local corpus** | The transient set of active current-change scenarios and vectors merged with permanent candidates for one exploration invocation. |
| **Projected target identity** | The normalized `(target file, scenario title)` identity a modified delta scenario will have in the permanent tree after archive; used to prevent self-matches before archive. |
| **Source metadata** | The additive result fields and text labels identifying a candidate's scope, source file, and owning change where applicable. |
| **Delta-to-permanent scope** | A semantic review comparison between one active current-change endpoint and one permanent endpoint. |
| **Delta-to-delta scope** | A semantic review comparison between two distinct active current-change endpoints from the same change, emitted once in canonical endpoint order. |
| **Deterministic facts** | The available `Fingerprint` evidence for a semantic review endpoint: exit code, `contains` polarity, masked table facts, `[DONE]`, TTY mention, and fact contradiction; facts do not classify candidates. |
| **Endpoint evidence** | The canonical evidence object for one report endpoint: scope, normalized file/title, all normalized steps, background steps, table contents, delta operation, projected target, content hash, E5 model/index metadata, deterministic facts, fingerprint/evidence schema version, and report schema version. |
| **Complete endpoint evidence** | The source-ordered report representation used for carry-forward hashing: feature `Background`, applicable `Rule` `Background`, then scenario steps, with every step docstring and data-table row/cell preserved. It records metadata and evidence separately from the complete semantic model text, whose exact serialization feeds E5, BGE, and NLI. |
| **Evidence hash** | The exactly 16-character lowercase hexadecimal hash of complete endpoint evidence. It is the authoritative carry-forward boundary. |
| **Report schema version** | The fixed integer identifying the semantic-review Markdown grammar. A mismatch invalidates prior report entries. |
| **Permanent-only index boundary** | The rule that the persisted retrieval index may refresh permanent entries but contains no active-delta vectors, semantic classifications, or report state. |
| **Semantic report gate** | The dedicated entrypoint called only from blocking `givn check review --change`; it generates the report, enforces complete classification, and returns the existing retrieval-unavailable error when the feature is absent. |
| **Permanent-index refresh recovery** | The existing behavior for a missing, malformed, stale, or incompatible permanent index: treat it as stale, rebuild it from permanent specifications, continue semantic review, and keep the rebuilt index free of `Delta` state, delta vectors, classifications, and report state. |
| **Projected remediation** | The read-only permanent-specification view obtained by applying the active delta in memory to verify whether an agent's semantic classification was actually resolved. |
| **Prior remediation entry** | The strict historical candidate identity, endpoint identities/hashes, classification, and rationale used to verify a candidate even when it disappears from the regenerated report. |
| **Parameterized variant contract** | A projected `Scenario Outline` with a non-empty `Examples` table that represents a `VARIANTE` without retaining two independent contracts. |
| **Observable boundary invariant** | A changed condition, outcome, or deterministic fact that makes a `VALID-BOUNDARY` distinction explicit in the projected endpoint evidence. |
| **Semantic remediation receipt** | The deterministic `semantic remediation verified` output emitted only after all prior resolved classifications satisfy their projected contracts. |
| **Project-relative path identity** | A file identity normalized through one helper relative to the project root and rendered with forward slashes, regardless of whether input was relative or absolute. |
| **Similarity candidate** | A distinct scenario pair whose passage-vector cosine score is at or above the binary-fixed `0.95` threshold. It is a hard review failure until resolved. |
| **MERGE remediation** | The required response when a similarity candidate is a true duplicate: delete the duplicate scenario rather than keeping a second copy. |
| **KEEP-AS-BOUNDARY remediation** | The required response when a similarity candidate is valid separate coverage: rewrite the retained title and steps so the observable distinction is explicit and the score falls below `0.95`; it is not an exception. |
| **Retrieve-then-rerank** | A two-stage search flow in which embeddings retrieve a bounded candidate set and a pair model performs the final comparison. |
| **Pair reranker / cross-encoder** | A model that receives two complete texts jointly and returns a relevance logit for their relationship; it cannot precompute one independent vector per document. |
| **Reranker raw logit** | The uncalibrated scalar returned by the pair model before the diagnostic sigmoid transformation. It is not an embedding cosine score or probability. |
| **E5 raw candidate** | A deterministic candidate produced by the E5 top-K generator before BGE admission. It remains visible even when BGE filters the recommendation. |
| **BGE evidence** | Local bidirectional relevance evidence from `rozgo/bge-reranker-v2-m3`, including forward raw logit, reverse raw logit, symmetric average, model identity, and elapsed runtime. It is not a Givn classification. |
| **BGE_RECOMMENDED** | Non-authoritative admission status for a BGE-scored pair whose symmetric raw logit is at least the fixed `3.0000` operating point. The pair still requires agent classification in blocking review. |
| **BGE_FILTERED** | Non-blocking evidence status for a BGE-scored pair below the operating point. It is recorded for auditability, receives no Givn classification, and cannot block change review. |
| **BGE_TOKEN_CAP** | Per-candidate evidence that the complete BGE pair exceeds the actual tokenizer's 512-token budget. It records left/right/pair/limit counts, has no score, remains visible and unresolved outside the filtered collection, and blocks review. |
| **BGE_UNSCORED** | A raw E5 candidate outside the successfully scored bounded BGE subset. It has no BGE score, remains visible, and remains a blocking E5 candidate requiring normal agent classification. |
| **BGE_UNAVAILABLE** | Explicit run-level state indicating that BGE scored no pair, including artifact refusal, bounded pair-pool overflow, or an all-over-cap token run with reason `token-cap`. A build with BGE disabled has no BGE evidence object and is represented by the existing E5-only contract, not by this status. |
| **Bounded BGE pool** | The deterministic union of at most five E5 neighbors per eligible query/endpoint, after identity/deterministic exclusions and before BGE inference, capped at 512 unique undirected pairs per invocation. |
| **Recommended review set** | The BGE-recommended subset shown as the agent's prioritized worklist; filtered and unavailable evidence remains separately visible. |
| **Directional raw score** | The BGE logit for one ordered pair direction. Both directions are scored; their arithmetic mean is the symmetric raw score. |
| **Admission operating point** | The labeled-benchmark operating value used to emit `BGE_RECOMMENDED` or `BGE_FILTERED`; it is not a Givn classification or remediation threshold. |
| **BGE admission approval** | A maintainer-generated `givn/.spec-bge-admission.json` artifact proving the five benchmark criteria, exact model/bounds, Rust candidate-pipeline parity, and local artifact hashes. Without it, product BGE remains E5-only. |
| **Rust production candidate pipeline** | The canonical E5 scoring, deterministic exclusion, top-K union, and pair canonicalization path in the compiled givn binary. Its fingerprint is required before benchmark approval. |
| **Python reference pipeline** | The offline benchmark parser and scorer used for measurement. It is explicitly not sufficient to approve product BGE admission. |
| **BGE local artifact set** | The offline `model.onnx`, referenced external weights, tokenizer, config, special-token map, and tokenizer-config files validated before user-defined FastEmbed construction. |
| **NLI evidence** | Local bidirectional evidence from the multilingual NLI model, containing entailment, neutral, and contradiction probabilities for both directions, a symmetric summary, model identity, and elapsed runtime. It is not a Givn classification. |
| **NLI_RECOMMENDED** | Non-authoritative admission status for an NLI-scored pair whose symmetric entailment probability is at least the fixed inclusive `0.5000` operating point. The pair still requires agent classification in blocking review. |
| **NLI_FILTERED** | Non-blocking evidence status for an NLI-scored pair below the symmetric entailment operating point. It remains visible and receives no Givn classification. |
| **NLI_TOKEN_CAP** | Per-candidate evidence that the complete NLI pair exceeds the actual tokenizer's 512-token budget. It records left/right/pair/limit counts, has null probabilities, remains visible and unresolved outside the filtered collection, and blocks review. |
| **NLI_UNSCORED** | A raw E5 candidate outside the successfully scored bounded NLI subset. It has no NLI probabilities, remains visible, and remains a blocking E5 candidate. |
| **NLI_UNAVAILABLE** | Explicit run-level state indicating that NLI scored no pair, including artifact refusal, bounded pair-pool overflow, or an all-over-cap token run with reason `token-cap`. It preserves the E5 fallback and does not claim NLI participation. |
| **Bounded second-stage pool** | The shared deterministic union of at most five E5 neighbors per eligible query/endpoint, after identity/deterministic exclusions and before BGE/NLI inference, capped at 512 unique undirected pairs per invocation. |
| **Symmetric NLI probability** | The softmax probability vector produced from the arithmetic mean of the forward and reverse NLI logits; its entailment component drives NLI admission only. |
| **Coverage measurement command** | The project-owned command in `coverage.measure_command` that runs the configured test subsets and writes the non-E2E and E2E source coverage reports. |
| **Coverage merge command** | The project-owned command in `coverage.merge_command` that combines the two source reports into `coverage.output_path`. Its fresh output is authoritative for README coverage. |
| **Merged coverage report** | The Cobertura XML emitted by the coverage merge command. givn validates this report and uses its line and branch counters for both README badges and the managed summary. |
| **Coverage badge block** | The managed `coverage-badge` README block directly below the title, containing line and branch badges linked to the merged report. |
| **`@givn.delta`** | Feature-level tag marking a delta document. |
| **`@givn.added` / `@givn.modified` / `@givn.removed`** | Scenario-level merge directives. |
| **`@wip`** | Scenario tag: step definitions not yet implemented. |
| **`GIVN_FEATURES`** | Env var overriding the feature path for the test runner (default `givn/specs`). |
| **arc42** | The architecture documentation template this `docs/arc42/` folder follows. |
| **Arc42 implementation conformance** | The completion-review comparison of durable Arc42 facts and the change-level assessment against the design, mapped tasks, and completed implementation. Required only when the `arc42` addon is enabled. |
| **Architecture fact** | A documented architectural constraint, decision, structure, flow, quality requirement, risk, or domain term that a change may need to preserve or update. |
| **ADR** | Architecture Decision Record (chapter 9). |
| **Ownership fixture** | A temporary generic project used to exercise Givn's repository-wide review and archive contract through the real CLI. |
| **Overlap disposition** | A maintainer's explicit decision recorded for a pair of scenarios reported as structurally related. |
| **Net-change receipt** | The deterministic count of added, modified, and removed scenarios reported by review or archive evidence. |
| **OpenSpec** | The TypeScript tool whose fork prototyped this workflow; givn is a clean-break successor. |
