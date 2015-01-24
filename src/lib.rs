#![feature(plugin)]

#[plugin]
extern crate peg_syntax_ext;

//use std::vec::Vector;
use std::io::File;


pub mod parser;

pub trait Element {
	fn parse(&parser::ElementSpec, &str) -> Option<Self>;
}

pub struct Position {
	x: f32, y: f32, z: f32
}

impl Element for Position {
	fn parse(h: &parser::ElementSpec, data: &str) -> Option<Position> {
		let n: Vec<f32> = data.split(' ').filter_map(|a| a.parse()).collect();
		if n.len() != 3 {
			None
		} else {
			Some(Position { x: n[0], y: n[1], z: n[2] })
		}
	}
}
/*
pub fn parse_data<T:Element>(h: &parser::ElementSpec,
                    foo: |&parser::ElementSpec, &str| -> Option<T>,
                    mut data: &str) -> Option<Vec<T>>
{
	let mut res: Vec<T> = Vec::with_capacity(h.count);

	let mut line_iter = data.splitn(h.count, '\n');
	for line in line_iter.take(h.count) {
		res.push(foo(h,line).unwrap())
	}
	data = line_iter.next().unwrap();
	println!("the rest: {}", data);
	Some(res)
}*/

pub fn parse_file(path_str: &str) -> Result<parser::PLY, String> {
	let path = Path::new(path_str);
	let mut fil = File::open(&path);
	let text: Vec<u8> = fil.read_to_end().unwrap();
	// let slice: &[u8] = text.as_slice();
	let string: &str = std::str::from_utf8(&text[]).unwrap();

	parser::parse(string)
}
