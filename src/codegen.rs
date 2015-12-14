#![feature(plugin_registrar, rustc_private, slice_patterns, plugin)]
#![plugin(quasi_macros)]

extern crate rustc_plugin;
extern crate syntax;
#[macro_use] extern crate lazy_static;
extern crate quasi;

use std::collections::HashMap;
use std::sync::Mutex;

use syntax::ast::{MetaItem, Ident, ItemImpl, ImplItemKind, TokenTree, Arm, Ty_};
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::Token;
use syntax::ast_util::path_to_ident;

lazy_static! {
    static ref HANDLERS: Mutex<HashMap<String, Vec<(Ident, Ident)>>> = Mutex::new(HashMap::new());
}

pub fn register_handlers(cx: &mut ExtCtxt, _: Span, _: &MetaItem, annotatable: &Annotatable,
                         _: &mut FnMut(Annotatable)) {
    if let Annotatable::Item(ref item) = *annotatable {
        if let ItemImpl(_, _, _, _, _, ref list) = item.node {
            for impl_item in list {
                if let ImplItemKind::Method(ref sig, _) = impl_item.node {
                    let mut table = HANDLERS.lock().unwrap();
                    if let Ty_::TyPath(_, ref path) = sig.decl.inputs[2].ty.node {
                        let base_mod = format!("{:?}", cx.mod_path()[2]);
                        let handlers = table.entry(base_mod).or_insert(Vec::new());
                        handlers.push((path_to_ident(path).unwrap(), impl_item.ident));
                    }
                }
            }
        }
    }
}

fn handle(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let (session, ch_ref, id, buffer) = match args {
        [TokenTree::Token(_, Token::Ident(session, _)),
         TokenTree::Token(_, Token::Comma),
         TokenTree::Token(_, Token::Ident(ch_ref, _)),
         TokenTree::Token(_, Token::Comma),
         TokenTree::Token(_, Token::Ident(id, _)),
         TokenTree::Token(_, Token::Comma),
         TokenTree::Token(_, Token::Ident(buffer, _))] => (session, ch_ref, id, buffer),
        _ => {
            cx.span_err(sp, "arguments should be four comma separated identifiers");
            return DummyResult::any(sp);
        }
    };

    let base_mod = format!("{:?}", cx.mod_path()[2]);
    let handlers = HANDLERS.lock().unwrap();
    let mut arms = Vec::new();
    for hdl in &*handlers.get(&base_mod).unwrap() {
        let name = hdl.0;
        let handler = hdl.1;

        let guard = quote_expr!(cx, $id == $name::id());
        let body = quote_expr!(
            cx,
            $session.$handler($ch_ref, try!($name::deserialize(&mut $buffer)))
        );

        arms.push(Arm {
            attrs: Vec::new(),
            pats: vec![cx.pat_wild(sp)],
            guard: Some(guard),
            body: body,
        });
    }

    let unit = quote_expr!(cx, Ok(()));
    arms.push(Arm {
        attrs: Vec::new(),
        pats: vec![cx.pat_wild(sp)],
        guard: None,
        body: unit,
    });

    MacEager::expr(cx.expr_match(sp, cx.expr_ident(sp, id), arms))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    use syntax::parse::token::intern;
    use syntax::ext::base::MultiDecorator;

    reg.register_syntax_extension(
        intern("register_handlers"),
        MultiDecorator(Box::new(register_handlers))
    );

    reg.register_macro("handle", handle);
}
