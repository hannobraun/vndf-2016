#version 130

uniform vec2 screen;
uniform vec2 camera;

in vec2 vertex;

void main()
{
	mat4 m = mat4(
		2.0 / screen.x,            0.0,  0.0 , 0.0,
		           0.0, 2.0 / screen.y,  0.0 , 0.0,
		           0.0,            0.0, -0.01, 0.0,
		          -1.0,           -1.0,  0.0 , 1.0);

	vec2 camera_trans = screen * 0.5 - camera;

	gl_Position = m * vec4(vertex + camera_trans, 0.0, 1.0);
}
