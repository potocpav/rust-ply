#![feature(plugin_registrar)]
#![feature(quote)]
#![feature(unboxed_closures)]

extern crate rustc;
extern crate syntax;

mod data;


#[doc(hidden)]
#[plugin_registrar]
pub fn registrar(registry: &mut rustc::plugin::Registry) {
    use syntax::parse::token;

    registry.register_syntax_extension(token::intern("ply_data"),
        syntax::ext::base::Decorator(Box::new(data::ply_data)));
}
