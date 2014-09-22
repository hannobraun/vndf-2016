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
	entity(Craft<Body, Visual, Interpolated>): |body: Body, visual: Visual, current_time: u64| {
		(
			body,
			visual,
			Interpolated::new(current_time, Some(body))
		)
	}

	world(ClientWorld<Craft>)
)
