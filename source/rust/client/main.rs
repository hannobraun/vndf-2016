extern crate common;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;


use std::hashmap::HashMap;
use std::io;
use std::os;
use std::path;
use std::str;

use camera::Camera;
use entities::Entities;


mod camera;
mod display;
mod entities;
mod font;
mod images;
mod input;
mod net;
mod protocol;
mod texture;
mod visual;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screen_width  = 800;
	let screen_height = 600;

	let server_address = get_server_address();

	let window = display::init(screen_width, screen_height);
	let images = images::load();
	let font   = font::load();

	let mut textures = HashMap::new();
	for (id, &texture) in images.iter().chain(font.iter()) {
		textures.insert(id.clone(), texture);
	}

	let     socket_fd  = net::connect(server_address, ~"34481");
	let mut connection = protocol::init(socket_fd);

	let mut entities = Entities::new();

	entities.update_asteroid(999, 0.0, 0.0);

	let mut cam = Camera::new();

	while !window.should_close() &&
		window.get_key(glfw::KeyEscape) == glfw::Release {

		protocol::receive_positions(
			&mut connection,
			entities);
		input::apply(
			&window,
			cam);
		display::render(
			&window,
			cam,
			&entities.positions,
			&entities.visuals,
			&textures);

		glfw::poll_events();
	}
}

fn get_server_address() -> ~str {
	let args = os::args();

	if args.len() > 2 {
		fail!("Usage: {:s} <server_address>\n", args[0]);
	}

	if args.len() == 2 {
		args[1]
	}
	else {
		let mut file = match io::File::open(&path::posix::Path::new("server")) {
			Ok(file) => file,
			Err(e)   => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		let contents = match file.read_to_end() {
			Ok(contents) => contents,
			Err(e)       => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		str::from_utf8(contents).unwrap_or_else(|| { fail!() }).to_owned()
	}
}
