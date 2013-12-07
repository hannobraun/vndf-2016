#[crate_type = "rlib"];
#[link(name = "dynamics", package_id = "dynamics", vers = "0.0")];


extern mod vec;


pub struct Body {
	pos: vec::Vec2,
	vec: vec::Vec2
}
