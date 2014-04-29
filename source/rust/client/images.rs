use stb_image::image;

use common::physics::Vec2;

use ui::{Texture, Textures};


struct Image {
	data  : ~[u8],
	width : uint,
	height: uint
}


pub fn load(textures: &mut Textures) {
	let image_path = ~"images/spaceship.png";

	let image   = load_image(image_path);
	let texture = Texture::new_rgb(
		image.data,
		Vec2(
			image.width as f64,
			image.height as f64));

	textures.add(image_path, texture);
}

fn load_image(image_path: &str) -> Image {
	match image::load(image_path.into_owned()) {
		image::ImageU8(image) => {
			let width  = image.width;
			let height = image.height;

			Image {
				data  : image.data,
				width : width,
				height: height }
		},

		image::ImageF32(_)    => fail!("Unexpected image type: ImageF32"),
		image::Error(message) => fail!(message)
	}
}
