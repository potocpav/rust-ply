
extern crate ply;

// http://paulbourke.net/dataformats/ply/
static TEST_STR1: &'static str = r#"ply
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

// http://w3.impa.br/~diego/software/rply/
static TEST_STR2: &'static str = r#"ply
format ascii 1.0
comment this is a simple file
obj_info any data, in one line of free form text
element vertex 3
property float x
property float y
property float z
element face 1
property list uchar int vertex_indices
end_header
-1 0 0
 0 1 0
 1 0 0
3 0 1 2
"#;

#[test]
fn test1() { assert!(ply::parser::parse(TEST_STR1).is_ok()); }
#[test]
fn test2() { assert!(ply::parser::parse(TEST_STR2).is_ok()); }
