#version 130

uniform vec2 screen;
uniform vec2 position;

in vec3 vertex;

out vec2 TexCoord;

void main()
{
	mat4 m = mat4(
		2.0 / screen.x,            0.0,  0.0 , 0.0,
		           0.0, 2.0 / screen.y,  0.0 , 0.0,
		           0.0,            0.0, -0.01, 0.0,
		          -1.0,           -1.0,  0.0 , 1.0);

	vec3 translated = vertex + vec3(position, 0.0);
	gl_Position = m * vec4(translated, 1.0);

	TexCoord = gl_MultiTexCoord0.st;
}
