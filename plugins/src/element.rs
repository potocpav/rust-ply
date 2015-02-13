
use syntax::ast;
use syntax::ast::Mutability::MutImmutable;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic;
use syntax::ext::deriving::generic::ty;
use syntax::parse::token;
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
                name: "parse_one",
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
                ret_ty: ty::Literal( // Result<Self, String>
                    ty::Path {
                        path: vec!["Result"],
                        lifetime: None,
                        params: vec![
                            Box::new(ty::Ty::Self),
                            Box::new(ty::Literal(
                                ty::Path::new(vec!["std","string","String"])
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
	            ), ty::Ptr(Box::new( // &ElementSpec
                        ty::Literal(
                            ty::Path::new(vec!["ply","ElementSpec"])
                        )
                    ), ty::PtrTy::Borrowed(None, MutImmutable)
                )],
	            ret_ty: ty::Literal( // Result<Self, String>
	                ty::Path {
	                    path: vec!["Result"],
	                    lifetime: None,
	                    params: vec![
	                        Box::new(ty::Ty::Tuple(vec![])),
	                        Box::new(ty::Literal(
                                ty::Path::new(vec!["std","string","String"])
                            ))],
	                    global: false
	                }
	            ),
	            attributes: vec![],
	            combine_substructure: generic::combine_substructure(Box::new(check_body)),
	        },
	    ],
        associated_types: vec![],
    }.expand(ecx, meta_item, item, |i| push(i));
}


// To print the expanded form, use `--pretty expanded -Z unstable-options`.
// Function 'parse' body expansion
fn parse_body(ecx: &mut ExtCtxt, span: Span,
        substr: &generic::Substructure) -> P<ast::Expr>
{
    let ecx: &ExtCtxt = ecx;
    let self_ty = &substr.type_ident;

    match substr.fields {
        &generic::StaticStruct(_, generic::Named(ref fields)) => {
            let field_count = fields.len();
			let mut initializers = Vec::with_capacity(field_count);
            for &(ident, _) in fields.iter() {
				initializers.push(ecx.field_imm(span, ident, quote_expr!(ecx, {

                    if let Some(p) = ply::Property::parse_prop(&mut input_it) {
                        p
                    } else {
                        return Err(format!("Error while parsing the data line with properties {:?}.", __arg_0))
                    }

				})));
            }
            let struct_expr = ecx.expr_struct_ident(span, *self_ty, initializers);

			quote_expr!(ecx, {
                let mut input_it = __arg_0.iter();
                let res = $struct_expr;
                if let Some(extra) = input_it.next() {
                    Err(format!("A superfluous property `{}` detected.", extra))
                } else {
                    Ok(res)
                }
			})

        },
		&generic::StaticStruct(ref definition, generic::Unnamed(_)) => {
            let field_count = definition.fields.len();
			let mut initializers = Vec::with_capacity(field_count);
            for _ in definition.fields.iter() {
				initializers.push(quote_expr!(ecx, {

                    if let Some(p) = ply::Property::parse_prop(&mut input_it) {
                        p
                    } else {
                        return Err(format!("Error while parsing the data line with properties {:?}.", __arg_0))
                    }

				}));
            }
            let struct_expr = ecx.expr_tuple(span, initializers);

			quote_expr!(ecx, {
                let mut input_it = __arg_0.iter();
                let res = $self_ty $struct_expr;
                if let Some(extra) = input_it.next() {
                    Err(format!("A superfluous property `{}` detected.", extra))
                } else {
                    Ok(res)
                }
			})
		},
        _ => {
            ecx.span_err(span, "Unable to implement `Model` on a non-structure");
            ecx.expr_int(span, 0)
        }
    }
}

// Function 'check' body expansion
fn check_body(ecx: &mut ExtCtxt, span: Span,
        substr: &generic::Substructure) -> P<ast::Expr>
{
    let ecx: &ExtCtxt = ecx;
    match substr.fields {
        // Handle both named and unnamed structs in a single branch
		&generic::StaticStruct(ref definition, ref fields) => {
			let field_count = definition.fields.len();

            let namecheck_block = match fields {
                &generic::Unnamed(_)       => ecx.block(span, vec![], None),
                &generic::Named(ref named) => ecx.block(span,
                    named.iter().enumerate().map(|(i, &(field_ident, _))| {
                        let ident_str = token::get_ident(field_ident);
                        let ident_str = &*ident_str;
                        ecx.stmt_expr(quote_expr!(ecx, {
                            if elem.props[$i].name != $ident_str {
                                return Err(format!("Field name `{}` does not match the property name `{}` \
                                    (in the Element `{}`).", $ident_str, elem.props[$i].name, elem.name));
                            }
                        }))
                    }).collect()
                , None)
            };

            let typecheck_block = ecx.block(span,
                definition.fields.iter().enumerate().map(|(i,def)| {
                    let ref prop_type = def.node.ty;
                    ecx.stmt_expr(quote_expr!(ecx, {

                        let expected_type = ply::Property::get_type(None::<$prop_type>);
                        if expected_type != elem.props[$i].type_ {
                            return Err(format!("Field type `{:?}` does not match the property type `{:?}` \
                                (in the Element `{}`).", expected_type, elem.props[$i].type_, elem.name));
                        }

                    }))
                }).collect(), None);

			quote_expr!(ecx, {

                let elem = __arg_1;
				if $field_count != elem.props.len() {
					return Err(format!("Wrong number of properties in the Element `{}` (\
                            PLY file: {}, structure: {}).",
                            elem.name, elem.props.len(), $field_count));
				}
                $namecheck_block
                $typecheck_block
                Ok(())

			})
        },
		_ => {
            ecx.span_err(span, "Unable to implement `Element` on a non-structure");
            ecx.expr_int(span, 0)
        }
    }
}
