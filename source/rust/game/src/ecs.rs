use cgmath::{
	Vector3,
	zero,
};

use physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Planet {
	pub position: Vector3<f64>,
	pub radius  : f64,
	pub mass    : f64,
	pub color   : Vector3<f32>,
}

impl Planet {
	pub fn new() -> Planet {
		Planet {
			position: zero(),
			radius  : 1.0,
			mass    : 1.0,
			color   : Vector3::new(1.0, 1.0, 1.0),
		}
	}

	pub fn at_position(mut self, x: f64, y: f64, z: f64) -> Planet {
		self.position = Vector3::new(x, y, z);
		self
	}

	pub fn with_radius(mut self, radius: f64) -> Planet {
		self.radius = radius;
		self
	}

	pub fn with_mass(mut self, mass: f64) -> Planet {
		self.mass = mass;
		self
	}
}

#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	ShowAsMissile,
	ShowAsShip
}


world! {
	components Body, Visual, Planet;

	derived_traits Clone, Decodable, Encodable, PartialEq, Show;
}
