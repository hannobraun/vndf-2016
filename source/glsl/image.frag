#version 130

uniform sampler2D tex;

out vec4 outColor;

void main()
{
	outColor = texture(tex, gl_TexCoord[0].st);
}
