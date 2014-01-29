use std::libc;
use std::ptr;

use stb_image::image;


struct Image {
	data  : *libc::c_uchar,
	width : libc::c_int,
	height: libc::c_int
}


#[no_mangle]
pub extern fn loadImage() -> Image {
	match image::load(~"images/spaceship.png") {
		image::ImageU8(image) => {
			Image {
				data  : image.data.as_ptr(),
				width : image.width  as libc::c_int,
				height: image.height as libc::c_int }
		},

		image::ImageF32(image) => fail!("Unexpected image type."),
		image::Error(message)  => fail!(message)
	}
}
