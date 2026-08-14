use std::path::Path;

use anyhow::{Context, Result, bail};

#[cfg(feature = "spec-retrieval")]
use std::collections::hash_map::DefaultHasher;
#[cfg(feature = "spec-retrieval")]
use std::hash::{Hash, Hasher};
#[cfg(feature = "spec-retrieval")]
use std::fs;

#[cfg(feature = "spec-retrieval")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "spec-retrieval")]
const INDEX_VERSION: u32 = 1;
#[cfg(feature = "spec-retrieval")]
const MODEL_ID: &str = "intfloat/multilingual-e5-large";

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
#[derive(Debug, Clone, Copy)]
enum EmbeddingRole {
    Passage,
    Query,
}

pub fn index(root: &Path) -> Result<()> {
    #[cfg(feature = "spec-retrieval")]
    {
        return index_enabled(root);
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
        return search_enabled(root, query);
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
        return review_enabled(root, change_id);
    }

    #[cfg(not(feature = "spec-retrieval"))]
    {
        let _ = (root, change_id);
        Ok(())
    }
}

#[cfg(feature = "spec-retrieval")]
fn index_enabled(root: &Path) -> Result<()> {
    let entries = collect_scenarios(root)?;
    let index_path = root.join("givn/.spec-index.json");
    let previous = read_index(&index_path).unwrap_or_else(|_| SpecIndex {
        version: INDEX_VERSION,
        model: MODEL_ID.to_string(),
        entries: Vec::new(),
    });

    let mut result = Vec::with_capacity(entries.len());
    let mut pending = Vec::new();
    let mut reused = 0usize;

    for scenario in entries {
        if let Some(existing) = previous.entries.iter().find(|entry| {
            entry.title == scenario.title
                && entry.file == scenario.file
                && entry.content_hash == scenario.content_hash
        }) {
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
    let index_path = root.join("givn/.spec-index.json");
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

    for (rank, (entry, score)) in ranked.into_iter().take(5).enumerate() {
        println!(
            "rank {}: {} ({}, score {:.4})",
            rank + 1,
            entry.title,
            entry.file,
            score
        );
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn review_enabled(root: &Path, change_id: &str) -> Result<()> {
    index_enabled(root)?;
    let index = read_index(&root.join("givn/.spec-index.json"))?;
    let delta_root = root.join("givn/changes").join(change_id).join("specs");
    if !delta_root.exists() {
        return Ok(());
    }

    let review_path = root
        .join("givn/changes")
        .join(change_id)
        .join("review.md");
    let (dispositions, _) = crate::overlap::disposition::parse_review(&review_path);
    for scenario in collect_scenarios_from(root, &delta_root)? {
        let query_vector = embed_texts(root, &[scenario.text], EmbeddingRole::Query)?
            .into_iter()
            .next()
            .context("retrieval embedder returned no review vector")?;
        let ranked = rank_entries(&index.entries, &query_vector);
        for (rank, (entry, score)) in ranked.into_iter().take(5).enumerate() {
            if crate::overlap::disposition::disposed(
                &dispositions,
                &scenario.title,
                &entry.title,
            ) {
                continue;
            }
            println!(
                "retrieval advisory: '{}' resembles '{}' (rank {}, score {:.4})",
                scenario.title,
                entry.title,
                rank + 1,
                score
            );
        }
    }
    Ok(())
}

#[cfg(feature = "spec-retrieval")]
fn collect_scenarios(root: &Path) -> Result<Vec<ScenarioText>> {
    collect_scenarios_from(root, &root.join("givn/specs"))
}

#[cfg(feature = "spec-retrieval")]
fn collect_scenarios_from(root: &Path, specs_root: &Path) -> Result<Vec<ScenarioText>> {
    let mut scenarios = Vec::new();
    for path in crate::gherkin::parse::collect_feature_files(&specs_root) {
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
fn collect_feature_scenarios(
    source: &[gherkin::Scenario],
    file: &str,
    output: &mut Vec<ScenarioText>,
) {
    for scenario in source {
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
        output.push(ScenarioText {
            title: scenario.name.clone(),
            file: file.to_string(),
            content_hash: content_hash(&normalized),
            text: normalized,
        });
    }
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
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
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
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        let index = (hasher.finish() as usize) % DIMENSIONS;
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
    left.iter()
        .zip(right)
        .map(|(a, b)| a * b)
        .sum::<f32>()
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
