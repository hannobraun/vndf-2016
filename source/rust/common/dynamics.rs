#[crate_type = "rlib"];
#[link(name = "dynamics", package_id = "dynamics", vers = "0.0")];


extern mod common;


pub struct Body {
	pos: common::vec::Vec2,
	vel: common::vec::Vec2
}
