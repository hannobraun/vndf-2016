use stb_image::image;

use common::physics::Vec2;

use error::exit;
use ui::{Texture, Textures};


struct Image {
	data  : Vec<u8>,
	width : uint,
	height: uint
}


pub fn load(textures: &mut Textures) {
	let paths = vec!(
		"images/missile.png",
		"images/spaceship.png");

	for &path in paths.iter() {
		let image   = load_image(path);
		let texture = Texture::new_rgb(
			&image.data,
			Vec2(
				image.width as f64,
				image.height as f64));

		textures.add(
			path.to_string(),
			texture)
	}
}

fn load_image(image_path: &str) -> Image {
	match image::load(&Path::new(image_path)) {
		image::ImageU8(image) => {
			let width  = image.width;
			let height = image.height;

			Image {
				data  : image.data,
				width : width,
				height: height }
		},

		image::ImageF32(_) =>
			exit(format!("Unexpected image type: ImageF32").as_slice()),

		image::Error(message) =>
			exit(message.as_slice())
	}
}
