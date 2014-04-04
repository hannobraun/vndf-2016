use std::num;


#[deriving(Show)]
pub struct Vec2 {
	x: f64,
	y: f64
}

impl Eq for Vec2 {
	fn eq(&self, other: &Vec2) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Add<Vec2, Vec2> for Vec2 {
	fn add(&self, other: &Vec2) -> Vec2 {
		Vec2 {
			x: self.x + other.x,
			y: self.y + other.y }
	}
}

impl Sub<Vec2, Vec2> for Vec2 {
	fn sub(&self, other: &Vec2) -> Vec2 {
		Vec2 {
			x: self.x - other.x,
			y: self.y - other.y }
	}
}

impl Mul<f64, Vec2> for Vec2 {
	fn mul(&self, s: &f64) -> Vec2 {
		Vec2 {
			x: self.x * *s,
			y: self.y * *s }
	}
}

impl Vec2 {
	pub fn magnitude(self) -> f64 {
		num::sqrt(self.x*self.x + self.y*self.y)
	}

	pub fn normalize(self) -> Vec2 {
		let m = self.magnitude();
		self * (1.0/m)
	}
}
