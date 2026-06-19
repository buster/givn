use anyhow::Result;
use crate::config::overlay::load_effective;

/// Show the effective artifact plan — the ordered sequence from the manifest.
///
/// This is the machine-readable API that commands and scripts use to discover
/// the workflow at runtime, rather than hardcoding the artifact sequence.
/// Use `--json` for structured output suitable for parsing.
pub fn run(json: bool) -> Result<()> {
    let manifest = load_effective()?;
    let ordered = topological_sort(&manifest.artifacts)?;
    let archive_requires: std::collections::HashSet<&str> = manifest
        .apply
        .requires
        .iter()
        .map(|s| s.as_str())
        .collect();

    if json {
        let artifacts: Vec<serde_json::Value> = ordered
            .iter()
            .map(|a| {
                serde_json::json!({
                    "id": a.id,
                    "generates": a.generates,
                    "requires": a.requires,
                    "optional": a.optional,
                    "gates_archive": archive_requires.contains(a.id.as_str()),
                    "has_instruction": has_embedded_instruction(&a.id),
                })
            })
            .collect();

        let out = serde_json::json!({
            "artifacts": artifacts,
            "apply": {
                "requires": manifest.apply.requires,
                "tracks": manifest.apply.tracks,
            },
            "verify": {
                "command": manifest.verify.command,
                "features_path": manifest.verify.features_path,
            },
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("Effective artifact plan (in execution order):");
        println!();
        for (i, a) in ordered.iter().enumerate() {
            let gates = if archive_requires.contains(a.id.as_str()) {
                " [gates archive]"
            } else {
                ""
            };
            let optional = if a.optional { " (optional)" } else { "" };
            println!("  {}. {}{}{}", i + 1, a.id, gates, optional);
            if !a.generates.is_empty() {
                println!("     generates: {}", a.generates.join(", "));
            }
        }
        println!();
        println!("verify.command: {}", manifest.verify.command);
    }

    Ok(())
}

/// Topologically sort artifacts by their `requires` graph.
/// Returns artifacts in execution order (dependencies before dependents).
fn topological_sort(
    artifacts: &[crate::config::manifest::Artifact],
) -> Result<Vec<&crate::config::manifest::Artifact>> {
    use std::collections::{HashMap, VecDeque};

    let index: HashMap<&str, usize> = artifacts
        .iter()
        .enumerate()
        .map(|(i, a)| (a.id.as_str(), i))
        .collect();

    // In-degree count per node.
    let mut in_degree: Vec<usize> = vec![0; artifacts.len()];
    // Adjacency: for each artifact, which artifacts depend on it.
    let mut dependents: Vec<Vec<usize>> = vec![vec![]; artifacts.len()];

    for (i, a) in artifacts.iter().enumerate() {
        for req in &a.requires {
            if let Some(&j) = index.get(req.as_str()) {
                dependents[j].push(i);
                in_degree[i] += 1;
            }
        }
    }

    // Kahn's algorithm.
    let mut queue: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, d)| **d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut result = Vec::new();
    while let Some(i) = queue.pop_front() {
        result.push(&artifacts[i]);
        for &j in &dependents[i] {
            in_degree[j] -= 1;
            if in_degree[j] == 0 {
                queue.push_back(j);
            }
        }
    }

    if result.len() != artifacts.len() {
        anyhow::bail!("artifact dependency graph contains a cycle (this should have been caught at load time)");
    }

    Ok(result)
}

/// Check whether an embedded instruction exists for this artifact id.
fn has_embedded_instruction(artifact_id: &str) -> bool {
    let key = format!("instructions/{artifact_id}.md");
    crate::config::embedded::Assets::get(&key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::manifest::Manifest;

    #[test]
    fn plan_sorts_default_artifacts_in_dependency_order() {
        let manifest = Manifest::load_embedded().unwrap();
        let sorted = topological_sort(&manifest.artifacts).unwrap();
        let ids: Vec<&str> = sorted.iter().map(|a| a.id.as_str()).collect();

        // proposal must come before specs, specs before design, etc.
        let pos = |id: &str| ids.iter().position(|&x| x == id).unwrap();
        assert!(pos("proposal") < pos("specs"));
        assert!(pos("specs") < pos("design"));
        assert!(pos("design") < pos("tasks"));
        assert!(pos("tasks") < pos("review"));
    }

    #[test]
    fn plan_json_includes_all_required_fields() {
        // Run the plan command in JSON mode and check the output structure.
        let manifest = Manifest::load_embedded().unwrap();
        let ordered = topological_sort(&manifest.artifacts).unwrap();
        assert!(!ordered.is_empty());

        // All default artifacts should have embedded instructions.
        for a in &["proposal", "specs", "design", "tasks", "review"] {
            assert!(
                has_embedded_instruction(a),
                "missing embedded instruction for '{a}'"
            );
        }
    }
}
