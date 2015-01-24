#![feature(plugin)]

#[plugin]
extern crate peg_syntax_ext;
#[plugin]
extern crate ply_plugins;

use std::io::File;

pub use parser::{PLY,Format,Version,ElementSpec,PropertySpec,Type};

pub mod parser;

#[ply_data]
struct S;

pub trait PlyModel {
	fn new(&parser::PLY) -> Result<Self,&'static str>;
}

#[derive(Debug)]
pub struct TestObj {
	vertices: Vec<Position>
}

impl PlyModel for TestObj {
	fn new(ply: &parser::PLY) -> Result<Self,&'static str> {
		// check
		if ply.elements.len() != 1 {
			return Err("Wrong number of elements.");
		}
		try!(Element::check(None::<Position>, &ply.elements[0]));

		// parse
		let mut lines: Vec<&str> = ply.data.split('\n').collect();
		let mut res: Vec<Position> = Vec::with_capacity(ply.elements[0].count);
		for &line in lines.iter().take(ply.elements[0].count) {
			res.push(try!(Element::parse(line)));
		}
		Ok(TestObj {vertices: res})
	}
}

pub trait Element {
	// dummy parameter until UFCS
	fn check(Option<Self>, &parser::ElementSpec) -> Result<(), &'static str>;
	fn parse(&str) -> Result<Self,&'static str>;
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

	fn parse(data: &str) -> Result<Self,&'static str> {
		let n: Vec<f32> = data.split(' ').filter_map(|a| a.parse()).collect();
		if n.len() != 3 {
			Err("Wrong number of elements on a line.")
		} else {
			Ok(Position { x: n[0], y: n[1], z: n[2] })
		}
	}
}

/*
pub fn parse_elem_data<T:Element>(spec: &parser::ElementSpec,
                             mut data: Vec<&str>) -> Result<Vec<T>,&'static str>
{
	try!(Element::check(None::<Position>, spec));

	let mut res: Vec<T> = Vec::with_capacity(spec.count);

	for &line in data.iter() {
		res.push(try!(Element::parse(line)));
	}
	Ok(res)
}


pub fn parse_file(path_str: &str) -> Result<parser::PLY, String> {
	let path = Path::new(path_str);
	let mut fil = File::open(&path);
	let text: Vec<u8> = fil.read_to_end().unwrap();
	let string: &str = std::str::from_utf8(&text[]).unwrap();

	parser::parse(string)
}
*/
