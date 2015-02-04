#![feature(plugin_registrar, quote, rustc_private)]

extern crate rustc;
extern crate syntax;

mod data;
mod element;


#[doc(hidden)]
#[plugin_registrar]
pub fn registrar(registry: &mut rustc::plugin::Registry) {
    use syntax::parse::token::intern;
    use syntax::ext::base::Decorator;

    registry.register_syntax_extension(intern("ply_model"),
        Decorator(Box::new(data::ply_model)));
    registry.register_syntax_extension(intern("ply_element"),
        Decorator(Box::new(element::element)));
}
