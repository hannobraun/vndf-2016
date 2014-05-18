use physics::Vec2;


pub fn is_on_line(line: (Vec2, Vec2), point: Vec2) -> bool {
	let (Vec2(x1, y1), Vec2(x2, y2)) = line;
	let Vec2(px, py) = point;

	((x2 - x1) * (py - y1) - (y2 - y1) * (px - x1)) == 0.0
}
