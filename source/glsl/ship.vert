#version 130

uniform vec2 screen;
uniform vec2 camera;
uniform vec2 position;

in vec3 vertex;

void main()
{
	mat4 m = mat4(
		2.0 / screen.x,            0.0,  0.0 , 0.0,
		           0.0, 2.0 / screen.y,  0.0 , 0.0,
		           0.0,            0.0, -0.01, 0.0,
		          -1.0,           -1.0,  0.0 , 1.0);

	vec2 camera_trans = screen * 0.5 - camera;

	vec3 translated = vertex + vec3(position, 0.0) + vec3(camera_trans, 0.0);
	gl_Position = m * vec4(translated, 1.0);

	gl_TexCoord[0] = gl_MultiTexCoord0;
}
