use crate::config::declaration::{DECLARATION_PATH, Role, SpecificationDeclaration};
use crate::gherkin::delta::is_removed_placeholder;
use crate::gherkin::parse::{LintKind, collect_feature_files, parse_file};
use crate::overlap::analysis::Finding;
use anyhow::Result;

/// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
///
/// Exit codes:
///   0 = clean
///   1 = parse error or a refused specification declaration
///   2 = @wip / PENDING
///   3 = duplicate scenario title (overlap hard error) or empty feature
///
/// Parse errors (1) outrank duplicate titles/empty features (3), which
/// outrank @wip (2).
/// `@givn.removed` scenarios suppress @wip warnings (they contain placeholder steps).
///
/// Returns the exit code; the caller (`main`) maps non-zero codes to
/// `std::process::exit`.
pub fn run(change: Option<&str>, paths: Vec<String>, json: bool) -> Result<i32> {
    let project_root = std::env::current_dir()?;
    run_with_root(&project_root, change, paths, json)
}

/// Run the change-scoped lint gate and refuse a non-zero exit.
///
/// Used by `givn check review --change`, where a duplicate title, an invalid
/// use-case document, or an interaction mismatch is a blocking finding.
pub(crate) fn run_scoped(project_root: &std::path::Path, change: &str) -> Result<()> {
    let code = run_with_root(project_root, Some(change), Vec::new(), false)?;
    if code != 0 {
        anyhow::bail!(
            "lint gate failed for change '{change}' (exit code {code}); fix the findings above"
        );
    }
    Ok(())
}

fn run_with_root(
    project_root: &std::path::Path,
    change: Option<&str>,
    paths: Vec<String>,
    json: bool,
) -> Result<i32> {
    // Advisory hygiene checks must not change the exit-code contract, so they
    // are rendered and counted separately from the blocking flags.
    let default_scope = paths.is_empty() && change.is_none();
    let change_scope = paths.is_empty() && change.is_some();
    let mut findings: Vec<Finding> = if default_scope {
        check_seed_status(project_root)
    } else {
        Vec::new()
    };
    if default_scope || change_scope {
        findings.extend(check_persona_references(project_root, change));
    }
    let advisory_count = findings.len();

    // The declaration is read before the empty-feature shortcut, so a malformed
    // or absent declaration is refused on a project with no `.feature` file. A
    // refused declaration is the only finding: the advisory pass would read a
    // partial role set. `ise` off reads nothing, whatever sits on disk.
    if default_scope {
        match specification_health(project_root)? {
            SpecificationHealth::Blocking(finding) => {
                if json {
                    println!("{}", findings_json(std::slice::from_ref(&finding))?);
                } else {
                    render_prose(&finding);
                }
                return Ok(1);
            }
            SpecificationHealth::Advisories(declaration_findings) => {
                findings.extend(declaration_findings);
            }
        }
    }

    let feature_files = if !paths.is_empty() {
        paths.iter().map(std::path::PathBuf::from).collect()
    } else if let Some(id) = change {
        // Scope to givn/changes/<id>/specs/
        let change_dir = crate::state::resolve_change_dir(project_root, id)?;
        if !change_dir.exists() {
            anyhow::bail!("change '{id}' not found");
        }
        collect_feature_files(&change_dir.join("specs"))
    } else {
        // Default: check all .feature files under givn/ in the project,
        // excluding the archive tree (historical, no longer executable).
        let archive_root = project_root.join("givn/archive");
        collect_feature_files(&project_root.join("givn"))
            .into_iter()
            .filter(|p| !p.starts_with(&archive_root))
            .collect::<Vec<_>>()
    };

    if feature_files.is_empty() && paths.is_empty() {
        let scope = match change {
            Some(id) => format!("givn/changes/{id}/specs/"),
            None => "givn/".to_string(),
        };
        if change.is_none() {
            if json {
                println!("{}", findings_json(&findings)?);
            } else {
                for finding in &findings {
                    render_prose(finding);
                }
                println!("givn lint: no .feature files found under {scope}");
            }
            return Ok(0);
        }
        // A change with a narrative-only delta still projects. Keep going so
        // the interaction inventory is checked before the empty-feature shortcut.
        if !json {
            println!("givn lint: no .feature files found under {scope}");
        }
    }

    let mut has_parse_error = false;
    let mut has_wip = false;
    let mut has_duplicate = false;
    let mut has_empty_feature = false;
    let mut has_use_case_error = false;
    let mut total_files = 0;
    let mut total_errors = advisory_count;

    for path in &feature_files {
        total_files += 1;
        let result = parse_file(path);

        // Filter out @wip findings from @givn.removed scenarios.
        let errors: Vec<_> = result
            .errors
            .iter()
            .filter(|e| {
                if e.kind == LintKind::Wip {
                    // Check if it belongs to a @givn.removed scenario — suppress if so.
                    // We do this by checking if the feature parsed and has a removed scenario
                    // at the same line.
                    if let Some(ref feature) = result.feature {
                        let wip_line = e.line.unwrap_or(0);
                        let is_in_removed = feature
                            .scenarios
                            .iter()
                            .any(|s| is_removed_placeholder(s) && s.position.line == wip_line);
                        if is_in_removed {
                            return false; // suppress
                        }
                    }
                }
                true
            })
            .collect();

        for error in &errors {
            total_errors += 1;
            let kind = match error.kind {
                LintKind::ParseError => {
                    has_parse_error = true;
                    "parse_error"
                }
                LintKind::Wip => {
                    has_wip = true;
                    "wip"
                }
            };
            findings.push(Finding {
                kind: kind.to_string(),
                level: if kind == "parse_error" {
                    "error"
                } else {
                    "warning"
                }
                .to_string(),
                files: vec![path.display().to_string()],
                titles: vec![error.message.clone()],
                contradiction: false,
            });
        }

        if let Some(feature) = &result.feature
            && feature.scenarios.is_empty()
            && !crate::gherkin::delta::is_retired_feature(feature)
        {
            has_empty_feature = true;
            total_errors += 1;
            findings.push(Finding {
                kind: "empty_feature".to_string(),
                level: "error".to_string(),
                files: vec![path.display().to_string()],
                titles: vec!["feature has zero scenarios".to_string()],
                contradiction: false,
            });
        }
    }

    // Cross-reference tasks.md for @e2e intent: if tasks.md plans e2e but no
    // @e2e-tagged scenarios exist in the specs, the tags may have been removed.
    if let Some(id) = change
        && !has_parse_error
    {
        let change_dir = crate::state::resolve_change_dir(project_root, id)?;
        let tasks_path = change_dir.join("tasks.md");
        if tasks_path.exists() {
            let tasks_content = std::fs::read_to_string(&tasks_path).unwrap_or_default();
            if crate::hooks::has_e2e_intent_in_tasks(&tasks_content) {
                let specs_dir = change_dir.join("specs");
                let has_e2e = crate::gherkin::parse::has_e2e_scenarios(&specs_dir);
                if !has_e2e {
                    let message = "@e2e scenarios referenced in tasks.md but none found in specs/";
                    if json {
                        findings.push(Finding {
                            kind: "e2e_intent".to_string(),
                            level: "warning".to_string(),
                            files: vec![specs_dir.display().to_string()],
                            titles: vec![message.to_string()],
                            contradiction: false,
                        });
                    } else {
                        eprintln!("[WARN] finding  {}  {}", specs_dir.display(), message);
                        eprintln!(
                            "       Restore @e2e tags or remove the e2e tasks from tasks.md."
                        );
                    }
                    total_errors += 1;
                    has_wip = true;
                }
            }
        }
    }

    // Tree-wide overlap scan. Whole-tree scope by default; with --change,
    // run the same scan but keep only findings that involve the change's
    // delta (same delta-prefix filter as run_review_overlap_gate), so the
    // authoring loop surfaces overlap findings before check review does.
    if paths.is_empty() && !has_parse_error {
        let all_findings = crate::overlap::analysis::scan_tree(project_root);
        let scoped_findings: Vec<_> = if let Some(id) = change {
            let change_dir = crate::state::resolve_change_dir(project_root, id)?;
            let delta_prefix = change_dir.join("specs").to_string_lossy().into_owned();
            all_findings
                .into_iter()
                .filter(|f| f.files.iter().any(|file| file.starts_with(&delta_prefix)))
                .collect()
        } else {
            all_findings
        };
        for finding in scoped_findings {
            if finding.kind == "duplicate_title" {
                total_errors += 1;
                has_duplicate = true;
            }
            if matches!(
                finding.kind.as_str(),
                "usecase_invalid"
                    | "fragment_invalid"
                    | "dangling_capability_ref"
                    | "ambiguous_relationship_target"
                    | "dangling_use_case_ref"
                    | "use_case_cycle"
                    | "extend_fragment"
                    | "interaction_mismatch"
                    | "usecase_interaction_invalid"
                    | "undeclared_capability"
            ) {
                total_errors += 1;
                has_use_case_error = true;
            }
            findings.push(finding);
        }

        // Reuse the same in-memory projection for --change. It reports only
        // the projected corpus; the delta scan above owns syntax and schema
        // findings, so skip the projection when one is already blocking.
        if let Some(id) = change
            && !has_use_case_error
        {
            for finding in crate::spec::projection::interaction_findings(project_root, id)? {
                total_errors += 1;
                has_use_case_error = true;
                findings.push(finding);
            }
        }
    }

    if json {
        println!("{}", findings_json(&findings)?);
    } else {
        for finding in &findings {
            render_prose(finding);
        }
        // Summary.
        if total_errors == 0 {
            println!("givn lint: {total_files} file(s) checked — clean");
        } else {
            eprintln!("givn lint: {total_files} file(s) checked, {total_errors} finding(s)");
        }
    }

    // Exit code: parse errors (1) outrank duplicate titles and empty
    // features (3), which outrank @wip/PENDING (2).
    let exit_code = if has_parse_error || has_use_case_error {
        1
    } else if has_duplicate || has_empty_feature {
        3
    } else if has_wip {
        2
    } else {
        0
    };
    Ok(exit_code)
}

/// The outcome of the default-scope specification pass. A refused declaration
/// is the only finding; otherwise the four advisory finding kinds are computed
/// from the complete role set.
enum SpecificationHealth {
    Blocking(Finding),
    Advisories(Vec<Finding>),
}

/// Read the declaration once and report its health. Runs only in the default
/// project scope, and only while `ise` is effective: nothing is read while the
/// addon is off, whatever sits on disk.
fn specification_health(project_root: &std::path::Path) -> Result<SpecificationHealth> {
    let manifest = crate::config::overlay::load_effective_at(project_root)?;
    if !SpecificationDeclaration::is_effective(&manifest) {
        return Ok(SpecificationHealth::Advisories(Vec::new()));
    }
    let declaration = match SpecificationDeclaration::load(project_root) {
        Ok(Some(declaration)) => declaration,
        Ok(None) => {
            return Ok(SpecificationHealth::Blocking(declaration_finding(
                "the declaration is absent; the declaration checks were not performed",
            )));
        }
        Err(error) => {
            return Ok(SpecificationHealth::Blocking(declaration_finding(
                &error.detail(),
            )));
        }
    };
    Ok(SpecificationHealth::Advisories(specification_findings(
        project_root,
        &declaration,
    )))
}

/// The blocking declaration finding. Its location is the declaration file the
/// developer must write or repair.
fn declaration_finding(message: &str) -> Finding {
    Finding {
        kind: "specification_declaration".to_string(),
        level: "error".to_string(),
        files: vec![DECLARATION_PATH.to_string()],
        titles: vec![message.to_string()],
        contradiction: false,
    }
}

/// One project-relative declaration finding, located at the declaration and
/// the path or package it is about.
fn declaration_advisory(kind: &str, subject: &str, message: String) -> Finding {
    Finding {
        kind: kind.to_string(),
        level: "warning".to_string(),
        files: vec![DECLARATION_PATH.to_string(), subject.to_string()],
        titles: vec![message],
        contradiction: false,
    }
}

/// The four advisory findings. Findings iterate declared non-empty lists only,
/// so the empty scaffold is silent, and `e2e` is excluded from the generic
/// path-missing finding: its absent package is reported once by the boundary
/// finding with the role entry and the reason.
fn specification_findings(
    project_root: &std::path::Path,
    declaration: &SpecificationDeclaration,
) -> Vec<Finding> {
    let roles = declaration.roles();
    if roles.iter().all(|(_, paths)| paths.is_empty()) {
        return Vec::new();
    }
    let tree = walk_project(project_root);
    let mut findings = Vec::new();

    for (role, declared) in declaration.roles() {
        if role == Role::E2e {
            continue;
        }
        for pattern in declared {
            let glob = Glob::compile(pattern);
            if !tree.iter().any(|entry| glob.matches(&entry.path)) {
                findings.push(declaration_advisory(
                    "specification_path_missing",
                    pattern,
                    format!(
                        "the '{}' role declares '{pattern}', which matches no path in the project",
                        role.as_str()
                    ),
                ));
            }
        }
    }

    // One finding per duplicated string, not one per role pair: three roles
    // declaring the same path is one ambiguous ownership, not three.
    let mut ownership: Vec<(&str, Vec<&str>)> = Vec::new();
    for (role, declared) in declaration.roles() {
        for pattern in declared {
            match ownership
                .iter_mut()
                .find(|(candidate, _)| *candidate == pattern.as_str())
            {
                Some((_, roles)) => {
                    if !roles.contains(&role.as_str()) {
                        roles.push(role.as_str());
                    }
                }
                None => ownership.push((pattern.as_str(), vec![role.as_str()])),
            }
        }
    }
    for (pattern, roles) in ownership {
        if roles.len() < 2 {
            continue;
        }
        let named = roles.join("', '");
        findings.push(declaration_advisory(
            "specification_role_duplicate",
            pattern,
            format!(
                "duplicate declaration: '{pattern}' is declared by '{named}'; a path belongs to one role"
            ),
        ));
    }

    // A behavior tree with no `.feature` file is a declared but empty role, not
    // an error: only a corpus with features can leave one unretained.
    let behavior = declaration
        .roles()
        .into_iter()
        .find(|(role, _)| *role == Role::Behavior)
        .map(|(_, paths)| paths)
        .unwrap_or(&[]);
    let behavior_globs: Vec<Glob> = behavior
        .iter()
        .map(|pattern| Glob::compile(pattern))
        .collect();
    if !behavior_globs.is_empty() {
        for entry in &tree {
            if entry.is_dir || !is_feature_file(&entry.path) {
                continue;
            }
            if behavior_globs.iter().any(|glob| glob.matches(&entry.path)) {
                continue;
            }
            findings.push(Finding {
                kind: "specification_feature_unretained".to_string(),
                level: "warning".to_string(),
                files: vec![entry.path.clone()],
                titles: vec![
                    "the .feature file matches no 'behavior' glob; it is not retained by the \
                     declared Specification"
                        .to_string(),
                ],
                contradiction: false,
            });
        }
    }

    let e2e = declaration
        .roles()
        .into_iter()
        .find(|(role, _)| *role == Role::E2e)
        .map(|(_, paths)| paths)
        .unwrap_or(&[]);
    let e2e_globs: Vec<Glob> = e2e.iter().map(|pattern| Glob::compile(pattern)).collect();
    let matched: Vec<&ProjectPath> = tree
        .iter()
        .filter(|entry| e2e_globs.iter().any(|glob| glob.matches(&entry.path)))
        .collect();
    if !e2e.is_empty() && matched.is_empty() {
        for pattern in e2e {
            findings.push(declaration_advisory(
                "specification_e2e_boundary",
                pattern,
                format!("the 'e2e' role entry '{pattern}' matches no path in the project"),
            ));
        }
    }
    if !matched.is_empty() {
        let manifest_directories: std::collections::BTreeSet<String> = tree
            .iter()
            .filter(|entry| !entry.is_dir && is_cargo_manifest(&entry.path))
            .map(|entry| parent_directory(&entry.path))
            .collect();
        let mut packages: std::collections::BTreeMap<String, CargoPackage> =
            std::collections::BTreeMap::new();
        for directory in &manifest_directories {
            let manifest = if directory.is_empty() {
                project_root.join("Cargo.toml")
            } else {
                project_root.join(directory).join("Cargo.toml")
            };
            if let Some(package) = CargoPackage::read(directory, &manifest) {
                packages.insert(directory.clone(), package);
            }
        }

        let mut resolved: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for entry in &matched {
            let candidate = if entry.is_dir {
                entry.path.clone()
            } else {
                parent_directory(&entry.path)
            };
            match nearest_manifest(&manifest_directories, &candidate) {
                Some(directory) => {
                    // A manifest that exists but does not parse (malformed, or
                    // carrying no [package]) skips the edge comparison for that
                    // package; no finding is invented for it.
                    if packages.contains_key(&directory) {
                        resolved.insert(directory);
                    }
                }
                None => findings.push(declaration_advisory(
                    "specification_e2e_boundary",
                    &entry.path,
                    format!(
                        "the matched path '{}' resolves no package manifest at or above it \
                         before the project root",
                        entry.path
                    ),
                )),
            }
        }

        let source_roots: Vec<(&String, Glob)> = declaration
            .source_roots()
            .iter()
            .map(|pattern| (pattern, Glob::compile(pattern)))
            .collect();
        let edges = dependency_edges(&packages);
        let mut reported: std::collections::BTreeSet<(String, String)> =
            std::collections::BTreeSet::new();
        for start in &resolved {
            let package = &packages[start];
            if let Some((pattern, _)) = source_roots.iter().find(|(_, glob)| glob.matches(start)) {
                findings.push(declaration_advisory(
                    "specification_e2e_boundary",
                    start,
                    format!(
                        "e2e package '{}' sits under a source root ('{pattern}')",
                        package.name
                    ),
                ));
            }
            let mut visited: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            visited.insert(start.clone());
            let mut queue: std::collections::VecDeque<(String, usize)> =
                std::collections::VecDeque::new();
            queue.push_back((start.clone(), 0));
            while let Some((current, distance)) = queue.pop_front() {
                for target in edges.get(&current).into_iter().flatten() {
                    if !visited.insert(target.clone()) {
                        continue;
                    }
                    let Some((pattern, _)) =
                        source_roots.iter().find(|(_, glob)| glob.matches(target))
                    else {
                        queue.push_back((target.clone(), distance + 1));
                        continue;
                    };
                    if !reported.insert((start.clone(), target.clone())) {
                        continue;
                    }
                    let edge = if distance + 1 == 1 {
                        "dependency edge"
                    } else {
                        "transitive dependency edge"
                    };
                    findings.push(declaration_advisory(
                        "specification_e2e_boundary",
                        start,
                        format!(
                            "e2e package '{}' has a {edge} into source (root '{pattern}') through \
                             package '{}'",
                            package.name, packages[target].name
                        ),
                    ));
                }
            }
        }
    }
    findings
}

/// Whether a project-relative path is a Cargo manifest.
fn is_cargo_manifest(path: &str) -> bool {
    path.rsplit('/').next() == Some("Cargo.toml")
}

/// The nearest manifest directory at or above `path`, bounded at the project
/// root (`""`).
fn nearest_manifest(
    manifest_directories: &std::collections::BTreeSet<String>,
    path: &str,
) -> Option<String> {
    let mut current = Some(path.to_string());
    while let Some(directory) = current {
        if manifest_directories.contains(&directory) {
            return Some(directory);
        }
        current = if directory.is_empty() {
            None
        } else {
            Some(parent_directory(&directory))
        };
    }
    None
}

/// A package manifest read statically, without invoking `cargo`.
struct CargoPackage {
    name: String,
    dependencies: Vec<Dependency>,
    workspace_dependencies: std::collections::BTreeMap<String, String>,
    has_workspace: bool,
}

/// One dependency edge declared by a manifest. A registry dependency carries
/// no project path and is not an edge into `source`.
enum Dependency {
    /// An explicit `path = "..."` dependency, normalized to a project-relative
    /// directory.
    Path(String),
    /// `workspace = true`; the path comes from the workspace root.
    Workspace(String),
}

impl CargoPackage {
    fn read(dir: &str, manifest: &std::path::Path) -> Option<Self> {
        let content = std::fs::read_to_string(manifest).ok()?;
        let document: toml::Value = toml::from_str(&content).ok()?;
        let name = document.get("package")?.get("name")?.as_str()?.to_string();
        let mut dependencies = Vec::new();
        for table in ["dependencies", "dev-dependencies", "build-dependencies"] {
            let Some(entries) = document.get(table).and_then(toml::Value::as_table) else {
                continue;
            };
            for (dependency, entry) in entries {
                let Some(entry) = entry.as_table() else {
                    continue;
                };
                if let Some(path) = entry.get("path").and_then(toml::Value::as_str) {
                    if let Some(target) = normalize_relative(dir, path) {
                        dependencies.push(Dependency::Path(target));
                    }
                } else if entry.get("workspace").and_then(toml::Value::as_bool) == Some(true) {
                    dependencies.push(Dependency::Workspace(dependency.clone()));
                }
            }
        }
        let workspace_dependencies = document
            .get("workspace")
            .and_then(|workspace| workspace.get("dependencies"))
            .and_then(toml::Value::as_table)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|(name, entry)| {
                        let path = entry.get("path")?.as_str()?;
                        Some((name.clone(), normalize_relative(dir, path)?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Some(CargoPackage {
            name,
            dependencies,
            workspace_dependencies,
            has_workspace: document.get("workspace").is_some(),
        })
    }
}

/// Resolve `path` (relative to the package at `base`) to a project-relative
/// directory, refusing an escape above the project root.
fn normalize_relative(base: &str, path: &str) -> Option<String> {
    let mut segments: Vec<&str> = base
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    for component in std::path::Path::new(path).components() {
        match component {
            std::path::Component::Normal(segment) => segments.push(segment.to_str()?),
            std::path::Component::ParentDir => {
                segments.pop()?;
            }
            std::path::Component::CurDir => {}
            std::path::Component::RootDir | std::path::Component::Prefix(_) => return None,
        }
    }
    Some(segments.join("/"))
}

/// The direct dependency edges between the parsed packages.
fn dependency_edges(
    packages: &std::collections::BTreeMap<String, CargoPackage>,
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut edges: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for (dir, package) in packages {
        let mut targets: Vec<String> = Vec::new();
        for dependency in &package.dependencies {
            let target = match dependency {
                Dependency::Path(target) => Some(target.clone()),
                Dependency::Workspace(name) => workspace_root(packages, dir)
                    .and_then(|root| root.workspace_dependencies.get(name).cloned()),
            };
            if let Some(target) = target
                && packages.contains_key(&target)
                && !targets.contains(&target)
            {
                targets.push(target);
            }
        }
        edges.insert(dir.clone(), targets);
    }
    edges
}

/// The nearest package whose manifest declares `[workspace]`, at or above
/// `dir`.
fn workspace_root<'a>(
    packages: &'a std::collections::BTreeMap<String, CargoPackage>,
    dir: &str,
) -> Option<&'a CargoPackage> {
    let mut current = Some(dir.to_string());
    while let Some(directory) = current {
        if let Some(package) = packages.get(&directory)
            && package.has_workspace
        {
            return Some(package);
        }
        current = if directory.is_empty() {
            None
        } else {
            Some(parent_directory(&directory))
        };
    }
    None
}

/// Whether a project-relative path names a `.feature` file.
fn is_feature_file(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "feature")
}

/// One entry of the bounded project walk: a project-relative path and whether
/// it is a directory.
struct ProjectPath {
    path: String,
    is_dir: bool,
}

/// Walk the project under the finding exclusions. Paths are project-relative
/// and use `/`; version-control, build, archive, and active-change trees are
/// skipped because they are not retained behavior.
fn walk_project(project_root: &std::path::Path) -> Vec<ProjectPath> {
    let mut entries = Vec::new();
    walk_directory(project_root, project_root, &mut entries);
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    entries
}

fn walk_directory(
    project_root: &std::path::Path,
    directory: &std::path::Path,
    entries: &mut Vec<ProjectPath>,
) {
    let Ok(read_dir) = std::fs::read_dir(directory) else {
        return;
    };
    let mut children: Vec<std::fs::DirEntry> = read_dir.flatten().collect();
    children.sort_by_key(|entry| entry.file_name());
    for child in children {
        let path = child.path();
        let Ok(relative) = path.strip_prefix(project_root) else {
            continue;
        };
        let relative = relative
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
        if is_excluded_from_findings(&relative) {
            continue;
        }
        let Ok(file_type) = child.file_type() else {
            continue;
        };
        let is_dir = file_type.is_dir();
        entries.push(ProjectPath {
            path: relative,
            is_dir,
        });
        if is_dir {
            walk_directory(project_root, &path, entries);
        }
    }
}

/// Whether a project-relative path belongs to a tree the findings never scan.
fn is_excluded_from_findings(relative: &str) -> bool {
    if relative == "givn/archive"
        || relative.starts_with("givn/archive/")
        || relative == "givn/changes"
        || relative.starts_with("givn/changes/")
    {
        return true;
    }
    relative
        .split('/')
        .any(|segment| matches!(segment, ".git" | ".hg" | ".svn" | ".fossil" | "target"))
}

/// The parent directory of a project-relative path; the project root is `""`.
fn parent_directory(path: &str) -> String {
    path.rsplit_once('/')
        .map_or(String::new(), |(parent, _)| parent.to_string())
}

/// One compiled role pattern. `*` matches within one segment, `?` matches one
/// character, and a `**` segment matches zero or more segments. Everything else
/// is literal, so a pattern without wildcards is an existence check.
struct Glob {
    segments: Vec<String>,
}

impl Glob {
    fn compile(pattern: &str) -> Self {
        let segments = std::path::Path::new(pattern)
            .components()
            .filter_map(|component| match component {
                std::path::Component::Normal(segment) => {
                    Some(segment.to_string_lossy().into_owned())
                }
                _ => None,
            })
            .collect();
        Glob { segments }
    }

    fn matches(&self, path: &str) -> bool {
        self.matches_segments(0, path)
    }

    fn matches_segments(&self, pattern_index: usize, rest: &str) -> bool {
        let Some(segment) = self.segments.get(pattern_index) else {
            return rest.is_empty();
        };
        if segment == "**" {
            let mut remaining = rest;
            loop {
                if self.matches_segments(pattern_index + 1, remaining) {
                    return true;
                }
                if remaining.is_empty() {
                    return false;
                }
                remaining = match remaining.find('/') {
                    Some(slash) => &remaining[slash + 1..],
                    None => "",
                };
            }
        }
        let (head, tail) = match rest.find('/') {
            Some(slash) => (&rest[..slash], &rest[slash + 1..]),
            None => (rest, ""),
        };
        !head.is_empty()
            && segment_matches(segment, head)
            && self.matches_segments(pattern_index + 1, tail)
    }
}

/// Whether one path segment matches one pattern segment. Wildcards consume
/// whole characters, so a multi-byte character counts as one `?`.
fn segment_matches(pattern: &str, text: &str) -> bool {
    match pattern.chars().next() {
        None => text.is_empty(),
        Some('*') => {
            let rest = &pattern[1..];
            segment_matches(rest, text)
                || text.chars().next().is_some_and(|character| {
                    segment_matches(pattern, &text[character.len_utf8()..])
                })
        }
        Some('?') => text
            .chars()
            .next()
            .is_some_and(|character| segment_matches(&pattern[1..], &text[character.len_utf8()..])),
        Some(literal) => text.chars().next().is_some_and(|character| {
            character == literal
                && segment_matches(
                    &pattern[literal.len_utf8()..],
                    &text[character.len_utf8()..],
                )
        }),
    }
}

/// The parsed status of a change seed.
enum SeedStatus {
    /// A known status that needs no change lookup.
    Valid,
    /// No `Status:` line, an unrecognised value, or an empty reference.
    Missing,
    /// `proposed by <change-id>` — the referenced change must be resolved.
    Proposed(String),
}

/// Advisory seed-status hygiene for `givn/ideation/*/changes/*.md`.
///
/// Tolerant by design: only the first `Status:` line is read; prose before and
/// after it is ignored. Findings are advisory and never affect the exit code.
fn check_seed_status(project_root: &std::path::Path) -> Vec<Finding> {
    let ideation_root = project_root.join("givn/ideation");
    if !ideation_root.is_dir() {
        return Vec::new();
    }

    let mut topic_dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&ideation_root)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .collect()
        })
        .unwrap_or_default();
    topic_dirs.sort();

    let mut seeds: Vec<std::path::PathBuf> = Vec::new();
    for topic in topic_dirs {
        let changes_dir = topic.join("changes");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&changes_dir)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.is_file()
                            && path
                                .extension()
                                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
                    })
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        seeds.extend(files);
    }

    let mut findings = Vec::new();
    for seed in seeds {
        let content = std::fs::read_to_string(&seed).unwrap_or_default();
        let message = match parse_seed_status(&content) {
            SeedStatus::Valid => None,
            SeedStatus::Missing => Some(
                "change seed has no valid status line (expected ready, proposed by <change-id>, \
                 implemented in <change-id>, superseded by <reference>, or withdrawn)"
                    .to_string(),
            ),
            SeedStatus::Proposed(id) => {
                if project_root.join("givn/changes").join(&id).is_dir() {
                    None
                } else if project_root.join("givn/archive").join(&id).is_dir() {
                    Some(format!(
                        "change seed is proposed by '{id}', but that change is already archived"
                    ))
                } else {
                    Some(format!(
                        "change seed is proposed by '{id}', but no such change exists"
                    ))
                }
            }
        };
        if let Some(message) = message {
            findings.push(Finding {
                kind: "seed_status".to_string(),
                level: "warning".to_string(),
                files: vec![seed.display().to_string()],
                titles: vec![message],
                contradiction: false,
            });
        }
    }
    findings
}

/// Advisory persona-reference hygiene for active change use-case documents
/// and ideation use cases. A `## Personas` entry resolves when the ID names a
/// file under `givn/personas/` or any topic's `personas/`. Matching is
/// ID-only; the finding never affects the exit code.
fn check_persona_references(project_root: &std::path::Path, change: Option<&str>) -> Vec<Finding> {
    let mut documents: Vec<std::path::PathBuf> = Vec::new();

    let changes_root = project_root.join("givn/changes");
    let change_ids: Vec<String> = match change {
        Some(id) => vec![id.to_string()],
        None => read_dir_names(&changes_root),
    };
    for id in change_ids {
        let specs_dir = changes_root.join(&id).join("specs");
        for usecase in read_dir_names(&specs_dir) {
            let path = specs_dir.join(&usecase).join("usecase.md");
            if path.is_file() {
                documents.push(path);
            }
        }
    }

    if change.is_none() {
        let ideation_root = project_root.join("givn/ideation");
        for topic in read_dir_names(&ideation_root) {
            let use_cases = ideation_root.join(&topic).join("use-cases");
            for entry in read_dir_names(&use_cases) {
                let path = use_cases.join(format!("{entry}.md"));
                if path.is_file() {
                    documents.push(path);
                }
            }
        }
    }

    let project_personas = project_root.join("givn/personas");
    let ideation_root = project_root.join("givn/ideation");

    let mut findings = Vec::new();
    for document in documents {
        let content = std::fs::read_to_string(&document).unwrap_or_default();
        for persona in persona_entries(&content) {
            if resolves_persona(&project_personas, &ideation_root, &persona) {
                continue;
            }
            findings.push(Finding {
                kind: "persona_ref".to_string(),
                level: "warning".to_string(),
                files: vec![document.display().to_string()],
                titles: vec![format!(
                    "persona reference '{persona}' matches no project or topic persona file"
                )],
                contradiction: false,
            });
        }
    }
    findings
}

/// Names of the immediate subdirectories of `dir`, sorted; empty when absent.
fn read_dir_names(dir: &std::path::Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.path().is_dir())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The `## Personas` bullet entries of a use-case document, tolerantly.
fn persona_entries(content: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut in_section = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            in_section = trimmed == "## Personas";
            continue;
        }
        if !in_section {
            continue;
        }
        let Some(value) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        else {
            continue;
        };
        let value = value.trim();
        if !value.is_empty() && !value.eq_ignore_ascii_case("none") {
            entries.push(value.to_string());
        }
    }
    entries
}

fn resolves_persona(
    project_personas: &std::path::Path,
    ideation_root: &std::path::Path,
    persona: &str,
) -> bool {
    if project_personas.join(format!("{persona}.md")).is_file() {
        return true;
    }
    for topic in read_dir_names(ideation_root) {
        if ideation_root
            .join(&topic)
            .join("personas")
            .join(format!("{persona}.md"))
            .is_file()
        {
            return true;
        }
    }
    false
}

/// Read the first `Status:` line of a seed and classify its value.
fn parse_seed_status(content: &str) -> SeedStatus {
    let Some(value) = content
        .lines()
        .find_map(|line| line.trim().strip_prefix("Status:").map(str::trim))
    else {
        return SeedStatus::Missing;
    };

    if value == "ready"
        || value.starts_with("ready ")
        || value.starts_with("withdrawn")
        || value.starts_with("implemented in ")
        || value.starts_with("superseded by ")
    {
        return SeedStatus::Valid;
    }
    if let Some(id) = value
        .strip_prefix("proposed by ")
        .map(str::trim)
        .filter(|id| !id.is_empty())
    {
        return SeedStatus::Proposed(id.to_string());
    }
    SeedStatus::Missing
}

/// The severity of one lint finding. Descriptive: the exit-code flags still
/// drive the exit code, so an advisory-only run exits 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Advisory,
    Finding,
    Blocking,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Severity::Advisory => "advisory",
            Severity::Finding => "finding",
            Severity::Blocking => "blocking",
        }
    }
}

/// The one registry mapping a finding kind to its severity. New kinds
/// register here; rendering never changes.
fn severity_for(kind: &str) -> Severity {
    match kind {
        "specification_declaration"
        | "parse_error"
        | "duplicate_title"
        | "empty_feature"
        | "usecase_invalid"
        | "fragment_invalid"
        | "dangling_capability_ref"
        | "ambiguous_relationship_target"
        | "dangling_use_case_ref"
        | "use_case_cycle"
        | "extend_fragment"
        | "interaction_mismatch"
        | "usecase_interaction_invalid"
        | "undeclared_capability" => Severity::Blocking,
        "specification_path_missing"
        | "specification_role_duplicate"
        | "specification_feature_unretained"
        | "specification_e2e_boundary" => Severity::Advisory,
        "wip" | "e2e_intent" => Severity::Finding,
        _ => Severity::Advisory,
    }
}

/// Render findings as JSON objects with the original fields plus `severity`.
fn findings_json(findings: &[Finding]) -> Result<String> {
    let enriched: Vec<serde_json::Value> = findings
        .iter()
        .map(|finding| {
            let mut value = serde_json::to_value(finding)?;
            value["severity"] = serde_json::json!(severity_for(&finding.kind).as_str());
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(serde_json::to_string_pretty(&enriched)?)
}

/// Render one finding in the human-readable format on stderr.
fn render_prose(finding: &Finding) {
    let severity = severity_for(&finding.kind).as_str();
    let level = match finding.kind.as_str() {
        "duplicate_title" => "DUPL ",
        "specification_declaration" => "DECL ",
        "specification_path_missing"
        | "specification_role_duplicate"
        | "specification_feature_unretained"
        | "specification_e2e_boundary" => "SPEC ",
        "shape_match" => "SHAPE",
        "subset" => "SUBST",
        "long_scenario" => "LONG ",
        "empty_feature" => "EMPTY",
        "seed_status" => "SEED ",
        "persona_ref" => "PERS ",
        "usecase_invalid" | "fragment_invalid" => "UCDOC",
        "dangling_use_case_ref" => "UCREF",
        "use_case_cycle" => "UCCYC",
        "parse_error" => "ERROR",
        "wip" => "WIP  ",
        _ => "WARN ",
    };
    let location = if finding.files.len() > 1 {
        format!("{} & {}", finding.files[0], finding.files[1])
    } else {
        finding.files.first().cloned().unwrap_or_default()
    };
    match finding.kind.as_str() {
        "duplicate_title" => {
            eprintln!(
                "[{level}] {severity}  {location}  duplicate scenario title '{}'",
                finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
        "shape_match" => {
            let (a, b) = (finding.titles[0].as_str(), finding.titles[1].as_str());
            if finding.contradiction {
                eprintln!(
                    "[{level}] {severity}  {location}  related_scenario: '{a}' and '{b}' share a shape but their facts contradict"
                );
            } else {
                eprintln!(
                    "[{level}] {severity}  {location}  scenarios '{a}' and '{b}' share a shape — consider merging"
                );
            }
        }
        "subset" => {
            eprintln!(
                "[{level}] {severity}  {location}  subset: scenario '{a}' is a subset of '{b}'",
                a = finding.titles[0],
                b = finding.titles[1]
            );
        }
        "long_scenario" => {
            eprintln!(
                "[{level}] {severity}  {location}  long scenario: '{title}' has more than {} steps",
                crate::overlap::analysis::LONG_SCENARIO_STEP_THRESHOLD,
                title = finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
        "empty_feature" => {
            eprintln!("[{level}] {severity}  {location}  feature has zero scenarios");
        }
        _ => {
            eprintln!(
                "[{level}] {severity}  {location}  {}",
                finding.titles.first().map(String::as_str).unwrap_or("?")
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A temp project with givn/specs/ seeded from the given (path, text)
    /// pairs. Returns the temp dir handle (keep alive) and the project root.
    fn temp_project(files: &[(&str, &str)]) -> (tempfile::TempDir, std::path::PathBuf) {
        let root = tempfile::TempDir::new().unwrap();
        let project = root.path().to_path_buf();
        for (rel, text) in files {
            let path = project.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        let mut usecases = std::collections::BTreeSet::new();
        for (rel, _) in files {
            let path = std::path::Path::new(rel);
            if path
                .extension()
                .is_some_and(|extension| extension == "feature")
                && let Some(parent) = path.parent()
            {
                usecases.insert(parent.to_path_buf());
            }
        }
        for relative in usecases {
            let usecase = relative.file_name().unwrap().to_string_lossy();
            let feature = relative
                .file_name()
                .map(|name| name.to_string_lossy())
                .unwrap_or_default();
            let document = format!(
                "# Use case: {usecase}\n\n## Level\n\n!\n\n## Actors\n\n- Operator\n\n## Goal\n\nInspect behavior.\n\n## Trigger\n\nThe operator asks.\n\n## Preconditions\n\n- The project exists.\n\n## Main flow\n\n1. Inspect.\n\n## Extensions\n\n- none\n\n## Rules\n\n- The project remains valid.\n\n## Examples\n\n- A project.\n\n## Minimal guarantee\n\nThe project remains inspectable.\n\n## Success guarantee\n\nThe result is visible.\n\n## Personas\n\n- none\n\n## Capabilities\n\n- {feature}\n\n## Interactions\n\n- none\n\n## Includes\n\n- none\n\n## Extends\n\n- none\n\n## Out of scope\n\nNothing.\n\n## Diagram\n\n```mermaid\nflowchart LR\n  A --> B\n```\n"
            );
            std::fs::write(project.join(relative).join("usecase.md"), document).unwrap();
        }
        (root, project)
    }

    #[test]
    fn empty_feature_is_an_error_and_exits_3() {
        let (_root, project) = temp_project(&[(
            "givn/specs/hollow/hollow.feature",
            "@givn.delta @hollow\nFeature: Hollow\n",
        )]);
        let code = run_with_root(&project, None, vec![], false).unwrap();
        assert_eq!(code, 3, "empty_feature must exit 3");
    }

    #[test]
    fn retired_empty_feature_does_not_trigger_the_finding() {
        let (_root, project) = temp_project(&[(
            "givn/specs/old-cap/old-cap.feature",
            "@givn.delta @givn.retired @old-cap\nFeature: Old capability\n",
        )]);
        let code = run_with_root(&project, None, vec![], false).unwrap();
        assert_eq!(code, 0, "retirement markers are legitimately empty");
    }

    #[test]
    fn populated_feature_is_clean() {
        let (_root, project) = temp_project(&[(
            "givn/specs/full/full.feature",
            "Feature: Full\n\n  Scenario: One\n    Given x\n    Then y\n",
        )]);
        let code = run_with_root(&project, None, vec![], false).unwrap();
        assert_eq!(code, 0);
    }

    #[test]
    fn an_invalid_interaction_structure_is_blocking() {
        assert_eq!(
            severity_for("usecase_interaction_invalid"),
            Severity::Blocking
        );
    }
}
