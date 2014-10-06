#![feature(phase)]

#[phase(plugin)]
extern crate peg_syntax_ext;

//use std::vec::Vector;

pub mod parser;

pub trait Element {
	fn parse(&parser::ElementSpec, &mut str) -> Option<Self>;
}

pub struct Position {
	x: f32, y: f32, z: f32
}

impl Element for Position {
	fn parse(h: &parser::ElementSpec, data: &str) -> Option<Position> {
		let n: Vec<f32> = data.split(' ').filter_map(from_str).collect();
		if n.len() != 3 { 
			None
		} else {
			Some(Position { x: n[0], y: n[1], z: n[2] })
		}
	}
}

pub fn parse<T:Element>(h: &parser::ElementSpec, 
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
}
