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
pub enum Visual {
	MissileVisual,
	ShipVisual
}
