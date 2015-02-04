#![feature(plugin,core)]

#[plugin]
extern crate ply_plugins;
extern crate ply;


#[derive(Debug)]
#[ply_model]
struct Model {
	vertex: Vec<Vertex>,
	face: Vec<Face>,
}

#[derive(Debug,Copy)]
#[ply_element]
pub struct Vertex {
	x: f32, y: f32, z: f32,
}


#[derive(Debug)]
#[ply_element]
pub struct Face {
	vertex_index: Vec<i32>,
}


fn main() {
	match ply::parse(PLY) { // Create an AST
		Ok(ref ply) => {
			// Fill in the structure from the AST
			let model: Result<Model,_> = ply::Model::new(ply);
			// Print the result
			println!("\nResult: {:?}", model);
		},
		Err(e) => println!("Error while parsing:\n\t{}", e),
	}
}


static PLY: &'static str = r#"ply
format ascii 1.0
element vertex 8
property float x
property float y
property float z
element face 6
property list uchar int vertex_index
end_header
0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
4 0 1 2 3
4 7 6 5 4
4 0 4 5 1
4 1 5 6 2
4 2 6 7 3
4 3 7 4 0
"#;
