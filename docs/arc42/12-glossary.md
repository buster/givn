# 12. Glossary

| Term | Definition |
|---|---|
| **Addon** | Embedded behavior toggled through `addons.<name>`. The embedded manifest starts disabled, but current `givn init` enables all known addons unless `--no-addons` is supplied. |
| **Addon-aware generation** | Generation that includes only the template sections whose declared addon condition matches the current effective addon set; an addon that is off leaves no trace in the generated skill or slash command. |
| **AGENTS.md ownership** | Givn owns only the content between its own named fenced markers. All other `AGENTS.md` content is user-owned and preserved. |
| **Agent target** | One generated guidance destination: `opencode`, `agents`, or `claude`, represented by `.opencode/`, `.agents/`, or `.claude/`. |
| **arc42-docs** | The effective lifecycle artifact added when the Arc42 addon is enabled. Its marker is `arc42.md`; durable chapters are scaffolded at fixed `docs/arc42/`. |
| **Artifact** | A lifecycle document or generated path described by the effective manifest. It may have `requires`, `optional`, and configurable `post` hooks. |
| **Capability** | A named behavior unit, represented by a feature file under `givn/specs/<group>/`. |
| **Cascade** | Project-overrides-first lookup for an individual template, instruction, skill, or command asset, with embedded content as fallback. |
| **Conditional section** | A marked region of a template retained only when its declared addon condition matches the current effective addon set; it is dropped from the generated file otherwise. |
| **Change** | A unit of work under `givn/changes/<id>/`, later copied to `givn/archive/<id>/`. |
| **Delta** | An active Gherkin feature marked `@givn.delta` that describes Added, Modified, or Removed scenarios. |
| **Disposition** | A review entry for a deterministic overlap finding involving a change delta. |
| **Fingerprint** | A deterministic masked assertion sequence plus best-effort facts: exit code, `contains` polarity, table values, `[DONE]`, and TTY mention. |
| **Generated skill** | A skill file written from a skill template into an agent target directory when `givn skills sync` runs. |
| **Net-delta receipt** | The count of Added, Modified, and Removed scenarios printed by the overlap review gate. |
| **Eject** | Materialization of an embedded default into a project-owned `givn/` override. |
| **Facet** | An independently resolvable asset unit such as a template or instruction. |
| **Feature file** | A Gherkin `.feature` file. In givn it is the executable behavior specification and, after archive, the permanent record. |
| **Manifest** | YAML data describing artifacts, dependencies, archive requirements, verify settings, coverage settings, and addons. |
| **Manifest overlay** | Project YAML applied to the embedded manifest. `givn/config.yaml` is applied first; `givn/commands.yaml` is applied later. |
| **Permanent spec** | A tag-stripped `.feature` under `givn/specs/<group>/`, used as the durable behavior source. |
| **Group** | A use-case directory `givn/specs/<group>/` holding one `.feature` per capability plus a `group.md` narrative (Actor, Goal, Interactions). |
| **Hook (`post`)** | A configurable post-artifact command. It may name `verify`, `verify-e2e`, `lint`, `integrity`, or a shell command. Custom shell hooks may allow failure; built-in names cannot weaken mandatory semantics. |
| **Mandatory check** | Binary-owned behavior run by explicit `givn check tasks` or `givn check review`, independent of an artifact's `post` list. |
| **Template** | A source asset (embedded default or project override) rendered into a generated skill or slash command. Skill templates carry the `.tmpl` marker; conditional sections are resolved against the current addon configuration at generation time. |
| **Test runner** | The project command in `verify.command` that executes feature scenarios. In this checkout it is supplied by `givn/commands.yaml`. |
| **`verify.command`** | The effective project test-runner command. The embedded fallback is `givn missing-testrunner`; initialization writes `./run-tests.sh` in the project command template. |
| **`verify.e2e_command`** | The command used for E2E scenarios. The current project command file uses `./run-tests.sh --e2e`. |
| **Lint (`givn lint`)** | Static feature parsing and overlap checking. It runs no project tests; exit precedence is parse `1`, duplicate title `3`, WIP `2`, clean `0`. |
| **Coverage gate** | Optional archive-level validation of project-owned measurement and merge commands and Cobertura output. It is operational/test governance, not an active ADR. |
| **Managed block** | A README region Givn owns and rewrites wholesale between its own fenced markers (coverage badges, coverage summary, development commands). User content outside managed blocks is never touched. |
| **Open placeholder** | A machine-recognizable request-for-content left in a README section by scaffolding (`givn:todo` marker text). A README without open placeholders is complete. Anti-term: "todo comment". |
| **README check** | `givn readme check` — deterministic report of every open placeholder by section plus damage to the development managed block; exits non-zero while any remains. |
| **README completeness gate** | Archive-level blocking validation that the README contains no open placeholders and its managed blocks are intact, enabled by the readme addon. It mirrors the coverage gate. |
| **README impact decision** | The machine-checkable review line recording that the README needed no update (`none`) or was updated, naming the sections. Required for review sign-off when the readme addon is enabled. Anti-term: "docs impact". |
| **README migration** | One-time agent-executed rewrite of a legacy-scaffolded or hand-written README into the marked form while preserving all real content; accepted only when the README check passes. Anti-term: "rescaffold". |
| **Complete semantic serialization** | Compact deterministic model text containing non-empty parsed Feature/Rule context, backgrounds, scenario data, resolved step roles, tables, docstrings, outline template, and examples. |
| **Semantic content hash** | Hash of the exact complete E5 passage. A vector is reused only when endpoint identity and this hash match. |
| **Actual-tokenizer preflight** | Measuring an untruncated, unpadded input with the tokenizer of the model that will consume it before construction or inference. |
| **Token budget** | The inclusive model-input limit of 512 tokens. Exactly 512 fits; 513 is over. |
| **Pair token count** | Length of a complete BGE/NLI pair encoding, including pair special tokens; it is not the sum of individual lengths. |
| **E5 token-cap candidate** | A complete passage over the E5 limit. It has no vector or E5 semantic score and is visible as `E5_TOKEN_CAP` metadata. |
| **BGE token-cap evidence** | Selected BGE pair evidence whose complete actual-tokenizer encoding exceeds 512. It has no score, remains visible and blocking, and is not filtered. |
| **NLI token-cap evidence** | Selected NLI pair evidence whose complete actual-tokenizer encoding exceeds 512. It has no probabilities, remains visible and blocking, and is not filtered. |
| **Second-stage pair budget** | The positive `GIVN_SPEC_MAX_RERANK_PAIRS` setting. Unset means 2048; it truncates only the rulebook-selected set as an emergency ceiling — candidates after the truncated prefix are unscored. |
| **Unified rulebook** | The single change-scoped comparison sequence: embed, compare (delta↔permanent and delta↔delta), cut at the E5 admission threshold, select, judge, report. All three change-scoped consumers (`givn spec review`, `givn check review`, the archive gate) apply it. Anti-terms: pipeline, pool. |
| **Pair cap** | The fixed maximum number of pairs the rulebook selects for second-stage judgement in one run (20). Pairs beyond the cap are not candidates. |
| **Neighbour count** | The fixed maximum number of highest-scored neighbours per delta scenario the rulebook keeps before the pair cap (5). |
| **Pair-score cache** | The persisted store of per-pair BGE/NLI evidence keyed by endpoint content and evidence hashes. Unchanged endpoints reuse recorded judgements; the run reports reused versus computed counts. Anti-terms: memo, result cache. |
| **Hybrid selection** | The rulebook's two-stage selection: per-scenario neighbour count first, then one shared pair cap by E5 score with deterministic tie order. |
| **Unselected pair** | A rulebook pair beyond the neighbour count or the pair cap. It is not a candidate: no evidence, no report entry, no blocking state. Anti-term: unscored candidate (reserved for budget truncation only). |
| **E5 candidate** | A semantic pair produced at or above the single E5 `0.9000` admission boundary after rulebook selection, before optional BGE/NLI evidence. |
| **Semantic review candidate** | A blocking-report pair involving an active Added/Modified delta in either `delta-to-permanent` or `delta-to-delta` scope. Removed and permanent-permanent pairs are excluded. |
| **Semantic report** | `givn/changes/<id>/semantic-review.md`, generated only by blocking `givn check review --change <id>` and retained under archive as historical evidence. |
| **Semantic report writer** | The code that emits fixed report headers/fields, compact JSON values, `null` absence, and atomic replacement. |
| **Decision-first reporting** | Advisory review's visibility policy: default text shows status lines, counters, and only recommended second-stage pairs ordered by model confidence; `--verbose` lifts all limits; `--format json` emits one complete document. |
| **Findings document** | The schema-version-1 JSON review document persisted to `givn/changes/<id>/review-findings.json` when the similarity gate fails; successful reviews write nothing. |
| **Prior report extraction** | The current line-oriented carry-forward reader for candidate id, classification, rationale, and evidence hashes. It is not a complete strict wire-grammar parser. |
| **Candidate identity** | A 16-character lowercase hexadecimal FNV-1a hash of scope and canonical endpoint scopes/files/titles. It excludes score, evidence hashes, and report schema. |
| **Endpoint evidence hash** | A separate 16-character hash over canonical endpoint evidence and model/index/schema metadata. Both endpoint hashes must match for classification carry-forward. |
| **Semantic classification** | Agent-owned `DUPLICATE`, `VARIANT`, `VALID-BOUNDARY`, or blocking `UNCERTAIN` value in the semantic report. |
| **Projected remediation** | Read-only application of the active delta to an in-memory permanent view for verifying a previous classification. |
| **Parameterized variant** | A projected `Scenario Outline` with a non-empty Examples table, required by the current `VARIANT` verifier. |
| **Observable distinction** | A changed Fingerprint assertion or fact, required by the current `VALID-BOUNDARY` verifier along with a score below `0.9000`. |
| **Delta-to-delta remediation** | A prior semantic candidate whose endpoints are both current-change entries. The current verifier rejects this scope; it is a known unsupported path. |
| **Permanent duplicate audit** | `givn spec duplicates`, a complete advisory scan of current `givn/specs/` excluding active changes and archive contents. |
| **Audit index reuse** | Duplicate Audit reads `.spec-index.json` and reuses vectors/cap metadata only when every current identity and complete hash matches. It does not write or refresh the index. |
| **Exact duplicate** | Two permanent scenarios with the same canonical behavior body, including relevant backgrounds, steps, tables, docstrings, and examples, independent of title/path. |
| **Structural overlap** | Equal masked assertion shape or a strict assertion-subset relation with Fingerprint facts. Given/When action structure is not part of the relation. |
| **Semantic candidate** | A remaining permanent pair whose E5 cosine reaches `0.9500`; it is advisory in Duplicate Audit and only evidence for later review. |
| **Scenario identity** | A project-relative forward-slash feature file plus scenario title. Modified deltas additionally have a projected target identity. |
| **Project-relative path identity** | The normalized path form used for permanent, delta-source, projected-target, and file-query comparisons. |
| **Same-file filter** | A surface-specific rule: Explore removes only the exact file/title identity; second-stage advisory projected comparisons exclude same-file pairs; strict E5, blocking report, and Duplicate Audit retain same-file pairs except for a Modified delta's own projected target where specified. |
| **Request-local corpus** | Active Added/Modified scenarios and transient vectors merged with permanent candidates for one change-aware Explore or review request. |
| **BGE evidence** | Bidirectional raw-logit evidence from `rozgo/bge-reranker-v2-m3`; its symmetric average is diagnostic and not a Givn classification. |
| **BGE_RECOMMENDED** | BGE symmetric raw logit at or above `3.0`; non-authoritative evidence that can enter a model-reduced worklist. |
| **BGE_FILTERED** | BGE symmetric raw logit below `3.0`; visible non-blocking evidence with no Givn classification. |
| **BGE_UNSCORED** | A candidate for which the selected BGE path produced no score, including a budget tail or a candidate outside the Duplicate Audit's endpoint top-five union. It has no BGE score and remains visible. |
| **BGE_UNAVAILABLE** | Selected BGE run state for missing artifact or an all-over-cap selected set. A failed approval or disabled selector instead leaves no BGE evidence object. |
| **BGE top-five endpoint union** | Duplicate Audit's union of up to five E5 neighbors per endpoint after deterministic exclusions, before budget selection. |
| **BGE approval** | `givn/.spec-bge-admission.json` metadata checked by the current runtime for schema, model, threshold, top-k, active budget, pipeline version, and `acceptance.pass`. Runtime artifact preflight is separate. |
| **BGE local artifact set** | `model.onnx`, optional referenced external weights, tokenizer/config/special-token/tokenizer-config files checked before the user-defined loader. |
| **NLI evidence** | Bidirectional local probabilities for entailment, neutral, and contradiction plus an averaged-logit symmetric vector. |
| **NLI_RECOMMENDED** | Symmetric NLI entailment probability at or above inclusive `0.5000`; non-authoritative evidence. |
| **NLI_FILTERED** | Symmetric NLI entailment probability below `0.5000`; visible non-blocking evidence. |
| **NLI_UNSCORED** | An E5 candidate outside the selected NLI budget. It has no NLI probabilities and is not a model rejection. |
| **NLI_UNAVAILABLE** | Selected NLI run state for missing artifacts or an all-over-cap selected set. Disabled NLI emits no NLI object. |
| **Surface-specific pool** | The E5 candidates supplied to a second-stage model by a particular command: Duplicate Audit BGE uses the top-five union, Duplicate Audit NLI uses the full E5 semantic set, and blocking review uses complete report candidates. |
| **Combined recommendation** | Intersection of participating BGE and NLI recommended states. Filtered, capped, unscored, and unavailable evidence is not silently resolved. |
| **Permanent-only index boundary** | The persisted index contains permanent entries and E5 cap metadata, not active delta vectors, audit state, classifications, or semantic report state. |
| **Retrieval feature** | Cargo feature `spec-retrieval`, which provides local E5/index behavior and is required for blocking semantic report generation. |
| **Strict E5 gate** | Retired by ADR-066: the separate `0.95` boundary was absorbed into the unified rulebook's `0.90` admission; `spec-similarity-gate` still controls whether the E5 layer blocks. |
| **Arc42 implementation conformance** | Human evidence review of durable Arc42 chapters and change assessment against design, tasks, and implementation, supplemented by affected-file integrity checks. |
| **ADR** | Architecture Decision Record. Active records live in `adr/`; historical records live in `adr/archive/`. |
| **Current architecture contract** | Only an ADR with `status: accepted` and no supersession relationship. Proposed and archived records do not bind the current system. |
| **ADR candidate** | A proposed choice that is evaluated for architectural significance before an ADR file is created. |
| **Qualification verdict** | The structured authoring result containing the five mandatory gate outcomes, supporting `must_be_shared` evidence, route, and route-specific target. It is not a runtime result. |
| **Mandatory qualification dimension** | One of `alternatives`, `architectural_impact`, `durable_consequence`, `lower_level_artifact`, or `existing_adr_check`; all five must pass. |
| **Canonical artifact** | The one authoritative location for a piece of information: Gherkin spec, design/code, Arc42 chapter, process guidance, or project documentation. |
| **Supporting-only evidence** | Evidence that informs a qualification decision but cannot make it pass; `Must be shared` has this role. |
| **ADR amendment** | A change to the rationale or scope of an existing decision that does not introduce an independent boundary; routed as `AMEND_ADR`. |
| **ADR supersession** | A qualified replacement of an existing decision that keeps the old record as explicit architectural history; routed as `SUPERSEDE_ADR`. |
| **ADR routing** | The single outcome `NEW_ADR`, `AMEND_ADR`, `SUPERSEDE_ADR`, or `CANONICAL_ARTIFACT` selected after qualification and duplicate checks. |
| **Single-source architecture documentation** | The rule that current facts, behavior, implementation, process, and decision rationale each have one authoritative home rather than redundant copies. |
| **Slash command** | A command file written from a command template into `.opencode/commands/` when `givn skills sync` runs. |
| **Managed upgrade path** | The machine-safe portion of a version migration: config scaffold and generated Givn integrations refreshed under an explicit Git allowlist. |
| **Maintenance change** | A visible Givn change for project-wide tooling or migration work that may intentionally skip a product feature specification. |
| **Migration plan** | The proposal and detailed design generated for an upgrade so an LLM can derive project-specific tasks without fabricated completion evidence. |
| **Upgrade Commit A** | The first automatic commit containing only the managed scaffold and generated integration changes. |
| **Migration Commit B** | The second automatic commit containing the maintenance plan and the recorded hash of Upgrade Commit A. |
| **Migration catalog** | Embedded release policy at `assets/migrations/catalog.yaml` that declares source/target bundle edges and their order. |
| **Migration bundle** | A release-owned Proposal/Design asset pair describing one project migration phase. |
| **Aggregate migration change** | One visible maintenance change containing all bundle phases required to cross a version gap, with ordered metadata in `migration.yaml`. |
| **Managed upgrade** | A migration-free upgrade that synchronizes Givn-owned files and creates one allowlist-only commit when needed. |
| **Default feature set** | Every currently declared Cargo feature activated by `cargo install` unless `--no-default-features` is supplied. |
| **XDG model cache** | User-level cache rooted at `$XDG_CACHE_HOME/givn` or `$HOME/.cache/givn`, containing shared E5, BGE, and NLI artifacts rather than project-specific index state. |
| **Role cache** | The `e5`, `bge`, or `nli` subdirectory of the XDG model cache, selected by the model layer that needs artifacts. |
| **Artifact resolver** | The validated acquisition boundary that reuses or downloads complete model files before tokenizer/model construction. |
| **Missing-index diagnostic** | `spec index is missing at <path>; run \`givn spec index\` first`, emitted when a semantic surface has no persistent index. |
| **Search evidence** | The opt-in `givn spec search --evidence` report: per-candidate `BGE_*`/`NLI_*` verdict lines over the five displayed results plus a layer-availability summary; it never changes the E5 ranking. |
| **Benchmark export cache** | The XDG `givn/nli-exported` directory containing a flat NLI ONNX export for the external comparison tooling; it is separate from the product's `givn/nli` Hub snapshot. |
| **Installed build** | A givn binary built for normal installation (the release profile used by `cargo install`). It always evaluates the semantic gates with the real models and ignores every fixture request. |
| **Development build** | A givn binary built for development and testing (the debug profile used by `cargo test` and `cargo build`). It honours fixture requests so the fixture suite stays fast and offline. |
| **Fixture request** | An environment-driven request to substitute simulated model output or to disable a model evidence stage. It is honoured only by a development build; an installed build ignores it, announces the refusal, and runs the real model. |
| **Model-location request** | An environment-driven request that still runs the real models, but from a chosen local, offline, or custom cache location. It is honoured by both builds; it never substitutes simulated output. |
| **Run declaration** | The author-owned `run:` block in `givn/commands.yaml`: product type, start command, exactly one server port source, readiness path, and readiness timeout. One replace-whole overlay unit, like `verify` and `coverage`. Anti-terms: "start config", "launch config". |
| **Product type** | The declared kind of the built product: `server`, `cli`, or `library`. Drives which run-declaration fields apply and how the README block renders. Anti-term: "product kind". |
| **Start command** | The declared command that starts the built product. Anti-terms: "entrypoint", "launch command". |
| **Sentinel start command** | `givn missing-run-command` — the scaffold placeholder meaning "not configured yet"; a hidden subcommand that prints guidance and exits 1 when executed. Anti-term: "default command". |
| **Run finding** | One labeled deterministic validation result for the run declaration: `UNCF`, `MISS`, `FORB`, or `NOOP`, from rules R1–R10. Anti-term: "warning". |
| **Run gate** | The blocking check of the run declaration in `givn check review` and `givn archive`, independent of the readme addon. Anti-terms: "start gate", "run-block gate". |
| **Run managed block** | The README region between `<!-- givn:begin:run -->` and `<!-- givn:end:run -->` rendered from the run declaration. Anti-terms: "quick start section", "run todo". |
| **Port hand-off** | Declaring `port_env` so the caller passes the port to the product through that environment variable; the declared alternative is a fixed `port`. Anti-terms: "port injection", "port mapping". |
| **Ephemeral port** | A kernel-assigned free port obtained by binding `127.0.0.1:0`, reading the port, and dropping the listener before spawn. Anti-term: "dynamic port". |
| **Readiness path** | The declared HTTP path on the started server that answers 2xx once it is ready to serve. Anti-terms: "health endpoint", "health URL". |
| **Readiness probe** | givn's blocking HTTP/1.0 poll against the readiness path: 2xx accepts, everything else retries until `ready_timeout_s`. Anti-terms: "health check", "ping". |
| **Readiness timeout** | The declared `ready_timeout_s` seconds the readiness probe may take before giving up; elapsed without a 2xx response, `givn run` exits 2. Anti-term: "grace period". |
| **Ready line** | The one stdout contract line `givn run: ready at http://127.0.0.1:<port><ready_path>`, emitted once after the first 2xx response, with a stable greppable prefix. Anti-terms: "banner", "status output". |
| **Start plan** | The resolved, printable form of the run declaration produced by `givn run --print`: one sh-eval-able line (`PORT=<port> <command>` with a port hand-off, else `<command>`). Anti-term: "run config". |
| **Backfill scaffold** | The managed-upgrade append of the run-declaration scaffold into an existing `givn/commands.yaml` when the `run:` key is absent; idempotent. Anti-term: "migration". |
| **Executor** | The givn subsystem behind `givn run` that resolves the start plan, spawns the product, probes readiness, forwards exit codes, and forwards signals on unix. Anti-terms: "supervisor", "daemon". |
| **Remediation log** | `givn/changes/<id>/semantic-remediation.md`, the append-only per-change artifact written by blocking `givn check review`; one entry per admitted candidate plus one review-run record per executed run, retained next to the semantic report under archive. Anti-terms: "remediation report", "history file". |
| **Remediation log entry** | The per-candidate log block: before state (scores, BGE/NLI evidence states, verbatim endpoint text), decision (classification and rationale), fix (remediation text and commits), and after state (verified end state and recomputed score). Anti-term: "candidate record". |
| **Review run record** | The log block for one executed semantic review run, anchored by its commit and recording per-stage durations (embedding recall, BGE, NLI); `null` only when a stage did not execute — a measured 0 is valid, including cache-only runs. Anti-term: "timing log". |
| **Git anchor** | The full 40-character SHA commit recorded in a run record or log entry field (`null` without Git history, including an unborn branch); it pins the log's historical facts to reproducible Git states. Anti-term: "short hash". |
| **Git stamp** | The footer line `semantic-remediation: <change-id>/<candidate-id>` in every remediation commit message, one line per fixed candidate; the review gate cross-validates it against the log entry. Anti-terms: "commit tag", "release stamp". |
| **Verbatim scenario text** | The exact source lines of an endpoint's Gherkin scenario, recorded 1:1 in the log entry with its content and evidence hashes; it is the reviewed text, not the model-internal serialization. Anti-terms: "normalized evidence", "scenario dump". |
| **Append-only log** | The remediation log's write discipline: entries are written once at run time and never regenerated; identical endpoint evidence is not duplicated, changed endpoints start a new entry. Anti-term: "rewrite log". |

### Run-declaration invariants

- A change cannot be reviewed as complete or archived while the run
  declaration has findings (enforcement site: `check review` mandatory check
  and the archive run gate).
- The sentinel start command is acceptable only for `product_type: library`
  (enforcement site: validation rule R1).
- A server declares exactly one port source — fixed `port` or `port_env`
  hand-off, never both, never neither (enforcement site: validation rules
  R4/R5; executor preflight for `--port`).
- givn never derives the product type or start command from repository
  content (enforcement site: product boundary — no derivation code exists).
