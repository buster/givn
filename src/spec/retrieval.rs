use std::path::Path;

use anyhow::{Result, bail};

#[cfg(feature = "spec-retrieval")]
use anyhow::Context;

#[cfg(feature = "nli-evidence")]
use ndarray::Array;

#[cfg(feature = "spec-retrieval")]
use std::collections::{BTreeMap, BTreeSet};

#[cfg(all(feature = "spec-retrieval", feature = "spec-similarity-gate"))]
use std::collections::HashSet;

#[cfg(feature = "spec-retrieval")]
use std::fs;

#[cfg(feature = "spec-retrieval")]
use std::io::{Read, Write};

#[cfg(feature = "spec-retrieval")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "spec-retrieval")]
use sha2::{Digest, Sha256};
#[cfg(feature = "spec-retrieval")]
use tokenizers::Tokenizer;

#[cfg(feature = "spec-retrieval")]
const INDEX_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const MODEL_ID: &str = "intfloat/multilingual-e5-large";
#[cfg(feature = "spec-retrieval")]
const E5_TOKENIZER_ID: &str = "intfloat/multilingual-e5-large/tokenizer.json";
#[cfg(feature = "spec-retrieval")]
const E5_MODEL_REPOSITORY: &str = "Qdrant/multilingual-e5-large-onnx";
#[cfg(feature = "spec-retrieval")]
const E5_MAX_LENGTH: usize = 512;
#[cfg(feature = "spec-retrieval")]
const RERANKER_MODEL_ID: &str = "rozgo/bge-reranker-v2-m3";
#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
const RERANKER_LOGIT_THRESHOLD: f32 = 3.0;
#[cfg(feature = "spec-retrieval")]
const BGE_ADMISSION_THRESHOLD: f32 = 3.0;
#[cfg(feature = "spec-retrieval")]
const E5_ADMISSION_THRESHOLD: f32 = 0.9;

/// The permanent↔permanent duplicate audit keeps its own binary-fixed
/// 0.95 boundary (out of scope for the unified change-scoped rulebook).
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_AUDIT_THRESHOLD: f32 = 0.95;

/// Unified rulebook selection: at most this many highest-scored neighbours
/// per delta scenario survive the per-scenario cut (ADR-066).
const NEIGHBOURS_PER_SCENARIO: usize = 5;

/// Unified rulebook selection: the shared cap on judged pairs per run
/// (ADR-066). Pairs beyond the cap are unselected, not candidates.
const PAIR_CAP: usize = 20;

#[cfg(feature = "spec-retrieval")]
/// Schema version of the persisted pair-score cache.
const PAIR_CACHE_VERSION: u32 = 1;

#[cfg(feature = "spec-retrieval")]
/// Stub stages record this as their cache model id so stub and real-model
/// entries can never alias.
const PAIR_CACHE_STUB_MODEL: &str = "stub";

#[cfg(feature = "spec-retrieval")]
/// Per-run pair-judgement accounting (one pair is reused only when every
/// participating stage hit the cache for it).
#[derive(Debug, Default, Clone, Copy)]
struct PairCacheAccounting {
    reused: usize,
    computed: usize,
}

#[cfg(feature = "spec-retrieval")]
impl PairCacheAccounting {
    fn record(&mut self, reused: bool) {
        if reused {
            self.reused += 1;
        } else {
            self.computed += 1;
        }
    }
}

#[cfg(feature = "spec-retrieval")]
/// Persisted per-pair model evidence keyed by stage, model, and both
/// endpoints' content/evidence hashes (ADR-066). Lives next to the spec
/// index; hash-identical pairs reuse recorded judgements.
#[derive(Debug, Serialize, Deserialize)]
struct PairScoreCache {
    version: u32,
    entries: BTreeMap<String, serde_json::Value>,
}

#[cfg(feature = "spec-retrieval")]
impl PairScoreCache {
    fn load(root: &Path) -> Self {
        let path = pair_cache_path(root);
        let Ok(raw) = std::fs::read(&path) else {
            return Self::default_cache();
        };
        match serde_json::from_slice::<PairScoreCache>(&raw) {
            Ok(cache) if cache.version == PAIR_CACHE_VERSION => cache,
            _ => {
                // Fail safe: an unreadable or outdated cache is rebuilt; it
                // never changes verdicts, only re-judges.
                Self::default_cache()
            }
        }
    }

    fn default_cache() -> Self {
        Self {
            version: PAIR_CACHE_VERSION,
            entries: BTreeMap::new(),
        }
    }

    fn get_raw(&self, key: &str) -> Option<&serde_json::Value> {
        self.entries.get(key)
    }

    fn put_raw(&mut self, key: String, value: serde_json::Value) {
        self.entries.insert(key, value);
    }

    fn store(&self, root: &Path) -> Result<()> {
        let path = pair_cache_path(root);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let body = serde_json::to_vec(self)?;
        std::fs::write(&path, body)?;
        Ok(())
    }
}

#[cfg(feature = "spec-retrieval")]
fn pair_cache_path(root: &Path) -> std::path::PathBuf {
    root.join("givn/.spec-pair-cache.json")
}

#[cfg(feature = "spec-retrieval")]
/// Combine the per-stage pair hit markers into per-pair accounting: a pair
/// is reused only when every participating stage hit the cache for it.
fn combine_pair_cache_accounting(
    bge_hits: Option<&Vec<(String, bool)>>,
    nli_hits: Option<&Vec<(String, bool)>>,
) -> PairCacheAccounting {
    let mut by_candidate: BTreeMap<String, (bool, bool)> = BTreeMap::new();
    if let Some(hits) = bge_hits {
        for (candidate_id, hit) in hits {
            by_candidate
                .entry(candidate_id.clone())
                .or_insert((false, false))
                .0 = *hit;
        }
    }
    if let Some(hits) = nli_hits {
        for (candidate_id, hit) in hits {
            by_candidate
                .entry(candidate_id.clone())
                .or_insert((false, false))
                .1 = *hit;
        }
    }
    let mut accounting = PairCacheAccounting::default();
    for (_, (bge_hit, nli_hit)) in by_candidate {
        accounting.record(bge_hit && nli_hit);
    }
    accounting
}

#[cfg(feature = "spec-retrieval")]
/// Cache key over stage, model identity, and both endpoints' content and
/// evidence hashes. Any endpoint change misses and re-judges.
fn pair_cache_key(
    stage: &str,
    model_id: &str,
    left: &SemanticEndpoint,
    right: &SemanticEndpoint,
) -> String {
    format!(
        "{stage}|{model_id}|{}|{}|{}|{}|{}|{}|{}|{}",
        left.file,
        left.title,
        left.content_hash,
        left.evidence_hash,
        right.file,
        right.title,
        right.content_hash,
        right.evidence_hash,
    )
}
#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
const BGE_PIPELINE_VERSION: &str = "rust-production-v2";
#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
const DEFAULT_MAX_RERANK_PAIRS: usize = 2048;
#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
const MAX_RERANK_PAIRS_ENV: &str = "GIVN_SPEC_MAX_RERANK_PAIRS";
#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
const BGE_MAX_LENGTH: usize = 512;
#[cfg(feature = "spec-retrieval")]
const RERANKER_TOP_K: usize = 5;
#[cfg(feature = "nli-evidence")]
const NLI_MODEL_ID: &str = "MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7";
#[cfg(feature = "cross-encoder-reranker")]
const BGE_MODEL_REVISION: &str = "fbd57b17b4db111a9d16813bb08b4c804fac18e9";
#[cfg(feature = "nli-evidence")]
const NLI_MODEL_REVISION: &str = "b5113eb38ab63efdd7f280f8c144ea8b13f978ce";
#[cfg(feature = "spec-retrieval")]
const ARTIFACT_MANIFEST_NAME: &str = "givn-artifacts.json";
#[cfg(feature = "spec-retrieval")]
const ARTIFACT_MANIFEST_VERSION: u32 = 1;
#[cfg(feature = "nli-evidence")]
const NLI_STUB_SELECTOR: &str = "nli-stub";
#[cfg(feature = "nli-evidence")]
const NLI_REPORT_LIMIT: usize = 20;
#[cfg(feature = "nli-evidence")]
const NLI_ADMISSION_THRESHOLD: f32 = 0.5;
#[cfg(feature = "nli-evidence")]
const SEMANTIC_NLI_REPORT_SCHEMA_VERSION: u32 = 3;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_REPORT_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_BGE_REPORT_SCHEMA_VERSION: u32 = 2;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_FINGERPRINT_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_EVIDENCE_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
/// Single change-scoped E5 admission boundary (unified rulebook, ADR-066).
#[cfg(feature = "spec-retrieval")]
const DUPLICATE_REPORT_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const DUPLICATE_BGE_REPORT_SCHEMA_VERSION: u32 = 2;
#[cfg(feature = "spec-retrieval")]
const TOKEN_CAP_GUIDANCE: &str = "shorten or split the scenarios, then rerun semantic review";
#[cfg(feature = "spec-retrieval")]
const E5_TOKEN_CAP_GUIDANCE: &str = "shorten or split the scenario, then rerun retrieval";
#[cfg(feature = "spec-retrieval")]
const DUPLICATE_E5_TOKEN_CAP_GUIDANCE: &str =
    "shorten or split the scenario, then rerun the duplicate audit";

/// Whether `GIVN_SPEC_*` fixture requests are honoured. Development builds
/// (compiled with debug assertions) honour them; installed/release builds
/// ignore the gate-bypassing fixture requests and always evaluate the
/// semantic gates with the real models.
#[cfg(feature = "spec-retrieval")]
const ALLOW_SPEC_OVERRIDES: bool = cfg!(debug_assertions);

/// Every `GIVN_SPEC_*` request channel the installed-build strictness can
/// ignore, listed so the one-shot announcement reports a blocked request even
/// when the running command never reads that channel.
#[cfg(feature = "spec-retrieval")]
const REQUEST_CHANNELS: &[&str] = &[
    "GIVN_SPEC_EMBEDDER",
    "GIVN_SPEC_BGE",
    "GIVN_SPEC_RERANKER",
    "GIVN_SPEC_BGE_APPROVAL",
    "GIVN_SPEC_BGE_LOCAL_STUB",
    "GIVN_SPEC_NLI_LOCAL_STUB",
    "GIVN_SPEC_BGE_TEST_POOL",
    "GIVN_SPEC_NLI_TEST_POOL",
    "GIVN_SPEC_BGE_DIRECTIONAL_SCORES",
    "GIVN_SPEC_NLI_DIRECTIONAL_SCORES",
    "GIVN_SPEC_STUB_PAIR_SCORES",
    "GIVN_SPEC_DUPLICATES_TEST_SCORES",
    "GIVN_SPEC_E5_TOKEN_BUDGET",
    "GIVN_SPEC_BGE_TOKEN_BUDGET",
    "GIVN_SPEC_NLI_TOKEN_BUDGET",
    "GIVN_SPEC_TOKEN_BUDGET",
    "GIVN_SPEC_NLI_REPORT_LIMIT",
    "GIVN_SPEC_MAX_RERANK_PAIRS",
    "GIVN_SPEC_RESOLUTION_NOTICES",
    "GIVN_SPEC_DOWNLOAD_EVENTS",
];

/// Whether a `GIVN_SPEC_*` request is ignored in an installed build.
/// Value-aware for the selector channels and by-name for the fixture-only
/// channels. Development builds never ignore anything.
#[cfg(feature = "spec-retrieval")]
fn release_blocked(name: &str, value: &str) -> bool {
    if ALLOW_SPEC_OVERRIDES {
        return false;
    }
    match name {
        // The embedder stage has no real-model selector: every value swaps
        // the real E5 output for simulated output.
        "GIVN_SPEC_EMBEDDER" => true,
        // The second-stage selectors distinguish fixture requests
        // (simulated/disabled output) from real-model location requests
        // (`auto`/`local` for BGE, `nli`/`nli-auto` for NLI).
        "GIVN_SPEC_BGE" => matches!(value, "stub" | "disabled" | "missing"),
        "GIVN_SPEC_RERANKER" => {
            matches!(
                value,
                "stub" | "disabled" | "missing" | "nli-stub" | "nli-missing"
            )
        }
        "GIVN_SPEC_BGE_APPROVAL" => value == "failed",
        // Fixture-only channels: any present value substitutes or re-sizes
        // the model output, so they are blocked by name.
        "GIVN_SPEC_BGE_LOCAL_STUB"
        | "GIVN_SPEC_NLI_LOCAL_STUB"
        | "GIVN_SPEC_BGE_TEST_POOL"
        | "GIVN_SPEC_NLI_TEST_POOL"
        | "GIVN_SPEC_BGE_DIRECTIONAL_SCORES"
        | "GIVN_SPEC_NLI_DIRECTIONAL_SCORES"
        | "GIVN_SPEC_STUB_PAIR_SCORES"
        | "GIVN_SPEC_DUPLICATES_TEST_SCORES"
        | "GIVN_SPEC_E5_TOKEN_BUDGET"
        | "GIVN_SPEC_BGE_TOKEN_BUDGET"
        | "GIVN_SPEC_NLI_TOKEN_BUDGET"
        | "GIVN_SPEC_TOKEN_BUDGET"
        | "GIVN_SPEC_NLI_REPORT_LIMIT"
        | "GIVN_SPEC_MAX_RERANK_PAIRS"
        | "GIVN_SPEC_RESOLUTION_NOTICES"
        | "GIVN_SPEC_DOWNLOAD_EVENTS" => true,
        _ => false,
    }
}

/// Gated `GIVN_SPEC_*` read returning `OsString`, mirroring
/// `std::env::var_os`. An ignored installed-build request resolves to `None`
/// so the existing `let-else` callers fall through to the real-model path.
#[cfg(feature = "spec-retrieval")]
fn spec_env(name: &str) -> Option<std::ffi::OsString> {
    let raw = std::env::var_os(name)?;
    if raw
        .to_str()
        .is_some_and(|value| release_blocked(name, value))
    {
        return None;
    }
    Some(raw)
}

/// Gated `GIVN_SPEC_*` read returning `String`, mirroring `std::env::var`.
/// An ignored installed-build request resolves to
/// `Err(VarError::NotPresent)` (never an empty string) so the existing
/// `Err(NotPresent) => real-model` match arms are reused verbatim.
#[cfg(feature = "spec-retrieval")]
fn spec_env_var(name: &str) -> Result<String, std::env::VarError> {
    match std::env::var(name) {
        Err(error) => Err(error),
        Ok(value) if release_blocked(name, &value) => Err(std::env::VarError::NotPresent),
        Ok(value) => Ok(value),
    }
}

#[cfg(feature = "spec-retrieval")]
static RELEASE_OVERRIDES_REPORTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// One-shot announcement of every fixture request this installed build
/// ignores. Called at the start of each semantic command, so a request is
/// announced even when the command itself never reads that channel (e.g.
/// `givn spec index` announces a blocked BGE request). No-op in development
/// builds and after the first call in any installed process.
#[cfg(feature = "spec-retrieval")]
pub fn report_ignored_release_overrides() {
    use std::sync::atomic::Ordering;
    if ALLOW_SPEC_OVERRIDES || RELEASE_OVERRIDES_REPORTED.swap(true, Ordering::Relaxed) {
        return;
    }
    for name in REQUEST_CHANNELS {
        if std::env::var(name)
            .ok()
            .is_some_and(|value| release_blocked(name, &value))
        {
            notice(format!(
                "{name} is ignored in installed builds; the real model output is always used"
            ));
        }
    }
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn max_rerank_pairs() -> Result<usize> {
    let Some(raw) = spec_env(MAX_RERANK_PAIRS_ENV) else {
        return Ok(DEFAULT_MAX_RERANK_PAIRS);
    };
    let raw = raw
        .into_string()
        .map_err(|_| anyhow::anyhow!("{MAX_RERANK_PAIRS_ENV} is not valid UTF-8"))?;
    let value = raw.parse::<usize>().with_context(|| {
        format!("{MAX_RERANK_PAIRS_ENV} must be a positive decimal integer, got `{raw}`")
    })?;
    anyhow::ensure!(
        value > 0,
        "{MAX_RERANK_PAIRS_ENV} must be greater than zero"
    );
    Ok(value)
}

#[cfg(feature = "spec-retrieval")]
const THREADS_ENV: &str = "GIVN_SPEC_THREADS";

/// Resolve the compute-thread budget shared by every model stage. When
/// `GIVN_SPEC_THREADS` is unset, the machine's available parallelism is
/// used (quota-aware on Linux). Malformed or zero values are rejected here,
/// naming the setting, before any model artifact is resolved. Stages run
/// sequentially, so the full budget is never contended between them; if
/// inter-stage overlap is ever introduced, this budget must be split.
#[cfg(feature = "spec-retrieval")]
fn resolve_thread_budget() -> Result<usize> {
    let Some(raw) = spec_env(THREADS_ENV) else {
        return Ok(std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get));
    };
    let raw = raw
        .into_string()
        .map_err(|_| anyhow::anyhow!("{THREADS_ENV} is not valid UTF-8"))?;
    let value = raw.parse::<usize>().with_context(|| {
        format!("{THREADS_ENV} must be a positive decimal integer, got `{raw}`")
    })?;
    anyhow::ensure!(value > 0, "{THREADS_ENV} must be greater than zero");
    Ok(value)
}

#[cfg(feature = "spec-retrieval")]
fn expose_e5_cap_contract(caps: &[E5TokenCap]) -> bool {
    !caps.is_empty() || spec_env("GIVN_SPEC_E5_TOKEN_BUDGET").is_some()
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpecIndex {
    version: u32,
    model: String,
    entries: Vec<IndexEntry>,
    #[serde(default)]
    e5_token_caps: Vec<E5TokenCap>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct IndexRefresh {
    index: SpecIndex,
    reused: usize,
    embedded: usize,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct E5TokenCap {
    status: String,
    file: String,
    title: String,
    content_hash: String,
    token_count: usize,
    limit: usize,
    model: String,
    tokenizer: String,
    refactoring_required: bool,
    guidance: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy)]
struct E5TokenBudget {
    token_count: usize,
    limit: usize,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct EmbeddingBatch {
    vectors: Vec<Option<Vec<f32>>>,
    token_caps: Vec<Option<E5TokenBudget>>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
pub(crate) struct LogCandidateInput {
    pub candidate_id: String,
    pub embedding_score: f32,
    pub bge_admission: Option<String>,
    pub bge_symmetric_raw: Option<f32>,
    pub nli_admission: Option<String>,
    pub nli_symmetric_entailment: Option<f32>,
    pub left_file: String,
    pub left_title: String,
    pub right_file: String,
    pub right_title: String,
    pub left_content_hash: String,
    pub right_content_hash: String,
    pub left_evidence_hash: String,
    pub right_evidence_hash: String,
    pub left_gherkin: Option<String>,
    pub right_gherkin: Option<String>,
    pub classification: String,
    pub rationale: Option<String>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
pub(crate) struct VerifiedRemediation {
    pub candidate_id: String,
    pub left_evidence_hash: String,
    pub right_evidence_hash: String,
    pub classification: String,
    pub recomputed_score: Option<f32>,
    pub left_file: String,
    pub left_title: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct SemanticCandidateSet {
    candidates: Vec<SemanticCandidate>,
    e5_token_caps: Vec<E5TokenCap>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexEntry {
    title: String,
    file: String,
    content_hash: String,
    embedding: Vec<f32>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct ScenarioText {
    title: String,
    file: String,
    content_hash: String,
    text: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct ExploreQuery {
    title: String,
    source: &'static str,
    file: Option<String>,
    content_hash: String,
    text: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct ExploreResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_version: Option<u32>,
    query: ExploreQueryOutput,
    index: ExploreIndexOutput,
    results: Vec<ExploreResultOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e5_token_cap_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e5_token_cap_candidates: Option<Vec<E5TokenCap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<ExploreEvidenceOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recommended_results: Option<Vec<ExploreResultOutput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nli_evidence: Option<ExploreNliEvidenceOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nli_recommended_results: Option<Vec<ExploreResultOutput>>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct ExploreQueryOutput {
    title: String,
    source: &'static str,
    file: Option<String>,
    content_hash: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct ExploreIndexOutput {
    model: String,
    version: u32,
    entries: usize,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct ExploreResultOutput {
    rank: usize,
    title: String,
    file: String,
    score: f32,
    scope: &'static str,
    change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bge: Option<ExploreBgeOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bge_token_cap: Option<ExploreTokenCapOutput>,
    #[cfg(feature = "nli-evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nli: Option<ExploreNliOutput>,
    #[cfg(feature = "nli-evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nli_token_cap: Option<ExploreTokenCapOutput>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct ExploreBgeOutput {
    status: &'static str,
    forward_raw: f32,
    reverse_raw: f32,
    symmetric_raw: f32,
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone, Serialize)]
struct ExploreNliOutput {
    status: &'static str,
    forward: [f32; 3],
    reverse: [f32; 3],
    symmetric: [f32; 3],
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct ExploreTokenCapOutput {
    status: &'static str,
    left_tokens: usize,
    right_tokens: usize,
    pair_tokens: usize,
    limit: usize,
    refactoring_required: bool,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct ExploreNliEvidenceOutput {
    status: &'static str,
    reason: Option<&'static str>,
    candidate_generator: &'static str,
    top_k: usize,
    max_unique_pairs: usize,
    e5_candidates: usize,
    recommended_candidates: usize,
    nli_filtered_candidates: usize,
    nli_unscored_candidates: usize,
    nli_token_cap_candidates: usize,
    refactoring_required: bool,
    nli_pairs: usize,
    nli_directional_calls: usize,
    nli_model: Option<&'static str>,
    nli_model_load_attempts: usize,
    nli_runtime_ms: Option<u128>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct ExploreEvidenceOutput {
    status: &'static str,
    reason: Option<&'static str>,
    candidate_generator: &'static str,
    top_k: usize,
    max_unique_pairs: usize,
    e5_candidates: usize,
    recommended_candidates: usize,
    bge_filtered_candidates: usize,
    bge_unscored_candidates: usize,
    bge_token_cap_candidates: usize,
    refactoring_required: bool,
    bge_pairs: usize,
    bge_directional_calls: usize,
    bge_model: Option<&'static str>,
    bge_model_load_attempts: usize,
    bge_runtime_ms: Option<u128>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct DeltaScenario {
    scenario: ScenarioText,
    target_file: String,
    operation: crate::gherkin::delta::DeltaOp,
    facts: crate::overlap::fingerprint::Fingerprint,
    evidence_json: String,
    is_outline: bool,
    has_examples: bool,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct SemanticEndpoint {
    scope: &'static str,
    file: String,
    title: String,
    content_hash: String,
    evidence_hash: String,
    evidence_json: String,
    operation: String,
    projected_target: Option<(String, String)>,
    facts: crate::overlap::fingerprint::Fingerprint,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct SemanticCandidate {
    scope: &'static str,
    left: SemanticEndpoint,
    right: SemanticEndpoint,
    score: f32,
    candidate_id: String,
    classification: String,
    rationale: Option<String>,
    bge: Option<BgePairEvidence>,
    bge_admission: Option<&'static str>,
    bge_token_cap: Option<PairTokenBudget>,
    #[cfg(feature = "nli-evidence")]
    nli: Option<NliEvidence>,
    #[cfg(feature = "nli-evidence")]
    nli_admission: Option<&'static str>,
    #[cfg(feature = "nli-evidence")]
    nli_token_cap: Option<PairTokenBudget>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct PriorRemediationEntry {
    candidate_id: String,
    classification: String,
    rationale: String,
    left_scope: String,
    left_file: String,
    left_title: String,
    left_content_hash: String,
    left_evidence_hash: String,
    right_scope: String,
    right_file: String,
    right_title: String,
    right_content_hash: String,
    right_evidence_hash: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct ExploreCandidate {
    title: String,
    file: String,
    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    text: String,
    embedding: Vec<f32>,
    scope: &'static str,
    change: Option<String>,
    projected_file: Option<String>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SimilarityEndpoint {
    file: String,
    title: String,
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
type SimilarityPair = (SimilarityEndpoint, SimilarityEndpoint);

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
type SimilarityTexts = BTreeMap<(String, String), String>;

#[cfg(feature = "nli-evidence")]
type NliDirectionalOverrides = BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), [f32; 3]>;

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct SimilarityCandidate {
    left: SimilarityEndpoint,
    right: SimilarityEndpoint,
    embedding_score: f32,
    bge: Option<BgePairEvidence>,
    bge_admission: Option<&'static str>,
    #[cfg(feature = "nli-evidence")]
    nli: Option<NliEvidence>,
    #[cfg(feature = "nli-evidence")]
    nli_admission: Option<&'static str>,
    #[cfg(feature = "nli-evidence")]
    nli_token_cap: Option<PairTokenBudget>,
    scopes: Vec<&'static str>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct ScenarioRecord {
    endpoint: SimilarityEndpoint,
    embedding: Vec<f32>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateEndpoint {
    file: String,
    title: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct DuplicateScenario {
    endpoint: DuplicateEndpoint,
    behavior: String,
    fingerprint: crate::overlap::fingerprint::Fingerprint,
    text: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateAssertionShape {
    left: Vec<String>,
    right: Vec<String>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateFacts {
    exit_code: Option<String>,
    contains_polarity: BTreeMap<usize, String>,
    tables: BTreeMap<String, String>,
    done: bool,
    tty: bool,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateFactsEvidence {
    left: DuplicateFacts,
    right: DuplicateFacts,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DistinguishingFact {
    field: String,
    left: serde_json::Value,
    right: serde_json::Value,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct DuplicateCandidate {
    kind: &'static str,
    left: DuplicateEndpoint,
    right: DuplicateEndpoint,
    relation: &'static str,
    assertion_shape: Option<DuplicateAssertionShape>,
    facts: Option<DuplicateFactsEvidence>,
    distinguishing_facts: Vec<DistinguishingFact>,
    fact_contradiction: bool,
    score: Option<f32>,
    bge: Option<BgePairEvidence>,
    bge_admission: Option<&'static str>,
    bge_token_cap: Option<PairTokenBudget>,
    #[cfg(feature = "nli-evidence")]
    nli: Option<NliEvidence>,
    #[cfg(feature = "nli-evidence")]
    nli_admission: Option<&'static str>,
    #[cfg(feature = "nli-evidence")]
    nli_token_cap: Option<PairTokenBudget>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct DuplicateAudit {
    candidates: Vec<DuplicateCandidate>,
    e5_token_caps: Vec<E5TokenCap>,
    filtered_candidates: Vec<DuplicateCandidate>,
    unscored_candidates: Vec<DuplicateCandidate>,
    bge_run: Option<BgeRun>,
    #[cfg(feature = "nli-evidence")]
    nli_filtered_candidates: Vec<DuplicateCandidate>,
    #[cfg(feature = "nli-evidence")]
    nli_unscored_candidates: Vec<DuplicateCandidate>,
    #[cfg(feature = "nli-evidence")]
    nli_run: Option<NliRun>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct DuplicateCandidateSet {
    candidates: Vec<DuplicateCandidate>,
    e5_token_caps: Vec<E5TokenCap>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateJsonEvidence {
    relation: &'static str,
    assertion_shape: Option<DuplicateAssertionShape>,
    facts: Option<DuplicateFactsEvidence>,
    distinguishing_facts: Vec<DistinguishingFact>,
    fact_contradiction: bool,
    score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bge: Option<DuplicateJsonBgeEvidence>,
    #[cfg(feature = "nli-evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nli: Option<DuplicateJsonNliEvidence>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateJsonBgeEvidence {
    status: &'static str,
    forward_raw: Option<f32>,
    reverse_raw: Option<f32>,
    symmetric_raw: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pair_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refactoring_required: Option<bool>,
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateJsonNliEvidence {
    status: &'static str,
    forward: Option<[f32; 3]>,
    reverse: Option<[f32; 3]>,
    symmetric: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pair_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refactoring_required: Option<bool>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateJsonCandidate {
    rank: usize,
    kind: &'static str,
    left: DuplicateEndpoint,
    right: DuplicateEndpoint,
    evidence: DuplicateJsonEvidence,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct DuplicateJsonReport {
    schema_version: u32,
    scanned_scenarios: usize,
    candidate_count: usize,
    candidates: Vec<DuplicateJsonCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e5_token_cap_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e5_token_cap_candidates: Option<Vec<E5TokenCap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e5_candidate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bge_run: Option<DuplicateJsonRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filtered_candidates: Option<Vec<DuplicateJsonCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unscored_candidates: Option<Vec<DuplicateJsonCandidate>>,
    #[cfg(feature = "nli-evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nli_run: Option<DuplicateJsonNliRun>,
    #[cfg(feature = "nli-evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nli_filtered_candidates: Option<Vec<DuplicateJsonCandidate>>,
    #[cfg(feature = "nli-evidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nli_unscored_candidates: Option<Vec<DuplicateJsonCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    combined_recommended_candidates: Option<Vec<DuplicateJsonCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refactoring_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: Option<&'static str>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct DuplicateJsonRun {
    status: &'static str,
    reason: Option<&'static str>,
    model: Option<&'static str>,
    top_k: usize,
    max_unique_pairs: usize,
    e5_candidates: usize,
    unique_pairs: usize,
    directional_calls: usize,
    recommended_count: usize,
    filtered_count: usize,
    unscored_count: usize,
    model_load_attempts: usize,
    runtime_ms: Option<u128>,
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Serialize)]
struct DuplicateJsonNliRun {
    status: &'static str,
    reason: Option<&'static str>,
    model: Option<&'static str>,
    top_k: usize,
    max_unique_pairs: usize,
    e5_candidates: usize,
    unique_pairs: usize,
    directional_calls: usize,
    recommended_count: usize,
    filtered_count: usize,
    unscored_count: usize,
    model_load_attempts: usize,
    runtime_ms: Option<u128>,
}

#[cfg(feature = "spec-retrieval")]
type DuplicatePairKey = ((String, String), (String, String));

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone, Copy)]
enum NliBackend {
    Stub,
    Local,
    Auto,
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone, Copy)]
struct NliEvidence {
    forward: [f32; 3],
    reverse: [f32; 3],
    symmetric: [f32; 3],
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NliUnavailableReason {
    MissingArtifact,
    TokenCap,
}

#[cfg(feature = "nli-evidence")]
impl NliUnavailableReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::MissingArtifact => "missing-artifact",
            Self::TokenCap => "token-cap",
        }
    }
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone)]
struct NliRun {
    status: &'static str,
    reason: Option<NliUnavailableReason>,
    evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), NliEvidence>,
    token_caps: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), PairTokenBudget>,
    e5_candidates: usize,
    unscored: usize,
    max_unique_pairs: usize,
    model_load_attempts: usize,
    runtime_ms: Option<u128>,
    /// Per judged pair: (candidate id, cache hit). Filled by the rulebook's
    /// NLI application; empty on non-rulebook paths.
    pair_cache_hits: Vec<(String, bool)>,
}

#[cfg(feature = "nli-evidence")]
impl NliRun {
    fn enabled(
        evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), NliEvidence>,
        e5_candidates: usize,
        max_unique_pairs: usize,
        runtime_ms: u128,
    ) -> Self {
        Self {
            status: "NLI_ENABLED",
            reason: None,
            unscored: e5_candidates.saturating_sub(evidence.len()),
            evidence,
            token_caps: BTreeMap::new(),
            e5_candidates,
            model_load_attempts: 1,
            max_unique_pairs,
            runtime_ms: Some(runtime_ms),
            pair_cache_hits: Vec::new(),
        }
    }

    fn unavailable(
        reason: NliUnavailableReason,
        e5_candidates: usize,
        max_unique_pairs: usize,
    ) -> Self {
        Self {
            status: "NLI_UNAVAILABLE",
            reason: Some(reason),
            evidence: BTreeMap::new(),
            token_caps: BTreeMap::new(),
            e5_candidates,
            unscored: e5_candidates,
            max_unique_pairs,
            model_load_attempts: 0,
            runtime_ms: None,
            pair_cache_hits: Vec::new(),
        }
    }
}

#[cfg(feature = "spec-retrieval")]
type CandidatePool = (
    Vec<SimilarityCandidate>,
    usize,
    BTreeMap<(String, String), String>,
    Vec<E5TokenCap>,
);

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BgeBackendKind {
    Stub,
    Local,
    Auto,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BgeUnavailableReason {
    #[cfg(feature = "cross-encoder-reranker")]
    MissingArtifact,
    TokenCap,
}

#[cfg(feature = "spec-retrieval")]
impl BgeUnavailableReason {
    fn as_str(self) -> &'static str {
        match self {
            #[cfg(feature = "cross-encoder-reranker")]
            Self::MissingArtifact => "missing-artifact",
            Self::TokenCap => "token-cap",
        }
    }
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct BgePairEvidence {
    forward_raw: f32,
    reverse_raw: f32,
    symmetric_raw: f32,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy, Serialize)]
struct PairTokenBudget {
    left_tokens: usize,
    right_tokens: usize,
    pair_tokens: usize,
    limit: usize,
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
#[derive(Debug, Clone, Copy)]
enum TokenBudgetModel {
    #[cfg(feature = "cross-encoder-reranker")]
    Bge,
    #[cfg(feature = "nli-evidence")]
    Nli,
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
#[derive(Debug, Default)]
struct PairBudgetPartition {
    fitting: Vec<(SimilarityEndpoint, SimilarityEndpoint)>,
    capped: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), PairTokenBudget>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct BgeRun {
    status: &'static str,
    reason: Option<BgeUnavailableReason>,
    evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), BgePairEvidence>,
    token_caps: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), PairTokenBudget>,
    e5_candidates: usize,
    unscored: usize,
    max_unique_pairs: usize,
    model_load_attempts: usize,
    runtime_ms: Option<u128>,
    /// Per judged pair: (candidate id, cache hit). Filled by the rulebook's
    /// BGE application; empty on non-rulebook paths.
    pair_cache_hits: Vec<(String, bool)>,
}

#[cfg(feature = "spec-retrieval")]
impl BgeRun {
    #[cfg(feature = "cross-encoder-reranker")]
    fn enabled(
        evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), BgePairEvidence>,
        e5_candidates: usize,
        max_unique_pairs: usize,
        runtime_ms: u128,
    ) -> Self {
        Self {
            status: "BGE_ENABLED",
            reason: None,
            unscored: e5_candidates.saturating_sub(evidence.len()),
            evidence,
            token_caps: BTreeMap::new(),
            e5_candidates,
            model_load_attempts: 1,
            max_unique_pairs,
            runtime_ms: Some(runtime_ms),
            pair_cache_hits: Vec::new(),
        }
    }

    #[cfg(feature = "cross-encoder-reranker")]
    fn unavailable(
        reason: BgeUnavailableReason,
        e5_candidates: usize,
        max_unique_pairs: usize,
    ) -> Self {
        Self {
            status: "BGE_UNAVAILABLE",
            reason: Some(reason),
            evidence: BTreeMap::new(),
            token_caps: BTreeMap::new(),
            e5_candidates,
            unscored: e5_candidates,
            max_unique_pairs,
            model_load_attempts: 0,
            runtime_ms: None,
            pair_cache_hits: Vec::new(),
        }
    }
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy)]
enum EmbeddingRole {
    Passage,
    Query,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArtifactManifestFile {
    bytes: u64,
    sha256: String,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArtifactManifest {
    schema_version: u32,
    model: String,
    repository: String,
    revision: String,
    files: BTreeMap<String, ArtifactManifestFile>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct ResolvedArtifacts {
    #[allow(dead_code)]
    snapshot: std::path::PathBuf,
    files: BTreeMap<String, std::path::PathBuf>,
}

#[cfg(feature = "nli-evidence")]
#[derive(Debug, Clone)]
struct NliArtifacts {
    directory: std::path::PathBuf,
    model_path: std::path::PathBuf,
    tokenizer_path: std::path::PathBuf,
}

#[cfg(feature = "spec-retrieval")]
fn model_cache_dir(root: &Path, role: &str) -> Result<std::path::PathBuf> {
    let resolved = model_cache_dir_from_values(
        role,
        std::env::var_os("HF_HOME").as_deref(),
        std::env::var_os("XDG_CACHE_HOME").as_deref(),
        std::env::var_os("HOME").as_deref(),
    )?;
    let _ = root;
    Ok(resolved)
}

#[cfg(feature = "spec-retrieval")]
fn model_cache_dir_from_values(
    role: &str,
    hf_home: Option<&std::ffi::OsStr>,
    xdg_cache_home: Option<&std::ffi::OsStr>,
    home: Option<&std::ffi::OsStr>,
) -> Result<std::path::PathBuf> {
    if let Some(path) = hf_home {
        anyhow::ensure!(!path.is_empty(), "HF_HOME must not be empty");
        return Ok(std::path::PathBuf::from(path));
    }

    let cache_home = match xdg_cache_home {
        Some(path) if !path.is_empty() => {
            let path = std::path::PathBuf::from(path);
            anyhow::ensure!(
                path.is_absolute(),
                "XDG_CACHE_HOME must be an absolute path, got {}",
                path.display()
            );
            path
        }
        _ => home
            .filter(|path| !path.is_empty())
            .map(std::path::PathBuf::from)
            .map(|path| path.join(".cache"))
            .with_context(|| "HOME is required when XDG_CACHE_HOME and HF_HOME are unset")?,
    };
    Ok(cache_home.join("givn").join(role))
}

#[cfg(feature = "spec-retrieval")]
fn artifact_digest(path: &Path) -> Result<ArtifactManifestFile> {
    let mut file =
        fs::File::open(path).with_context(|| format!("read model artifact {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 8192];
    loop {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("read model artifact {}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        bytes += read as u64;
    }
    let sha256 = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(ArtifactManifestFile { bytes, sha256 })
}

#[cfg(feature = "spec-retrieval")]
fn artifact_manifest_path(cache_root: &Path) -> std::path::PathBuf {
    cache_root.join(ARTIFACT_MANIFEST_NAME)
}

#[cfg(feature = "spec-retrieval")]
fn read_artifact_manifest(cache_root: &Path) -> Option<ArtifactManifest> {
    let bytes = fs::read(artifact_manifest_path(cache_root)).ok()?;
    serde_json::from_slice(&bytes).ok()
}

#[cfg(feature = "spec-retrieval")]
fn snapshot_root(path: &Path, filename: &str) -> Option<std::path::PathBuf> {
    let mut root = path.to_path_buf();
    for _ in std::path::Path::new(filename).components() {
        root.pop();
    }
    root.is_dir().then_some(root)
}

#[cfg(feature = "spec-retrieval")]
fn manifest_matches(
    manifest: &ArtifactManifest,
    model: &str,
    repository: &str,
    revision: &str,
    files: &[&str],
    paths: &BTreeMap<String, std::path::PathBuf>,
) -> bool {
    if manifest.schema_version != ARTIFACT_MANIFEST_VERSION
        || manifest.model != model
        || manifest.repository != repository
        || manifest.revision != revision
        || manifest.files.len() != files.len()
        || !files.iter().all(|file| manifest.files.contains_key(*file))
    {
        return false;
    }
    let Some(snapshot) = files
        .iter()
        .filter_map(|file| paths.get(*file).and_then(|path| snapshot_root(path, file)))
        .next()
    else {
        return false;
    };
    files.iter().all(|file| {
        let Some(path) = paths.get(*file) else {
            return false;
        };
        snapshot_root(path, file).as_ref() == Some(&snapshot)
            && path.is_file()
            && artifact_digest(path)
                .ok()
                .as_ref()
                .zip(manifest.files.get(*file))
                .is_some_and(|(actual, expected)| {
                    actual.bytes == expected.bytes && actual.sha256 == expected.sha256
                })
    })
}

#[cfg(feature = "spec-retrieval")]
fn notice(message: impl AsRef<str>) {
    eprintln!("{}", message.as_ref());
}

/// Whether the current advisory review runs with full detail output.
#[cfg(feature = "spec-retrieval")]
static REPORTING_VERBOSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether resolution chatter is scoped to a decision-first review run.
/// Every non-review command keeps today's per-attempt lines verbatim.
#[cfg(feature = "spec-retrieval")]
static REVIEW_CHATTER: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(feature = "spec-retrieval")]
fn reporting_verbose() -> bool {
    REPORTING_VERBOSE.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg(feature = "spec-retrieval")]
fn review_chatter() -> bool {
    REVIEW_CHATTER.load(std::sync::atomic::Ordering::Relaxed)
}

/// Stages whose collapsed resolution summary was already emitted once.
#[cfg(feature = "spec-retrieval")]
static RESOLUTION_SUMMARIZED: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Stages whose collapsed resolution start line was already emitted once.
#[cfg(feature = "spec-retrieval")]
static RESOLUTION_STARTED: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

#[cfg(feature = "spec-retrieval")]
const RESOLUTION_STAGE_E5: u8 = 1;
#[cfg(feature = "spec-retrieval")]
const RESOLUTION_STAGE_BGE: u8 = 2;
#[cfg(feature = "spec-retrieval")]
const RESOLUTION_STAGE_NLI: u8 = 4;

/// Announce the start of model artifact resolution for one stage.
///
/// Verbose output keeps today's per-attempt line; default output stays
/// silent here and reports once via [`resolve_stage_done`].
#[cfg(feature = "spec-retrieval")]
fn resolve_stage_start(stage: &'static str, detail: &str) {
    let key = stage_key(stage);
    if !review_chatter() || reporting_verbose() {
        notice(format!("resolving {stage} into {detail}"));
        return;
    }
    // Default review output keeps the first start line per stage so
    // download progress remains observable, and drops the repeats.
    let mask = match key {
        Some(RESOLUTION_STAGE_E5) => RESOLUTION_STAGE_E5,
        Some(RESOLUTION_STAGE_BGE) => RESOLUTION_STAGE_BGE,
        Some(RESOLUTION_STAGE_NLI) => RESOLUTION_STAGE_NLI,
        _ => {
            notice(format!("resolving {stage} into {detail}"));
            return;
        }
    };
    if RESOLUTION_STARTED.fetch_or(mask, std::sync::atomic::Ordering::Relaxed) & mask == 0 {
        notice(format!("resolving {stage} into {detail}"));
    }
}

/// Report completed model artifact resolution for one stage.
///
/// Verbose output keeps today's per-attempt line; default output emits
/// exactly one summary line per stage regardless of how often resolution
/// ran.
#[cfg(feature = "spec-retrieval")]
fn resolve_stage_done(stage: &'static str, detail: &str) {
    let key = stage_key(stage);
    if !review_chatter() {
        notice(format!("{stage} resolved"));
        return;
    }
    if reporting_verbose() {
        notice(format!("{stage} resolved"));
        return;
    }
    let mask = match key {
        Some(RESOLUTION_STAGE_E5) => RESOLUTION_STAGE_E5,
        Some(RESOLUTION_STAGE_BGE) => RESOLUTION_STAGE_BGE,
        Some(RESOLUTION_STAGE_NLI) => RESOLUTION_STAGE_NLI,
        _ => {
            notice(format!("{stage} ready ({detail})"));
            return;
        }
    };
    if RESOLUTION_SUMMARIZED.fetch_or(mask, std::sync::atomic::Ordering::Relaxed) & mask == 0 {
        notice(format!("{stage} ready ({detail})"));
    }
}

#[cfg(feature = "spec-retrieval")]
fn stage_key(stage: &str) -> Option<u8> {
    if stage.contains("(E5)") {
        Some(RESOLUTION_STAGE_E5)
    } else if stage.contains("(BGE)") {
        Some(RESOLUTION_STAGE_BGE)
    } else if stage.contains("NLI") {
        Some(RESOLUTION_STAGE_NLI)
    } else {
        None
    }
}

/// Deterministic test seam: emit repeated artifact-resolution notices for
/// every stage so chatter gating is observable under stub backends.
#[cfg(feature = "spec-retrieval")]
fn stub_resolution_chatter() {
    let stages = [
        ("retrieval model artifacts (E5)", "/stub/e5-cache"),
        ("reranker model artifacts (BGE)", "/stub/bge-cache"),
        ("NLI model artifacts", "/stub/nli-cache"),
    ];
    for _ in 0..2 {
        for (stage, detail) in stages {
            resolve_stage_start(stage, detail);
            resolve_stage_done(stage, detail);
        }
    }
}

/// stderr-based per-file download progress for hf-hub.
///
/// hf-hub re-invokes `init` on every download attempt (including retries)
/// and passes the cumulative resume offset to `update`, so this
/// implementation treats every reported byte count as absolute and resets
/// on `init`. Updates are throttled to >= 1 % deltas plus a final line.
#[cfg(feature = "spec-retrieval")]
struct StderrDownloadProgress {
    filename: String,
    total: usize,
    last_reported: usize,
}

#[cfg(feature = "spec-retrieval")]
impl StderrDownloadProgress {
    fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            total: 0,
            last_reported: 0,
        }
    }

    fn report(&mut self, current: usize, force: bool) {
        if self.total == 0 {
            return;
        }
        let step = (self.total / 100).max(1);
        if !force && current.saturating_sub(self.last_reported) < step {
            return;
        }
        notice(format!(
            "downloading {} {}/{} bytes",
            self.filename, current, self.total
        ));
        self.last_reported = current;
    }
}

#[cfg(feature = "spec-retrieval")]
impl hf_hub::api::Progress for StderrDownloadProgress {
    fn init(&mut self, size: usize, filename: &str) {
        self.filename = filename.to_string();
        self.total = size;
        self.last_reported = 0;
    }

    fn update(&mut self, size: usize) {
        self.report(size, false);
    }

    fn finish(&mut self) {
        self.report(self.total, true);
    }
}

#[cfg(feature = "spec-retrieval")]
fn hf_api(cache_root: &Path) -> Result<hf_hub::api::sync::Api> {
    let mut builder =
        hf_hub::api::sync::ApiBuilder::from_cache(hf_hub::Cache::new(cache_root.to_path_buf()))
            .with_progress(false)
            .with_retries(2);
    if let Some(endpoint) = std::env::var_os("HF_ENDPOINT") {
        let endpoint = endpoint
            .into_string()
            .map_err(|_| anyhow::anyhow!("HF_ENDPOINT is not valid UTF-8"))?;
        builder = builder.with_endpoint(endpoint);
    }
    builder
        .build()
        .context("initialize Hugging Face artifact source")
}

#[cfg(feature = "spec-retrieval")]
fn fetch_hf_artifacts(
    cache_root: &Path,
    model: &str,
    repository: &str,
    revision: &str,
    files: &[&str],
    force_download: bool,
) -> Result<ResolvedArtifacts> {
    fs::create_dir_all(cache_root)
        .with_context(|| format!("create model cache {}", cache_root.display()))?;
    let cache = hf_hub::Cache::new(cache_root.to_path_buf());
    let api = hf_api(cache_root)?;
    let repo_definition = hf_hub::Repo::with_revision(
        repository.to_string(),
        hf_hub::RepoType::Model,
        revision.to_string(),
    );
    let repo = api.repo(repo_definition.clone());
    let cache_repo = cache.repo(repo_definition);
    let mut paths = BTreeMap::new();
    if !force_download {
        for file in files {
            let Some(path) = cache_repo.get(file) else {
                paths.clear();
                break;
            };
            paths.insert((*file).to_string(), path);
        }
        if paths.len() == files.len()
            && read_artifact_manifest(cache_root).is_some_and(|manifest| {
                manifest_matches(&manifest, model, repository, revision, files, &paths)
            })
        {
            let first_file = files
                .first()
                .copied()
                .context("model artifact list is empty")?;
            let snapshot = paths
                .get(first_file)
                .and_then(|path| snapshot_root(path, first_file))
                .context("model artifact cache has no snapshot root")?;
            return Ok(ResolvedArtifacts {
                snapshot,
                files: paths,
            });
        }
        if paths.len() == files.len() {
            let first_file = files
                .first()
                .copied()
                .context("model artifact list is empty")?;
            let snapshot = paths
                .get(first_file)
                .and_then(|path| snapshot_root(path, first_file))
                .context("model artifact cache has no snapshot root")?;
            if files.iter().all(|file| {
                paths
                    .get(*file)
                    .and_then(|path| snapshot_root(path, file))
                    .as_ref()
                    == Some(&snapshot)
            }) {
                return Ok(ResolvedArtifacts {
                    snapshot,
                    files: paths,
                });
            }
        }
    }

    for file in files {
        if let Some(path) = cache_repo.get(file) {
            let _ = fs::remove_file(path);
        }
    }
    paths.clear();
    for file in files {
        let expected_size = api
            .metadata(&repo.url(file))
            .with_context(|| format!("read metadata for {model} artifact {file}"))?
            .size();
        let path = repo
            .download_with_progress(file, StderrDownloadProgress::new(file))
            .with_context(|| {
                format!(
                    "download {model} artifact {file} into {}",
                    cache_root.display()
                )
            })?;
        let actual = artifact_digest(&path)?;
        anyhow::ensure!(
            actual.bytes == expected_size as u64,
            "downloaded {model} artifact {file} has {} bytes; expected {expected_size}",
            actual.bytes
        );
        paths.insert((*file).to_string(), path);
    }
    let first_file = files
        .first()
        .copied()
        .context("model artifact list is empty")?;
    let snapshot = paths
        .get(first_file)
        .and_then(|path| snapshot_root(path, first_file))
        .context("downloaded model artifacts have no snapshot root")?;
    anyhow::ensure!(
        files.iter().all(|file| {
            paths
                .get(*file)
                .and_then(|path| snapshot_root(path, file))
                .as_ref()
                == Some(&snapshot)
        }),
        "downloaded model artifacts belong to mixed snapshots"
    );
    Ok(ResolvedArtifacts {
        snapshot,
        files: paths,
    })
}

#[cfg(feature = "spec-retrieval")]
fn write_artifact_manifest(
    cache_root: &Path,
    model: &str,
    repository: &str,
    revision: &str,
    files: &BTreeMap<String, std::path::PathBuf>,
) -> Result<()> {
    let manifest = ArtifactManifest {
        schema_version: ARTIFACT_MANIFEST_VERSION,
        model: model.to_string(),
        repository: repository.to_string(),
        revision: revision.to_string(),
        files: files
            .iter()
            .map(|(name, path)| Ok((name.clone(), artifact_digest(path)?)))
            .collect::<Result<BTreeMap<_, _>>>()?,
    };
    let temporary = artifact_manifest_path(cache_root).with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(&manifest)?)
        .with_context(|| format!("write temporary model manifest {}", temporary.display()))?;
    fs::rename(&temporary, artifact_manifest_path(cache_root)).with_context(|| {
        format!(
            "replace model manifest {}",
            artifact_manifest_path(cache_root).display()
        )
    })?;
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn record_download_event(event: &str) -> Result<()> {
    let Some(path) = spec_env("GIVN_SPEC_DOWNLOAD_EVENTS") else {
        return Ok(());
    };
    let path = std::path::PathBuf::from(path);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("open download event log {}", path.display()))?;
    writeln!(file, "{event}")
        .with_context(|| format!("write download event log {}", path.display()))?;
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn validate_json_artifact(path: &Path, label: &str) -> Result<()> {
    let bytes = fs::read(path).with_context(|| format!("read {label} {}", path.display()))?;
    serde_json::from_slice::<serde_json::Value>(&bytes)
        .with_context(|| format!("parse {label} {}", path.display()))?;
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn validate_required_files(files: &BTreeMap<String, std::path::PathBuf>) -> Result<()> {
    for (name, path) in files {
        anyhow::ensure!(
            path.is_file(),
            "model artifact {name} is not a regular file at {}",
            path.display()
        );
        anyhow::ensure!(
            fs::metadata(path)
                .with_context(|| format!("read model artifact {}", path.display()))?
                .len()
                > 0,
            "model artifact {name} is empty at {}",
            path.display()
        );
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn auto_e5_artifacts(root: &Path) -> Result<()> {
    let cache_root = e5_model_cache_dir(root)?;
    resolve_stage_start(
        "retrieval model artifacts (E5)",
        &cache_root.display().to_string(),
    );
    let files = [
        "model.onnx",
        "model.onnx_data",
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ];
    let mut artifacts = fetch_hf_artifacts(
        &cache_root,
        MODEL_ID,
        E5_MODEL_REPOSITORY,
        "main",
        &files,
        false,
    )?;
    let validate = |artifacts: &ResolvedArtifacts| -> Result<()> {
        validate_required_files(&artifacts.files)?;
        load_unbounded_tokenizer(artifacts.files.get("tokenizer.json").unwrap())?;
        for name in [
            "config.json",
            "special_tokens_map.json",
            "tokenizer_config.json",
        ] {
            validate_json_artifact(artifacts.files.get(name).unwrap(), "E5 metadata")?;
        }
        Ok(())
    };
    if validate(&artifacts).is_err() {
        artifacts = fetch_hf_artifacts(
            &cache_root,
            MODEL_ID,
            E5_MODEL_REPOSITORY,
            "main",
            &files,
            true,
        )?;
        validate(&artifacts)?;
    }
    write_artifact_manifest(
        &cache_root,
        MODEL_ID,
        E5_MODEL_REPOSITORY,
        "main",
        &artifacts.files,
    )?;
    resolve_stage_done(
        "retrieval model artifacts (E5)",
        &cache_root.display().to_string(),
    );
    record_download_event("artifact-resolved")?;
    Ok(())
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn auto_bge_artifacts(root: &Path) -> Result<std::path::PathBuf> {
    let cache_root = spec_env("GIVN_SPEC_BGE_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or(model_cache_dir(root, "bge")?);
    resolve_stage_start(
        "reranker model artifacts (BGE)",
        &cache_root.display().to_string(),
    );
    let files = [
        "model.onnx",
        "model.onnx.data",
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ];
    let mut artifacts = fetch_hf_artifacts(
        &cache_root,
        RERANKER_MODEL_ID,
        RERANKER_MODEL_ID,
        BGE_MODEL_REVISION,
        &files,
        false,
    )?;
    let valid = validate_required_files(&artifacts.files)
        .and_then(|_| load_unbounded_tokenizer(artifacts.files.get("tokenizer.json").unwrap()))
        .and_then(|_| {
            validate_json_artifact(artifacts.files.get("config.json").unwrap(), "BGE config")
        })
        .and_then(|_| {
            validate_json_artifact(
                artifacts.files.get("special_tokens_map.json").unwrap(),
                "BGE special-token map",
            )
        })
        .and_then(|_| {
            validate_json_artifact(
                artifacts.files.get("tokenizer_config.json").unwrap(),
                "BGE tokenizer config",
            )
        });
    if valid.is_err() {
        artifacts = fetch_hf_artifacts(
            &cache_root,
            RERANKER_MODEL_ID,
            RERANKER_MODEL_ID,
            BGE_MODEL_REVISION,
            &files,
            true,
        )?;
        validate_required_files(&artifacts.files)?;
        load_unbounded_tokenizer(artifacts.files.get("tokenizer.json").unwrap())?;
        for name in [
            "config.json",
            "special_tokens_map.json",
            "tokenizer_config.json",
        ] {
            validate_json_artifact(artifacts.files.get(name).unwrap(), "BGE metadata")?;
        }
    }
    write_artifact_manifest(
        &cache_root,
        RERANKER_MODEL_ID,
        RERANKER_MODEL_ID,
        BGE_MODEL_REVISION,
        &artifacts.files,
    )?;
    resolve_stage_done(
        "reranker model artifacts (BGE)",
        &cache_root.display().to_string(),
    );
    Ok(artifacts.snapshot)
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_artifact_directory(root: &Path, automatic: bool) -> Result<std::path::PathBuf> {
    if automatic {
        auto_bge_artifacts(root)
    } else {
        preflight_bge_artifacts(root)
    }
}

#[cfg(feature = "nli-evidence")]
fn auto_nli_artifacts(root: &Path) -> Result<NliArtifacts> {
    let cache_root = spec_env("GIVN_SPEC_NLI_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or(model_cache_dir(root, "nli")?);
    resolve_stage_start("NLI model artifacts", &cache_root.display().to_string());
    let files = [
        "onnx/model.onnx",
        "config.json",
        "tokenizer.json",
        "tokenizer_config.json",
    ];
    let mut artifacts = fetch_hf_artifacts(
        &cache_root,
        NLI_MODEL_ID,
        NLI_MODEL_ID,
        NLI_MODEL_REVISION,
        &files,
        false,
    )?;
    let validate = |artifacts: &ResolvedArtifacts| -> Result<()> {
        validate_required_files(&artifacts.files)?;
        load_unbounded_tokenizer(artifacts.files.get("tokenizer.json").unwrap())?;
        for name in ["config.json", "tokenizer_config.json"] {
            validate_json_artifact(artifacts.files.get(name).unwrap(), "NLI metadata")?;
        }
        let config_bytes = fs::read(artifacts.files.get("config.json").unwrap())?;
        let config: serde_json::Value = serde_json::from_slice(&config_bytes)?;
        nli_label_indices(&config)?;
        Ok(())
    };
    if validate(&artifacts).is_err() {
        artifacts = fetch_hf_artifacts(
            &cache_root,
            NLI_MODEL_ID,
            NLI_MODEL_ID,
            NLI_MODEL_REVISION,
            &files,
            true,
        )?;
        validate(&artifacts)?;
    }
    write_artifact_manifest(
        &cache_root,
        NLI_MODEL_ID,
        NLI_MODEL_ID,
        NLI_MODEL_REVISION,
        &artifacts.files,
    )?;
    resolve_stage_done("NLI model artifacts", &cache_root.display().to_string());
    Ok(NliArtifacts {
        directory: artifacts.snapshot,
        model_path: artifacts.files.get("onnx/model.onnx").unwrap().clone(),
        tokenizer_path: artifacts.files.get("tokenizer.json").unwrap().clone(),
    })
}

#[cfg(feature = "nli-evidence")]
fn nli_artifacts_for_backend(root: &Path, automatic: bool) -> Result<NliArtifacts> {
    if automatic {
        return auto_nli_artifacts(root);
    }
    preflight_nli_artifacts(root)
}

pub fn index(root: &Path) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        index_enabled(root, true)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = root;
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

#[cfg(feature = "spec-retrieval")]
pub fn index_without_download(root: &Path) -> Result<()> {
    index_enabled(root, false)
}

pub fn search(root: &Path, query: &str, evidence: bool) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        search_enabled(root, query, evidence)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, query, evidence);
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

pub fn explore(
    root: &Path,
    file: Option<&Path>,
    stdin: bool,
    change: Option<&str>,
    json: bool,
) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        explore_enabled(root, file, stdin, change, json)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, file, stdin, change, json);
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

/// Reporting options for the advisory review surface.
#[cfg(feature = "spec-retrieval")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ReviewOptions {
    /// Lift every detail-listing limit and restore full resolution chatter.
    pub verbose: bool,
    /// Emit one structured JSON document instead of text lines.
    pub json: bool,
    /// Persist flagged-review evidence beside the change. Only the
    /// authoring command enables this; blocking surfaces keep their own
    /// report artifacts.
    pub findings: bool,
}

#[cfg(feature = "spec-retrieval")]
pub fn review_with_options(root: &Path, change_id: &str, options: ReviewOptions) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        REPORTING_VERBOSE.store(options.verbose, std::sync::atomic::Ordering::Relaxed);
        REVIEW_CHATTER.store(true, std::sync::atomic::Ordering::Relaxed);
        report_missing_index(root);
        review_enabled(root, change_id, true, options)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, change_id, options);
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

#[allow(dead_code)]
pub fn semantic_review(root: &Path, change_id: &str) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        report_missing_index(root);
        semantic_review_enabled(root, change_id)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, change_id);
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

#[cfg(feature = "spec-retrieval")]
pub fn semantic_review_without_index_notice(root: &Path, change_id: &str) -> Result<()> {
    semantic_review_enabled(root, change_id)
}

pub fn duplicates(root: &Path, json: bool) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        duplicates_enabled(root, json)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, json);
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

pub fn bge_parity(root: &Path) -> Result<()> {
    #[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
    {
        let _ = root;
        let max_unique_pairs = max_rerank_pairs()?;
        let descriptor = bge_pipeline_descriptor(max_unique_pairs);
        let document = serde_json::json!({
            "schema_version": 1,
            "source": "givn spec bge-parity",
            "candidate_pipeline": BGE_PIPELINE_VERSION,
            "pipeline_fingerprint": format!("{:016x}", stable_hash(descriptor.as_bytes())),
            "descriptor": descriptor,
            "top_k": RERANKER_TOP_K,
            "threshold": RERANKER_LOGIT_THRESHOLD,
            "max_unique_pairs": max_unique_pairs,
        });
        println!("{}", serde_json::to_string_pretty(&document)?);
        Ok(())
    }

    #[cfg(not(all(feature = "spec-retrieval", feature = "cross-encoder-reranker")))]
    {
        let _ = root;
        bail!(
            "BGE parity attestation is unavailable in this build; rebuild with --features spec-retrieval,cross-encoder-reranker"
        );
    }
}

#[cfg(feature = "spec-retrieval")]
fn index_enabled(root: &Path, allow_download: bool) -> Result<()> {
    report_ignored_release_overrides();
    let refresh = refresh_index(root, allow_download)?;
    println!(
        "Indexed {} permanent scenarios",
        refresh.index.entries.len()
    );
    println!("Reused {} scenario vectors", refresh.reused);
    println!(
        "E5 token-cap candidates: {}",
        refresh.index.e5_token_caps.len()
    );
    for cap in &refresh.index.e5_token_caps {
        println!(
            "E5_TOKEN_CAP: '{}' ({}) requires {} tokens (limit {}); refactoring required: {}; guidance: {}",
            cap.title, cap.file, cap.token_count, cap.limit, cap.refactoring_required, cap.guidance,
        );
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn duplicates_enabled(root: &Path, json: bool) -> Result<()> {
    report_ignored_release_overrides();
    let scenarios = collect_duplicate_scenarios(root)?;
    let audit = duplicate_audit(root, &scenarios)?;

    if json {
        println!("{}", render_duplicate_json(scenarios.len(), &audit)?);
    } else {
        print!("{}", render_duplicate_text(scenarios.len(), &audit));
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_audit(root: &Path, scenarios: &[DuplicateScenario]) -> Result<DuplicateAudit> {
    let candidate_set = duplicate_candidates(root, scenarios)?;
    let DuplicateCandidateSet {
        candidates,
        e5_token_caps,
    } = candidate_set;
    let mut audit = {
        #[cfg(feature = "cross-encoder-reranker")]
        {
            apply_duplicate_bge(root, scenarios, candidates)?
        }
        #[cfg(not(feature = "cross-encoder-reranker"))]
        {
            DuplicateAudit {
                candidates,
                e5_token_caps: Vec::new(),
                filtered_candidates: Vec::new(),
                unscored_candidates: Vec::new(),
                bge_run: None,
                #[cfg(feature = "nli-evidence")]
                nli_filtered_candidates: Vec::new(),
                #[cfg(feature = "nli-evidence")]
                nli_unscored_candidates: Vec::new(),
                #[cfg(feature = "nli-evidence")]
                nli_run: None,
            }
        }
    };
    audit.e5_token_caps = e5_token_caps;
    #[cfg(feature = "nli-evidence")]
    {
        apply_duplicate_nli(root, scenarios, audit)
    }
    #[cfg(not(feature = "nli-evidence"))]
    {
        Ok(audit)
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn apply_duplicate_bge(
    root: &Path,
    scenarios: &[DuplicateScenario],
    mut candidates: Vec<DuplicateCandidate>,
) -> Result<DuplicateAudit> {
    let max_unique_pairs = max_rerank_pairs()?;
    let semantic: Vec<DuplicateCandidate> = candidates
        .iter()
        .filter(|candidate| candidate.kind == "semantic_candidate")
        .cloned()
        .collect();
    if semantic.is_empty() {
        return Ok(DuplicateAudit {
            candidates,
            e5_token_caps: Vec::new(),
            filtered_candidates: Vec::new(),
            unscored_candidates: Vec::new(),
            bge_run: None,
            #[cfg(feature = "nli-evidence")]
            nli_filtered_candidates: Vec::new(),
            #[cfg(feature = "nli-evidence")]
            nli_unscored_candidates: Vec::new(),
            #[cfg(feature = "nli-evidence")]
            nli_run: None,
        });
    }
    let Some(backend) = bge_backend(root)? else {
        return Ok(DuplicateAudit {
            candidates,
            e5_token_caps: Vec::new(),
            filtered_candidates: Vec::new(),
            unscored_candidates: Vec::new(),
            bge_run: None,
            #[cfg(feature = "nli-evidence")]
            nli_filtered_candidates: Vec::new(),
            #[cfg(feature = "nli-evidence")]
            nli_unscored_candidates: Vec::new(),
            #[cfg(feature = "nli-evidence")]
            nli_run: None,
        });
    };

    if let Some((bounded_count, raw_count)) = bge_test_pool()? {
        let (pairs, texts) = synthetic_bge_pairs(bounded_count.min(max_unique_pairs));
        let mut run = match backend {
            Ok(kind) => score_bge_pairs(root, &pairs, &texts, kind, max_unique_pairs)?,
            Err(reason) => BgeRun::unavailable(reason, raw_count, max_unique_pairs),
        };
        run.e5_candidates = raw_count;
        run.unscored = raw_count.saturating_sub(pairs.len());
        let mut unscored = Vec::new();
        if run.unscored > 0
            && let Some(template) = semantic.first()
        {
            let mut candidate = template.clone();
            candidate.bge_admission = Some("BGE_UNSCORED");
            unscored.push(candidate);
        }
        return Ok(DuplicateAudit {
            candidates,
            e5_token_caps: Vec::new(),
            filtered_candidates: Vec::new(),
            unscored_candidates: unscored,
            bge_run: Some(run),
            #[cfg(feature = "nli-evidence")]
            nli_filtered_candidates: Vec::new(),
            #[cfg(feature = "nli-evidence")]
            nli_unscored_candidates: Vec::new(),
            #[cfg(feature = "nli-evidence")]
            nli_run: None,
        });
    }

    let mut selected = BTreeSet::new();
    for scenario in scenarios {
        let mut neighbours: Vec<&DuplicateCandidate> = semantic
            .iter()
            .filter(|candidate| {
                candidate.left.file == scenario.endpoint.file
                    && candidate.left.title == scenario.endpoint.title
                    || candidate.right.file == scenario.endpoint.file
                        && candidate.right.title == scenario.endpoint.title
            })
            .collect();
        neighbours.sort_by(|left, right| {
            right
                .score
                .unwrap_or_default()
                .total_cmp(&left.score.unwrap_or_default())
                .then_with(|| duplicate_candidate_order(left, right))
        });
        for candidate in neighbours.into_iter().take(RERANKER_TOP_K) {
            selected.insert(duplicate_pair_key(&candidate.left, &candidate.right));
        }
    }

    let mut selected_candidates: Vec<&DuplicateCandidate> = semantic
        .iter()
        .filter(|candidate| {
            selected.contains(&duplicate_pair_key(&candidate.left, &candidate.right))
        })
        .collect();
    selected_candidates.sort_by(|left, right| duplicate_candidate_order(left, right));
    selected_candidates.truncate(max_unique_pairs);

    let texts: BTreeMap<(String, String), String> = scenarios
        .iter()
        .map(|scenario| {
            (
                (
                    scenario.endpoint.file.clone(),
                    scenario.endpoint.title.clone(),
                ),
                scenario.text.clone(),
            )
        })
        .collect();
    let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = selected_candidates
        .iter()
        .map(|candidate| {
            (
                endpoint(&candidate.left.file, &candidate.left.title),
                endpoint(&candidate.right.file, &candidate.right.title),
            )
        })
        .collect();
    let tokenizer = match backend {
        Ok(BgeBackendKind::Local | BgeBackendKind::Auto)
            if spec_env_var("GIVN_SPEC_BGE_LOCAL_STUB").as_deref() != Ok("1") =>
        {
            let automatic = matches!(backend, Ok(BgeBackendKind::Auto));
            Some(load_unbounded_tokenizer(
                &bge_artifact_directory(root, automatic)?.join("tokenizer.json"),
            )?)
        }
        _ => None,
    };
    let partition =
        preflight_pair_budgets(&pairs, &texts, TokenBudgetModel::Bge, tokenizer.as_ref())?;
    let mut run = match backend {
        Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
            BgeRun::unavailable(
                BgeUnavailableReason::TokenCap,
                semantic.len(),
                max_unique_pairs,
            )
        }
        Ok(kind) => score_bge_pairs(root, &partition.fitting, &texts, kind, max_unique_pairs)?,
        Err(reason) => BgeRun::unavailable(reason, semantic.len(), max_unique_pairs),
    };
    run.e5_candidates = semantic.len();
    run.unscored = semantic.len().saturating_sub(run.evidence.len());
    run.token_caps = partition.capped.clone();

    let mut filtered = Vec::new();
    let mut unscored = Vec::new();
    for candidate in &mut candidates {
        if candidate.kind != "semantic_candidate" {
            continue;
        }
        let key = canonical_pair_key(
            endpoint(&candidate.left.file, &candidate.left.title),
            endpoint(&candidate.right.file, &candidate.right.title),
        );
        if let Some(budget) = partition.capped.get(&key).copied() {
            candidate.bge = None;
            candidate.bge_token_cap = Some(budget);
            candidate.bge_admission = Some("BGE_TOKEN_CAP");
            unscored.push(candidate.clone());
            continue;
        }
        let Some(evidence) = run.evidence.get(&key).cloned() else {
            candidate.bge = None;
            candidate.bge_token_cap = None;
            candidate.bge_admission = Some("BGE_UNSCORED");
            unscored.push(candidate.clone());
            continue;
        };
        candidate.bge_token_cap = None;
        candidate.bge = Some(evidence.clone());
        if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
            candidate.bge_admission = Some("BGE_RECOMMENDED");
        } else {
            candidate.bge_admission = Some("BGE_FILTERED");
            filtered.push(candidate.clone());
        }
    }
    candidates.sort_by(duplicate_candidate_order);
    filtered.sort_by(duplicate_candidate_order);
    unscored.sort_by(duplicate_candidate_order);
    Ok(DuplicateAudit {
        candidates,
        e5_token_caps: Vec::new(),
        filtered_candidates: filtered,
        unscored_candidates: unscored,
        bge_run: Some(run),
        #[cfg(feature = "nli-evidence")]
        nli_filtered_candidates: Vec::new(),
        #[cfg(feature = "nli-evidence")]
        nli_unscored_candidates: Vec::new(),
        #[cfg(feature = "nli-evidence")]
        nli_run: None,
    })
}

#[cfg(feature = "nli-evidence")]
fn apply_duplicate_nli(
    root: &Path,
    scenarios: &[DuplicateScenario],
    mut audit: DuplicateAudit,
) -> Result<DuplicateAudit> {
    let max_unique_pairs = max_rerank_pairs()?;
    let semantic: Vec<DuplicateCandidate> = audit
        .candidates
        .iter()
        .filter(|candidate| candidate.kind == "semantic_candidate")
        .cloned()
        .collect();
    if semantic.is_empty() {
        return Ok(audit);
    }
    let Some(selection) = nli_backend_selection(root)? else {
        return Ok(audit);
    };
    let Some((bounded_count, raw_count)) = nli_test_pool()? else {
        let texts: BTreeMap<(String, String), String> = scenarios
            .iter()
            .map(|scenario| {
                (
                    (
                        scenario.endpoint.file.clone(),
                        scenario.endpoint.title.clone(),
                    ),
                    scenario.text.clone(),
                )
            })
            .collect();
        let mut selected = semantic.clone();
        selected.sort_by(duplicate_candidate_order);
        selected.truncate(max_unique_pairs);
        let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = selected
            .iter()
            .map(|candidate| {
                (
                    endpoint(&candidate.left.file, &candidate.left.title),
                    endpoint(&candidate.right.file, &candidate.right.title),
                )
            })
            .collect();
        let tokenizer = match selection {
            Ok(NliBackend::Local | NliBackend::Auto) => {
                let artifacts =
                    nli_artifacts_for_backend(root, matches!(selection, Ok(NliBackend::Auto)))?;
                Some(load_unbounded_tokenizer(&artifacts.tokenizer_path)?)
            }
            _ => None,
        };
        let partition =
            preflight_pair_budgets(&pairs, &texts, TokenBudgetModel::Nli, tokenizer.as_ref())?;
        let mut run = match selection {
            Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
                NliRun::unavailable(
                    NliUnavailableReason::TokenCap,
                    semantic.len(),
                    max_unique_pairs,
                )
            }
            Ok(backend) => {
                score_nli_pairs(root, &partition.fitting, &texts, backend, max_unique_pairs)?
            }
            Err(reason) => NliRun::unavailable(reason, semantic.len(), max_unique_pairs),
        };
        run.e5_candidates = semantic.len();
        run.unscored = semantic.len().saturating_sub(run.evidence.len());
        run.token_caps = partition.capped.clone();
        let mut filtered = Vec::new();
        let mut unscored = Vec::new();
        for candidate in &mut audit.candidates {
            if candidate.kind != "semantic_candidate" {
                continue;
            }
            let key = canonical_pair_key(
                endpoint(&candidate.left.file, &candidate.left.title),
                endpoint(&candidate.right.file, &candidate.right.title),
            );
            if let Some(budget) = partition.capped.get(&key).copied() {
                candidate.nli = None;
                candidate.nli_token_cap = Some(budget);
                candidate.nli_admission = Some("NLI_TOKEN_CAP");
                unscored.push(candidate.clone());
                continue;
            }
            let Some(evidence) = run.evidence.get(&key).copied() else {
                candidate.nli = None;
                candidate.nli_token_cap = None;
                candidate.nli_admission = Some("NLI_UNSCORED");
                unscored.push(candidate.clone());
                continue;
            };
            candidate.nli_token_cap = None;
            candidate.nli = Some(evidence);
            if evidence.symmetric[0] >= NLI_ADMISSION_THRESHOLD {
                candidate.nli_admission = Some("NLI_RECOMMENDED");
            } else {
                candidate.nli_admission = Some("NLI_FILTERED");
                filtered.push(candidate.clone());
            }
        }
        audit.candidates.sort_by(duplicate_candidate_order);
        filtered.sort_by(duplicate_candidate_order);
        unscored.sort_by(duplicate_candidate_order);
        audit.nli_filtered_candidates = filtered;
        audit.nli_unscored_candidates = unscored;
        audit.nli_run = Some(run);
        return Ok(audit);
    };
    let (pairs, texts) = synthetic_nli_pairs(bounded_count.min(max_unique_pairs));
    let mut run = match selection {
        Ok(backend) => score_nli_pairs(root, &pairs, &texts, backend, max_unique_pairs)?,
        Err(reason) => NliRun::unavailable(reason, raw_count, max_unique_pairs),
    };
    run.e5_candidates = raw_count;
    run.unscored = raw_count.saturating_sub(pairs.len());
    if run.unscored > 0
        && let Some(template) = semantic.first()
    {
        let mut candidate = template.clone();
        candidate.nli_admission = Some("NLI_UNSCORED");
        audit.nli_unscored_candidates.push(candidate);
    }
    for candidate in &mut audit.candidates {
        if candidate.kind == "semantic_candidate" {
            candidate.nli_admission = Some(if run.unscored > 0 {
                "NLI_UNSCORED"
            } else {
                "NLI_RECOMMENDED"
            });
        }
    }
    audit.nli_run = Some(run);
    Ok(audit)
}

#[cfg(feature = "nli-evidence")]
fn nli_test_pool() -> Result<Option<(usize, usize)>> {
    if spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(None);
    }
    let Some(value) = spec_env("GIVN_SPEC_NLI_TEST_POOL") else {
        return Ok(None);
    };
    let value = value
        .to_str()
        .context("GIVN_SPEC_NLI_TEST_POOL is not valid UTF-8")?;
    match value {
        "bounded-512" => Ok(Some((512, 512))),
        "overflow-513" => Ok(Some((513, 513))),
        "partial-raw-513" => Ok(Some((512, 513))),
        _ => bail!("unknown GIVN_SPEC_NLI_TEST_POOL value `{value}`"),
    }
}

#[cfg(feature = "nli-evidence")]
fn synthetic_nli_pairs(count: usize) -> (Vec<SimilarityPair>, SimilarityTexts) {
    let mut pairs = Vec::with_capacity(count);
    let mut texts = BTreeMap::new();
    for index in 0..count {
        let left = endpoint(
            "givn/specs/__nli-fixture__/left.feature",
            &format!("left-{index}"),
        );
        let right = endpoint(
            "givn/specs/__nli-fixture__/right.feature",
            &format!("right-{index}"),
        );
        texts.insert((left.file.clone(), left.title.clone()), left.title.clone());
        texts.insert(
            (right.file.clone(), right.title.clone()),
            right.title.clone(),
        );
        pairs.push((left, right));
    }
    (pairs, texts)
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn token_budget_fixture(model: TokenBudgetModel) -> Result<Option<String>> {
    if spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(None);
    }
    let key = match model {
        #[cfg(feature = "cross-encoder-reranker")]
        TokenBudgetModel::Bge => "GIVN_SPEC_BGE_TOKEN_BUDGET",
        #[cfg(feature = "nli-evidence")]
        TokenBudgetModel::Nli => "GIVN_SPEC_NLI_TOKEN_BUDGET",
    };
    let value = spec_env_var(key).or_else(|error| match error {
        std::env::VarError::NotPresent => spec_env_var("GIVN_SPEC_TOKEN_BUDGET"),
        other => Err(other),
    });
    match value {
        Ok(value) if !value.trim().is_empty() => Ok(Some(value)),
        Ok(_) => bail!("{key} must not be empty"),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => bail!("{key} is not valid UTF-8"),
    }
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn fixture_pair_budget(fixture: &str, index: usize) -> Result<PairTokenBudget> {
    let budget = match fixture {
        "exact-512" | "bounded-512" | "all-512" => PairTokenBudget {
            left_tokens: 256,
            right_tokens: 254,
            pair_tokens: 512,
            limit: E5_MAX_LENGTH,
        },
        "overflow-513" | "all-513" => PairTokenBudget {
            left_tokens: 257,
            right_tokens: 255,
            pair_tokens: 513,
            limit: E5_MAX_LENGTH,
        },
        "partial-512-513" | "partial-raw-513" => {
            if index == 0 {
                PairTokenBudget {
                    left_tokens: 256,
                    right_tokens: 254,
                    pair_tokens: 512,
                    limit: E5_MAX_LENGTH,
                }
            } else {
                PairTokenBudget {
                    left_tokens: 257,
                    right_tokens: 255,
                    pair_tokens: 513,
                    limit: E5_MAX_LENGTH,
                }
            }
        }
        _ => bail!(
            "unknown token budget fixture `{fixture}`; expected exact-512, overflow-513, or partial-512-513"
        ),
    };
    Ok(budget)
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn preflight_pair_budgets(
    pairs: &[(SimilarityEndpoint, SimilarityEndpoint)],
    texts: &BTreeMap<(String, String), String>,
    model: TokenBudgetModel,
    tokenizer: Option<&Tokenizer>,
) -> Result<PairBudgetPartition> {
    let fixture = token_budget_fixture(model)?;
    let limit = E5_MAX_LENGTH;
    let mut partition = PairBudgetPartition::default();
    for (index, (left, right)) in pairs.iter().enumerate() {
        let budget = if let Some(fixture) = fixture.as_deref() {
            fixture_pair_budget(fixture, index)?
        } else if let Some(tokenizer) = tokenizer {
            actual_pair_budget(
                tokenizer,
                scenario_text_for(texts, left)?,
                scenario_text_for(texts, right)?,
                limit,
            )?
        } else {
            partition.fitting.push((left.clone(), right.clone()));
            continue;
        };
        let key = canonical_pair_key(left.clone(), right.clone());
        if token_budget_fits(budget) {
            partition.fitting.push((left.clone(), right.clone()));
        } else {
            partition.capped.insert(key, budget);
        }
    }
    Ok(partition)
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn token_budget_fits(budget: PairTokenBudget) -> bool {
    budget.pair_tokens <= budget.limit
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn actual_pair_budget(
    tokenizer: &Tokenizer,
    left: &str,
    right: &str,
    limit: usize,
) -> Result<PairTokenBudget> {
    let left_tokens = tokenizer
        .encode(left, true)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?
        .len();
    let right_tokens = tokenizer
        .encode(right, true)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?
        .len();
    let pair_tokens = tokenizer
        .encode((left, right), true)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?
        .len();
    Ok(PairTokenBudget {
        left_tokens,
        right_tokens,
        pair_tokens,
        limit,
    })
}

#[cfg(feature = "spec-retrieval")]
fn load_unbounded_tokenizer(path: &Path) -> Result<Tokenizer> {
    let mut tokenizer = Tokenizer::from_file(path)
        .map_err(|error| anyhow::anyhow!("failed to load tokenizer {}: {error}", path.display()))?;
    tokenizer.with_padding(None);
    tokenizer
        .with_truncation(None)
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    Ok(tokenizer)
}

#[cfg(feature = "spec-retrieval")]
fn render_duplicate_text(scanned_scenarios: usize, audit: &DuplicateAudit) -> String {
    let mut report = format!(
        "Givn spec duplicates\nScanned scenarios: {scanned_scenarios}\nCandidate count: {}\n",
        audit.candidates.len()
    );
    if let Some(run) = audit.bge_run.as_ref() {
        report.push_str(&format!(
            "BGE status: {}{}\nBGE model: {}\nBGE top_k: {}\nBGE max unique pairs: {}\nBGE E5 raw count: {}\nBGE recommended count: {}\nBGE filtered count: {}\nBGE unscored count: {}\nBGE unique pair count: {}\nBGE directional call count: {}\nBGE model load attempts: {}\nBGE runtime ms: {}\n",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default(),
            if run.status == "BGE_ENABLED" { RERANKER_MODEL_ID } else { "null" },
            RERANKER_TOP_K,
            run.max_unique_pairs,
            run.e5_candidates,
            audit
                .candidates
                .iter()
                .filter(|candidate| candidate.bge_admission == Some("BGE_RECOMMENDED"))
                .count(),
            audit.filtered_candidates.len(),
            run.unscored,
            run.evidence.len(),
            run.evidence.len() * 2,
            run.model_load_attempts,
            run.runtime_ms.map(|value| value.to_string()).unwrap_or_else(|| "null".to_string()),
        ));
        if run.status == "BGE_UNAVAILABLE" {
            if run.reason == Some(BgeUnavailableReason::TokenCap) {
                report.push_str("BGE_UNAVAILABLE: no complete semantic pair fits 512 tokens\n");
            }
            report.push_str("BGE fallback: E5-only unavailable fallback\n");
        }
    }
    #[cfg(feature = "nli-evidence")]
    if let Some(run) = audit.nli_run.as_ref() {
        let recommended = audit
            .candidates
            .iter()
            .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
            .count();
        report.push_str(&format!(
            "NLI status: {}{}\nNLI model: {}\nNLI top_k: {}\nNLI max unique pairs: {}\nNLI E5 raw count: {}\nNLI recommended count: {}\nNLI filtered count: {}\nNLI unscored count: {}\nNLI unique pair count: {}\nNLI directional call count: {}\nNLI model load attempts: {}\nNLI runtime ms: {}\n",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default(),
            if run.status == "NLI_ENABLED" { NLI_MODEL_ID } else { "null" },
            RERANKER_TOP_K,
            run.max_unique_pairs,
            run.e5_candidates,
            recommended,
            audit.nli_filtered_candidates.len(),
            run.unscored,
            run.evidence.len(),
            run.evidence.len() * 2,
            run.model_load_attempts,
            run.runtime_ms
                .map(|value| value.to_string())
                .unwrap_or_else(|| "null".to_string()),
        ));
        if run.reason == Some(NliUnavailableReason::TokenCap) {
            report.push_str("NLI_UNAVAILABLE: no complete semantic pair fits 512 tokens\n");
        }
        report.push_str("NLI fallback: E5-only unavailable fallback\n");
        for candidate in &audit.nli_unscored_candidates {
            report.push_str(&format!(
                "NLI unscored: {}\n",
                render_duplicate_candidate(0, candidate)
            ));
        }
    }
    for (rank, candidate) in audit.candidates.iter().enumerate() {
        report.push_str(&render_duplicate_candidate(rank + 1, candidate));
        report.push('\n');
    }
    for candidate in &audit.filtered_candidates {
        report.push_str(&format!(
            "BGE filtered: {}\n",
            render_duplicate_candidate(0, candidate)
        ));
    }
    for candidate in &audit.unscored_candidates {
        report.push_str(&format!(
            "BGE unscored: {}\n",
            render_duplicate_candidate(0, candidate)
        ));
    }
    #[cfg(feature = "nli-evidence")]
    for candidate in &audit.nli_filtered_candidates {
        report.push_str(&format!(
            "NLI filtered: {}\n",
            render_duplicate_candidate(0, candidate)
        ));
    }
    if !audit.e5_token_caps.is_empty() {
        report.push_str(&format!(
            "E5 token-cap candidate count: {}\n",
            audit.e5_token_caps.len()
        ));
        for cap in &audit.e5_token_caps {
            report.push_str(&format!(
                "E5_TOKEN_CAP: file={} title={:?} content_hash={} token_count={} limit={} model={} tokenizer={} refactoring_required={} guidance={:?}\n",
                cap.file,
                cap.title,
                cap.content_hash,
                cap.token_count,
                cap.limit,
                cap.model,
                cap.tokenizer,
                cap.refactoring_required,
                cap.guidance,
            ));
        }
        report.push_str(&format!(
            "refactoring required: {}\n",
            DUPLICATE_E5_TOKEN_CAP_GUIDANCE
        ));
    }
    if audit.candidates.iter().any(|candidate| {
        candidate.bge_token_cap.is_some() || {
            #[cfg(feature = "nli-evidence")]
            {
                candidate.nli_token_cap.is_some()
            }
            #[cfg(not(feature = "nli-evidence"))]
            {
                false
            }
        }
    }) {
        report.push_str(&format!("refactoring required: {TOKEN_CAP_GUIDANCE}\n"));
    }
    report
}

#[cfg(feature = "spec-retrieval")]
fn render_duplicate_json(scanned_scenarios: usize, audit: &DuplicateAudit) -> Result<String> {
    #[cfg(feature = "nli-evidence")]
    let nli_active = audit.nli_run.is_some();
    #[cfg(not(feature = "nli-evidence"))]
    let nli_active = false;
    let schema_version = if nli_active {
        #[cfg(feature = "nli-evidence")]
        {
            3
        }
        #[cfg(not(feature = "nli-evidence"))]
        {
            unreachable!()
        }
    } else if audit.bge_run.is_some() {
        DUPLICATE_BGE_REPORT_SCHEMA_VERSION
    } else {
        DUPLICATE_REPORT_SCHEMA_VERSION
    };
    #[cfg(feature = "nli-evidence")]
    let mut e5_candidate_count = audit.bge_run.as_ref().map(|run| run.e5_candidates);
    #[cfg(not(feature = "nli-evidence"))]
    let e5_candidate_count = audit.bge_run.as_ref().map(|run| run.e5_candidates);
    #[cfg(feature = "nli-evidence")]
    {
        e5_candidate_count =
            e5_candidate_count.or_else(|| audit.nli_run.as_ref().map(|run| run.e5_candidates));
    }
    let pair_refactoring_required = audit.candidates.iter().any(|candidate| {
        candidate.bge_token_cap.is_some() || {
            #[cfg(feature = "nli-evidence")]
            {
                candidate.nli_token_cap.is_some()
            }
            #[cfg(not(feature = "nli-evidence"))]
            {
                false
            }
        }
    });
    let refactoring_required = pair_refactoring_required || !audit.e5_token_caps.is_empty();
    let expose_e5_caps = expose_e5_cap_contract(&audit.e5_token_caps);
    let guidance = if !audit.e5_token_caps.is_empty() {
        DUPLICATE_E5_TOKEN_CAP_GUIDANCE
    } else {
        TOKEN_CAP_GUIDANCE
    };
    #[cfg(feature = "nli-evidence")]
    let combined_recommended_candidates = (audit.bge_run.is_some() && audit.nli_run.is_some())
        .then(|| {
            audit
                .candidates
                .iter()
                .filter(|candidate| {
                    candidate.kind == "semantic_candidate"
                        && candidate.bge_admission == Some("BGE_RECOMMENDED")
                        && candidate.nli_admission == Some("NLI_RECOMMENDED")
                })
                .map(duplicate_json_candidate)
                .enumerate()
                .map(|(index, mut candidate)| {
                    candidate.rank = index + 1;
                    candidate
                })
                .collect()
        });
    #[cfg(not(feature = "nli-evidence"))]
    let combined_recommended_candidates = None;
    let report = DuplicateJsonReport {
        schema_version,
        scanned_scenarios,
        candidate_count: audit.candidates.len(),
        candidates: audit
            .candidates
            .iter()
            .map(duplicate_json_candidate)
            .enumerate()
            .map(|(index, mut candidate)| {
                candidate.rank = index + 1;
                candidate
            })
            .collect(),
        e5_token_cap_count: expose_e5_caps.then_some(audit.e5_token_caps.len()),
        e5_token_cap_candidates: expose_e5_caps.then(|| audit.e5_token_caps.clone()),
        e5_candidate_count,
        bge_run: audit
            .bge_run
            .as_ref()
            .map(|run| duplicate_json_run(run, audit)),
        filtered_candidates: audit.bge_run.as_ref().map(|_| {
            audit
                .filtered_candidates
                .iter()
                .map(duplicate_json_candidate)
                .enumerate()
                .map(|(index, mut candidate)| {
                    candidate.rank = index + 1;
                    candidate
                })
                .collect()
        }),
        unscored_candidates: audit.bge_run.as_ref().map(|_| {
            audit
                .unscored_candidates
                .iter()
                .map(duplicate_json_candidate)
                .enumerate()
                .map(|(index, mut candidate)| {
                    candidate.rank = index + 1;
                    candidate
                })
                .collect()
        }),
        #[cfg(feature = "nli-evidence")]
        nli_run: audit
            .nli_run
            .as_ref()
            .map(|run| duplicate_json_nli_run(run, audit)),
        #[cfg(feature = "nli-evidence")]
        nli_filtered_candidates: audit.nli_run.as_ref().map(|_| {
            audit
                .nli_filtered_candidates
                .iter()
                .map(duplicate_json_candidate)
                .enumerate()
                .map(|(index, mut candidate)| {
                    candidate.rank = index + 1;
                    candidate
                })
                .collect()
        }),
        #[cfg(feature = "nli-evidence")]
        nli_unscored_candidates: audit.nli_run.as_ref().map(|_| {
            audit
                .nli_unscored_candidates
                .iter()
                .map(duplicate_json_candidate)
                .enumerate()
                .map(|(index, mut candidate)| {
                    candidate.rank = index + 1;
                    candidate
                })
                .collect()
        }),
        combined_recommended_candidates,
        refactoring_required: refactoring_required.then_some(true),
        guidance: refactoring_required.then_some(guidance),
    };
    serde_json::to_string(&report).context("serialize duplicate audit JSON")
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_json_candidate(candidate: &DuplicateCandidate) -> DuplicateJsonCandidate {
    DuplicateJsonCandidate {
        rank: 0,
        kind: candidate.kind,
        left: candidate.left.clone(),
        right: candidate.right.clone(),
        evidence: DuplicateJsonEvidence {
            relation: candidate.relation,
            assertion_shape: candidate.assertion_shape.clone(),
            facts: candidate.facts.clone(),
            distinguishing_facts: candidate.distinguishing_facts.clone(),
            fact_contradiction: candidate.fact_contradiction,
            score: candidate.score.map(|score| format!("{score:.4}")),
            bge: duplicate_json_bge(candidate),
            #[cfg(feature = "nli-evidence")]
            nli: duplicate_json_nli(candidate),
        },
    }
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_json_bge(candidate: &DuplicateCandidate) -> Option<DuplicateJsonBgeEvidence> {
    if let Some(evidence) = candidate.bge.as_ref() {
        return Some(DuplicateJsonBgeEvidence {
            status: candidate.bge_admission.unwrap_or("BGE_RECOMMENDED"),
            forward_raw: Some(evidence.forward_raw),
            reverse_raw: Some(evidence.reverse_raw),
            symmetric_raw: Some(evidence.symmetric_raw),
            left_tokens: None,
            right_tokens: None,
            pair_tokens: None,
            limit: None,
            refactoring_required: None,
        });
    }
    candidate
        .bge_token_cap
        .map(|budget| DuplicateJsonBgeEvidence {
            status: "BGE_TOKEN_CAP",
            forward_raw: None,
            reverse_raw: None,
            symmetric_raw: None,
            left_tokens: Some(budget.left_tokens),
            right_tokens: Some(budget.right_tokens),
            pair_tokens: Some(budget.pair_tokens),
            limit: Some(budget.limit),
            refactoring_required: Some(true),
        })
}

#[cfg(feature = "nli-evidence")]
fn duplicate_json_nli(candidate: &DuplicateCandidate) -> Option<DuplicateJsonNliEvidence> {
    if let Some(evidence) = candidate.nli {
        return Some(DuplicateJsonNliEvidence {
            status: candidate.nli_admission.unwrap_or("NLI_RECOMMENDED"),
            forward: Some(evidence.forward),
            reverse: Some(evidence.reverse),
            symmetric: Some(evidence.symmetric),
            left_tokens: None,
            right_tokens: None,
            pair_tokens: None,
            limit: None,
            refactoring_required: None,
        });
    }
    candidate
        .nli_token_cap
        .map(|budget| DuplicateJsonNliEvidence {
            status: "NLI_TOKEN_CAP",
            forward: None,
            reverse: None,
            symmetric: None,
            left_tokens: Some(budget.left_tokens),
            right_tokens: Some(budget.right_tokens),
            pair_tokens: Some(budget.pair_tokens),
            limit: Some(budget.limit),
            refactoring_required: Some(true),
        })
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_json_run(run: &BgeRun, audit: &DuplicateAudit) -> DuplicateJsonRun {
    let recommended_count = audit
        .candidates
        .iter()
        .filter(|candidate| candidate.bge_admission == Some("BGE_RECOMMENDED"))
        .count();
    DuplicateJsonRun {
        status: run.status,
        reason: run.reason.map(BgeUnavailableReason::as_str),
        model: (run.status == "BGE_ENABLED").then_some(RERANKER_MODEL_ID),
        top_k: RERANKER_TOP_K,
        max_unique_pairs: run.max_unique_pairs,
        e5_candidates: run.e5_candidates,
        unique_pairs: run.evidence.len(),
        directional_calls: run.evidence.len() * 2,
        recommended_count,
        filtered_count: audit.filtered_candidates.len(),
        unscored_count: run.unscored,
        model_load_attempts: run.model_load_attempts,
        runtime_ms: run.runtime_ms,
    }
}

#[cfg(feature = "nli-evidence")]
fn duplicate_json_nli_run(run: &NliRun, audit: &DuplicateAudit) -> DuplicateJsonNliRun {
    let recommended_count = audit
        .candidates
        .iter()
        .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
        .count();
    DuplicateJsonNliRun {
        status: run.status,
        reason: run.reason.map(NliUnavailableReason::as_str),
        model: (run.status == "NLI_ENABLED").then_some(NLI_MODEL_ID),
        top_k: RERANKER_TOP_K,
        max_unique_pairs: run.max_unique_pairs,
        e5_candidates: run.e5_candidates,
        unique_pairs: run.evidence.len(),
        directional_calls: run.evidence.len() * 2,
        recommended_count,
        filtered_count: audit.nli_filtered_candidates.len(),
        unscored_count: run.unscored,
        model_load_attempts: run.model_load_attempts,
        runtime_ms: run.runtime_ms,
    }
}

#[cfg(feature = "spec-retrieval")]
fn render_duplicate_candidate(rank: usize, candidate: &DuplicateCandidate) -> String {
    let score = candidate
        .score
        .map(|score| format!("{score:.4}"))
        .unwrap_or_else(|| "null".to_string());
    let bge = candidate
        .bge
        .as_ref()
        .map(|evidence| {
            format!(
                " bge_status={} bge_forward_raw={:.4} bge_reverse_raw={:.4} bge_symmetric_raw={:.4}",
                candidate.bge_admission.unwrap_or("BGE_RECOMMENDED"),
                evidence.forward_raw,
                evidence.reverse_raw,
                evidence.symmetric_raw,
            )
        })
        .unwrap_or_else(|| {
            candidate
                .bge_token_cap
                .map(|budget| {
                    format!(
                        " bge_status=BGE_TOKEN_CAP bge_left_tokens={} bge_right_tokens={} bge_pair_tokens={} bge_limit={} refactoring_required=true",
                        budget.left_tokens, budget.right_tokens, budget.pair_tokens, budget.limit
                    )
                })
                .or_else(|| {
                    candidate
                        .bge_admission
                        .map(|status| format!(" bge_status={status}"))
                })
                .unwrap_or_default()
        });
    #[cfg(feature = "nli-evidence")]
    let nli = candidate
        .nli
        .map(|evidence| {
            format!(
                " nli_status={} nli_forward={:?} nli_reverse={:?} nli_symmetric={:?}",
                candidate.nli_admission.unwrap_or("NLI_RECOMMENDED"),
                evidence.forward,
                evidence.reverse,
                evidence.symmetric,
            )
        })
        .unwrap_or_else(|| {
            candidate
                .nli_token_cap
                .map(|budget| {
                    format!(
                        " nli_status=NLI_TOKEN_CAP nli_left_tokens={} nli_right_tokens={} nli_pair_tokens={} nli_limit={} refactoring_required=true",
                        budget.left_tokens, budget.right_tokens, budget.pair_tokens, budget.limit
                    )
                })
                .or_else(|| {
                    candidate
                        .nli_admission
                        .map(|status| format!(" nli_status={status}"))
                })
                .unwrap_or_default()
        });
    #[cfg(not(feature = "nli-evidence"))]
    let nli = String::new();
    format!(
        "rank {rank}: kind={} left={} right={} relation={} assertion_shape={} facts={} distinguishing_facts={} fact_contradiction={} score={score}{bge}{nli}",
        candidate.kind,
        json_encoded(&candidate.left),
        json_encoded(&candidate.right),
        candidate.relation,
        json_encoded(&candidate.assertion_shape),
        json_encoded(&candidate.facts),
        json_encoded(&candidate.distinguishing_facts),
        candidate.fact_contradiction,
    )
}

#[cfg(feature = "spec-retrieval")]
fn semantic_review_enabled(root: &Path, change_id: &str) -> Result<()> {
    semantic_review_core(root, change_id, true)
}

/// The archive similarity gate: the same unified rulebook evaluated against
/// the current permanent tree, carrying earlier classifications, with
/// stdout-only rendering — the archive never writes the worklist.
#[cfg(feature = "spec-retrieval")]
pub fn semantic_review_archive_gate(root: &Path, change_id: &str) -> Result<()> {
    semantic_review_core(root, change_id, false)
}

#[cfg(feature = "spec-retrieval")]
fn semantic_review_core(root: &Path, change_id: &str, write_report: bool) -> Result<()> {
    report_ignored_release_overrides();
    let change_dir = validated_review_change_dir(root, change_id)?;
    let delta_root = delta_specs_path(&change_dir, change_id)?;
    let refresh = refresh_index(root, true)?;
    let delta = collect_delta_scenarios(root, &delta_root)?;
    let report_path = change_dir.join("semantic-review.md");
    let e5_started = std::time::Instant::now();
    let semantic_set = semantic_candidates(root, &refresh.index, &delta)?;
    let e5_runtime_ms = e5_started.elapsed().as_millis();
    let mut candidates = semantic_set.candidates;
    let e5_token_caps = semantic_set.e5_token_caps;
    #[cfg(feature = "cross-encoder-reranker")]
    let bge_run = apply_semantic_bge(root, &delta, &mut candidates)?;
    #[cfg(not(feature = "cross-encoder-reranker"))]
    let bge_run: Option<BgeRun> = None;
    #[cfg(feature = "nli-evidence")]
    let nli_run = apply_semantic_nli(root, &delta, &mut candidates)?;
    let pair_cache_accounting = combine_pair_cache_accounting(
        bge_run.as_ref().map(|run| &run.pair_cache_hits),
        #[cfg(feature = "nli-evidence")]
        nli_run.as_ref().map(|run| &run.pair_cache_hits),
        #[cfg(not(feature = "nli-evidence"))]
        None,
    );
    println!(
        "pair cache: {} reused, {} computed",
        pair_cache_accounting.reused, pair_cache_accounting.computed
    );
    let prior = read_prior_remediation_entries(&report_path);
    let carried_classifications = apply_prior_classifications(&report_path, &mut candidates)?;
    let remediation: Result<Vec<VerifiedRemediation>> =
        verify_semantic_remediation(root, &refresh.index, &delta, &prior, &candidates);
    let mut gate_failure: Option<String> = None;
    if write_report {
        let head_commit = super::remediation_log::git_head_commit(root);
        let log_path = change_dir.join("semantic-remediation.md");
        let bge_runtime_ms = bge_run.as_ref().and_then(|run| run.runtime_ms);
        let nli_runtime_ms = {
            #[cfg(feature = "nli-evidence")]
            {
                nli_run.as_ref().and_then(|run| run.runtime_ms)
            }
            #[cfg(not(feature = "nli-evidence"))]
            {
                None
            }
        };
        let log_candidates = log_candidate_inputs(root, &candidates);
        super::remediation_log::update_log(
            &log_path,
            change_id,
            super::remediation_log::LogRunKind::Before,
            head_commit.as_deref(),
            Some(e5_runtime_ms),
            bge_runtime_ms,
            nli_runtime_ms,
            &log_candidates,
            &[],
        )?;
        if let Ok(verified) = &remediation {
            let verified_entries = verified
                .iter()
                .map(|entry| super::remediation_log::VerifiedLogEntry {
                    candidate_id: entry.candidate_id.clone(),
                    left_evidence_hash: entry.left_evidence_hash.clone(),
                    right_evidence_hash: entry.right_evidence_hash.clone(),
                })
                .collect::<Vec<_>>();
            gate_failure = super::remediation_log::validate_log(
                &log_path,
                root,
                change_id,
                &verified_entries,
                head_commit.as_deref(),
            )
            .err()
            .map(|error| format!("{error:#}"));
            if gate_failure.is_none() {
                let completions = verified
                    .iter()
                    .map(|entry| super::remediation_log::LogCompletion {
                        candidate_id: entry.candidate_id.clone(),
                        left_evidence_hash: entry.left_evidence_hash.clone(),
                        right_evidence_hash: entry.right_evidence_hash.clone(),
                        state: match entry.classification.as_str() {
                            "VARIANT" => "outline-consolidated",
                            "VALID-BOUNDARY" => "score-below-boundary",
                            _ => "endpoint-absent",
                        },
                        after_score: entry.recomputed_score,
                        after_gherkin: if entry.classification == "DUPLICATE" {
                            None
                        } else {
                            super::remediation_log::verbatim_scenario_text(
                                root,
                                &entry.left_file,
                                &entry.left_title,
                            )
                        },
                    })
                    .collect::<Vec<_>>();
                super::remediation_log::update_log(
                    &log_path,
                    change_id,
                    super::remediation_log::LogRunKind::After,
                    head_commit.as_deref(),
                    Some(e5_runtime_ms),
                    bge_runtime_ms,
                    nli_runtime_ms,
                    &[],
                    &completions,
                )?;
            }
        }
    }
    let remediation_blocked = remediation.is_err() || gate_failure.is_some();
    let report = render_semantic_report(
        change_id,
        &candidates,
        &e5_token_caps,
        remediation_blocked,
        bge_run.as_ref(),
        #[cfg(feature = "nli-evidence")]
        nli_run.as_ref(),
        pair_cache_accounting,
    );
    if write_report {
        write_semantic_report(&report_path, &report)?;
    }
    if write_report {
        match std::fs::canonicalize(&report_path) {
            Ok(absolute) => println!("semantic review report saved: {}", absolute.display()),
            Err(_) => println!("semantic review report saved: {}", report_path.display()),
        }
    }
    if let Some(failure) = gate_failure {
        bail!("{failure}");
    }
    let verified_remediations = match remediation {
        Ok(verified) => verified.len(),
        Err(error) => bail!("{error}"),
    };
    if carried_classifications > 0 || verified_remediations > 0 {
        println!(
            "semantic remediation verified: {}",
            carried_classifications.max(verified_remediations)
        );
    }
    let unresolved = candidates
        .iter()
        .filter(|candidate| {
            candidate.bge_admission != Some("BGE_FILTERED")
                && {
                    #[cfg(feature = "nli-evidence")]
                    {
                        candidate.nli_admission != Some("NLI_FILTERED")
                    }
                    #[cfg(not(feature = "nli-evidence"))]
                    {
                        true
                    }
                }
                && (candidate.classification == "UNCERTAIN" || candidate.rationale.is_none())
        })
        .count()
        + e5_token_caps.len();
    if unresolved == 0 {
        return Ok(());
    }
    bail!(
        "semantic review blocked: {} candidate(s) remain UNCERTAIN or lack rationale; classify every candidate in semantic-review.md",
        unresolved
    )
}

#[cfg(feature = "spec-retrieval")]
fn semantic_candidates(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<SemanticCandidateSet> {
    let active: Vec<&DeltaScenario> = delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
        .collect();
    if active.is_empty() {
        return Ok(SemanticCandidateSet {
            candidates: Vec::new(),
            e5_token_caps: index.e5_token_caps.clone(),
        });
    }
    let batch = embed_texts(
        root,
        &active
            .iter()
            .map(|item| item.scenario.text.clone())
            .collect::<Vec<_>>(),
        EmbeddingRole::Passage,
    )?;
    let mut e5_token_caps = index.e5_token_caps.clone();
    for (item, budget) in active.iter().zip(batch.token_caps.iter()) {
        if let Some(budget) = budget {
            e5_token_caps.push(e5_token_cap(&item.scenario, *budget));
        }
    }
    e5_token_caps.sort_by(e5_token_cap_order);
    let permanent = permanent_semantic_endpoints(root)?;
    let mut candidates = BTreeMap::new();
    for (active_index, item) in active.iter().enumerate() {
        let Some(vector) = batch.vectors[active_index].as_ref() else {
            continue;
        };
        for entry in &index.entries {
            if item.operation == crate::gherkin::delta::DeltaOp::Modified
                && entry.file == item.target_file
                && entry.title == item.scenario.title
            {
                continue;
            }
            let left = semantic_delta_endpoint(item);
            let right = permanent
                .get(&(entry.file.clone(), entry.title.clone()))
                .cloned()
                .unwrap_or_else(|| SemanticEndpoint {
                    scope: "permanent",
                    file: entry.file.clone(),
                    title: entry.title.clone(),
                    content_hash: entry.content_hash.clone(),
                    evidence_hash: entry.content_hash.clone(),
                    evidence_json: "{}".to_string(),
                    operation: "permanent".to_string(),
                    projected_target: None,
                    facts: crate::overlap::fingerprint::Fingerprint {
                        assertions: Vec::new(),
                        facts: Default::default(),
                    },
                });
            let score = semantic_score(&left, &right, cosine_similarity(vector, &entry.embedding))?;
            if score < E5_ADMISSION_THRESHOLD {
                continue;
            }
            let candidate = semantic_candidate("delta-to-permanent", left, right, score);
            candidates.insert(candidate.candidate_id.clone(), candidate);
        }
    }
    for left_index in 0..active.len() {
        for right_index in (left_index + 1)..active.len() {
            let (Some(left_vector), Some(right_vector)) = (
                batch.vectors[left_index].as_ref(),
                batch.vectors[right_index].as_ref(),
            ) else {
                continue;
            };
            let left = semantic_delta_endpoint(active[left_index]);
            let right = semantic_delta_endpoint(active[right_index]);
            let (left, right) = if semantic_endpoint_key(&left) <= semantic_endpoint_key(&right) {
                (left, right)
            } else {
                (right, left)
            };
            let score =
                semantic_score(&left, &right, cosine_similarity(left_vector, right_vector))?;
            if score < E5_ADMISSION_THRESHOLD {
                continue;
            }
            let candidate = semantic_candidate("delta-to-delta", left, right, score);
            candidates.insert(candidate.candidate_id.clone(), candidate);
        }
    }
    let mut result: Vec<SemanticCandidate> = candidates.into_values().collect();
    result.sort_by(|left, right| {
        left.scope
            .cmp(right.scope)
            .then_with(|| right.score.total_cmp(&left.score))
            .then_with(|| {
                semantic_endpoint_key(&left.left).cmp(&semantic_endpoint_key(&right.left))
            })
            .then_with(|| {
                semantic_endpoint_key(&left.right).cmp(&semantic_endpoint_key(&right.right))
            })
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
    });
    let result = apply_hybrid_selection(result);
    Ok(SemanticCandidateSet {
        candidates: result,
        e5_token_caps,
    })
}

/// Unified rulebook hybrid selection (ADR-066): per delta scenario keep at
/// most `NEIGHBOURS_PER_SCENARIO` highest-scored pairs, then cap the union at
/// `PAIR_CAP` pairs by score with deterministic endpoint tie order. Pairs
/// dropped here are unselected — they are not candidates and carry no
/// blocking state.
#[cfg(feature = "spec-retrieval")]
fn apply_hybrid_selection(candidates: Vec<SemanticCandidate>) -> Vec<SemanticCandidate> {
    let mut by_scenario: BTreeMap<(String, String, String), Vec<&SemanticCandidate>> =
        BTreeMap::new();
    for candidate in &candidates {
        for endpoint in [&candidate.left, &candidate.right] {
            if endpoint.scope == "current-change" {
                by_scenario
                    .entry(semantic_endpoint_key(endpoint))
                    .or_default()
                    .push(candidate);
            }
        }
    }
    let mut selected: BTreeMap<String, SemanticCandidate> = BTreeMap::new();
    for neighbours in by_scenario.values_mut() {
        neighbours.sort_by(|a, b| semantic_candidate_order(a, b));
        neighbours.truncate(NEIGHBOURS_PER_SCENARIO);
        for candidate in neighbours.drain(..) {
            selected.insert(candidate.candidate_id.clone(), candidate.clone());
        }
    }
    let mut result: Vec<SemanticCandidate> = selected.into_values().collect();
    result.sort_by(semantic_candidate_order);
    result.truncate(PAIR_CAP);
    result
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn apply_semantic_bge(
    root: &Path,
    delta: &[DeltaScenario],
    candidates: &mut [SemanticCandidate],
) -> Result<Option<BgeRun>> {
    if candidates.is_empty() {
        return Ok(None);
    }
    let Some(backend) = bge_backend(root)? else {
        return Ok(None);
    };
    let max_unique_pairs = max_rerank_pairs()?;
    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    let permanent_texts: BTreeMap<(String, String), String> = collect_scenarios(root)?
        .into_iter()
        .map(|scenario| ((scenario.file, scenario.title), scenario.text))
        .collect();
    let delta_texts: BTreeMap<(String, String), String> = delta
        .iter()
        .map(|item| {
            (
                (item.scenario.file.clone(), item.scenario.title.clone()),
                item.scenario.text.clone(),
            )
        })
        .collect();
    let mut texts = permanent_texts;
    texts.extend(delta_texts);
    if let Some((bounded_count, raw_count)) = bge_test_pool()? {
        let (pairs, synthetic_texts) = synthetic_bge_pairs(bounded_count.min(max_unique_pairs));
        let mut run = match backend {
            Ok(kind) => score_bge_pairs(root, &pairs, &synthetic_texts, kind, max_unique_pairs)?,
            Err(reason) => BgeRun::unavailable(reason, raw_count, max_unique_pairs),
        };
        run.e5_candidates = raw_count;
        run.unscored = raw_count.saturating_sub(pairs.len());
        for candidate in candidates {
            candidate.bge_admission = Some(if run.unscored > 0 {
                "BGE_UNSCORED"
            } else {
                "BGE_RECOMMENDED"
            });
        }
        return Ok(Some(run));
    }
    let mut selected = candidates.to_vec();
    selected.sort_by(semantic_candidate_order);
    selected.truncate(max_unique_pairs);
    let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = selected
        .iter()
        .map(|candidate| {
            (
                endpoint(&candidate.left.file, &candidate.left.title),
                endpoint(&candidate.right.file, &candidate.right.title),
            )
        })
        .collect();
    let tokenizer = match backend {
        Ok(BgeBackendKind::Local | BgeBackendKind::Auto)
            if spec_env_var("GIVN_SPEC_BGE_LOCAL_STUB").as_deref() != Ok("1") =>
        {
            let automatic = matches!(backend, Ok(BgeBackendKind::Auto));
            Some(load_unbounded_tokenizer(
                &bge_artifact_directory(root, automatic)?.join("tokenizer.json"),
            )?)
        }
        _ => None,
    };
    let partition =
        preflight_pair_budgets(&pairs, &texts, TokenBudgetModel::Bge, tokenizer.as_ref())?;
    let model_id = match &backend {
        Ok(BgeBackendKind::Stub) => PAIR_CACHE_STUB_MODEL,
        Ok(_) => RERANKER_MODEL_ID,
        Err(_) => RERANKER_MODEL_ID,
    };
    let mut cache = PairScoreCache::load(root);
    let mut cached_bge_evidence: BTreeMap<
        (SimilarityEndpoint, SimilarityEndpoint),
        BgePairEvidence,
    > = BTreeMap::new();
    let mut pair_cache_hits: Vec<(String, bool)> = Vec::with_capacity(selected.len());
    let mut miss_indices: Vec<usize> = Vec::new();
    let mut miss_pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = Vec::new();
    for (index, candidate) in selected.iter().enumerate() {
        let capped = partition.capped.contains_key(&canonical_pair_key(
            endpoint(&candidate.left.file, &candidate.left.title),
            endpoint(&candidate.right.file, &candidate.right.title),
        ));
        let cache_key = pair_cache_key("bge", model_id, &candidate.left, &candidate.right);
        let hit = (!capped)
            .then(|| cache.get_raw(&cache_key))
            .flatten()
            .and_then(|value| {
                let forward_raw = value.get("forward_raw")?.as_f64()? as f32;
                let reverse_raw = value.get("reverse_raw")?.as_f64()? as f32;
                Some(BgePairEvidence {
                    forward_raw,
                    reverse_raw,
                    symmetric_raw: (forward_raw + reverse_raw) / 2.0,
                })
            });
        match hit {
            Some(evidence) => {
                pair_cache_hits.push((candidate.candidate_id.clone(), true));
                // Reused evidence joins the run through its canonical pair
                // key, so admission assignment below stays uniform.
                cached_bge_evidence.insert(
                    canonical_pair_key(
                        endpoint(&candidate.left.file, &candidate.left.title),
                        endpoint(&candidate.right.file, &candidate.right.title),
                    ),
                    evidence,
                );
            }
            None => {
                pair_cache_hits.push((candidate.candidate_id.clone(), false));
                miss_indices.push(index);
                miss_pairs.push(pairs[index].clone());
            }
        }
    }
    let mut run = match backend {
        Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
            BgeRun::unavailable(
                BgeUnavailableReason::TokenCap,
                candidates.len(),
                max_unique_pairs,
            )
        }
        Ok(_) if miss_pairs.is_empty() => BgeRun {
            status: "BGE_ENABLED",
            reason: None,
            evidence: cached_bge_evidence.clone(),
            token_caps: BTreeMap::new(),
            e5_candidates: candidates.len(),
            unscored: 0,
            max_unique_pairs,
            model_load_attempts: 0,
            runtime_ms: Some(0),
            pair_cache_hits: Vec::new(),
        },
        Ok(kind) => score_bge_pairs(root, &miss_pairs, &texts, kind, max_unique_pairs)?,
        Err(reason) => BgeRun::unavailable(reason, candidates.len(), max_unique_pairs),
    };
    if !miss_pairs.is_empty() {
        // Persist freshly computed evidence so reruns reuse it.
        for (position, index) in miss_indices.iter().enumerate() {
            let candidate = &selected[*index];
            let (left, right) = &miss_pairs[position];
            if let Some(evidence) = run
                .evidence
                .get(&canonical_pair_key(left.clone(), right.clone()))
            {
                cache.put_raw(
                    pair_cache_key("bge", model_id, &candidate.left, &candidate.right),
                    serde_json::json!({
                        "forward_raw": evidence.forward_raw,
                        "reverse_raw": evidence.reverse_raw,
                    }),
                );
            }
        }
        if cache.store(root).is_err() {
            // The cache never changes verdicts; a write failure only means
            // the next run re-judges. Keep going.
        }
    }
    // Cached entries join the run evidence so admission assignment stays
    // uniform across cached and freshly computed pairs.
    for (key, evidence) in &cached_bge_evidence {
        run.evidence.entry(key.clone()).or_insert(evidence.clone());
    }
    run.pair_cache_hits = pair_cache_hits;
    run.e5_candidates = candidates.len();
    run.unscored = candidates.len().saturating_sub(run.evidence.len());
    run.token_caps = partition.capped.clone();
    for candidate in candidates {
        let key = canonical_pair_key(
            endpoint(&candidate.left.file, &candidate.left.title),
            endpoint(&candidate.right.file, &candidate.right.title),
        );
        if let Some(budget) = partition.capped.get(&key).copied() {
            candidate.bge = None;
            candidate.bge_token_cap = Some(budget);
            candidate.bge_admission = Some("BGE_TOKEN_CAP");
        } else if let Some(evidence) = run.evidence.get(&key).cloned() {
            candidate.bge_token_cap = None;
            candidate.bge_admission = Some(if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
                "BGE_RECOMMENDED"
            } else {
                "BGE_FILTERED"
            });
            candidate.bge = Some(evidence);
        } else {
            candidate.bge_token_cap = None;
            candidate.bge_admission = Some("BGE_UNSCORED");
        }
    }
    Ok(Some(run))
}

#[cfg(feature = "nli-evidence")]
fn apply_semantic_nli(
    root: &Path,
    delta: &[DeltaScenario],
    candidates: &mut [SemanticCandidate],
) -> Result<Option<NliRun>> {
    if candidates.is_empty() {
        return Ok(None);
    }
    let Some(selection) = nli_backend_selection(root)? else {
        return Ok(None);
    };
    let max_unique_pairs = max_rerank_pairs()?;
    let permanent_texts: BTreeMap<(String, String), String> = collect_scenarios(root)?
        .into_iter()
        .map(|scenario| ((scenario.file, scenario.title), scenario.text))
        .collect();
    let delta_texts: BTreeMap<(String, String), String> = delta
        .iter()
        .map(|item| {
            (
                (item.scenario.file.clone(), item.scenario.title.clone()),
                item.scenario.text.clone(),
            )
        })
        .collect();
    let mut texts = permanent_texts;
    texts.extend(delta_texts);
    let mut selected = candidates.to_vec();
    selected.sort_by(semantic_candidate_order);
    selected.truncate(max_unique_pairs);
    let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = selected
        .iter()
        .map(|candidate| {
            (
                endpoint(&candidate.left.file, &candidate.left.title),
                endpoint(&candidate.right.file, &candidate.right.title),
            )
        })
        .collect();
    let tokenizer = match selection {
        Ok(NliBackend::Local | NliBackend::Auto) => {
            let artifacts =
                nli_artifacts_for_backend(root, matches!(selection, Ok(NliBackend::Auto)))?;
            Some(load_unbounded_tokenizer(&artifacts.tokenizer_path)?)
        }
        _ => None,
    };
    let partition =
        preflight_pair_budgets(&pairs, &texts, TokenBudgetModel::Nli, tokenizer.as_ref())?;
    let model_id = match &selection {
        Ok(NliBackend::Stub) => PAIR_CACHE_STUB_MODEL,
        Ok(_) => NLI_MODEL_ID,
        Err(_) => NLI_MODEL_ID,
    };
    let mut cache = PairScoreCache::load(root);
    let mut cached_nli_evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), NliEvidence> =
        BTreeMap::new();
    let mut pair_cache_hits: Vec<(String, bool)> = Vec::with_capacity(selected.len());
    let mut miss_indices: Vec<usize> = Vec::new();
    let mut miss_pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = Vec::new();
    for (index, candidate) in selected.iter().enumerate() {
        let capped = partition.capped.contains_key(&canonical_pair_key(
            endpoint(&candidate.left.file, &candidate.left.title),
            endpoint(&candidate.right.file, &candidate.right.title),
        ));
        let cache_key = pair_cache_key("nli", model_id, &candidate.left, &candidate.right);
        let hit = (!capped)
            .then(|| cache.get_raw(&cache_key))
            .flatten()
            .and_then(|value| {
                let forward: Vec<f32> = value
                    .get("forward")?
                    .as_array()?
                    .iter()
                    .map(|item| item.as_f64().map(|number| number as f32))
                    .collect::<Option<Vec<_>>>()?;
                let reverse: Vec<f32> = value
                    .get("reverse")?
                    .as_array()?
                    .iter()
                    .map(|item| item.as_f64().map(|number| number as f32))
                    .collect::<Option<Vec<_>>>()?;
                if forward.len() != 3 || reverse.len() != 3 {
                    return None;
                }
                Some((
                    [forward[0], forward[1], forward[2]],
                    [reverse[0], reverse[1], reverse[2]],
                ))
            })
            .map(|(forward_logits, reverse_logits)| nli_evidence(forward_logits, reverse_logits));
        match hit {
            Some(evidence) => {
                pair_cache_hits.push((candidate.candidate_id.clone(), true));
                cached_nli_evidence.insert(
                    canonical_pair_key(
                        endpoint(&candidate.left.file, &candidate.left.title),
                        endpoint(&candidate.right.file, &candidate.right.title),
                    ),
                    evidence,
                );
            }
            None => {
                pair_cache_hits.push((candidate.candidate_id.clone(), false));
                miss_indices.push(index);
                miss_pairs.push(pairs[index].clone());
            }
        }
    }
    let mut run = match selection {
        Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
            NliRun::unavailable(
                NliUnavailableReason::TokenCap,
                candidates.len(),
                max_unique_pairs,
            )
        }
        Ok(_) if miss_pairs.is_empty() => NliRun {
            status: "NLI_ENABLED",
            reason: None,
            evidence: cached_nli_evidence.clone(),
            token_caps: BTreeMap::new(),
            e5_candidates: candidates.len(),
            unscored: 0,
            max_unique_pairs,
            model_load_attempts: 0,
            runtime_ms: Some(0),
            pair_cache_hits: Vec::new(),
        },
        Ok(backend) => score_nli_pairs(root, &miss_pairs, &texts, backend, max_unique_pairs)?,
        Err(reason) => NliRun::unavailable(reason, candidates.len(), max_unique_pairs),
    };
    if !miss_pairs.is_empty() {
        // Persist freshly computed evidence so reruns reuse it.
        for (position, index) in miss_indices.iter().enumerate() {
            let candidate = &selected[*index];
            let (left, right) = &miss_pairs[position];
            if let Some(evidence) = run
                .evidence
                .get(&canonical_pair_key(left.clone(), right.clone()))
            {
                cache.put_raw(
                    pair_cache_key("nli", model_id, &candidate.left, &candidate.right),
                    serde_json::json!({
                        "forward": [evidence.forward[0], evidence.forward[1], evidence.forward[2]],
                        "reverse": [evidence.reverse[0], evidence.reverse[1], evidence.reverse[2]],
                    }),
                );
            }
        }
        if cache.store(root).is_err() {
            // A write failure only means the next run re-judges.
        }
    }
    for (key, evidence) in &cached_nli_evidence {
        run.evidence.entry(key.clone()).or_insert(*evidence);
    }
    run.pair_cache_hits = pair_cache_hits;
    run.e5_candidates = candidates.len();
    run.unscored = candidates.len().saturating_sub(run.evidence.len());
    run.token_caps = partition.capped.clone();
    for candidate in candidates {
        let key = canonical_pair_key(
            endpoint(&candidate.left.file, &candidate.left.title),
            endpoint(&candidate.right.file, &candidate.right.title),
        );
        if let Some(budget) = partition.capped.get(&key).copied() {
            candidate.nli = None;
            candidate.nli_token_cap = Some(budget);
            candidate.nli_admission = Some("NLI_TOKEN_CAP");
        } else if let Some(evidence) = run.evidence.get(&key).copied() {
            candidate.nli_token_cap = None;
            candidate.nli = Some(evidence);
            candidate.nli_admission = Some(if evidence.symmetric[0] >= NLI_ADMISSION_THRESHOLD {
                "NLI_RECOMMENDED"
            } else {
                "NLI_FILTERED"
            });
        } else {
            candidate.nli_token_cap = None;
            candidate.nli_admission = Some("NLI_UNSCORED");
        }
    }
    Ok(Some(run))
}

#[cfg(feature = "spec-retrieval")]
fn semantic_score(left: &SemanticEndpoint, right: &SemanticEndpoint, raw: f32) -> Result<f32> {
    if spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(raw);
    }
    let Some(overrides) = spec_env("GIVN_SPEC_STUB_PAIR_SCORES") else {
        return Ok(raw);
    };
    let key = format!("{}|{}", left.title, right.title);
    let reverse_key = format!("{}|{}", right.title, left.title);
    for entry in overrides.to_string_lossy().split(';') {
        let Some((pair, score)) = entry.split_once('=') else {
            continue;
        };
        if pair == key || pair == reverse_key {
            return score
                .parse::<f32>()
                .with_context(|| format!("invalid semantic stub score `{score}`"));
        }
    }
    Ok(raw)
}

#[cfg(feature = "spec-retrieval")]
fn permanent_semantic_endpoints(
    root: &Path,
) -> Result<BTreeMap<(String, String), SemanticEndpoint>> {
    let mut endpoints = BTreeMap::new();
    for path in crate::gherkin::parse::collect_feature_files(&permanent_specs_path(root)) {
        let feature = parse_permanent_feature(root, &path)?;
        let file = normalize_project_file_identity(root, &path);
        for scenario in &feature.scenarios {
            let endpoint = semantic_endpoint_from_scenario(
                &feature,
                None,
                scenario,
                &file,
                "permanent",
                "permanent",
                None,
            );
            endpoints.insert((file.clone(), scenario.name.clone()), endpoint);
        }
        for rule in &feature.rules {
            for scenario in &rule.scenarios {
                let endpoint = semantic_endpoint_from_scenario(
                    &feature,
                    Some(rule),
                    scenario,
                    &file,
                    "permanent",
                    "permanent",
                    None,
                );
                endpoints.insert((file.clone(), scenario.name.clone()), endpoint);
            }
        }
    }
    Ok(endpoints)
}

#[cfg(feature = "spec-retrieval")]
fn semantic_delta_endpoint(item: &DeltaScenario) -> SemanticEndpoint {
    SemanticEndpoint {
        scope: "current-change",
        file: item.scenario.file.clone(),
        title: item.scenario.title.clone(),
        content_hash: item.scenario.content_hash.clone(),
        evidence_hash: semantic_evidence_hash(
            &item.evidence_json,
            &format_delta_operation(&item.operation),
            Some(&item.target_file),
        ),
        evidence_json: item.evidence_json.clone(),
        operation: format_delta_operation(&item.operation),
        projected_target: Some((item.target_file.clone(), item.scenario.title.clone())),
        facts: item.facts.clone(),
    }
}

#[cfg(feature = "spec-retrieval")]
fn semantic_endpoint_from_scenario(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    scenario: &gherkin::Scenario,
    file: &str,
    scope: &'static str,
    operation: &str,
    projected_target: Option<(String, String)>,
) -> SemanticEndpoint {
    let text = scenario_text(feature, rule, scenario, file);
    let evidence_json = canonical_evidence_json(feature, rule, scenario);
    let facts = crate::overlap::fingerprint::Fingerprint::of_scenario(
        scenario,
        rule.and_then(|rule| rule.background.as_ref())
            .or(feature.background.as_ref()),
    );
    SemanticEndpoint {
        scope,
        file: file.to_string(),
        title: scenario.name.clone(),
        evidence_hash: semantic_evidence_hash(&evidence_json, operation, None),
        evidence_json,
        content_hash: text.content_hash,
        operation: operation.to_string(),
        projected_target,
        facts,
    }
}

#[cfg(feature = "spec-retrieval")]
fn semantic_evidence_hash(
    evidence_json: &str,
    operation: &str,
    projected_target: Option<&String>,
) -> String {
    format!(
        "{:016x}",
        stable_hash(
            format!(
                "{}|{}|{:?}|{}|{}|{}|{}|{}",
                evidence_json,
                operation,
                projected_target,
                MODEL_ID,
                INDEX_VERSION,
                SEMANTIC_FINGERPRINT_SCHEMA_VERSION,
                SEMANTIC_EVIDENCE_SCHEMA_VERSION,
                SEMANTIC_REPORT_SCHEMA_VERSION
            )
            .as_bytes(),
        )
    )
}

#[cfg(feature = "spec-retrieval")]
fn format_delta_operation(operation: &crate::gherkin::delta::DeltaOp) -> String {
    match operation {
        crate::gherkin::delta::DeltaOp::Added => "added".to_string(),
        crate::gherkin::delta::DeltaOp::Modified => "modified".to_string(),
        crate::gherkin::delta::DeltaOp::Removed => "removed".to_string(),
    }
}

#[cfg(feature = "spec-retrieval")]
fn semantic_endpoint_key(endpoint: &SemanticEndpoint) -> (String, String, String) {
    (
        endpoint.scope.to_string(),
        endpoint.file.clone(),
        endpoint.title.clone(),
    )
}

#[cfg(feature = "spec-retrieval")]
fn semantic_candidate(
    scope: &'static str,
    left: SemanticEndpoint,
    right: SemanticEndpoint,
    score: f32,
) -> SemanticCandidate {
    let score_text = format!("{score:.4}");
    let identity = format!(
        "{scope}|{}|{}|{}|{}|{}|{}",
        left.scope, left.file, left.title, right.scope, right.file, right.title
    );
    let candidate_id = format!("{:016x}", stable_hash(identity.as_bytes()));
    SemanticCandidate {
        scope,
        left,
        right,
        score: score_text.parse().unwrap_or(score),
        candidate_id,
        classification: "UNCERTAIN".to_string(),
        rationale: None,
        bge: None,
        bge_admission: None,
        bge_token_cap: None,
        #[cfg(feature = "nli-evidence")]
        nli: None,
        #[cfg(feature = "nli-evidence")]
        nli_admission: None,
        #[cfg(feature = "nli-evidence")]
        nli_token_cap: None,
    }
}

#[cfg(feature = "spec-retrieval")]
fn apply_prior_classifications(path: &Path, candidates: &mut [SemanticCandidate]) -> Result<usize> {
    let Ok(content) = fs::read_to_string(path) else {
        return Ok(0);
    };
    let mut seen_ids = BTreeSet::new();
    for line in content.lines() {
        if let Some(id) = line.strip_prefix("- candidate_id: ")
            && !seen_ids.insert(id.trim().to_string())
        {
            return Ok(0);
        }
    }
    let mut current_id: Option<String> = None;
    let mut current_classification: Option<String> = None;
    let mut current_rationale: Option<String> = None;
    let mut current_left_hash: Option<String> = None;
    let mut current_right_hash: Option<String> = None;
    let mut prior = BTreeMap::new();
    for line in content.lines() {
        if let Some(value) = line.strip_prefix("- candidate_id: ") {
            if let (Some(id), Some(classification), Some(rationale), Some(left), Some(right)) = (
                current_id.take(),
                current_classification.take(),
                current_rationale.take(),
                current_left_hash.take(),
                current_right_hash.take(),
            ) {
                prior
                    .entry(id)
                    .or_insert((classification, rationale, left, right));
            }
            current_id = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("- classification: ") {
            current_classification = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("- rationale: ") {
            let value = value.trim();
            if value != "null" && !value.is_empty() {
                current_rationale = Some(value.trim_matches('"').to_string());
            }
        } else if let Some(value) = line.strip_prefix("- left_evidence_hash: ") {
            current_left_hash = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("- right_evidence_hash: ") {
            current_right_hash = Some(value.trim().to_string());
        }
    }
    if let (Some(id), Some(classification), Some(rationale), Some(left), Some(right)) = (
        current_id,
        current_classification,
        current_rationale,
        current_left_hash,
        current_right_hash,
    ) {
        prior
            .entry(id)
            .or_insert((classification, rationale, left, right));
    }
    let mut carried = 0;
    for candidate in candidates {
        let Some((classification, rationale, left, right)) = prior.get(&candidate.candidate_id)
        else {
            continue;
        };
        if matches!(
            classification.as_str(),
            "DUPLICATE" | "VARIANT" | "VALID-BOUNDARY"
        ) && left == &candidate.left.evidence_hash
            && right == &candidate.right.evidence_hash
            && !rationale.is_empty()
        {
            candidate.classification = classification.clone();
            candidate.rationale = Some(rationale.clone());
            carried += 1;
        }
    }
    Ok(carried)
}

#[cfg(feature = "spec-retrieval")]
fn read_prior_remediation_entries(path: &Path) -> Vec<PriorRemediationEntry> {
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    let mut fields = BTreeMap::new();
    for line in content.lines().chain(std::iter::once("## Candidate end")) {
        if line.starts_with("## Candidate") && !fields.is_empty() {
            if let Some(entry) = prior_remediation_entry(&fields) {
                entries.push(entry);
            }
            fields.clear();
        }
        let Some(value) = line.strip_prefix("- ") else {
            continue;
        };
        let Some((key, value)) = value.split_once(": ") else {
            continue;
        };
        fields.insert(key.to_string(), report_scalar(value));
    }
    entries
}

#[cfg(feature = "spec-retrieval")]
fn prior_remediation_entry(fields: &BTreeMap<String, String>) -> Option<PriorRemediationEntry> {
    let classification = fields.get("classification")?;
    if !matches!(
        classification.as_str(),
        "DUPLICATE" | "VARIANT" | "VALID-BOUNDARY"
    ) {
        return None;
    }
    let rationale = fields.get("rationale")?;
    if rationale.is_empty() || rationale == "null" {
        return None;
    }
    Some(PriorRemediationEntry {
        candidate_id: fields.get("candidate_id")?.clone(),
        classification: classification.clone(),
        rationale: rationale.clone(),
        left_scope: fields.get("left_scope")?.clone(),
        left_file: fields.get("left_file")?.clone(),
        left_title: fields.get("left_title")?.clone(),
        left_content_hash: fields.get("left_content_hash")?.clone(),
        left_evidence_hash: fields.get("left_evidence_hash")?.clone(),
        right_scope: fields.get("right_scope")?.clone(),
        right_file: fields.get("right_file")?.clone(),
        right_title: fields.get("right_title")?.clone(),
        right_content_hash: fields.get("right_content_hash")?.clone(),
        right_evidence_hash: fields.get("right_evidence_hash")?.clone(),
    })
}

#[cfg(feature = "spec-retrieval")]
fn report_scalar(value: &str) -> String {
    value.trim().trim_matches('"').replace("\\\"", "\"")
}

#[cfg(feature = "spec-retrieval")]
fn log_candidate_inputs(root: &Path, candidates: &[SemanticCandidate]) -> Vec<LogCandidateInput> {
    candidates
        .iter()
        .filter(|candidate| {
            candidate.bge_admission != Some("BGE_FILTERED") && {
                #[cfg(feature = "nli-evidence")]
                {
                    candidate.nli_admission != Some("NLI_FILTERED")
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    true
                }
            }
        })
        .map(|candidate| LogCandidateInput {
            candidate_id: candidate.candidate_id.clone(),
            embedding_score: candidate.score,
            bge_admission: {
                #[cfg(feature = "cross-encoder-reranker")]
                {
                    Some(candidate.bge_admission.unwrap_or("BGE_UNSCORED").to_string())
                }
                #[cfg(not(feature = "cross-encoder-reranker"))]
                {
                    candidate.bge_admission.map(|value| value.to_string())
                }
            },
            bge_symmetric_raw: candidate
                .bge
                .as_ref()
                .map(|evidence| evidence.symmetric_raw),
            nli_admission: {
                #[cfg(feature = "nli-evidence")]
                {
                    Some(candidate.nli_admission.unwrap_or("NLI_UNSCORED").to_string())
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    None
                }
            },
            nli_symmetric_entailment: {
                #[cfg(feature = "nli-evidence")]
                {
                    candidate.nli.as_ref().map(|evidence| evidence.symmetric[0])
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    None
                }
            },
            left_file: candidate.left.file.clone(),
            left_title: candidate.left.title.clone(),
            right_file: candidate.right.file.clone(),
            right_title: candidate.right.title.clone(),
            left_content_hash: candidate.left.content_hash.clone(),
            right_content_hash: candidate.right.content_hash.clone(),
            left_evidence_hash: candidate.left.evidence_hash.clone(),
            right_evidence_hash: candidate.right.evidence_hash.clone(),
            left_gherkin: super::remediation_log::verbatim_scenario_text(
                root,
                &candidate.left.file,
                &candidate.left.title,
            ),
            right_gherkin: super::remediation_log::verbatim_scenario_text(
                root,
                &candidate.right.file,
                &candidate.right.title,
            ),
            classification: candidate.classification.clone(),
            rationale: candidate.rationale.clone(),
        })
        .collect()
}

#[cfg(feature = "spec-retrieval")]
fn verify_semantic_remediation(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    prior: &[PriorRemediationEntry],
    candidates: &[SemanticCandidate],
) -> Result<Vec<VerifiedRemediation>> {
    let permanent = permanent_semantic_endpoints(root)?;
    let mut verified: Vec<VerifiedRemediation> = Vec::new();
    for entry in prior {
        let delta_to_delta = entry.right_scope == "current-change";
        if entry.rationale.is_empty()
            || entry.left_scope != "current-change"
            || !matches!(entry.right_scope.as_str(), "permanent" | "current-change")
            || entry.left_file.is_empty()
            || entry.left_title.is_empty()
            || entry.left_content_hash.is_empty()
            || entry.left_evidence_hash.is_empty()
            || entry.right_content_hash.is_empty()
            || entry.right_evidence_hash.is_empty()
        {
            bail!(
                "semantic classification is stale (candidate {}): prior record is incomplete — classification, rationale, or endpoint identity/evidence hashes are missing; reclassify the candidate in semantic-review.md",
                entry.candidate_id
            );
        }
        let current_candidate = candidates
            .iter()
            .find(|candidate| candidate.candidate_id == entry.candidate_id);
        let current_left = delta.iter().find(|item| {
            item.scenario.file == entry.left_file
                && item.scenario.title == entry.left_title
                && item.operation != crate::gherkin::delta::DeltaOp::Removed
        });
        let current_right = delta.iter().find(|item| {
            item.scenario.file == entry.right_file
                && item.scenario.title == entry.right_title
                && item.operation != crate::gherkin::delta::DeltaOp::Removed
        });
        let right = (!delta_to_delta)
            .then(|| permanent.get(&(entry.right_file.clone(), entry.right_title.clone())))
            .flatten();
        match entry.classification.as_str() {
            "DUPLICATE" => {
                if delta_to_delta {
                    verify_duplicate_delta_to_delta(root, entry, current_left, current_right)?;
                    verified.push(VerifiedRemediation {
                        candidate_id: entry.candidate_id.clone(),
                        left_evidence_hash: entry.left_evidence_hash.clone(),
                        right_evidence_hash: entry.right_evidence_hash.clone(),
                        classification: entry.classification.clone(),
                        recomputed_score: None,
                        left_file: entry.left_file.clone(),
                        left_title: entry.left_title.clone(),
                    });
                    continue;
                }
                if let Some(left) = current_left {
                    let endpoint = semantic_delta_endpoint(left);
                    if endpoint.content_hash != entry.left_content_hash
                        || endpoint.evidence_hash != entry.left_evidence_hash
                    {
                        bail!(
                            "semantic classification is stale (candidate {}): the classified endpoint changed after classification — recorded content/evidence hashes no longer match; apply the projected spec change and rerun",
                            entry.candidate_id
                        );
                    }
                }
                let right_mismatch = right.is_none_or(|endpoint| {
                    endpoint.content_hash != entry.right_content_hash
                        || endpoint.evidence_hash != entry.right_evidence_hash
                });
                if current_candidate.is_some() || current_left.is_some() || right_mismatch {
                    let reason = if let Some(current) = current_candidate {
                        format!(
                            "the pair is still admitted at embedding score {:.4} (>= {:.4}) and the redundant endpoint is still present",
                            current.score, E5_ADMISSION_THRESHOLD,
                        )
                    } else if current_left.is_some() {
                        format!(
                            "the redundant endpoint \"{}\" is still present in the active delta; remove it (or fold its distinct invariant into the retained endpoint)",
                            entry.left_title
                        )
                    } else {
                        "the retained permanent endpoint no longer matches the recorded evidence"
                            .to_string()
                    };
                    bail!(
                        "DUPLICATE remediation requires removing the redundant scenario (candidate {}): {reason}",
                        entry.candidate_id
                    );
                }
                verified.push(VerifiedRemediation {
                    candidate_id: entry.candidate_id.clone(),
                    left_evidence_hash: entry.left_evidence_hash.clone(),
                    right_evidence_hash: entry.right_evidence_hash.clone(),
                    classification: entry.classification.clone(),
                    recomputed_score: None,
                    left_file: entry.left_file.clone(),
                    left_title: entry.left_title.clone(),
                });
            }
            "VARIANT" => {
                if delta_to_delta {
                    verify_variant_delta_to_delta(
                        entry,
                        current_candidate,
                        current_left,
                        current_right,
                    )?;
                    verified.push(VerifiedRemediation {
                        candidate_id: entry.candidate_id.clone(),
                        left_evidence_hash: entry.left_evidence_hash.clone(),
                        right_evidence_hash: entry.right_evidence_hash.clone(),
                        classification: entry.classification.clone(),
                        recomputed_score: None,
                        left_file: entry.left_file.clone(),
                        left_title: entry.left_title.clone(),
                    });
                    continue;
                }
                let Some(left) = current_left else {
                    bail!(
                        "semantic classification is stale (candidate {}): the left endpoint no longer exists after rerun; regenerate the report and reclassify",
                        entry.candidate_id
                    );
                };
                let Some(right) = right else {
                    bail!(
                        "semantic classification is stale (candidate {}): the permanent right endpoint no longer exists after rerun; regenerate the report and reclassify",
                        entry.candidate_id
                    );
                };
                let score = recompute_prior_pair_score(root, index, left, right)?;
                let left_endpoint = semantic_delta_endpoint(left);
                if current_candidate.is_some()
                    || !left.is_outline
                    || !left.has_examples
                    || left_endpoint.content_hash == entry.left_content_hash
                    || left_endpoint.evidence_hash == entry.left_evidence_hash
                    || right.content_hash != entry.right_content_hash
                    || right.evidence_hash != entry.right_evidence_hash
                    || score >= E5_ADMISSION_THRESHOLD
                {
                    let reason = if current_candidate.is_some() {
                        format!(
                            "the pair is still admitted (recomputed score {score:.4} >= {E5_ADMISSION_THRESHOLD:.4}); the variant is not consolidated"
                        )
                    } else if !left.is_outline || !left.has_examples {
                        "the active scenario is not a parameterized Scenario Outline with a non-empty Examples table".to_string()
                    } else if score >= E5_ADMISSION_THRESHOLD {
                        format!(
                            "recomputed score {score:.4} is not strictly below {E5_ADMISSION_THRESHOLD:.4}"
                        )
                    } else if left_endpoint.content_hash == entry.left_content_hash
                        || left_endpoint.evidence_hash == entry.left_evidence_hash
                    {
                        "the active endpoint was not consolidated — recorded content/evidence hashes are unchanged".to_string()
                    } else {
                        "the permanent endpoint changed after classification — the prior classification is stale".to_string()
                    };
                    bail!(
                        "VARIANT remediation requires consolidation (candidate {}): {reason}",
                        entry.candidate_id
                    );
                }
                verified.push(VerifiedRemediation {
                    candidate_id: entry.candidate_id.clone(),
                    left_evidence_hash: entry.left_evidence_hash.clone(),
                    right_evidence_hash: entry.right_evidence_hash.clone(),
                    classification: entry.classification.clone(),
                    recomputed_score: Some(score),
                    left_file: entry.left_file.clone(),
                    left_title: entry.left_title.clone(),
                });
            }
            "VALID-BOUNDARY" => {
                if delta_to_delta {
                    verify_boundary_delta_to_delta(
                        root,
                        entry,
                        current_candidate,
                        current_left,
                        current_right,
                    )?;
                    verified.push(VerifiedRemediation {
                        candidate_id: entry.candidate_id.clone(),
                        left_evidence_hash: entry.left_evidence_hash.clone(),
                        right_evidence_hash: entry.right_evidence_hash.clone(),
                        classification: entry.classification.clone(),
                        recomputed_score: None,
                        left_file: entry.left_file.clone(),
                        left_title: entry.left_title.clone(),
                    });
                    continue;
                }
                let Some(left) = current_left else {
                    bail!(
                        "semantic classification is stale (candidate {}): the left endpoint no longer exists after rerun; regenerate the report and reclassify",
                        entry.candidate_id
                    );
                };
                let Some(right) = right else {
                    bail!(
                        "semantic classification is stale (candidate {}): the permanent right endpoint no longer exists after rerun; regenerate the report and reclassify",
                        entry.candidate_id
                    );
                };
                let score = recompute_prior_pair_score(root, index, left, right)?;
                let left_endpoint = semantic_delta_endpoint(left);
                if current_candidate.is_some()
                    || left_endpoint.content_hash == entry.left_content_hash
                    || left_endpoint.evidence_hash == entry.left_evidence_hash
                    || right.content_hash != entry.right_content_hash
                    || right.evidence_hash != entry.right_evidence_hash
                    || score >= E5_ADMISSION_THRESHOLD
                    || !observable_distinction(&left_endpoint, right)
                {
                    let reason = if current_candidate.is_some() {
                        format!(
                            "the pair is still admitted (recomputed score {score:.4} >= {E5_ADMISSION_THRESHOLD:.4}); sharpen a distinct precondition, causal action, or end state so it scores below"
                        )
                    } else if score >= E5_ADMISSION_THRESHOLD {
                        format!(
                            "recomputed score {score:.4} is not strictly below {E5_ADMISSION_THRESHOLD:.4}"
                        )
                    } else if left_endpoint.content_hash == entry.left_content_hash
                        || left_endpoint.evidence_hash == entry.left_evidence_hash
                    {
                        "the active endpoint was not reworded — recorded content/evidence hashes are unchanged".to_string()
                    } else if right.content_hash != entry.right_content_hash
                        || right.evidence_hash != entry.right_evidence_hash
                    {
                        "the permanent endpoint changed after classification — the prior classification is stale".to_string()
                    } else {
                        "the endpoints still read as the same observable outcome — a different driver or phrasing is not a boundary; name a distinct precondition, causal action, or end state".to_string()
                    };
                    bail!(
                        "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary (candidate {}): {reason}",
                        entry.candidate_id
                    );
                }
                verified.push(VerifiedRemediation {
                    candidate_id: entry.candidate_id.clone(),
                    left_evidence_hash: entry.left_evidence_hash.clone(),
                    right_evidence_hash: entry.right_evidence_hash.clone(),
                    classification: entry.classification.clone(),
                    recomputed_score: Some(score),
                    left_file: entry.left_file.clone(),
                    left_title: entry.left_title.clone(),
                });
            }
            _ => {}
        }
    }
    Ok(verified)
}

#[cfg(feature = "spec-retrieval")]
fn recompute_prior_pair_score(
    root: &Path,
    index: &SpecIndex,
    left: &DeltaScenario,
    right: &SemanticEndpoint,
) -> Result<f32> {
    let right_entry = index
        .entries
        .iter()
        .find(|entry| entry.file == right.file && entry.title == right.title)
        .with_context(|| format!("permanent endpoint missing from index: {}", right.title))?;
    let left_vector = embed_texts(
        root,
        std::slice::from_ref(&left.scenario.text),
        EmbeddingRole::Passage,
    )?
    .vectors
    .into_iter()
    .next()
    .flatten()
    .context("retrieval embedder returned no remediation vector")?;
    let raw = cosine_similarity(&left_vector, &right_entry.embedding);
    semantic_score(&semantic_delta_endpoint(left), right, raw)
}

/// Recompute the pair similarity for a delta-to-delta prior entry: neither
/// endpoint lives in the permanent index, so both are embedded fresh from the
/// active delta and scored with the same pair-score policy as admission.
#[cfg(feature = "spec-retrieval")]
fn recompute_delta_pair_score(
    root: &Path,
    left: &DeltaScenario,
    right: &DeltaScenario,
) -> Result<f32> {
    let batch = embed_texts(
        root,
        &[left.scenario.text.clone(), right.scenario.text.clone()],
        EmbeddingRole::Passage,
    )?;
    let left_vector = batch.vectors[0]
        .as_ref()
        .context("retrieval embedder returned no left delta vector")?;
    let right_vector = batch.vectors[1]
        .as_ref()
        .context("retrieval embedder returned no right delta vector")?;
    let raw = cosine_similarity(left_vector, right_vector);
    semantic_score(
        &semantic_delta_endpoint(left),
        &semantic_delta_endpoint(right),
        raw,
    )
}

/// True when the active delta endpoint still carries the recorded content and
/// evidence hashes (i.e. it was not reworded after classification).
#[cfg(feature = "spec-retrieval")]
fn delta_endpoint_unchanged(item: &DeltaScenario, content_hash: &str, evidence_hash: &str) -> bool {
    let endpoint = semantic_delta_endpoint(item);
    endpoint.content_hash == content_hash && endpoint.evidence_hash == evidence_hash
}

/// Verify a classified delta-to-delta `DUPLICATE` prior entry. The redundant
/// endpoint is removed from the active delta (either side may be the removed
/// one, and the retained side must be unchanged), or the pair's recomputed
/// score drops strictly below the similarity boundary so the pair is no longer
/// admitted.
#[cfg(feature = "spec-retrieval")]
fn verify_duplicate_delta_to_delta(
    root: &Path,
    entry: &PriorRemediationEntry,
    current_left: Option<&DeltaScenario>,
    current_right: Option<&DeltaScenario>,
) -> Result<()> {
    match (current_left, current_right) {
        (Some(left), Some(right)) => {
            let score = recompute_delta_pair_score(root, left, right)?;
            if score >= E5_ADMISSION_THRESHOLD {
                bail!(
                    "DUPLICATE remediation requires removing the redundant scenario (candidate {}): the pair is still admitted at recomputed embedding score {score:.4} (>= {E5_ADMISSION_THRESHOLD:.4}) and both endpoints are still present in the active delta",
                    entry.candidate_id
                );
            }
            Ok(())
        }
        (retained, None) if retained.is_some() => {
            let retained = retained.expect("retained delta endpoint");
            if !delta_endpoint_unchanged(
                retained,
                &entry.left_content_hash,
                &entry.left_evidence_hash,
            ) {
                bail!(
                    "DUPLICATE remediation requires removing the redundant scenario (candidate {}): the retained endpoint \"{}\" changed after classification — remove the redundant scenario without altering the retained endpoint",
                    entry.candidate_id,
                    entry.left_title
                );
            }
            Ok(())
        }
        (None, retained) if retained.is_some() => {
            let retained = retained.expect("retained delta endpoint");
            if !delta_endpoint_unchanged(
                retained,
                &entry.right_content_hash,
                &entry.right_evidence_hash,
            ) {
                bail!(
                    "DUPLICATE remediation requires removing the redundant scenario (candidate {}): the retained endpoint \"{}\" changed after classification — remove the redundant scenario without altering the retained endpoint",
                    entry.candidate_id,
                    entry.right_title
                );
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Verify a classified delta-to-delta `VARIANT` prior entry. The surviving
/// endpoint is consolidated into a parameterized `Scenario Outline` with a
/// non-empty `Examples` table (its recorded hashes changed), and the redundant
/// endpoint is removed so the pair is no longer admitted.
#[cfg(feature = "spec-retrieval")]
fn verify_variant_delta_to_delta(
    entry: &PriorRemediationEntry,
    current_candidate: Option<&SemanticCandidate>,
    current_left: Option<&DeltaScenario>,
    current_right: Option<&DeltaScenario>,
) -> Result<()> {
    let left_consolidated = current_left.is_some_and(|left| {
        left.is_outline
            && left.has_examples
            && !delta_endpoint_unchanged(left, &entry.left_content_hash, &entry.left_evidence_hash)
    });
    let right_consolidated = current_right.is_some_and(|right| {
        right.is_outline
            && right.has_examples
            && !delta_endpoint_unchanged(
                right,
                &entry.right_content_hash,
                &entry.right_evidence_hash,
            )
    });
    let survivor_consolidated = matches!(
        (
            left_consolidated,
            right_consolidated,
            current_left.is_some(),
            current_right.is_some()
        ),
        (true, false, true, false) | (false, true, false, true)
    );
    if current_candidate.is_some() || !survivor_consolidated {
        let reason = if current_candidate.is_some() {
            "the pair is still admitted; the redundant endpoint has not been removed and the variant is not consolidated into a single parameterized outline"
                .to_string()
        } else {
            "the surviving endpoint must be a parameterized Scenario Outline with a non-empty Examples table whose recorded content changed, and the redundant endpoint must be removed"
                .to_string()
        };
        bail!(
            "VARIANT remediation requires consolidation (candidate {}): {reason}",
            entry.candidate_id
        );
    }
    Ok(())
}

/// Verify a classified delta-to-delta `VALID-BOUNDARY` prior entry. Both
/// endpoints remain in the active delta; at least one is reworded, the pair's
/// recomputed score is strictly below the similarity boundary, and the two
/// endpoints describe observably distinct outcomes.
#[cfg(feature = "spec-retrieval")]
fn verify_boundary_delta_to_delta(
    root: &Path,
    entry: &PriorRemediationEntry,
    current_candidate: Option<&SemanticCandidate>,
    current_left: Option<&DeltaScenario>,
    current_right: Option<&DeltaScenario>,
) -> Result<()> {
    let Some(left) = current_left else {
        bail!(
            "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary (candidate {}): the left endpoint no longer exists in the active delta; regenerate the report and reclassify",
            entry.candidate_id
        );
    };
    let Some(right) = current_right else {
        bail!(
            "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary (candidate {}): the right endpoint no longer exists in the active delta; regenerate the report and reclassify",
            entry.candidate_id
        );
    };
    let left_endpoint = semantic_delta_endpoint(left);
    let right_endpoint = semantic_delta_endpoint(right);
    let score = recompute_delta_pair_score(root, left, right)?;
    let left_changed =
        !delta_endpoint_unchanged(left, &entry.left_content_hash, &entry.left_evidence_hash);
    let right_changed =
        !delta_endpoint_unchanged(right, &entry.right_content_hash, &entry.right_evidence_hash);
    if current_candidate.is_some()
        || score >= E5_ADMISSION_THRESHOLD
        || !(left_changed || right_changed)
        || !observable_distinction(&left_endpoint, &right_endpoint)
    {
        let reason = if score >= E5_ADMISSION_THRESHOLD {
            format!("recomputed score {score:.4} is not strictly below {E5_ADMISSION_THRESHOLD:.4}")
        } else if !(left_changed || right_changed) {
            "neither endpoint was reworded — recorded content/evidence hashes are unchanged"
                .to_string()
        } else if !observable_distinction(&left_endpoint, &right_endpoint) {
            "the endpoints still read as the same observable outcome — a different driver or phrasing is not a boundary; name a distinct precondition, causal action, or end state".to_string()
        } else {
            "the pair is still admitted; reword a distinct precondition, causal action, or end state so it scores below the boundary".to_string()
        };
        bail!(
            "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary (candidate {}): {reason}",
            entry.candidate_id
        );
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn observable_distinction(left: &SemanticEndpoint, right: &SemanticEndpoint) -> bool {
    left.facts.facts != right.facts.facts || left.facts.assertions != right.facts.assertions
}

#[cfg(feature = "spec-retrieval")]
fn render_semantic_report(
    change_id: &str,
    candidates: &[SemanticCandidate],
    e5_token_caps: &[E5TokenCap],
    remediation_blocked: bool,
    bge_run: Option<&BgeRun>,
    #[cfg(feature = "nli-evidence")] nli_run: Option<&NliRun>,
    pair_cache_accounting: PairCacheAccounting,
) -> String {
    let blocking = candidates
        .iter()
        .filter(|candidate| {
            candidate.bge_admission != Some("BGE_FILTERED") && {
                #[cfg(feature = "nli-evidence")]
                {
                    candidate.nli_admission != Some("NLI_FILTERED")
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    true
                }
            }
        })
        .collect::<Vec<_>>();
    let status = if candidates
        .iter()
        .filter(|candidate| {
            candidate.bge_admission != Some("BGE_FILTERED") && {
                #[cfg(feature = "nli-evidence")]
                {
                    candidate.nli_admission != Some("NLI_FILTERED")
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    true
                }
            }
        })
        .all(|candidate| candidate.classification != "UNCERTAIN" && candidate.rationale.is_some())
        && e5_token_caps.is_empty()
        && !remediation_blocked
    {
        "PASS"
    } else {
        "BLOCKED"
    };
    let report_schema_version = {
        #[cfg(feature = "nli-evidence")]
        {
            nli_run
                .is_some()
                .then_some(SEMANTIC_NLI_REPORT_SCHEMA_VERSION)
        }
        #[cfg(not(feature = "nli-evidence"))]
        {
            None
        }
    }
    .or_else(|| bge_run.map(|_| SEMANTIC_BGE_REPORT_SCHEMA_VERSION))
    .unwrap_or(SEMANTIC_REPORT_SCHEMA_VERSION);
    let mut report = format!(
        "# Semantic Review\nReport schema version: {}\nChange: \"{change_id}\"\nStatus: {status}\nCandidate count: {}\nPair cache: {} reused, {} computed\n",
        report_schema_version,
        blocking.len(),
        pair_cache_accounting.reused,
        pair_cache_accounting.computed,
    );
    if expose_e5_cap_contract(e5_token_caps) {
        report.push_str(&format!(
            "E5 token-cap count: {}\ne5_token_cap_count: {}\ne5_token_cap_candidates: [",
            e5_token_caps.len(),
            e5_token_caps.len(),
        ));
        report.push_str(
            &e5_token_caps
                .iter()
                .map(|cap| serde_json::to_string(cap).expect("serialize semantic E5 token cap"))
                .collect::<Vec<_>>()
                .join(","),
        );
        report.push_str("]\n");
        for (index, cap) in e5_token_caps.iter().enumerate() {
            report.push_str(&format!(
                "\n## E5 Token Cap Candidate {}\n- status: E5_TOKEN_CAP\n- classification: null\n- rationale: null\n- file: \"{}\"\n- title: \"{}\"\n- content_hash: {}\n- token_count: {}\n- limit: {}\n- model: {}\n- tokenizer: {}\n- refactoring_required: {}\n- guidance: \"shorten or split the scenario, then rerun semantic review\"\n",
                index + 1,
                cap.file,
                cap.title,
                cap.content_hash,
                cap.token_count,
                cap.limit,
                cap.model,
                cap.tokenizer,
                cap.refactoring_required,
            ));
        }
        if !e5_token_caps.is_empty() {
            report.push_str(
                "refactoring required: shorten or split the scenario, then rerun semantic review\n",
            );
        }
    }
    if candidates.iter().any(|candidate| {
        candidate.bge_token_cap.is_some() || {
            #[cfg(feature = "nli-evidence")]
            {
                candidate.nli_token_cap.is_some()
            }
            #[cfg(not(feature = "nli-evidence"))]
            {
                false
            }
        }
    }) {
        report.push_str(&format!("refactoring required: {TOKEN_CAP_GUIDANCE}\n"));
    }
    #[cfg(feature = "nli-evidence")]
    if bge_run.is_none()
        && let Some(run) = nli_run
    {
        let recommended = candidates
            .iter()
            .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
            .count();
        let filtered = candidates
            .iter()
            .filter(|candidate| candidate.nli_admission == Some("NLI_FILTERED"))
            .count();
        report.push_str(&format!(
                "NLI status: {}\nNLI reason: {}\nNLI model: {}\nNLI top_k: {}\nNLI max unique pairs: {}\nNLI E5 raw count: {}\nNLI recommended count: {}\nNLI filtered count: {}\nNLI unscored count: {}\nNLI unique pair count: {}\nNLI directional call count: {}\nNLI model load attempts: {}\nNLI runtime ms: {}\n",
                run.status,
                run.reason.map(NliUnavailableReason::as_str).unwrap_or("null"),
                if run.status == "NLI_ENABLED" { NLI_MODEL_ID } else { "null" },
                RERANKER_TOP_K,
                run.max_unique_pairs,
                run.e5_candidates,
                recommended,
                filtered,
                run.unscored,
                run.evidence.len(),
                run.evidence.len() * 2,
                run.model_load_attempts,
                run.runtime_ms.map(|value| value.to_string()).unwrap_or_else(|| "null".to_string()),
            ));
        let mut index = 0;
        for candidate in candidates {
            if candidate.nli_admission == Some("NLI_FILTERED") {
                report.push_str(&render_nli_filtered_candidate(candidate));
                continue;
            }
            index += 1;
            report.push_str(&format!(
                    "\n## Candidate {index}\n- candidate_id: {}\n- classification: {}\n- rationale: {}\n- scope: {}\n- left_scope: {}\n- left_file: \"{}\"\n- left_title: \"{}\"\n- left_delta_operation: {}\n- left_projected_target: {}\n- left_evidence_json: {}\n- left_content_hash: {}\n- left_evidence_hash: {}\n- right_scope: {}\n- right_file: \"{}\"\n- right_title: \"{}\"\n- right_delta_operation: {}\n- right_projected_target: {}\n- right_evidence_json: {}\n- right_content_hash: {}\n- right_evidence_hash: {}\n- embedding_score: {:.4}\n- fact_contradiction: {}\n",
                    candidate.candidate_id,
                    candidate.classification,
                    candidate.rationale.as_deref().map(|value| format!("\"{}\"", value.replace('"', "\\\""))).unwrap_or_else(|| "null".to_string()),
                    candidate.scope,
                    candidate.left.scope,
                    candidate.left.file,
                    candidate.left.title,
                    candidate.left.operation,
                    json_encoded(&candidate.left.projected_target),
                    candidate.left.evidence_json,
                    candidate.left.content_hash,
                    candidate.left.evidence_hash,
                    candidate.right.scope,
                    candidate.right.file,
                    candidate.right.title,
                    candidate.right.operation,
                    json_encoded(&candidate.right.projected_target),
                    candidate.right.evidence_json,
                    candidate.right.content_hash,
                    candidate.right.evidence_hash,
                    candidate.score,
                    candidate.left.facts.facts.contradicts(&candidate.right.facts.facts),
                ));
            append_semantic_bge_token_cap_fields(&mut report, candidate);
            append_nli_candidate_fields(&mut report, candidate);
        }
        return report;
    }
    if let Some(run) = bge_run {
        let recommended = candidates
            .iter()
            .filter(|candidate| candidate.bge_admission == Some("BGE_RECOMMENDED"))
            .count();
        let filtered = candidates
            .iter()
            .filter(|candidate| candidate.bge_admission == Some("BGE_FILTERED"))
            .count();
        report.push_str(&format!(
            "BGE status: {}\nBGE reason: {}\nBGE model: {}\nBGE top_k: {}\nBGE max unique pairs: {}\nBGE E5 raw count: {}\nBGE recommended count: {}\nBGE filtered count: {}\nBGE unscored count: {}\nBGE unique pair count: {}\nBGE directional call count: {}\nBGE model load attempts: {}\nBGE runtime ms: {}\n",
            run.status,
            run.reason.map(BgeUnavailableReason::as_str).unwrap_or("null"),
            if run.status == "BGE_ENABLED" { RERANKER_MODEL_ID } else { "null" },
            RERANKER_TOP_K,
            run.max_unique_pairs,
            run.e5_candidates,
            recommended,
            filtered,
            run.unscored,
            run.evidence.len(),
            run.evidence.len() * 2,
            run.model_load_attempts,
            run.runtime_ms.map(|value| value.to_string()).unwrap_or_else(|| "null".to_string()),
        ));
        #[cfg(feature = "nli-evidence")]
        if let Some(run) = nli_run {
            let recommended = candidates
                .iter()
                .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
                .count();
            let filtered = candidates
                .iter()
                .filter(|candidate| candidate.nli_admission == Some("NLI_FILTERED"))
                .count();
            report.push_str(&format!(
                "NLI status: {}\nNLI reason: {}\nNLI model: {}\nNLI top_k: {}\nNLI max unique pairs: {}\nNLI E5 raw count: {}\nNLI recommended count: {}\nNLI filtered count: {}\nNLI unscored count: {}\nNLI unique pair count: {}\nNLI directional call count: {}\nNLI model load attempts: {}\nNLI runtime ms: {}\n",
                run.status,
                run.reason.map(NliUnavailableReason::as_str).unwrap_or("null"),
                if run.status == "NLI_ENABLED" { NLI_MODEL_ID } else { "null" },
                RERANKER_TOP_K,
                run.max_unique_pairs,
                run.e5_candidates,
                recommended,
                filtered,
                run.unscored,
                run.evidence.len(),
                run.evidence.len() * 2,
                run.model_load_attempts,
                run.runtime_ms.map(|value| value.to_string()).unwrap_or_else(|| "null".to_string()),
            ));
        }
        let mut index = 0;
        for candidate in candidates {
            if candidate.bge_admission == Some("BGE_FILTERED") {
                report.push_str(&format!(
                    "\n## BGE Filtered Candidate\n- candidate_id: {}\n- left_file: \"{}\"\n- left_title: \"{}\"\n- right_file: \"{}\"\n- right_title: \"{}\"\n- embedding_score: {:.4}\n- bge_forward_raw: {}\n- bge_reverse_raw: {}\n- bge_symmetric_raw: {}\n- bge_admission: BGE_FILTERED\n",
                    candidate.candidate_id,
                    candidate.left.file,
                    candidate.left.title,
                    candidate.right.file,
                    candidate.right.title,
                    candidate.score,
                    candidate.bge.as_ref().map(|e| format!("{:.4}", e.forward_raw)).unwrap_or_else(|| "null".to_string()),
                    candidate.bge.as_ref().map(|e| format!("{:.4}", e.reverse_raw)).unwrap_or_else(|| "null".to_string()),
                    candidate.bge.as_ref().map(|e| format!("{:.4}", e.symmetric_raw)).unwrap_or_else(|| "null".to_string()),
                ));
                continue;
            }
            #[cfg(feature = "nli-evidence")]
            if candidate.nli_admission == Some("NLI_FILTERED") {
                report.push_str(&render_nli_filtered_candidate(candidate));
                continue;
            }
            index += 1;
            report.push_str(&format!(
                "\n## Candidate {}\n- candidate_id: {}\n- classification: {}\n- rationale: {}\n- scope: {}\n- left_scope: {}\n- left_file: \"{}\"\n- left_title: \"{}\"\n- left_delta_operation: {}\n- left_projected_target: {}\n- left_evidence_json: {}\n- left_content_hash: {}\n- left_evidence_hash: {}\n- right_scope: {}\n- right_file: \"{}\"\n- right_title: \"{}\"\n- right_delta_operation: {}\n- right_projected_target: {}\n- right_evidence_json: {}\n- right_content_hash: {}\n- right_evidence_hash: {}\n- embedding_score: {:.4}\n- fact_contradiction: {}\n- bge_forward_raw: {}\n- bge_reverse_raw: {}\n- bge_symmetric_raw: {}\n- bge_admission: {}\n",
                index,
                candidate.candidate_id,
                candidate.classification,
                candidate.rationale.as_deref().map(|value| format!("\"{}\"", value.replace('"', "\\\""))).unwrap_or_else(|| "null".to_string()),
                candidate.scope,
                candidate.left.scope,
                candidate.left.file,
                candidate.left.title,
                candidate.left.operation,
                json_encoded(&candidate.left.projected_target),
                candidate.left.evidence_json,
                candidate.left.content_hash,
                candidate.left.evidence_hash,
                candidate.right.scope,
                candidate.right.file,
                candidate.right.title,
                candidate.right.operation,
                json_encoded(&candidate.right.projected_target),
                candidate.right.evidence_json,
                candidate.right.content_hash,
                candidate.right.evidence_hash,
                candidate.score,
                candidate.left.facts.facts.contradicts(&candidate.right.facts.facts),
                candidate.bge.as_ref().map(|e| format!("{:.4}", e.forward_raw)).unwrap_or_else(|| "null".to_string()),
                candidate.bge.as_ref().map(|e| format!("{:.4}", e.reverse_raw)).unwrap_or_else(|| "null".to_string()),
                candidate.bge.as_ref().map(|e| format!("{:.4}", e.symmetric_raw)).unwrap_or_else(|| "null".to_string()),
                candidate.bge_admission.unwrap_or("BGE_UNSCORED"),
            ));
            append_semantic_bge_token_cap_fields(&mut report, candidate);
            #[cfg(feature = "nli-evidence")]
            append_nli_candidate_fields(&mut report, candidate);
        }
        return report;
    }
    for (index, candidate) in candidates.iter().enumerate() {
        report.push_str(&format!(
            "\n## Candidate {}\n- candidate_id: {}\n- classification: {}\n- rationale: {}\n- scope: {}\n- left_scope: {}\n- left_file: \"{}\"\n- left_title: \"{}\"\n- left_delta_operation: {}\n- left_projected_target: {}\n- left_evidence_json: {}\n- left_content_hash: {}\n- left_evidence_hash: {}\n- right_scope: {}\n- right_file: \"{}\"\n- right_title: \"{}\"\n- right_delta_operation: {}\n- right_projected_target: {}\n- right_evidence_json: {}\n- right_content_hash: {}\n- right_evidence_hash: {}\n- embedding_score: {:.4}\n- fact_contradiction: {}\n",
            index + 1,
            candidate.candidate_id,
            candidate.classification,
            candidate
                .rationale
                .as_deref()
                .map(|value| format!("\"{}\"", value.replace('"', "\\\"")))
                .unwrap_or_else(|| "null".to_string()),
            candidate.scope,
            candidate.left.scope,
            candidate.left.file,
            candidate.left.title,
            candidate.left.operation,
            json_encoded(&candidate.left.projected_target),
            candidate.left.evidence_json,
            candidate.left.content_hash,
            candidate.left.evidence_hash,
            candidate.right.scope,
            candidate.right.file,
            candidate.right.title,
            candidate.right.operation,
            json_encoded(&candidate.right.projected_target),
            candidate.right.evidence_json,
            candidate.right.content_hash,
            candidate.right.evidence_hash,
            candidate.score,
            candidate.left.facts.facts.contradicts(&candidate.right.facts.facts)
        ));
    }
    report
}

#[cfg(feature = "nli-evidence")]
fn nli_probability_text(value: Option<f32>) -> String {
    value
        .map(|value| format!("{value:.4}"))
        .unwrap_or_else(|| "null".to_string())
}

#[cfg(feature = "nli-evidence")]
fn append_nli_candidate_fields(report: &mut String, candidate: &SemanticCandidate) {
    let evidence = candidate.nli;
    report.push_str(&format!(
        "- nli_forward_entailment: {}\n- nli_forward_neutral: {}\n- nli_forward_contradiction: {}\n- nli_reverse_entailment: {}\n- nli_reverse_neutral: {}\n- nli_reverse_contradiction: {}\n- nli_symmetric_entailment: {}\n- nli_symmetric_neutral: {}\n- nli_symmetric_contradiction: {}\n- nli_admission: {}\n",
        nli_probability_text(evidence.map(|value| value.forward[0])),
        nli_probability_text(evidence.map(|value| value.forward[1])),
        nli_probability_text(evidence.map(|value| value.forward[2])),
        nli_probability_text(evidence.map(|value| value.reverse[0])),
        nli_probability_text(evidence.map(|value| value.reverse[1])),
        nli_probability_text(evidence.map(|value| value.reverse[2])),
        nli_probability_text(evidence.map(|value| value.symmetric[0])),
        nli_probability_text(evidence.map(|value| value.symmetric[1])),
        nli_probability_text(evidence.map(|value| value.symmetric[2])),
        candidate.nli_admission.unwrap_or("NLI_UNSCORED"),
    ));
    if let Some(budget) = candidate.nli_token_cap {
        report.push_str(&format!(
            "- nli_left_tokens: {}\n- nli_right_tokens: {}\n- nli_pair_tokens: {}\n- nli_limit: {}\n- nli_refactoring_required: true\n",
            budget.left_tokens, budget.right_tokens, budget.pair_tokens, budget.limit
        ));
    }
}

#[cfg(feature = "spec-retrieval")]
fn append_semantic_bge_token_cap_fields(report: &mut String, candidate: &SemanticCandidate) {
    if let Some(budget) = candidate.bge_token_cap {
        report.push_str(&format!(
            "- bge_left_tokens: {}\n- bge_right_tokens: {}\n- bge_pair_tokens: {}\n- bge_limit: {}\n- bge_refactoring_required: true\n",
            budget.left_tokens, budget.right_tokens, budget.pair_tokens, budget.limit
        ));
    }
}

#[cfg(feature = "nli-evidence")]
fn render_nli_filtered_candidate(candidate: &SemanticCandidate) -> String {
    let mut report = format!(
        "\n## NLI Filtered Candidate\n- candidate_id: {}\n- left_file: \"{}\"\n- left_title: \"{}\"\n- right_file: \"{}\"\n- right_title: \"{}\"\n- embedding_score: {:.4}\n",
        candidate.candidate_id,
        candidate.left.file,
        candidate.left.title,
        candidate.right.file,
        candidate.right.title,
        candidate.score,
    );
    append_nli_candidate_fields(&mut report, candidate);
    report
}

#[cfg(feature = "spec-retrieval")]
fn refresh_index(root: &Path, allow_download: bool) -> Result<IndexRefresh> {
    let entries = collect_scenarios(root)?;
    let index_path = spec_index_path(root);
    let previous = read_index(&index_path).unwrap_or_else(|_| SpecIndex {
        version: INDEX_VERSION,
        model: MODEL_ID.to_string(),
        entries: Vec::new(),
        e5_token_caps: Vec::new(),
    });

    let mut result = Vec::with_capacity(entries.len());
    let mut token_caps = Vec::new();
    let mut pending = Vec::new();
    let mut reused = 0usize;

    for scenario in entries {
        if let Some(existing) = reusable_entry(&previous, &scenario) {
            result.push(existing.clone());
            reused += 1;
        } else if let Some(existing) = reusable_e5_token_cap(&previous, &scenario) {
            token_caps.push(existing);
        } else {
            pending.push(scenario);
        }
    }

    let mut embedded = 0usize;
    if !pending.is_empty() {
        notice(format!("embedding {} pending scenarios", pending.len()));
        let texts: Vec<String> = pending.iter().map(|item| item.text.clone()).collect();
        let mut report_progress = |done: usize, total: usize| {
            notice(format!("embedded {done}/{total} scenarios"));
        };
        let batch = embed_texts_with_policy(
            root,
            &texts,
            EmbeddingRole::Passage,
            allow_download,
            Some(&mut report_progress),
        )?;
        for (index, scenario) in pending.into_iter().enumerate() {
            if let Some(embedding) = batch.vectors[index].clone() {
                result.push(IndexEntry {
                    title: scenario.title,
                    file: scenario.file,
                    content_hash: scenario.content_hash,
                    embedding,
                });
                embedded += 1;
            } else {
                let budget = batch.token_caps[index]
                    .expect("an input without an embedding must have an E5 token cap");
                token_caps.push(e5_token_cap(&scenario, budget));
            }
        }
        notice("embedding complete");
    }

    result.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.title.cmp(&right.title))
    });
    token_caps.sort_by(e5_token_cap_order);

    let index = SpecIndex {
        version: INDEX_VERSION,
        model: MODEL_ID.to_string(),
        entries: result,
        e5_token_caps: token_caps,
    };
    write_index(&index_path, &index)?;

    Ok(IndexRefresh {
        index,
        reused,
        embedded,
    })
}

#[cfg(feature = "spec-retrieval")]
fn search_enabled(root: &Path, query: &str, evidence: bool) -> Result<()> {
    report_ignored_release_overrides();
    let index_path = spec_index_path(root);
    let index = match read_index(&index_path) {
        Ok(index) => index,
        Err(error) if !index_path.exists() => {
            return Err(error).with_context(|| {
                format!(
                    "spec index is missing at {}; run `givn spec index` first",
                    index_path.display()
                )
            });
        }
        Err(error) => return Err(error),
    };
    let query_vector = embed_texts(root, &[query.to_string()], EmbeddingRole::Query)?
        .vectors
        .into_iter()
        .next()
        .flatten()
        .context("retrieval embedder returned no query vector")?;

    let ranked = rank_entries(&index.entries, &query_vector);

    if !evidence {
        print_search_results(&ranked);
        return Ok(());
    }

    let query_obj = ExploreQuery {
        title: query.trim().to_string(),
        source: "search",
        file: None,
        content_hash: Sha256::digest(query.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        text: query.to_string(),
    };
    let query_endpoint = explore_query_endpoint(&query_obj);
    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    let permanent_texts: BTreeMap<(String, String), String> = collect_scenarios(root)?
        .into_iter()
        .map(|scenario| ((scenario.file, scenario.title), scenario.text))
        .collect();
    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    let candidates: Vec<(ExploreCandidate, f32)> = ranked
        .iter()
        .take(RERANKER_TOP_K)
        .map(|(entry, score)| {
            (
                ExploreCandidate {
                    title: entry.title.clone(),
                    file: entry.file.clone(),
                    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
                    text: permanent_texts
                        .get(&(entry.file.clone(), entry.title.clone()))
                        .cloned()
                        .unwrap_or_else(|| entry.title.clone()),
                    embedding: entry.embedding.clone(),
                    scope: "permanent",
                    change: None,
                    projected_file: None,
                },
                *score,
            )
        })
        .collect();
    #[cfg(feature = "cross-encoder-reranker")]
    let bge_run = bge_explore_run(root, &query_obj, &candidates)?;
    #[cfg(not(feature = "cross-encoder-reranker"))]
    let bge_run: Option<BgeRun> = None;
    #[cfg(all(feature = "spec-retrieval", feature = "nli-evidence"))]
    let nli_run = nli_explore_run(root, &query_obj, &candidates)?;

    print_search_results_with_evidence(
        &ranked,
        &query_endpoint,
        bge_run.as_ref(),
        #[cfg(feature = "nli-evidence")]
        nli_run.as_ref(),
    );
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn explore_enabled(
    root: &Path,
    file: Option<&Path>,
    stdin: bool,
    change: Option<&str>,
    json: bool,
) -> Result<()> {
    report_ignored_release_overrides();
    let query = read_explore_query(root, file, stdin)?;
    let delta = if let Some(change_id) = change {
        let change_dir = validated_review_change_dir(root, change_id)?;
        anyhow::ensure!(change_dir.is_dir(), "change `{change_id}` was not found");
        let delta_root = delta_specs_path(&change_dir, change_id)?;
        Some((
            change_id.to_string(),
            collect_delta_scenarios(root, &delta_root)?,
        ))
    } else {
        None
    };
    report_missing_index(root);
    let refresh = refresh_index(root, true)?;
    let mut e5_token_caps = refresh.index.e5_token_caps.clone();
    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    let permanent_texts: BTreeMap<(String, String), String> = collect_scenarios(root)?
        .into_iter()
        .map(|scenario| ((scenario.file, scenario.title), scenario.text))
        .collect();
    let query_batch = embed_texts(
        root,
        std::slice::from_ref(&query.text),
        EmbeddingRole::Query,
    )?;
    let query_vector = query_batch.vectors[0].clone();
    if let Some(budget) = query_batch.token_caps[0] {
        e5_token_caps.push(e5_token_cap_for_query(&query, budget));
    }

    let mut candidates: Vec<ExploreCandidate> = if query_vector.is_some() {
        refresh
            .index
            .entries
            .iter()
            .map(|entry| ExploreCandidate {
                title: entry.title.clone(),
                file: entry.file.clone(),
                #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
                text: permanent_texts
                    .get(&(entry.file.clone(), entry.title.clone()))
                    .cloned()
                    .unwrap_or_else(|| entry.title.clone()),
                embedding: entry.embedding.clone(),
                scope: "permanent",
                change: None,
                projected_file: None,
            })
            .collect()
    } else {
        Vec::new()
    };

    if let Some((change_id, delta)) = &delta {
        let active_delta: Vec<&DeltaScenario> = delta
            .iter()
            .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
            .collect();
        let delta_texts: Vec<String> = active_delta
            .iter()
            .map(|item| item.scenario.text.clone())
            .collect();
        let delta_batch = if delta_texts.is_empty() {
            EmbeddingBatch {
                vectors: Vec::new(),
                token_caps: Vec::new(),
            }
        } else {
            embed_texts(root, &delta_texts, EmbeddingRole::Passage)?
        };
        for (index, item) in active_delta.iter().enumerate() {
            if let Some(budget) = delta_batch.token_caps[index] {
                e5_token_caps.push(e5_token_cap(&item.scenario, budget));
                continue;
            }
            if query_vector.is_none() {
                continue;
            }
            let embedding = delta_batch.vectors[index]
                .clone()
                .context("retrieval embedder returned no fitting change vector")?;
            candidates.push(ExploreCandidate {
                title: item.scenario.title.clone(),
                file: item.scenario.file.clone(),
                #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
                text: item.scenario.text.clone(),
                embedding,
                scope: "current-change",
                change: Some(change_id.clone()),
                projected_file: (item.operation == crate::gherkin::delta::DeltaOp::Modified)
                    .then(|| item.target_file.clone()),
            });
        }
    }

    let modified_target_identities = delta
        .as_ref()
        .map(|(_, items)| {
            items
                .iter()
                .filter(|item| {
                    item.operation == crate::gherkin::delta::DeltaOp::Modified
                        && delta_identity_matches_query(item, &query)
                })
                .map(|item| (item.target_file.clone(), item.scenario.title.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut ranked: Vec<(ExploreCandidate, f32)> = if let Some(query_vector) = query_vector.as_ref()
    {
        candidates
            .into_iter()
            .filter(|candidate| {
                !matches_query_identity(candidate, &query, &modified_target_identities)
            })
            .map(|candidate| {
                let score = cosine_similarity(query_vector, &candidate.embedding);
                (candidate, score)
            })
            .collect()
    } else {
        Vec::new()
    };
    ranked.sort_by(|(left, left_score), (right, right_score)| {
        right_score
            .total_cmp(left_score)
            .then_with(|| left.scope.cmp(right.scope))
            .then_with(|| left.file.cmp(&right.file))
            .then_with(|| left.title.cmp(&right.title))
    });
    let ranked: Vec<(ExploreCandidate, f32)> = ranked.into_iter().take(RERANKER_TOP_K).collect();
    #[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
    let bge_run = if query_vector.is_some() {
        bge_explore_run(root, &query, &ranked)?
    } else {
        None
    };
    #[cfg(not(all(feature = "spec-retrieval", feature = "cross-encoder-reranker")))]
    let bge_run: Option<BgeRun> = None;
    #[cfg(feature = "nli-evidence")]
    let nli_run = if query_vector.is_some() {
        nli_explore_run(root, &query, &ranked)?
    } else {
        None
    };

    let mut results = Vec::with_capacity(ranked.len());
    for (index, (candidate, score)) in ranked.into_iter().enumerate() {
        let bge = bge_run.as_ref().and_then(|run| {
            let query_endpoint = explore_query_endpoint(&query);
            let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
            let key = canonical_pair_key(query_endpoint, candidate_endpoint);
            run.evidence.get(&key).map(|evidence| ExploreBgeOutput {
                status: if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
                    "BGE_RECOMMENDED"
                } else {
                    "BGE_FILTERED"
                },
                forward_raw: evidence.forward_raw,
                reverse_raw: evidence.reverse_raw,
                symmetric_raw: evidence.symmetric_raw,
            })
        });
        let bge_token_cap = bge_run.as_ref().and_then(|run| {
            let query_endpoint = explore_query_endpoint(&query);
            let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
            let key = canonical_pair_key(query_endpoint, candidate_endpoint);
            run.token_caps
                .get(&key)
                .copied()
                .map(|budget| ExploreTokenCapOutput {
                    status: "BGE_TOKEN_CAP",
                    left_tokens: budget.left_tokens,
                    right_tokens: budget.right_tokens,
                    pair_tokens: budget.pair_tokens,
                    limit: budget.limit,
                    refactoring_required: true,
                })
        });
        #[cfg(feature = "nli-evidence")]
        let nli = nli_run.as_ref().and_then(|run| {
            let query_endpoint = explore_query_endpoint(&query);
            let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
            let key = canonical_pair_key(query_endpoint, candidate_endpoint);
            run.evidence.get(&key).map(|evidence| ExploreNliOutput {
                status: if evidence.symmetric[0] >= NLI_ADMISSION_THRESHOLD {
                    "NLI_RECOMMENDED"
                } else {
                    "NLI_FILTERED"
                },
                forward: evidence.forward,
                reverse: evidence.reverse,
                symmetric: evidence.symmetric,
            })
        });
        #[cfg(feature = "nli-evidence")]
        let nli_token_cap = nli_run.as_ref().and_then(|run| {
            let query_endpoint = explore_query_endpoint(&query);
            let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
            let key = canonical_pair_key(query_endpoint, candidate_endpoint);
            run.token_caps
                .get(&key)
                .copied()
                .map(|budget| ExploreTokenCapOutput {
                    status: "NLI_TOKEN_CAP",
                    left_tokens: budget.left_tokens,
                    right_tokens: budget.right_tokens,
                    pair_tokens: budget.pair_tokens,
                    limit: budget.limit,
                    refactoring_required: true,
                })
        });
        results.push(ExploreResultOutput {
            rank: index + 1,
            title: candidate.title,
            file: candidate.file,
            score,
            scope: candidate.scope,
            change: candidate.change,
            bge,
            bge_token_cap,
            #[cfg(feature = "nli-evidence")]
            nli,
            #[cfg(feature = "nli-evidence")]
            nli_token_cap,
        });
    }

    let bge_participates = bge_run.is_some();
    #[cfg(feature = "nli-evidence")]
    let nli_participates = nli_run.is_some();
    #[cfg(not(feature = "nli-evidence"))]
    let nli_participates = false;
    let has_second_stage = bge_participates || nli_participates;
    let recommended_results = has_second_stage.then(|| {
        results
            .iter()
            .filter(|result| {
                (!bge_participates || {
                    result
                        .bge
                        .as_ref()
                        .is_some_and(|evidence| evidence.status == "BGE_RECOMMENDED")
                }) && {
                    #[cfg(feature = "nli-evidence")]
                    {
                        !nli_participates || {
                            result
                                .nli
                                .as_ref()
                                .is_some_and(|evidence| evidence.status == "NLI_RECOMMENDED")
                        }
                    }
                    #[cfg(not(feature = "nli-evidence"))]
                    {
                        true
                    }
                }
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    #[cfg(feature = "nli-evidence")]
    let nli_recommended_results = nli_run.as_ref().map(|_| {
        results
            .iter()
            .filter(|result| {
                result
                    .nli
                    .as_ref()
                    .is_some_and(|evidence| evidence.status == "NLI_RECOMMENDED")
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    e5_token_caps.sort_by(e5_token_cap_order);
    let expose_e5_caps = expose_e5_cap_contract(&e5_token_caps);

    if json {
        let response = ExploreResponse {
            schema_version: {
                #[cfg(feature = "nli-evidence")]
                {
                    nli_run.is_some().then_some(3)
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    None
                }
            },
            query: ExploreQueryOutput {
                title: query.title,
                source: query.source,
                file: query.file,
                content_hash: query.content_hash,
            },
            index: ExploreIndexOutput {
                model: refresh.index.model,
                version: refresh.index.version,
                entries: refresh.index.entries.len(),
            },
            results: results.clone(),
            e5_token_cap_count: expose_e5_caps.then_some(e5_token_caps.len()),
            e5_token_cap_candidates: expose_e5_caps.then(|| e5_token_caps.clone()),
            evidence: bge_run.as_ref().map(explore_evidence_output),
            recommended_results,
            #[cfg(feature = "nli-evidence")]
            nli_evidence: nli_run.as_ref().map(explore_nli_evidence_output),
            #[cfg(not(feature = "nli-evidence"))]
            nli_evidence: None,
            #[cfg(feature = "nli-evidence")]
            nli_recommended_results,
            #[cfg(not(feature = "nli-evidence"))]
            nli_recommended_results: None,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&response).context("serialize explore JSON")?
        );
    } else {
        print_explore_text(
            &query,
            &refresh,
            &results,
            &e5_token_caps,
            bge_run.as_ref(),
            #[cfg(feature = "nli-evidence")]
            nli_run.as_ref(),
        );
    }

    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn read_explore_query(root: &Path, file: Option<&Path>, stdin: bool) -> Result<ExploreQuery> {
    let (content, source, identity_file) = if let Some(path) = file {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read explore input file {}", path.display()))?;
        (
            content,
            "file",
            Some(normalize_input_file_identity(root, path)),
        )
    } else if stdin {
        let mut content = String::new();
        std::io::stdin()
            .read_to_string(&mut content)
            .context("failed to read explore input from stdin")?;
        (content, "stdin", None)
    } else {
        bail!("spec explore requires exactly one input: use --file or --stdin");
    };

    let feature = gherkin::Feature::parse(&content, gherkin::GherkinEnv::default())
        .map_err(|error| anyhow::anyhow!("explore input is not valid Gherkin: {error}"))?;
    let mut scenarios = feature
        .scenarios
        .iter()
        .map(|scenario| (scenario, None))
        .collect::<Vec<_>>();
    for rule in &feature.rules {
        scenarios.extend(rule.scenarios.iter().map(|scenario| (scenario, Some(rule))));
    }
    anyhow::ensure!(
        scenarios.len() == 1,
        "explore input must contain exactly one scenario; found {}",
        scenarios.len()
    );

    let (scenario, rule) = scenarios[0];
    let file_identity = identity_file.as_deref().unwrap_or("");
    let text = scenario_text(&feature, rule, scenario, file_identity);
    Ok(ExploreQuery {
        title: text.title,
        source,
        file: identity_file,
        content_hash: text.content_hash,
        text: text.text,
    })
}

#[cfg(feature = "spec-retrieval")]
fn normalize_input_file_identity(root: &Path, path: &Path) -> String {
    normalize_project_file_identity(root, path)
}

#[cfg(feature = "spec-retrieval")]
fn normalize_project_file_identity(root: &Path, path: &Path) -> String {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    absolute
        .strip_prefix(root)
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

#[cfg(feature = "spec-retrieval")]
fn matches_query_identity(
    candidate: &ExploreCandidate,
    query: &ExploreQuery,
    modified_target_identities: &[(String, String)],
) -> bool {
    let Some(query_file) = query.file.as_deref() else {
        return false;
    };
    let source_matches = candidate.file == query_file && candidate.title == query.title;
    let target_matches = candidate
        .projected_file
        .as_deref()
        .is_some_and(|file| file == query_file && candidate.title == query.title);
    let projected_permanent_matches = modified_target_identities
        .iter()
        .any(|(file, title)| candidate.file == *file && candidate.title == *title);
    source_matches || target_matches || projected_permanent_matches
}

#[cfg(feature = "spec-retrieval")]
fn delta_identity_matches_query(item: &DeltaScenario, query: &ExploreQuery) -> bool {
    query.file.as_deref().is_some_and(|file| {
        item.scenario.title == query.title
            && (item.scenario.file == file || item.target_file == file)
    })
}

#[cfg(feature = "spec-retrieval")]
fn explore_query_endpoint(query: &ExploreQuery) -> SimilarityEndpoint {
    endpoint(query.file.as_deref().unwrap_or("__query__"), &query.title)
}

#[cfg(feature = "spec-retrieval")]
fn canonical_pair_key(
    left: SimilarityEndpoint,
    right: SimilarityEndpoint,
) -> (SimilarityEndpoint, SimilarityEndpoint) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

#[cfg(feature = "spec-retrieval")]
fn explore_evidence_output(run: &BgeRun) -> ExploreEvidenceOutput {
    let recommended = run
        .evidence
        .values()
        .filter(|evidence| evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD)
        .count();
    ExploreEvidenceOutput {
        status: run.status,
        reason: run.reason.map(BgeUnavailableReason::as_str),
        candidate_generator: "E5",
        top_k: RERANKER_TOP_K,
        max_unique_pairs: run.max_unique_pairs,
        e5_candidates: run.e5_candidates,
        recommended_candidates: recommended,
        bge_filtered_candidates: run.evidence.len().saturating_sub(recommended),
        bge_unscored_candidates: run.unscored,
        bge_token_cap_candidates: run.token_caps.len(),
        refactoring_required: !run.token_caps.is_empty(),
        bge_pairs: run.evidence.len(),
        bge_directional_calls: run.evidence.len() * 2,
        bge_model: (run.status == "BGE_ENABLED").then_some(RERANKER_MODEL_ID),
        bge_model_load_attempts: run.model_load_attempts,
        bge_runtime_ms: run.runtime_ms,
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_explore_run(
    root: &Path,
    query: &ExploreQuery,
    ranked: &[(ExploreCandidate, f32)],
) -> Result<Option<BgeRun>> {
    let Some(backend) = bge_backend(root)? else {
        return Ok(None);
    };
    let max_unique_pairs = max_rerank_pairs()?;
    let query_endpoint = explore_query_endpoint(query);
    let mut texts = BTreeMap::new();
    texts.insert(
        (query_endpoint.file.clone(), query_endpoint.title.clone()),
        query.text.clone(),
    );
    let selected = ranked.iter().take(max_unique_pairs);
    let mut pairs = Vec::with_capacity(ranked.len().min(max_unique_pairs));
    for (candidate, _) in selected {
        let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
        texts.insert(
            (
                candidate_endpoint.file.clone(),
                candidate_endpoint.title.clone(),
            ),
            candidate.text.clone(),
        );
        pairs.push((query_endpoint.clone(), candidate_endpoint));
    }
    let tokenizer = match backend {
        Ok(BgeBackendKind::Local | BgeBackendKind::Auto)
            if spec_env_var("GIVN_SPEC_BGE_LOCAL_STUB").as_deref() != Ok("1") =>
        {
            let automatic = matches!(backend, Ok(BgeBackendKind::Auto));
            Some(load_unbounded_tokenizer(
                &bge_artifact_directory(root, automatic)?.join("tokenizer.json"),
            )?)
        }
        _ => None,
    };
    let partition =
        preflight_pair_budgets(&pairs, &texts, TokenBudgetModel::Bge, tokenizer.as_ref())?;
    let mut run = match backend {
        Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
            BgeRun::unavailable(
                BgeUnavailableReason::TokenCap,
                ranked.len(),
                max_unique_pairs,
            )
        }
        Ok(kind) => score_bge_pairs(root, &partition.fitting, &texts, kind, max_unique_pairs)?,
        Err(reason) => BgeRun::unavailable(reason, ranked.len(), max_unique_pairs),
    };
    run.e5_candidates = ranked.len();
    run.unscored = ranked.len().saturating_sub(run.evidence.len());
    run.token_caps = partition.capped;
    Ok(Some(run))
}

#[cfg(feature = "nli-evidence")]
fn nli_explore_run(
    root: &Path,
    query: &ExploreQuery,
    ranked: &[(ExploreCandidate, f32)],
) -> Result<Option<NliRun>> {
    let Some(selection) = nli_backend_selection(root)? else {
        return Ok(None);
    };
    let max_unique_pairs = max_rerank_pairs()?;
    let query_endpoint = explore_query_endpoint(query);
    let mut texts = BTreeMap::new();
    texts.insert(
        (query_endpoint.file.clone(), query_endpoint.title.clone()),
        query.text.clone(),
    );
    let selected = ranked.iter().take(max_unique_pairs);
    let mut pairs = Vec::with_capacity(ranked.len().min(max_unique_pairs));
    for (candidate, _) in selected {
        let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
        texts.insert(
            (
                candidate_endpoint.file.clone(),
                candidate_endpoint.title.clone(),
            ),
            candidate.text.clone(),
        );
        pairs.push((query_endpoint.clone(), candidate_endpoint));
    }
    let tokenizer = match selection {
        Ok(NliBackend::Local | NliBackend::Auto) => {
            let artifacts =
                nli_artifacts_for_backend(root, matches!(selection, Ok(NliBackend::Auto)))?;
            Some(load_unbounded_tokenizer(&artifacts.tokenizer_path)?)
        }
        _ => None,
    };
    let partition =
        preflight_pair_budgets(&pairs, &texts, TokenBudgetModel::Nli, tokenizer.as_ref())?;
    let mut run = match selection {
        Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
            NliRun::unavailable(
                NliUnavailableReason::TokenCap,
                ranked.len(),
                max_unique_pairs,
            )
        }
        Ok(backend) => {
            score_nli_pairs(root, &partition.fitting, &texts, backend, max_unique_pairs)?
        }
        Err(reason) => NliRun::unavailable(reason, ranked.len(), max_unique_pairs),
    };
    run.e5_candidates = ranked.len();
    run.unscored = ranked.len().saturating_sub(run.evidence.len());
    run.token_caps = partition.capped;
    Ok(Some(run))
}

#[cfg(feature = "nli-evidence")]
fn explore_nli_evidence_output(run: &NliRun) -> ExploreNliEvidenceOutput {
    let recommended = run
        .evidence
        .values()
        .filter(|evidence| evidence.symmetric[0] >= NLI_ADMISSION_THRESHOLD)
        .count();
    let filtered = run
        .evidence
        .values()
        .filter(|evidence| evidence.symmetric[0] < NLI_ADMISSION_THRESHOLD)
        .count();
    ExploreNliEvidenceOutput {
        status: run.status,
        reason: run.reason.map(NliUnavailableReason::as_str),
        candidate_generator: "E5",
        top_k: RERANKER_TOP_K,
        max_unique_pairs: run.max_unique_pairs,
        e5_candidates: run.e5_candidates,
        recommended_candidates: recommended,
        nli_filtered_candidates: filtered,
        nli_unscored_candidates: run.unscored,
        nli_token_cap_candidates: run.token_caps.len(),
        refactoring_required: !run.token_caps.is_empty(),
        nli_pairs: run.evidence.len(),
        nli_directional_calls: run.evidence.len() * 2,
        nli_model: (run.status == "NLI_ENABLED").then_some(NLI_MODEL_ID),
        nli_model_load_attempts: run.model_load_attempts,
        nli_runtime_ms: run.runtime_ms,
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_backend(
    root: &Path,
) -> Result<Option<std::result::Result<BgeBackendKind, BgeUnavailableReason>>> {
    let selected = spec_env_var("GIVN_SPEC_BGE");
    if spec_env_var("GIVN_SPEC_BGE_APPROVAL").as_deref() == Ok("failed") {
        return Ok(None);
    }
    match selected.as_deref() {
        Ok("disabled") => Ok(None),
        Ok("stub") => Ok(Some(Ok(BgeBackendKind::Stub))),
        Ok("missing") => {
            eprintln!(
                "BGE model is unavailable; model={RERANKER_MODEL_ID}; failed artifact=model.onnx; set GIVN_SPEC_BGE_MODEL_DIR or GIVN_SPEC_BGE_CACHE"
            );
            Ok(Some(Err(BgeUnavailableReason::MissingArtifact)))
        }
        Ok("auto") => Ok(Some(match auto_bge_artifacts(root) {
            Ok(_) => Ok(BgeBackendKind::Auto),
            Err(error) => {
                eprintln!(
                    "BGE model is unavailable; automatic acquisition failed: {error:#}; set GIVN_SPEC_BGE_MODEL_DIR for a validated local artifact set"
                );
                Err(BgeUnavailableReason::MissingArtifact)
            }
        })),
        Ok("local") => Ok(Some(match preflight_bge_artifacts(root) {
            Ok(_) => Ok(BgeBackendKind::Local),
            Err(_) => Err(BgeUnavailableReason::MissingArtifact),
        })),
        Ok(value) => bail!("unknown GIVN_SPEC_BGE value `{value}`"),
        Err(std::env::VarError::NotPresent) => Ok(Some(match auto_bge_artifacts(root) {
            Ok(_) => Ok(BgeBackendKind::Auto),
            Err(error) => {
                eprintln!(
                    "BGE model is unavailable; automatic acquisition failed: {error:#}; set GIVN_SPEC_BGE_MODEL_DIR for a validated local artifact set"
                );
                Err(BgeUnavailableReason::MissingArtifact)
            }
        })),
        Err(std::env::VarError::NotUnicode(_)) => bail!("GIVN_SPEC_BGE is not valid UTF-8"),
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_test_pool() -> Result<Option<(usize, usize)>> {
    if spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(None);
    }
    let Some(value) = spec_env("GIVN_SPEC_BGE_TEST_POOL") else {
        return Ok(None);
    };
    let value = value
        .to_str()
        .context("GIVN_SPEC_BGE_TEST_POOL is not valid UTF-8")?;
    match value {
        "bounded-512" => Ok(Some((512, 512))),
        "overflow-513" => Ok(Some((513, 513))),
        "partial-raw-513" => Ok(Some((512, 513))),
        _ => bail!("unknown GIVN_SPEC_BGE_TEST_POOL value `{value}`"),
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn synthetic_bge_pairs(count: usize) -> (Vec<SimilarityPair>, SimilarityTexts) {
    let mut pairs = Vec::with_capacity(count);
    let mut texts = BTreeMap::new();
    for index in 0..count {
        let left = endpoint(
            "givn/specs/__bge-fixture__/left.feature",
            &format!("left-{index}"),
        );
        let right = endpoint(
            "givn/specs/__bge-fixture__/right.feature",
            &format!("right-{index}"),
        );
        texts.insert((left.file.clone(), left.title.clone()), left.title.clone());
        texts.insert(
            (right.file.clone(), right.title.clone()),
            right.title.clone(),
        );
        pairs.push((left, right));
    }
    (pairs, texts)
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn preflight_bge_artifacts(_root: &Path) -> Result<std::path::PathBuf> {
    // Model-location request: honoured in every build kind. The artifact
    // set is structure-validated; its identity is not verified — the
    // accepted exception documented in arc42 ch. 08 § 8.5a.
    let directory = spec_env("GIVN_SPEC_BGE_MODEL_DIR")
        .map(std::path::PathBuf::from)
        .context(
            "GIVN_SPEC_BGE=local requires GIVN_SPEC_BGE_MODEL_DIR; use GIVN_SPEC_BGE=auto for XDG acquisition",
        )?;
    let model = directory.join("model.onnx");
    let model_bytes =
        fs::read(&model).with_context(|| format!("read BGE model artifact {}", model.display()))?;
    for name in [
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ] {
        let path = directory.join(name);
        let bytes = fs::read(&path)
            .with_context(|| format!("read BGE model artifact {}", path.display()))?;
        if name != "tokenizer.json" {
            serde_json::from_slice::<serde_json::Value>(&bytes)
                .with_context(|| format!("parse BGE model artifact {}", path.display()))?;
        }
    }
    if model_bytes
        .windows(b"model.onnx.data".len())
        .any(|window| window == b"model.onnx.data")
        && !directory.join("model.onnx.data").is_file()
    {
        bail!(
            "read BGE model artifact {}",
            directory.join("model.onnx.data").display()
        );
    }
    Ok(directory)
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn score_bge_pairs(
    root: &Path,
    pairs: &[(SimilarityEndpoint, SimilarityEndpoint)],
    texts: &BTreeMap<(String, String), String>,
    backend: BgeBackendKind,
    max_unique_pairs: usize,
) -> Result<BgeRun> {
    let started = std::time::Instant::now();
    let overrides = bge_directional_overrides()?;
    let mut evidence = BTreeMap::new();
    match backend {
        BgeBackendKind::Stub => {
            for (left, right) in pairs {
                let forward = bge_directional_score(left, right, texts, overrides.as_ref())?;
                let reverse = bge_directional_score(right, left, texts, overrides.as_ref())?;
                evidence.insert(
                    canonical_pair_key(left.clone(), right.clone()),
                    BgePairEvidence {
                        forward_raw: forward,
                        reverse_raw: reverse,
                        symmetric_raw: (forward + reverse) / 2.0,
                    },
                );
            }
        }
        BgeBackendKind::Local | BgeBackendKind::Auto => {
            let directory = bge_artifact_directory(root, matches!(backend, BgeBackendKind::Auto))?;
            if spec_env_var("GIVN_SPEC_BGE_LOCAL_STUB").as_deref() == Ok("1") {
                for (left, right) in pairs {
                    let forward = bge_directional_score(left, right, texts, overrides.as_ref())?;
                    let reverse = bge_directional_score(right, left, texts, overrides.as_ref())?;
                    evidence.insert(
                        canonical_pair_key(left.clone(), right.clone()),
                        BgePairEvidence {
                            forward_raw: forward,
                            reverse_raw: reverse,
                            symmetric_raw: (forward + reverse) / 2.0,
                        },
                    );
                }
            } else {
                let tokenizer_files = fastembed::TokenizerFiles {
                    tokenizer_file: fs::read(directory.join("tokenizer.json"))?,
                    config_file: fs::read(directory.join("config.json"))?,
                    special_tokens_map_file: fs::read(directory.join("special_tokens_map.json"))?,
                    tokenizer_config_file: fs::read(directory.join("tokenizer_config.json"))?,
                };
                let model = fastembed::UserDefinedRerankingModel::new(
                    fastembed::OnnxSource::File(directory.join("model.onnx")),
                    tokenizer_files,
                );
                let options = fastembed::RerankInitOptionsUserDefined::new()
                    .with_max_length(BGE_MAX_LENGTH)
                    .with_intra_threads(resolve_thread_budget()?);
                let mut model = fastembed::TextRerank::try_new_from_user_defined(model, options)
                    .context("BGE model is unavailable; check the local BGE artifact set")?;
                for (left, right) in pairs {
                    let left_text = scenario_text_for(texts, left)?;
                    let right_text = scenario_text_for(texts, right)?;
                    let forward = model
                        .rerank(left_text, [right_text], false, Some(32))?
                        .into_iter()
                        .next()
                        .context("BGE omitted forward pair score")?
                        .score;
                    let reverse = model
                        .rerank(right_text, [left_text], false, Some(32))?
                        .into_iter()
                        .next()
                        .context("BGE omitted reverse pair score")?
                        .score;
                    evidence.insert(
                        canonical_pair_key(left.clone(), right.clone()),
                        BgePairEvidence {
                            forward_raw: forward,
                            reverse_raw: reverse,
                            symmetric_raw: (forward + reverse) / 2.0,
                        },
                    );
                }
            }
        }
    }
    Ok(BgeRun::enabled(
        evidence,
        pairs.len(),
        max_unique_pairs,
        started.elapsed().as_millis(),
    ))
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn stub_reranker_score(left: &str, right: &str) -> f32 {
    let left_tokens = reranker_tokens(left);
    let right_tokens = reranker_tokens(right);
    let shared = left_tokens.intersection(&right_tokens).count() as f32;
    let union = left_tokens.union(&right_tokens).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        let overlap = shared / union;
        if overlap >= 0.85 {
            7.0 * overlap
        } else {
            2.0 * overlap
        }
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_directional_score(
    left: &SimilarityEndpoint,
    right: &SimilarityEndpoint,
    texts: &BTreeMap<(String, String), String>,
    overrides: Option<&BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), f32>>,
) -> Result<f32> {
    if let Some(score) = overrides.and_then(|items| items.get(&(left.clone(), right.clone()))) {
        return Ok(*score);
    }
    Ok(stub_reranker_score(
        scenario_text_for(texts, left)?,
        scenario_text_for(texts, right)?,
    ))
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_directional_overrides()
-> Result<Option<BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), f32>>> {
    let Some(raw) = spec_env("GIVN_SPEC_BGE_DIRECTIONAL_SCORES") else {
        return Ok(None);
    };
    let raw = raw
        .to_str()
        .context("GIVN_SPEC_BGE_DIRECTIONAL_SCORES is not valid UTF-8")?;
    if raw.trim().is_empty() {
        bail!("GIVN_SPEC_BGE_DIRECTIONAL_SCORES must not be empty");
    }
    let mut overrides = BTreeMap::new();
    for entry in raw.split(';') {
        let (pair, score) = entry
            .split_once('=')
            .with_context(|| format!("malformed directional evidence entry `{entry}`"))?;
        let (left, right) = pair
            .split_once("->")
            .with_context(|| format!("malformed directional evidence entry `{entry}`"))?;
        let left = parse_bge_endpoint(left)?;
        let right = parse_bge_endpoint(right)?;
        let score = score
            .parse::<f32>()
            .with_context(|| format!("malformed directional evidence entry `{entry}`"))?;
        if !score.is_finite() || overrides.insert((left, right), score).is_some() {
            bail!("malformed directional evidence entry `{entry}`");
        }
    }
    for (left, right) in overrides.keys() {
        if !overrides.contains_key(&(right.clone(), left.clone())) {
            bail!("malformed directional evidence entry: missing reverse direction");
        }
    }
    Ok(Some(overrides))
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn parse_bge_endpoint(value: &str) -> Result<SimilarityEndpoint> {
    let (file, title) = value
        .trim()
        .split_once('|')
        .with_context(|| format!("malformed directional evidence endpoint `{value}`"))?;
    if file.trim().is_empty()
        || title.trim().is_empty()
        || value.contains('\n')
        || value.contains('\r')
    {
        bail!("malformed directional evidence endpoint `{value}`");
    }
    Ok(endpoint(file.trim(), title.trim()))
}

#[cfg(feature = "spec-retrieval")]
fn print_explore_text(
    query: &ExploreQuery,
    refresh: &IndexRefresh,
    results: &[ExploreResultOutput],
    e5_token_caps: &[E5TokenCap],
    bge_run: Option<&BgeRun>,
    #[cfg(feature = "nli-evidence")] nli_run: Option<&NliRun>,
) {
    println!("query: '{}' (source {})", query.title, query.source);
    println!("model: {}", refresh.index.model);
    println!(
        "index: version {}, entries {}",
        refresh.index.version,
        refresh.index.entries.len()
    );
    println!("Reused {} scenario vectors", refresh.reused);
    println!("Embedded {} new scenario vectors", refresh.embedded);
    println!("E5 token-cap candidates: {}", e5_token_caps.len());
    for cap in e5_token_caps {
        println!(
            "E5_TOKEN_CAP: file={} title={:?} content_hash={} token_count={} limit={} model={} tokenizer={} refactoring_required={} guidance={:?}",
            cap.file,
            cap.title,
            cap.content_hash,
            cap.token_count,
            cap.limit,
            cap.model,
            cap.tokenizer,
            cap.refactoring_required,
            cap.guidance,
        );
    }
    if let Some(run) = bge_run {
        println!(
            "BGE status: {}{}",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default()
        );
        let recommended = results
            .iter()
            .filter(|result| {
                result
                    .bge
                    .as_ref()
                    .is_some_and(|evidence| evidence.status == "BGE_RECOMMENDED")
            })
            .count();
        let filtered = results
            .iter()
            .filter(|result| {
                result
                    .bge
                    .as_ref()
                    .is_some_and(|evidence| evidence.status == "BGE_FILTERED")
            })
            .count();
        println!(
            "BGE candidate generator: E5, top_k {}, max pairs {}, raw {}, recommended {}, BGE filtered {}, unscored {}, bounded pool",
            RERANKER_TOP_K,
            run.max_unique_pairs,
            run.e5_candidates,
            recommended,
            filtered,
            run.unscored,
        );
        println!(
            "BGE model: {}, max pairs {}, unique pairs {}, directional calls {}, model load attempts {}, elapsed runtime ms {}",
            if run.status == "BGE_ENABLED" {
                RERANKER_MODEL_ID
            } else {
                "null"
            },
            run.max_unique_pairs,
            run.evidence.len(),
            run.evidence.len() * 2,
            run.model_load_attempts,
            run.runtime_ms.unwrap_or(0),
        );
        if let Some(evidence) = results.iter().find_map(|result| result.bge.as_ref()) {
            println!(
                "BGE symmetric raw score: {:.4} (forward {:.4}, reverse {:.4})",
                evidence.symmetric_raw, evidence.forward_raw, evidence.reverse_raw
            );
        }
        for ((left, right), budget) in &run.token_caps {
            println!(
                "token-cap: BGE_TOKEN_CAP complete semantic pair '{}' ({}) and '{}' ({}) requires {} tokens (limit {}); left {}, right {}; refactoring required: {}",
                left.title,
                left.file,
                right.title,
                right.file,
                budget.pair_tokens,
                budget.limit,
                budget.left_tokens,
                budget.right_tokens,
                TOKEN_CAP_GUIDANCE,
            );
        }
    }
    #[cfg(feature = "nli-evidence")]
    if let Some(run) = nli_run {
        let recommended = results
            .iter()
            .filter(|result| {
                result
                    .nli
                    .as_ref()
                    .is_some_and(|evidence| evidence.status == "NLI_RECOMMENDED")
            })
            .count();
        let filtered = results
            .iter()
            .filter(|result| {
                result
                    .nli
                    .as_ref()
                    .is_some_and(|evidence| evidence.status == "NLI_FILTERED")
            })
            .count();
        println!(
            "NLI status: {}{}",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default()
        );
        println!(
            "NLI candidate generator: E5, top_k {}, max pairs {}, raw {}, recommended {}, NLI filtered {}, unscored {}, bounded pool",
            RERANKER_TOP_K,
            run.max_unique_pairs,
            run.e5_candidates,
            recommended,
            filtered,
            run.unscored
        );
        println!(
            "NLI model: {}, max pairs {}, unique pairs {}, directional calls {}, model load attempts {}, elapsed runtime ms {}",
            if run.status == "NLI_ENABLED" {
                NLI_MODEL_ID
            } else {
                "null"
            },
            run.max_unique_pairs,
            run.evidence.len(),
            run.evidence.len() * 2,
            run.model_load_attempts,
            run.runtime_ms.unwrap_or(0),
        );
        if let Some(evidence) = results.iter().find_map(|result| result.nli.as_ref()) {
            println!(
                "NLI forward probabilities: entailment {:.4}, neutral {:.4}, contradiction {:.4}",
                evidence.forward[0], evidence.forward[1], evidence.forward[2]
            );
            println!(
                "NLI reverse probabilities: entailment {:.4}, neutral {:.4}, contradiction {:.4}",
                evidence.reverse[0], evidence.reverse[1], evidence.reverse[2]
            );
            println!(
                "NLI symmetric probability summary: entailment {:.4}, neutral {:.4}, contradiction {:.4}",
                evidence.symmetric[0], evidence.symmetric[1], evidence.symmetric[2]
            );
        }
        for ((left, right), budget) in &run.token_caps {
            println!(
                "token-cap: NLI_TOKEN_CAP complete semantic pair '{}' ({}) and '{}' ({}) requires {} tokens (limit {}); left {}, right {}; refactoring required: {}",
                left.title,
                left.file,
                right.title,
                right.file,
                budget.pair_tokens,
                budget.limit,
                budget.left_tokens,
                budget.right_tokens,
                TOKEN_CAP_GUIDANCE,
            );
        }
    }
    for result in results {
        if let Some(change) = &result.change {
            println!(
                "rank {}: '{}' (scope {}, change {}, source {}, embedding score {:.4})",
                result.rank, result.title, result.scope, change, result.file, result.score
            );
        } else {
            println!(
                "rank {}: '{}' (scope {}, source {}, embedding score {:.4})",
                result.rank, result.title, result.scope, result.file, result.score
            );
        }
    }
}

#[cfg(feature = "spec-retrieval")]
fn review_enabled(
    root: &Path,
    change_id: &str,
    allow_download: bool,
    options: ReviewOptions,
) -> Result<()> {
    report_ignored_release_overrides();
    resolve_thread_budget()?;
    let change_dir = validated_review_change_dir(root, change_id)?;
    let delta_root = delta_specs_path(&change_dir, change_id)?;
    let index_summary = index_refresh_summary(root, allow_download, options)?;
    let index = read_index(&spec_index_path(root))?;
    if !options.json && spec_env_var("GIVN_SPEC_RESOLUTION_NOTICES").as_deref() == Ok("repeat") {
        stub_resolution_chatter();
    }

    let delta = collect_delta_scenarios(root, &delta_root)?;
    let second_stage_selected = {
        #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
        {
            let mut selected = false;
            #[cfg(feature = "cross-encoder-reranker")]
            {
                selected |= bge_backend(root)?.is_some();
            }
            #[cfg(feature = "nli-evidence")]
            {
                selected |= nli_backend_selection(root)?.is_some();
            }
            selected
        }
        #[cfg(not(any(feature = "cross-encoder-reranker", feature = "nli-evidence")))]
        {
            false
        }
    };
    let findings_path = change_dir.join("review-findings.json");
    #[cfg(all(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    let corroborated_printing = {
        let both_selected = bge_backend(root)?.is_some() && nli_backend_selection(root)?.is_some();
        both_selected && !options.json
    };
    #[cfg(not(all(feature = "cross-encoder-reranker", feature = "nli-evidence")))]
    let corroborated_printing = false;
    if second_stage_selected {
        #[cfg(feature = "spec-similarity-gate")]
        let gate = run_embedding_review(root, &index, &delta, options, corroborated_printing)?;
        #[cfg(not(feature = "spec-similarity-gate"))]
        let gate = GateReport::default();
        let second_stage_result = run_second_stage_advisory_review(root, &index, &delta, options);
        // Second-stage failures propagate unchanged; E5 evidence never shapes
        // or replaces the verdict.
        let second_stage = second_stage_result?;
        let (flagged, corroborated, union_recommended) =
            review_problem_verdict(Some(&second_stage));
        let gate_message = review_failure_message(flagged, corroborated, &gate);
        let document = render_review_document(
            change_id,
            &index_summary,
            Some(&gate),
            Some(&second_stage),
            flagged,
            corroborated,
            union_recommended,
        )?;
        return finish_review_output(options, &findings_path, document, flagged > 0, gate_message);
    }
    #[cfg(feature = "spec-similarity-gate")]
    {
        // E5-only run: threshold hits are advisory evidence, never blockers.
        let gate = run_embedding_review(root, &index, &delta, options, false)?;
        let document =
            render_review_document(change_id, &index_summary, Some(&gate), None, 0, false, 0)?;
        finish_review_output(options, &findings_path, document, false, None)
    }
    #[cfg(not(feature = "spec-similarity-gate"))]
    advisory_review(root, &index, &delta, &change_dir.join("review.md"))
}

/// Problem verdict: the union of participating stages' recommendations.
/// E5 similarity never blocks — it only shortlists candidates for second-stage
/// scoring. A pair is a problem when BGE recommends it, when NLI recommends
/// it, or both; each stage flags independently of the other's participation.
/// Returns (problem count, corroborated mode, union size).
#[cfg(feature = "spec-retrieval")]
fn review_problem_verdict(second: Option<&SecondStageReport>) -> (usize, bool, usize) {
    #[cfg(not(feature = "spec-similarity-gate"))]
    {
        let _ = second;
        return (0, false, 0);
    }
    #[cfg(feature = "spec-similarity-gate")]
    {
        let Some(second) = second else {
            return (0, false, 0);
        };
        #[cfg(feature = "cross-encoder-reranker")]
        let bge_enabled = second
            .bge_run
            .as_ref()
            .is_some_and(|run| run.status == "BGE_ENABLED");
        #[cfg(not(feature = "cross-encoder-reranker"))]
        let bge_enabled = false;
        #[cfg(feature = "nli-evidence")]
        let nli_enabled = second
            .nli_run
            .as_ref()
            .is_some_and(|run| run.status == "NLI_ENABLED");
        #[cfg(not(feature = "nli-evidence"))]
        let nli_enabled = false;
        if !bge_enabled && !nli_enabled {
            return (0, false, 0);
        }
        let mut union_recommended = 0usize;
        for candidate in &second.candidates {
            #[cfg(feature = "cross-encoder-reranker")]
            let bge_rec = bge_enabled && candidate.bge_admission == Some("BGE_RECOMMENDED");
            #[cfg(not(feature = "cross-encoder-reranker"))]
            let bge_rec = false;
            #[cfg(feature = "nli-evidence")]
            let nli_rec = nli_enabled && candidate.nli_admission == Some("NLI_RECOMMENDED");
            #[cfg(not(feature = "nli-evidence"))]
            let nli_rec = false;
            if bge_rec || nli_rec {
                union_recommended += 1;
            }
        }
        (union_recommended, true, union_recommended)
    }
}

/// Failure message for a flagged advisory review. Only second-stage
/// recommendations produce it — E5 threshold hits are advisory evidence.
#[cfg(feature = "spec-retrieval")]
fn review_failure_message(
    flagged: usize,
    _corroborated: bool,
    _gate: &GateReport,
) -> Option<String> {
    if flagged == 0 {
        return None;
    }
    Some(format!(
        "semantic review flagged {flagged} recommended candidate(s); classify or rewrite them before proceeding"
    ))
}

/// Persist the review findings on a failed gate and print the structured
/// document in JSON mode. Text success runs leave no artifact.
#[cfg(feature = "spec-retrieval")]
fn finish_review_output(
    options: ReviewOptions,
    findings_path: &Path,
    document: String,
    flagged: bool,
    mut gate_message: Option<String>,
) -> Result<()> {
    if flagged && options.findings {
        match std::fs::write(findings_path, &document) {
            Ok(()) => {
                if let Some(message) = gate_message.as_mut() {
                    message.push_str(&format!("; findings: {}", findings_path.display()));
                }
            }
            Err(error) => notice(format!(
                "warning: could not write review findings to {}: {error}",
                findings_path.display()
            )),
        }
    }
    if options.json {
        println!("{document}");
    }
    match gate_message {
        Some(message) => Err(anyhow::anyhow!(message)),
        None => Ok(()),
    }
}

/// Build the schema-version-1 review document. It extends the
/// duplicate-audit JSON contract with review-only sections.
#[cfg(feature = "spec-retrieval")]
#[allow(clippy::too_many_arguments)]
fn render_review_document(
    change_id: &str,
    index_summary: &serde_json::Value,
    gate: Option<&GateReport>,
    second_stage: Option<&SecondStageReport>,
    flagged: usize,
    corroborated: bool,
    union_recommended: usize,
) -> Result<String> {
    let (candidates, considered) = match second_stage {
        Some(second) => (&second.candidates, second.considered),
        None => match gate {
            Some(gate) => (&gate.candidates, gate.considered),
            None => (&Vec::new(), 0),
        },
    };
    #[cfg(feature = "spec-similarity-gate")]
    let threshold = E5_ADMISSION_THRESHOLD;
    #[cfg(not(feature = "spec-similarity-gate"))]
    let threshold = 0.95_f32;
    #[cfg(feature = "cross-encoder-reranker")]
    let bge_run = second_stage.and_then(|second| second.bge_run.as_ref());
    #[cfg(feature = "nli-evidence")]
    let nli_run = second_stage.and_then(|second| second.nli_run.as_ref());

    let candidate_values: Vec<serde_json::Value> =
        candidates.iter().map(review_json_candidate).collect();
    let bge_recommended = candidates
        .iter()
        .filter(|candidate| candidate.bge_admission == Some("BGE_RECOMMENDED"))
        .count();
    #[cfg(feature = "nli-evidence")]
    let nli_recommended = candidates
        .iter()
        .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
        .count();
    #[cfg(not(feature = "nli-evidence"))]
    let nli_recommended = 0;
    #[cfg(feature = "nli-evidence")]
    let combined_recommended: Vec<serde_json::Value> = {
        #[cfg(feature = "cross-encoder-reranker")]
        let bge_participating = bge_run.is_some();
        #[cfg(not(feature = "cross-encoder-reranker"))]
        let bge_participating = false;
        let nli_participating = nli_run.is_some();
        candidates
            .iter()
            .filter(|candidate| {
                (!bge_participating || candidate.bge_admission == Some("BGE_RECOMMENDED"))
                    && (!nli_participating || candidate.nli_admission == Some("NLI_RECOMMENDED"))
            })
            .enumerate()
            .map(|(index, candidate)| {
                let mut value = review_json_candidate(candidate);
                value["rank"] = serde_json::json!(index + 1);
                value
            })
            .collect()
    };
    #[cfg(not(feature = "nli-evidence"))]
    let combined_recommended: Vec<serde_json::Value> = Vec::new();

    #[cfg(feature = "nli-evidence")]
    let mut nli_distribution = [0usize; 3];
    #[cfg(not(feature = "nli-evidence"))]
    let nli_distribution = [0usize; 3];
    #[cfg(feature = "nli-evidence")]
    if let Some(second) = second_stage {
        for candidate in &second.candidates {
            if let Some(evidence) = candidate.nli {
                let (index, _) = evidence
                    .symmetric
                    .iter()
                    .copied()
                    .enumerate()
                    .max_by(|left, right| left.1.total_cmp(&right.1))
                    .unwrap_or((1, 0.0));
                nli_distribution[index] += 1;
            }
        }
    }

    #[cfg(feature = "cross-encoder-reranker")]
    let bge_run_value = bge_run.map(|run| review_json_bge_run(run, bge_recommended));
    #[cfg(not(feature = "cross-encoder-reranker"))]
    let bge_run_value: Option<serde_json::Value> = None;
    #[cfg(feature = "nli-evidence")]
    let nli_run_value =
        nli_run.map(|run| review_json_nli_run(run, nli_recommended, nli_distribution));
    #[cfg(not(feature = "nli-evidence"))]
    let nli_run_value: Option<serde_json::Value> = None;

    let document = serde_json::json!({
        "schema_version": 1,
        "source": "givn spec review",
        "change": change_id,
        "index": index_summary,
        "verdict": {
            "problem_flagged": flagged > 0,
            "flagged_recommended_count": union_recommended,
            "threshold_fallback": !corroborated,
            "e5_candidates": considered,
            "threshold": threshold,
        },
        "bge_run": bge_run_value,
        "nli_run": nli_run_value,
        "candidates": candidate_values,
        "combined_recommended_count": combined_recommended.len(),
        "combined_recommended_candidates": combined_recommended,
    });
    Ok(serde_json::to_string_pretty(&document)?)
}

#[cfg(feature = "spec-retrieval")]
fn review_json_candidate(candidate: &SimilarityCandidate) -> serde_json::Value {
    let bge_evidence = candidate.bge.as_ref().map(|evidence| {
        serde_json::json!({
            "status": candidate.bge_admission.unwrap_or("BGE_RECOMMENDED"),
            "forward_raw": evidence.forward_raw,
            "reverse_raw": evidence.reverse_raw,
            "symmetric_raw": evidence.symmetric_raw,
        })
    });
    #[cfg(feature = "nli-evidence")]
    let nli_admission = candidate.nli_admission;
    #[cfg(not(feature = "nli-evidence"))]
    let nli_admission: Option<&'static str> = None;
    #[cfg(feature = "nli-evidence")]
    let nli_evidence_value = candidate.nli.map(|evidence| {
        serde_json::json!({
            "status": nli_admission.unwrap_or("NLI_RECOMMENDED"),
            "forward": evidence.forward,
            "reverse": evidence.reverse,
            "symmetric": evidence.symmetric,
        })
    });
    #[cfg(not(feature = "nli-evidence"))]
    let nli_evidence_value: Option<serde_json::Value> = None;
    serde_json::json!({
        "scope": candidate.scopes,
        "left": {
            "title": candidate.left.title,
            "file": candidate.left.file,
        },
        "right": {
            "title": candidate.right.title,
            "file": candidate.right.file,
        },
        "embedding_score": format!("{:.4}", candidate.embedding_score),
        "admission": {
            "bge": candidate.bge_admission,
            "nli": nli_admission,
        },
        "evidence": {
            "bge": bge_evidence,
            "nli": nli_evidence_value,
        },
    })
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn review_json_bge_run(run: &BgeRun, recommended: usize) -> serde_json::Value {
    serde_json::json!({
        "status": run.status,
        "reason": run.reason.map(BgeUnavailableReason::as_str),
        "model": (run.status == "BGE_ENABLED").then_some(RERANKER_MODEL_ID),
        "top_k": RERANKER_TOP_K,
        "max_unique_pairs": run.max_unique_pairs,
        "e5_candidates": run.e5_candidates,
        "unique_pairs": run.evidence.len(),
        "directional_calls": run.evidence.len() * 2,
        "recommended_count": recommended,
        "filtered_count": run
            .e5_candidates
            .saturating_sub(recommended)
            .saturating_sub(run.unscored),
        "unscored_count": run.unscored,
        "model_load_attempts": run.model_load_attempts,
        "runtime_ms": run.runtime_ms,
    })
}

#[cfg(all(feature = "spec-retrieval", feature = "nli-evidence"))]
fn review_json_nli_run(
    run: &NliRun,
    recommended: usize,
    distribution: [usize; 3],
) -> serde_json::Value {
    serde_json::json!({
        "status": run.status,
        "reason": run.reason.map(NliUnavailableReason::as_str),
        "model": (run.status == "NLI_ENABLED").then_some(NLI_MODEL_ID),
        "top_k": RERANKER_TOP_K,
        "max_unique_pairs": run.max_unique_pairs,
        "e5_candidates": run.e5_candidates,
        "unique_pairs": run.evidence.len(),
        "directional_calls": run.evidence.len() * 2,
        "recommended_count": recommended,
        "filtered_count": run
            .e5_candidates
            .saturating_sub(recommended)
            .saturating_sub(run.unscored),
        "unscored_count": run.unscored,
        "distribution": {
            "entailment": distribution[0],
            "neutral": distribution[1],
            "contradiction": distribution[2],
        },
        "model_load_attempts": run.model_load_attempts,
        "runtime_ms": run.runtime_ms,
    })
}

#[cfg(all(feature = "spec-retrieval", not(feature = "spec-similarity-gate")))]
fn advisory_review(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    review_path: &Path,
) -> Result<()> {
    let (dispositions, _) = crate::overlap::disposition::parse_review(review_path);
    let mut e5_token_caps = index.e5_token_caps.clone();
    println!("retrieval advisory: E5-only configuration");
    for item in delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
    {
        let query_batch = embed_texts(
            root,
            std::slice::from_ref(&item.scenario.text),
            EmbeddingRole::Query,
        )?;
        if let Some(budget) = query_batch.token_caps[0] {
            e5_token_caps.push(e5_token_cap(&item.scenario, budget));
            continue;
        }
        let query_vector = query_batch.vectors[0]
            .clone()
            .context("retrieval embedder returned no review vector")?;
        let ranked = rank_entries(&index.entries, &query_vector);
        for (rank, (entry, score)) in ranked.into_iter().take(5).enumerate() {
            if crate::overlap::disposition::disposed(
                &dispositions,
                &item.scenario.title,
                &entry.title,
            ) {
                continue;
            }
            println!(
                "retrieval advisory: '{}' resembles '{}' (rank {}, score {:.4})",
                item.scenario.title,
                entry.title,
                rank + 1,
                score
            );
        }
    }
    e5_token_caps.sort_by(e5_token_cap_order);
    if !e5_token_caps.is_empty() {
        println!("e5_token_cap_count: {}", e5_token_caps.len());
        println!(
            "e5_token_cap_candidates: {}",
            serde_json::to_string(&e5_token_caps).expect("serialize advisory E5 token caps")
        );
        for cap in &e5_token_caps {
            println!(
                "E5_TOKEN_CAP: '{}' ({}) requires {} tokens (limit {}); model {}, tokenizer {}; refactoring_required: true; guidance: shorten or split the scenario and rerun semantic review",
                cap.title, cap.file, cap.token_count, cap.limit, cap.model, cap.tokenizer,
            );
        }
    }
    Ok(())
}

/// E5 similarity-gate evidence collected by the advisory review.
#[cfg(feature = "spec-retrieval")]
#[derive(Default)]
struct GateReport {
    candidates: Vec<SimilarityCandidate>,
    considered: usize,
}

/// Refresh (or read) the permanent index and report its summary. Text mode
/// prints the established banner lines; JSON mode captures them into the
/// returned document section instead.
#[cfg(feature = "spec-retrieval")]
fn index_refresh_summary(
    root: &Path,
    allow_download: bool,
    options: ReviewOptions,
) -> Result<serde_json::Value> {
    let refresh = refresh_index(root, allow_download)?;
    if !options.json {
        println!(
            "Indexed {} permanent scenarios",
            refresh.index.entries.len()
        );
        println!("Reused {} scenario vectors", refresh.reused);
        println!(
            "E5 token-cap candidates: {}",
            refresh.index.e5_token_caps.len()
        );
        for cap in &refresh.index.e5_token_caps {
            println!(
                "E5_TOKEN_CAP: '{}' ({}) requires {} tokens (limit {}); model {}, tokenizer {}; refactoring_required: true; guidance: shorten or split the scenario and rerun semantic review",
                cap.title, cap.file, cap.token_count, cap.limit, cap.model, cap.tokenizer,
            );
        }
    }
    Ok(serde_json::json!({
        "indexed": refresh.index.entries.len(),
        "reused": refresh.reused,
        "e5_token_caps": refresh.index.e5_token_caps.len(),
        "caps": refresh.index.e5_token_caps,
    }))
}

#[cfg(all(feature = "spec-retrieval", feature = "spec-similarity-gate"))]
fn run_embedding_review(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    options: ReviewOptions,
    corroborated_printing: bool,
) -> Result<GateReport> {
    let (candidates, considered, e5_token_caps) =
        embedding_similarity_candidates(root, index, delta)?;
    if !options.json {
        print_advisory_e5_token_caps(&e5_token_caps);
        let show_raw_list = !corroborated_printing || options.verbose;
        if show_raw_list {
            for candidate in &candidates {
                println!(
                    "similarity candidate [{}]: '{}' ({}) resembles '{}' ({}) score {:.4}, threshold {:.4}; remediation: MERGE (delete the true duplicate) or KEEP-AS-BOUNDARY (rewrite the retained scenario below the threshold)",
                    candidate.scopes.join(","),
                    candidate.left.title,
                    candidate.left.file,
                    candidate.right.title,
                    candidate.right.file,
                    candidate.embedding_score,
                    E5_ADMISSION_THRESHOLD,
                );
            }
        }
        if !corroborated_printing {
            println!(
                "retrieval advisory: embedding similarity evaluated {} candidate pairs",
                considered
            );
            if candidates.is_empty() {
                println!(
                    "similarity gate: no scenario pair at or above fixed threshold {:.4}",
                    E5_ADMISSION_THRESHOLD
                );
            } else {
                println!(
                    "similarity gate: {} candidate(s) at or above fixed threshold {:.4} shortlisted for second-stage evidence",
                    candidates.len(),
                    E5_ADMISSION_THRESHOLD
                );
            }
        } else {
            println!(
                "retrieval advisory: embedding similarity evaluated {} candidate pairs",
                considered
            );
        }
    }
    Ok(GateReport {
        candidates,
        considered,
    })
}

#[cfg(feature = "spec-retrieval")]
fn print_advisory_e5_token_caps(caps: &[E5TokenCap]) {
    if caps.is_empty() {
        return;
    }
    println!("e5_token_cap_count: {}", caps.len());
    println!(
        "e5_token_cap_candidates: {}",
        serde_json::to_string(caps).expect("serialize advisory E5 token caps")
    );
    for cap in caps {
        println!(
            "E5_TOKEN_CAP: '{}' ({}) requires {} tokens (limit {}); model {}, tokenizer {}; refactoring_required: true; guidance: shorten or split the scenario and rerun semantic review",
            cap.title, cap.file, cap.token_count, cap.limit, cap.model, cap.tokenizer,
        );
    }
}

/// Second-stage advisory review evidence and runs.
#[cfg(feature = "spec-retrieval")]
struct SecondStageReport {
    candidates: Vec<SimilarityCandidate>,
    #[allow(dead_code)]
    considered: usize,
    #[cfg(feature = "cross-encoder-reranker")]
    bge_run: Option<BgeRun>,
    #[cfg(not(feature = "cross-encoder-reranker"))]
    bge_run: Option<()>,
    #[cfg(feature = "nli-evidence")]
    nli_run: Option<NliRun>,
    #[cfg(not(feature = "nli-evidence"))]
    nli_run: Option<()>,
}

#[cfg(feature = "spec-retrieval")]
fn run_second_stage_advisory_review(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    options: ReviewOptions,
) -> Result<SecondStageReport> {
    #[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
    let (mut candidates, considered, texts, e5_token_caps) =
        similarity_candidate_pool(root, index, delta)?;
    #[cfg(not(any(feature = "cross-encoder-reranker", feature = "nli-evidence")))]
    let (candidates, considered, _texts, e5_token_caps) =
        similarity_candidate_pool(root, index, delta)?;
    if !options.json {
        print_advisory_e5_token_caps(&e5_token_caps);
    }
    #[cfg(feature = "cross-encoder-reranker")]
    let mut bge_enabled = false;
    #[cfg(not(feature = "cross-encoder-reranker"))]
    let bge_enabled = false;
    #[cfg(feature = "cross-encoder-reranker")]
    let mut bge_run: Option<BgeRun> = None;
    #[cfg(not(feature = "cross-encoder-reranker"))]
    let bge_run: Option<BgeRun> = None;

    #[cfg(feature = "cross-encoder-reranker")]
    if let Some(backend) = bge_backend(root)? {
        bge_enabled = true;
        let max_unique_pairs = max_rerank_pairs()?;
        candidates.sort_by(similarity_candidate_order);
        let selected: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = candidates
            .iter()
            .take(max_unique_pairs)
            .map(|candidate| (candidate.left.clone(), candidate.right.clone()))
            .collect();
        let tokenizer = match backend {
            Ok(BgeBackendKind::Local | BgeBackendKind::Auto)
                if spec_env_var("GIVN_SPEC_BGE_LOCAL_STUB").as_deref() != Ok("1") =>
            {
                let automatic = matches!(backend, Ok(BgeBackendKind::Auto));
                Some(load_unbounded_tokenizer(
                    &bge_artifact_directory(root, automatic)?.join("tokenizer.json"),
                )?)
            }
            _ => None,
        };
        let partition =
            preflight_pair_budgets(&selected, &texts, TokenBudgetModel::Bge, tokenizer.as_ref())?;
        let mut run = match backend {
            Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
                BgeRun::unavailable(
                    BgeUnavailableReason::TokenCap,
                    candidates.len(),
                    max_unique_pairs,
                )
            }
            Ok(kind) => score_bge_pairs(root, &partition.fitting, &texts, kind, max_unique_pairs)
                .map_err(|error| anyhow::anyhow!("BGE model load attempts: 0; {error}"))?,
            Err(reason) => BgeRun::unavailable(reason, candidates.len(), max_unique_pairs),
        };
        run.e5_candidates = considered;
        run.unscored = considered.saturating_sub(run.evidence.len());
        run.token_caps = partition.capped;
        for candidate in &mut candidates {
            let key = canonical_pair_key(candidate.left.clone(), candidate.right.clone());
            if run.token_caps.contains_key(&key) {
                candidate.bge = None;
                candidate.bge_admission = Some("BGE_TOKEN_CAP");
            } else if let Some(evidence) = run.evidence.get(&key).cloned() {
                candidate.bge_admission =
                    Some(if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
                        "BGE_RECOMMENDED"
                    } else {
                        "BGE_FILTERED"
                    });
                candidate.bge = Some(evidence);
            } else {
                candidate.bge_admission = Some("BGE_UNSCORED");
            }
        }
        bge_run = Some(run);
    }

    #[cfg(feature = "nli-evidence")]
    let mut nli_enabled = false;
    #[cfg(feature = "nli-evidence")]
    let mut nli_run: Option<NliRun> = None;
    #[cfg(feature = "nli-evidence")]
    if let Some(selection) = nli_backend_selection(root)? {
        nli_enabled = true;
        let max_unique_pairs = max_rerank_pairs()?;
        candidates.sort_by(similarity_candidate_order);
        let selected: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = candidates
            .iter()
            .take(max_unique_pairs)
            .map(|candidate| (candidate.left.clone(), candidate.right.clone()))
            .collect();
        let tokenizer = match selection {
            Ok(NliBackend::Local | NliBackend::Auto) => {
                let artifacts =
                    nli_artifacts_for_backend(root, matches!(selection, Ok(NliBackend::Auto)))?;
                Some(load_unbounded_tokenizer(&artifacts.tokenizer_path)?)
            }
            _ => None,
        };
        let partition =
            preflight_pair_budgets(&selected, &texts, TokenBudgetModel::Nli, tokenizer.as_ref())?;
        let mut run = match selection {
            Ok(_) if partition.fitting.is_empty() && !partition.capped.is_empty() => {
                NliRun::unavailable(
                    NliUnavailableReason::TokenCap,
                    candidates.len(),
                    max_unique_pairs,
                )
            }
            Ok(backend) => {
                score_nli_pairs(root, &partition.fitting, &texts, backend, max_unique_pairs)?
            }
            Err(reason) => NliRun::unavailable(reason, candidates.len(), max_unique_pairs),
        };
        run.e5_candidates = considered;
        run.unscored = considered.saturating_sub(run.evidence.len());
        run.token_caps = partition.capped;
        for candidate in &mut candidates {
            let key = canonical_pair_key(candidate.left.clone(), candidate.right.clone());
            if let Some(budget) = run.token_caps.get(&key).copied() {
                candidate.nli = None;
                candidate.nli_token_cap = Some(budget);
                candidate.nli_admission = Some("NLI_TOKEN_CAP");
            } else if let Some(evidence) = run.evidence.get(&key).copied() {
                candidate.nli_token_cap = None;
                candidate.nli = Some(evidence);
                candidate.nli_admission =
                    Some(if evidence.symmetric[0] >= NLI_ADMISSION_THRESHOLD {
                        "NLI_RECOMMENDED"
                    } else {
                        "NLI_FILTERED"
                    });
            } else {
                candidate.nli_token_cap = None;
                candidate.nli_admission = Some("NLI_UNSCORED");
            }
        }
        if !options.json {
            print_nli_results(&candidates, considered, max_unique_pairs, options.verbose);
        }
        nli_run = Some(run);
    }
    #[cfg(feature = "nli-evidence")]
    if !options.json
        && let Some(run) = nli_run.as_ref()
    {
        println!(
            "NLI status: {}{}, model {}, max pairs {}, unique pairs {}, directional calls {}, recommended {}, filtered {}, unscored {}, model load attempts {}, runtime ms {}",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default(),
            if run.status == "NLI_ENABLED" {
                NLI_MODEL_ID
            } else {
                "null"
            },
            run.max_unique_pairs,
            run.evidence.len(),
            run.evidence.len() * 2,
            candidates
                .iter()
                .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
                .count(),
            candidates
                .iter()
                .filter(|candidate| candidate.nli_admission == Some("NLI_FILTERED"))
                .count(),
            run.unscored,
            run.model_load_attempts,
            run.runtime_ms
                .map(|value| value.to_string())
                .unwrap_or_else(|| "null".to_string()),
        );
        for ((left, right), budget) in &run.token_caps {
            println!(
                "token-cap: NLI_TOKEN_CAP complete semantic pair '{}' ({}) and '{}' ({}) requires {} tokens (limit {}); left {}, right {}; refactoring required: {}",
                left.title,
                left.file,
                right.title,
                right.file,
                budget.pair_tokens,
                budget.limit,
                budget.left_tokens,
                budget.right_tokens,
                TOKEN_CAP_GUIDANCE,
            );
        }
    }

    if !options.json
        && let Some(run) = bge_run.as_ref()
    {
        let recommended = candidates
            .iter()
            .filter(|candidate| candidate.bge_admission == Some("BGE_RECOMMENDED"))
            .count();
        let filtered = candidates
            .iter()
            .filter(|candidate| candidate.bge_admission == Some("BGE_FILTERED"))
            .count();
        println!(
            "BGE status: {}{}",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default()
        );
        println!(
            "BGE: model {}, top_k {}, max pairs {}, E5 raw {}, unique pairs {}, directional calls {}, runtime ms {}, evidence is advisory",
            if run.status == "BGE_ENABLED" {
                RERANKER_MODEL_ID
            } else {
                "null"
            },
            RERANKER_TOP_K,
            run.max_unique_pairs,
            considered,
            run.evidence.len(),
            run.evidence.len() * 2,
            run.runtime_ms.unwrap_or(0),
        );
        println!(
            "BGE candidate accounting: recommended {}, BGE filtered {}, unscored {}, budget <= {}",
            recommended, filtered, run.unscored, run.max_unique_pairs
        );
        let raw_score = |candidate: &SimilarityCandidate| {
            candidate
                .bge
                .as_ref()
                .map(|evidence| evidence.symmetric_raw)
                .unwrap_or(f32::NEG_INFINITY)
        };
        let mut bge_recommended_rows: Vec<&SimilarityCandidate> = candidates
            .iter()
            .filter(|candidate| {
                candidate.bge.is_some() && candidate.bge_admission == Some("BGE_RECOMMENDED")
            })
            .collect();
        bge_recommended_rows.sort_by(|left, right| raw_score(right).total_cmp(&raw_score(left)));
        let mut bge_rows: Vec<&SimilarityCandidate> = bge_recommended_rows;
        if options.verbose {
            bge_rows.extend(candidates.iter().filter(|candidate| {
                candidate.bge.is_some() && candidate.bge_admission != Some("BGE_RECOMMENDED")
            }));
        }
        for candidate in bge_rows {
            let Some(evidence) = candidate.bge.as_ref() else {
                continue;
            };
            println!(
                "{}: '{}' ({}) resembles '{}' ({}) E5 {:.4}, forward {:.4}, reverse {:.4}, symmetric {:.4}; evidence is advisory and no Givn classification is assigned",
                candidate.bge_admission.unwrap_or("BGE_UNSCORED"),
                candidate.left.title,
                candidate.left.file,
                candidate.right.title,
                candidate.right.file,
                candidate.embedding_score,
                evidence.forward_raw,
                evidence.reverse_raw,
                evidence.symmetric_raw,
            );
        }
        for ((left, right), budget) in &run.token_caps {
            println!(
                "token-cap: BGE_TOKEN_CAP complete semantic pair '{}' ({}) and '{}' ({}) requires {} tokens (limit {}); left {}, right {}; refactoring required: {}",
                left.title,
                left.file,
                right.title,
                right.file,
                budget.pair_tokens,
                budget.limit,
                budget.left_tokens,
                budget.right_tokens,
                TOKEN_CAP_GUIDANCE,
            );
        }
    }

    let recommended = candidates
        .iter()
        .filter(|candidate| {
            (!bge_enabled || candidate.bge_admission == Some("BGE_RECOMMENDED")) && {
                #[cfg(feature = "nli-evidence")]
                {
                    !nli_enabled || candidate.nli_admission == Some("NLI_RECOMMENDED")
                }
                #[cfg(not(feature = "nli-evidence"))]
                {
                    true
                }
            }
        })
        .count();
    if !options.json {
        println!(
            "combined recommended {} of {} E5 candidates; evidence is advisory and no Givn classification is assigned",
            recommended, considered
        );
    }
    Ok(SecondStageReport {
        candidates,
        considered,
        bge_run: {
            #[cfg(feature = "cross-encoder-reranker")]
            {
                bge_run
            }
            #[cfg(not(feature = "cross-encoder-reranker"))]
            {
                None
            }
        },
        nli_run: {
            #[cfg(feature = "nli-evidence")]
            {
                nli_run
            }
            #[cfg(not(feature = "nli-evidence"))]
            {
                None
            }
        },
    })
}

#[cfg(feature = "spec-retrieval")]
fn collect_scenarios(root: &Path) -> Result<Vec<ScenarioText>> {
    collect_scenarios_from(root, &permanent_specs_path(root))
}

#[cfg(feature = "spec-retrieval")]
fn spec_index_path(root: &Path) -> std::path::PathBuf {
    root.join("givn/.spec-index.json")
}

#[cfg(feature = "spec-retrieval")]
pub fn report_missing_index(root: &Path) {
    let path = spec_index_path(root);
    if !path.exists() {
        eprintln!(
            "spec index is missing at {}; run `givn spec index` first",
            path.display()
        );
    }
}

#[cfg(feature = "spec-retrieval")]
fn permanent_specs_path(root: &Path) -> std::path::PathBuf {
    root.join("givn/specs")
}

#[cfg(feature = "spec-retrieval")]
fn parse_permanent_feature(root: &Path, path: &Path) -> Result<gherkin::Feature> {
    let file = normalize_project_file_identity(root, path);
    gherkin::Feature::parse_path(path, gherkin::GherkinEnv::default())
        .with_context(|| format!("failed to parse permanent spec {file}"))
}

#[cfg(feature = "spec-retrieval")]
fn collect_duplicate_scenarios(root: &Path) -> Result<Vec<DuplicateScenario>> {
    let mut scenarios = Vec::new();
    for path in collect_duplicate_feature_files(root)? {
        let feature = parse_permanent_feature(root, &path)?;
        let file = normalize_project_file_identity(root, &path);
        for scenario in &feature.scenarios {
            collect_duplicate_scenario(&feature, None, scenario, &file, &mut scenarios);
        }
        for rule in &feature.rules {
            for scenario in &rule.scenarios {
                collect_duplicate_scenario(&feature, Some(rule), scenario, &file, &mut scenarios);
            }
        }
    }
    scenarios.sort_by(|left, right| {
        left.endpoint
            .file
            .cmp(&right.endpoint.file)
            .then_with(|| left.endpoint.title.cmp(&right.endpoint.title))
    });
    Ok(scenarios)
}

#[cfg(feature = "spec-retrieval")]
fn collect_duplicate_feature_files(root: &Path) -> Result<Vec<std::path::PathBuf>> {
    let specs_root = permanent_specs_path(root);
    match fs::metadata(&specs_root) {
        Ok(_) => Ok(crate::gherkin::parse::collect_feature_files(&specs_root)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error).with_context(|| {
            format!(
                "failed to inspect permanent specs directory {}",
                normalize_project_file_identity(root, &specs_root)
            )
        }),
    }
}

#[cfg(feature = "spec-retrieval")]
fn collect_duplicate_scenario(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    scenario: &gherkin::Scenario,
    file: &str,
    output: &mut Vec<DuplicateScenario>,
) {
    if !is_queryable_scenario(scenario) {
        return;
    }
    output.push(DuplicateScenario {
        endpoint: DuplicateEndpoint {
            file: file.to_string(),
            title: duplicate_endpoint_title(&scenario.name),
        },
        behavior: duplicate_behavior_body(feature, rule, scenario),
        fingerprint: crate::overlap::fingerprint::Fingerprint::of_scenario(
            scenario,
            rule.and_then(|rule| rule.background.as_ref())
                .or(feature.background.as_ref()),
        ),
        text: scenario_text(feature, rule, scenario, file).text,
    });
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_endpoint_title(title: &str) -> String {
    title
        .strip_prefix("[DONE]")
        .map(str::trim_start)
        .unwrap_or(title)
        .to_string()
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_behavior_body(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    scenario: &gherkin::Scenario,
) -> String {
    format!(
        "{{\"feature_background\":{},\"rule_background\":{},\"steps\":{},\"examples\":[{}]}}",
        canonical_background(feature.background.as_ref()),
        canonical_background(rule.and_then(|rule| rule.background.as_ref())),
        canonical_steps(&scenario.steps),
        scenario
            .examples
            .iter()
            .map(canonical_examples)
            .collect::<Vec<_>>()
            .join(","),
    )
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_candidates(
    root: &Path,
    scenarios: &[DuplicateScenario],
) -> Result<DuplicateCandidateSet> {
    report_missing_index(root);
    let mut candidates = Vec::new();
    let mut classified = BTreeSet::new();
    for (left_index, left) in scenarios.iter().enumerate() {
        for (right_index, right) in scenarios.iter().enumerate().skip(left_index + 1) {
            if left.behavior == right.behavior {
                candidates.push(DuplicateCandidate {
                    kind: "exact_duplicate",
                    left: left.endpoint.clone(),
                    right: right.endpoint.clone(),
                    relation: "identical_behavior",
                    assertion_shape: None,
                    facts: None,
                    distinguishing_facts: Vec::new(),
                    fact_contradiction: false,
                    score: None,
                    bge: None,
                    bge_admission: None,
                    bge_token_cap: None,
                    #[cfg(feature = "nli-evidence")]
                    nli: None,
                    #[cfg(feature = "nli-evidence")]
                    nli_admission: None,
                    #[cfg(feature = "nli-evidence")]
                    nli_token_cap: None,
                });
                classified.insert((left_index, right_index));
            } else if !left.fingerprint.assertions.is_empty()
                && left.fingerprint.assertions == right.fingerprint.assertions
            {
                candidates.push(structural_candidate(left, right, "same_assertion_shape"));
                classified.insert((left_index, right_index));
            } else if let Some(relation) = assertion_subset_relation(
                &left.fingerprint.assertions,
                &right.fingerprint.assertions,
            ) {
                candidates.push(structural_candidate(left, right, relation));
                classified.insert((left_index, right_index));
            }
        }
    }

    let score_overrides = duplicate_score_overrides()?;
    let (vectors, e5_token_caps) = match read_index(&spec_index_path(root)) {
        Ok(index)
            if scenarios.iter().all(|scenario| {
                index.entries.iter().any(|entry| {
                    entry.file == scenario.endpoint.file
                        && entry.title == scenario.endpoint.title
                        && entry.content_hash == content_hash(&scenario.text)
                }) || index.e5_token_caps.iter().any(|cap| {
                    cap.file == scenario.endpoint.file
                        && cap.title == scenario.endpoint.title
                        && cap.content_hash == content_hash(&scenario.text)
                })
            }) =>
        {
            (
                index
                    .entries
                    .into_iter()
                    .map(|entry| ((entry.file, entry.title), entry.embedding))
                    .collect(),
                index.e5_token_caps,
            )
        }
        _ => {
            let batch = if scenarios.is_empty() {
                EmbeddingBatch {
                    vectors: Vec::new(),
                    token_caps: Vec::new(),
                }
            } else {
                embed_texts(
                    root,
                    &scenarios
                        .iter()
                        .map(|scenario| scenario.text.clone())
                        .collect::<Vec<_>>(),
                    EmbeddingRole::Passage,
                )?
            };
            let vectors: BTreeMap<(String, String), Vec<f32>> = scenarios
                .iter()
                .zip(batch.vectors)
                .filter_map(|(scenario, vector)| {
                    vector.map(|vector| {
                        (
                            (
                                scenario.endpoint.file.clone(),
                                scenario.endpoint.title.clone(),
                            ),
                            vector,
                        )
                    })
                })
                .collect();
            let e5_token_caps = scenarios
                .iter()
                .zip(batch.token_caps)
                .filter_map(|(scenario, budget)| {
                    budget.map(|budget| {
                        e5_token_cap(
                            &ScenarioText {
                                title: scenario.endpoint.title.clone(),
                                file: scenario.endpoint.file.clone(),
                                content_hash: content_hash(&scenario.text),
                                text: scenario.text.clone(),
                            },
                            budget,
                        )
                    })
                })
                .collect();
            (vectors, e5_token_caps)
        }
    };
    for (left_index, left) in scenarios.iter().enumerate() {
        for (right_index, right) in scenarios.iter().enumerate().skip(left_index + 1) {
            if classified.contains(&(left_index, right_index)) {
                continue;
            }
            let (Some(left_embedding), Some(right_embedding)) = (
                vectors.get(&(left.endpoint.file.clone(), left.endpoint.title.clone())),
                vectors.get(&(right.endpoint.file.clone(), right.endpoint.title.clone())),
            ) else {
                continue;
            };
            let raw_score = cosine_similarity(left_embedding, right_embedding);
            let score = score_overrides
                .as_ref()
                .and_then(|overrides| {
                    overrides.get(&duplicate_pair_key(&left.endpoint, &right.endpoint))
                })
                .copied()
                .unwrap_or(raw_score);
            if score < SEMANTIC_AUDIT_THRESHOLD {
                continue;
            }
            candidates.push(DuplicateCandidate {
                kind: "semantic_candidate",
                left: left.endpoint.clone(),
                right: right.endpoint.clone(),
                relation: "semantic_similarity",
                assertion_shape: None,
                facts: None,
                distinguishing_facts: Vec::new(),
                fact_contradiction: false,
                score: Some(score),
                bge: None,
                bge_admission: None,
                bge_token_cap: None,
                #[cfg(feature = "nli-evidence")]
                nli: None,
                #[cfg(feature = "nli-evidence")]
                nli_admission: None,
                #[cfg(feature = "nli-evidence")]
                nli_token_cap: None,
            });
        }
    }
    candidates.sort_by(duplicate_candidate_order);
    Ok(DuplicateCandidateSet {
        candidates,
        e5_token_caps,
    })
}

#[cfg(feature = "spec-retrieval")]
fn structural_candidate(
    left: &DuplicateScenario,
    right: &DuplicateScenario,
    relation: &'static str,
) -> DuplicateCandidate {
    DuplicateCandidate {
        kind: "structural_overlap",
        left: left.endpoint.clone(),
        right: right.endpoint.clone(),
        relation,
        assertion_shape: Some(DuplicateAssertionShape {
            left: left.fingerprint.assertions.clone(),
            right: right.fingerprint.assertions.clone(),
        }),
        facts: Some(DuplicateFactsEvidence {
            left: DuplicateFacts::from_fingerprint(&left.fingerprint),
            right: DuplicateFacts::from_fingerprint(&right.fingerprint),
        }),
        distinguishing_facts: distinguishing_facts(
            &left.fingerprint.facts,
            &right.fingerprint.facts,
        ),
        fact_contradiction: left.fingerprint.facts.contradicts(&right.fingerprint.facts),
        score: None,
        bge: None,
        bge_admission: None,
        bge_token_cap: None,
        #[cfg(feature = "nli-evidence")]
        nli: None,
        #[cfg(feature = "nli-evidence")]
        nli_admission: None,
        #[cfg(feature = "nli-evidence")]
        nli_token_cap: None,
    }
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_candidate_order(
    left: &DuplicateCandidate,
    right: &DuplicateCandidate,
) -> std::cmp::Ordering {
    duplicate_kind_order(left.kind)
        .cmp(&duplicate_kind_order(right.kind))
        .then_with(|| {
            if left.kind == "structural_overlap" && right.kind == "structural_overlap" {
                duplicate_relation_order(left.relation)
                    .cmp(&duplicate_relation_order(right.relation))
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .then_with(|| {
            if left.kind == "semantic_candidate" && right.kind == "semantic_candidate" {
                right.score.unwrap().total_cmp(&left.score.unwrap())
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .then_with(|| duplicate_endpoint_key(&left.left).cmp(&duplicate_endpoint_key(&right.left)))
        .then_with(|| {
            duplicate_endpoint_key(&left.right).cmp(&duplicate_endpoint_key(&right.right))
        })
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_kind_order(kind: &str) -> u8 {
    [
        "exact_duplicate",
        "structural_overlap",
        "semantic_candidate",
    ]
    .iter()
    .position(|candidate| *candidate == kind)
    .unwrap() as u8
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_relation_order(relation: &str) -> u8 {
    [
        "same_assertion_shape",
        "left_assertions_subset_of_right",
        "right_assertions_subset_of_left",
    ]
    .iter()
    .position(|candidate| *candidate == relation)
    .unwrap() as u8
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_endpoint_key(endpoint: &DuplicateEndpoint) -> (&str, &str) {
    (&endpoint.file, &endpoint.title)
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_pair_key(left: &DuplicateEndpoint, right: &DuplicateEndpoint) -> DuplicatePairKey {
    let left = (left.file.clone(), left.title.clone());
    let right = (right.file.clone(), right.title.clone());
    (left, right)
}

#[cfg(feature = "spec-retrieval")]
fn duplicate_score_overrides() -> Result<Option<BTreeMap<DuplicatePairKey, f32>>> {
    if spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(None);
    }
    let Some(raw) = spec_env("GIVN_SPEC_DUPLICATES_TEST_SCORES") else {
        return Ok(None);
    };
    let raw = raw
        .to_str()
        .context("GIVN_SPEC_DUPLICATES_TEST_SCORES is not valid UTF-8")?;
    if raw.trim().is_empty() {
        bail!("GIVN_SPEC_DUPLICATES_TEST_SCORES must not be empty");
    }

    let mut overrides = BTreeMap::new();
    for entry in raw.split(';') {
        let entry = entry.trim();
        if entry.is_empty() {
            bail!("GIVN_SPEC_DUPLICATES_TEST_SCORES contains an empty entry");
        }
        let (pair, score) = entry.rsplit_once('=').with_context(|| {
            format!(
                "malformed duplicate score entry `{entry}`; expected <endpoint>::<endpoint>=<score>"
            )
        })?;
        let (left, right) = pair.split_once("::").with_context(|| {
            format!("malformed duplicate score pair `{pair}`; expected two endpoint identities")
        })?;
        let left = parse_duplicate_endpoint_key(left)?;
        let right = parse_duplicate_endpoint_key(right)?;
        if left == right {
            bail!("duplicate score pair must contain two distinct endpoints: `{pair}`");
        }
        let score = parse_duplicate_score(score)?;
        let key = if left <= right {
            (left, right)
        } else {
            (right, left)
        };
        if overrides.insert(key, score).is_some() {
            bail!("duplicate score pair is repeated: `{pair}`");
        }
    }
    Ok(Some(overrides))
}

#[cfg(feature = "spec-retrieval")]
fn parse_duplicate_endpoint_key(value: &str) -> Result<(String, String)> {
    let (file, title) = value.split_once('|').with_context(|| {
        format!("malformed duplicate endpoint `{value}`; expected <file>|<title>")
    })?;
    let file = file.trim().replace('\\', "/");
    let title = title.trim();
    if !file.starts_with("givn/specs/")
        || file.contains("//")
        || file.split('/').any(|part| part == "." || part == "..")
        || title.is_empty()
        || title.contains('\n')
        || title.contains('\r')
    {
        bail!("invalid duplicate endpoint identity `{value}`");
    }
    Ok((file, title.to_string()))
}

#[cfg(feature = "spec-retrieval")]
fn parse_duplicate_score(value: &str) -> Result<f32> {
    let valid_shape = value.len() >= 6
        && value.matches('.').count() == 1
        && value.split_once('.').is_some_and(|(whole, fraction)| {
            !whole.is_empty()
                && fraction.len() == 4
                && whole
                    .strip_prefix('-')
                    .unwrap_or(whole)
                    .chars()
                    .all(|character| character.is_ascii_digit())
                && fraction.chars().all(|character| character.is_ascii_digit())
        });
    if !valid_shape {
        bail!("invalid duplicate score `{value}`; expected four decimal places");
    }
    let score = value
        .parse::<f32>()
        .with_context(|| format!("invalid duplicate score `{value}`"))?;
    if !score.is_finite() || !(-1.0..=1.0).contains(&score) {
        bail!("duplicate score `{value}` is outside the cosine range -1.0000..=1.0000");
    }
    Ok(score)
}

#[cfg(feature = "spec-retrieval")]
fn assertion_subset_relation(left: &[String], right: &[String]) -> Option<&'static str> {
    if left.is_empty() || right.is_empty() {
        return None;
    }
    if left.len() < right.len() && is_subsequence(left, right) {
        Some("left_assertions_subset_of_right")
    } else if right.len() < left.len() && is_subsequence(right, left) {
        Some("right_assertions_subset_of_left")
    } else {
        None
    }
}

#[cfg(feature = "spec-retrieval")]
fn is_subsequence(needle: &[String], haystack: &[String]) -> bool {
    let mut haystack = haystack.iter();
    needle
        .iter()
        .all(|assertion| haystack.any(|candidate| candidate == assertion))
}

#[cfg(feature = "spec-retrieval")]
impl DuplicateFacts {
    fn from_fingerprint(fingerprint: &crate::overlap::fingerprint::Fingerprint) -> Self {
        Self {
            exit_code: fingerprint.facts.exit_code.clone(),
            contains_polarity: fingerprint.facts.contains_polarity.clone(),
            tables: fingerprint.facts.tables.clone(),
            done: fingerprint.facts.done,
            tty: fingerprint.facts.tty,
        }
    }
}

#[cfg(feature = "spec-retrieval")]
fn distinguishing_facts(
    left: &crate::overlap::fingerprint::Facts,
    right: &crate::overlap::fingerprint::Facts,
) -> Vec<DistinguishingFact> {
    let mut facts = Vec::new();
    if left.exit_code != right.exit_code {
        facts.push(DistinguishingFact {
            field: "exit_code".to_string(),
            left: json_encoded_value(&left.exit_code),
            right: json_encoded_value(&right.exit_code),
        });
    }

    let positions: BTreeSet<usize> = left
        .contains_polarity
        .keys()
        .chain(right.contains_polarity.keys())
        .copied()
        .collect();
    for position in positions {
        let left_value = left.contains_polarity.get(&position);
        let right_value = right.contains_polarity.get(&position);
        if left_value != right_value {
            facts.push(DistinguishingFact {
                field: format!("contains_polarity[{position}]"),
                left: json_encoded_value(&left_value),
                right: json_encoded_value(&right_value),
            });
        }
    }

    let table_keys: BTreeSet<&String> = left.tables.keys().chain(right.tables.keys()).collect();
    for key in table_keys {
        let left_value = left.tables.get(key);
        let right_value = right.tables.get(key);
        if left_value != right_value {
            facts.push(DistinguishingFact {
                field: format!("tables[{key}]"),
                left: json_encoded_value(&left_value),
                right: json_encoded_value(&right_value),
            });
        }
    }

    if left.done != right.done {
        facts.push(DistinguishingFact {
            field: "done".to_string(),
            left: serde_json::Value::Bool(left.done),
            right: serde_json::Value::Bool(right.done),
        });
    }
    if left.tty != right.tty {
        facts.push(DistinguishingFact {
            field: "tty".to_string(),
            left: serde_json::Value::Bool(left.tty),
            right: serde_json::Value::Bool(right.tty),
        });
    }
    facts
}

#[cfg(feature = "spec-retrieval")]
fn json_encoded_value<T: serde::Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).expect("serialize distinguishing fact")
}

#[cfg(feature = "spec-retrieval")]
fn collect_scenarios_from(root: &Path, specs_root: &Path) -> Result<Vec<ScenarioText>> {
    let mut scenarios = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(specs_root) {
        let feature = parse_permanent_feature(root, &path)?;
        let file = normalize_project_file_identity(root, &path);
        collect_feature_scenarios(&feature, None, &feature.scenarios, &file, &mut scenarios);
        for rule in &feature.rules {
            collect_feature_scenarios(&feature, Some(rule), &rule.scenarios, &file, &mut scenarios);
        }
    }
    scenarios.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.title.cmp(&right.title))
    });
    Ok(scenarios)
}

#[cfg(feature = "spec-retrieval")]
fn collect_delta_scenarios(root: &Path, delta_root: &Path) -> Result<Vec<DeltaScenario>> {
    let mut scenarios = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(delta_root) {
        let feature = gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default()).map_err(
            |error| {
                anyhow::anyhow!(
                    "delta feature is not valid Gherkin ({}): {error}",
                    path.display()
                )
            },
        )?;
        let capability = crate::gherkin::delta::capability_tag(&feature)
            .map(str::to_string)
            .or_else(|| {
                path.parent()
                    .and_then(Path::file_name)
                    .and_then(|name| name.to_str())
                    .map(str::to_string)
            })
            .or_else(|| {
                path.file_stem()
                    .and_then(|name| name.to_str())
                    .map(str::to_string)
            })
            .context("delta feature has no capability or filename for similarity projection")?;
        let target_file = normalize_project_file_identity(
            root,
            Path::new(&format!("givn/specs/{capability}/{capability}.feature")),
        );
        collect_delta_feature_scenarios(
            root,
            &feature.scenarios,
            &path,
            &target_file,
            &feature,
            None,
            &mut scenarios,
        );
        for rule in &feature.rules {
            collect_delta_feature_scenarios(
                root,
                &rule.scenarios,
                &path,
                &target_file,
                &feature,
                Some(rule),
                &mut scenarios,
            );
        }
    }
    scenarios.sort_by(|left, right| {
        left.scenario
            .file
            .cmp(&right.scenario.file)
            .then_with(|| left.scenario.title.cmp(&right.scenario.title))
    });
    Ok(scenarios)
}

#[cfg(feature = "spec-retrieval")]
fn collect_delta_feature_scenarios(
    root: &Path,
    source: &[gherkin::Scenario],
    path: &Path,
    target_file: &str,
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    output: &mut Vec<DeltaScenario>,
) {
    let file = normalize_project_file_identity(root, path);
    for scenario in source {
        let evidence_json = canonical_evidence_json(feature, rule, scenario);
        output.push(DeltaScenario {
            scenario: scenario_text(feature, rule, scenario, &file),
            target_file: target_file.to_string(),
            operation: crate::gherkin::delta::delta_op(scenario),
            facts: crate::overlap::fingerprint::Fingerprint::of_scenario(
                scenario,
                rule.and_then(|rule| rule.background.as_ref())
                    .or(feature.background.as_ref()),
            ),
            evidence_json,
            is_outline: scenario.keyword == "Scenario Outline",
            has_examples: !scenario.examples.is_empty(),
        });
    }
}

#[cfg(feature = "spec-retrieval")]
fn collect_feature_scenarios(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    source: &[gherkin::Scenario],
    file: &str,
    output: &mut Vec<ScenarioText>,
) {
    for scenario in source {
        if !is_queryable_scenario(scenario) {
            continue;
        }
        output.push(scenario_text(feature, rule, scenario, file));
    }
}

#[cfg(feature = "spec-retrieval")]
fn canonical_evidence_json(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    scenario: &gherkin::Scenario,
) -> String {
    format!(
        "{{\"feature_description\":{},\"feature_tags\":{},\"rule_description\":{},\"rule_tags\":{},\"scenario_description\":{},\"scenario_tags\":{},\"outline_name\":{},\"examples\":[{}],\"feature_background_steps\":{},\"rule_background_steps\":{},\"normalized_steps\":{},\"model_id\":{},\"index_version\":{},\"fingerprint_schema_version\":{},\"evidence_schema_version\":{},\"report_schema_version\":{}}}",
        json_encoded(&feature.description),
        json_encoded(&feature.tags),
        json_encoded(&rule.and_then(|rule| rule.description.clone())),
        json_encoded(&rule.map(|rule| rule.tags.clone()).unwrap_or_default()),
        json_encoded(&scenario.description),
        json_encoded(&scenario.tags),
        json_encoded(&if scenario.examples.is_empty() {
            None::<String>
        } else {
            Some(scenario.name.clone())
        }),
        scenario
            .examples
            .iter()
            .map(canonical_examples)
            .collect::<Vec<_>>()
            .join(","),
        canonical_background(feature.background.as_ref()),
        canonical_background(rule.and_then(|rule| rule.background.as_ref())),
        canonical_steps(&scenario.steps),
        json_encoded(&MODEL_ID),
        INDEX_VERSION,
        SEMANTIC_FINGERPRINT_SCHEMA_VERSION,
        SEMANTIC_EVIDENCE_SCHEMA_VERSION,
        SEMANTIC_REPORT_SCHEMA_VERSION,
    )
}

#[cfg(feature = "spec-retrieval")]
fn canonical_background(background: Option<&gherkin::Background>) -> String {
    background
        .map(|background| canonical_steps(&background.steps))
        .unwrap_or_else(|| "[]".to_string())
}

#[cfg(feature = "spec-retrieval")]
fn canonical_steps(steps: &[gherkin::Step]) -> String {
    format!(
        "[{}]",
        steps
            .iter()
            .map(|step| {
                format!(
                    "{{\"text\":{},\"docstring\":{},\"data_table\":{}}}",
                    json_encoded(&format!("{} {}", step.keyword.trim(), step.value.trim())),
                    json_encoded(&step.docstring),
                    json_encoded(&step.table.as_ref().map(|table| table.rows.to_vec())),
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(feature = "spec-retrieval")]
fn canonical_examples(examples: &gherkin::Examples) -> String {
    format!(
        "{{\"name\":{},\"description\":{},\"tags\":{},\"table\":{}}}",
        json_encoded(&examples.name),
        json_encoded(&examples.description),
        json_encoded(&examples.tags),
        json_encoded(&examples.table.as_ref().map(|table| table.rows.to_vec())),
    )
}

#[cfg(feature = "spec-retrieval")]
fn json_encoded<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("serialize semantic evidence value")
}

#[cfg(feature = "spec-retrieval")]
fn scenario_text(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    scenario: &gherkin::Scenario,
    file: &str,
) -> ScenarioText {
    let text = semantic_serialization(feature, rule, scenario);
    ScenarioText {
        title: scenario.name.clone(),
        file: file.to_string(),
        content_hash: content_hash(&text),
        text,
    }
}

#[cfg(feature = "spec-retrieval")]
fn semantic_serialization(
    feature: &gherkin::Feature,
    rule: Option<&gherkin::Rule>,
    scenario: &gherkin::Scenario,
) -> String {
    let mut sections = Vec::new();
    sections.push(format!(
        "feature: {} {}",
        semantic_normalize(&feature.keyword),
        semantic_normalize(&feature.name)
    ));
    push_optional_text_section(
        &mut sections,
        "feature_description",
        feature.description.as_deref(),
    );
    push_optional_serialized_section(
        &mut sections,
        "feature_background",
        semantic_background(feature.background.as_ref()),
    );
    if let Some(rule) = rule {
        sections.push(format!(
            "rule: {} {}",
            semantic_normalize(&rule.keyword),
            semantic_normalize(&rule.name)
        ));
        push_optional_text_section(
            &mut sections,
            "rule_description",
            rule.description.as_deref(),
        );
        push_optional_serialized_section(
            &mut sections,
            "rule_background",
            semantic_background(rule.background.as_ref()),
        );
    }
    sections.push(format!(
        "scenario: {} {}",
        semantic_normalize(&scenario.keyword),
        semantic_normalize(&scenario.name)
    ));
    push_optional_text_section(
        &mut sections,
        "scenario_description",
        scenario.description.as_deref(),
    );
    if !scenario.steps.is_empty() {
        sections.push("steps:".to_string());
        sections.extend(scenario.steps.iter().map(semantic_step));
    }
    if scenario.keyword.trim() == "Scenario Outline"
        && let Some(template) = semantic_optional_text(Some(&scenario.name))
    {
        sections.push(format!("outline_template: {template}"));
    }
    if scenario.examples.is_empty() {
        return sections.join("\n");
    } else {
        let examples = scenario
            .examples
            .iter()
            .filter_map(semantic_examples)
            .collect::<Vec<_>>();
        if !examples.is_empty() {
            sections.push("examples:".to_string());
            sections.extend(examples);
        }
    }
    sections.join("\n")
}

#[cfg(feature = "spec-retrieval")]
fn semantic_background(background: Option<&gherkin::Background>) -> Option<String> {
    let background = background?;
    let name = semantic_optional_text(Some(&background.name));
    let description = semantic_optional_text(background.description.as_deref());
    let has_steps = !background.steps.is_empty();
    if name.is_none() && description.is_none() && !has_steps {
        return None;
    }

    let mut fields = Vec::new();
    push_optional_json_field(
        &mut fields,
        "keyword",
        Some(semantic_normalize(&background.keyword)),
    );
    if let Some(name) = name {
        fields.push(format!("\"name\":{}", json_encoded(&name)));
    }
    if let Some(description) = description {
        fields.push(format!("\"description\":{}", json_encoded(&description)));
    }
    if has_steps {
        let steps = background
            .steps
            .iter()
            .map(semantic_step)
            .map(|step| json_encoded(&step))
            .collect::<Vec<_>>()
            .join(",");
        fields.push(format!("\"steps\":[{steps}]"));
    }
    (!fields.is_empty()).then(|| format!("{{{}}}", fields.join(",")))
}

#[cfg(feature = "spec-retrieval")]
fn semantic_step(step: &gherkin::Step) -> String {
    let mut fields = vec![
        format!("role={:?}", step.ty),
        format!("text={}", semantic_normalize(&step.value)),
    ];
    if let Some(table) = step.table.as_ref().and_then(semantic_table) {
        fields.push(format!("table={table}"));
    }
    if let Some(docstring) = semantic_optional_text(step.docstring.as_deref()) {
        fields.push(format!("docstring={}", json_encoded(&docstring)));
    }
    format!("step: {}", fields.join("; "))
}

#[cfg(feature = "spec-retrieval")]
fn semantic_table(table: &gherkin::Table) -> Option<String> {
    if table.rows.is_empty() {
        return None;
    }
    Some(json_encoded(
        &table
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| semantic_normalize(cell))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    ))
}

#[cfg(feature = "spec-retrieval")]
fn semantic_examples(examples: &gherkin::Examples) -> Option<String> {
    let name = semantic_optional_text(examples.name.as_deref());
    let description = semantic_optional_text(examples.description.as_deref());
    let table = examples.table.as_ref().and_then(semantic_table);
    if name.is_none() && description.is_none() && table.is_none() {
        return None;
    }

    let mut fields = Vec::new();
    let keyword = semantic_normalize(&examples.keyword);
    if !keyword.is_empty() {
        fields.push(format!("keyword={keyword}"));
    }
    if let Some(name) = name {
        fields.push(format!("name={}", json_encoded(&name)));
    }
    if let Some(description) = description {
        fields.push(format!("description={}", json_encoded(&description)));
    }
    if let Some(table) = table {
        fields.push(format!("table={table}"));
    }
    (!fields.is_empty()).then(|| format!("examples_row: {}", fields.join(" ")))
}

#[cfg(feature = "spec-retrieval")]
fn semantic_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(semantic_normalize)
        .filter(|value| !value.is_empty())
}

#[cfg(feature = "spec-retrieval")]
fn push_optional_text_section(sections: &mut Vec<String>, label: &str, value: Option<&str>) {
    if let Some(value) = semantic_optional_text(value) {
        sections.push(format!("{label}: {}", json_encoded(&value)));
    }
}

#[cfg(feature = "spec-retrieval")]
fn push_optional_serialized_section(
    sections: &mut Vec<String>,
    label: &str,
    value: Option<String>,
) {
    if let Some(value) = value {
        sections.push(format!("{label}: {value}"));
    }
}

#[cfg(feature = "spec-retrieval")]
fn push_optional_json_field(fields: &mut Vec<String>, label: &str, value: Option<String>) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        fields.push(format!("\"{label}\":{}", json_encoded(&value)));
    }
}

#[cfg(feature = "spec-retrieval")]
fn semantic_normalize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(feature = "spec-retrieval")]
fn is_queryable_scenario(scenario: &gherkin::Scenario) -> bool {
    !scenario.tags.iter().any(|tag| tag == "givn.removed")
}

#[cfg(feature = "spec-retrieval")]
fn similarity_candidate_pool(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<CandidatePool> {
    let permanent_texts: BTreeMap<(String, String), String> = collect_scenarios(root)?
        .into_iter()
        .map(|scenario| ((scenario.file, scenario.title), scenario.text))
        .collect();
    let active_delta: Vec<&DeltaScenario> = delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
        .collect();
    let active_delta_identities: BTreeSet<(String, String)> = active_delta
        .iter()
        .map(|item| (item.target_file.clone(), item.scenario.title.clone()))
        .collect();
    let mut candidate_texts = permanent_texts.clone();
    for item in &active_delta {
        candidate_texts.insert(
            (item.target_file.clone(), item.scenario.title.clone()),
            item.scenario.text.clone(),
        );
    }
    let delta_texts: Vec<String> = active_delta
        .iter()
        .map(|item| item.scenario.text.clone())
        .collect();
    let delta_batch = if delta_texts.is_empty() {
        EmbeddingBatch {
            vectors: Vec::new(),
            token_caps: Vec::new(),
        }
    } else {
        embed_texts(root, &delta_texts, EmbeddingRole::Passage)?
    };

    let mut e5_token_caps = index.e5_token_caps.clone();
    for (item, budget) in active_delta.iter().zip(delta_batch.token_caps.iter()) {
        if let Some(budget) = budget {
            e5_token_caps.push(e5_token_cap(&item.scenario, *budget));
        }
    }
    e5_token_caps.sort_by(e5_token_cap_order);

    // The E5 admission floor is a property of the real embedder's semantic
    // space. Deterministic stub backends are canned scores, not a real E5
    // space, so the floor does not apply to them: the pinned format/exit-code
    // fixtures feed pre-floored pairs and stay deterministic.
    let e5_admission = !matches!(spec_env_var("GIVN_SPEC_EMBEDDER").as_deref(), Ok("stub"));

    let mut candidates = BTreeMap::new();
    for (active_index, delta) in active_delta.iter().enumerate() {
        let Some(vector) = delta_batch.vectors[active_index].as_ref() else {
            continue;
        };
        let own_target = (delta.target_file.as_str(), delta.scenario.title.as_str());
        let mut neighbours: Vec<(f32, &IndexEntry)> = index
            .entries
            .iter()
            .filter(|entry| !((entry.file.as_str(), entry.title.as_str()) == own_target))
            .map(|entry| (cosine_similarity(vector, &entry.embedding), entry))
            .collect();
        neighbours.sort_by(|left, right| right.0.total_cmp(&left.0));
        for (score, entry) in neighbours
            .into_iter()
            .take_while(|(score, _)| !e5_admission || *score >= E5_ADMISSION_THRESHOLD)
        {
            add_similarity_candidate(
                &mut candidates,
                "delta-to-permanent",
                endpoint(&delta.target_file, &delta.scenario.title),
                endpoint(&entry.file, &entry.title),
                score,
            );
        }
    }

    let projected = project_records(index, delta, &delta_batch.vectors, &active_delta)?;
    for (left, right) in projected.iter().enumerate() {
        let right_identity = (right.endpoint.file.clone(), right.endpoint.title.clone());
        let mut neighbours: Vec<(f32, &ScenarioRecord)> = projected
            .iter()
            .enumerate()
            .filter(|(index, other)| *index != left && other.endpoint.file != right.endpoint.file)
            .map(|(_, other)| (cosine_similarity(&right.embedding, &other.embedding), other))
            .collect();
        neighbours.sort_by(|left, right| right.0.total_cmp(&left.0));
        for (score, other) in neighbours
            .into_iter()
            .take_while(|(score, _)| !e5_admission || *score >= E5_ADMISSION_THRESHOLD)
        {
            // A change's review only judges candidates the change itself is
            // part of. Permanent-permanent pairs that pre-date the change are
            // corpus hygiene (givn spec duplicates), not this change's signal.
            let other_identity = (other.endpoint.file.clone(), other.endpoint.title.clone());
            if !active_delta_identities.contains(&right_identity)
                && !active_delta_identities.contains(&other_identity)
            {
                continue;
            }
            add_similarity_candidate(
                &mut candidates,
                "projected-permanent",
                right.endpoint.clone(),
                other.endpoint.clone(),
                score,
            );
        }
    }

    let result: Vec<SimilarityCandidate> = candidates.into_values().collect();
    let considered = result.len();
    Ok((result, considered, candidate_texts, e5_token_caps))
}

#[cfg(all(feature = "spec-retrieval", feature = "spec-similarity-gate"))]
fn embedding_similarity_candidates(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<(Vec<SimilarityCandidate>, usize, Vec<E5TokenCap>)> {
    let active_delta: Vec<&DeltaScenario> = delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
        .collect();
    let active_delta_identities: HashSet<(String, String)> = active_delta
        .iter()
        .map(|item| (item.target_file.clone(), item.scenario.title.clone()))
        .collect();
    let delta_texts: Vec<String> = active_delta
        .iter()
        .map(|item| item.scenario.text.clone())
        .collect();
    let delta_batch = if delta_texts.is_empty() {
        EmbeddingBatch {
            vectors: Vec::new(),
            token_caps: Vec::new(),
        }
    } else {
        embed_texts(root, &delta_texts, EmbeddingRole::Passage)?
    };
    let mut e5_token_caps = index.e5_token_caps.clone();
    for (item, budget) in active_delta.iter().zip(delta_batch.token_caps.iter()) {
        if let Some(budget) = budget {
            e5_token_caps.push(e5_token_cap(&item.scenario, *budget));
        }
    }
    e5_token_caps.sort_by(e5_token_cap_order);

    let mut candidates = BTreeMap::new();
    for (active_index, delta) in active_delta.iter().enumerate() {
        let Some(vector) = delta_batch.vectors[active_index].as_ref() else {
            continue;
        };
        let own_target = (delta.target_file.as_str(), delta.scenario.title.as_str());
        for entry in &index.entries {
            if delta.operation == crate::gherkin::delta::DeltaOp::Modified
                && (entry.file.as_str(), entry.title.as_str()) == own_target
            {
                continue;
            }
            let score = cosine_similarity(vector, &entry.embedding);
            if score >= E5_ADMISSION_THRESHOLD {
                add_similarity_candidate(
                    &mut candidates,
                    "delta-to-permanent",
                    endpoint(&delta.target_file, &delta.scenario.title),
                    endpoint(&entry.file, &entry.title),
                    score,
                );
            }
        }
    }

    let projected = project_records(index, delta, &delta_batch.vectors, &active_delta)?;
    for (left, right) in projected.iter().enumerate() {
        for other in projected.iter().skip(left + 1) {
            let left_is_delta = active_delta_identities
                .contains(&(right.endpoint.file.clone(), right.endpoint.title.clone()));
            let right_is_delta = active_delta_identities
                .contains(&(other.endpoint.file.clone(), other.endpoint.title.clone()));
            if !left_is_delta && !right_is_delta {
                continue;
            }
            let score = cosine_similarity(&right.embedding, &other.embedding);
            if score >= E5_ADMISSION_THRESHOLD {
                add_similarity_candidate(
                    &mut candidates,
                    if left_is_delta && right_is_delta {
                        "delta-to-delta"
                    } else {
                        "delta-to-permanent"
                    },
                    right.endpoint.clone(),
                    other.endpoint.clone(),
                    score,
                );
            }
        }
    }

    let mut result: Vec<SimilarityCandidate> = candidates.into_values().collect();
    let considered = result.len();
    result.sort_by(|left, right| {
        right
            .embedding_score
            .total_cmp(&left.embedding_score)
            .then_with(|| left.left.cmp(&right.left))
            .then_with(|| left.right.cmp(&right.right))
    });
    Ok((result, considered, e5_token_caps))
}

#[cfg(feature = "nli-evidence")]
fn nli_backend_selection(
    root: &Path,
) -> Result<Option<std::result::Result<NliBackend, NliUnavailableReason>>> {
    match spec_env_var("GIVN_SPEC_RERANKER").as_deref() {
        Ok("nli") => Ok(Some(match preflight_nli_artifacts(root) {
            Ok(_) => Ok(NliBackend::Local),
            Err(_) => Err(NliUnavailableReason::MissingArtifact),
        })),
        Ok("nli-auto") => Ok(Some(match auto_nli_artifacts(root) {
            Ok(_) => Ok(NliBackend::Auto),
            Err(error) => {
                eprintln!(
                    "NLI model is unavailable; automatic acquisition failed: {error:#}; set GIVN_SPEC_NLI_MODEL_DIR for a validated local artifact set"
                );
                Err(NliUnavailableReason::MissingArtifact)
            }
        })),
        Ok(value) if value == NLI_STUB_SELECTOR => Ok(Some(Ok(NliBackend::Stub))),
        Ok("nli-missing") => {
            eprintln!(
                "NLI model is unavailable; model={NLI_MODEL_ID}; failed artifact=onnx/model.onnx; set GIVN_SPEC_NLI_MODEL_DIR or GIVN_SPEC_NLI_CACHE"
            );
            Ok(Some(Err(NliUnavailableReason::MissingArtifact)))
        }
        Ok("disabled") | Ok("stub") | Ok("missing") => Ok(None),
        Ok(value) => bail!("unknown GIVN_SPEC_RERANKER value `{value}`"),
        Err(std::env::VarError::NotPresent) => Ok(Some(match auto_nli_artifacts(root) {
            Ok(_) => Ok(NliBackend::Auto),
            Err(error) => {
                eprintln!(
                    "NLI model is unavailable; automatic acquisition failed: {error:#}; set GIVN_SPEC_NLI_MODEL_DIR for a validated local artifact set"
                );
                Err(NliUnavailableReason::MissingArtifact)
            }
        })),
        Err(std::env::VarError::NotUnicode(_)) => {
            bail!("GIVN_SPEC_RERANKER is not valid UTF-8")
        }
    }
}

#[cfg(feature = "nli-evidence")]
fn preflight_nli_artifacts(root: &Path) -> Result<NliArtifacts> {
    // Model-location request: honoured in every build kind — accepted
    // exception, see arc42 ch. 08 § 8.5a (identity not verified).
    let directory = spec_env("GIVN_SPEC_NLI_MODEL_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or(model_cache_dir(root, "nli")?);
    let model_path = directory.join("model.onnx");
    anyhow::ensure!(
        fs::metadata(&model_path)
            .with_context(|| format!("read NLI model artifact {}", model_path.display()))?
            .len()
            > 0,
        "NLI model artifact is empty"
    );
    for name in ["config.json", "tokenizer.json", "tokenizer_config.json"] {
        let path = directory.join(name);
        let bytes = fs::read(&path)
            .with_context(|| format!("read NLI model artifact {}", path.display()))?;
        if name != "tokenizer.json" {
            serde_json::from_slice::<serde_json::Value>(&bytes)
                .with_context(|| format!("parse NLI model artifact {}", path.display()))?;
        }
    }
    load_unbounded_tokenizer(&directory.join("tokenizer.json"))?;
    let config: serde_json::Value = serde_json::from_slice(
        &fs::read(directory.join("config.json")).context("read NLI config.json")?,
    )
    .context("parse NLI config.json")?;
    nli_label_indices(&config)?;
    Ok(NliArtifacts {
        tokenizer_path: directory.join("tokenizer.json"),
        model_path,
        directory,
    })
}

#[cfg(feature = "nli-evidence")]
fn score_nli_pairs(
    root: &Path,
    pairs: &[(SimilarityEndpoint, SimilarityEndpoint)],
    texts: &BTreeMap<(String, String), String>,
    backend: NliBackend,
    max_unique_pairs: usize,
) -> Result<NliRun> {
    let started = std::time::Instant::now();
    let overrides = nli_directional_overrides()?;
    let mut evidence = BTreeMap::new();
    match backend {
        NliBackend::Stub => {
            for (left, right) in pairs {
                let forward = nli_directional_logits(left, right, texts, overrides.as_ref())?;
                let reverse = nli_directional_logits(right, left, texts, overrides.as_ref())?;
                evidence.insert(
                    canonical_pair_key(left.clone(), right.clone()),
                    nli_evidence(forward, reverse),
                );
            }
        }
        NliBackend::Local | NliBackend::Auto => {
            if spec_env_var("GIVN_SPEC_NLI_LOCAL_STUB").as_deref() == Ok("1") {
                for (left, right) in pairs {
                    let forward = nli_directional_logits(left, right, texts, overrides.as_ref())?;
                    let reverse = nli_directional_logits(right, left, texts, overrides.as_ref())?;
                    evidence.insert(
                        canonical_pair_key(left.clone(), right.clone()),
                        nli_evidence(forward, reverse),
                    );
                }
            } else {
                let mut classifier = NliClassifier::load(root, matches!(backend, NliBackend::Auto))
                    .context("NLI model is unavailable; check GIVN_SPEC_NLI_MODEL_DIR")?;
                let mut input_pairs = Vec::with_capacity(pairs.len() * 2);
                for (left, right) in pairs {
                    let left_text = scenario_text_for(texts, left)?;
                    let right_text = scenario_text_for(texts, right)?;
                    input_pairs.push((left_text.to_string(), right_text.to_string()));
                    input_pairs.push((right_text.to_string(), left_text.to_string()));
                }
                let logits = classifier.score_pairs(&input_pairs)?;
                for ((left, right), pair_logits) in pairs.iter().zip(logits.as_chunks::<2>().0) {
                    evidence.insert(
                        canonical_pair_key(left.clone(), right.clone()),
                        nli_evidence(pair_logits[0], pair_logits[1]),
                    );
                }
            }
        }
    }
    Ok(NliRun::enabled(
        evidence,
        pairs.len(),
        max_unique_pairs,
        started.elapsed().as_millis(),
    ))
}

#[cfg(feature = "nli-evidence")]
fn nli_directional_logits(
    left: &SimilarityEndpoint,
    right: &SimilarityEndpoint,
    texts: &BTreeMap<(String, String), String>,
    overrides: Option<&NliDirectionalOverrides>,
) -> Result<[f32; 3]> {
    if let Some(logits) = overrides.and_then(|items| items.get(&(left.clone(), right.clone()))) {
        return Ok(*logits);
    }
    Ok(stub_nli_logits(
        scenario_text_for(texts, left)?,
        scenario_text_for(texts, right)?,
    ))
}

#[cfg(feature = "nli-evidence")]
fn nli_directional_overrides() -> Result<Option<NliDirectionalOverrides>> {
    let Some(raw) = spec_env("GIVN_SPEC_NLI_DIRECTIONAL_SCORES") else {
        return Ok(None);
    };
    let raw = raw
        .to_str()
        .context("GIVN_SPEC_NLI_DIRECTIONAL_SCORES is not valid UTF-8")?;
    if raw.trim().is_empty() {
        bail!("GIVN_SPEC_NLI_DIRECTIONAL_SCORES must not be empty");
    }
    let mut overrides = BTreeMap::new();
    for entry in raw.split(';') {
        let (pair, values) = entry
            .split_once('=')
            .with_context(|| format!("malformed NLI directional evidence entry `{entry}`"))?;
        let (left, right) = pair
            .split_once("->")
            .with_context(|| format!("malformed NLI directional evidence entry `{entry}`"))?;
        let left = parse_nli_endpoint(left)?;
        let right = parse_nli_endpoint(right)?;
        let values = values
            .split(',')
            .map(|value| {
                value
                    .parse::<f32>()
                    .with_context(|| format!("malformed NLI directional evidence entry `{entry}`"))
            })
            .collect::<Result<Vec<_>>>()?;
        anyhow::ensure!(
            values.len() == 3 && values.iter().all(|value| value.is_finite()),
            "malformed NLI directional evidence entry `{entry}`"
        );
        if overrides
            .insert((left, right), [values[0], values[1], values[2]])
            .is_some()
        {
            bail!("malformed NLI directional evidence entry `{entry}`");
        }
    }
    for (left, right) in overrides.keys() {
        if !overrides.contains_key(&(right.clone(), left.clone())) {
            bail!("malformed NLI directional evidence entry: missing reverse direction");
        }
    }
    Ok(Some(overrides))
}

#[cfg(feature = "nli-evidence")]
fn parse_nli_endpoint(value: &str) -> Result<SimilarityEndpoint> {
    let (file, title) = value
        .trim()
        .split_once('|')
        .with_context(|| format!("malformed NLI directional evidence endpoint `{value}`"))?;
    if file.trim().is_empty()
        || title.trim().is_empty()
        || value.contains('\n')
        || value.contains('\r')
    {
        bail!("malformed NLI directional evidence endpoint `{value}`");
    }
    Ok(endpoint(file.trim(), title.trim()))
}

#[cfg(feature = "nli-evidence")]
fn print_nli_results(
    candidates: &[SimilarityCandidate],
    considered: usize,
    max_unique_pairs: usize,
    verbose: bool,
) {
    let mut label_counts = [0usize; 3];
    for candidate in candidates {
        if let Some(evidence) = candidate.nli {
            let (index, _) = evidence
                .symmetric
                .iter()
                .copied()
                .enumerate()
                .max_by(|left, right| left.1.total_cmp(&right.1))
                .unwrap_or((1, 0.0));
            label_counts[index] += 1;
        }
    }
    // Deprecated alias: parsed for compatibility but silently ignored.
    let _deprecated_report_limit = nli_report_limit();
    println!(
        "nli: model {}, top_k {}, considered {} candidate pairs; evidence is diagnostic only",
        NLI_MODEL_ID, RERANKER_TOP_K, considered
    );
    let recommended = candidates
        .iter()
        .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
        .count();
    let filtered = candidates
        .iter()
        .filter(|candidate| candidate.nli_admission == Some("NLI_FILTERED"))
        .count();
    println!(
        "NLI candidate accounting: raw {}, max pairs {}, recommended {}, NLI filtered {}, unscored {}, bounded pool",
        considered,
        max_unique_pairs,
        recommended,
        filtered,
        considered.saturating_sub(
            candidates
                .iter()
                .filter(|candidate| candidate.nli.is_some())
                .count(),
        )
    );
    println!(
        "nli distribution: entailment {}, neutral {}, contradiction {}",
        label_counts[0], label_counts[1], label_counts[2]
    );
    let scored_rows: Vec<&SimilarityCandidate> = candidates
        .iter()
        .filter(|candidate| candidate.nli.is_some())
        .collect();
    let entailment = |candidate: &SimilarityCandidate| {
        candidate
            .nli
            .map(|evidence| evidence.symmetric[0])
            .unwrap_or(f32::NEG_INFINITY)
    };
    let mut recommended_rows: Vec<&SimilarityCandidate> = scored_rows
        .iter()
        .copied()
        .filter(|candidate| candidate.nli_admission == Some("NLI_RECOMMENDED"))
        .collect();
    recommended_rows.sort_by(|left, right| entailment(right).total_cmp(&entailment(left)));
    let other_rows: Vec<&SimilarityCandidate> = scored_rows
        .iter()
        .copied()
        .filter(|candidate| candidate.nli_admission != Some("NLI_RECOMMENDED"))
        .collect();
    let mut rows: Vec<&SimilarityCandidate> = recommended_rows;
    if verbose {
        rows.extend(other_rows);
    }
    for candidate in &rows {
        let Some(evidence) = candidate.nli else {
            continue;
        };
        println!(
            "nli candidate [{}]: '{}' ({}) resembles '{}' ({}) embedding {:.4}; forward [entailment {:.4}, neutral {:.4}, contradiction {:.4}], reverse [entailment {:.4}, neutral {:.4}, contradiction {:.4}], symmetric [entailment {:.4}, neutral {:.4}, contradiction {:.4}], strongest {}; evidence is advisory and no Givn classification is assigned",
            candidate.scopes.join(","),
            candidate.left.title,
            candidate.left.file,
            candidate.right.title,
            candidate.right.file,
            candidate.embedding_score,
            evidence.forward[0],
            evidence.forward[1],
            evidence.forward[2],
            evidence.reverse[0],
            evidence.reverse[1],
            evidence.reverse[2],
            evidence.symmetric[0],
            evidence.symmetric[1],
            evidence.symmetric[2],
            nli_label(&evidence.symmetric),
        );
        println!(
            "NLI admission: {}",
            candidate.nli_admission.unwrap_or("NLI_UNSCORED")
        );
    }
    if rows.len() < scored_rows.len() {
        println!(
            "nli diagnostic: reported {} of {} scored candidates; NLI probabilities are not calibrated duplicate, variant, or boundary decisions",
            rows.len(),
            scored_rows.len()
        );
    }
}

#[cfg(feature = "nli-evidence")]
fn nli_report_limit() -> usize {
    spec_env_var("GIVN_SPEC_NLI_REPORT_LIMIT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NLI_REPORT_LIMIT)
}

#[cfg(feature = "nli-evidence")]
fn nli_evidence(forward_logits: [f32; 3], reverse_logits: [f32; 3]) -> NliEvidence {
    let forward = softmax3(forward_logits);
    let reverse = softmax3(reverse_logits);
    let symmetric = softmax3([
        (forward_logits[0] + reverse_logits[0]) / 2.0,
        (forward_logits[1] + reverse_logits[1]) / 2.0,
        (forward_logits[2] + reverse_logits[2]) / 2.0,
    ]);
    NliEvidence {
        forward,
        reverse,
        symmetric,
    }
}

#[cfg(feature = "nli-evidence")]
fn softmax3(logits: [f32; 3]) -> [f32; 3] {
    let maximum = logits.into_iter().fold(f32::NEG_INFINITY, f32::max);
    let exponentials = logits.map(|logit| (logit - maximum).exp());
    let total = exponentials.into_iter().sum::<f32>();
    exponentials.map(|value| value / total)
}

#[cfg(feature = "nli-evidence")]
fn nli_label(probabilities: &[f32; 3]) -> &'static str {
    let (index, _) = probabilities
        .iter()
        .copied()
        .enumerate()
        .max_by(|left, right| left.1.total_cmp(&right.1))
        .unwrap_or((1, 0.0));
    ["entailment", "neutral", "contradiction"][index]
}

#[cfg(feature = "nli-evidence")]
fn stub_nli_logits(left: &str, right: &str) -> [f32; 3] {
    let left_tokens = reranker_tokens(left);
    let right_tokens = reranker_tokens(right);
    let shared = left_tokens.intersection(&right_tokens).count() as f32;
    let union = left_tokens.union(&right_tokens).count() as f32;
    let overlap = if union == 0.0 { 0.0 } else { shared / union };
    if overlap >= 0.65 {
        [2.2, -0.2, 0.1]
    } else {
        [-0.4, 1.4, 0.2]
    }
}

#[cfg(feature = "nli-evidence")]
struct NliClassifier {
    tokenizer: Tokenizer,
    session: ort::session::Session,
    label_indices: [usize; 3],
    need_token_type_ids: bool,
}

#[cfg(feature = "nli-evidence")]
impl NliClassifier {
    fn load(root: &Path, automatic: bool) -> Result<Self> {
        let artifacts = nli_artifacts_for_backend(root, automatic)?;
        let model_dir = artifacts.directory;
        let config_path = model_dir.join("config.json");
        let config_bytes = fs::read(&config_path)
            .with_context(|| format!("read NLI config {}", config_path.display()))?;
        let config: serde_json::Value =
            serde_json::from_slice(&config_bytes).context("parse NLI config.json")?;
        let label_indices = nli_label_indices(&config)?;

        let tokenizer = load_unbounded_tokenizer(&artifacts.tokenizer_path)?;

        let session = ort::session::Session::builder()
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .with_intra_threads(resolve_thread_budget()?)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .commit_from_file(&artifacts.model_path)
            .with_context(|| format!("load NLI ONNX model {}", artifacts.model_path.display()))?;
        let need_token_type_ids = session
            .inputs()
            .iter()
            .any(|input| input.name() == "token_type_ids");

        Ok(Self {
            tokenizer,
            session,
            label_indices,
            need_token_type_ids,
        })
    }

    fn score_pairs(&mut self, pairs: &[(String, String)]) -> Result<Vec<[f32; 3]>> {
        let mut scores = Vec::with_capacity(pairs.len());
        for (left, right) in pairs {
            let encoding = self
                .tokenizer
                .encode((left.as_str(), right.as_str()), true)
                .map_err(|error| anyhow::anyhow!(error.to_string()))
                .context("tokenize NLI scenario pairs")?;
            let encoding_length = encoding.len();
            let ids = Array::from_shape_vec(
                (1, encoding_length),
                encoding
                    .get_ids()
                    .iter()
                    .map(|id| *id as i64)
                    .collect::<Vec<_>>(),
            )?;
            let masks = Array::from_shape_vec(
                (1, encoding_length),
                encoding
                    .get_attention_mask()
                    .iter()
                    .map(|mask| *mask as i64)
                    .collect::<Vec<_>>(),
            )?;
            let type_ids = Array::from_shape_vec(
                (1, encoding_length),
                encoding
                    .get_type_ids()
                    .iter()
                    .map(|id| *id as i64)
                    .collect::<Vec<_>>(),
            )?;
            let mut session_inputs = ort::inputs![
                "input_ids" => ort::value::Value::from_array(ids)?,
                "attention_mask" => ort::value::Value::from_array(masks)?,
            ];
            if self.need_token_type_ids {
                session_inputs.push((
                    "token_type_ids".into(),
                    ort::value::Value::from_array(type_ids)?.into(),
                ));
            }
            let outputs = self.session.run(session_inputs)?;
            let logits = outputs
                .get("logits")
                .context("NLI ONNX output has no logits tensor")?
                .try_extract_array::<f32>()
                .context("extract NLI logits")?;
            let values: Vec<f32> = logits.iter().copied().collect();
            anyhow::ensure!(
                values.len() == 3,
                "NLI logits shape has {} values for one pair",
                values.len(),
            );
            scores.push([
                values[self.label_indices[0]],
                values[self.label_indices[1]],
                values[self.label_indices[2]],
            ]);
        }
        Ok(scores)
    }
}

#[cfg(feature = "nli-evidence")]
fn nli_label_indices(config: &serde_json::Value) -> Result<[usize; 3]> {
    let labels = config["id2label"]
        .as_object()
        .context("NLI config.json has no id2label object")?;
    let mut indices = [None; 3];
    for (key, value) in labels {
        let index = key.parse::<usize>().context("parse NLI label index")?;
        let label = value
            .as_str()
            .context("NLI id2label value is not a string")?
            .to_ascii_lowercase();
        let slot = if label.contains("entail") {
            0
        } else if label.contains("neutral") {
            1
        } else if label.contains("contrad") {
            2
        } else {
            continue;
        };
        indices[slot] = Some(index);
    }
    Ok([
        indices[0].context("NLI config has no entailment label")?,
        indices[1].context("NLI config has no neutral label")?,
        indices[2].context("NLI config has no contradiction label")?,
    ])
}

#[cfg(feature = "spec-retrieval")]
fn project_records(
    index: &SpecIndex,
    delta: &[DeltaScenario],
    delta_vectors: &[Option<Vec<f32>>],
    active_delta: &[&DeltaScenario],
) -> Result<Vec<ScenarioRecord>> {
    let mut entries: BTreeMap<(String, String), ScenarioRecord> = index
        .entries
        .iter()
        .map(|entry| {
            let key = (entry.file.clone(), entry.title.clone());
            (
                key,
                ScenarioRecord {
                    endpoint: endpoint(&entry.file, &entry.title),
                    embedding: entry.embedding.clone(),
                },
            )
        })
        .collect();

    for item in delta {
        let key = (item.target_file.clone(), item.scenario.title.clone());
        match item.operation {
            crate::gherkin::delta::DeltaOp::Removed => {
                entries.remove(&key);
            }
            crate::gherkin::delta::DeltaOp::Modified => {
                entries.remove(&key);
                let position = active_delta
                    .iter()
                    .position(|active| std::ptr::eq(*active, item))
                    .context("modified delta vector is missing")?;
                if let Some(embedding) = delta_vectors[position].clone() {
                    entries.insert(
                        key,
                        ScenarioRecord {
                            endpoint: endpoint(&item.target_file, &item.scenario.title),
                            embedding,
                        },
                    );
                }
            }
            crate::gherkin::delta::DeltaOp::Added => {
                let position = active_delta
                    .iter()
                    .position(|active| std::ptr::eq(*active, item))
                    .context("added delta vector is missing")?;
                if let Some(embedding) = delta_vectors[position].clone() {
                    entries.insert(
                        key,
                        ScenarioRecord {
                            endpoint: endpoint(&item.target_file, &item.scenario.title),
                            embedding,
                        },
                    );
                }
            }
        }
    }

    Ok(entries.into_values().collect())
}

#[cfg(feature = "spec-retrieval")]
fn endpoint(file: &str, title: &str) -> SimilarityEndpoint {
    SimilarityEndpoint {
        file: file.to_string(),
        title: title.to_string(),
    }
}

#[cfg(feature = "spec-retrieval")]
fn semantic_candidate_order(
    left: &SemanticCandidate,
    right: &SemanticCandidate,
) -> std::cmp::Ordering {
    right
        .score
        .total_cmp(&left.score)
        .then_with(|| semantic_endpoint_key(&left.left).cmp(&semantic_endpoint_key(&right.left)))
        .then_with(|| semantic_endpoint_key(&left.right).cmp(&semantic_endpoint_key(&right.right)))
        .then_with(|| left.scope.cmp(right.scope))
        .then_with(|| left.candidate_id.cmp(&right.candidate_id))
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn similarity_candidate_order(
    left: &SimilarityCandidate,
    right: &SimilarityCandidate,
) -> std::cmp::Ordering {
    right
        .embedding_score
        .total_cmp(&left.embedding_score)
        .then_with(|| left.left.cmp(&right.left))
        .then_with(|| left.right.cmp(&right.right))
}

#[cfg(feature = "spec-retrieval")]
fn add_similarity_candidate(
    candidates: &mut BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), SimilarityCandidate>,
    scope: &'static str,
    left: SimilarityEndpoint,
    right: SimilarityEndpoint,
    score: f32,
) {
    if left == right {
        return;
    }
    let (left, right) = if left <= right {
        (left, right)
    } else {
        (right, left)
    };
    let key = (left.clone(), right.clone());
    if let Some(existing) = candidates.get_mut(&key) {
        if !existing.scopes.contains(&scope) {
            existing.scopes.push(scope);
            existing.scopes.sort_unstable();
        }
        if score > existing.embedding_score {
            existing.embedding_score = score;
        }
        return;
    }
    candidates.insert(
        key,
        SimilarityCandidate {
            left,
            right,
            embedding_score: score,
            bge: None,
            bge_admission: None,
            #[cfg(feature = "nli-evidence")]
            nli: None,
            #[cfg(feature = "nli-evidence")]
            nli_admission: None,
            #[cfg(feature = "nli-evidence")]
            nli_token_cap: None,
            scopes: vec![scope],
        },
    );
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn scenario_text_for<'a>(
    texts: &'a BTreeMap<(String, String), String>,
    endpoint: &SimilarityEndpoint,
) -> Result<&'a str> {
    texts
        .get(&(endpoint.file.clone(), endpoint.title.clone()))
        .map(String::as_str)
        .with_context(|| format!("missing scenario text for '{}'", endpoint.title))
}

#[cfg(any(feature = "cross-encoder-reranker", feature = "nli-evidence"))]
fn reranker_tokens(text: &str) -> std::collections::BTreeSet<String> {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_lowercase)
        .filter(|token| !is_stub_structure_token(token))
        .collect()
}

#[cfg(feature = "spec-retrieval")]
fn delta_specs_path(change_dir: &Path, change_id: &str) -> Result<std::path::PathBuf> {
    let path = change_dir.join("specs");
    if !path.exists() {
        bail!("change `{change_id}` has no specs directory; create an active change delta first");
    }
    Ok(path)
}

#[cfg(feature = "spec-retrieval")]
fn validated_review_change_dir(root: &Path, change_id: &str) -> Result<std::path::PathBuf> {
    crate::state::resolve_change_dir(root, change_id)
        .with_context(|| format!("review change id `{change_id}` must be a kebab-case identifier"))
}

#[cfg(feature = "spec-retrieval")]
fn read_index(path: &Path) -> Result<SpecIndex> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read index {}", path.display()))?;
    let index: SpecIndex = serde_json::from_str(&content)
        .with_context(|| format!("failed to parse index {}", path.display()))?;
    if index.version != INDEX_VERSION || index.model != MODEL_ID {
        bail!("spec index format or model version is incompatible; rebuild it");
    }
    validate_index(&index)?;
    Ok(index)
}

#[cfg(feature = "spec-retrieval")]
fn validate_index(index: &SpecIndex) -> Result<()> {
    let mut identities = BTreeSet::new();
    let mut cap_identities = BTreeSet::new();
    let mut dimensions = None;
    for entry in &index.entries {
        if entry.title.trim().is_empty()
            || entry.content_hash.trim().is_empty()
            || !entry.file.starts_with("givn/specs/")
            || std::path::Path::new(&entry.file)
                .components()
                .any(|component| component == std::path::Component::ParentDir)
        {
            bail!("spec index contains an invalid permanent entry; rebuild it");
        }
        if !identities.insert((entry.file.as_str(), entry.title.as_str())) {
            bail!("spec index contains duplicate permanent scenario identities; rebuild it");
        }
        if entry.embedding.is_empty() || entry.embedding.iter().any(|value| !value.is_finite()) {
            bail!("spec index contains an invalid embedding vector; rebuild it");
        }
        match dimensions {
            Some(expected) if expected != entry.embedding.len() => {
                bail!("spec index contains vectors with inconsistent dimensions; rebuild it")
            }
            None => dimensions = Some(entry.embedding.len()),
            _ => {}
        }
    }
    for cap in &index.e5_token_caps {
        if cap.status != "E5_TOKEN_CAP"
            || cap.title.trim().is_empty()
            || cap.content_hash.trim().is_empty()
            || !cap.file.starts_with("givn/specs/")
            || std::path::Path::new(&cap.file)
                .components()
                .any(|component| component == std::path::Component::ParentDir)
            || cap.token_count <= cap.limit
            || cap.limit != E5_MAX_LENGTH
            || cap.model != MODEL_ID
            || cap.tokenizer != E5_TOKENIZER_ID
            || !cap.refactoring_required
            || cap.guidance.trim().is_empty()
        {
            bail!("spec index contains invalid E5 token-cap metadata; rebuild it");
        }
        if !cap_identities.insert((cap.file.as_str(), cap.title.as_str())) {
            bail!("spec index contains duplicate E5 token-cap identities; rebuild it");
        }
        if identities.contains(&(cap.file.as_str(), cap.title.as_str())) {
            bail!("spec index contains both a vector and E5 token-cap metadata for one scenario");
        }
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn write_index(path: &Path, index: &SpecIndex) -> Result<()> {
    let content = serde_json::to_string_pretty(index).context("serialize spec index")?;
    let temp_path = path.with_extension("json.tmp");
    fs::write(&temp_path, content)
        .with_context(|| format!("write temporary index {}", temp_path.display()))?;
    fs::rename(&temp_path, path)
        .with_context(|| format!("replace spec index {}", path.display()))?;
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn write_semantic_report(path: &Path, content: &str) -> Result<()> {
    let temp_path = path.with_extension("md.tmp");
    fs::write(&temp_path, content)
        .with_context(|| format!("write temporary semantic report {}", temp_path.display()))?;
    fs::rename(&temp_path, path)
        .with_context(|| format!("replace semantic report {}", path.display()))?;
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn content_hash(text: &str) -> String {
    format!("{:016x}", stable_hash(text.as_bytes()))
}

#[cfg(feature = "spec-retrieval")]
fn reusable_entry(index: &SpecIndex, scenario: &ScenarioText) -> Option<IndexEntry> {
    index
        .entries
        .iter()
        .find(|entry| {
            entry.title == scenario.title
                && entry.file == scenario.file
                && entry.content_hash == scenario.content_hash
        })
        .cloned()
}

#[cfg(feature = "spec-retrieval")]
fn reusable_e5_token_cap(index: &SpecIndex, scenario: &ScenarioText) -> Option<E5TokenCap> {
    index
        .e5_token_caps
        .iter()
        .find(|cap| {
            cap.file == scenario.file
                && cap.title == scenario.title
                && cap.content_hash == scenario.content_hash
        })
        .cloned()
}

#[cfg(feature = "spec-retrieval")]
fn e5_token_cap(scenario: &ScenarioText, budget: E5TokenBudget) -> E5TokenCap {
    e5_token_cap_for_identity(
        &scenario.file,
        &scenario.title,
        &scenario.content_hash,
        budget,
    )
}

#[cfg(feature = "spec-retrieval")]
fn e5_token_cap_for_query(query: &ExploreQuery, budget: E5TokenBudget) -> E5TokenCap {
    e5_token_cap_for_identity(
        query.file.as_deref().unwrap_or("<query>"),
        &query.title,
        &query.content_hash,
        budget,
    )
}

#[cfg(feature = "spec-retrieval")]
fn e5_token_cap_for_identity(
    file: &str,
    title: &str,
    content_hash: &str,
    budget: E5TokenBudget,
) -> E5TokenCap {
    E5TokenCap {
        status: "E5_TOKEN_CAP".to_string(),
        file: file.to_string(),
        title: title.to_string(),
        content_hash: content_hash.to_string(),
        token_count: budget.token_count,
        limit: budget.limit,
        model: MODEL_ID.to_string(),
        tokenizer: E5_TOKENIZER_ID.to_string(),
        refactoring_required: true,
        guidance: E5_TOKEN_CAP_GUIDANCE.to_string(),
    }
}

#[cfg(feature = "spec-retrieval")]
fn e5_token_cap_order(left: &E5TokenCap, right: &E5TokenCap) -> std::cmp::Ordering {
    left.file
        .cmp(&right.file)
        .then_with(|| left.title.cmp(&right.title))
        .then_with(|| left.content_hash.cmp(&right.content_hash))
}

#[cfg(feature = "spec-retrieval")]
fn embed_texts(root: &Path, texts: &[String], role: EmbeddingRole) -> Result<EmbeddingBatch> {
    embed_texts_with_policy(root, texts, role, true, None)
}

#[cfg(feature = "spec-retrieval")]
fn embed_texts_with_policy(
    root: &Path,
    texts: &[String],
    role: EmbeddingRole,
    allow_download: bool,
    progress: Option<&mut dyn FnMut(usize, usize)>,
) -> Result<EmbeddingBatch> {
    notice("preparing retrieval embedder");
    let use_real_embedder = matches!(
        spec_env_var("GIVN_SPEC_EMBEDDER"),
        Err(std::env::VarError::NotPresent)
    );
    let mut model = if use_real_embedder {
        if allow_download {
            auto_e5_artifacts(root)?;
        } else {
            ensure_e5_cache_available(root)?;
        }
        Some(fastembed_model(root)?)
    } else {
        None
    };
    let token_caps = match spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() {
        Ok("stub") => {
            let caps = preflight_e5_stub_texts(texts)?;
            record_download_event("token-budget-measured")?;
            caps
        }
        Ok("download-stub") => {
            auto_e5_artifacts(root)?;
            let caps = preflight_e5_stub_texts(texts)?;
            record_download_event("token-budget-measured")?;
            caps
        }
        Ok("missing") => {
            let cache = e5_model_cache_dir(root)
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "<unresolved>".to_string());
            bail!(
                "retrieval model is unavailable; repository={E5_MODEL_REPOSITORY}; E5 cache={cache}; failed artifact=model.onnx or tokenizer.json; configure GIVN_SPEC_MODEL_CACHE or set GIVN_SPEC_E5_TOKENIZER, or unset GIVN_SPEC_EMBEDDER"
            )
        }
        Ok(value) => bail!("unknown GIVN_SPEC_EMBEDDER value `{value}`"),
        Err(std::env::VarError::NotPresent) => preflight_e5_texts(root, texts, role)?,
        Err(std::env::VarError::NotUnicode(_)) => {
            bail!("GIVN_SPEC_EMBEDDER is not valid UTF-8")
        }
    };
    let fitting_indices: Vec<usize> = token_caps
        .iter()
        .enumerate()
        .filter_map(|(index, cap)| cap.is_none().then_some(index))
        .collect();
    let fitting_texts: Vec<String> = fitting_indices
        .iter()
        .map(|index| texts[*index].clone())
        .collect();
    let vectors = match spec_env_var("GIVN_SPEC_EMBEDDER").as_deref() {
        Ok("stub") => fitting_texts
            .iter()
            .map(|text| stub_embedding(text, role))
            .collect(),
        Ok("download-stub") => fitting_texts
            .iter()
            .map(|text| stub_embedding(text, role))
            .collect(),
        Ok("missing") => unreachable!("missing embedder handled by preflight"),
        Ok(value) => bail!("unknown GIVN_SPEC_EMBEDDER value `{value}`"),
        Err(std::env::VarError::NotPresent) => {
            let model = model.as_mut().expect("real embedder model initialized");
            fastembed_texts_with_model(model, &fitting_texts, role)?
        }
        Err(std::env::VarError::NotUnicode(_)) => {
            bail!("GIVN_SPEC_EMBEDDER is not valid UTF-8")
        }
    };
    anyhow::ensure!(
        vectors.len() == fitting_indices.len(),
        "retrieval embedder returned {} vectors for {} fitting inputs",
        vectors.len(),
        fitting_indices.len()
    );
    let mut mapped = vec![None; texts.len()];
    for (index, vector) in fitting_indices.into_iter().zip(vectors) {
        mapped[index] = Some(vector);
    }
    if let Some(progress) = progress {
        for index in 0..texts.len() {
            progress(index + 1, texts.len());
        }
    }
    Ok(EmbeddingBatch {
        vectors: mapped,
        token_caps,
    })
}

#[cfg(feature = "spec-retrieval")]
fn preflight_e5_stub_texts(texts: &[String]) -> Result<Vec<Option<E5TokenBudget>>> {
    let Some(fixture) = spec_env("GIVN_SPEC_E5_TOKEN_BUDGET") else {
        return Ok(vec![None; texts.len()]);
    };
    let fixture = fixture
        .to_str()
        .context("GIVN_SPEC_E5_TOKEN_BUDGET is not valid UTF-8")?;
    if fixture == "active-550" {
        return Ok(texts
            .iter()
            .map(|text| {
                text.to_ascii_lowercase()
                    .contains("active")
                    .then_some(E5TokenBudget {
                        token_count: 550,
                        limit: E5_MAX_LENGTH,
                    })
            })
            .collect());
    }
    fixture_e5_token_budgets(fixture, texts.len())
}

#[cfg(feature = "spec-retrieval")]
fn fixture_e5_token_budgets(
    fixture: &str,
    text_count: usize,
) -> Result<Vec<Option<E5TokenBudget>>> {
    let counts = match fixture {
        "exact-512" | "bounded-512" | "all-512" => vec![E5_MAX_LENGTH; text_count],
        "overflow-513" | "all-513" => vec![513; text_count],
        "overflow-550" | "all-550" => vec![550; text_count],
        value if value.starts_with("partial-") => {
            let values = value
                .strip_prefix("partial-")
                .unwrap_or_default()
                .split('-')
                .map(|token| {
                    token
                        .parse::<usize>()
                        .with_context(|| format!("unknown E5 token budget fixture `{fixture}`"))
                })
                .collect::<Result<Vec<_>>>()?;
            anyhow::ensure!(
                !values.is_empty(),
                "unknown E5 token budget fixture `{fixture}`"
            );
            (0..text_count)
                .map(|index| values[index.min(values.len() - 1)])
                .collect()
        }
        _ => bail!(
            "unknown E5 token budget fixture `{fixture}`; expected exact-512, overflow-513, or partial-*"
        ),
    };
    Ok(counts
        .into_iter()
        .map(|token_count| {
            (token_count > E5_MAX_LENGTH).then_some(E5TokenBudget {
                token_count,
                limit: E5_MAX_LENGTH,
            })
        })
        .collect())
}

#[cfg(feature = "spec-retrieval")]
fn preflight_e5_texts(
    root: &Path,
    texts: &[String],
    role: EmbeddingRole,
) -> Result<Vec<Option<E5TokenBudget>>> {
    let tokenizer_path = spec_env("GIVN_SPEC_E5_TOKENIZER")
        .map(std::path::PathBuf::from)
        .or_else(|| find_e5_tokenizer(root));
    let Some(tokenizer_path) = tokenizer_path else {
        bail!(
            "retrieval tokenizer is unavailable; provide local E5 artifacts or set GIVN_SPEC_E5_TOKENIZER"
        );
    };
    let tokenizer = load_unbounded_tokenizer(&tokenizer_path)?;
    let prefix = match role {
        EmbeddingRole::Passage => "passage: ",
        EmbeddingRole::Query => "query: ",
    };
    let mut caps = Vec::with_capacity(texts.len());
    for text in texts {
        let count = tokenizer
            .encode(format!("{prefix}{text}"), true)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .len();
        caps.push((count > E5_MAX_LENGTH).then_some(E5TokenBudget {
            token_count: count,
            limit: E5_MAX_LENGTH,
        }));
    }
    record_download_event("token-budget-measured")?;
    Ok(caps)
}

#[cfg(feature = "spec-retrieval")]
fn find_e5_tokenizer(root: &Path) -> Option<std::path::PathBuf> {
    let cache_dir = e5_model_cache_dir(root).ok()?;
    let cache = hf_hub::Cache::new(cache_dir.clone());
    let repo = cache.repo(hf_hub::Repo::new(
        E5_MODEL_REPOSITORY.to_string(),
        hf_hub::RepoType::Model,
    ));
    repo.get("tokenizer.json").or_else(|| {
        spec_env("GIVN_SPEC_MODEL_CACHE")?;
        cache_dir
            .join("tokenizer.json")
            .is_file()
            .then(|| cache_dir.join("tokenizer.json"))
    })
}

#[cfg(feature = "spec-retrieval")]
fn e5_model_cache_dir(root: &Path) -> Result<std::path::PathBuf> {
    if let Some(path) = spec_env("GIVN_SPEC_MODEL_CACHE") {
        anyhow::ensure!(!path.is_empty(), "GIVN_SPEC_MODEL_CACHE must not be empty");
        return Ok(std::path::PathBuf::from(path));
    }
    model_cache_dir(root, "e5")
}

#[cfg(feature = "spec-retrieval")]
fn stub_embedding(text: &str, role: EmbeddingRole) -> Vec<f32> {
    const DIMENSIONS: usize = 256;
    let prefix = match role {
        EmbeddingRole::Passage => "passage",
        EmbeddingRole::Query => "query",
    };
    let mut vector = vec![0.0f32; DIMENSIONS];
    for token in prefix
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_lowercase)
        .filter(|token| !is_stub_structure_token(token))
    {
        let index = (stable_hash(token.as_bytes()) as usize) % DIMENSIONS;
        vector[index] += 1.0;
    }
    for line in text.lines() {
        let weight = if line.starts_with("scenario_description:") {
            5.0
        } else if line.starts_with("scenario:") || line.starts_with("step:") {
            3.0
        } else {
            1.0
        };
        for token in line
            .split(|character: char| !character.is_alphanumeric())
            .filter(|token| !token.is_empty())
            .map(str::to_lowercase)
            .filter(|token| !is_stub_structure_token(token))
        {
            let index = (stable_hash(token.as_bytes()) as usize) % DIMENSIONS;
            vector[index] += weight;
        }
    }
    normalize(vector)
}

#[cfg(feature = "spec-retrieval")]
fn is_stub_structure_token(token: &str) -> bool {
    matches!(
        token,
        "passage"
            | "query"
            | "feature"
            | "description"
            | "background"
            | "rule"
            | "scenario"
            | "outline"
            | "steps"
            | "step"
            | "role"
            | "text"
            | "table"
            | "docstring"
            | "examples"
            | "row"
            | "keyword"
            | "name"
            | "given"
            | "when"
            | "then"
            | "and"
            | "but"
    )
}

#[cfg(feature = "spec-retrieval")]
fn fastembed_model(root: &Path) -> Result<fastembed::TextEmbedding> {
    let cache_dir = e5_model_cache_dir(root)?;
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("create retrieval model cache {}", cache_dir.display()))?;
    let options = fastembed::TextInitOptions::new(fastembed::EmbeddingModel::MultilingualE5Large)
        .with_cache_dir(cache_dir)
        .with_show_download_progress(false)
        .with_intra_threads(resolve_thread_budget()?);
    fastembed::TextEmbedding::try_new(options).context(
        "retrieval model is unavailable; check the local E5 model cache, network, and GIVN_SPEC_MODEL_CACHE",
    )
}

#[cfg(feature = "spec-retrieval")]
fn fastembed_texts_with_model(
    model: &mut fastembed::TextEmbedding,
    texts: &[String],
    role: EmbeddingRole,
) -> Result<Vec<Vec<f32>>> {
    let prefix = match role {
        EmbeddingRole::Passage => "passage: ",
        EmbeddingRole::Query => "query: ",
    };
    let input: Vec<String> = texts.iter().map(|text| format!("{prefix}{text}")).collect();
    model
        .embed(input, Some(32))
        .context("generate retrieval embeddings")
}

#[cfg(feature = "spec-retrieval")]
fn ensure_e5_cache_available(root: &Path) -> Result<()> {
    let cache_dir = e5_model_cache_dir(root)?;
    let cache = hf_hub::Cache::new(cache_dir.clone());
    let repo = cache.repo(hf_hub::Repo::new(
        E5_MODEL_REPOSITORY.to_string(),
        hf_hub::RepoType::Model,
    ));
    let required = [
        "model.onnx",
        "model.onnx_data",
        "tokenizer.json",
        "config.json",
        "special_tokens_map.json",
        "tokenizer_config.json",
    ];
    anyhow::ensure!(
        required.iter().all(|file| repo.get(file).is_some()),
        "model download is disabled for this operation; provide a complete E5 cache at {} or run `givn spec index` first",
        cache_dir.display()
    );
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn normalize(mut vector: Vec<f32>) -> Vec<f32> {
    let norm = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in &mut vector {
            *value /= norm;
        }
    }
    vector
}

#[cfg(feature = "spec-retrieval")]
fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    left.iter().zip(right).map(|(a, b)| a * b).sum::<f32>()
}

#[cfg(feature = "spec-retrieval")]
fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_pipeline_descriptor(max_unique_pairs: usize) -> String {
    format!(
        "model={RERANKER_MODEL_ID};top_k={RERANKER_TOP_K};threshold={RERANKER_LOGIT_THRESHOLD:.4};max_pairs={max_unique_pairs};exclusions=identity,deterministic;ordering=e5-desc,file,title;pair=canonical-undirected;directions=forward,reverse;score=symmetric-average;selection=ordered-prefix"
    )
}

#[cfg(feature = "spec-retrieval")]
fn rank_entries<'a>(entries: &'a [IndexEntry], query: &[f32]) -> Vec<(&'a IndexEntry, f32)> {
    let mut ranked: Vec<(&IndexEntry, f32)> = entries
        .iter()
        .map(|entry| (entry, cosine_similarity(query, &entry.embedding)))
        .collect();
    ranked.sort_by(|(left_entry, left_score), (right_entry, right_score)| {
        right_score
            .total_cmp(left_score)
            .then_with(|| left_entry.file.cmp(&right_entry.file))
            .then_with(|| left_entry.title.cmp(&right_entry.title))
    });
    ranked
}

#[cfg(feature = "spec-retrieval")]
const SEARCH_EVIDENCE_TOP_K: usize = 5;

#[cfg(feature = "spec-retrieval")]
fn print_search_results_with_evidence(
    ranked: &[(&IndexEntry, f32)],
    query_endpoint: &SimilarityEndpoint,
    bge_run: Option<&BgeRun>,
    #[cfg(feature = "nli-evidence")] nli_run: Option<&NliRun>,
) {
    for (rank, (entry, score)) in ranked.iter().take(SEARCH_EVIDENCE_TOP_K).enumerate() {
        println!(
            "rank {}: {} ({}, score {:.4})",
            rank + 1,
            entry.title,
            entry.file,
            score
        );
        if let Some(run) = bge_run {
            println!(
                "  {}",
                bge_candidate_line(run, query_endpoint, &entry.file, &entry.title)
            );
        }
        #[cfg(feature = "nli-evidence")]
        if let Some(run) = nli_run {
            println!(
                "  {}",
                nli_candidate_line(run, query_endpoint, &entry.file, &entry.title)
            );
        }
    }
    if bge_run.is_none() || bge_run.is_some_and(|run| run.status == "BGE_UNAVAILABLE") {
        println!("BGE evidence: unavailable");
    }
    #[cfg(feature = "nli-evidence")]
    if nli_run.is_none() || nli_run.is_some_and(|run| run.status == "NLI_UNAVAILABLE") {
        println!("NLI evidence: unavailable");
    }
    #[cfg(not(feature = "nli-evidence"))]
    println!("NLI evidence: unavailable");
}

#[cfg(feature = "spec-retrieval")]
fn print_search_results(ranked: &[(&IndexEntry, f32)]) {
    for (rank, (entry, score)) in ranked.iter().take(SEARCH_EVIDENCE_TOP_K).enumerate() {
        println!(
            "rank {}: {} ({}, score {:.4})",
            rank + 1,
            entry.title,
            entry.file,
            score
        );
    }
}

#[cfg(feature = "spec-retrieval")]
fn bge_candidate_line(
    run: &BgeRun,
    query_endpoint: &SimilarityEndpoint,
    file: &str,
    title: &str,
) -> String {
    let key = canonical_pair_key(query_endpoint.clone(), endpoint(file, title));
    if let Some(evidence) = run.evidence.get(&key) {
        let status = if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
            "BGE_RECOMMENDED"
        } else {
            "BGE_FILTERED"
        };
        format!(
            "BGE: {status} (symmetric raw {:.4}, forward {:.4}, reverse {:.4})",
            evidence.symmetric_raw, evidence.forward_raw, evidence.reverse_raw
        )
    } else if let Some(budget) = run.token_caps.get(&key) {
        format!(
            "BGE: BGE_TOKEN_CAP (pair {} tokens, left {}, right {}, limit {})",
            budget.pair_tokens, budget.left_tokens, budget.right_tokens, budget.limit
        )
    } else {
        "BGE: BGE_UNSCORED".to_string()
    }
}

#[cfg(all(test, feature = "spec-retrieval"))]
mod tests {
    use super::*;

    #[test]
    fn model_cache_resolution_prefers_explicit_hf_then_xdg_then_home() {
        let hf = model_cache_dir_from_values(
            "e5",
            Some(Path::new("/srv/huggingface").as_os_str()),
            Some(Path::new("/srv/xdg").as_os_str()),
            Some(Path::new("/home/tester").as_os_str()),
        )
        .unwrap();
        assert_eq!(hf, Path::new("/srv/huggingface"));

        let xdg = model_cache_dir_from_values(
            "bge",
            None,
            Some(Path::new("/srv/xdg").as_os_str()),
            Some(Path::new("/home/tester").as_os_str()),
        )
        .unwrap();
        assert_eq!(xdg, Path::new("/srv/xdg/givn/bge"));

        let home = model_cache_dir_from_values(
            "nli",
            None,
            None,
            Some(Path::new("/home/tester").as_os_str()),
        )
        .unwrap();
        assert_eq!(home, Path::new("/home/tester/.cache/givn/nli"));
    }

    #[test]
    fn model_cache_resolution_rejects_relative_xdg_and_missing_home() {
        let relative = model_cache_dir_from_values(
            "e5",
            None,
            Some(Path::new("relative/cache").as_os_str()),
            Some(Path::new("/home/tester").as_os_str()),
        );
        assert!(relative.is_err());

        let missing_home = model_cache_dir_from_values("e5", None, None, None);
        assert!(missing_home.is_err());
    }

    #[test]
    #[cfg(feature = "nli-evidence")]
    fn nli_evidence_averages_directional_logits_before_softmax() {
        let evidence = nli_evidence([4.0, 0.0, 0.0], [0.0, 4.0, 0.0]);

        assert!(evidence.forward[0] > 0.9);
        assert!(evidence.reverse[1] > 0.9);
        assert!(evidence.symmetric[0] > 0.45);
        assert!(evidence.symmetric[1] > 0.45);
        assert!((evidence.symmetric.iter().sum::<f32>() - 1.0).abs() < 1e-5);
    }

    #[test]
    #[cfg(feature = "nli-evidence")]
    fn nli_labels_are_read_by_name_not_position() {
        let config = serde_json::json!({
            "id2label": {
                "0": "contradiction",
                "1": "entailment",
                "2": "neutral"
            }
        });

        assert_eq!(nli_label_indices(&config).unwrap(), [1, 2, 0]);
    }

    #[test]
    fn duplicate_boundary_passage_scores_are_below_the_similarity_threshold() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let delta_root = root.join("givn/specs/spec-duplicates");
        let delta = collect_delta_scenarios(root, &delta_root).expect("load duplicate delta");
        let active: Vec<&DeltaScenario> = delta
            .iter()
            .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
            .collect();
        let vectors: Vec<Vec<f32>> = active
            .iter()
            .map(|item| stub_embedding(&item.scenario.text, EmbeddingRole::Passage))
            .collect();
        let pairs = [
            (
                "A malformed permanent feature reports its path",
                "An unreadable permanent feature reports its path",
            ),
            (
                "A malformed persistent index is ignored and unchanged",
                "An absent persistent index is ignored and remains absent",
            ),
            (
                "A semantic candidate at the inclusive threshold is reported",
                "A semantic pair below the threshold is not reported",
            ),
            (
                "Auditing leaves active changes and archive contents unchanged",
                "Auditing leaves permanent specifications unchanged",
            ),
            (
                "Auditing leaves permanent specifications unchanged",
                "Auditing leaves the persistent retrieval index unchanged",
            ),
            (
                "Exact duplicates across features are reported together",
                "Exact duplicates in one feature are reported together",
            ),
            (
                "The audit reports an unavailable retrieval model",
                "The audit reports retrieval unavailable without the retrieval feature",
            ),
        ];

        for (left_title, right_title) in pairs {
            let left = active
                .iter()
                .position(|item| item.scenario.title == left_title)
                .expect("left boundary scenario");
            let right = active
                .iter()
                .position(|item| item.scenario.title == right_title)
                .expect("right boundary scenario");
            let score = cosine_similarity(&vectors[left], &vectors[right]);
            println!("{left_title} <> {right_title}: {score:.4}");
            assert!(
                score < E5_ADMISSION_THRESHOLD,
                "boundary pair remained above threshold: {left_title} <> {right_title} = {score:.4}"
            );
        }
    }

    #[test]
    fn compact_serializer_omits_empty_optional_sections_and_fields() {
        let feature = gherkin::Feature::parse(
            "Feature: compact retrieval\n\n  Scenario: Minimal scenario\n    Given minimal context\n    When minimal action runs\n    Then minimal result appears\n",
            gherkin::GherkinEnv::default(),
        )
        .expect("parse minimal compact fixture");

        let text = semantic_serialization(&feature, None, &feature.scenarios[0]);

        assert_eq!(
            text,
            "feature: Feature compact retrieval\nscenario: Scenario Minimal scenario\nsteps:\nstep: role=Given; text=minimal context\nstep: role=When; text=minimal action runs\nstep: role=Then; text=minimal result appears"
        );
        for marker in [
            "[]",
            "feature_description:",
            "feature_background:",
            "rule:",
            "rule_description:",
            "rule_background:",
            "scenario_description:",
            "outline_template:",
            "examples:",
            "table=",
            "docstring=",
        ] {
            assert!(!text.contains(marker), "unexpected empty marker {marker:?}");
        }
    }

    #[test]
    fn compact_serializer_retains_non_empty_optional_content() {
        let feature = gherkin::Feature::parse(
            "Feature: rich retrieval\n  Feature description for semantic retrieval.\n\n  Background: feature setup\n    Given feature background is active\n\n  Rule: provider boundaries\n    Rule description for semantic retrieval.\n\n    Background: rule setup\n      Given rule background is active\n\n    Scenario Outline: Rich scenario\n      Given a scenario context\n        | header | value |\n        | input | first |\n      And the scenario includes documentation\n        \"\"\"\n        scenario docstring\n        \"\"\"\n      When the rich action runs\n      Then the rich result appears\n\n      Examples: rich example\n        | variant |\n        | first |\n",
            gherkin::GherkinEnv::default(),
        )
        .expect("parse rich compact fixture");
        let rule = &feature.rules[0];
        let scenario = &rule.scenarios[0];
        let text = semantic_serialization(&feature, Some(rule), scenario);

        for content in [
            "feature_description: \"Feature description for semantic retrieval.\"",
            "feature background is active",
            "rule: Rule provider boundaries",
            "rule_description: \"Rule description for semantic retrieval.\"",
            "rule background is active",
            "table=[[\"header\",\"value\"],[\"input\",\"first\"]]",
            "docstring=\"scenario docstring\"",
            "outline_template: Rich scenario",
            "examples_row: keyword=Examples name=\"rich example\" table=[[\"variant\"],[\"first\"]]",
        ] {
            assert!(
                text.contains(content),
                "missing retained content {content:?}: {text}"
            );
        }
        assert!(!text.contains("[]"));
        assert_eq!(semantic_optional_text(Some(" \n\t")), None);
    }
}

#[cfg(feature = "nli-evidence")]
fn nli_candidate_line(
    run: &NliRun,
    query_endpoint: &SimilarityEndpoint,
    file: &str,
    title: &str,
) -> String {
    let key = canonical_pair_key(query_endpoint.clone(), endpoint(file, title));
    if let Some(evidence) = run.evidence.get(&key) {
        let status = if evidence.symmetric[0] >= NLI_ADMISSION_THRESHOLD {
            "NLI_RECOMMENDED"
        } else {
            "NLI_FILTERED"
        };
        format!(
            "NLI: {status} (entailment {:.4}, neutral {:.4}, contradiction {:.4})",
            evidence.symmetric[0], evidence.symmetric[1], evidence.symmetric[2]
        )
    } else if let Some(budget) = run.token_caps.get(&key) {
        format!(
            "NLI: NLI_TOKEN_CAP (pair {} tokens, left {}, right {}, limit {})",
            budget.pair_tokens, budget.left_tokens, budget.right_tokens, budget.limit
        )
    } else {
        "NLI: NLI_UNSCORED".to_string()
    }
}

#[cfg(all(test, feature = "spec-retrieval"))]
mod release_strict_channel_completeness {
    use super::REQUEST_CHANNELS;
    use std::collections::BTreeSet;

    /// Channels the semantic review reads outside the fixture-request policy:
    /// the compute-thread budget (no verdict influence) and the model-location
    /// requests that stay honoured for air-gapped `local` mode — the
    /// documented accepted exception. An unusable artifact set fails the run;
    /// it never fakes a verdict.
    const EXEMPT_CHANNELS: &[&str] = &[
        "GIVN_SPEC_THREADS",
        "GIVN_SPEC_MODEL_CACHE",
        "GIVN_SPEC_E5_TOKENIZER",
        "GIVN_SPEC_BGE_MODEL_DIR",
        "GIVN_SPEC_BGE_CACHE",
        "GIVN_SPEC_NLI_MODEL_DIR",
        "GIVN_SPEC_NLI_CACHE",
        // Test-only probe of the gated read helpers; never a real channel.
        "GIVN_SPEC_RELEASE_STRICT_UNSET_PROBE",
    ];

    /// Extract every `GIVN_SPEC_*` string literal from one source file.
    fn scan_source(source: &str, channels: &mut BTreeSet<String>) {
        let mut rest = source;
        while let Some(position) = rest.find("GIVN_SPEC_") {
            let tail = &rest[position..];
            let end = tail
                .find(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
                .unwrap_or(tail.len());
            // A bare prefix (e.g. inside prose like "the GIVN_SPEC_* channels")
            // names no channel.
            if end > "GIVN_SPEC_".len() {
                channels.insert(tail[..end].to_string());
            }
            rest = &tail[end..];
        }
    }

    /// Walk every Rust source file under `src/` so the net covers channels
    /// read outside this module too — the registry must be complete over the
    /// whole binary, not over one file.
    fn source_channels() -> BTreeSet<String> {
        let mut channels = BTreeSet::new();
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        visit_sources(&src, &mut channels);
        channels
    }

    fn visit_sources(dir: &std::path::Path, channels: &mut BTreeSet<String>) {
        for entry in std::fs::read_dir(dir).expect("walk src directory") {
            let path = entry.expect("src directory entry").path();
            if path.is_dir() {
                visit_sources(&path, channels);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let source = std::fs::read_to_string(&path)
                    .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
                scan_source(&source, channels);
            }
        }
    }

    /// The drift net: every environment channel the semantic review reads is
    /// either policy-governed (`REQUEST_CHANNELS`) or explicitly exempt. A
    /// new channel without a policy entry fails this test on both profiles.
    #[test]
    fn every_read_channel_is_registered_or_exempt() {
        let channels = source_channels();
        let registered: BTreeSet<&str> = REQUEST_CHANNELS.iter().copied().collect();
        let exempt: BTreeSet<&str> = EXEMPT_CHANNELS.iter().copied().collect();
        for channel in &channels {
            assert!(
                registered.contains(channel.as_str()) || exempt.contains(channel.as_str()),
                "channel {channel} is read in the source but is neither in REQUEST_CHANNELS nor EXEMPT_CHANNELS — an installed build could read it ungated"
            );
        }
        for name in &registered {
            assert!(
                channels.contains(*name),
                "channel {name} is registered in REQUEST_CHANNELS but no longer read in the source — drop it from the registry"
            );
        }
    }
}

#[cfg(all(test, feature = "spec-retrieval"))]
mod release_strict_tests {
    use super::{ALLOW_SPEC_OVERRIDES, release_blocked, spec_env, spec_env_var};
    use std::env;

    /// std::env mutation is process-global, so the env-mutating tests below
    /// run serially against each other.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn set_var(name: &str, value: &str) {
        // Rust 2024 marks the std::env mutators unsafe; the test owns the
        // (single-threaded, serialized) process env for the variables it uses.
        unsafe {
            env::set_var(name, value);
        }
    }

    fn remove_var(name: &str) {
        unsafe {
            env::remove_var(name);
        }
    }

    /// A representative fixture request for every gated channel, exercised
    /// against `release_blocked` on both profiles so the sweep list and the
    /// taxonomy cannot drift from each other.
    const REQUEST_REPRESENTATIVE: &[(&str, &str)] = &[
        ("GIVN_SPEC_EMBEDDER", "stub"),
        ("GIVN_SPEC_BGE", "stub"),
        ("GIVN_SPEC_RERANKER", "nli-stub"),
        ("GIVN_SPEC_BGE_APPROVAL", "failed"),
        ("GIVN_SPEC_BGE_LOCAL_STUB", "1"),
        ("GIVN_SPEC_NLI_LOCAL_STUB", "1"),
        ("GIVN_SPEC_BGE_TEST_POOL", "pool"),
        ("GIVN_SPEC_NLI_TEST_POOL", "pool"),
        ("GIVN_SPEC_BGE_DIRECTIONAL_SCORES", "scores"),
        ("GIVN_SPEC_NLI_DIRECTIONAL_SCORES", "scores"),
        ("GIVN_SPEC_STUB_PAIR_SCORES", "scores"),
        ("GIVN_SPEC_DUPLICATES_TEST_SCORES", "scores"),
        ("GIVN_SPEC_E5_TOKEN_BUDGET", "budget"),
        ("GIVN_SPEC_BGE_TOKEN_BUDGET", "budget"),
        ("GIVN_SPEC_NLI_TOKEN_BUDGET", "budget"),
        ("GIVN_SPEC_TOKEN_BUDGET", "budget"),
        ("GIVN_SPEC_NLI_REPORT_LIMIT", "5"),
        ("GIVN_SPEC_MAX_RERANK_PAIRS", "64"),
        ("GIVN_SPEC_RESOLUTION_NOTICES", "repeat"),
        ("GIVN_SPEC_DOWNLOAD_EVENTS", "/events.log"),
    ];

    #[cfg(not(debug_assertions))]
    mod release {
        use super::*;
        use crate::spec::retrieval::REQUEST_CHANNELS;
        use std::env::{VarError, var};

        #[test]
        fn gates_only_disqualifying_selector_values() {
            for (name, value) in [
                ("GIVN_SPEC_BGE", "auto"),
                ("GIVN_SPEC_BGE", "local"),
                ("GIVN_SPEC_RERANKER", "nli"),
                ("GIVN_SPEC_RERANKER", "nli-auto"),
            ] {
                assert!(
                    !release_blocked(name, value),
                    "{name}={value} must stay honoured"
                );
            }
        }

        #[test]
        fn gates_every_fixture_request() {
            for &(name, value) in REQUEST_REPRESENTATIVE {
                assert!(
                    release_blocked(name, value),
                    "{name}={value} must be ignored in an installed build"
                );
            }
        }

        #[test]
        fn location_channels_are_honoured() {
            for (name, value) in [
                ("GIVN_SPEC_MODEL_CACHE", "/models/e5"),
                ("GIVN_SPEC_E5_TOKENIZER", "/models/tokenizer.json"),
                ("GIVN_SPEC_BGE_CACHE", "/models/bge"),
                ("GIVN_SPEC_BGE_MODEL_DIR", "/models/bge/dir"),
                ("GIVN_SPEC_NLI_CACHE", "/models/nli"),
                ("GIVN_SPEC_NLI_MODEL_DIR", "/models/nli/dir"),
                ("GIVN_SPEC_THREADS", "4"),
            ] {
                assert!(
                    !release_blocked(name, value),
                    "{name}={value} is a model-location request and must be honoured"
                );
            }
        }

        #[test]
        fn sweep_list_fully_gated() {
            let mut channels: Vec<&str> = REQUEST_CHANNELS.iter().copied().collect();
            channels.sort_unstable();
            let mut representatives: Vec<&str> = REQUEST_REPRESENTATIVE
                .iter()
                .map(|(name, _)| *name)
                .collect();
            representatives.sort_unstable();
            assert_eq!(
                channels, representatives,
                "REQUEST_CHANNELS and the test representative list must match"
            );
            for &(name, value) in REQUEST_REPRESENTATIVE {
                assert!(release_blocked(name, value), "{name} must be gated");
            }
        }

        #[test]
        fn spec_env_maps_blocked_request_to_none() {
            let _guard = ENV_LOCK.lock().unwrap();
            set_var("GIVN_SPEC_EMBEDDER", "stub");
            assert!(spec_env("GIVN_SPEC_EMBEDDER").is_none());
            assert_eq!(
                var("GIVN_SPEC_EMBEDDER").ok(),
                Some("stub".to_string()),
                "the raw variable is untouched"
            );
            remove_var("GIVN_SPEC_EMBEDDER");
        }

        #[test]
        fn spec_env_var_maps_blocked_request_to_not_present() {
            let _guard = ENV_LOCK.lock().unwrap();
            set_var("GIVN_SPEC_BGE", "disabled");
            assert_eq!(
                spec_env_var("GIVN_SPEC_BGE"),
                Err(VarError::NotPresent),
                "a blocked request must resolve like an unset variable"
            );
            remove_var("GIVN_SPEC_BGE");
        }

        #[test]
        fn spec_env_var_maps_location_request_to_value() {
            let _guard = ENV_LOCK.lock().unwrap();
            set_var("GIVN_SPEC_BGE", "auto");
            assert_eq!(spec_env_var("GIVN_SPEC_BGE"), Ok("auto".to_string()));
            remove_var("GIVN_SPEC_BGE");
        }

        #[test]
        fn spec_env_var_unset_stays_not_present() {
            let _guard = ENV_LOCK.lock().unwrap();
            remove_var("GIVN_SPEC_RELEASE_STRICT_UNSET_PROBE");
            assert_eq!(
                spec_env_var("GIVN_SPEC_RELEASE_STRICT_UNSET_PROBE"),
                Err(VarError::NotPresent)
            );
            assert!(spec_env("GIVN_SPEC_RELEASE_STRICT_UNSET_PROBE").is_none());
        }

        #[test]
        #[allow(clippy::assertions_on_constants)]
        fn release_profile_really_disallows_overrides() {
            // A cfg-known value, asserted so a mis-configured runner that
            // executes the release matrix under a debug profile fails loudly.
            assert!(!ALLOW_SPEC_OVERRIDES);
        }
    }

    #[cfg(debug_assertions)]
    mod development {
        use super::*;
        use std::env::VarError;

        #[test]
        fn dev_profile_honours_every_fixture_request() {
            for &(name, value) in REQUEST_REPRESENTATIVE {
                assert!(
                    !release_blocked(name, value),
                    "{name}={value} must be honoured by a development build"
                );
            }
        }

        #[test]
        fn spec_env_dev_passthrough() {
            let _guard = ENV_LOCK.lock().unwrap();
            set_var("GIVN_SPEC_EMBEDDER", "stub");
            assert_eq!(spec_env_var("GIVN_SPEC_EMBEDDER"), Ok("stub".to_string()));
            assert_eq!(
                spec_env("GIVN_SPEC_EMBEDDER"),
                Some(std::ffi::OsString::from("stub"))
            );
            remove_var("GIVN_SPEC_EMBEDDER");
        }

        #[test]
        fn spec_env_dev_unset_returns_not_present() {
            let _guard = ENV_LOCK.lock().unwrap();
            remove_var("GIVN_SPEC_RELEASE_STRICT_UNSET_PROBE");
            assert_eq!(
                spec_env_var("GIVN_SPEC_RELEASE_STRICT_UNSET_PROBE"),
                Err(VarError::NotPresent)
            );
        }

        #[test]
        #[allow(clippy::assertions_on_constants)]
        fn dev_profile_allows_overrides() {
            // A cfg-known value, asserted so a mis-configured runner that
            // executes the dev passthrough under a release profile fails loudly.
            assert!(ALLOW_SPEC_OVERRIDES);
        }
    }
}
