use gfx;


pub static PLANET: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4  projection;
		uniform float distance_to_eye;
		uniform float radius;
		uniform vec3  base_color;

		in vec2 point;

		out vec4 out_color;

		void main() {
			float atmosphere_height = 0.06;

			float r2 = 1.0;
			float r1 = r2 - atmosphere_height;

			float r = length(point);

			// Lighten the circle a bit towards the center to give the
			// perception of roundness and depth.
			vec3 color = base_color * (1.0 - r * 0.2);

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
			return 1.0 - abs(r - d) * 150.0;
		}
		float line(float inner, float outer, float r, vec2 point, vec2 v) {
			if (r < inner || r > outer) {
				return 0.0;
			}
			return 1.0 - abs(dot(v, point)) * 150;
		}

		void main() {
			float r     = length(point);
			float outer = 0.95;
			float inner = 0.15;

			float alpha_base = 0.3;
			if (r > outer || r < inner) {
				alpha_base = 0.0;
			}

			float alphas[9];
			alphas[0] = alpha_base;
			alphas[1] = ring(r, outer);
			alphas[2] = ring(r, inner);
			alphas[3] = line(inner, outer, r, point, vec2(1.0, 0.0));
			alphas[4] = line(inner, outer, r, point, vec2(0.0, 1.0));
			alphas[5] = line(outer - 0.05, outer, r, point, vec2(0.577, 1.0));
			alphas[6] = line(outer - 0.05, outer, r, point, vec2(0.577, -1.0));
			alphas[7] = line(outer - 0.05, outer, r, point, vec2(1.732, 1.0));
			alphas[8] = line(outer - 0.05, outer, r, point, vec2(1.732, -1.0));

			float alpha = 0.0;
			for (int i = 0; i < 9; i += 1) {
				alpha = max(alpha, alphas[i]);
			}

			out_color = vec4(0.5, 0.5, 1.0, alpha);

			if (out_color.a == 0.0) {
				gl_FragDepth = 1.0;
			}
			else {
				gl_FragDepth = gl_FragCoord.z;
			}
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

			if (out_color.a == 0.0) {
				gl_FragDepth = 1.0;
			}
			else {
				gl_FragDepth = gl_FragCoord.z;
			}
		}
	"
};
