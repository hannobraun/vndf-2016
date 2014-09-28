use gfx;


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
