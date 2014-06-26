use physics::Vec2;


pub fn is_on_line(line: (Vec2, Vec2), point: Vec2, precision: uint) -> bool {
	let (Vec2(x1, y1), Vec2(x2, y2)) = line;
	let Vec2(px, py) = point;

	let factor = (1 << precision) as f64;
	let x      = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);

	(x * factor).floor() / factor == 0.0
}
