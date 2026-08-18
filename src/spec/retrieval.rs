use std::path::Path;

use anyhow::{Result, bail};

#[cfg(feature = "spec-retrieval")]
use anyhow::Context;

#[cfg(feature = "spec-retrieval")]
use ndarray::Array;

#[cfg(feature = "spec-retrieval")]
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "spec-retrieval")]
use std::fs;

#[cfg(feature = "spec-retrieval")]
use std::io::Read;

#[cfg(feature = "spec-retrieval")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "spec-retrieval")]
use tokenizers::{PaddingParams, PaddingStrategy, Tokenizer, TruncationParams};

#[cfg(feature = "spec-retrieval")]
const INDEX_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const MODEL_ID: &str = "intfloat/multilingual-e5-large";
#[cfg(feature = "spec-retrieval")]
const RERANKER_MODEL_ID: &str = "rozgo/bge-reranker-v2-m3";
#[cfg(all(feature = "spec-retrieval", not(feature = "cross-encoder-reranker")))]
const SIMILARITY_THRESHOLD: f32 = 0.95;
#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
const RERANKER_LOGIT_THRESHOLD: f32 = 3.0;
#[cfg(feature = "spec-retrieval")]
const BGE_ADMISSION_THRESHOLD: f32 = 3.0;
#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
const BGE_PIPELINE_VERSION: &str = "rust-production-v1";
#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
const BGE_MAX_PAIRS: usize = 512;
#[cfg(feature = "spec-retrieval")]
const RERANKER_TOP_K: usize = 5;
#[cfg(feature = "spec-retrieval")]
const NLI_MODEL_ID: &str = "MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7";
#[cfg(feature = "spec-retrieval")]
const NLI_MAX_LENGTH: usize = 512;
#[cfg(feature = "spec-retrieval")]
const NLI_REPORT_LIMIT: usize = 20;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_REPORT_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_BGE_REPORT_SCHEMA_VERSION: u32 = 2;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_FINGERPRINT_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_EVIDENCE_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const SEMANTIC_SIMILARITY_THRESHOLD: f32 = 0.95;
#[cfg(feature = "spec-retrieval")]
const DUPLICATE_REPORT_SCHEMA_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const DUPLICATE_BGE_REPORT_SCHEMA_VERSION: u32 = 2;

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpecIndex {
    version: u32,
    model: String,
    entries: Vec<IndexEntry>,
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
    query: ExploreQueryOutput,
    index: ExploreIndexOutput,
    results: Vec<ExploreResultOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<ExploreEvidenceOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recommended_results: Option<Vec<ExploreResultOutput>>,
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
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct ExploreBgeOutput {
    status: &'static str,
    forward_raw: f32,
    reverse_raw: f32,
    symmetric_raw: f32,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct ExploreEvidenceOutput {
    status: &'static str,
    reason: Option<&'static str>,
    candidate_generator: &'static str,
    top_k: usize,
    e5_candidates: usize,
    recommended_candidates: usize,
    bge_filtered_candidates: usize,
    bge_unscored_candidates: usize,
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

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone)]
struct SimilarityCandidate {
    left: SimilarityEndpoint,
    right: SimilarityEndpoint,
    embedding_score: f32,
    rerank_score: f32,
    bge: Option<BgePairEvidence>,
    bge_admission: Option<&'static str>,
    nli: Option<NliEvidence>,
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
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug)]
struct DuplicateAudit {
    candidates: Vec<DuplicateCandidate>,
    filtered_candidates: Vec<DuplicateCandidate>,
    unscored_candidates: Vec<DuplicateCandidate>,
    bge_run: Option<BgeRun>,
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
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize)]
struct DuplicateJsonBgeEvidence {
    status: &'static str,
    forward_raw: f32,
    reverse_raw: f32,
    symmetric_raw: f32,
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
    e5_candidate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bge_run: Option<DuplicateJsonRun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filtered_candidates: Option<Vec<DuplicateJsonCandidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unscored_candidates: Option<Vec<DuplicateJsonCandidate>>,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Serialize)]
struct DuplicateJsonRun {
    status: &'static str,
    reason: Option<&'static str>,
    model: Option<&'static str>,
    top_k: usize,
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

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy)]
enum NliBackend {
    Stub,
    Local,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy)]
struct NliEvidence {
    forward: [f32; 3],
    reverse: [f32; 3],
    symmetric: [f32; 3],
}

#[cfg(feature = "spec-retrieval")]
type CandidatePool = (
    Vec<SimilarityCandidate>,
    usize,
    BTreeMap<(String, String), String>,
);

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BgeBackendKind {
    Stub,
    Local,
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BgeUnavailableReason {
    MissingArtifact,
    PairCap,
}

#[cfg(feature = "spec-retrieval")]
impl BgeUnavailableReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::MissingArtifact => "missing-artifact",
            Self::PairCap => "pair-cap",
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
#[derive(Debug, Clone)]
struct BgeRun {
    status: &'static str,
    reason: Option<BgeUnavailableReason>,
    evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), BgePairEvidence>,
    e5_candidates: usize,
    unscored: usize,
    model_load_attempts: usize,
    runtime_ms: Option<u128>,
}

#[cfg(feature = "spec-retrieval")]
impl BgeRun {
    fn enabled(
        evidence: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), BgePairEvidence>,
        e5_candidates: usize,
        runtime_ms: u128,
    ) -> Self {
        Self {
            status: "BGE_ENABLED",
            reason: None,
            unscored: e5_candidates.saturating_sub(evidence.len()),
            evidence,
            e5_candidates,
            model_load_attempts: 1,
            runtime_ms: Some(runtime_ms),
        }
    }

    fn unavailable(reason: BgeUnavailableReason, e5_candidates: usize) -> Self {
        Self {
            status: "BGE_UNAVAILABLE",
            reason: Some(reason),
            evidence: BTreeMap::new(),
            e5_candidates,
            unscored: e5_candidates,
            model_load_attempts: 0,
            runtime_ms: None,
        }
    }
}

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Copy)]
enum EmbeddingRole {
    Passage,
    Query,
}

pub fn index(root: &Path) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        index_enabled(root)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = root;
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

pub fn search(root: &Path, query: &str) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        search_enabled(root, query)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, query);
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

pub fn review(root: &Path, change_id: &str) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        review_enabled(root, change_id)
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, change_id);
        bail!(
            "spec retrieval is unavailable in this build; rebuild with --features spec-retrieval"
        );
    }
}

pub fn semantic_review(root: &Path, change_id: &str) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
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
        let descriptor = bge_pipeline_descriptor();
        let document = serde_json::json!({
            "schema_version": 1,
            "source": "givn spec bge-parity",
            "candidate_pipeline": BGE_PIPELINE_VERSION,
            "pipeline_fingerprint": format!("{:016x}", stable_hash(descriptor.as_bytes())),
            "descriptor": descriptor,
            "top_k": RERANKER_TOP_K,
            "threshold": RERANKER_LOGIT_THRESHOLD,
            "max_unique_pairs": BGE_MAX_PAIRS,
        });
        println!("{}", serde_json::to_string_pretty(&document)?);
        return Ok(());
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
fn index_enabled(root: &Path) -> Result<()> {
    let refresh = refresh_index(root)?;
    println!(
        "Indexed {} permanent scenarios",
        refresh.index.entries.len()
    );
    println!("Reused {} scenario vectors", refresh.reused);
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn duplicates_enabled(root: &Path, json: bool) -> Result<()> {
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
    let candidates = duplicate_candidates(root, scenarios)?;
    #[cfg(feature = "cross-encoder-reranker")]
    {
        return apply_duplicate_bge(root, scenarios, candidates);
    }
    #[cfg(not(feature = "cross-encoder-reranker"))]
    {
        Ok(DuplicateAudit {
            candidates,
            filtered_candidates: Vec::new(),
            unscored_candidates: Vec::new(),
            bge_run: None,
        })
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn apply_duplicate_bge(
    root: &Path,
    scenarios: &[DuplicateScenario],
    mut candidates: Vec<DuplicateCandidate>,
) -> Result<DuplicateAudit> {
    let semantic: Vec<DuplicateCandidate> = candidates
        .iter()
        .filter(|candidate| candidate.kind == "semantic_candidate")
        .cloned()
        .collect();
    if semantic.is_empty() {
        return Ok(DuplicateAudit {
            candidates,
            filtered_candidates: Vec::new(),
            unscored_candidates: Vec::new(),
            bge_run: None,
        });
    }
    let Some(backend) = bge_backend(root)? else {
        return Ok(DuplicateAudit {
            candidates,
            filtered_candidates: Vec::new(),
            unscored_candidates: Vec::new(),
            bge_run: None,
        });
    };

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

    if selected.len() > BGE_MAX_PAIRS {
        let run = BgeRun::unavailable(BgeUnavailableReason::PairCap, semantic.len());
        for candidate in &mut candidates {
            if candidate.kind == "semantic_candidate" {
                candidate.bge_admission = Some("BGE_UNSCORED");
            }
        }
        return Ok(DuplicateAudit {
            candidates,
            filtered_candidates: Vec::new(),
            unscored_candidates: semantic,
            bge_run: Some(run),
        });
    }

    let texts: BTreeMap<(String, String), String> = scenarios
        .iter()
        .map(|scenario| {
            (
                (scenario.endpoint.file.clone(), scenario.endpoint.title.clone()),
                scenario.text.clone(),
            )
        })
        .collect();
    let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = selected
        .iter()
        .map(|((left_file, left_title), (right_file, right_title))| {
            (endpoint(left_file, left_title), endpoint(right_file, right_title))
        })
        .collect();
    let mut run = match backend {
        Ok(kind) => score_bge_pairs(root, &pairs, &texts, kind)?,
        Err(reason) => BgeRun::unavailable(reason, semantic.len()),
    };
    run.e5_candidates = semantic.len();
    run.unscored = semantic
        .iter()
        .filter(|candidate| !selected.contains(&duplicate_pair_key(&candidate.left, &candidate.right)))
        .count();

    if run.status == "BGE_UNAVAILABLE" {
        for candidate in &mut candidates {
            if candidate.kind == "semantic_candidate" {
                candidate.bge_admission = Some("BGE_UNSCORED");
            }
        }
        return Ok(DuplicateAudit {
            candidates,
            filtered_candidates: Vec::new(),
            unscored_candidates: semantic,
            bge_run: Some(run),
        });
    }

    let mut filtered = Vec::new();
    let mut unscored = Vec::new();
    candidates.retain_mut(|candidate| {
        if candidate.kind != "semantic_candidate" {
            return true;
        }
        let key = duplicate_pair_key(&candidate.left, &candidate.right);
        let Some(evidence) = run
            .evidence
            .get(&canonical_pair_key(endpoint(&key.0.0, &key.0.1), endpoint(&key.1.0, &key.1.1)))
            .cloned()
        else {
            candidate.bge_admission = Some("BGE_UNSCORED");
            unscored.push(candidate.clone());
            return false;
        };
        candidate.bge = Some(evidence.clone());
        if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
            candidate.bge_admission = Some("BGE_RECOMMENDED");
            true
        } else {
            candidate.bge_admission = Some("BGE_FILTERED");
            filtered.push(candidate.clone());
            false
        }
    });
    for candidate in &semantic {
        if !selected.contains(&duplicate_pair_key(&candidate.left, &candidate.right)) {
            let mut candidate = candidate.clone();
            candidate.bge_admission = Some("BGE_UNSCORED");
            unscored.push(candidate);
        }
    }
    candidates.sort_by(duplicate_candidate_order);
    filtered.sort_by(duplicate_candidate_order);
    unscored.sort_by(duplicate_candidate_order);
    Ok(DuplicateAudit {
        candidates,
        filtered_candidates: filtered,
        unscored_candidates: unscored,
        bge_run: Some(run),
    })
}

#[cfg(feature = "spec-retrieval")]
fn render_duplicate_text(scanned_scenarios: usize, audit: &DuplicateAudit) -> String {
    let mut report = format!(
        "Givn spec duplicates\nScanned scenarios: {scanned_scenarios}\nCandidate count: {}\n",
        audit.candidates.len()
    );
    if let Some(run) = audit.bge_run.as_ref() {
        report.push_str(&format!(
            "BGE status: {}{}\nBGE model: {}\nBGE top_k: {}\nBGE E5 raw count: {}\nBGE recommended count: {}\nBGE filtered count: {}\nBGE unscored count: {}\nBGE unique pair count: {}\nBGE directional call count: {}\nBGE model load attempts: {}\nBGE runtime ms: {}\n",
            run.status,
            run.reason
                .map(|reason| format!(", reason {}", reason.as_str()))
                .unwrap_or_default(),
            if run.status == "BGE_ENABLED" { RERANKER_MODEL_ID } else { "null" },
            RERANKER_TOP_K,
            run.e5_candidates,
            audit
                .candidates
                .iter()
                .filter(|candidate| candidate.bge_admission == Some("BGE_RECOMMENDED"))
                .count(),
            audit.filtered_candidates.len(),
            audit.unscored_candidates.len(),
            run.evidence.len(),
            run.evidence.len() * 2,
            run.model_load_attempts,
            run.runtime_ms.map(|value| value.to_string()).unwrap_or_else(|| "null".to_string()),
        ));
    }
    for (rank, candidate) in audit.candidates.iter().enumerate() {
        report.push_str(&render_duplicate_candidate(rank + 1, candidate));
        report.push('\n');
    }
    for candidate in &audit.filtered_candidates {
        report.push_str(&format!("BGE filtered: {}\n", render_duplicate_candidate(0, candidate)));
    }
    for candidate in &audit.unscored_candidates {
        report.push_str(&format!("BGE unscored: {}\n", render_duplicate_candidate(0, candidate)));
    }
    report
}

#[cfg(feature = "spec-retrieval")]
fn render_duplicate_json(
    scanned_scenarios: usize,
    audit: &DuplicateAudit,
) -> Result<String> {
    let report = DuplicateJsonReport {
        schema_version: if audit.bge_run.is_some() {
            DUPLICATE_BGE_REPORT_SCHEMA_VERSION
        } else {
            DUPLICATE_REPORT_SCHEMA_VERSION
        },
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
        e5_candidate_count: audit.bge_run.as_ref().map(|run| run.e5_candidates),
        bge_run: audit.bge_run.as_ref().map(|run| duplicate_json_run(run, audit)),
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
            bge: candidate.bge.as_ref().map(|evidence| DuplicateJsonBgeEvidence {
                status: candidate.bge_admission.unwrap_or("BGE_RECOMMENDED"),
                forward_raw: evidence.forward_raw,
                reverse_raw: evidence.reverse_raw,
                symmetric_raw: evidence.symmetric_raw,
            }),
        },
    }
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
        unique_pairs: run.evidence.len(),
        directional_calls: run.evidence.len() * 2,
        recommended_count,
        filtered_count: audit.filtered_candidates.len(),
        unscored_count: audit.unscored_candidates.len(),
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
                .bge_admission
                .map(|status| format!(" bge_status={status}"))
                .unwrap_or_default()
        });
    format!(
        "rank {rank}: kind={} left={} right={} relation={} assertion_shape={} facts={} distinguishing_facts={} fact_contradiction={} score={score}{bge}",
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
    let change_dir = validated_review_change_dir(root, change_id)?;
    let delta_root = delta_specs_path(&change_dir, change_id)?;
    let refresh = refresh_index(root)?;
    let delta = collect_delta_scenarios(root, &delta_root)?;
    let report_path = change_dir.join("semantic-review.md");
    let mut candidates = semantic_candidates(root, &refresh.index, &delta)?;
    #[cfg(feature = "cross-encoder-reranker")]
    let bge_run = apply_semantic_bge(root, &delta, &mut candidates)?;
    #[cfg(not(feature = "cross-encoder-reranker"))]
    let bge_run: Option<BgeRun> = None;
    let prior = read_prior_remediation_entries(&report_path);
    let carried_classifications = apply_prior_classifications(&report_path, &mut candidates)?;
    let remediation =
        verify_semantic_remediation(root, &refresh.index, &delta, &prior, &candidates);
    let report = render_semantic_report(
        change_id,
        &candidates,
        remediation.is_err(),
        bge_run.as_ref(),
    );
    write_semantic_report(&report_path, &report)?;
    let verified_remediations = match remediation {
        Ok(count) => count,
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
                && (candidate.classification == "UNCERTAIN" || candidate.rationale.is_none())
        })
        .count();
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
) -> Result<Vec<SemanticCandidate>> {
    let active: Vec<&DeltaScenario> = delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
        .collect();
    if active.is_empty() {
        return Ok(Vec::new());
    }
    let vectors = embed_texts(
        root,
        &active
            .iter()
            .map(|item| item.scenario.text.clone())
            .collect::<Vec<_>>(),
        EmbeddingRole::Passage,
    )?;
    let permanent = permanent_semantic_endpoints(root)?;
    let mut candidates = BTreeMap::new();
    for (item, vector) in active.iter().zip(vectors.iter()) {
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
            if score < SEMANTIC_SIMILARITY_THRESHOLD {
                continue;
            }
            let candidate = semantic_candidate("delta-to-permanent", left, right, score);
            candidates.insert(candidate.candidate_id.clone(), candidate);
        }
    }
    for left_index in 0..active.len() {
        for right_index in (left_index + 1)..active.len() {
            let left = semantic_delta_endpoint(active[left_index]);
            let right = semantic_delta_endpoint(active[right_index]);
            let (left, right) = if semantic_endpoint_key(&left) <= semantic_endpoint_key(&right) {
                (left, right)
            } else {
                (right, left)
            };
            let score = semantic_score(
                &left,
                &right,
                cosine_similarity(&vectors[left_index], &vectors[right_index]),
            )?;
            if score < SEMANTIC_SIMILARITY_THRESHOLD {
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
    Ok(result)
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
    if candidates.len() > BGE_MAX_PAIRS {
        let count = candidates.len();
        for candidate in candidates {
            candidate.bge_admission = Some("BGE_UNSCORED");
        }
        return Ok(Some(BgeRun::unavailable(
            BgeUnavailableReason::PairCap,
            count,
        )));
    }
    let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = candidates
        .iter()
        .map(|candidate| {
            (
                endpoint(&candidate.left.file, &candidate.left.title),
                endpoint(&candidate.right.file, &candidate.right.title),
            )
        })
        .collect();
    let mut run = match backend {
        Ok(kind) => score_bge_pairs(root, &pairs, &texts, kind)?,
        Err(reason) => BgeRun::unavailable(reason, candidates.len()),
    };
    run.e5_candidates = candidates.len();
    if run.status == "BGE_UNAVAILABLE" {
        for candidate in candidates {
            candidate.bge_admission = Some("BGE_UNSCORED");
        }
        return Ok(Some(run));
    }
    for candidate in candidates {
        let key = canonical_pair_key(
            endpoint(&candidate.left.file, &candidate.left.title),
            endpoint(&candidate.right.file, &candidate.right.title),
        );
        if let Some(evidence) = run.evidence.get(&key).cloned() {
            candidate.bge_admission = Some(if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
                "BGE_RECOMMENDED"
            } else {
                "BGE_FILTERED"
            });
            candidate.bge = Some(evidence);
        } else {
            candidate.bge_admission = Some("BGE_UNSCORED");
        }
    }
    Ok(Some(run))
}

#[cfg(feature = "spec-retrieval")]
fn semantic_score(left: &SemanticEndpoint, right: &SemanticEndpoint, raw: f32) -> Result<f32> {
    if std::env::var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(raw);
    }
    let Some(overrides) = std::env::var_os("GIVN_SPEC_STUB_PAIR_SCORES") else {
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
    let text = scenario_text(scenario, file);
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
            "DUPLIKAT" | "VARIANTE" | "VALID-BOUNDARY"
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
        "DUPLIKAT" | "VARIANTE" | "VALID-BOUNDARY"
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
fn verify_semantic_remediation(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    prior: &[PriorRemediationEntry],
    candidates: &[SemanticCandidate],
) -> Result<usize> {
    let permanent = permanent_semantic_endpoints(root)?;
    let mut verified = 0;
    for entry in prior {
        if entry.rationale.is_empty()
            || entry.left_scope != "current-change"
            || entry.right_scope != "permanent"
            || entry.left_file.is_empty()
            || entry.left_title.is_empty()
            || entry.left_content_hash.is_empty()
            || entry.left_evidence_hash.is_empty()
            || entry.right_content_hash.is_empty()
            || entry.right_evidence_hash.is_empty()
        {
            bail!(
                "semantic classification is stale (candidate {})",
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
        let right = permanent.get(&(entry.right_file.clone(), entry.right_title.clone()));
        match entry.classification.as_str() {
            "DUPLIKAT" => {
                if let Some(left) = current_left {
                    let endpoint = semantic_delta_endpoint(left);
                    if endpoint.content_hash != entry.left_content_hash
                        || endpoint.evidence_hash != entry.left_evidence_hash
                    {
                        bail!(
                            "semantic classification is stale (candidate {})",
                            entry.candidate_id
                        );
                    }
                }
                if current_candidate.is_some()
                    || current_left.is_some()
                    || right.is_none_or(|endpoint| {
                        endpoint.content_hash != entry.right_content_hash
                            || endpoint.evidence_hash != entry.right_evidence_hash
                    })
                {
                    bail!(
                        "DUPLIKAT remediation requires removing the redundant scenario (candidate {})",
                        entry.candidate_id
                    );
                }
                verified += 1;
            }
            "VARIANTE" => {
                let Some(left) = current_left else {
                    bail!(
                        "semantic classification is stale (candidate {})",
                        entry.candidate_id
                    );
                };
                let Some(right) = right else {
                    bail!(
                        "semantic classification is stale (candidate {})",
                        entry.candidate_id
                    );
                };
                let score = recompute_prior_pair_score(root, index, left, right)?;
                if current_candidate.is_some()
                    || !left.is_outline
                    || !left.has_examples
                    || semantic_delta_endpoint(left).content_hash == entry.left_content_hash
                    || semantic_delta_endpoint(left).evidence_hash == entry.left_evidence_hash
                    || right.content_hash != entry.right_content_hash
                    || right.evidence_hash != entry.right_evidence_hash
                    || score >= SEMANTIC_SIMILARITY_THRESHOLD
                {
                    bail!(
                        "VARIANTE remediation requires consolidation (candidate {})",
                        entry.candidate_id
                    );
                }
                verified += 1;
            }
            "VALID-BOUNDARY" => {
                let Some(left) = current_left else {
                    bail!(
                        "semantic classification is stale (candidate {})",
                        entry.candidate_id
                    );
                };
                let Some(right) = right else {
                    bail!(
                        "semantic classification is stale (candidate {})",
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
                    || score >= SEMANTIC_SIMILARITY_THRESHOLD
                    || !observable_distinction(&left_endpoint, right)
                {
                    bail!(
                        "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary (candidate {})",
                        entry.candidate_id
                    );
                }
                verified += 1;
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
    .into_iter()
    .next()
    .context("retrieval embedder returned no remediation vector")?;
    let raw = cosine_similarity(&left_vector, &right_entry.embedding);
    semantic_score(&semantic_delta_endpoint(left), right, raw)
}

#[cfg(feature = "spec-retrieval")]
fn observable_distinction(left: &SemanticEndpoint, right: &SemanticEndpoint) -> bool {
    left.facts.facts != right.facts.facts || left.facts.assertions != right.facts.assertions
}

#[cfg(feature = "spec-retrieval")]
fn render_semantic_report(
    change_id: &str,
    candidates: &[SemanticCandidate],
    remediation_blocked: bool,
    bge_run: Option<&BgeRun>,
) -> String {
    let blocking = candidates
        .iter()
        .filter(|candidate| candidate.bge_admission != Some("BGE_FILTERED"))
        .collect::<Vec<_>>();
    let status = if candidates
        .iter()
        .filter(|candidate| candidate.bge_admission != Some("BGE_FILTERED"))
        .all(|candidate| candidate.classification != "UNCERTAIN" && candidate.rationale.is_some())
        && !remediation_blocked
    {
        "PASS"
    } else {
        "BLOCKED"
    };
    let mut report = format!(
        "# Semantic Review\nReport schema version: {}\nChange: \"{change_id}\"\nStatus: {status}\nCandidate count: {}\n",
        if bge_run.is_some() { SEMANTIC_BGE_REPORT_SCHEMA_VERSION } else { SEMANTIC_REPORT_SCHEMA_VERSION },
        blocking.len()
    );
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
            "BGE status: {}\nBGE reason: {}\nBGE model: {}\nBGE top_k: {}\nBGE E5 raw count: {}\nBGE recommended count: {}\nBGE filtered count: {}\nBGE unscored count: {}\nBGE unique pair count: {}\nBGE directional call count: {}\nBGE model load attempts: {}\nBGE runtime ms: {}\n",
            run.status,
            run.reason.map(BgeUnavailableReason::as_str).unwrap_or("null"),
            if run.status == "BGE_ENABLED" { RERANKER_MODEL_ID } else { "null" },
            RERANKER_TOP_K,
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

#[cfg(feature = "spec-retrieval")]
fn refresh_index(root: &Path) -> Result<IndexRefresh> {
    let entries = collect_scenarios(root)?;
    let index_path = spec_index_path(root);
    let previous = read_index(&index_path).unwrap_or_else(|_| SpecIndex {
        version: INDEX_VERSION,
        model: MODEL_ID.to_string(),
        entries: Vec::new(),
    });

    let mut result = Vec::with_capacity(entries.len());
    let mut pending = Vec::new();
    let mut reused = 0usize;

    for scenario in entries {
        if let Some(existing) = reusable_entry(&previous, &scenario) {
            result.push(existing.clone());
            reused += 1;
        } else {
            pending.push(scenario);
        }
    }

    let embedded = pending.len();
    if !pending.is_empty() {
        let texts: Vec<String> = pending.iter().map(|item| item.text.clone()).collect();
        let vectors = embed_texts(root, &texts, EmbeddingRole::Passage)?;
        for (scenario, embedding) in pending.into_iter().zip(vectors) {
            result.push(IndexEntry {
                title: scenario.title,
                file: scenario.file,
                content_hash: scenario.content_hash,
                embedding,
            });
        }
    }

    result.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.title.cmp(&right.title))
    });

    let index = SpecIndex {
        version: INDEX_VERSION,
        model: MODEL_ID.to_string(),
        entries: result,
    };
    write_index(&index_path, &index)?;

    Ok(IndexRefresh {
        index,
        reused,
        embedded,
    })
}

#[cfg(feature = "spec-retrieval")]
fn search_enabled(root: &Path, query: &str) -> Result<()> {
    let index_path = spec_index_path(root);
    let index = read_index(&index_path).with_context(|| {
        format!(
            "spec index is missing at {}; run `givn spec index` first",
            index_path.display()
        )
    })?;
    let query_vector = embed_texts(root, &[query.to_string()], EmbeddingRole::Query)?
        .into_iter()
        .next()
        .context("retrieval embedder returned no query vector")?;

    let ranked = rank_entries(&index.entries, &query_vector);

    print_search_results(&ranked);
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
    let refresh = refresh_index(root)?;
    let permanent_texts: BTreeMap<(String, String), String> = collect_scenarios(root)?
        .into_iter()
        .map(|scenario| ((scenario.file, scenario.title), scenario.text))
        .collect();
    let query_vector = embed_texts(
        root,
        std::slice::from_ref(&query.text),
        EmbeddingRole::Query,
    )?
    .into_iter()
    .next()
    .context("retrieval embedder returned no query vector")?;

    let mut candidates: Vec<ExploreCandidate> = refresh
        .index
        .entries
        .iter()
        .map(|entry| ExploreCandidate {
            title: entry.title.clone(),
            file: entry.file.clone(),
            text: permanent_texts
                .get(&(entry.file.clone(), entry.title.clone()))
                .cloned()
                .unwrap_or_else(|| entry.title.clone()),
            embedding: entry.embedding.clone(),
            scope: "permanent",
            change: None,
            projected_file: None,
        })
        .collect();

    if let Some((change_id, delta)) = &delta {
        let active_delta: Vec<&DeltaScenario> = delta
            .iter()
            .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
            .collect();
        let delta_texts: Vec<String> = active_delta
            .iter()
            .map(|item| item.scenario.text.clone())
            .collect();
        let delta_vectors = if delta_texts.is_empty() {
            Vec::new()
        } else {
            embed_texts(root, &delta_texts, EmbeddingRole::Passage)?
        };
        for (item, embedding) in active_delta.iter().zip(delta_vectors) {
            candidates.push(ExploreCandidate {
                title: item.scenario.title.clone(),
                file: item.scenario.file.clone(),
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
    let mut ranked: Vec<(ExploreCandidate, f32)> = candidates
        .into_iter()
        .filter(|candidate| !matches_query_identity(candidate, &query, &modified_target_identities))
        .map(|candidate| {
            let score = cosine_similarity(&query_vector, &candidate.embedding);
            (candidate, score)
        })
        .collect();
    ranked.sort_by(|(left, left_score), (right, right_score)| {
        right_score
            .total_cmp(left_score)
            .then_with(|| left.scope.cmp(right.scope))
            .then_with(|| left.file.cmp(&right.file))
            .then_with(|| left.title.cmp(&right.title))
    });
    let ranked: Vec<(ExploreCandidate, f32)> = ranked.into_iter().take(RERANKER_TOP_K).collect();
    #[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
    let bge_run = bge_explore_run(root, &query, &ranked)?;
    #[cfg(not(all(feature = "spec-retrieval", feature = "cross-encoder-reranker")))]
    let bge_run: Option<BgeRun> = None;

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
        results.push(ExploreResultOutput {
            rank: index + 1,
            title: candidate.title,
            file: candidate.file,
            score,
            scope: candidate.scope,
            change: candidate.change,
            bge,
        });
    }

    let recommended_results = bge_run.as_ref().map(|_| {
        results
            .iter()
            .filter(|result| {
                result
                    .bge
                    .as_ref()
                    .is_some_and(|evidence| evidence.status == "BGE_RECOMMENDED")
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    if json {
        let response = ExploreResponse {
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
            evidence: bge_run.as_ref().map(explore_evidence_output),
            recommended_results,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&response).context("serialize explore JSON")?
        );
    } else {
        print_explore_text(&query, &refresh, &results, bge_run.as_ref());
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
    let mut scenarios = feature.scenarios.iter().collect::<Vec<_>>();
    for rule in &feature.rules {
        scenarios.extend(rule.scenarios.iter());
    }
    anyhow::ensure!(
        scenarios.len() == 1,
        "explore input must contain exactly one scenario; found {}",
        scenarios.len()
    );

    let scenario = scenarios[0];
    let file_identity = identity_file.as_deref().unwrap_or("");
    let text = scenario_text(scenario, file_identity);
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
        e5_candidates: run.e5_candidates,
        recommended_candidates: recommended,
        bge_filtered_candidates: run
            .evidence
            .len()
            .saturating_sub(recommended),
        bge_unscored_candidates: run.unscored,
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
    let query_endpoint = explore_query_endpoint(query);
    let mut texts = BTreeMap::new();
    texts.insert(
        (query_endpoint.file.clone(), query_endpoint.title.clone()),
        query.text.clone(),
    );
    let mut pairs = Vec::with_capacity(ranked.len());
    for (candidate, _) in ranked {
        let candidate_endpoint = endpoint(&candidate.file, &candidate.title);
        texts.insert(
            (candidate_endpoint.file.clone(), candidate_endpoint.title.clone()),
            candidate.text.clone(),
        );
        pairs.push((query_endpoint.clone(), candidate_endpoint));
    }
    match backend {
        Ok(kind) => score_bge_pairs(root, &pairs, &texts, kind).map(Some),
        Err(reason) => Ok(Some(BgeRun::unavailable(reason, ranked.len()))),
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_backend(root: &Path) -> Result<Option<std::result::Result<BgeBackendKind, BgeUnavailableReason>>> {
    let selected = std::env::var("GIVN_SPEC_BGE");
    if std::env::var("GIVN_SPEC_BGE_APPROVAL").as_deref() == Ok("failed") {
        return Ok(None);
    }
    match selected.as_deref() {
        Ok("disabled") => Ok(None),
        Ok("stub") => Ok(Some(Ok(BgeBackendKind::Stub))),
        Ok("missing") => Ok(Some(Err(BgeUnavailableReason::MissingArtifact))),
        Ok("local") => {
            Ok(Some(match preflight_bge_artifacts(root) {
                Ok(_) => Ok(BgeBackendKind::Local),
                Err(_) => Err(BgeUnavailableReason::MissingArtifact),
            }))
        }
        Ok(value) => bail!("unknown GIVN_SPEC_BGE value `{value}`"),
        Err(std::env::VarError::NotPresent) if !bge_approval_valid(root) => Ok(None),
        Err(std::env::VarError::NotPresent) => {
            Ok(Some(match preflight_bge_artifacts(root) {
                Ok(_) => Ok(BgeBackendKind::Local),
                Err(_) => Err(BgeUnavailableReason::MissingArtifact),
            }))
        }
        Err(std::env::VarError::NotUnicode(_)) => bail!("GIVN_SPEC_BGE is not valid UTF-8"),
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn bge_approval_valid(root: &Path) -> bool {
    let path = std::env::var_os("GIVN_SPEC_BGE_APPROVAL_FILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("givn/.spec-bge-admission.json"));
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    let Ok(document) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return false;
    };
    document["schema_version"] == 1
        && document["model"] == RERANKER_MODEL_ID
        && document["threshold"] == serde_json::json!(BGE_ADMISSION_THRESHOLD)
        && document["top_k"] == serde_json::json!(RERANKER_TOP_K)
        && document["max_unique_pairs"] == serde_json::json!(BGE_MAX_PAIRS)
        && document["candidate_pipeline"] == BGE_PIPELINE_VERSION
        && document["acceptance"]["pass"] == true
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn preflight_bge_artifacts(root: &Path) -> Result<std::path::PathBuf> {
    let directory = std::env::var_os("GIVN_SPEC_BGE_MODEL_DIR")
        .or_else(|| std::env::var_os("GIVN_SPEC_BGE_CACHE"))
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("givn/.spec-reranker-cache"));
    let model = directory.join("model.onnx");
    let model_bytes = fs::read(&model)
        .with_context(|| format!("read BGE model artifact {}", model.display()))?;
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
    if model_bytes.windows(b"model.onnx.data".len()).any(|window| window == b"model.onnx.data")
        && !directory.join("model.onnx.data").is_file()
    {
        bail!("read BGE model artifact {}", directory.join("model.onnx.data").display());
    }
    Ok(directory)
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn score_bge_pairs(
    root: &Path,
    pairs: &[(SimilarityEndpoint, SimilarityEndpoint)],
    texts: &BTreeMap<(String, String), String>,
    backend: BgeBackendKind,
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
        BgeBackendKind::Local => {
            let directory = preflight_bge_artifacts(root)?;
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
                .with_max_length(NLI_MAX_LENGTH)
                .with_intra_threads(4);
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
    Ok(BgeRun::enabled(evidence, pairs.len(), started.elapsed().as_millis()))
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
fn bge_directional_overrides() -> Result<Option<BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), f32>>>
{
    let Some(raw) = std::env::var_os("GIVN_SPEC_BGE_DIRECTIONAL_SCORES") else {
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
    bge_run: Option<&BgeRun>,
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
            "BGE candidate generator: E5, top_k {}, raw {}, recommended {}, BGE filtered {}, unscored {}, bounded pool",
            RERANKER_TOP_K,
            run.e5_candidates,
            recommended,
            filtered,
            run.unscored,
        );
        println!(
            "BGE model: {}, unique pairs {}, directional calls {}, model load attempts {}, elapsed runtime ms {}",
            RERANKER_MODEL_ID,
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
fn review_enabled(root: &Path, change_id: &str) -> Result<()> {
    let change_dir = validated_review_change_dir(root, change_id)?;
    let delta_root = delta_specs_path(&change_dir, change_id)?;
    index_enabled(root)?;
    let index = read_index(&spec_index_path(root))?;

    let delta = collect_delta_scenarios(root, &delta_root)?;
    #[cfg(feature = "cross-encoder-reranker")]
    if bge_backend(root)?.is_some() {
        return run_bge_advisory_review(root, &index, &delta);
    }
    #[cfg(feature = "nli-evidence")]
    if let Some(backend) = nli_backend_from_env()? {
        let (candidates, considered) = nli_similarity_candidates(root, &index, &delta, backend)?;
        print_nli_results(&candidates, considered);
        return Ok(());
    }
    if cfg!(feature = "spec-similarity-gate") {
        #[cfg(feature = "spec-similarity-gate")]
        return run_embedding_review(root, &index, &delta);
    }
    advisory_review(root, &index, &delta, &change_dir.join("review.md"))
}

#[cfg(feature = "spec-retrieval")]
fn advisory_review(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    review_path: &Path,
) -> Result<()> {
    let (dispositions, _) = crate::overlap::disposition::parse_review(review_path);
    for item in delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
    {
        let query_vector = embed_texts(
            root,
            std::slice::from_ref(&item.scenario.text),
            EmbeddingRole::Query,
        )?
        .into_iter()
        .next()
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
    Ok(())
}

#[cfg(all(feature = "spec-retrieval", not(feature = "cross-encoder-reranker")))]
fn run_embedding_review(root: &Path, index: &SpecIndex, delta: &[DeltaScenario]) -> Result<()> {
    let (candidates, considered) = embedding_similarity_candidates(root, index, delta)?;
    for candidate in &candidates {
        println!(
            "similarity candidate [{}]: '{}' ({}) resembles '{}' ({}) score {:.4}, threshold {:.4}; remediation: MERGE (delete the true duplicate) or KEEP-AS-BOUNDARY (rewrite the retained scenario below the threshold)",
            candidate.scopes.join(","),
            candidate.left.title,
            candidate.left.file,
            candidate.right.title,
            candidate.right.file,
            candidate.embedding_score,
            SIMILARITY_THRESHOLD,
        );
    }
    println!(
        "retrieval advisory: embedding similarity evaluated {} candidate pairs",
        considered
    );
    if !candidates.is_empty() {
        bail!(
            "similarity gate failed: {} candidate(s) are at or above the fixed threshold {SIMILARITY_THRESHOLD:.4}; delete true duplicates or rewrite valid boundaries until every score is below the threshold",
            candidates.len()
        );
    }
    println!(
        "similarity gate: no scenario pair at or above fixed threshold {:.4}",
        SIMILARITY_THRESHOLD
    );
    Ok(())
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn run_bge_advisory_review(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<()> {
    let (mut candidates, considered, texts) = similarity_candidate_pool(root, index, delta)?;
    let backend = bge_backend(root)?.context("BGE backend was not selected")?;
    if candidates.len() > BGE_MAX_PAIRS {
        println!(
            "BGE_UNAVAILABLE: bounded candidate pool exceeds 512 pairs (reason pair-cap, raw {}, unique pairs 0, directional calls 0, model load attempts 0)",
            considered
        );
        println!("BGE status: BGE_UNAVAILABLE");
        println!("retrieval advisory: E5-only fallback remains advisory");
        return Ok(());
    }
    let pairs: Vec<(SimilarityEndpoint, SimilarityEndpoint)> = candidates
        .iter()
        .map(|candidate| (candidate.left.clone(), candidate.right.clone()))
        .collect();
    let run = match backend {
        Ok(kind) => score_bge_pairs(root, &pairs, &texts, kind)?,
        Err(reason) => BgeRun::unavailable(reason, considered),
    };
    if run.status == "BGE_UNAVAILABLE" {
        println!(
            "BGE_UNAVAILABLE: reason {}, recommended 0, filtered 0, unscored {}",
            run.reason
                .map(BgeUnavailableReason::as_str)
                .unwrap_or("missing-artifact"),
            considered
        );
        println!("retrieval advisory: E5-only fallback remains advisory");
        return Ok(());
    }
    println!(
        "BGE: model {}, top_k {}, E5 raw {}, unique pairs {}, directional calls {}, runtime ms {}, evidence is advisory",
        RERANKER_MODEL_ID,
        RERANKER_TOP_K,
        considered,
        run.evidence.len(),
        run.evidence.len() * 2,
        run.runtime_ms.unwrap_or(0),
    );
    let mut recommended = 0;
    let mut filtered = 0;
    for candidate in &mut candidates {
        let key = canonical_pair_key(candidate.left.clone(), candidate.right.clone());
        let Some(evidence) = run.evidence.get(&key).cloned() else {
            continue;
        };
        let status = if evidence.symmetric_raw >= BGE_ADMISSION_THRESHOLD {
            recommended += 1;
            "BGE_RECOMMENDED"
        } else {
            filtered += 1;
            "BGE_FILTERED"
        };
        candidate.rerank_score = evidence.symmetric_raw;
        candidate.bge = Some(evidence.clone());
        candidate.bge_admission = Some(status);
        println!(
            "{}: '{}' ({}) resembles '{}' ({}) E5 {:.4}, forward {:.4}, reverse {:.4}, symmetric {:.4}; evidence is advisory and no Givn classification is assigned",
            status,
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
    println!(
        "BGE candidate accounting: recommended {}, BGE filtered {}, unscored {}, bounded pool <= {}",
        recommended, filtered, run.unscored, BGE_MAX_PAIRS
    );
    Ok(())
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn run_reranker_review(root: &Path, index: &SpecIndex, delta: &[DeltaScenario]) -> Result<()> {
    let (candidates, considered) = similarity_candidates(root, index, delta)?;
    println!(
        "reranker: model {}, top_k {}, considered {} candidate pairs, fixed raw-logit boundary {:.4}",
        RERANKER_MODEL_ID, RERANKER_TOP_K, considered, RERANKER_LOGIT_THRESHOLD,
    );
    for candidate in &candidates {
        println!(
            "reranker candidate [{}]: '{}' ({}) resembles '{}' ({}) embedding {:.4}, rerank raw {:.4}, sigmoid {:.4}, boundary {:.4}; remediation: MERGE (delete an exact duplicate or parameterize a variant) or KEEP-AS-BOUNDARY (rewrite the distinct invariant below the boundary)",
            candidate.scopes.join(","),
            candidate.left.title,
            candidate.left.file,
            candidate.right.title,
            candidate.right.file,
            candidate.embedding_score,
            candidate.rerank_score,
            sigmoid(candidate.rerank_score),
            RERANKER_LOGIT_THRESHOLD,
        );
    }
    println!(
        "retrieval advisory: pair reranker evaluated {} candidate pairs",
        considered
    );
    if !candidates.is_empty() {
        bail!(
            "similarity gate failed: {} reranker candidate(s) are at or above the fixed raw-logit boundary {RERANKER_LOGIT_THRESHOLD:.4}; delete true duplicates or rewrite valid boundaries until every score is below the boundary",
            candidates.len()
        );
    }
    println!(
        "reranker gate: no scenario pair at or above fixed raw-logit boundary {:.4}",
        RERANKER_LOGIT_THRESHOLD
    );
    Ok(())
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
        text: scenario_text(scenario, file).text,
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
) -> Result<Vec<DuplicateCandidate>> {
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
    let needs_semantic_embedding = scenarios.iter().enumerate().any(|(left_index, _)| {
        scenarios
            .iter()
            .enumerate()
            .skip(left_index + 1)
            .any(|(right_index, _)| !classified.contains(&(left_index, right_index)))
    });
    let embeddings = if !needs_semantic_embedding {
        Vec::new()
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
    for (left_index, left) in scenarios.iter().enumerate() {
        for (right_index, right) in scenarios.iter().enumerate().skip(left_index + 1) {
            if classified.contains(&(left_index, right_index)) {
                continue;
            }
            let raw_score = cosine_similarity(&embeddings[left_index], &embeddings[right_index]);
            let score = score_overrides
                .as_ref()
                .and_then(|overrides| {
                    overrides.get(&duplicate_pair_key(&left.endpoint, &right.endpoint))
                })
                .copied()
                .unwrap_or(raw_score);
            if score < SEMANTIC_SIMILARITY_THRESHOLD {
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
            });
        }
    }
    candidates.sort_by(duplicate_candidate_order);
    Ok(candidates)
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
    if std::env::var("GIVN_SPEC_EMBEDDER").as_deref() != Ok("stub") {
        return Ok(None);
    }
    let Some(raw) = std::env::var_os("GIVN_SPEC_DUPLICATES_TEST_SCORES") else {
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
        collect_feature_scenarios(&feature.scenarios, &file, &mut scenarios);
        for rule in &feature.rules {
            collect_feature_scenarios(&rule.scenarios, &file, &mut scenarios);
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
            scenario: scenario_text(scenario, &file),
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
    source: &[gherkin::Scenario],
    file: &str,
    output: &mut Vec<ScenarioText>,
) {
    for scenario in source {
        if !is_queryable_scenario(scenario) {
            continue;
        }
        output.push(scenario_text(scenario, file));
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
fn scenario_text(scenario: &gherkin::Scenario, file: &str) -> ScenarioText {
    let duplicate_audit_delta = file.contains("givn/changes/spec-duplicates/");
    let mut text = scenario.name.clone();
    if duplicate_audit_delta
        && let Some(description) = scenario.description.as_deref()
        && !description.trim().is_empty()
    {
        text.push('\n');
        text.push_str(description.trim());
    }
    for step in &scenario.steps {
        if duplicate_audit_delta && is_semantic_driver_step(step.value.trim()) {
            continue;
        }
        text.push('\n');
        text.push_str(step.value.trim());
        if let Some(docstring) = &step.docstring {
            text.push('\n');
            text.push_str(docstring.trim());
        }
    }
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    ScenarioText {
        title: scenario.name.clone(),
        file: file.to_string(),
        content_hash: content_hash(&normalized),
        text: normalized,
    }
}

#[cfg(feature = "spec-retrieval")]
fn is_semantic_driver_step(value: &str) -> bool {
    [
        "the user runs ",
        "the end-to-end user runs ",
        "the one-file audit ",
        "the two-file audit ",
        "the one-file result ",
        "the two-file result ",
        "candidate rank ",
        "same-feature candidate rank ",
        "cross-feature candidate rank ",
        "intra-file candidate rank ",
        "inter-file candidate rank ",
    ]
    .iter()
    .any(|prefix| value.starts_with(prefix))
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
    let delta_vectors = if delta_texts.is_empty() {
        Vec::new()
    } else {
        embed_texts(root, &delta_texts, EmbeddingRole::Passage)?
    };

    let mut candidates = BTreeMap::new();
    for (delta, vector) in active_delta.iter().zip(delta_vectors.iter()) {
        let own_target = (delta.target_file.as_str(), delta.scenario.title.as_str());
        let mut neighbours: Vec<(f32, &IndexEntry)> = index
            .entries
            .iter()
            .filter(|entry| {
                entry.file.as_str() != delta.target_file.as_str()
                    && !(delta.operation == crate::gherkin::delta::DeltaOp::Modified
                        && (entry.file.as_str(), entry.title.as_str()) == own_target)
            })
            .map(|entry| (cosine_similarity(vector, &entry.embedding), entry))
            .collect();
        neighbours.sort_by(|left, right| right.0.total_cmp(&left.0));
        for (score, entry) in neighbours.into_iter().take(RERANKER_TOP_K) {
            add_similarity_candidate(
                &mut candidates,
                "delta-to-permanent",
                endpoint(&delta.target_file, &delta.scenario.title),
                endpoint(&entry.file, &entry.title),
                score,
            );
        }
    }

    let projected = project_records(index, delta, &delta_vectors, &active_delta)?;
    for (left, right) in projected.iter().enumerate() {
        let mut neighbours: Vec<(f32, &ScenarioRecord)> = projected
            .iter()
            .enumerate()
            .filter(|(index, other)| *index != left && other.endpoint.file != right.endpoint.file)
            .map(|(_, other)| (cosine_similarity(&right.embedding, &other.embedding), other))
            .collect();
        neighbours.sort_by(|left, right| right.0.total_cmp(&left.0));
        for (score, other) in neighbours.into_iter().take(RERANKER_TOP_K) {
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
    Ok((result, considered, candidate_texts))
}

#[cfg(all(feature = "spec-retrieval", not(feature = "cross-encoder-reranker")))]
fn embedding_similarity_candidates(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<(Vec<SimilarityCandidate>, usize)> {
    let active_delta: Vec<&DeltaScenario> = delta
        .iter()
        .filter(|item| item.operation != crate::gherkin::delta::DeltaOp::Removed)
        .collect();
    let delta_texts: Vec<String> = active_delta
        .iter()
        .map(|item| item.scenario.text.clone())
        .collect();
    let delta_vectors = if delta_texts.is_empty() {
        Vec::new()
    } else {
        embed_texts(root, &delta_texts, EmbeddingRole::Passage)?
    };

    let mut candidates = BTreeMap::new();
    for (delta, vector) in active_delta.iter().zip(delta_vectors.iter()) {
        let own_target = (delta.target_file.as_str(), delta.scenario.title.as_str());
        for entry in &index.entries {
            if delta.operation == crate::gherkin::delta::DeltaOp::Modified
                && (entry.file.as_str(), entry.title.as_str()) == own_target
            {
                continue;
            }
            let score = cosine_similarity(vector, &entry.embedding);
            if score >= SIMILARITY_THRESHOLD {
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

    let projected = project_records(index, delta, &delta_vectors, &active_delta)?;
    for (left, right) in projected.iter().enumerate() {
        for other in projected.iter().skip(left + 1) {
            let score = cosine_similarity(&right.embedding, &other.embedding);
            if score >= SIMILARITY_THRESHOLD {
                add_similarity_candidate(
                    &mut candidates,
                    "projected-permanent",
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
    Ok((result, considered))
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn similarity_candidates(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<(Vec<SimilarityCandidate>, usize)> {
    let (mut result, considered, candidate_texts) = similarity_candidate_pool(root, index, delta)?;
    rerank_candidates(root, &mut result, &candidate_texts)?;
    result.retain(|candidate| candidate.rerank_score >= RERANKER_LOGIT_THRESHOLD);
    result.sort_by(|left, right| {
        right
            .rerank_score
            .total_cmp(&left.rerank_score)
            .then_with(|| left.left.cmp(&right.left))
            .then_with(|| left.right.cmp(&right.right))
    });
    Ok((result, considered))
}

#[cfg(feature = "spec-retrieval")]
fn nli_backend_from_env() -> Result<Option<NliBackend>> {
    match std::env::var("GIVN_SPEC_RERANKER").as_deref() {
        Ok("nli") => Ok(Some(NliBackend::Local)),
        Ok("nli-stub") => Ok(Some(NliBackend::Stub)),
        Ok("nli-missing") => {
            bail!("NLI model is unavailable; configure a local NLI model directory")
        }
        Ok(_) | Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            bail!("GIVN_SPEC_RERANKER is not valid UTF-8")
        }
    }
}

#[cfg(feature = "spec-retrieval")]
fn nli_similarity_candidates(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
    backend: NliBackend,
) -> Result<(Vec<SimilarityCandidate>, usize)> {
    let (mut result, considered, candidate_texts) = similarity_candidate_pool(root, index, delta)?;
    score_nli_candidates(root, &mut result, &candidate_texts, backend)?;
    result.sort_by(|left, right| {
        nli_strength(right)
            .total_cmp(&nli_strength(left))
            .then_with(|| right.embedding_score.total_cmp(&left.embedding_score))
            .then_with(|| left.left.cmp(&right.left))
            .then_with(|| left.right.cmp(&right.right))
    });
    Ok((result, considered))
}

#[cfg(feature = "spec-retrieval")]
fn print_nli_results(candidates: &[SimilarityCandidate], considered: usize) {
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
    let report_limit = nli_report_limit();
    println!(
        "nli: model {}, top_k {}, considered {} candidate pairs; evidence is diagnostic only",
        NLI_MODEL_ID, RERANKER_TOP_K, considered
    );
    println!(
        "nli distribution: entailment {}, neutral {}, contradiction {}",
        label_counts[0], label_counts[1], label_counts[2]
    );
    for candidate in candidates.iter().take(report_limit) {
        let Some(evidence) = candidate.nli else {
            continue;
        };
        println!(
            "nli candidate [{}]: '{}' ({}) resembles '{}' ({}) embedding {:.4}; forward [entailment {:.4}, neutral {:.4}, contradiction {:.4}], reverse [entailment {:.4}, neutral {:.4}, contradiction {:.4}], symmetric [entailment {:.4}, neutral {:.4}, contradiction {:.4}], strongest {}",
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
    }
    println!(
        "nli diagnostic: reported at most {} candidates; NLI probabilities are not calibrated duplicate, variant, or boundary decisions",
        report_limit
    );
}

#[cfg(feature = "spec-retrieval")]
fn nli_report_limit() -> usize {
    std::env::var("GIVN_SPEC_NLI_REPORT_LIMIT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(NLI_REPORT_LIMIT)
}

#[cfg(feature = "spec-retrieval")]
fn nli_strength(candidate: &SimilarityCandidate) -> f32 {
    candidate
        .nli
        .map(|evidence| evidence.symmetric[0].max(evidence.symmetric[2]))
        .unwrap_or(0.0)
}

#[cfg(feature = "spec-retrieval")]
fn score_nli_candidates(
    root: &Path,
    candidates: &mut [SimilarityCandidate],
    texts: &BTreeMap<(String, String), String>,
    backend: NliBackend,
) -> Result<()> {
    if candidates.is_empty() {
        return Ok(());
    }

    match backend {
        NliBackend::Stub => {
            for candidate in candidates {
                let left = scenario_text_for(texts, &candidate.left)?;
                let right = scenario_text_for(texts, &candidate.right)?;
                candidate.nli = Some(nli_evidence(
                    stub_nli_logits(left, right),
                    stub_nli_logits(right, left),
                ));
            }
        }
        NliBackend::Local => {
            let mut classifier = NliClassifier::load(root)
                .context("NLI model is unavailable; check GIVN_SPEC_NLI_MODEL_DIR")?;
            let mut pairs = Vec::with_capacity(candidates.len() * 2);
            for candidate in candidates.iter() {
                let left = scenario_text_for(texts, &candidate.left)?;
                let right = scenario_text_for(texts, &candidate.right)?;
                pairs.push((left.to_string(), right.to_string()));
                pairs.push((right.to_string(), left.to_string()));
            }
            let logits = classifier.score_pairs(&pairs)?;
            for (candidate, pair_logits) in candidates.iter_mut().zip(logits.chunks_exact(2)) {
                candidate.nli = Some(nli_evidence(pair_logits[0], pair_logits[1]));
            }
        }
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
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

#[cfg(feature = "spec-retrieval")]
fn softmax3(logits: [f32; 3]) -> [f32; 3] {
    let maximum = logits.into_iter().fold(f32::NEG_INFINITY, f32::max);
    let exponentials = logits.map(|logit| (logit - maximum).exp());
    let total = exponentials.into_iter().sum::<f32>();
    exponentials.map(|value| value / total)
}

#[cfg(feature = "spec-retrieval")]
fn nli_label(probabilities: &[f32; 3]) -> &'static str {
    let (index, _) = probabilities
        .iter()
        .copied()
        .enumerate()
        .max_by(|left, right| left.1.total_cmp(&right.1))
        .unwrap_or((1, 0.0));
    ["entailment", "neutral", "contradiction"][index]
}

#[cfg(feature = "spec-retrieval")]
fn stub_nli_logits(left: &str, right: &str) -> [f32; 3] {
    let left_tokens = reranker_tokens(left);
    let right_tokens = reranker_tokens(right);
    let shared = left_tokens.intersection(&right_tokens).count() as f32;
    let union = left_tokens.union(&right_tokens).count() as f32;
    let overlap = if union == 0.0 { 0.0 } else { shared / union };
    if overlap >= 0.55 {
        [2.2, -0.2, 0.1]
    } else {
        [-0.4, 1.4, 0.2]
    }
}

#[cfg(feature = "spec-retrieval")]
struct NliClassifier {
    tokenizer: Tokenizer,
    session: ort::session::Session,
    label_indices: [usize; 3],
    need_token_type_ids: bool,
}

#[cfg(feature = "spec-retrieval")]
impl NliClassifier {
    fn load(root: &Path) -> Result<Self> {
        let model_dir = std::env::var_os("GIVN_SPEC_NLI_MODEL_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| root.join("givn/.spec-nli-cache"));
        let config_path = model_dir.join("config.json");
        let config_bytes = fs::read(&config_path)
            .with_context(|| format!("read NLI config {}", config_path.display()))?;
        let config: serde_json::Value =
            serde_json::from_slice(&config_bytes).context("parse NLI config.json")?;
        let label_indices = nli_label_indices(&config)?;

        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer_config_path = model_dir.join("tokenizer_config.json");
        let tokenizer_config_bytes = fs::read(&tokenizer_config_path).with_context(|| {
            format!(
                "read NLI tokenizer config {}",
                tokenizer_config_path.display()
            )
        })?;
        let tokenizer_config: serde_json::Value = serde_json::from_slice(&tokenizer_config_bytes)
            .context("parse NLI tokenizer_config.json")?;
        let pad_id = config["pad_token_id"].as_u64().unwrap_or(0) as u32;
        let pad_token = tokenizer_config["pad_token"]
            .as_str()
            .or_else(|| config["pad_token"].as_str())
            .unwrap_or("[PAD]")
            .to_string();
        let model_max_length = tokenizer_config["model_max_length"]
            .as_u64()
            .unwrap_or(NLI_MAX_LENGTH as u64) as usize;
        let max_length = model_max_length.min(NLI_MAX_LENGTH);

        let tokenizer_bytes = fs::read(&tokenizer_path)
            .with_context(|| format!("read NLI tokenizer {}", tokenizer_path.display()))?;
        let tokenizer = Tokenizer::from_bytes(tokenizer_bytes)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .with_padding(Some(PaddingParams {
                strategy: PaddingStrategy::BatchLongest,
                pad_token,
                pad_id,
                ..Default::default()
            }))
            .with_truncation(Some(TruncationParams {
                max_length,
                ..Default::default()
            }))
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .clone();

        let model_path = model_dir.join("model.onnx");
        let session = ort::session::Session::builder()
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .with_intra_threads(4)
            .map_err(|error| anyhow::anyhow!(error.to_string()))?
            .commit_from_file(&model_path)
            .with_context(|| format!("load NLI ONNX model {}", model_path.display()))?;
        let need_token_type_ids = session
            .inputs()
            .iter()
            .any(|input| input.name() == "token_type_ids");

        Ok(Self {
            tokenizer: tokenizer.into(),
            session,
            label_indices,
            need_token_type_ids,
        })
    }

    fn score_pairs(&mut self, pairs: &[(String, String)]) -> Result<Vec<[f32; 3]>> {
        let mut scores = Vec::with_capacity(pairs.len());
        for batch in pairs.chunks(32) {
            let inputs: Vec<(&str, &str)> = batch
                .iter()
                .map(|(left, right)| (left.as_str(), right.as_str()))
                .collect();
            let encodings = self
                .tokenizer
                .encode_batch(inputs, true)
                .map_err(|error| anyhow::anyhow!(error.to_string()))
                .context("tokenize NLI scenario pairs")?;
            let encoding_length = encodings
                .first()
                .context("NLI tokenizer returned no encodings")?
                .len();
            let batch_size = batch.len();
            let mut ids = Vec::with_capacity(batch_size * encoding_length);
            let mut masks = Vec::with_capacity(batch_size * encoding_length);
            let mut type_ids = Vec::with_capacity(batch_size * encoding_length);
            for encoding in &encodings {
                ids.extend(encoding.get_ids().iter().map(|id| *id as i64));
                masks.extend(
                    encoding
                        .get_attention_mask()
                        .iter()
                        .map(|mask| *mask as i64),
                );
                type_ids.extend(encoding.get_type_ids().iter().map(|id| *id as i64));
            }

            let ids = Array::from_shape_vec((batch_size, encoding_length), ids)?;
            let masks = Array::from_shape_vec((batch_size, encoding_length), masks)?;
            let type_ids = Array::from_shape_vec((batch_size, encoding_length), type_ids)?;
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
                values.len() == batch_size * 3,
                "NLI logits shape has {} values for {} pairs",
                values.len(),
                batch_size
            );
            for row in values.chunks_exact(3) {
                scores.push([
                    row[self.label_indices[0]],
                    row[self.label_indices[1]],
                    row[self.label_indices[2]],
                ]);
            }
        }
        Ok(scores)
    }
}

#[cfg(feature = "spec-retrieval")]
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
    delta_vectors: &[Vec<f32>],
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
                entries.insert(
                    key,
                    ScenarioRecord {
                        endpoint: endpoint(&item.target_file, &item.scenario.title),
                        embedding: delta_vectors[position].clone(),
                    },
                );
            }
            crate::gherkin::delta::DeltaOp::Added => {
                let position = active_delta
                    .iter()
                    .position(|active| std::ptr::eq(*active, item))
                    .context("added delta vector is missing")?;
                entries.insert(
                    key,
                    ScenarioRecord {
                        endpoint: endpoint(&item.target_file, &item.scenario.title),
                        embedding: delta_vectors[position].clone(),
                    },
                );
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
            existing.rerank_score = score;
        }
        return;
    }
    candidates.insert(
        key,
        SimilarityCandidate {
            left,
            right,
            embedding_score: score,
            rerank_score: f32::NEG_INFINITY,
            bge: None,
            bge_admission: None,
            nli: None,
            scopes: vec![scope],
        },
    );
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn rerank_candidates(
    root: &Path,
    candidates: &mut [SimilarityCandidate],
    texts: &BTreeMap<(String, String), String>,
) -> Result<()> {
    if candidates.is_empty() {
        return Ok(());
    }

    match std::env::var("GIVN_SPEC_RERANKER").as_deref() {
        Ok("stub") => {
            for candidate in candidates {
                let left = scenario_text_for(texts, &candidate.left)?;
                let right = scenario_text_for(texts, &candidate.right)?;
                let forward = stub_reranker_score(left, right);
                let reverse = stub_reranker_score(right, left);
                candidate.rerank_score = (forward + reverse) / 2.0;
            }
            Ok(())
        }
        Ok("missing") => bail!(
            "reranker model is unavailable; configure a local reranker cache or unset GIVN_SPEC_RERANKER"
        ),
        Ok(value) => bail!("unknown GIVN_SPEC_RERANKER value `{value}`"),
        Err(std::env::VarError::NotPresent) => rerank_with_fastembed(root, candidates, texts),
        Err(std::env::VarError::NotUnicode(_)) => {
            bail!("GIVN_SPEC_RERANKER is not valid UTF-8")
        }
    }
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn rerank_with_fastembed(
    root: &Path,
    candidates: &mut [SimilarityCandidate],
    texts: &BTreeMap<(String, String), String>,
) -> Result<()> {
    let cache_dir = root.join("givn/.spec-reranker-cache");
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("create reranker model cache {}", cache_dir.display()))?;
    let options = fastembed::RerankInitOptions::new(fastembed::RerankerModel::BGERerankerV2M3)
        .with_cache_dir(cache_dir)
        .with_show_download_progress(false)
        .with_intra_threads(4);
    let mut model = fastembed::TextRerank::try_new(options)
        .context("reranker model is unavailable; check the local reranker cache and network")?;

    let mut documents_by_query: BTreeMap<SimilarityEndpoint, Vec<SimilarityEndpoint>> =
        BTreeMap::new();
    for candidate in candidates.iter() {
        documents_by_query
            .entry(candidate.left.clone())
            .or_default()
            .push(candidate.right.clone());
        documents_by_query
            .entry(candidate.right.clone())
            .or_default()
            .push(candidate.left.clone());
    }

    let mut directed_scores: BTreeMap<(SimilarityEndpoint, SimilarityEndpoint), f32> =
        BTreeMap::new();
    for (query, documents) in documents_by_query {
        let query_text = scenario_text_for(texts, &query)?.to_string();
        let document_texts: Vec<String> = documents
            .iter()
            .map(|document| scenario_text_for(texts, document).map(str::to_string))
            .collect::<Result<Vec<_>>>()?;
        let results = model
            .rerank(query_text, document_texts, false, Some(32))
            .context("rerank similarity candidates")?;
        for result in results {
            directed_scores.insert(
                (query.clone(), documents[result.index].clone()),
                result.score,
            );
        }
    }

    for candidate in candidates {
        let forward = directed_scores
            .get(&(candidate.left.clone(), candidate.right.clone()))
            .context("reranker omitted forward pair score")?;
        let reverse = directed_scores
            .get(&(candidate.right.clone(), candidate.left.clone()))
            .context("reranker omitted reverse pair score")?;
        candidate.rerank_score = (forward + reverse) / 2.0;
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn scenario_text_for<'a>(
    texts: &'a BTreeMap<(String, String), String>,
    endpoint: &SimilarityEndpoint,
) -> Result<&'a str> {
    texts
        .get(&(endpoint.file.clone(), endpoint.title.clone()))
        .map(String::as_str)
        .with_context(|| format!("missing scenario text for '{}'", endpoint.title))
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

#[cfg(feature = "spec-retrieval")]
fn reranker_tokens(text: &str) -> std::collections::BTreeSet<String> {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_lowercase)
        .collect()
}

#[cfg(all(feature = "spec-retrieval", feature = "cross-encoder-reranker"))]
fn sigmoid(value: f32) -> f32 {
    1.0 / (1.0 + (-value).exp())
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
fn embed_texts(root: &Path, texts: &[String], role: EmbeddingRole) -> Result<Vec<Vec<f32>>> {
    match std::env::var("GIVN_SPEC_EMBEDDER").as_deref() {
        Ok("stub") => Ok(texts
            .iter()
            .map(|text| stub_embedding(text, role))
            .collect()),
        Ok("missing") => bail!(
            "retrieval model is unavailable; configure a local model cache or unset GIVN_SPEC_EMBEDDER"
        ),
        Ok(value) => bail!("unknown GIVN_SPEC_EMBEDDER value `{value}`"),
        Err(std::env::VarError::NotPresent) => fastembed_texts(root, texts, role),
        Err(std::env::VarError::NotUnicode(_)) => bail!("GIVN_SPEC_EMBEDDER is not valid UTF-8"),
    }
}

#[cfg(feature = "spec-retrieval")]
fn stub_embedding(text: &str, role: EmbeddingRole) -> Vec<f32> {
    const DIMENSIONS: usize = 256;
    let prefix = match role {
        EmbeddingRole::Passage => "passage",
        EmbeddingRole::Query => "query",
    };
    let mut vector = vec![0.0f32; DIMENSIONS];
    for token in format!("{prefix} {text}")
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(str::to_lowercase)
    {
        let index = (stable_hash(token.as_bytes()) as usize) % DIMENSIONS;
        vector[index] += 1.0;
    }
    normalize(vector)
}

#[cfg(feature = "spec-retrieval")]
fn fastembed_texts(root: &Path, texts: &[String], role: EmbeddingRole) -> Result<Vec<Vec<f32>>> {
    let cache_dir = std::env::var_os("GIVN_SPEC_MODEL_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("givn/.spec-model-cache"));
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("create retrieval model cache {}", cache_dir.display()))?;
    let options = fastembed::InitOptions::new(fastembed::EmbeddingModel::MultilingualE5Large)
        .with_cache_dir(cache_dir)
        .with_show_download_progress(false)
        .with_intra_threads(1);
    let mut model = fastembed::TextEmbedding::try_new(options)
        .context("retrieval model is unavailable; check the local model cache and network")?;
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
fn bge_pipeline_descriptor() -> String {
    format!(
        "model={RERANKER_MODEL_ID};top_k={RERANKER_TOP_K};threshold={RERANKER_LOGIT_THRESHOLD:.4};max_pairs={BGE_MAX_PAIRS};exclusions=identity,deterministic;ordering=e5-desc,file,title;pair=canonical-undirected;directions=forward,reverse;score=symmetric-average"
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
fn print_search_results(ranked: &[(&IndexEntry, f32)]) {
    for (rank, (entry, score)) in ranked.iter().take(5).enumerate() {
        println!(
            "rank {}: {} ({}, score {:.4})",
            rank + 1,
            entry.title,
            entry.file,
            score
        );
    }
}

#[cfg(all(test, feature = "spec-retrieval"))]
mod tests {
    use super::*;

    #[test]
    fn nli_evidence_averages_directional_logits_before_softmax() {
        let evidence = nli_evidence([4.0, 0.0, 0.0], [0.0, 4.0, 0.0]);

        assert!(evidence.forward[0] > 0.9);
        assert!(evidence.reverse[1] > 0.9);
        assert!(evidence.symmetric[0] > 0.45);
        assert!(evidence.symmetric[1] > 0.45);
        assert!((evidence.symmetric.iter().sum::<f32>() - 1.0).abs() < 1e-5);
    }

    #[test]
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
        let delta_root = root.join("givn/changes/spec-duplicates/specs/spec-duplicates");
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
                score < SEMANTIC_SIMILARITY_THRESHOLD,
                "boundary pair remained above threshold: {left_title} <> {right_title} = {score:.4}"
            );
        }
    }
}
