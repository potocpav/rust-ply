

pub trait Property {
	fn get_type() -> super::Type;
}

macro_rules! property_type { ($t:ty, $i:ident) => (
    impl Property for $t {
		fn get_type() -> super::Type {
			super::Type::$i
		}
	}
)}

property_type!{i32, Int}
property_type!{u32, UInt}
property_type!{f32, Float}
property_type!{f64, Double}
