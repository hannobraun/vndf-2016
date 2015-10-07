use nalgebra::Vec2;


#[derive(Debug)]
pub struct Spawner {
	pub position: Vec2<f64>,
}

impl Spawner {
	pub fn new() -> Self {
		Spawner {
			position: Vec2::new(0.0, 0.0),
		}
	}
}
