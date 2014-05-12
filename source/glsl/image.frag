#version 130

uniform sampler2D tex;

in vec2 TexCoord;

out vec4 outColor;

void main()
{
	outColor = texture(tex, TexCoord);
}
