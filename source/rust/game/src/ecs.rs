use cgmath::Vector3;

use physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Planet {
	pub position: Vector3<f64>,
	pub radius  : f64,
	pub color   : Vector3<f32>,
}

#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	ShowAsMissile,
	ShowAsShip
}


world!(
	Missile(Body, Visual): (body: Body) {
		(body, ShowAsMissile)
	}
	Ship(Body, Visual): (body: Body) {
		(body, ShowAsShip)
	}
)
