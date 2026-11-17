//! The scenario-slug rule the visual-evidence paths must match.
//!
//! The product owns this rule (`src/integrity_hook.rs::scenario_slug`): it
//! names the directory the visual gate reads. A second, divergent copy would
//! make every capture invisible to the gate, so this copy is deliberately
//! literal and small, and the gate's own failure names the missing file.

/// Lowercase ASCII alphanumerics, one dash between runs of other characters.
pub fn scenario_slug(title: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for character in title.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(character);
        } else {
            pending_dash = true;
        }
    }
    slug
}
