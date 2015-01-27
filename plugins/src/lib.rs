#![feature(plugin_registrar, quote)]
#![allow(unstable)]

extern crate rustc;
extern crate syntax;

mod data;
mod element;


#[doc(hidden)]
#[plugin_registrar]
pub fn registrar(registry: &mut rustc::plugin::Registry) {
    use syntax::parse::token::intern;
    use syntax::ext::base::Decorator;

    registry.register_syntax_extension(intern("ply_data"),
        Decorator(Box::new(data::ply_data)));
    registry.register_syntax_extension(intern("ply_element"),
        Decorator(Box::new(element::element)));
}
