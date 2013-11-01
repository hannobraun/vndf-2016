#[crate_type = "lib"];
#[link(name = "vec", vers = "0.0")];
#[no_std];


struct Vec2 {
	x: f64,
	y: f64
}


#[no_mangle]
pub extern fn vec_add(a: Vec2, b: Vec2) -> Vec2 {
	Vec2 {
		x: a.x + b.x,
		y: a.y + b.y }
}
