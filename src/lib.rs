#![feature(plugin)]

#[plugin]
extern crate peg_syntax_ext;

pub use parser::{PLY,Format,Version,ElementSpec,PropertySpec,Type};

pub mod parser;
pub mod property;


pub trait PlyModel {
	fn new(&parser::PLY) -> Result<Self,&'static str>;
}

pub trait ElementVec {
	fn check(Option<Self>, &parser::ElementSpec) -> Result<(), &'static str>;
}
impl<T:Element> ElementVec for Vec<T> {
	fn check(_dummy: Option<Self>, spec: &parser::ElementSpec) -> Result<(), &'static str> {
		Element::check(None::<T>, spec)
	}
}

pub trait Element {
	// dummy parameter until UFCS
	fn check(Option<Self>, &parser::ElementSpec) -> Result<(), &'static str>;
	fn parse(&Vec<String>) -> Result<Self,&'static str>;
}
