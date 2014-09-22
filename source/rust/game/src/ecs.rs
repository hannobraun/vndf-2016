use cgmath::{
	Quaternion,
	Vector3,
};

use net::ConnId;
use physics::Body;


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
