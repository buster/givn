//! Syn-to-neutral item and type mapping.

use super::super::model::Visibility;
use syn::{
    Attribute, Expr, ExprLit, GenericArgument, Item, Lit, Meta, Path, PathArguments, ReturnType,
    Type, TypeParamBound, Visibility as SynVisibility,
};

pub(super) fn type_label(ty: &Type) -> String {
    match ty {
        Type::Path(path) => {
            if let Some(qself) = &path.qself {
                return qualified_path_label(qself, &path.path);
            }
            path_label(&path.path)
        }
        Type::Reference(reference) => {
            let mut text = String::from("&");
            if let Some(lifetime) = &reference.lifetime {
                text.push_str(&lifetime.to_string());
                text.push(' ');
            }
            if reference.mutability.is_some() {
                text.push_str("mut ");
            }
            text.push_str(&type_label(&reference.elem));
            text
        }
        Type::Ptr(pointer) => {
            let qualifier = match pointer.mutability {
                syn::PointerMutability::Mut(_) => "mut",
                syn::PointerMutability::Const(_) => "const",
            };
            format!("*{qualifier} {}", type_label(&pointer.elem))
        }
        Type::Tuple(tuple) => {
            let items: Vec<String> = tuple.elems.iter().map(type_label).collect();
            if items.len() == 1 {
                format!("({},)", items[0])
            } else {
                format!("({})", items.join(", "))
            }
        }
        Type::Slice(slice) => format!("[{}]", type_label(&slice.elem)),
        Type::Array(array) => format!(
            "[{}; {}]",
            type_label(&array.elem),
            expr_label(&array.len)
        ),
        Type::Paren(paren) => format!("({})", type_label(&paren.elem)),
        Type::Group(group) => type_label(&group.elem),
        Type::Never(_) => "!".to_string(),
        Type::Infer(_) => "_".to_string(),
        Type::TraitObject(object) => {
            format!("dyn {}", bounds_label(&object.bounds))
        }
        Type::ImplTrait(implemented) => {
            format!("impl {}", bounds_label(&implemented.bounds))
        }
        Type::FnPtr(function) => {
            let inputs: Vec<String> = function.inputs.iter().map(|arg| type_label(&arg.ty)).collect();
            let mut text = format!("fn({})", inputs.join(", "));
            if let ReturnType::Type(_, ty) = &function.output {
                text.push_str(" -> ");
                text.push_str(&type_label(ty));
            }
            text
        }
        Type::Macro(mac) => format!(
            "{}!({})",
            path_label(&mac.mac.path),
            mac.mac.tokens
        ),
        Type::Verbatim(tokens) => tokens.to_string(),
        _ => "type".to_string(),
    }
}

fn qualified_path_label(qself: &syn::QSelf, path: &Path) -> String {
    let position = qself.position.min(path.segments.len());
    let trait_path = segments_label(path.segments.iter().take(position));
    let associated = segments_label(path.segments.iter().skip(position));
    if trait_path.is_empty() {
        format!("<{}>::{associated}", type_label(&qself.ty))
    } else {
        format!("<{} as {trait_path}>::{associated}", type_label(&qself.ty))
    }
}

fn path_label(path: &Path) -> String {
    let body = segments_label(path.segments.iter());
    if path.leading_colon.is_some() {
        format!("::{body}")
    } else {
        body
    }
}

fn segments_label<'a>(segments: impl Iterator<Item = &'a syn::PathSegment>) -> String {
    segments
        .map(|segment| {
            format!(
                "{}{}",
                segment.ident,
                path_arguments_label(&segment.arguments)
            )
        })
        .collect::<Vec<_>>()
        .join("::")
}

fn path_arguments_label(arguments: &PathArguments) -> String {
    match arguments {
        PathArguments::None => String::new(),
        PathArguments::AngleBracketed(arguments) => {
            let items: Vec<String> = arguments
                .args
                .iter()
                .map(generic_argument_label)
                .collect();
            format!("<{}>", items.join(", "))
        }
        PathArguments::Parenthesized(arguments) => {
            let inputs: Vec<String> = arguments
                .inputs
                .iter()
                .map(|input| type_label(&input.ty))
                .collect();
            let mut text = format!("({})", inputs.join(", "));
            if let ReturnType::Type(_, ty) = &arguments.output {
                text.push_str(" -> ");
                text.push_str(&type_label(ty));
            }
            text
        }
    }
}

fn generic_argument_label(argument: &GenericArgument) -> String {
    match argument {
        GenericArgument::Lifetime(lifetime) => lifetime.to_string(),
        GenericArgument::Type(ty) => type_label(ty),
        GenericArgument::Const(value) => expr_label(value),
        GenericArgument::AssocType(binding) => {
            let generics = binding
                .generics
                .as_ref()
                .map(|generics| {
                    let items: Vec<String> = generics
                        .args
                        .iter()
                        .map(generic_argument_label)
                        .collect();
                    format!("<{}>", items.join(", "))
                })
                .unwrap_or_default();
            format!("{}{generics} = {}", binding.ident, type_label(&binding.ty))
        }
        GenericArgument::AssocConst(binding) => {
            let generics = binding
                .generics
                .as_ref()
                .map(|generics| {
                    let items: Vec<String> = generics
                        .args
                        .iter()
                        .map(generic_argument_label)
                        .collect();
                    format!("<{}>", items.join(", "))
                })
                .unwrap_or_default();
            format!("{}{generics} = {}", binding.ident, expr_label(&binding.value))
        }
        GenericArgument::Constraint(constraint) => {
            let generics = constraint
                .generics
                .as_ref()
                .map(|generics| {
                    let items: Vec<String> = generics
                        .args
                        .iter()
                        .map(generic_argument_label)
                        .collect();
                    format!("<{}>", items.join(", "))
                })
                .unwrap_or_default();
            format!(
                "{}{generics}: {}",
                constraint.ident,
                bounds_label(&constraint.bounds)
            )
        }
        _ => "_".to_string(),
    }
}

fn bounds_label(bounds: &syn::punctuated::Punctuated<TypeParamBound, syn::Token![+]>) -> String {
    bounds
        .iter()
        .map(|bound| match bound {
            TypeParamBound::Trait(bound) => path_label(&bound.path),
            TypeParamBound::Lifetime(lifetime) => lifetime.to_string(),
            _ => "_".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn expr_label(expr: &Expr) -> String {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(value),
            ..
        }) => value.base10_digits().to_string(),
        Expr::Path(path) => path_label(&path.path),
        _ => "_".to_string(),
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
