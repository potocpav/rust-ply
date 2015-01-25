#![feature(plugin)]

#[plugin]
extern crate ply_plugins;
extern crate ply;

use ply::{PlyModel,TestObj};
/*
// http://paulbourke.net/dataformats/ply/
static PLY: &'static str = r#"ply
format ascii 1.0
comment author: Greg Turk
comment object: another cube
element vertex 8
property float x
property float y
property float z
property uchar red
property uchar green
property uchar blue
element face 7
property list uchar int vertex_index
element edge 5
property int vertex1
property int vertex2
property uchar red
property uchar green
property uchar blue
end_header
0 0 0 255 0 0
0 0 1 255 0 0
0 1 1 255 0 0
0 1 0 255 0 0
1 0 0 0 0 255
1 0 1 0 0 255
1 1 1 0 0 255
1 1 0 0 0 255
3 0 1 2
3 0 2 3
4 7 6 5 4
4 0 4 5 1
4 1 5 6 2
4 2 6 7 3
4 3 7 4 0
0 1 255 255 255
1 2 255 255 255
2 3 255 255 255
3 0 255 255 255
2 0 0 0 0
"#; */

static PLY: &'static str = r#"ply
format ascii 1.0
comment author: Greg Turk
comment object: another cube
element vertex 8
property float x
property float y
property float z
element face 7
property float x
property float y
property float z
end_header
0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
"#;

#[derive(Debug)]
#[ply_data]
struct S {
	vertex: Vec<ply::Position>,
	face: Vec<ply::Position>,
	prd: bool,
}

fn main() {
	let res = ply::parser::parse(PLY);
	match res {
		Ok(ply) => {
			println!("Format: {:?}, {:?}", ply.format, ply.version);
			for e in ply.elements.iter() {
				println!("Element \"{}\": {} instances.", e.name, e.count);
				for p in e.props.iter() {
					println!("    Property \"{}\": \t{:?}.", p.name, p.type_);
				}
				println!("  Data: {:?}", e.data);
			}
		//	println!("\nData:\n");
		//	println!("{}", ply.data);

			//let obj: TestObj;
			//println!("Result of object check: {:?}", Object::check(None::<TestObj>, &ply));

		//	let res: Result<TestObj,&'static str> = PlyModel::new(&ply);
			let res2: Result<S, &'static str> = PlyModel::new(&ply);
			println!("Result: {:?}", res2);
		//	match res {
		//		Ok(data) => println!("res: {:?}", data),
		//		Err(e) => println!("Error while parsing: {}", e),
		//	}

		}, Err(e) => println!("E: {}", e),
	}
	println!("Hello!");
}
