#![feature(plugin)]

#[plugin]
extern crate peg_syntax_ext;

use std::io::File;

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

#[derive(Debug)]
pub struct Position {
	x: f32, y: f32, z: f32
}

impl Element for Position {
	fn check(_dummy: Option<Self>, spec: &parser::ElementSpec) -> Result<(), &'static str> {
		if spec.props.len() != 3 {
			return Err("Wrong number of params.")
		}
		for prop in spec.props.iter() {
			if prop.type_ != Type::Float && prop.type_ != Type::Double {
				return Err("Wrong type of params.");
			}
		}
		Ok(())
	}

	fn parse(line: &Vec<String>) -> Result<Self,&'static str> {
		let n: Vec<f32> = line.iter().filter_map(|a| a.parse()).collect();
		if n.len() != 3 {
			Err("Wrong number of elements on a line.")
		} else {
			Ok(Position { x: n[0], y: n[1], z: n[2] })
		}
	}
}
