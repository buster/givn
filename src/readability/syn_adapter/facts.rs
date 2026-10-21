//! AST visitor and method facts for the syn adapter.

use super::super::model::{
    Call, Flow, Method, OutlineItem, OutlineKind, RepoPath, Resolution, Signature, UnitOrigin,
    Visibility,
};
use super::spans::span_for_ident;
use super::syntax::type_label;
use syn::visit::Visit;
use syn::{
    Expr, ExprBinary, ExprCall, ExprField, ExprIf, ExprLoop, ExprMatch, ExprMethodCall, ExprTry,
    FnArg, Member, Pat, ReturnType,
};

pub(super) fn method_from_fn(
    sig: &syn::Signature,
    block: &syn::Block,
    file: &RepoPath,
    module_path: &str,
    trait_method: bool,
    visibility: Visibility,
    content: &str,
) -> Method {
    let name = sig.ident.to_string();
    let mut visitor = FactsVisitor::default();
    visitor.visit_block(block);
    let id = format!("{module_path}::{name}");
    Method {
        id,
        name,
        visibility,
        signature: signature(sig),
        span: span_for_ident(&sig.ident, content, file),
        flow: Flow::Block(visitor.flow),
        field_accesses: visitor.fields.into_iter().collect(),
        calls: visitor
            .calls
            .into_iter()
            .enumerate()
            .map(|(index, callee)| Call {
                order: index as u32,
                callee,
                resolution: Resolution::SameFile,
                target: None,
            })
            .collect(),
        is_trait_default: trait_method,
        origin: UnitOrigin::Production,
    }
}

pub(super) fn method_without_body(
    sig: &syn::Signature,
    file: &RepoPath,
    module_path: &str,
    visibility: Visibility,
    content: &str,
) -> Method {
    let name = sig.ident.to_string();
    Method {
        id: format!("{module_path}::{name}"),
        name,
        visibility,
        signature: signature(sig),
        span: span_for_ident(&sig.ident, content, file),
        flow: Flow::Block(Vec::new()),
        field_accesses: Vec::new(),
        calls: Vec::new(),
        is_trait_default: false,
        origin: UnitOrigin::Production,
    }
}

pub(super) fn outline_method(method: &Method, default: bool) -> OutlineItem {
    OutlineItem {
        kind: if default {
            OutlineKind::TraitMethod
        } else {
            OutlineKind::Method
        },
        name: method.name.clone(),
        visibility: method.visibility,
        signature: method.signature.text.clone(),
        order: method.span.start_line,
        calls: method.calls.clone(),
    }
}
#[derive(Default)]
struct FactsVisitor {
    fields: std::collections::BTreeSet<String>,
    calls: Vec<String>,
    flow: Vec<Flow>,
}

impl<'ast> Visit<'ast> for FactsVisitor {
    fn visit_expr_field(&mut self, node: &'ast ExprField) {
        if is_self(&node.base) {
            match &node.member {
                Member::Named(name) => {
                    self.fields.insert(name.to_string());
                }
                Member::Unnamed(index) => {
                    self.fields.insert(index.index.to_string());
                }
            }
        }
        syn::visit::visit_expr_field(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        self.calls.push(node.method.to_string());
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = &*node.func
            && let Some(segment) = path.path.segments.last()
        {
            self.calls.push(segment.ident.to_string());
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast ExprIf) {
        self.flow
            .push(Flow::Branch(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_if(self, node);
    }

    fn visit_expr_loop(&mut self, node: &'ast ExprLoop) {
        self.flow
            .push(Flow::Loop(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_loop(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.flow
            .push(Flow::Loop(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_while(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.flow
            .push(Flow::Loop(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast ExprMatch) {
        self.flow.push(Flow::Match {
            arms: node.arms.len() as u32,
            bodies: Vec::new(),
        });
        syn::visit::visit_expr_match(self, node);
    }

    fn visit_expr_try(&mut self, node: &'ast ExprTry) {
        self.flow.push(Flow::Try(Box::new(Flow::Block(Vec::new()))));
        syn::visit::visit_expr_try(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        if matches!(node.op, syn::BinOp::And(_) | syn::BinOp::Or(_)) {
            self.flow.push(Flow::ShortCircuit(
                Box::new(Flow::Block(Vec::new())),
                Box::new(Flow::Block(Vec::new())),
            ));
        }
        syn::visit::visit_expr_binary(self, node);
    }
}

fn is_self(expr: &Expr) -> bool {
    matches!(expr, Expr::Path(path) if path.path.is_ident("self"))
}
fn signature(sig: &syn::Signature) -> Signature {
    let mut text = String::new();
    if sig.constness.is_some() {
        text.push_str("const ");
    }
    if sig.asyncness.is_some() {
        text.push_str("async ");
    }
    if matches!(sig.safety, syn::Safety::Unsafe(_)) {
        text.push_str("unsafe ");
    }
    text.push_str("fn ");
    text.push_str(&sig.ident.to_string());
    text.push('(');
    for (index, arg) in sig.inputs.iter().enumerate() {
        if index > 0 {
            text.push_str(", ");
        }
        match arg {
            FnArg::Receiver(receiver) => {
                match &receiver.kind {
                    syn::ReceiverKind::Reference(_, _, mutable) => {
                        text.push('&');
                        if mutable.is_some() {
                            text.push_str("mut ");
                        }
                    }
                    syn::ReceiverKind::Value => {
                        if receiver.mutability.is_some() {
                            text.push_str("mut ");
                        }
                    }
                    syn::ReceiverKind::Typed(_, ty) => {
                        text.push_str("self: ");
                        text.push_str(&type_label(ty));
                        continue;
                    }
                    _ => {}
                }
                text.push_str("self");
            }
            FnArg::Typed(pat) => {
                if let Pat::Ident(ident) = &*pat.pat {
                    text.push_str(&ident.ident.to_string());
                } else {
                    text.push('_');
                }
                text.push_str(": ");
                text.push_str(&type_label(&pat.ty));
            }
        }
    }
    text.push(')');
    if let ReturnType::Type(_, ty) = &sig.output {
        text.push_str(" -> ");
        text.push_str(&type_label(ty));
    }
    Signature { text }
}
