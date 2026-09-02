#[cfg(test)]
use crate::config::manifest::Coverage;
use crate::config::manifest::{Artifact, HookEntry, Manifest};
use crate::config::overlay::load_effective;
use crate::state::{ArtifactStatus, ChangeState, topological_sort};
use anyhow::Result;
use std::collections::HashMap;

/// Show the artifact workflow graph: nodes, dependency (`requires`) edges,
/// optional/gates-archive flags, and post hooks.
///
/// Without --change: prints the general workflow derived from the manifest
/// alone (no filesystem/change context).
/// With --change <id>: prints the same graph, annotated with each artifact's
/// live status for that change.
/// With --json: machine-readable output for use by agents/commands.
pub fn run(change: Option<&str>, json: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("givn").exists() {
        anyhow::bail!("givn/ not found. Run `givn init` first.");
    }

    let manifest = load_effective()?;

    match change {
        Some(id) => {
            let change_dir = crate::state::resolve_change_dir(&project_root, id)?;
            if !change_dir.exists() {
                let archived_dir = project_root.join("givn/archive").join(id);
                if archived_dir.exists() {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "id": id,
                                "archived": true,
                            }))?
                        );
                    } else {
                        println!("Change '{id}': archived");
                    }
                    return Ok(());
                }
                anyhow::bail!("change '{}' not found", id);
            }
            let state = ChangeState::from_manifest(&change_dir, &manifest);
            if json {
                print_json(&manifest, Some(&state))?;
            } else {
                print_graph(&manifest, Some(&state));
            }
        }
        None => {
            if json {
                print_json(&manifest, None)?;
            } else {
                print_graph(&manifest, None);
            }
        }
    }

    Ok(())
}

fn hook_names(post: &[HookEntry]) -> Vec<&str> {
    post.iter()
        .map(|h| match h {
            HookEntry::Short(s) => s.as_str(),
            HookEntry::Full(f) => f.run.as_str(),
        })
        .collect()
}

fn status_for<'a>(state: Option<&'a ChangeState>, id: &str) -> Option<&'a ArtifactStatus> {
    state.and_then(|s| s.artifacts.iter().find(|a| a.id == id).map(|a| &a.status))
}

fn status_marker(status: &ArtifactStatus) -> &'static str {
    match status {
        ArtifactStatus::Done => "✓",
        ArtifactStatus::Partial => "~",
        ArtifactStatus::Pending => "·",
        ArtifactStatus::Skipped => "⊘",
    }
}

// ─── Human-readable output ───────────────────────────────────────────────────

fn print_graph(manifest: &Manifest, state: Option<&ChangeState>) {
    match state {
        Some(s) => {
            println!("Change: {}", s.id);
            println!("Stage:  {}", s.stage_label());
        }
        None => println!("Workflow:"),
    }
    println!();

    let ordered = topological_sort(&manifest.artifacts);
    let gates_archive: std::collections::HashSet<&str> =
        manifest.apply.requires.iter().map(|s| s.as_str()).collect();

    for a in &ordered {
        let marker = status_for(state, &a.id).map(status_marker).unwrap_or(" ");
        let mut tags = Vec::new();
        if gates_archive.contains(a.id.as_str()) {
            tags.push("gates archive".to_string());
        }
        if a.optional {
            tags.push("optional".to_string());
        }
        let hooks = hook_names(&a.post);
        if !hooks.is_empty() {
            tags.push(format!("post: {}", hooks.join(", ")));
        }
        let tag_str = if tags.is_empty() {
            String::new()
        } else {
            format!("  ({})", tags.join(", "))
        };
        let requires_str = if a.requires.is_empty() {
            String::new()
        } else {
            format!("\n      requires: {}", a.requires.join(", "))
        };
        println!("  {marker} {}{tag_str}{requires_str}", a.id);
    }

    if let Some(s) = state {
        println!();
        match s.next_required() {
            None => println!("All required artifacts complete. Ready to archive."),
            Some(next) => println!("Next required:  {}", next.id),
        }
    }
}

// ─── JSON output ─────────────────────────────────────────────────────────────

fn print_json(manifest: &Manifest, state: Option<&ChangeState>) -> Result<()> {
    let ordered = topological_sort(&manifest.artifacts);
    let gates_archive: std::collections::HashSet<&str> =
        manifest.apply.requires.iter().map(|s| s.as_str()).collect();
    let status_by_id: HashMap<&str, &ArtifactStatus> = state
        .map(|s| {
            s.artifacts
                .iter()
                .map(|a| (a.id.as_str(), &a.status))
                .collect()
        })
        .unwrap_or_default();

    let nodes: Vec<serde_json::Value> = ordered
        .iter()
        .map(|a: &&Artifact| {
            let mut v = serde_json::json!({
                "id": a.id,
                "requires": a.requires,
                "optional": a.optional,
                "gates_archive": gates_archive.contains(a.id.as_str()),
                "post": hook_names(&a.post),
            });
            if let Some(status) = status_by_id.get(a.id.as_str()) {
                let status_str = match status {
                    ArtifactStatus::Done => "done",
                    ArtifactStatus::Partial => "partial",
                    ArtifactStatus::Pending => "pending",
                    ArtifactStatus::Skipped => "skipped",
                };
                v["status"] = serde_json::json!(status_str);
            }
            v
        })
        .collect();

    let mut out = serde_json::json!({ "artifacts": nodes });
    if let Some(s) = state {
        out["id"] = serde_json::json!(s.id);
        out["stage"] = serde_json::json!(s.stage_label());
        out["all_required_done"] = serde_json::json!(s.all_required_done());
        out["next_required"] = s
            .next_required()
            .map(|a| serde_json::json!({"id": a.id, "optional": a.optional}))
            .unwrap_or(serde_json::Value::Null);
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::manifest::{Addons, Apply, HookFull, Run, Verify};
    use crate::state::ArtifactStatus;

    fn artifact(id: &str, requires: &[&str], optional: bool, post: Vec<HookEntry>) -> Artifact {
        Artifact {
            id: id.to_string(),
            generates: vec![],
            requires: requires.iter().map(|s| s.to_string()).collect(),
            template: None,
            instruction: None,
            optional,
            remove: false,
            post,
        }
    }

    fn sample_manifest() -> Manifest {
        Manifest {
            artifacts: vec![
                artifact("design", &[], false, vec![]),
                artifact("arc42-docs", &["design"], false, vec![]),
                artifact("design-review", &["design", "arc42-docs"], false, vec![]),
                artifact(
                    "review",
                    &["design-review"],
                    false,
                    vec![HookEntry::Short("verify".to_string())],
                ),
            ],
            apply: Apply {
                requires: vec!["arc42-docs".to_string(), "review".to_string()],
                tracks: None,
            },
            verify: Verify {
                command: "true".to_string(),
                e2e_command: String::new(),
                features_path: "givn/specs".to_string(),
            },
            run: Run::default(),
            addons: Addons {
                arc42: true,
                ..Default::default()
            },
            coverage: Coverage::default(),
            givn_config_version: None,
            extras: false,
        }
    }

    #[test]
    fn hook_names_resolves_short_and_full_entries() {
        let post = vec![
            HookEntry::Short("verify".to_string()),
            HookEntry::Full(HookFull {
                run: "verify-e2e".to_string(),
                allow_failure: false,
                name: None,
            }),
        ];
        assert_eq!(hook_names(&post), vec!["verify", "verify-e2e"]);
    }

    #[test]
    fn status_marker_matches_each_variant() {
        assert_eq!(status_marker(&ArtifactStatus::Done), "✓");
        assert_eq!(status_marker(&ArtifactStatus::Partial), "~");
        assert_eq!(status_marker(&ArtifactStatus::Pending), "·");
        assert_eq!(status_marker(&ArtifactStatus::Skipped), "⊘");
    }

    #[test]
    fn manifest_json_exposes_requires_edges_and_gates_archive() {
        let manifest = sample_manifest();
        let ordered = topological_sort(&manifest.artifacts);
        let gates_archive: std::collections::HashSet<&str> =
            manifest.apply.requires.iter().map(|s| s.as_str()).collect();

        let design_review = ordered
            .iter()
            .find(|a| a.id == "design-review")
            .expect("design-review present");
        assert_eq!(design_review.requires, vec!["design", "arc42-docs"]);
        assert!(!gates_archive.contains("design-review"));

        let arc42 = ordered
            .iter()
            .find(|a| a.id == "arc42-docs")
            .expect("arc42-docs present");
        assert!(gates_archive.contains(arc42.id.as_str()));
    }

    #[test]
    fn print_json_smoke_test_without_change() {
        let manifest = sample_manifest();
        // Just confirm it doesn't panic and produces valid JSON structure.
        print_json(&manifest, None).expect("prints valid json");
    }
}
