use std::from_str;


#[deriving(Eq)]
pub struct Update {
	id: uint,
	x : f64,
	y : f64,
	z : f64
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
			id: id.unwrap(),
			x : x.unwrap(),
			y : y.unwrap(),
			z : z.unwrap()
		})
	}
}
