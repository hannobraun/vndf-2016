use collections::HashMap;
use std::c_str::CString;
use std::io::File;
use std::ptr;

use gl;
use gl::types::{
	GLenum,
	GLuint
};

use error::exit;
use ui::Window;


pub struct Shaders {
	programs: HashMap<~str, Program>
}

pub type Program = GLuint;

type ShaderMap = HashMap<~str, Shader>;
type Shader    = GLuint;



impl Shaders {
	pub fn new(_: &Window) -> Shaders {
		// The window argument isn't actually used. It's here to document this
		// function's dependence on an OpenGL context, which is implied by
		// Window.

		let mut shaders = HashMap::new();
		create_shader(gl::VERTEX_SHADER, "glsl/ui-overlay.vert", &mut shaders);
		create_shader(gl::VERTEX_SHADER, "glsl/ship.vert", &mut shaders);
		create_shader(gl::FRAGMENT_SHADER, "glsl/text.frag", &mut shaders);
		create_shader(gl::FRAGMENT_SHADER, "glsl/image.frag", &mut shaders);
		create_shader(gl::VERTEX_SHADER, "glsl/grid.vert", &mut shaders);
		create_shader(gl::FRAGMENT_SHADER, "glsl/grid.frag", &mut shaders);

		let mut programs = HashMap::new();
		create_program(
			"ui-overlay",
			[
				*shaders.get(&"glsl/ui-overlay.vert".to_owned()),
				*shaders.get(&"glsl/text.frag".to_owned())],
			&mut programs);
		create_program(
			"ship-text",
			[
				*shaders.get(&"glsl/ship.vert".to_owned()),
				*shaders.get(&"glsl/text.frag".to_owned())],
			&mut programs);
		create_program(
			"ship-image",
			[
				*shaders.get(&"glsl/ship.vert".to_owned()),
				*shaders.get(&"glsl/image.frag".to_owned())],
			&mut programs);

		Shaders {
			programs: programs
		}
	}

	pub fn program(&self, key: &str) -> Program {
		match self.programs.find(&key.to_owned()) {
			Some(&program) => program,
			None           => exit(format!("Shader program not found: {}", key))
		}
	}
}


fn create_shader(kind: GLenum, path: &str, shaders: &mut ShaderMap) {
	let shader = gl::CreateShader(kind);
	unsafe {
		gl::ShaderSource(
			shader,
			1,
			&load_shader(path).to_c_str().unwrap(),
			ptr::null());
	}
	gl::CompileShader(shader);

	let mut status = 0;
	unsafe {
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
	}

	if status == gl::FALSE as i32 {
		let mut buffer = [0, .. 1024];
		unsafe {
			gl::GetShaderInfoLog(
				shader,
				buffer.len() as i32,
				::std::ptr::mut_null(),
				buffer.as_mut_ptr());
		}

		let c_str = unsafe {
			CString::new(
				buffer.as_ptr(),
				false)
		};

		print!("Error compiling shader {}:\n\n", path);
		print!("{}\n", c_str.as_str().unwrap());

		exit("");
	}

	shaders.insert(path.to_owned(), shader);
}

fn load_shader(path: &str) -> ~str {
	match File::open(&Path::new(path)).read_to_str() {
		Ok(string) => string,
		Err(error) => fail!("Error loading shader: {}", error)
	}
}

fn create_program(id: &str, shaders: &[Shader], programs: &mut HashMap<~str, Program>) {
	let program = gl::CreateProgram();
	for &shader in shaders.iter() {
		gl::AttachShader(program, shader);
	}
	gl::LinkProgram(program);
	programs.insert(id.to_owned(), program);
}
