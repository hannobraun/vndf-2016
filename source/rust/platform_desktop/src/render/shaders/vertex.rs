use gfx;


pub static ICON: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec2 size;
		uniform mat4 transform;

		in vec3 vertex;
		in vec2 tex_coord;

		out vec2 texture_coordinate;

		void main()
		{
			gl_Position =
				transform
				* vec4(
					vertex.xy * size.xy,
					vertex.z,
					1.0
				);
			texture_coordinate = tex_coord;
		}
	"
};

pub static PLANET: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform float radius;
		uniform mat4  transform;
		uniform vec3  camera_right_world;
		uniform vec3  camera_up_world;

		in vec3 vertex;

		out vec2 point;

		void main() {
			vec3 vertex_world =
				vec3(0.0, 0.0, 0.0)
				+ camera_right_world * vertex.x * radius
				+ camera_up_world * vertex.y * radius;

			gl_Position = transform * vec4(vertex_world, 1.0);
			point = vertex.xy;
		}
	"
};

pub static SIMPLE: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4 transform;

		in vec3 vertex;

		void main() {
			gl_Position = transform * vec4(vertex, 1.0);
		}
	"
};
