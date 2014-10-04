use cgmath::Vector3;

use physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Planet {
	pub position: Vector3<f64>,
	pub radius  : f64,
	pub color   : Vector3<f32>,
}

impl Planet {
	pub fn new() -> Planet {
		Planet {
			position: Vector3::zero(),
			radius  : 1.0,
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
	Planet(Planet): (position: Vector3<f64>, radius: f64, color: Vector3<f32>) {
		(
			Planet {
				position: position,
				radius  : radius,
				color   : color,
			},
		)
	}
)
