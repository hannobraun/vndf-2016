#include "text.h"

#include <assert.h>

#include <ft2build.h>
#include FT_FREETYPE_H


Texture text_loadCharAsTexture(unsigned long c)
{
	FT_Library freeType;
	FT_Face    fontFace;

	FT_Error error = FT_Init_FreeType(&freeType);
	assert(!error);

	error = FT_New_Face(
		freeType,
		"fonts/amble/Amble-Bold.ttf",
		0,
		&fontFace);
	assert(!error);

	error = FT_Set_Pixel_Sizes(
		fontFace,
		0,
		16);
	assert(!error);

	unsigned int glyphIndex = FT_Get_Char_Index(fontFace, c);

	error = FT_Load_Glyph(
		fontFace,
		glyphIndex,
		FT_LOAD_DEFAULT);
	assert(!error);

	error = FT_Render_Glyph(
		fontFace->glyph,
		FT_RENDER_MODE_NORMAL);
	assert(!error);

	// Generate texture names.
	GLuint textureName;
	glGenTextures(1, &textureName);

	glBindTexture(
		GL_TEXTURE_2D,
		textureName);

	// Configure texture.
	glTexParameteri(
		GL_TEXTURE_2D,
		GL_TEXTURE_MIN_FILTER,
		GL_NEAREST);

	// Bind image data to texture name.
	glTexImage2D(
		GL_TEXTURE_2D,
		0,
		GL_ALPHA8,
		fontFace->glyph->bitmap.width,
		fontFace->glyph->bitmap.rows,
		0,
		GL_ALPHA,
		GL_UNSIGNED_BYTE,
		fontFace->glyph->bitmap.buffer);

	Texture texture = {
		textureName,
		fontFace->glyph->bitmap.width,
		fontFace->glyph->bitmap.rows};

	return texture;
}
