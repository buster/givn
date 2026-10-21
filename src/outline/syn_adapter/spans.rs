//! Source span mapping for the syn adapter.

use super::super::model::{RepoPath, SourceSpan};
use super::syntax::attrs;
use syn::{ImplItem, Item, ItemImpl};

pub(super) fn span_for_ident(ident: &syn::Ident, content: &str, file: &RepoPath) -> SourceSpan {
    line_span(content, file, ident.span().start().line as u32)
}

pub(super) fn item_span(item: &Item, content: &str, file: &RepoPath) -> SourceSpan {
    let mut line = match item {
        Item::Struct(item) => item.ident.span().start().line,
        Item::Enum(item) => item.ident.span().start().line,
        Item::Union(item) => item.ident.span().start().line,
        Item::Trait(item) => item.ident.span().start().line,
        Item::Type(item) => item.ident.span().start().line,
        Item::Fn(item) => item.sig.ident.span().start().line,
        Item::Mod(item) => item.mod_token.span.start().line,
        Item::Impl(item) => item.impl_token.span.start().line,
        Item::Macro(item) => macro_line(item) as usize,
        _ => 1,
    };
    if let Some(attribute) = attrs(item).first()
        && let Some(segment) = attribute.path().segments.first()
    {
        line = line.min(segment.ident.span().start().line);
    }
    line_span(content, file, line as u32)
}

pub(super) fn span_with_attrs(
    attributes: &[syn::Attribute],
    ident: &syn::Ident,
    content: &str,
    file: &RepoPath,
) -> SourceSpan {
    let mut line = ident.span().start().line;
    if let Some(attribute) = attributes.first()
        && let Some(segment) = attribute.path().segments.first()
    {
        line = line.min(segment.ident.span().start().line);
    }
    line_span(content, file, line as u32)
}

pub(super) fn field_span(field: &syn::Field, file: &RepoPath) -> SourceSpan {
    let line = field
        .ident
        .as_ref()
        .map(|ident| ident.span().start().line as u32)
        .unwrap_or(1);
    SourceSpan {
        file: file.clone(),
        start_line: line,
        end_line: line,
    }
}

pub(super) fn macro_line(item: &syn::ItemMacro) -> u32 {
    item.mac
        .path
        .segments
        .last()
        .map(|segment| segment.ident.span().start().line as u32)
        .unwrap_or(1)
}

pub(super) fn impl_span_of(item: &ItemImpl, content: &str, file: &RepoPath) -> SourceSpan {
    let first_method = item
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(function) => Some(function.sig.ident.span().start().line as u32),
            _ => None,
        })
        .unwrap_or(1);
    let lines: Vec<&str> = content.lines().collect();
    let start = (1..=first_method)
        .rev()
        .find(|line| {
            lines
                .get(*line as usize - 1)
                .is_some_and(|text| text.trim_start().starts_with("impl"))
        })
        .unwrap_or(first_method);
    line_span(content, file, start)
}

pub(super) fn line_span(content: &str, file: &RepoPath, start_line: u32) -> SourceSpan {
    let lines: Vec<&str> = content.lines().collect();
    let start_index = start_line.saturating_sub(1) as usize;
    let mut depth = 0i32;
    let mut seen = false;
    let mut end_line = start_line;
    for (index, line) in lines.iter().enumerate().skip(start_index) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    seen = true;
                }
                '}' => {
                    depth -= 1;
                    if seen && depth <= 0 {
                        end_line = index as u32 + 1;
                        return SourceSpan {
                            file: file.clone(),
                            start_line,
                            end_line,
                        };
                    }
                }
                _ => {}
            }
        }
        end_line = index as u32 + 1;
    }
    SourceSpan {
        file: file.clone(),
        start_line,
        end_line: if seen { end_line } else { start_line },
    }
}
