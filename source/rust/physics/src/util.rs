use cgmath::Vector2;

use super::Vec2;


pub fn is_on_line(line: (Vec2, Vec2), point: Vector2<f64>, precision: uint) -> bool {
	let (Vec2(x1, y1), Vec2(x2, y2)) = line;

	let px = point[0];
	let py = point[1];

	let factor = (1u << precision) as f64;
	let x      = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);

	(x * factor).floor() / factor == 0.0
}
