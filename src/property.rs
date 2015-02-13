
// Convert a type into a Type

use super::Type;

pub trait Property: Sized {
	fn get_type(Option<Self>) -> Type;
	fn parse_prop<'a,T>(&mut T) -> Option<Self> where
				T: Iterator<Item=&'a String>;
}


/// Parse the lists
impl<P: Property> Property for Vec<P> {
	fn get_type(_dummy: Option<Self>) -> Type {
		Type::List(Box::new(Property::get_type(None::<P>)))
	}

	fn parse_prop<'a,T>(it: &mut T) -> Option<Self> where
			T: Iterator<Item=&'a String> {
		if let Some(count) = it.next().and_then(|s| s.parse().ok()) {
			let mut ret = Vec::with_capacity(count);
			for _ in 0..count {
				if let Some(p) = Property::parse_prop(it) {
					ret.push(p);
				} else {
					return None;
				}
			}
			Some(ret)
		} else { None }
	}
}

macro_rules! property_type { ($t:ty, $i:ident) => (
    impl Property for $t {
		fn get_type(_dummy: Option<Self>) -> Type {
			Type::$i
		}
		fn parse_prop<'a,T>(it: &mut T) -> Option<Self> where
				T: Iterator<Item=&'a String> {
			it.next().and_then(|v| v.parse().ok())
		}
	}
)}

property_type!{i8, Char}
property_type!{u8, UChar}
property_type!{i32, Int}
property_type!{u32, UInt}
property_type!{f32, Float}
property_type!{f64, Double}
