use std::hashmap::HashMap;

use stb_image::image;

use texture::Texture;


struct Image {
	data  : ~[u8],
	width : uint,
	height: uint
}


pub fn load() -> HashMap<~str, Texture> {
	let image_path = ~"images/spaceship.png";

	let image   = load_image(image_path);
	let texture = Texture::new_rgb(
		image.data,
		image.width,
		image.height);

	let mut images = HashMap::new();
	images.insert(image_path, texture);

	images
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
