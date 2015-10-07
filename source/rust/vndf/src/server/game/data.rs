use nalgebra::Vec2;


#[derive(Clone, Copy, Debug, RustcDecodable, RustcEncodable)]
pub struct Spawner {
	pub position: Vec2<f64>,
	pub velocity: Vec2<f64>,
}

impl Spawner {
	pub fn new() -> Self {
		Spawner {
			position: Vec2::new(0.0, 0.0),
			velocity: Vec2::new(1.0, 0.0),
		}
	}
}
