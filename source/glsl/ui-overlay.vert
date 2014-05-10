#version 130

attribute vec3 vertex;

uniform vec2 position;

void main()
{
	mat4 m = mat4(
		2.0 / 800.0,         0.0,  0.0 , 0.0,
		        0.0, 2.0 / 600.0,  0.0 , 0.0,
		        0.0,         0.0, -0.01, 0.0,
		       -1.0,        -1.0,  0.0 , 1.0);

	vec3 translated = vertex + vec3(position, 0.0);
	gl_Position = m * vec4(translated, 1.0);
}
