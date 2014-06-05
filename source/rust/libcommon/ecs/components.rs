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
pub struct ShipKind {
	dummy: bool // Required to derive Encodable
}

impl ShipKind {
	pub fn new() -> ShipKind {
		ShipKind {
			dummy: false
		}
	}
}
