use net::ConnId;
use physics::{
	Body,
	Radians,
	Vec2,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Player {
	pub client_id    : ConnId,
	pub missile_index: u64
}

#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	ShowAsMissile,
	ShowAsShip
}


ecs!(
	// Shared
	component(Body, bodies): Body
	component(Visual, visuals): Visual

	entity(SharedMissile<Body, Visual>): |body: Body| {
		(body, ShowAsMissile)
	}
	entity(SharedShip<Body, Visual>): |body: Body| {
		(body, ShowAsShip)
	}

	world(SharedWorld<SharedMissile, SharedShip>)


	// Server-only
	component(Player, players): Player

	entity(Missile<Body, Visual>): |position: Vec2, attitude: Radians| {
		let body = Body {
			position: position,
			velocity: Vec2::zero(),
			attitude: attitude
		};

		(body, ShowAsMissile)
	}
	entity(Ship<Body, Player, Visual>): |client_id: ConnId| {
		let body = Body {
			position: Vec2::zero(),
			velocity: Vec2::zero(),
			attitude: Radians(0.0)
		};

		let player = Player {
			client_id    : client_id,
			missile_index: 0
		};

		(body, player, ShowAsShip)
	}

	world(World<Missile, Ship>)
)
