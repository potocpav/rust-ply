#![feature(plugin)]

#[plugin]
extern crate ply_plugins;
extern crate ply;


#[derive(Debug)]
#[ply_data]
struct S {
	vertex: Vec<Position>,
	face: Vec<Color>,
}

#[derive(Debug, Copy)]
#[ply_element]
pub struct Color (f32, f32, f32);

#[derive(Debug, Copy)]
pub struct Position {
	x: f32, y: f32, z: f32
}

impl ply::Element for Position {
	fn check(_dummy: Option<Self>, spec: &ply::ElementSpec) -> Result<(), &'static str> {
		if spec.props.len() != 3 {
			println!("num: {}", spec.props.len());
			return Err("Wrong number of params.")
		}
		for prop in spec.props.iter() {
			if prop.type_ != ply::Type::Float && prop.type_ != ply::Type::Double {
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


fn main() {
	match ply::parser::parse(PLY1) {
		Ok(ply) => {
			// Print the parsed PLY file
			println!("Format: {:?}, {:?}", ply.format, ply.version);
			for e in ply.elements.iter() {
				println!("Element \"{}\": {} instances.", e.name, e.data.len());
				for p in e.props.iter() {
					println!("    Property \"{}\": \t{:?}.", p.name, p.type_);
				}
				println!("  Data: {:?}", e.data);
			}

			// Fill a data structure
			let model: Result<S, &'static str> = ply::PlyModel::new(&ply);
			println!("\nResult: {:?}", model);

		},
		Err(e) => println!("Error: {}", e),
	}
}


static PLY1: &'static str = r#"ply
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



// http://paulbourke.net/dataformats/ply/
/*
static PLY2: &'static str = r#"ply
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
"#;
*/
