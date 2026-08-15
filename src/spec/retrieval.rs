use std::path::Path;

use anyhow::{Result, bail};

#[cfg(feature = "spec-retrieval")]
use anyhow::Context;

#[cfg(feature = "spec-retrieval")]
use std::collections::BTreeMap;

#[cfg(feature = "spec-retrieval")]
use std::fs;

#[cfg(feature = "spec-retrieval")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "spec-retrieval")]
const INDEX_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const MODEL_ID: &str = "intfloat/multilingual-e5-large";
#[cfg(feature = "spec-retrieval")]
const SIMILARITY_THRESHOLD: f32 = 0.95;

#[cfg(feature = "spec-retrieval")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpecIndex {
    version: u32,
    model: String,
    entries: Vec<IndexEntry>,
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
#[derive(Debug, Clone)]
struct DeltaScenario {
    scenario: ScenarioText,
    target_file: String,
    operation: crate::gherkin::delta::DeltaOp,
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
    score: f32,
    scopes: Vec<&'static str>,
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

#[cfg(feature = "spec-retrieval")]
fn index_enabled(root: &Path) -> Result<()> {
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

    println!("Indexed {} permanent scenarios", index.entries.len());
    println!("Reused {} scenario vectors", reused);
    Ok(())
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
fn review_enabled(root: &Path, change_id: &str) -> Result<()> {
    let change_dir = validated_review_change_dir(root, change_id)?;
    let delta_root = delta_specs_path(&change_dir, change_id)?;
    index_enabled(root)?;
    let index = read_index(&spec_index_path(root))?;

    let delta = collect_delta_scenarios(root, &delta_root)?;
    let candidates = similarity_candidates(root, &index, &delta)?;
    for candidate in &candidates {
        println!(
            "similarity candidate [{}]: '{}' ({}) resembles '{}' ({}) score {:.4}, threshold {:.4}; remediation: MERGE (delete the true duplicate) or KEEP-AS-BOUNDARY (rewrite the retained scenario below the threshold)",
            candidate.scopes.join(","),
            candidate.left.title,
            candidate.left.file,
            candidate.right.title,
            candidate.right.file,
            candidate.score,
            SIMILARITY_THRESHOLD,
        );
    }
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
fn collect_scenarios_from(root: &Path, specs_root: &Path) -> Result<Vec<ScenarioText>> {
    let mut scenarios = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(specs_root) {
        let feature = gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default())
            .with_context(|| format!("failed to parse permanent spec {}", path.display()))?;
        let file = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
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
        let feature = gherkin::Feature::parse_path(&path, gherkin::GherkinEnv::default())
            .with_context(|| format!("failed to parse delta spec {}", path.display()))?;
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
        let target_file = format!("givn/specs/{capability}/{capability}.feature");
        collect_delta_feature_scenarios(&feature.scenarios, &path, &target_file, &mut scenarios);
        for rule in &feature.rules {
            collect_delta_feature_scenarios(&rule.scenarios, &path, &target_file, &mut scenarios);
        }
    }
    scenarios.sort_by(|left, right| {
        left.scenario
            .file
            .cmp(&right.scenario.file)
            .then_with(|| left.scenario.title.cmp(&right.scenario.title))
    });
    let _ = root;
    Ok(scenarios)
}

#[cfg(feature = "spec-retrieval")]
fn collect_delta_feature_scenarios(
    source: &[gherkin::Scenario],
    path: &Path,
    target_file: &str,
    output: &mut Vec<DeltaScenario>,
) {
    let file = path.to_string_lossy().replace('\\', "/");
    for scenario in source {
        output.push(DeltaScenario {
            scenario: scenario_text(scenario, &file),
            target_file: target_file.to_string(),
            operation: crate::gherkin::delta::delta_op(scenario),
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
fn scenario_text(scenario: &gherkin::Scenario, file: &str) -> ScenarioText {
    let mut text = scenario.name.clone();
    for step in &scenario.steps {
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
fn is_queryable_scenario(scenario: &gherkin::Scenario) -> bool {
    !scenario.tags.iter().any(|tag| tag == "givn.removed")
}

#[cfg(feature = "spec-retrieval")]
fn similarity_candidates(
    root: &Path,
    index: &SpecIndex,
    delta: &[DeltaScenario],
) -> Result<Vec<SimilarityCandidate>> {
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

    let projected = project_entries(index, delta, &delta_vectors, &active_delta)?;
    for (left, right) in projected.iter().enumerate() {
        for other in projected.iter().skip(left + 1) {
            let score = cosine_similarity(&right.embedding, &other.embedding);
            if score >= SIMILARITY_THRESHOLD {
                add_similarity_candidate(
                    &mut candidates,
                    "projected-permanent",
                    endpoint(&right.file, &right.title),
                    endpoint(&other.file, &other.title),
                    score,
                );
            }
        }
    }

    let mut result: Vec<SimilarityCandidate> = candidates.into_values().collect();
    result.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| left.left.cmp(&right.left))
            .then_with(|| left.right.cmp(&right.right))
    });
    Ok(result)
}

#[cfg(feature = "spec-retrieval")]
fn project_entries(
    index: &SpecIndex,
    delta: &[DeltaScenario],
    delta_vectors: &[Vec<f32>],
    active_delta: &[&DeltaScenario],
) -> Result<Vec<IndexEntry>> {
    let mut entries: BTreeMap<(String, String), IndexEntry> = index
        .entries
        .iter()
        .cloned()
        .map(|entry| ((entry.file.clone(), entry.title.clone()), entry))
        .collect();
    let mut pending: BTreeMap<(String, String), IndexEntry> = BTreeMap::new();

    for item in delta {
        let key = (item.target_file.clone(), item.scenario.title.clone());
        match item.operation {
            crate::gherkin::delta::DeltaOp::Removed => {
                entries.remove(&key);
                pending.remove(&key);
            }
            crate::gherkin::delta::DeltaOp::Modified => {
                entries.remove(&key);
                pending.remove(&key);
                let position = active_delta
                    .iter()
                    .position(|active| std::ptr::eq(*active, item))
                    .context("modified delta vector is missing")?;
                pending.insert(
                    key,
                    IndexEntry {
                        title: item.scenario.title.clone(),
                        file: item.target_file.clone(),
                        content_hash: item.scenario.content_hash.clone(),
                        embedding: delta_vectors[position].clone(),
                    },
                );
            }
            crate::gherkin::delta::DeltaOp::Added => {
                if entries.contains_key(&key) || pending.contains_key(&key) {
                    bail!(
                        "similarity projection cannot add existing scenario '{}' in {}",
                        item.scenario.title,
                        item.target_file
                    );
                }
                let position = active_delta
                    .iter()
                    .position(|active| std::ptr::eq(*active, item))
                    .context("added delta vector is missing")?;
                pending.insert(
                    key,
                    IndexEntry {
                        title: item.scenario.title.clone(),
                        file: item.target_file.clone(),
                        content_hash: item.scenario.content_hash.clone(),
                        embedding: delta_vectors[position].clone(),
                    },
                );
            }
        }
    }

    entries.extend(pending);
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
        if score > existing.score {
            existing.score = score;
        }
        return;
    }
    candidates.insert(
        key,
        SimilarityCandidate {
            left,
            right,
            score,
            scopes: vec![scope],
        },
    );
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
    Ok(index)
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
