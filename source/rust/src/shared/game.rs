use nalgebra::Vec2;


pub type EntityId = u64;


#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Body {
	pub position: Vec2<f64>,
	pub velocity: Vec2<f64>,
}

#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Broadcast {
	pub sender : EntityId,
	pub message: String,
}
