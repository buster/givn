use serde::{Deserialize, Serialize};

/// Top-level manifest document (embedded default or project overlay).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Artifact definitions.
    #[serde(default)]
    pub artifacts: Vec<Artifact>,

    /// Archive gate and tracking configuration.
    #[serde(default)]
    pub apply: Apply,

    /// Test-runner configuration.
    #[serde(default)]
    pub verify: Verify,

    /// Product start configuration.
    #[serde(default)]
    pub run: Run,

    /// Coverage merge script configuration.
    #[serde(default)]
    pub coverage: Coverage,

    /// Enabled addons (e.g. `arc42: true`).
    #[serde(default)]
    pub addons: Addons,

    /// Config version marker (written to givn/config.yaml by init/upgrade).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub givn_config_version: Option<String>,

    /// Whether extra tooling (e.g. grilling subagent) is active.
    #[serde(default)]
    pub extras: bool,
}

/// An artifact node in the lifecycle graph.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Artifact {
    /// Unique artifact identifier (e.g. `proposal`, `specs`, `tasks`).
    pub id: String,

    /// Glob(s) the artifact produces (relative to the change dir or project root).
    #[serde(default)]
    pub generates: Vec<String>,

    /// Artifact ids that must exist before this one (defines the DAG order).
    #[serde(default)]
    pub requires: Vec<String>,

    /// Template facet name (resolved via cascade).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// Instruction facet name (resolved via cascade).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,

    /// If true, the artifact may be skipped without blocking the lifecycle.
    #[serde(default)]
    pub optional: bool,

    /// Overlay directive: if true, remove this artifact from the effective graph.
    #[serde(default, rename = "remove")]
    pub remove: bool,

    /// Post-completion hooks (e.g. `verify`, `lint`, or a shell command).
    #[serde(default)]
    pub post: Vec<HookEntry>,
}

/// A single post-hook entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HookEntry {
    /// Shorthand: just the hook name (e.g. `"verify"` or a shell command).
    Short(String),
    /// Object form: allows `allow_failure` and `name`.
    Full(HookFull),
}

/// Full hook object form.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HookFull {
    /// Hook name or shell command.
    pub run: String,
    /// If true, a non-zero exit only warns and does not block the gate.
    #[serde(default)]
    pub allow_failure: bool,
    /// Display name for the hook (shown in output).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Archive gate configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Apply {
    /// Artifact ids that must be complete before a change can be archived.
    #[serde(default)]
    pub requires: Vec<String>,
    /// Path (relative to change dir) of the artifact whose checkboxes are tracked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<String>,
}

/// Test-runner configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Verify {
    /// Command to run the test suite. Default is the sentinel.
    pub command: String,
    /// Command to run end-to-end smoke tests (@e2e-tagged scenarios).
    /// The sentinel `"givn missing-e2e-runner"` means unconfigured.
    #[serde(default = "Verify::default_e2e_command")]
    pub e2e_command: String,
    /// Features path passed to the runner via GIVN_FEATURES.
    #[serde(default = "Verify::default_features_path")]
    pub features_path: String,
}

impl Default for Verify {
    fn default() -> Self {
        Self {
            command: "givn missing-testrunner".to_string(),
            e2e_command: "givn missing-e2e-runner".to_string(),
            features_path: Self::default_features_path(),
        }
    }
}

impl Verify {
    fn default_e2e_command() -> String {
        "givn missing-e2e-runner".to_string()
    }

    fn default_features_path() -> String {
        "givn/specs".to_string()
    }
}

/// Product start configuration (`givn/commands.yaml`, key `run`).
///
/// givn never executes `command` as a side effect of validation — it
/// validates the block, renders it into the README, and blocks the archive
/// gate while it is unconfigured. Starting the product is the job of
/// `givn run` and of automation around givn.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    /// Command that starts the product. Default is the sentinel.
    #[serde(default = "Run::default_command")]
    pub command: String,

    /// `server`, `cli`, or `library`. `None` means unconfigured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,

    /// Fixed listen port. `product_type: server` only.
    /// Mutually exclusive with `port_env`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// Name of the environment variable through which the caller passes
    /// the port. `product_type: server` only. Mutually exclusive with `port`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_env: Option<String>,

    /// HTTP path that answers 2xx once the server is ready to serve.
    /// `product_type: server` only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready_path: Option<String>,

    /// Seconds the caller waits for readiness before giving up.
    #[serde(default = "Run::default_ready_timeout_s")]
    pub ready_timeout_s: u64,
}

/// The kind of product the run declaration starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    /// Long-running process that listens on a port.
    Server,
    /// One-shot command-line tool.
    Cli,
    /// Consumed as a dependency; nothing to start.
    Library,
}

impl Default for Run {
    fn default() -> Self {
        Self {
            command: Self::default_command(),
            product_type: None,
            port: None,
            port_env: None,
            ready_path: None,
            ready_timeout_s: Self::default_ready_timeout_s(),
        }
    }
}

impl Run {
    /// Sentinel: the block exists but was never filled in.
    pub const SENTINEL: &'static str = "givn missing-run-command";

    fn default_command() -> String {
        Self::SENTINEL.to_string()
    }

    fn default_ready_timeout_s() -> u64 {
        60
    }
}

/// Coverage instrumentation configuration.
///
/// When the coverage addon is enabled (`addons.coverage: true`), givn runs a
/// project-owned measurement command and then a project-owned merge command
/// during archive. The merge command produces the report used for README
/// output.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Coverage {
    /// Command that produces both source Cobertura reports.
    #[serde(default = "sentinel")]
    pub measure_command: String,
    /// Command that merges the source reports into `output_path`.
    #[serde(default = "sentinel")]
    pub merge_command: String,
    /// Legacy non-e2e command accepted during migration. New projects must
    /// use `measure_command`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_e2e_command: Option<String>,
    /// Legacy e2e command accepted during migration. New projects must use
    /// `measure_command`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e2e_command: Option<String>,
    /// Path (relative to project root) to the non-e2e source report.
    #[serde(default = "default_non_e2e_path")]
    pub non_e2e_path: String,
    /// Path (relative to project root) to the e2e source report.
    #[serde(default = "default_e2e_path")]
    pub e2e_path: String,
    /// Path (relative to project root) where merge_command writes the merged
    /// Cobertura XML.
    #[serde(default = "default_cobertura_output")]
    pub output_path: String,
}

fn sentinel() -> String {
    "givn missing-coverage".to_string()
}

fn default_non_e2e_path() -> String {
    "coverage/non-e2e-cobertura.xml".to_string()
}

fn default_e2e_path() -> String {
    "coverage/e2e-cobertura.xml".to_string()
}

fn default_cobertura_output() -> String {
    "coverage/cobertura-coverage.xml".to_string()
}

impl Default for Coverage {
    fn default() -> Self {
        Self {
            measure_command: sentinel(),
            merge_command: sentinel(),
            non_e2e_command: None,
            e2e_command: None,
            non_e2e_path: default_non_e2e_path(),
            e2e_path: default_e2e_path(),
            output_path: default_cobertura_output(),
        }
    }
}

/// Addon switches.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Addons {
    /// Enable the arc42 documentation artifact.
    #[serde(default)]
    pub arc42: bool,
    /// Enable the Absolute Mode communication policy. When true, givn
    /// manages a fenced `<!-- givn:begin:absolute-mode -->` block in the
    /// project's AGENTS.md containing the policy text.
    #[serde(default)]
    pub absolute_mode: bool,
    /// Enable the readme scaffolding addon. When true, `givn init` and
    /// `givn addons enable readme` write an initial README.md to the
    /// project root from the embedded template.
    #[serde(default)]
    pub readme: bool,
    /// Enable the archive-level coverage gate. When true, givn measures
    /// full-project coverage during archive (runs coverage.*_command, merges
    /// Cobertura XML, updates README). The archive will be blocked and
    /// rolled back if coverage fails.
    #[serde(default)]
    pub coverage: bool,
    /// Skill — model a change's domain (ubiquitous language, aggregates,
    /// invariants) before design.
    #[serde(default)]
    pub domain_modeling: bool,
    /// Skill — apply the team's engineering principles during implementation.
    #[serde(default)]
    pub dev_principles: bool,
    /// Command — Socratically grill the current plan/design, one question at a time.
    #[serde(default)]
    pub grill_me: bool,
    /// Command — grill the plan/design, citing a source in the project's own
    /// docs for every challenge.
    #[serde(default)]
    pub grill_with_docs: bool,
}

impl Manifest {
    /// Load and parse the embedded default manifest.
    pub fn load_embedded() -> anyhow::Result<Self> {
        let yaml = crate::config::embedded::Assets::get_str("default-manifest.yaml")
            .ok_or_else(|| anyhow::anyhow!("embedded default-manifest.yaml not found"))?;
        let manifest: Self = serde_yaml_ng::from_str(&yaml)?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The run scaffold is a single source of truth: `assets/run.yaml.tmpl`
    /// is `include_str!`ed by the upgrade backfill, while
    /// `assets/commands.yaml.tmpl` embeds a copy (assets are static). This
    /// test pins the copy so the two cannot drift.
    #[test]
    fn run_scaffold_is_embedded_verbatim() {
        let scaffold = include_str!("../../assets/run.yaml.tmpl");
        let commands = include_str!("../../assets/commands.yaml.tmpl");
        assert!(
            commands.contains(scaffold),
            "commands.yaml.tmpl must embed run.yaml.tmpl byte for byte"
        );
    }

    /// Verify that every key documented in config.yaml.tmpl maps to a real
    /// manifest schema key, and vice versa (no drift allowed).
    ///
    /// This is a structural smoke test: we check that the config template
    /// references the same top-level keys the manifest schema defines.
    #[test]
    fn config_template_keys_match_manifest_schema() {
        let tmpl = crate::config::embedded::Assets::get_str("config.yaml.tmpl")
            .expect("config.yaml.tmpl must be embedded");

        // The documented top-level keys in the template.
        let documented_keys = ["verify", "apply", "artifacts", "addons", "coverage"];
        for key in &documented_keys {
            assert!(
                tmpl.contains(key),
                "config.yaml.tmpl must document the '{key}' key"
            );
        }

        // The manifest schema top-level fields.
        // If you add a new field to Manifest, add it here too.
        let schema_fields = [
            "artifacts",
            "apply",
            "verify",
            "coverage",
            "addons",
            "givn_config_version",
        ];
        for field in &schema_fields {
            assert!(
                tmpl.contains(field),
                "config.yaml.tmpl must reference the schema field '{field}'"
            );
        }
    }

    #[test]
    fn embedded_manifest_deserialises() {
        let manifest = Manifest::load_embedded().expect("should load embedded manifest");
        assert!(
            !manifest.artifacts.is_empty(),
            "manifest must define at least one artifact"
        );

        // Check required built-in artifacts are present.
        let ids: Vec<&str> = manifest.artifacts.iter().map(|a| a.id.as_str()).collect();
        for required in &["proposal", "specs", "design", "tasks", "review"] {
            assert!(
                ids.contains(required),
                "embedded manifest must contain artifact '{required}'"
            );
        }

        // apply.requires must reference [tasks, review] by default.
        assert!(
            manifest.apply.requires.contains(&"tasks".to_string()),
            "apply.requires must include 'tasks'"
        );
        assert!(
            manifest.apply.requires.contains(&"review".to_string()),
            "apply.requires must include 'review'"
        );

        // Default verify.command is the sentinel.
        assert_eq!(
            manifest.verify.command, "givn missing-testrunner",
            "default verify.command must be the sentinel"
        );

        // Built-in hooks have been moved out of `post` and into the binary
        // as mandatory checks on `givn check tasks` and `givn check review`.
        // `post` lists are now purely additive for user-custom hooks.
        let review = manifest
            .artifacts
            .iter()
            .find(|a| a.id == "review")
            .expect("embedded manifest must contain artifact 'review'");
        assert!(
            review.post.is_empty(),
            "review.post must be empty in the default manifest — verify, verify-e2e, \
             and integrity are now mandatory binary checks, not configurable hooks"
        );
    }

    #[test]
    fn legacy_coverage_commands_are_accepted_during_migration() {
        let base = Manifest::load_embedded().unwrap();
        let result = crate::config::overlay::apply(
            base,
            "coverage:\n  non_e2e_command: \"old-non-e2e\"\n  e2e_command: \"old-e2e\"\n",
        )
        .unwrap();
        assert_eq!(
            result.coverage.non_e2e_command.as_deref(),
            Some("old-non-e2e")
        );
        assert_eq!(result.coverage.e2e_command.as_deref(), Some("old-e2e"));
    }
}
