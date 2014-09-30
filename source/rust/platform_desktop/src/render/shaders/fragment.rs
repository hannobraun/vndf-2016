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

			if (r > r2) {
				// We're outside the circle that represents the planet. Whatever
				// position the billboard has in space, it must not prevent any
				// other elements from being drawn over it.
				gl_FragDepth = 1.0;
			}
			else {
				// Since this is a billboard (which is flat), we need a bit of
				// math to set the depth buffer value as if it were a sphere.
				float depth  = distance_to_eye - sqrt(r2*r2 - r*r) * radius;
				float A      = projection[2].z;
				float B      = projection[3].z;
				gl_FragDepth = 0.5*(-A*depth + B) / depth + 0.5;
			}
		}
	"
};

pub static RINGS: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		in vec2 point;

		out vec4 out_color;

		float ring(float r, float d) {
			return 1.0 - abs(r - d) * 250.0;
		}

		void main() {
			float r     = length(point);
			float d_outer = 0.95;
			float d_inner = 0.15;

			float alpha_base = 0.2;
			if (r > d_outer || r < d_inner) {
				alpha_base = 0.0;
			}

			float alpha = max(
				alpha_base,
				max(
					ring(r, d_outer),
					ring(r, d_inner)
				)
			);

			out_color = vec4(1.0, 1.0, 1.0, alpha);
		}
	"
};

pub static TEXTURE: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform sampler2D tex;

		in vec2 texture_coordinate;

		out vec4 out_color;

		void main()
		{
			out_color = texture(tex, texture_coordinate);
		}
	"
};
