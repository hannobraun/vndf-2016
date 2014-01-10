#[cfg(test)]
mod test {
	use vec::{Vec2, vec_add, vec_scale, vec_magnitude, vec_normalize};

	#[test]
	fn it_should_add_two_vectors() {
		let a = Vec2 {x: 1.0, y: 2.0};
		let b = Vec2 {x: 2.0, y: 1.0};

		assert!(vec_add(a, b) == Vec2 {x: 3.0, y: 3.0});
	}

	#[test]
	fn it_should_scale_a_vector() {
		let v = Vec2 {x: 1.0, y: 2.0};
		let s = 2.0;

		assert!(vec_scale(v, s) == Vec2 {x: 2.0, y: 4.0});
	}

	#[test]
	fn it_should_compute_a_vectors_magnitude() {
		let v = Vec2 {x: 3.0, y: 4.0};
		let m = 5.0;

		assert!(vec_magnitude(v) == m);
	}

	#[test]
	fn it_should_normalize_a_vector() {
		let a = Vec2 {x: 3.0, y: 0.0};
		let b = Vec2 {x: 0.0, y: 4.0};

		assert!(vec_normalize(a) == Vec2 {x: 1.0, y: 0.0});
		assert!(vec_normalize(b) == Vec2 {x: 0.0, y: 1.0});
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


#[no_mangle]
pub extern fn vec_add(a: Vec2, b: Vec2) -> Vec2 {
	Vec2 {
		x: a.x + b.x,
		y: a.y + b.y }
}

#[no_mangle]
pub extern fn vec_scale(v: Vec2, s: f64) -> Vec2 {
	Vec2 {
		x: v.x * s,
		y: v.y * s }
}

#[no_mangle]
pub extern fn vec_magnitude(v: Vec2) -> f64 {
	::std::num::sqrt(v.x*v.x + v.y*v.y)
}

#[no_mangle]
pub extern fn vec_normalize(v: Vec2) -> Vec2 {
	let m = vec_magnitude(v);
	vec_scale(v, 1.0/m)
}
