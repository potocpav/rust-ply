#![feature(plugin,core)]

#[plugin]
extern crate ply_plugins;
extern crate ply;


#[derive(Debug)]
#[ply_model]
struct Model1 {
	vertex: Vec<Vertex>,
}

#[derive(Debug)]
#[ply_model]
struct Model2 {
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


#[test]
fn test1() { assert!(ply::parse(TEST_STR1).and_then(|x| {
		let m: Result<Model1,_> = ply::Model::new(&x);
		m
	}).is_ok()); }

#[test]
fn test2() { assert!(ply::parse(TEST_STR2).and_then(|x| {
		let m: Result<Model2,_> = ply::Model::new(&x);
		m
	}).is_ok()); }


static TEST_STR1: &'static str = r#"ply
format ascii 1.0
element vertex 1
property float x
property float y
property float z
end_header
0 0 0
"#;

static TEST_STR2: &'static str = r#"ply
format ascii 1.0
comment made by Greg Turk
comment this file is a cube
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
