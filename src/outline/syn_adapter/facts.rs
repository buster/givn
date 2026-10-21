//! AST visitor and method facts for the syn adapter.

use super::super::model::{
    Call, CallShape, Flow, Method, OutlineItem, OutlineKind, ReceiverEvidence, RepoPath,
    Resolution, Signature, UnitOrigin, Visibility,
};
use super::spans::span_for_ident;
use super::syntax::type_label;
use syn::parse::Parser;
use syn::{
    Expr, ExprBinary, ExprCall, ExprField, ExprIf, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall,
    ExprTry, FnArg, Member, Pat, ReturnType, visit::Visit,
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
            .map(|(index, fact)| Call {
                order: index as u32,
                callee: fact.callee,
                shape: fact.shape,
                resolution: Resolution::External,
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
        id: method.id.clone(),
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
struct CallFact {
    callee: String,
    shape: CallShape,
}

#[derive(Default)]
struct FactsVisitor {
    fields: std::collections::BTreeSet<String>,
    calls: Vec<CallFact>,
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
        self.calls.push(CallFact {
            callee: node.method.to_string(),
            shape: CallShape::Method {
                receiver: receiver_evidence(&node.receiver),
            },
        });
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = &*node.func
            && let Some(segment) = path.path.segments.last()
        {
            let qualifier = if let Some(qself) = &path.qself {
                Some(type_owner(&qself.ty))
            } else {
                let prefix: Vec<String> = path
                    .path
                    .segments
                    .iter()
                    .take(path.path.segments.len().saturating_sub(1))
                    .map(|segment| segment.ident.to_string())
                    .collect();
                if prefix.is_empty() {
                    None
                } else {
                    Some(prefix.join("::"))
                }
            };
            self.calls.push(CallFact {
                callee: segment.ident.to_string(),
                shape: CallShape::Function { qualifier },
            });
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        // `syn` does not parse macro tokens. Expression-list macros such as
        // `format!`, `println!`, and `vec!` contain calls that belong to the
        // enclosing method; visit the arguments when they parse as expressions.
        let parser = syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated;
        if let Ok(expressions) = parser.parse2(node.mac.tokens.clone()) {
            for expression in expressions {
                self.visit_expr(&expression);
            }
        }
        syn::visit::visit_expr_macro(self, node);
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

fn receiver_evidence(expr: &Expr) -> ReceiverEvidence {
    let Expr::Path(path) = expr else {
        return ReceiverEvidence::Unknown;
    };
    if path.path.is_ident("self") || path.path.is_ident("Self") {
        return ReceiverEvidence::SelfUnit;
    }
    let Some(segment) = path.path.segments.last() else {
        return ReceiverEvidence::Unknown;
    };
    let name = segment.ident.to_string();
    if name
        .chars()
        .next()
        .is_some_and(|first| first.is_uppercase())
    {
        ReceiverEvidence::Qualifier(name)
    } else {
        ReceiverEvidence::Unknown
    }
}

fn type_owner(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_else(|| "type".to_string()),
        _ => "type".to_string(),
    }
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

#[cfg(test)]
mod tests {
    use super::FactsVisitor;
    use syn::visit::Visit;

    #[test]
    fn macro_arguments_contribute_calls() {
        let block: syn::Block =
            syn::parse_str("{ let _ = format!(\"{}\", self.last_done_id()); }").unwrap();
        let mut visitor = FactsVisitor::default();
        visitor.visit_block(&block);
        let names: Vec<&str> = visitor
            .calls
            .iter()
            .map(|call| call.callee.as_str())
            .collect();
        assert_eq!(names, vec!["last_done_id"]);
    }
}
