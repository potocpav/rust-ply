#![feature(plugin,core,collections)]

#![plugin(peg_syntax_ext)]

extern crate peg_syntax_ext;

pub use parser::{PLY,Format,Version,ElementSpec,PropertySpec,Type,parse};
pub use property::Property;

pub mod parser;
pub mod property;


pub trait Model {
	fn new(&PLY) -> Result<Self,String>;
}

pub trait Element: Sized {
	// dummy parameter until UFCS
	fn check(Option<Self>, &ElementSpec) -> Result<(),String>;

	fn parse_one(&Vec<String>) -> Result<Self,String> {
		Err("`parse_one` must be implemented when used by the `parse` function.".to_string())
	}

	fn parse(e: &ElementSpec) -> Result<Vec<Self>,String> {
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
	fn check(Option<Self>, &ElementSpec) -> Result<(),String>;
}
impl<T:Element> ElementVec for Vec<T> {
	fn check(_dummy: Option<Self>, spec: &ElementSpec) -> Result<(),String> {
		Element::check(None::<T>, spec)
	}
}
