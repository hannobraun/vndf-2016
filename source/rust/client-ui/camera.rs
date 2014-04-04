#[deriving(Show)]
pub struct Camera {
	x: f64,
	y: f64,
	h: f64,
	v: f64
}

impl Camera {
	pub fn new() -> ~Camera {
		~Camera {
			x: 0.0,
			y: 0.0,
			v: 0.0,
			h: 0.0 }
	}
}
