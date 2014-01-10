#[cfg(test)]
mod test {
	use vec::{Vec2, vec_add};

	#[test]
	fn it_should_add_two_vectors() {
		let a = Vec2 {x: 1.0, y: 2.0};
		let b = Vec2 {x: 2.0, y: 1.0};

		assert!(vec_add(a, b) == Vec2 {x: 3.0, y: 3.0});
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
