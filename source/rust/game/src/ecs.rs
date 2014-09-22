use cgmath::{
	Quaternion,
	Vector3,
};

use net::ConnId;
use physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Interpolated {
	pub previous_time: u64,
	pub current_time : u64,

	pub previous: Option<Body>,
	pub current : Option<Body>
}

impl Interpolated {
	pub fn new(current_time: u64, body: Option<Body>) -> Interpolated {
		Interpolated {
			previous_time: current_time,
			current_time : current_time,

			previous: body,
			current : body
		}
	}
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Player {
	pub client_id    : ConnId,
	pub missile_index: u64,
	pub last_snapshot: Vec<SharedWorldEntity>
}

#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	ShowAsMissile,
	ShowAsShip
}


ecs!(
	entity(Missile<Body, Visual>): |body: Body| {
		(body, ShowAsMissile)
	}
	entity(Ship<Body, Visual>): |body: Body| {
		(body, ShowAsShip)
	}

	world(SharedWorld<Missile, Ship>)
)

ecs!(
	entity(Missile<Body, Visual>): |position: Vector3<f64>, attitude: Quaternion<f64>| {
		let mut body = Body::default();
		body.position = position;
		body.velocity = Vector3::new(60.0, 0.0, 0.0);
		body.attitude = attitude;

		(body, ShowAsMissile)
	}
	entity(Ship<Body, Player, Visual>): |client_id: ConnId| {
		let mut body = Body::default();
		body.velocity = Vector3::new(30.0, 0.0, 0.0);

		let player = Player {
			client_id    : client_id,
			missile_index: 0,
			last_snapshot: Vec::new(),
		};

		(body, player, ShowAsShip)
	}

	world(World<Missile, Ship>)
)

ecs!(
	entity(Craft<Body, Visual, Interpolated>): |body: Body, visual: Visual, current_time: u64| {
		(
			body,
			visual,
			Interpolated::new(current_time, Some(body))
		)
	}

	world(ClientWorld<Craft>)
)
