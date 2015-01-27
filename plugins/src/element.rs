#![allow(unstable)]

use syntax::ast;
use syntax::ast::Mutability::MutImmutable;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic;
use syntax::ext::deriving::generic::ty;
//use syntax::parse::token;
use syntax::ptr::P;


pub fn element(ecx: &mut ExtCtxt, span: Span,
                meta_item: &ast::MetaItem, item: &ast::Item,
                mut push: Box<FnMut(P<ast::Item>)>)
{
    generic::TraitDef {
        span: span,
        attributes: Vec::new(),
        path: ty::Path {
            path: vec!["ply", "Element"],
            lifetime: None,
            params: Vec::new(),
            global: true,
        },
        additional_bounds: Vec::new(),
        generics: ty::LifetimeBounds::empty(),
        methods: vec![
            generic::MethodDef {
                name: "parse",
                generics: ty::LifetimeBounds::empty(),
                explicit_self: None,
                args: vec![ty::Ptr(Box::new( // &Vec<Self>
                        ty::Literal(
                            ty::Path {
								path: vec!["Vec"],
								lifetime: None,
								params: vec![Box::new(ty::Literal(
									    ty::Path::new(vec!["std","string","String"])
									))],
								global: false
							}
                        )
                    ), ty::PtrTy::Borrowed(None, MutImmutable)
                )],
                ret_ty: ty::Literal( // Result<Self, &'static str>
                    ty::Path {
                        path: vec!["Result"],
                        lifetime: None,
                        params: vec![
                            Box::new(ty::Ty::Self),
                            Box::new(ty::Ty::Ptr(Box::new(
                                    ty::Literal(
                                        ty::Path::new(vec!["str"])
                                    )
                                ), ty::PtrTy::Borrowed(Some("'static"), MutImmutable)
                            ))],
                        global: false
                    }
                ),
                attributes: vec![],
                combine_substructure: generic::combine_substructure(Box::new(parse_body)),

        }, generic::MethodDef {
	            name: "check",
	            generics: ty::LifetimeBounds::empty(),
	            explicit_self: None,
	            args: vec![ty::Literal( // Option<Self>
                    ty::Path {
						path: vec!["Option"],
						lifetime: None,
						params: vec![Box::new(ty::Self)],
						global: false
					}
	            ), ty::Ptr(Box::new( // &parser::ElementSpec
                        ty::Literal(
                            ty::Path::new(vec!["ply","ElementSpec"])
                        )
                    ), ty::PtrTy::Borrowed(None, MutImmutable)
                )],
	            ret_ty: ty::Literal( // Result<Self, &'static str>
	                ty::Path {
	                    path: vec!["Result"],
	                    lifetime: None,
	                    params: vec![
	                        Box::new(ty::Ty::Tuple(vec![])),
	                        Box::new(ty::Ty::Ptr(Box::new(
	                                ty::Literal(
	                                    ty::Path::new(vec!["str"])
	                                )
	                            ), ty::PtrTy::Borrowed(Some("'static"), MutImmutable)
	                        ))],
	                    global: false
	                }
	            ),
	            attributes: vec![],
	            combine_substructure: generic::combine_substructure(Box::new(check_body)),
	        },
	    ],
    }.expand(ecx, meta_item, item, |i| push(i));
}


// To print the expanded form, use `--pretty expanded -Z unstable-options`.
// Function body expansion
fn parse_body(ecx: &mut ExtCtxt, span: Span,
        substr: &generic::Substructure) -> P<ast::Expr>
{
    let ecx: &ExtCtxt = ecx;
    let self_ty = &substr.type_ident;

    match substr.fields { /*
        &generic::StaticStruct(ref definition, generic::Named(ref fields)) => {
            let field_count = definition.fields.len();

            let struct_expr = ecx.expr_struct_ident(span, *self_ty,
                fields.iter().zip(definition.fields.iter())
                             .map(|(&(field_ident, _), field_def)| {
                    let ref field_type = field_def.node.ty;
                    let ident_str = token::get_ident(field_ident);
                    let ident_str = ident_str.get();
                    ecx.field_imm(span, field_ident, quote_expr!(ecx, {

                        // Construct each element. This is inside a stuct initialization.
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

        },*/
		&generic::StaticStruct(ref definition, generic::Unnamed(ref fields)) => {
			let field_count = fields.len();
			let struct_expr = ecx.expr_tuple(span,
				definition.fields.iter().enumerate().map(|(i,field_def)| {
					quote_expr!(ecx, {
						match input[$i].parse() {
							Some(x) => x,
							None => return Err("Could not parse a number in the Data section.")
						}
					})
				}).collect()
			);
			quote_expr!(ecx, {

                let input: &Vec<String> = __arg_0;
        		if input.len() != $field_count {
        			return Err("The number of entries in Element is not correct.");
        		}

				Ok($self_ty $struct_expr)
			})
		}
        _ => {
            ecx.span_err(span, "Unable to implement `PlyModel` on a non-structure");
            ecx.expr_int(span, 0)
        }
    }
}

fn check_body(ecx: &mut ExtCtxt, span: Span,
        substr: &generic::Substructure) -> P<ast::Expr>
{
    let ecx: &ExtCtxt = ecx;
    //let self_ty = &substr.type_ident;

    match substr.fields {
		&generic::StaticStruct(ref definition, generic::Unnamed(ref fields)) => {
			let field_count = fields.len();
			quote_expr!(ecx, {
				if $field_count == __arg_1.props.len() {
					Ok(())
				} else {
					Err("Wrong number of properties.")
				}
			})
        },
		_ => {
            ecx.span_err(span, "Unable to implement `PlyModel` on a non-structure");
            ecx.expr_int(span, 0)
        }
    }
}
