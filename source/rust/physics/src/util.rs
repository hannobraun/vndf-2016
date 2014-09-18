use cgmath::Vector2;


pub fn is_on_line(line: (Vector2<f64>, Vector2<f64>), point: Vector2<f64>, precision: uint) -> bool {
	let (l1, l2) = line;

	let x1 = l1[0];
	let y1 = l1[1];
	let x2 = l2[0];
	let y2 = l2[1];
	let px = point[0];
	let py = point[1];

	let factor = (1u << precision) as f64;
	let x      = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);

	(x * factor).floor() / factor == 0.0
}
