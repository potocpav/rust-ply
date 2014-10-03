

#[deriving(Show)]
enum Format { Ascii }

#[deriving(Show)]
struct Version (uint, uint);

#[deriving(Show)]
struct Element<'a> {
	name: &'a str,
	count: uint,
	props: Vec<Property<'a>>
}

#[deriving(Show)]
struct Property<'a> {
	name: &'a str,
	type_: Type
}

#[deriving(Show)]
enum Type {
	Char, UChar, Short, UShort, Int, UInt, Float, Double,
	List (Box<Type>, Box<Type>)
}

#[deriving(Show)]
pub struct PLY<'a> {
	format: Format,
	version: Version,
	elems: Vec<Element<'a>>,
	data: Vec<Vec<&'a str>>
}

peg! ply(r#"
#[pub]
parse -> super::PLY = 
	first_line newline 
	fl:format_line newline
	obj_info_line?
	elems:dataspec_section newline
	end_header_line newline
	data:data_line**newline newline?
		{{ 
			let (f,v) = fl; 
			super::PLY { format: f, version: v, elems: elems, data: data } 
		}}
	
first_line -> ()
	= "ply"

format_line -> (super::Format, super::Version)
	= "format" white f:format white v:version { (f,v) }
	
obj_info_line -> &str
	= "obj_info" white info:raw_string newline { info }
	
dataspec_section -> Vec<super::Element>
	= element_section**newline

element_section -> super::Element
	= e:element_line newline ps:property_line**newline
		{ super::Element { name: e.name, count: e.count, props: ps } }
	
element_line -> super::Element
	= "element" white n:identifier white i:uint
		{ super::Element { name: n, count: i, props: vec![] } }
	
property_line -> super::Property
	= "property" white t:type white n:identifier
		{ super::Property { name: n, type_: t } }

end_header_line -> () 
	= "end_header"
	
data_line -> Vec<&str>
	= white? rn:raw_num++white { rn }


format -> super::Format
	= "ascii" { super::Ascii }
	
version -> super::Version
	= maj:uint "." min:uint { super::Version(maj,min) }

type -> super::Type 
	= i:integraltype { i }
	/ "float" { super::Float }
	/ "double" { super::Double }
	/ "list" white t:type white q:integraltype { super::List (box t, box q) }
	
integraltype -> super::Type
	= "char" { super::Char }
	/ "uchar" { super::UChar }
	/ "short" { super::Short }
	/ "ushort" { super::UShort }
	/ "int" { super::Int }
	/ "uint" { super::UInt }

identifier -> &str =
	[a-zA-Z_] [a-zA-Z0-9_]* { match_str }

raw_num -> &str
	= "-"? [0-9]+ "." [0-9]+ { match_str }
	/ "-"? [0-9]+ { match_str }

uint -> uint
	= [0-9]+ { from_str::<uint>(match_str).unwrap() }

raw_string -> &str
	= [^\n]* { match_str }

skipped_line -> () = comment_line / endline

comment_line -> () = "comment" white raw_string "\n"

newline -> ()
	= endline skipped_line*
	
endline -> () = white? "\n"
	
white -> () = [\t ]+
"#)


// re-export. Maybe, there is a better way to do this.
pub fn parse(s: &str) -> Result<PLY, String> { ply::parse(s) }
