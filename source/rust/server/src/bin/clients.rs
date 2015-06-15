use nalgebra::Vec2;


#[derive(Debug)]
pub struct Client {
	pub id           : String,
	pub last_active_s: f64,
	pub position     : Vec2<f64>,
	pub velocity     : Vec2<f64>,
}
