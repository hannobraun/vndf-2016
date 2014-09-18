use cgmath::{
	Vector2,
	Vector3,
};


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Vec2(pub f64, pub f64);


impl Add<Vec2, Vec2> for Vec2 {
	fn add(&self, &Vec2(x2, y2): &Vec2) -> Vec2 {
		let &Vec2(x1, y1) = self;
		Vec2(
			x1 + x2,
			y1 + y2)
	}
}

impl Sub<Vec2, Vec2> for Vec2 {
	fn sub(&self, &Vec2(x2, y2): &Vec2) -> Vec2 {
		let &Vec2(x1, y1) = self;
		Vec2(
			x1 - x2,
			y1 - y2)
	}
}

impl Mul<f64, Vec2> for Vec2 {
	fn mul(&self, s: &f64) -> Vec2 {
		let &Vec2(x, y) = self;
		Vec2(
			x * *s,
			y * *s)
	}
}

impl Neg<Vec2> for Vec2 {
	fn neg(&self) -> Vec2 {
		let &Vec2(x, y) = self;
		Vec2(
			-x,
			-y
		)
	}
}

impl Vec2 {
	pub fn zero() -> Vec2 {
		Vec2(0.0, 0.0)
	}

	pub fn x(&self) -> f64 {
		let &Vec2(x, _) = self;
		x
	}

	pub fn y(&self) -> f64 {
		let &Vec2(_, y) = self;
		y
	}

	pub fn mag(&self) -> f64 {
		let &Vec2(x, y) = self;
		(x*x + y*y).sqrt()
	}

	pub fn normalize(&self) -> Vec2 {
		self * (1.0 / self.mag())
	}

	pub fn round(&self, precision_in_bits: uint) -> Vec2 {
		let &Vec2(x, y) = self;
		let factor = (1u << precision_in_bits) as f64;
		Vec2(
			(x * factor).floor() / factor,
			(y * factor).floor() / factor)
	}

	// Temporary methods to ease the transition away from our own math code.
	pub fn to_vector2_f32(&self) -> Vector2<f32> {
		Vector2::new(self.x() as f32, self.y() as f32)
	}
	pub fn to_vector2_f64(&self) -> Vector2<f64> {
		Vector2::new(self.x(), self.y())
	}
	pub fn to_vector3_f32(&self) -> Vector3<f32> {
		Vector3::new(self.x() as f32, self.y() as f32, 0.0)
	}
}
