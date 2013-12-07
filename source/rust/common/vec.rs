#[crate_type = "lib"];
#[link(name = "vec", package_id = "vec", vers = "0.0")];
#[no_std];


extern {
	fn sqrt(x: f64) -> f64;
}


pub struct Vec2 {
	x: f64,
	y: f64
}


#[no_mangle]
extern fn vec_add(a: Vec2, b: Vec2) -> Vec2 {
	Vec2 {
		x: a.x + b.x,
		y: a.y + b.y }
}

#[no_mangle]
extern fn vec_scale(v: Vec2, s: f64) -> Vec2 {
	Vec2 {
		x: v.x * s,
		y: v.y * s }
}

#[no_mangle]
extern fn vec_magnitude(v: Vec2) -> f64 {
	unsafe {
		sqrt(v.x*v.x + v.y*v.y)
	}
}

#[no_mangle]
extern fn vec_normalize(v: Vec2) -> Vec2 {
	let m = vec_magnitude(v);
	vec_scale(v, 1.0/m)
}
