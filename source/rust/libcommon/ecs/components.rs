#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct MissileKind {
	dummy: bool // Required to derive Encodable
}

impl MissileKind {
	pub fn new() -> MissileKind {
		MissileKind {
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
