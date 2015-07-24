use nalgebra::{
	Dot,
	Norm,
	Vec2,
};


pub fn is_point_on_line(p: Vec2<f64>, l1: Vec2<f64>, l2: Vec2<f64>) -> bool {
	let distance_from_line =
		((l2.y - l1.y) * p.x - (l2.x - l1.x) * p.y + l2.x * l1.y - l1.x * l2.y)
		.abs()
		/ ((l2.y - l1.y).powi(2) + (l2.x - l1.y).powi(2)).sqrt();

	distance_from_line < 0.001
}

pub fn angle_from(v: Vec2<f64>) -> f64 {
	angle_between(Vec2::new(1.0, 0.0), v)
}

pub fn angle_between(v1: Vec2<f64>, v2: Vec2<f64>) -> f64 {
	(v1.dot(&v2) / (v1.norm() * v2.norm())).acos()
}

pub fn roughly_equal(a: f64, b: f64, epsilon: f64) -> bool {
	(a - b).abs() < epsilon
}
