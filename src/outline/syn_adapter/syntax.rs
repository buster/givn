//! Syn-to-neutral item and type mapping.

use super::super::model::Visibility;
use syn::{Attribute, Item, Meta, Type, Visibility as SynVisibility};

pub(super) fn type_label(ty: &Type) -> String {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_else(|| "type".to_string()),
        Type::Reference(reference) => format!("&{}", type_label(&reference.elem)),
        Type::Tuple(tuple) => format!("tuple{}", tuple.elems.len()),
        Type::Slice(slice) => format!("[{}]", type_label(&slice.elem)),
        Type::Array(array) => format!("[{}; _]", type_label(&array.elem)),
        _ => "type".to_string(),
    }
}

pub(super) fn impl_target_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

pub(super) fn visibility(vis: &SynVisibility) -> Visibility {
    match vis {
        SynVisibility::Public(_) => Visibility::Public,
        SynVisibility::Restricted(restricted) => {
            if restricted.path.is_ident("crate") {
                Visibility::Crate
            } else {
                Visibility::Restricted
            }
        }
        SynVisibility::Inherited => Visibility::Private,
    }
}

pub(super) fn attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Struct(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        _ => &[],
    }
}

pub(super) fn has_cfg_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        match &attr.meta {
            Meta::List(list) => list.tokens.to_string().trim() == "test",
            _ => false,
        }
    })
}
