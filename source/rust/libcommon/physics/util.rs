use physics::Vec2;


pub fn is_on_line(Vec2(x1, y1): Vec2, Vec2(x2, y2): Vec2, Vec2(px, py): Vec2) -> bool {
	((x2 - x1) * (py - y1) - (y2 - y1) * (px - x1)) == 0.0
}
