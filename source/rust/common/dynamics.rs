#[crate_type = "lib"];
#[link(name = "dynamics", vers = "0.0")];
#[no_std];


extern mod vec;


pub struct Body {
	pos: vec::Vec2,
	vec: vec::Vec2
}
