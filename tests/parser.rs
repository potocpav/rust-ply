
extern crate ply;

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
comment A basic test
element vertex 2
property int x
property float y
element face 1
property list uchar int vertex
end_header
1  2.345
0 -6.789
3 1 2 3
"#;

static TEST_STR3: &'static str = r#"ply
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


#[test]
fn test1() { assert!(ply::parse(TEST_STR1).is_ok()); }

#[test]
fn test2() { assert!(ply::parse(TEST_STR2).is_ok()); }

#[test]
fn test3() { assert!(ply::parse(TEST_STR3).is_ok()); }
