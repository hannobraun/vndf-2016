#[cfg(test)]
mod test {
	use vec;
	use vec::{Vec2};

	#[test]
	fn it_should_add_two_vectors() {
		let a = Vec2 { x: 1.0, y: 2.0 };
		let b = Vec2 { x: 2.0, y: 1.0 };

		let c = Vec2 { x: 3.0, y: 3.0 };

		assert!(vec::add(a, b) == c);
	}

	#[test]
	fn it_should_scale_a_vector() {
		let v = Vec2 { x: 1.0, y: 2.0 };
		let s = 2.0;

		assert!(vec::scale(v, s) == Vec2 { x: 2.0, y: 4.0 });
	}

	#[test]
	fn it_should_compute_a_vectors_magnitude() {
		let v = Vec2 { x: 3.0, y: 4.0 };
		let m = 5.0;

		assert!(vec::magnitude(v) == m);
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


pub extern fn add(a: Vec2, b: Vec2) -> Vec2 {
	Vec2 {
		x: a.x + b.x,
		y: a.y + b.y }
}

pub extern fn scale(v: Vec2, s: f64) -> Vec2 {
	Vec2 {
		x: v.x * s,
		y: v.y * s }
}

pub extern fn magnitude(v: Vec2) -> f64 {
	::std::num::sqrt(v.x*v.x + v.y*v.y)
}

pub extern fn normalize(v: Vec2) -> Vec2 {
	let m = magnitude(v);
	scale(v, 1.0/m)
}
