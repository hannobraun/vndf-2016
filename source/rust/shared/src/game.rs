use nalgebra::Vec2;


#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Broadcast {
	pub sender : String,
	pub message: String,
}


#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Ship {
	pub position: Vec2<f64>,
	pub velocity: Vec2<f64>,
}
