use anyhow::Result;
use crate::config::manifest::Manifest;

/// Merge the project overlay (from `givn/config.yaml`) onto the embedded default manifest.
///
/// Overlay semantics (ADR-010, ADR-024):
/// - Artifacts keyed by `id`. New id = append. Existing id = deep-merge.
/// - `remove: true` on an artifact removes it from the effective graph.
/// - `optional: true` marks it skippable.
/// - Scalar fields (e.g. `verify.command`) in the overlay replace the default.
/// - `addons: { arc42: true }` expands to the arc42-docs artifact definition.
///
/// Not yet implemented: full validation (cycles, dangling requires, apply.requires
/// referencing removed artifacts). This is Phase 2.
pub fn apply(base: Manifest, _overlay_yaml: &str) -> Result<Manifest> {
    // Phase 2: implement full overlay + validation.
    // For now: return the base manifest unchanged.
    Ok(base)
}
