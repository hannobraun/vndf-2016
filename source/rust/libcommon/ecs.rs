use net::ConnId;


pub struct Player {
	pub client_id    : ConnId,
	pub missile_index: u64
}

#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	MissileVisual,
	ShipVisual
}
