

#[derive(Debug)]
pub enum Format { Ascii }


#[derive(Debug)]
pub struct Version (u32, u32);

#[derive(Debug)]
pub struct ElementSpec {
	pub name: String,
	pub count: u32,
	pub props: Vec<PropertySpec>
}

#[derive(Debug)]
pub struct PropertySpec {
	pub name: String,
	pub type_: Type
}

#[derive(Debug)]
pub enum Type {
	Char, UChar, Short, UShort, Int, UInt, Float, Double,
	List (Box<Type>, Box<Type>)
}

#[derive(Debug)]
pub struct PLY {
	pub format: Format,
	pub version: Version,
	pub elements: Vec<ElementSpec>,
	pub data: String
}

peg! ply{r#"
#[pub]
parse -> super::PLY =
	first_line newline
	fl:format_line newline
	obj_info_line?
	elements:dataspec_section newline
	end_header_line newline
	data:data
		{{
			let (f,v) = fl;
			super::PLY { format: f, version: v, elements: elements, data: data }
		}}

first_line -> ()
	= "ply"

format_line -> (super::Format, super::Version)
	= "format" white f:format white v:version { (f,v) }

obj_info_line -> &'input str
	= "obj_info" white info:raw_string newline { info }

dataspec_section -> Vec<super::ElementSpec>
	= element_section**newline

element_section -> super::ElementSpec
	= e:element_line newline ps:property_line**newline
		{ super::ElementSpec { name: e.name, count: e.count, props: ps } }

element_line -> super::ElementSpec
	= "element" white n:identifier white i:uint
		{ super::ElementSpec { name: n, count: i, props: vec![] } }

property_line -> super::PropertySpec
	= "property" white t:type white n:identifier
		{ super::PropertySpec { name: n, type_: t } }

end_header_line -> ()
	= "end_header"

data -> String = d:.* { match_str.to_string() }


format -> super::Format
	= "ascii" { super::Format::Ascii }

version -> super::Version
	= maj:uint "." min:uint { super::Version(maj,min) }

type -> super::Type
	= i:integraltype { i }
	/ "float" { super::Type::Float }
	/ "double" { super::Type::Double }
	/ "list" white t:type white q:integraltype { super::Type::List (box t, box q) }

integraltype -> super::Type
	= "char" {   super::Type::Char }
	/ "uchar" {  super::Type::UChar }
	/ "short" {  super::Type::Short }
	/ "ushort" { super::Type::UShort }
	/ "int" {    super::Type::Int }
	/ "uint" {   super::Type::UInt }

identifier -> String =
	[a-zA-Z_] [a-zA-Z0-9_]* { match_str.to_string() }

raw_num -> &'input str
	= "-"? [0-9]+ "." [0-9]+ { match_str }
	/ "-"? [0-9]+ { match_str }

uint -> u32
	= [0-9]+ { match_str.parse().unwrap() }

raw_string -> &'input str
	= [^\n]* { match_str }

skipped_line -> () = comment_line / endline

comment_line -> () = "comment" white raw_string "\n"

newline -> ()
	= endline skipped_line*

endline -> () = white? "\n"

white -> () = [\t ]+
"#}


// re-export. Maybe, there is a better way to do this.
pub fn parse(s: &str) -> Result<PLY, String> { ply::parse(s) }
