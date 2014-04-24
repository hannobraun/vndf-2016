use common::physics::Radians;


pub struct Control {
	pub attitude: Radians,
	pub send    : bool
}

pub struct Visual {
	pub texture: ~str
}
