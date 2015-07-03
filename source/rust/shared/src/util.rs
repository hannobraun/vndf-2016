use nalgebra::Vec2;


pub fn is_point_on_line(p: Vec2<f64>, l1: Vec2<f64>, l2: Vec2<f64>) -> bool {
	let distance_from_line =
		((l2.y - l1.y) * p.x - (l2.x - l1.x) * p.y + l2.x * l1.y - l1.x * l2.y)
		.abs()
		/ ((l2.y - l1.y).powi(2) + (l2.x - l1.y).powi(2)).sqrt();

	distance_from_line < 0.001
}
