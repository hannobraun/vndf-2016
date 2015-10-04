use nalgebra::Vec2;

use shared::color::Color;


pub type EntityId = u64;


#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Body {
    pub position: Vec2<f64>,
    pub velocity: Vec2<f64>,
    pub mass    : f32,
    
}

#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Broadcast {
    pub sender : EntityId,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Maneuver {
    pub ship_id: EntityId,
    pub data   : ManeuverData,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct ManeuverData {
    pub start_s   : f64,
    pub duration_s: f64,
    pub angle     : f64,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Planet {
	pub color: Color,
	pub size : f64,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Ship;
