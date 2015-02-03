#![feature(plugin,core)]

#[plugin]
extern crate ply_plugins;
extern crate ply;


#[derive(Debug)]
#[ply_data]
struct Model {
	vertex: Vec<Vertex>,
	face: Vec<Face>,
	edge: Vec<Edge>,
}


#[derive(Debug, Copy)]
#[ply_element]
pub struct Vertex {
	x: f32, y: f32, z: f32,
	red: u8, green: u8, blue: u8,
}


#[derive(Debug)]
#[ply_element]
pub struct Face (Vec<i32>);


#[derive(Debug, Copy)]
#[ply_element]
pub struct Edge {
	vertex1: i32, vertex2: i32,
	red: u8, green: u8, blue: u8,
}


fn main() {
	match ply::parser::parse(PLY) {
		Ok(ref ply) => {

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
			let model: Result<Model, String> = ply::PlyModel::new(ply);
			println!("\nResult: {:?}", model);

		},
		Err(e) => println!("Error:\n{}", e),
	}
}


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
"#;
