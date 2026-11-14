//! The bounded project walk and the declaration-pattern matcher shared by the
//! specification-health pass and the declared `e2e` boundary.

/// One entry of the bounded project walk: a project-relative path and whether
/// it is a directory.
pub(crate) struct ProjectPath {
    pub(crate) path: String,
    pub(crate) is_dir: bool,
}

/// Walk the project under the finding exclusions. Paths are project-relative
/// and use `/`; version-control, build, archive, and active-change trees are
/// skipped because they are not retained behavior.
pub(crate) fn walk_project(project_root: &std::path::Path) -> Vec<ProjectPath> {
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

/// Whether a project-relative path belongs to a tree neither pass scans.
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

/// One compiled role pattern. `*` matches within one segment, `?` matches one
/// character, and a `**` segment matches zero or more segments. Everything else
/// is literal, so a pattern without wildcards is an existence check.
pub(crate) struct Glob {
    segments: Vec<String>,
}

impl Glob {
    /// Compile a declared pattern. `.` and repeated separators disappear and an
    /// inner `..` cancels the segment before it, so a pattern accepted by the
    /// declaration loader matches the path it spells.
    pub(crate) fn compile(pattern: &str) -> Self {
        let mut segments: Vec<String> = Vec::new();
        for component in std::path::Path::new(pattern).components() {
            match component {
                std::path::Component::Normal(segment) => {
                    segments.push(segment.to_string_lossy().into_owned())
                }
                std::path::Component::ParentDir => {
                    segments.pop();
                }
                std::path::Component::CurDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_) => {}
            }
        }
        Glob { segments }
    }

    pub(crate) fn matches(&self, path: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_star_matches_within_one_segment() {
        let glob = Glob::compile("givn/specs/*.feature");
        assert!(glob.matches("givn/specs/a.feature"));
        assert!(!glob.matches("givn/specs/nested/a.feature"));
    }

    #[test]
    fn a_question_mark_matches_one_character() {
        let glob = Glob::compile("src/?.rs");
        assert!(glob.matches("src/a.rs"));
        assert!(!glob.matches("src/ab.rs"));
        assert!(!glob.matches("src/.rs"));
    }

    #[test]
    fn wildcards_count_a_multi_byte_character_once() {
        assert!(Glob::compile("src/?.rs").matches("src/é.rs"));
        assert!(Glob::compile("sp*/x").matches("spéc/x"));
        assert!(!Glob::compile("src/??.rs").matches("src/é.rs"));
    }

    #[test]
    fn an_inner_parent_segment_compiles_to_the_same_pattern() {
        let glob = Glob::compile("docs/../docs/glossary.md");
        assert!(glob.matches("docs/glossary.md"));
        assert!(!glob.matches("docs/../docs/glossary.md"));
    }

    #[test]
    fn an_empty_pattern_matches_the_project_root_only() {
        assert!(Glob::compile(".").matches(""));
        assert!(!Glob::compile(".").matches("src"));
    }
}
