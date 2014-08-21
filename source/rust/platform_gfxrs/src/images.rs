use std::collections::HashMap;

use stb_image::image;

use physics::Vec2;


struct Image {
	data  : Vec<u8>,
	width : uint,
	height: uint
}


pub fn load() -> HashMap<String, Image> {
	let paths = vec!(
		"images/missile.png",
		"images/spaceship.png");

	let mut images = HashMap::new();

	for &path in paths.iter() {
		let image = match load_image(path) {
			Ok(image)  => image,
			Err(error) => fail!(error)
		};

		images.insert(path.to_string(), image);
	}

	images
}

fn load_image(image_path: &str) -> Result<Image, String> {
	match image::load(&Path::new(image_path)) {
		image::ImageU8(image) => {
			let width  = image.width;
			let height = image.height;

			Ok(Image {
				data  : image.data,
				width : width,
				height: height
			})
		},

		image::ImageF32(_) =>
			Err(format!("Unexpected image type: ImageF32")),

		image::Error(message) =>
			Err(message)
	}
}
