use std::num;


#[deriving(Show)]
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64
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
			y: self.y + other.y,
			z: self.z + other.z }
	}
}

impl Sub<Vec3, Vec3> for Vec3 {
	fn sub(&self, other: &Vec3) -> Vec3 {
		Vec3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z }
	}
}

impl Mul<f64, Vec3> for Vec3 {
	fn mul(&self, s: &f64) -> Vec3 {
		Vec3 {
			x: self.x * *s,
			y: self.y * *s,
			z: self.z * *s }
	}
}

impl Vec3 {
	pub fn magnitude(self) -> f64 {
		num::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
	}

	pub fn normalize(self) -> Vec3 {
		let m = self.magnitude();
		self * (1.0/m)
	}
}
