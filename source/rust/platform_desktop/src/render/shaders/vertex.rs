use gfx;


pub static BASE: gfx::ShaderSource<'static> = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3 center;
		uniform vec3 position;
		uniform mat4 transform;

		in vec3 vertex;

		out vec2 point;

		void main() {
			float size = transform[3][3] * 0.025;
			vec3 v = vec3(position.xy, center.z) + vertex * size;
			gl_Position = transform * vec4(v, 1.0);

			point = vertex.xy;
		}
	"
};

pub static FIXED_SIZE_BILLBOARD: gfx::ShaderSource<'static> = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3 position;
		uniform mat4 transform;
		uniform vec2 size;
		uniform vec2 offset;
		uniform vec2 screen_size;

		in vec3 vertex;
		in vec2 tex_coord;

		out vec2 texture_coordinate;
		out vec2 point;

		void main() {
			gl_Position = transform * vec4(position, 1.0);
			gl_Position /= gl_Position.w;
			gl_Position.xy += vertex.xy * size / screen_size;
			gl_Position.xy += offset * 2 / screen_size;

			texture_coordinate = tex_coord;
			point = vertex.xy;
		}
	"
};

pub static LINE: gfx::ShaderSource<'static> = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3 center;
		uniform vec3 position;
		uniform mat4 transform;

		in vec3 vertex;

		void main() {
			vec3 point = position - vec3(0.0, 0.0, position.z - center.z) * vertex.z;
			gl_Position = transform * vec4(point, 1.0);
		}
	"
};

pub static PROJECTED_COURSE: gfx::ShaderSource<'static> = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3 position;
		uniform vec3 velocity;
		uniform mat4 transform;

		in vec3 vertex;

		void main() {
			vec3 point = position + velocity * vertex.x;
			gl_Position = transform * vec4(point, 1.0);
		}
	"
};

pub static RINGS: gfx::ShaderSource<'static> = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4  transform;
		uniform float radius;

		in vec3 vertex;

		out vec2 point;

		void main() {
			gl_Position = transform * vec4(vertex * radius, 1.0);

			point = vertex.xy;
		}
	"
};

pub static SCALED_BILLBOARD: gfx::ShaderSource<'static> = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3  position;
		uniform float radius;
		uniform mat4  transform;
		uniform vec3  camera_right_world;
		uniform vec3  camera_up_world;

		in vec3 vertex;

		out vec2 point;

		void main() {
			vec3 vertex_world =
				position
				+ camera_right_world * vertex.x * radius
				+ camera_up_world * vertex.y * radius;

			gl_Position = transform * vec4(vertex_world, 1.0);
			point = vertex.xy;
		}
	"
};
