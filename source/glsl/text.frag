#version 130

uniform sampler2D tex;

out vec4 outColor;

void main()
{
	outColor = vec4(1.0, 1.0, 1.0, texture(tex, gl_TexCoord[0].st).r);
}
