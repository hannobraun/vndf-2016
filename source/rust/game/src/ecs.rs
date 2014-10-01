use cgmath::Vector3;

use physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Planet {
	position: Vector3<f64>,
}

#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	ShowAsMissile,
	ShowAsShip
}


world!(
	Missile(Body, Visual): |body: Body| {
		(body, ShowAsMissile)
	}
	Ship(Body, Visual): |body: Body| {
		(body, ShowAsShip)
	}
)
