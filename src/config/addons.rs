//! Addon registry: the declared default, prerequisites, and filesystem effect
//! of every known addon.
//!
//! `config` owns the registry because both `cli` and `config::lock` read it;
//! `cli::addons` consumes it and re-exports nothing. See arc42 ch.8.11.

/// The filesystem effect an addon declares (ADR-080). The enable path and
/// `givn addons list` both read it, so the two cannot disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Materialization {
    /// No project files are written.
    None,
    /// Paths that must all exist, relative to the project root.
    Files(&'static [&'static str]),
    /// One managed fenced block inside a project file.
    ManagedBlock {
        file: &'static str,
        block: &'static str,
    },
    /// Scaffolded paths plus a generated skill directory checked in every
    /// selected agent target.
    FilesAndSkill {
        files: &'static [&'static str],
        skill: &'static str,
    },
}

/// One registered addon.
pub struct AddonSpec {
    pub id: &'static str,
    pub description: &'static str,
    /// The default the effective addon state resolves to when
    /// `givn/config.yaml` omits the key.
    pub default_enabled: bool,
    /// The filesystem effect this addon declares.
    pub materialization: Materialization,
    /// Addon ids that must be effective before this addon may be enabled.
    pub requires: &'static [&'static str],
}

/// Registry of all known addons.
pub const KNOWN_ADDONS: &[AddonSpec] = &[
    AddonSpec {
        id: "arc42",
        description: "Arc42 architecture documentation — creates and maintains durable docs/arc42/ chapters",
        default_enabled: true,
        materialization: Materialization::Files(&["docs/arc42", "docs/glossary.md"]),
        requires: &[],
    },
    AddonSpec {
        id: "communication_policy",
        description: "Communication policy — manages a fenced presentation and documentation block in AGENTS.md; the dialog obligation is independent of this addon",
        default_enabled: true,
        materialization: Materialization::ManagedBlock {
            file: "AGENTS.md",
            block: "communication-policy",
        },
        requires: &[],
    },
    AddonSpec {
        id: "readme",
        description: "README scaffolding — writes an initial README.md to the project root",
        default_enabled: true,
        materialization: Materialization::Files(&["README.md"]),
        requires: &[],
    },
    AddonSpec {
        id: "coverage",
        description: "Archive-level coverage gate — runs project measurement and merge scripts, updates README from the merged Cobertura XML",
        default_enabled: true,
        materialization: Materialization::None,
        requires: &[],
    },
    AddonSpec {
        id: "domain_modeling",
        description: "Skill — maintain the standalone glossary and structural domain model before design; does not require arc42",
        default_enabled: true,
        // `docs/glossary.md` is shared with `arc42` and is checked under
        // `arc42` only, so one addon's observed state cannot contradict the
        // other's (the report skips a path another addon declares).
        materialization: Materialization::FilesAndSkill {
            files: &["docs/glossary.md", "docs/domain-model.md"],
            skill: "givn-domain-modeling",
        },
        requires: &[],
    },
    AddonSpec {
        id: "dev_principles",
        description: "Skill — apply the team's engineering principles during implementation",
        default_enabled: true,
        materialization: Materialization::FilesAndSkill {
            files: &[],
            skill: "givn-dev-principles",
        },
        requires: &[],
    },
    AddonSpec {
        id: "architecture_principles",
        description: "Architecture-principles guidance — last-responsible-moment decision timing, consumer contracts, component splits and port/adapter boundaries; enabled by default",
        default_enabled: true,
        materialization: Materialization::None,
        requires: &[],
    },
    AddonSpec {
        id: "challenge",
        description: "Skill — research proposals, tests, specs, design, code, and whole changes before asking unresolved decisions; no lifecycle gate",
        default_enabled: true,
        materialization: Materialization::FilesAndSkill {
            files: &[],
            skill: "givn-challenge",
        },
        requires: &[],
    },
    AddonSpec {
        id: "outline",
        description: "Outline report — enabled by default; disable with `givn addons disable outline`",
        default_enabled: true,
        materialization: Materialization::None,
        requires: &[],
    },
    AddonSpec {
        id: "ise",
        description: "Ise rebuildability — serves `givn instructions ise`, adds the two change records to design and review guidance, and scaffolds givn/specification.yaml; opt-in and requires architecture_principles",
        default_enabled: false,
        materialization: Materialization::Files(&[crate::config::declaration::DECLARATION_PATH]),
        requires: &["architecture_principles"],
    },
];

impl AddonSpec {
    /// The declared default of one registered addon, or `None` when the id is
    /// not registered. This is the single declared default read by the
    /// listing, the lock proposal, and `givn addons enable`.
    pub fn declared_default(id: &str) -> Option<bool> {
        KNOWN_ADDONS
            .iter()
            .find_map(|addon| (addon.id == id).then_some(addon.default_enabled))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addon_registry_matches_the_manifest_schema_and_defaults() {
        let manifest = crate::config::manifest::Manifest::load_embedded().unwrap();
        assert_eq!(
            KNOWN_ADDONS.len(),
            crate::config::manifest::Addons::NAMES.len()
        );
        for addon in KNOWN_ADDONS {
            assert_eq!(
                manifest.addons.get(addon.id),
                Some(addon.default_enabled),
                "registry entry '{}' does not match the manifest schema",
                addon.id
            );
        }
        for name in crate::config::manifest::Addons::NAMES {
            assert!(
                KNOWN_ADDONS.iter().any(|addon| addon.id == name),
                "manifest addon '{name}' is missing from the registry"
            );
        }
    }

    #[test]
    fn ise_is_the_opt_in_addon_with_one_prerequisite() {
        let ise = KNOWN_ADDONS
            .iter()
            .find(|addon| addon.id == "ise")
            .expect("ise must be registered");
        assert!(!ise.default_enabled);
        assert_eq!(ise.requires, &["architecture_principles"]);
        assert_eq!(AddonSpec::declared_default("ise"), Some(false));
        assert_eq!(AddonSpec::declared_default("outline"), Some(true));
        assert_eq!(AddonSpec::declared_default("unknown"), None);
        for addon in KNOWN_ADDONS.iter().filter(|addon| addon.id != "ise") {
            assert!(
                addon.default_enabled,
                "every addon except the opt-in addon is default-on"
            );
            assert!(addon.requires.is_empty(), "{}", addon.id);
        }
    }
}
