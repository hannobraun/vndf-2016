use std::from_str;

use vec::Vec3;


pub enum Message {
	Update(Update),
	Remove(Remove),
	Invalid
}

impl Message {
	pub fn from_str(s: &str) -> Message {
		let words: ~[&str] = s.words().collect();

		match words[0] {
			"UPDATE" => {
				let id: Option<uint> = from_str::from_str(words[1]);
				let x : Option<f64>  = from_str::from_str(words[2]);
				let y : Option<f64>  = from_str::from_str(words[3]);
				let z : Option<f64>  = from_str::from_str(words[4]);

				Update(Update {
					id : id.unwrap(),
					pos: Vec3 {
						x : x.unwrap(),
						y : y.unwrap(),
						z : z.unwrap()
					}
				})
			},

			"REMOVE" => {
				let id: Option<uint> = from_str::from_str(words[1]);

				Remove(Remove {
					id: id.unwrap()
				})
			},

			_ => Invalid
		}
	}
}


#[deriving(Eq)]
pub struct Update {
	id : uint,
	pos: Vec3
}

impl Update {
	pub fn to_str(&self) -> ~str {
		format!(
			"UPDATE {} {} {} {}",
			self.id,
			self.pos.x,
			self.pos.y,
			self.pos.z)
	}
}


pub struct Remove {
	id: uint
}

impl Remove {
	pub fn to_str(&self) -> ~str {
		format!(
			"REMOVE {}",
			self.id)
	}
}
