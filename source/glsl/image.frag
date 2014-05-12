#version 130

uniform sampler2D tex;

in vec4 TexCoord;

out vec4 outColor;

void main()
{
	outColor = texture(tex, TexCoord.st);
}
