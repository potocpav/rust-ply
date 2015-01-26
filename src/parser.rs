

peg_file! ply_rustpeg("ply.rustpeg");

pub fn parse(s: &str) -> Result<PLY, String> {
	ply_rustpeg::parse(s)
}

// Fill the data contained in elements.
// Called by a function defined in the grammar.
fn fill_data(elems: &mut Vec<ElementSpec>, data: Vec<Vec<String>>) {
	let mut counter = 0us;
	for i in 0..elems.len() {
		let count = elems[i].count;
		elems[i].data.push_all(&data[counter .. counter + count]);
		counter += count;
	}
}


#[derive(Debug)]
pub enum Format { Ascii }

#[derive(Debug)]
pub struct Version (u32, u32);

#[derive(Debug)]
pub struct ElementSpec {
	pub name: String,
	pub count: usize,
	pub props: Vec<PropertySpec>,
	pub data: Vec<Vec<String>>, // individual lines of the data
}

#[derive(Debug)]
pub struct PropertySpec {
	pub name: String,
	pub type_: Type,
}

#[derive(Debug,PartialEq)]
pub enum Type {
	Char, UChar, Short, UShort, Int, UInt, Float, Double,
	List (Box<Type>, Box<Type>),
}

#[derive(Debug)]
pub struct PLY {
	pub format: Format,
	pub version: Version,
	pub elements: Vec<ElementSpec>,
}
