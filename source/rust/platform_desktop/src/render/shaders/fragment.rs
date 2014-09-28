use gfx;


pub static PLANET: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4  projection;
		uniform float distance_to_eye;
		uniform float radius;

		in vec2 point;

		out vec4 out_color;

		void main() {
			float atmosphere_height = 0.06;

			float r2 = 1.0;
			float r1 = r2 - atmosphere_height;

			float r = length(point);

			// Lighten the circle a bit towards the center to give the
			// perception of roundness and depth.
			vec3 base_color = vec3(0.8, 0.68, 0.27);
			vec3 color      = base_color * (1.0 - r * 0.2);

			// At the edges, the planet should not be completely solid, to give
			// the appearance of an atmosphere.
			float a = 1.0 - (1.0 / (r2 - r1) * (r - r1));

			// The final color is just composed from the two previous
			// computations.
			out_color = vec4(color, a);

			// If we're outside the circle, we're done. Otherwise, the following
			// will mess up the depth buffer.
			if (r > r2) {
				return;
			}

			// Since this is a billboard, we need a bit of math to set the depth
			// buffer value as if it were a sphere.
			float depth  = distance_to_eye - sqrt(r2*r2 - r*r) * radius;
			float A      = projection[2].z;
			float B      = projection[3].z;
			gl_FragDepth = 0.5*(-A*depth + B) / depth + 0.5;
		}
	"
};

pub static SIMPLE: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		out vec4 out_color;

		void main() {
			out_color = vec4(1.0, 1.0, 1.0, 1.0);
		}
	"
};
