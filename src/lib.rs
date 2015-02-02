#![feature(plugin,core,collections)]

#[plugin]
extern crate peg_syntax_ext;

pub use parser::{PLY,Format,Version,ElementSpec,PropertySpec,Type};

pub mod parser;
pub mod property;


pub trait PlyModel {
	fn new(&parser::PLY) -> Result<Self,String>;
}

pub trait Element {
	// dummy parameter until UFCS
	fn check(Option<Self>, &parser::ElementSpec) -> Result<(),String>;
	fn parse(&Vec<String>) -> Result<Self,String>;
}


// Used in the macro expansions only.
#[doc(hidden)]
pub trait ElementVec {
	fn check(Option<Self>, &parser::ElementSpec) -> Result<(),String>;
}
impl<T:Element> ElementVec for Vec<T> {
	fn check(_dummy: Option<Self>, spec: &parser::ElementSpec) -> Result<(),String> {
		Element::check(None::<T>, spec)
	}
}
