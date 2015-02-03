
peg_file! ply_rustpeg("ply.rustpeg");

pub fn parse(s: &str) -> Result<PLY, String> {
	let (f, v, mut counted_elems, data) = try!(ply_rustpeg::parse(s));

	let mut counter = 0us;
	for &mut (count, ref mut elem) in counted_elems.iter_mut() {
		//let (count, ref mut elem) = counted_elems.get_mut(i).unwrap();
		if data.len() < count + counter {
			return Err(format!("Data section too short."));
		}
		elem.data.push_all(&data[counter .. counter + count]);
		counter += count;
	}

	Ok(PLY {format: f, version: v, elements: counted_elems.into_iter().map(|(_,e)|e).collect()})
}

#[derive(Debug, Copy)]
pub enum Format { Ascii }

#[derive(Debug, Copy)]
pub struct Version (u32, u32);

#[derive(Debug)]
pub struct ElementSpec {
	pub name: String,
	pub props: Vec<PropertySpec>,
	pub data: Vec<Vec<String>>, // individual lines of the data
}

impl ElementSpec {
	pub fn get_prop(&self, name: String) -> Option<&PropertySpec> {
		self.props.iter().filter(|&e| e.name == name).next()
	}
}

#[derive(Debug)]
pub struct PropertySpec {
	pub name: String,
	pub type_: Type,
}

#[derive(Debug,PartialEq)]
pub enum Type {
	Char, UChar, Short, UShort, Int, UInt, Float, Double,
	List (Box<Type>),
}

#[derive(Debug)]
pub struct PLY {
	pub format: Format,
	pub version: Version,
	pub elements: Vec<ElementSpec>,
}

impl PLY {
	pub fn get_elem(&self, name: String) -> Option<&ElementSpec> {
		self.elements.iter().filter(|&e| e.name == name).next()
	}
}
