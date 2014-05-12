#version 130

uniform sampler2D tex;

in vec4 TexCoord;

out vec4 outColor;

void main()
{
	outColor = vec4(1.0, 1.0, 1.0, texture(tex, TexCoord.st).r);
}
