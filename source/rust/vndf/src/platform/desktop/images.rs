use stb_image::image;

use physics::Vec2;

use super::{
	Texture,
	Textures
};


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
		let image = match load_image(path) {
			Ok(image)  => image,
			Err(error) => fail!(error)
		};
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
