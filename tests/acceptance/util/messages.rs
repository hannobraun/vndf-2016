use std::from_str;

use common::vec::Vec3;


#[deriving(Eq)]
pub struct Update {
	id : uint,
	pos: Vec3
}

impl Update {
	pub fn from_str(s: &str) -> Option<Update> {
		let words: ~[&str] = s.words().collect();

		assert!(words[0] == "UPDATE");

		let id: Option<uint> = from_str::from_str(words[1]);
		let x : Option<f64>  = from_str::from_str(words[2]);
		let y : Option<f64>  = from_str::from_str(words[3]);
		let z : Option<f64>  = from_str::from_str(words[4]);

		Some(Update {
			id : id.unwrap(),
			pos: Vec3 {
				x : x.unwrap(),
				y : y.unwrap(),
				z : z.unwrap()
			}
		})
	}
}
