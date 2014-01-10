#[cfg(test)]
mod test {
	use vec;
	use vec::{Vec2};

	#[test]
	fn it_should_add_two_vectors() {
		let a = Vec2 { x: 1.0, y: 2.0 };
		let b = Vec2 { x: 2.0, y: 1.0 };

		let c = Vec2 { x: 3.0, y: 3.0 };

		assert!(a + b == c);
	}

	#[test]
	fn it_should_scale_a_vector() {
		let v = Vec2 { x: 1.0, y: 2.0 };
		let s = 2.0;

		assert!(v * s == Vec2 { x: 2.0, y: 4.0 });
	}

	#[test]
	fn it_should_compute_a_vectors_magnitude() {
		let v = Vec2 { x: 3.0, y: 4.0 };

		assert!(v.magnitude() == 5.0);
	}

	#[test]
	fn it_should_normalize_a_vector() {
		let a = Vec2 { x: 3.0, y: 0.0 };
		let b = Vec2 { x: 0.0, y: 4.0 };

		assert!(vec::normalize(a) == Vec2 { x: 1.0, y: 0.0 });
		assert!(vec::normalize(b) == Vec2 { x: 0.0, y: 1.0 });
	}
}


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

impl Mul<f64, Vec2> for Vec2 {
	fn mul(&self, s: &f64) -> Vec2 {
		Vec2 {
			x: self.x * *s,
			y: self.y * *s }
	}
}

impl Vec2 {
	pub fn magnitude(self) -> f64 {
		::std::num::sqrt(self.x*self.x + self.y*self.y)
	}
}

pub fn normalize(v: Vec2) -> Vec2 {
	let m = v.magnitude();
	v * (1.0/m)
}
