# 2. Architecture Constraints

These constraints are fixed decisions from the design Q&A. They are not open for
relitigation during implementation (see chapter 9 for rationale).

## 2.1 Technical Constraints

| ID | Constraint | Rationale (short) |
| --- | --- | --- |
| TC-1 | Implementation language is **Rust**. | Small static binaries, strong type safety for merge/state logic, author familiarity. |
| TC-2 | Ships as a **single static binary**. | Frictionless distribution; no runtime dependency. |
| TC-3 | **No npm**, **no root/sudo**, **no language runtime** required to install or run. | Direct fix of prior-tool pain points. |
| TC-4 | Project configuration lives in **one visible `givn/` directory**. | "All config in one place"; discoverable. |
| TC-5 | Defaults are **embedded in the binary**; project files override via cascade. | Upgrades never clobber user files. |
| TC-6 | Manifest format is **YAML**, parsed with **`serde_yaml_ng`**. | `serde_yaml` is deprecated; `serde_yaml_ng` is the maintained drop-in fork. |
| TC-7 | Gherkin parsing uses the **`gherkin` crate** (same parser cucumber-rs uses). | Proven on the gherkintest project; consistency with cucumber-rs runners. |
| TC-8 | CLI built with **`clap`**. | De-facto standard, derive API, help/usage generation. |
| TC-9 | Embedded assets via **`rust-embed`** (or `include_dir`). | Bake defaults into the binary. |
| TC-10 | Lifecycle state is **derived from the filesystem**; no separate state file. | No drift, git-friendly, stateless. |
| TC-11 | Delta tags use the `@givn.*` namespace. | Clean break from `@openspec.*`. |
| TC-12 | `givn lint` exit codes: **0** clean, **1** parse error, **2** `@wip`/PENDING. | Static Gherkin check; preserved CI-friendly contract. Distinct from the test runner (`verify.command`). |
| TC-13 | **Minimum dependency age:** every crate version in `Cargo.lock` (direct *and* transitive) must have been published on crates.io **≥ 30 days** ago. | Avoid brand-new releases that have had no time to surface bugs or supply-chain compromise. See 2.5. |
| TC-14 | Spec retrieval is an **opt-in Cargo feature**. The default release and musl builds must not require its native ONNX Runtime or model cache. | Preserve TC-2/TC-3/DC-2 while permitting local authoring retrieval with an explicit feature build. |
| TC-15 | The semantic similarity gate uses the private binary constant `0.95`; no project config, CLI flag, environment variable, or review note may raise or lower it. | Prevent an authoring agent from weakening the uniqueness gate to make its own review pass. |
| TC-16 | The opt-in BGE evidence layer uses a local multilingual ONNX pair model behind `cross-encoder-reranker`; no hosted inference is allowed. Its `3.0` raw-logit operating point admits a recommended worklist only; it is not a Givn classification or hard gate. | Keep evidence reproducible, local, and independent of external services while reducing false-positive worklists. |
| TC-17 | Blocking semantic review requires a retrieval-enabled local build; the default feature-free build must fail explicitly rather than claim semantic completeness. | Keep the opt-in native retrieval dependency explicit while ensuring the dogfood binary and review/archive workflow use `--features spec-retrieval`. |
| TC-18 | The complete permanent duplicate audit is advisory and read-only: it reads current permanent specs, excludes active changes and archive contents, never uses or writes `.spec-index.json`, and computes transient embeddings in memory. Existing local model-cache creation is allowed. | Preserve ordinary authoring and review state while giving maintainers a complete corpus view. |
| TC-19 | `cross-encoder-reranker` and `spec-similarity-gate` are independent feature capabilities. BGE uses deterministic E5 top-K unions and the positive `GIVN_SPEC_MAX_RERANK_PAIRS` budget (default 2048), selecting the highest E5 scores first and retaining the tail as unscored evidence. | Prevent the old BGE hard-gate behavior while avoiding all-pairs CPU inference. |
| TC-20 | `BGE_FILTERED` is a non-blocking evidence status in the semantic report. It is not a Givn classification and does not require a rationale; BGE-recommended candidates retain the normal agent-owned classification/remediation contract. | Reduce false-positive authoring pressure without automatic model decisions. |
| TC-21 | The opt-in `nli-evidence` layer uses the same eligible E5 pool and positive `GIVN_SPEC_MAX_RERANK_PAIRS` budget as BGE, scores both directions, and admits only a symmetric entailment probability of at least `0.5000`; NLI-filtered and budget-unscored evidence are non-blocking/model-free respectively and never a Givn classification. | Add an independent candidate-reduction signal without expanding CPU inference or allowing a model label to make a domain decision. |
| TC-22 | E5, BGE, and NLI consume one complete compact deterministic Gherkin semantic serialization. Its exact E5 passage text is the `content_hash` invalidation boundary; identity/hash matches reuse vectors, mismatches re-embed, and no separate serializer-version field is required. Empty optional fields are omitted; paths, tags, scores, classifications, and deterministic fingerprints remain separate metadata/evidence. | Prevent incomplete semantic context, synthetic boilerplate, and stale vectors while keeping model input distinct from audit metadata. |
| TC-23 | Before model construction or inference, the actual model tokenizer counts untruncated and unpadded E5 passages and BGE/NLI pair encodings, including pair special tokens. The limit is 512, exactly 512 fits, and 513 is over. | Prevent tokenizer truncation or padding from hiding over-cap semantic pairs. |
| TC-24 | BGE and NLI independently score the same E5 candidate pool. Combined recommendations intersect participating recommendations; per-candidate token-cap, unscored, or unavailable evidence is not silently filtered, and run-level unavailable is used only when that layer scores no pair. | Preserve comparable recall and blocking safety across second-stage layers. |
| TC-25 | Regular and E2E token-budget tests use a deterministic test-only seam; production uses real local tokenizers and artifacts. Tests do not download models, use network services, or require multi-gigabyte inference artifacts. | Keep acceptance deterministic and bounded without changing production model selection. |
| TC-26 | E5 passage preflight partitions complete inputs before embedding. A passage above 512 is metadata-only, never truncated or represented by a sentinel vector, while fitting passages continue through normal indexing and scoring. Cap findings are additive, visible, blocking, and receive no automatic Givn classification. | Preserve usable retrieval and honest semantic evidence when one complete scenario exceeds the E5 input window. |
| TC-27 | The complete semantic serializer omits optional sections and fields whose normalized content is empty. It retains every non-empty fachliche value in deterministic order and does not emit synthetic empty markers. | Prevent shared empty boilerplate from inflating cosine similarity or consuming model-token budget. |
| TC-28 | `GIVN_SPEC_MAX_RERANK_PAIRS` is unset-by-default at 2048 and accepts only explicit positive decimal values; empty, zero, negative, malformed, overflowing, and non-UTF-8 values fail before BGE/NLI inference. Selected candidates are ordered by E5 score, while budget-omitted candidates are `*_UNSCORED`, never `*_FILTERED`. | Make second-stage CPU work configurable without hiding unscored E5 candidates or confusing capacity with model judgment. |

## 2.2 Distribution Constraints

| ID | Constraint |
| --- | --- |
| DC-1 | Must be installable via: GitHub release binary, `curl \| sh` installer, Homebrew tap, and `cargo install givn`. |
| DC-2 | Target platforms: linux x64+arm64, macOS x64+arm64, windows x64. |
| DC-3 | Crate/binary name is `givn` (verified available on crates.io and Homebrew). |
| DC-4 | Release pipeline uses `cargo-dist` (or equivalent) for multi-platform artifacts. |

## 2.3 Organisational Constraints

| ID | Constraint |
| --- | --- |
| OC-1 | Repository lives at `~/projects/givn`, local `git init` only (no remote yet). |
| OC-2 | Clean break from OpenSpec: no backward-compat read of `openspec/` dirs. A `givn migrate` command is a deferred nice-to-have. |
| OC-3 | MVP scope includes the full stack: core engine + lifecycle + skills + install. |
| OC-4 | Open source under dual MIT OR Apache-2.0 license (see ADR-034). |

## 2.4 Conventions

| ID | Convention |
| --- | --- |
| CV-1 | `AGENTS.md` at repo root is user-owned; givn never writes or overwrites it. |
| CV-2 | Agent skills emitted to neutral `.agents/skills/<name>/SKILL.md` (primary) with `.claude/skills` and `.opencode/skills` mirrors. |
| CV-3 | Capability names use kebab-case (e.g. `user-auth`). |
| CV-4 | Test-runner feature path overridable via `GIVN_FEATURES` env var, default `givn/specs`. |
| CV-5 | Default `verify.command` is the sentinel `givn missing-testrunner` (prints setup guidance, exits non-zero) until the project sets a real runner. |
| CV-6 | `givn/config.yaml` ships fully commented with documented defaults; every key is discoverable in-file. |
| CV-7 | `givn/config.yaml` carries `givn_config_version`; `givn upgrade` provides a version-to-version upgrade path, with git as the diff/merge surface. |
| CV-8 | Artifacts may declare `post` hooks (built-in `verify`/`lint` or shell). Defaults: `tasks.post: [verify]`, `review.post: [verify]`. `givn check` runs them; `givn archive` runs the hooks of `apply.requires` artifacts as the gate. Failing hooks block (unless `allow_failure`). |

## 2.5 Dependency Age Policy (TC-13)

**Rule.** Every crate version pinned in `Cargo.lock` — direct *and* transitive —
must have been published on crates.io at least **30 days** ago.

**Why.** Cargo has **no native minimum-dependency-age feature** (no `--max-age`
flag; `-Z minimal-versions` is about minimum *version*, not *age*). Brand-new
releases are the highest-risk window for both ordinary regressions and
supply-chain compromise. A 30-day soak requirement lets issues surface before we
adopt a release.

**Enforcement.** A checked-in script, [`scripts/check-dep-age.sh`](../../scripts/check-dep-age.sh),
parses `Cargo.lock`, queries each crate version's `created_at` from the crates.io
API, and fails (exit 1) if any registry dependency is younger than the threshold.

- Threshold configurable via `MIN_AGE_DAYS` (default 30).
- Local `path`/`git` dependencies and the workspace's own crates are skipped
  (only `registry+…` sources are checked).
- Rare, justified exceptions go in `GIVN_DEP_AGE_ALLOWLIST` (`name@version` list).
- Intended to run as: (a) a CI gate, and (b) optionally a givn `post` hook on a
  dev/release artifact. Failure blocks the gate.

**Remediation when it fails.** Pin an older version
(`cargo update -p <crate> --precise <older>`), wait until the release ages past
the threshold, or allowlist with explicit justification.

## 2.6 Acceptance-boundary constraints

Repository-owned workflow acceptance runs through the compiled Givn CLI in an
isolated temporary project. Fixtures use only local deterministic commands and
must not require a consumer checkout, provider credential, network service, or
shared filesystem. The active change path is supplied through `GIVN_FEATURES`
so the same Gherkin runner can execute the delta during lifecycle gates.
