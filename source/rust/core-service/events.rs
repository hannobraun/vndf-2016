#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[link(name = "protocol", package_id = "protocol", vers = "0.0")];


extern mod clients;
extern mod vec;


extern {
	fn close(fd: std::libc::c_int) -> std::libc::c_int;
}


#[no_mangle]
pub extern fn onConnect(clientFD: std::libc::c_int, clientMap: &mut clients::ClientMap) {
	if (clients::clients_canAdd(clientMap)) {
		let distance = 100.0;

		let alpha = 90.0 / 180.0 * std::f64::consts::PI;

		let pos = vec::Vec2 {
			x: distance * std::f64::cos(alpha),
			y: distance * std::f64::sin(alpha) };

		let vel = vec::Vec2 {
			x: 30.0,
			y: 0.0 };

		clients::clients_add(clientMap, clientFD, pos, vel);
	}
	else
	{
		unsafe {
			close(clientFD);
		}
	}
}
