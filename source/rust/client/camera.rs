pub struct Camera {
	h: f64,
	v: f64
}

impl Camera {
	pub fn new() -> ~Camera {
		~Camera {
			v: 0.0,
			h: 0.0 }
	}
}
