use collections::HashMap;
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
	shaders : ShaderMap,
	programs: HashMap<~str, Program>
}

type ShaderMap = HashMap<~str, Shader>;
type Shader    = GLuint;
type Program   = GLuint;


impl Shaders {
	pub fn new(_: &Window) -> Shaders {
		// The window argument isn't actually used. It's here to document this
		// function's dependence on an OpenGL context, which is implied by
		// Window.

		let mut shaders = HashMap::new();
		create_shader(gl::VERTEX_SHADER, "glsl/ui-overlay.vert", &mut shaders);
		create_shader(gl::FRAGMENT_SHADER, "glsl/ui-overlay.frag", &mut shaders);

		let mut programs = HashMap::new();
		let shader_program = gl::CreateProgram();
		gl::AttachShader(
			shader_program,
			*shaders.get(&"glsl/ui-overlay.vert".to_owned()));
		gl::AttachShader(
			shader_program,
			*shaders.get(&"glsl/ui-overlay.frag".to_owned()));
		gl::LinkProgram(shader_program);
		programs.insert("ui-overlay".to_owned(), shader_program);

		Shaders {
			shaders : shaders,
			programs: programs
		}
	}

	pub fn shader<'a>(&self, key: &str) -> Shader {
		match self.shaders.find(&key.to_owned()) {
			Some(&shader) => shader,
			None          => exit(format!("Shader not found: {}", key))
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
	shaders.insert(path.to_owned(), shader);
}

fn load_shader(path: &str) -> ~str {
	match File::open(&Path::new(path)).read_to_str() {
		Ok(string) => string,
		Err(error) => fail!("Error loading shader: {}", error)
	}
}
