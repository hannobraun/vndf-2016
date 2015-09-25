use nalgebra::Vec2;

use shared::color::Color;
use shared::planet;


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
pub struct ManeuverData {
    pub start_s   : f64,
    pub duration_s: f64,
    pub angle     : f64,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Planet {
	color: Color,
	size : f32,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Ship;

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Attributes {
    Planet(planet::PlanetAttr),
}
