#![feature(plugin,core,collections)]

#[plugin]
extern crate peg_syntax_ext;

pub use parser::{PLY,Format,Version,ElementSpec,PropertySpec,Type};
pub use property::Property;

pub mod parser;
pub mod property;


pub trait Model {
	fn new(&parser::PLY) -> Result<Self,String>;
}

pub trait Element: Sized {
	// dummy parameter until UFCS
	fn check(Option<Self>, &parser::ElementSpec) -> Result<(),String>;
	fn parse_one(&Vec<String>) -> Result<Self,String>;
	fn parse(e: &parser::ElementSpec) -> Result<Vec<Self>,String> {
		let mut res = Vec::with_capacity(e.data.len());
		for l in e.data.iter() {
			res.push(try!(Element::parse_one(l)));
		}
		Ok(res)
	}
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
