#[deriving(Show)]
pub struct Camera {
	x: f64,
	y: f64
}

impl Camera {
	pub fn new() -> ~Camera {
		~Camera {
			x: 0.0,
			y: 0.0 }
	}
}
