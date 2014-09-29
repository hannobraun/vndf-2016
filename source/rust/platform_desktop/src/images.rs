use std::collections::HashMap;

use stb_image::image;


pub type Images = HashMap<String, Image>;

pub struct Image {
	pub data  : Vec<u8>,
	pub width : uint,
	pub height: uint
}


pub fn load() -> Images {
	let paths = vec!(
		"images/missile.png",
		"images/spaceship.png",
	);

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

fn load_image(path: &str) -> Result<Image, String> {
	match image::load(&Path::new(path)) {
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
