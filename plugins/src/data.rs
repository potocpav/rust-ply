//
// This is a bit of cleanup work to clean out some old deprecated flags and
// deprecated lint names from the compiler (they've been deprecated for quite awhile now).
//
// This also notably puts --pretty behind the -Z unstable-options flag
// (where it was supposed to be previously).



use syntax::ast;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic;
use syntax::codemap;
use syntax::parse::token;
use syntax::ptr::P;

use syntax::ext::deriving::generic::ty::{Ty,PtrTy};
use syntax::ast::Mutability::MutImmutable;


pub fn ply_data(ecx: &mut base::ExtCtxt, span: codemap::Span,
                meta_item: &ast::MetaItem, item: &ast::Item,
                mut push: Box<FnMut(P<ast::Item>)>)
{
//
    generic::TraitDef {
        span: span,
        attributes: Vec::new(),
        path: generic::ty::Path {
            path: vec!["ply", "PlyModel"],
            lifetime: None,
            params: Vec::new(),
            global: true,
        },
        additional_bounds: Vec::new(),
        generics: generic::ty::LifetimeBounds::empty(),
        methods: vec![
            generic::MethodDef {
                name: "new",
                generics: generic::ty::LifetimeBounds::empty(),
                explicit_self: None,
                args: vec![generic::ty::Ptr(Box::new(
                        generic::ty::Literal(
                            generic::ty::Path::new(vec!["ply","parser","PLY"])
                        )
                    ), PtrTy::Borrowed(None, MutImmutable)
                )],
                ret_ty: generic::ty::Literal( // Result<Self, &'static str>
                    generic::ty::Path {
                        path: vec!["std", "result", "Result"],
                        lifetime: None,
                        params: vec![
                            Box::new(Ty::Self),
                            Box::new(Ty::Ptr(Box::new(
                                    generic::ty::Literal(
                                        generic::ty::Path::new(vec!["str"])
                                    )
                                ), PtrTy::Borrowed(Some("'static"), MutImmutable)
                            ))],
                        global: true
                    }
                ),
                attributes: vec![/*
                    ecx.attribute(span.clone(), ecx.meta_list(span.clone(),
                        token::InternedString::new("allow"),
                        vec![ecx.meta_word(span.clone(),
                                token::InternedString::new("unused_assignments"))]
                    ))*/
                ],
                combine_substructure: generic::combine_substructure(Box::new(body)),
            },
        ],
    }.expand(ecx, meta_item, item, |i| push(i));
}


fn body(ecx: &mut base::ExtCtxt, span: codemap::Span,
        substr: &generic::Substructure) -> P<ast::Expr>
{
    let ecx: &base::ExtCtxt = ecx;
    let self_ty = &substr.type_ident;

    match substr.fields {
        &generic::StaticStruct(ref definition, generic::Named(ref fields)) => {
            let field_count = definition.fields.len();

            let struct_expr = ecx.expr_struct_ident(span, *self_ty,
                fields.iter().zip(definition.fields.iter())
                             .map(|(&(field_ident, _), field_def)| {
                    let ref field_type = field_def.node.ty;
                    let ident_str = token::get_ident(field_ident);
                    let ident_str = ident_str.get();
                    ecx.field_imm(span, field_ident, quote_expr!(ecx, {

                        if let Some(e) = ply.elements.iter()
                                         .filter(|&e| e.name == $ident_str).next() {
                            try!(ply::ElementVec::check(None::<$field_type>, e));
                            let mut accum = vec![];
                            for line in e.data.iter() {
                                let res = try!(ply::Element::parse(line));
                                accum.push(res);
                            }
                            accum
                        } else {
                            return Err("Did not find a corresponding element name.");
                        }

                    }))
                }).collect()
            );
            quote_expr!(ecx, {

                let ply = __arg_0;
                // check
        		if ply.elements.len() != $field_count {
        			return Err("The number of elements in PlyModel is not correct.");
        		}
                let ret = $struct_expr;
                Ok(ret)

            })
        },
        _ => {
            ecx.span_err(span, "Unable to implement `PlyModel` on a non-structure");
            ecx.expr_int(span, 0)
        }
    }
}
