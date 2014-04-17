#[deriving(Decodable, Encodable, Eq, Show)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64
}

impl Add<Vec2, Vec2> for Vec2 {
	fn add(&self, other: &Vec2) -> Vec2 {
		Vec2 {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Sub<Vec2, Vec2> for Vec2 {
	fn sub(&self, other: &Vec2) -> Vec2 {
		Vec2 {
			x: self.x - other.x,
			y: self.y - other.y
		}
	}
}

impl Mul<f64, Vec2> for Vec2 {
	fn mul(&self, s: &f64) -> Vec2 {
		Vec2 {
			x: self.x * *s,
			y: self.y * *s
		}
	}
}

impl Vec2 {
	pub fn zero() -> Vec2 {
		Vec2 {
			x: 0.0,
			y: 0.0
		}
	}

	pub fn magnitude(self) -> f64 {
		(self.x*self.x + self.y*self.y).sqrt()
	}

	pub fn normalize(self) -> Vec2 {
		let m = self.magnitude();
		self * (1.0/m)
	}

	pub fn round(self, precision_in_bits: u8) -> Vec2 {
		let factor = (1 << precision_in_bits) as f64;
		Vec2 {
			x: (self.x * factor).floor() / factor,
			y: (self.y * factor).floor() / factor
		}
	}
}
