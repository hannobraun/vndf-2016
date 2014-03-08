pub struct Vec3 {
	x: f64,
	y: f64
}

impl Eq for Vec3 {
	fn eq(&self, other: &Vec3) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Add<Vec3, Vec3> for Vec3 {
	fn add(&self, other: &Vec3) -> Vec3 {
		Vec3 {
			x: self.x + other.x,
			y: self.y + other.y }
	}
}

impl Mul<f64, Vec3> for Vec3 {
	fn mul(&self, s: &f64) -> Vec3 {
		Vec3 {
			x: self.x * *s,
			y: self.y * *s }
	}
}

impl Vec3 {
	pub fn magnitude(self) -> f64 {
		::std::num::sqrt(self.x*self.x + self.y*self.y)
	}

	pub fn normalize(self) -> Vec3 {
		let m = self.magnitude();
		self * (1.0/m)
	}
}
