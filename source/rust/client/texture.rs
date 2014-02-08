use std::libc;

use gl;


pub struct Texture {
	name  : gl::types::GLuint,
	width : libc::c_int,
	height: libc::c_int
}
