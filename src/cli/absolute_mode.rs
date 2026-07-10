/// Manages a fenced `<!-- givn:begin:<name> -->` / `<!-- givn:end:<name> -->`
/// block inside a project's `AGENTS.md`.
///
/// This is the one deliberate, narrow exception to "givn never writes
/// AGENTS.md" — givn owns exactly the content between its own markers and
/// never touches anything else in the file. The marker scheme is generic
/// (`givn:begin:<name>`) so future addons can manage their own blocks the
/// same way.
use anyhow::Result;

const BLOCK_NAME: &str = "absolute-mode";

/// Ensure the absolute-mode block is present in AGENTS.md with current
/// policy content. Creates AGENTS.md if it does not exist. Idempotent:
/// re-running with the same policy text is a no-op; re-running with changed
/// policy text updates only the block.
pub fn apply(project_root: &std::path::Path) -> Result<()> {
    let policy = crate::config::embedded::Assets::get_str("addons/absolute-mode/policy.md")
        .ok_or_else(|| anyhow::anyhow!("embedded addons/absolute-mode/policy.md not found"))?;

    let path = project_root.join("AGENTS.md");
    let existing = if path.exists() {
        std::fs::read_to_string(&path)?
    } else {
        String::new()
    };

    let updated = upsert_block(&existing, BLOCK_NAME, policy.trim_end());
    if updated != existing {
        std::fs::write(&path, updated)?;
    }
    Ok(())
}

/// Remove the absolute-mode block from AGENTS.md. Leaves all other content
/// untouched. If AGENTS.md becomes empty as a result AND it contained
/// nothing but givn-managed blocks (i.e. was created by `apply` with no
/// user content ever added), delete the file. Otherwise leave it, even if
/// empty — an empty file the user created is still theirs.
pub fn remove(project_root: &std::path::Path) -> Result<()> {
    let path = project_root.join("AGENTS.md");
    if !path.exists() {
        return Ok(());
    }

    let existing = std::fs::read_to_string(&path)?;
    let updated = remove_block(&existing, BLOCK_NAME);

    if updated.trim().is_empty() && only_ever_contained_block(&existing, BLOCK_NAME) {
        std::fs::remove_file(&path)?;
    } else if updated != existing {
        std::fs::write(&path, updated)?;
    }
    Ok(())
}

fn begin_marker(name: &str) -> String {
    format!("<!-- givn:begin:{name} -->")
}

fn end_marker(name: &str) -> String {
    format!("<!-- givn:end:{name} -->")
}

/// Insert or replace the named block in `content`. If the block markers
/// already exist, the content between them is replaced in place (surrounding
/// text untouched). If the markers do not exist, the block is appended at
/// the end (preceded by a blank line if `content` is non-empty).
fn upsert_block(content: &str, name: &str, body: &str) -> String {
    let begin = begin_marker(name);
    let end = end_marker(name);
    let block = format!("{begin}\n{body}\n{end}");

    if let Some((before, after)) = split_on_block(content, &begin, &end) {
        format!("{before}{block}{after}")
    } else if content.trim().is_empty() {
        format!("{block}\n")
    } else {
        let sep = if content.ends_with('\n') {
            "\n"
        } else {
            "\n\n"
        };
        format!("{content}{sep}{block}\n")
    }
}

/// Remove the named block (and one adjacent blank line, if any) from `content`.
/// Surrounding content is preserved exactly.
fn remove_block(content: &str, name: &str) -> String {
    let begin = begin_marker(name);
    let end = end_marker(name);

    let Some((before, after)) = split_on_block(content, &begin, &end) else {
        return content.to_string();
    };

    // Trim exactly one trailing blank line from `before` and one leading
    // blank line from `after`, so removing the block doesn't leave a gap.
    let before = match before.strip_suffix("\n\n") {
        Some(s) => format!("{s}\n"),
        None => before.to_string(),
    };
    let after = after.strip_prefix('\n').unwrap_or(after);

    format!("{before}{after}")
}

/// Find the byte ranges before/after a named block's markers, if present.
/// Returns `(before, after)` where `before` includes everything up to and
/// including the character preceding `begin`, and `after` includes
/// everything from the character following `end` onward.
fn split_on_block<'a>(content: &'a str, begin: &str, end: &str) -> Option<(&'a str, &'a str)> {
    let start = content.find(begin)?;
    let rest = &content[start..];
    let end_rel = rest.find(end)?;
    let end_abs = start + end_rel + end.len();
    Some((&content[..start], &content[end_abs..]))
}

/// True if `content`, once the named block is stripped, is empty — meaning
/// the file (as it existed before this removal) contained nothing but that
/// block and possibly surrounding whitespace. Used to decide whether givn
/// may delete the file on `remove`.
fn only_ever_contained_block(content: &str, name: &str) -> bool {
    remove_block(content, name).trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_creates_block_in_empty_content() {
        let result = upsert_block("", "absolute-mode", "POLICY TEXT");
        assert_eq!(
            result,
            "<!-- givn:begin:absolute-mode -->\nPOLICY TEXT\n<!-- givn:end:absolute-mode -->\n"
        );
    }

    #[test]
    fn upsert_appends_block_after_existing_content() {
        let existing = "# AGENTS.md\n\nSome user content.\n";
        let result = upsert_block(existing, "absolute-mode", "POLICY TEXT");
        assert!(result.starts_with(existing));
        assert!(result.contains(
            "<!-- givn:begin:absolute-mode -->\nPOLICY TEXT\n<!-- givn:end:absolute-mode -->"
        ));
    }

    #[test]
    fn upsert_replaces_existing_block_in_place() {
        let existing = "# AGENTS.md\n\n<!-- givn:begin:absolute-mode -->\nOLD TEXT\n<!-- givn:end:absolute-mode -->\n\nAfter block.\n";
        let result = upsert_block(existing, "absolute-mode", "NEW TEXT");
        assert!(result.contains("NEW TEXT"));
        assert!(!result.contains("OLD TEXT"));
        assert!(result.starts_with("# AGENTS.md\n\n"));
        assert!(result.ends_with("After block.\n"));
    }

    #[test]
    fn upsert_is_idempotent() {
        let once = upsert_block("# AGENTS.md\n", "absolute-mode", "POLICY TEXT");
        let twice = upsert_block(&once, "absolute-mode", "POLICY TEXT");
        assert_eq!(once, twice);
    }

    #[test]
    fn upsert_never_touches_unrelated_blocks() {
        let existing = "<!-- givn:begin:other -->\nOTHER\n<!-- givn:end:other -->\n";
        let result = upsert_block(existing, "absolute-mode", "POLICY TEXT");
        assert!(result.contains("<!-- givn:begin:other -->\nOTHER\n<!-- givn:end:other -->"));
        assert!(result.contains("absolute-mode"));
    }

    #[test]
    fn remove_strips_block_and_preserves_surrounding_content() {
        let existing = "# AGENTS.md\n\nUser content before.\n\n<!-- givn:begin:absolute-mode -->\nPOLICY TEXT\n<!-- givn:end:absolute-mode -->\n\nUser content after.\n";
        let result = remove_block(existing, "absolute-mode");
        assert!(result.contains("User content before."));
        assert!(result.contains("User content after."));
        assert!(!result.contains("POLICY TEXT"));
        assert!(!result.contains("givn:begin:absolute-mode"));
    }

    #[test]
    fn remove_on_content_without_block_is_noop() {
        let existing = "# AGENTS.md\n\nNo block here.\n";
        let result = remove_block(existing, "absolute-mode");
        assert_eq!(result, existing);
    }

    #[test]
    fn only_ever_contained_block_true_for_block_only_file() {
        let content =
            "<!-- givn:begin:absolute-mode -->\nPOLICY TEXT\n<!-- givn:end:absolute-mode -->\n";
        assert!(only_ever_contained_block(content, "absolute-mode"));
    }

    #[test]
    fn only_ever_contained_block_false_when_user_content_present() {
        let content = "# My AGENTS.md\n\n<!-- givn:begin:absolute-mode -->\nPOLICY TEXT\n<!-- givn:end:absolute-mode -->\n";
        assert!(!only_ever_contained_block(content, "absolute-mode"));
    }
}
