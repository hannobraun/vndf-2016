#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Missile {
	dummy: bool // Required to derive Encodable
}

impl Missile {
	pub fn new() -> Missile {
		Missile {
			dummy: false
		}
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Ship {
	dummy: bool // Required to derive Encodable
}

impl Ship {
	pub fn new() -> Ship {
		Ship {
			dummy: false
		}
	}
}
