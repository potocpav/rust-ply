
use syntax::ast;
use syntax::ast::Mutability::MutImmutable;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic;
use syntax::ext::deriving::generic::ty;
use syntax::parse::token;
use syntax::ptr::P;


pub fn ply_data(ecx: &mut ExtCtxt, span: Span,
                meta_item: &ast::MetaItem, item: &ast::Item,
                mut push: Box<FnMut(P<ast::Item>)>)
{
    generic::TraitDef {
        span: span,
        attributes: Vec::new(),
        path: ty::Path {
            path: vec!["ply", "PlyModel"],
            lifetime: None,
            params: Vec::new(),
            global: true,
        },
        additional_bounds: Vec::new(),
        generics: ty::LifetimeBounds::empty(),
        methods: vec![
            generic::MethodDef {
                name: "new",
                generics: ty::LifetimeBounds::empty(),
                explicit_self: None,
                args: vec![ty::Ptr(Box::new(
                        ty::Literal(
                            ty::Path::new(vec!["ply","parser","PLY"])
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
                // If ever superfluous fields are allowed, check out how glium supresses warnings.
                attributes: vec![],
                combine_substructure: generic::combine_substructure(Box::new(body)),
            },
        ],
        associated_types: vec![],
    }.expand(ecx, meta_item, item, |i| push(i));
}


// To print the expanded form, use `--pretty expanded -Z unstable-options`.
// Function body expansion
fn body(ecx: &mut ExtCtxt, span: Span,
        substr: &generic::Substructure) -> P<ast::Expr>
{
    let ecx: &ExtCtxt = ecx;
    let self_ty = &substr.type_ident;

    match substr.fields {
        &generic::StaticStruct(ref definition, generic::Named(ref fields)) => {
            let field_count = fields.len();

            let struct_expr = ecx.expr_struct_ident(span, *self_ty,
                fields.iter().zip(definition.fields.iter())
                             .map(|(&(field_ident, _), field_def)| {
                    let ref field_type = field_def.node.ty;
                    let ident_str = token::get_ident(field_ident);
                    let ident_str = ident_str.get();
                    ecx.field_imm(span, field_ident, quote_expr!(ecx, {

                        // Construct each element. This is inside a stuct initialization.
                        if let Some(e) = ply.get_elem($ident_str.to_string()) {
                            try!(ply::ElementVec::check(None::<$field_type>, e));
                            try!(ply::Element::parse(e))
                        } else {
                            return Err(format!("Did not find a corresponding element name."));
                        }

                    }))
                }).collect()
            );
            quote_expr!(ecx, { // The function body

                let ply = __arg_0;
                // check
        		if ply.elements.len() != $field_count {
        			return Err(format!("The number of elements in PlyModel is not correct."));
        		}
                Ok($struct_expr)

            })
        },
        _ => {
            ecx.span_err(span, "Unable to implement `PlyModel` on a non-structure");
            ecx.expr_int(span, 0)
        }
    }
}
