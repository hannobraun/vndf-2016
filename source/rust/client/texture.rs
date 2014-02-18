use gl;


type Name = gl::types::GLuint;


pub struct Texture {
	name  : Name,
	width : uint,
	height: uint
}
